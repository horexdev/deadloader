<#
Starts deadloader.exe from a fresh folder, watches the TCP connections of its whole process tree (the exe and the
WebView2 processes it starts) for a while, and fails if any of them has a remote endpoint. Also fails if the window
never loaded its page, so "no connections" cannot come from a webview that did not start.

The process tree is built from the parent PID *and* the creation time: Windows never clears ParentProcessId and reuses
PIDs, so an unrelated older process can look like our child.
#>
param(
    [Parameter(Mandatory)] [string] $Exe,
    [int] $Seconds = 20,
    [int] $MaxIdlePrivateMB = 200
)

$ErrorActionPreference = 'Stop'

$runDir = Join-Path ([IO.Path]::GetTempPath()) "deadloader-zero-connections-$PID"
Remove-Item -Recurse -Force $runDir -ErrorAction SilentlyContinue
New-Item -ItemType Directory $runDir | Out-Null
$runExe = Join-Path $runDir 'deadloader.exe'
Copy-Item $Exe $runExe

function Test-Local([string] $address) {
    $ip = [Net.IPAddress]::Parse($address)
    if ($ip.IsIPv4MappedToIPv6) { $ip = $ip.MapToIPv4() }
    return [Net.IPAddress]::IsLoopback($ip) -or $ip.Equals([Net.IPAddress]::Any) -or $ip.Equals([Net.IPAddress]::IPv6Any)
}

function Get-Tree([int] $rootId) {
    $all = Get-CimInstance Win32_Process -Property ProcessId, ParentProcessId, CreationDate, Name, CommandLine
    $byParent = $all | Group-Object ParentProcessId -AsHashTable -AsString
    $root = $all | Where-Object ProcessId -EQ $rootId
    if (-not $root) { return @() }
    $tree = [Collections.Generic.List[object]]::new()
    $queue = [Collections.Generic.Queue[object]]::new()
    $queue.Enqueue($root)
    while ($queue.Count -gt 0) {
        $p = $queue.Dequeue()
        $tree.Add($p)
        foreach ($c in @($byParent["$($p.ProcessId)"])) {
            # A child is created after its parent; an older process with that parent id is someone else's.
            if ($c -and $c.ProcessId -ne $p.ProcessId -and $c.CreationDate -ge $p.CreationDate) { $queue.Enqueue($c) }
        }
    }
    return $tree
}

$proc = Start-Process -FilePath $runExe -WorkingDirectory $runDir -PassThru
$rootId = $proc.Id
Write-Host "started deadloader.exe, pid $rootId"

$remote = [Collections.Generic.List[object]]::new()
$seen = @{}
$tree = @()
$names = @{}
$deadline = (Get-Date).AddSeconds($Seconds)
$nextTree = Get-Date
while ((Get-Date) -lt $deadline) {
    if ((Get-Date) -ge $nextTree) {
        $tree = Get-Tree $rootId
        foreach ($p in $tree) { $names[[int]$p.ProcessId] = $p.Name }
        $nextTree = (Get-Date).AddSeconds(1)
    }
    $ids = [Collections.Generic.HashSet[int]]::new()
    foreach ($p in $tree) { [void]$ids.Add([int]$p.ProcessId) }
    foreach ($c in Get-NetTCPConnection -ErrorAction SilentlyContinue) {
        if (-not $ids.Contains([int]$c.OwningProcess)) { continue }
        if (Test-Local $c.RemoteAddress) { continue }
        $key = "$($c.OwningProcess) $($c.RemoteAddress):$($c.RemotePort)"
        if ($seen.ContainsKey($key)) { continue }
        $seen[$key] = $true
        $remote.Add([pscustomobject]@{
                Process = "$($names[[int]$c.OwningProcess]) ($($c.OwningProcess))"
                Remote  = "$($c.RemoteAddress):$($c.RemotePort)"
                State   = "$($c.State)"
                At      = (Get-Date).ToString('HH:mm:ss.fff')
            })
    }
    Start-Sleep -Milliseconds 100
}

$failures = [Collections.Generic.List[string]]::new()
$tree = Get-Tree $rootId
$alive = Get-Process -Id $rootId -ErrorAction SilentlyContinue
if (-not $alive) { $failures.Add('deadloader.exe exited during the check') }
$webviews = @($tree | Where-Object Name -EQ 'msedgewebview2.exe')
if ($webviews.Count -eq 0) { $failures.Add('no WebView2 process was started') }

# The arguments must have reached the WebView2 browser process (the one without a --type switch).
$browser = $webviews | Where-Object { $_.CommandLine -notmatch '--type=' } | Select-Object -First 1
if ($browser) {
    foreach ($required in '--proxy-server=127.0.0.1:9', 'msOneAuthWAM', 'DnsOverHttpsUpgrade') {
        if ($browser.CommandLine -notmatch [regex]::Escape($required)) { $failures.Add("WebView2 was started without $required") }
    }
    if ($browser.CommandLine -notmatch [regex]::Escape((Join-Path $runDir 'deadloader-data\webview'))) {
        $failures.Add('the WebView2 profile is not in deadloader-data\webview')
    }
} elseif ($webviews.Count -gt 0) {
    $failures.Add('the WebView2 browser process was not found')
}

$privateMB = 0
foreach ($p in $tree) {
    $gp = Get-Process -Id $p.ProcessId -ErrorAction SilentlyContinue
    if ($gp) { $privateMB += $gp.PrivateMemorySize64 / 1MB }
}
$privateMB = [math]::Round($privateMB, 1)
if ($MaxIdlePrivateMB -gt 0 -and $privateMB -gt $MaxIdlePrivateMB) {
    $failures.Add("the process tree uses $privateMB MB private memory at idle (budget $MaxIdlePrivateMB MB)")
}

$log = Get-ChildItem (Join-Path $runDir 'deadloader-data\logs') -Filter 'app-*.jsonl' -ErrorAction SilentlyContinue |
    Get-Content | ForEach-Object { $_ | ConvertFrom-Json }
$loaded = $log | Where-Object msg -EQ 'page loaded' | Select-Object -First 1
if (-not $loaded) { $failures.Add('the page never finished loading (no "page loaded" line in the app log)') }
$runtime = ($log | Where-Object msg -EQ 'webview2 runtime' | Select-Object -First 1).fields.version
$profileWarning = $log | Where-Object msg -EQ 'webview2 profile folder is not where deadloader put it'
if ($profileWarning) { $failures.Add('the app reported that the WebView2 profile folder was moved') }

# Stop only our own tree, children first.
[array]::Reverse($tree)
foreach ($p in $tree) { Stop-Process -Id $p.ProcessId -Force -ErrorAction SilentlyContinue }
Start-Sleep -Seconds 2
Remove-Item -Recurse -Force $runDir -ErrorAction SilentlyContinue

foreach ($r in $remote) { $failures.Add("remote connection: $($r.Process) -> $($r.Remote) [$($r.State)] at $($r.At)") }

$summary = @(
    '### Zero-connection check'
    ''
    "| | |"
    "|---|---|"
    "| WebView2 runtime | $runtime |"
    "| processes in the tree | $($tree.Count) ($($webviews.Count) WebView2) |"
    "| watched | $Seconds s |"
    "| remote TCP connections | $($remote.Count) |"
    "| page loaded after start | $($loaded.fields.ms_since_start) ms |"
    "| private memory at the end | $privateMB MB |"
    ''
)
$summary | Write-Host
if ($env:GITHUB_STEP_SUMMARY) { $summary | Add-Content $env:GITHUB_STEP_SUMMARY }

if ($failures.Count -gt 0) {
    $failures | ForEach-Object { Write-Host "::error::$_" }
    exit 1
}
Write-Host 'no remote connections'
