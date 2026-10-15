<#
Starts deadloader.exe from a fresh folder, watches the TCP connections of its whole process tree (the exe and the
WebView2 processes it starts) and fails if any of them has a remote endpoint. Also fails if the window never loaded
its page, so "no connections" cannot come from a webview that did not start.

The first start creates the WebView2 profile, a one-time cost that decision 166 exempts from the 800 ms first-frame
budget. Its time varies a lot on CI runners (about 3 to 16 s), so the script does not sleep a fixed time: it polls the
app log for "page loaded" (up to -MaxLoadSeconds) and then keeps watching for -Seconds more. Idle private memory is
read at the end of that window, when the page has been loaded and idle for -Seconds.

Then it starts the exe -WarmRuns more times on the same data folder. These are regular starts: each must load its
page, the median of their load times must stay within -MaxWarmStartMs, and their connections are watched too.

The process tree is built from the parent PID *and* the creation time: Windows never clears ParentProcessId and reuses
PIDs, so an unrelated older process can look like our child.
#>
param(
    [Parameter(Mandatory)] [string] $Exe,
    # Watched after the first start has loaded its page.
    [int] $Seconds = 20,
    # How long a start may take to load its page before it counts as failed.
    [int] $MaxLoadSeconds = 60,
    [int] $MaxIdlePrivateMB = 200,
    [int] $MaxWarmStartMs = 800,
    [int] $WarmRuns = 3
)

$ErrorActionPreference = 'Stop'

$runDir = Join-Path ([IO.Path]::GetTempPath()) "deadloader-zero-connections-$PID"
Remove-Item -Recurse -Force $runDir -ErrorAction SilentlyContinue
New-Item -ItemType Directory $runDir | Out-Null
$runExe = Join-Path $runDir 'deadloader.exe'
Copy-Item $Exe $runExe
$logDir = Join-Path $runDir 'deadloader-data\logs'

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

function Stop-Tree([object[]] $processes) {
    # Only our own tree, children first.
    [array]::Reverse($processes)
    foreach ($p in $processes) { Stop-Process -Id $p.ProcessId -Force -ErrorAction SilentlyContinue }
    Start-Sleep -Seconds 2
}

# Every "page loaded" line in the app log, oldest first. A line the app is still writing is skipped, not fatal.
function Get-PageLoads {
    @(Get-ChildItem $logDir -Filter 'app-*.jsonl' -ErrorAction SilentlyContinue | Sort-Object Name |
            Get-Content | Where-Object { $_ -like '*"page loaded"*' } |
            ForEach-Object { try { $_ | ConvertFrom-Json } catch { $null } } |
            Where-Object { $_ -and $_.msg -eq 'page loaded' })
}

$remote = [Collections.Generic.List[object]]::new()
$seen = @{}
$names = @{}

# One look at the TCP table: records every remote endpoint owned by a process of the tree.
function Watch-Connections([object[]] $tree) {
    $ids = [Collections.Generic.HashSet[int]]::new()
    foreach ($p in $tree) {
        [void]$ids.Add([int]$p.ProcessId)
        $names[[int]$p.ProcessId] = $p.Name
    }
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
}

# Starts the exe and watches its connections until it logs a new "page loaded" line (or -MaxLoadSeconds pass, or it
# exits), then for -IdleSeconds more. Returns the load time the app logged ($null if none) and the tree at the end;
# the tree is left running for the caller to inspect and stop.
function Invoke-Start([string] $label, [int] $idleSeconds) {
    $before = (Get-PageLoads).Count
    $proc = Start-Process -FilePath $runExe -WorkingDirectory $runDir -PassThru
    Write-Host "$label`: started deadloader.exe, pid $($proc.Id)"
    $started = Get-Date
    $loadDeadline = $started.AddSeconds($MaxLoadSeconds)
    $deadline = $loadDeadline
    $loadedMs = $null
    $tree = @()
    $nextTree = $started
    $nextLog = $started
    while ((Get-Date) -lt $deadline) {
        $now = Get-Date
        if ($now -ge $nextTree) {
            $tree = Get-Tree $proc.Id
            if ($tree.Count -eq 0) { break }
            $nextTree = $now.AddSeconds(1)
        }
        Watch-Connections $tree
        if ($null -eq $loadedMs -and $now -ge $nextLog) {
            $loads = Get-PageLoads
            if ($loads.Count -gt $before) {
                $loadedMs = [int]$loads[-1].fields.ms_since_start
                $deadline = (Get-Date).AddSeconds($idleSeconds)
                Write-Host "$label`: page loaded after $loadedMs ms, watching $idleSeconds s more"
            }
            $nextLog = $now.AddMilliseconds(250)
        }
        Start-Sleep -Milliseconds 100
    }
    $watched = [math]::Round(((Get-Date) - $started).TotalSeconds, 1)
    return [pscustomobject]@{
        Ms      = $loadedMs
        Tree    = @(Get-Tree $proc.Id)
        Alive   = [bool](Get-Process -Id $proc.Id -ErrorAction SilentlyContinue)
        Watched = $watched
    }
}

$failures = [Collections.Generic.List[string]]::new()

$first = Invoke-Start 'first start' $Seconds
$tree = $first.Tree
if (-not $first.Alive) { $failures.Add('deadloader.exe exited during the check') }
if ($null -eq $first.Ms) {
    $failures.Add("the page never finished loading within $MaxLoadSeconds s (no ""page loaded"" line in the app log)")
}
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

# Idle memory only means something once the page is up; otherwise WebView2 may not have started yet.
$privateMB = $null
if ($null -ne $first.Ms) {
    $privateMB = 0
    foreach ($p in $tree) {
        $gp = Get-Process -Id $p.ProcessId -ErrorAction SilentlyContinue
        if ($gp) { $privateMB += $gp.PrivateMemorySize64 / 1MB }
    }
    $privateMB = [math]::Round($privateMB, 1)
    if ($MaxIdlePrivateMB -gt 0 -and $privateMB -gt $MaxIdlePrivateMB) {
        $failures.Add("the process tree uses $privateMB MB private memory at idle (budget $MaxIdlePrivateMB MB)")
    }
}

$log = @(Get-ChildItem $logDir -Filter 'app-*.jsonl' -ErrorAction SilentlyContinue | Get-Content |
        ForEach-Object { try { $_ | ConvertFrom-Json } catch { $null } } | Where-Object { $_ })
$runtime = ($log | Where-Object msg -EQ 'webview2 runtime' | Select-Object -First 1).fields.version
if ($log | Where-Object msg -EQ 'webview2 profile folder is not where deadloader put it') {
    $failures.Add('the app reported that the WebView2 profile folder was moved')
}

Stop-Tree $tree

# Regular starts: the profile exists now. Each is stopped as soon as its page has loaded.
$warmMs = [Collections.Generic.List[int]]::new()
for ($i = 1; $i -le $WarmRuns; $i++) {
    $run = Invoke-Start "regular start $i" 0
    Stop-Tree $run.Tree
    if ($null -eq $run.Ms) { $failures.Add("regular start $i never finished loading its page within $MaxLoadSeconds s") }
    else { $warmMs.Add($run.Ms) }
}
Remove-Item -Recurse -Force $runDir -ErrorAction SilentlyContinue

$warmMedian = $null
if ($warmMs.Count -gt 0) {
    $sorted = @($warmMs | Sort-Object)
    $mid = [math]::Floor($sorted.Count / 2)
    $warmMedian = if ($sorted.Count % 2) { $sorted[$mid] } else { [int](($sorted[$mid - 1] + $sorted[$mid]) / 2) }
    if ($MaxWarmStartMs -gt 0 -and $warmMedian -gt $MaxWarmStartMs) {
        $failures.Add("regular starts loaded their page after $warmMedian ms (median of $($warmMs -join ', ') ms; budget $MaxWarmStartMs ms)")
    }
}

foreach ($r in $remote) { $failures.Add("remote connection: $($r.Process) -> $($r.Remote) [$($r.State)] at $($r.At)") }

$summary = @(
    '### Zero-connection check'
    ''
    "| | |"
    "|---|---|"
    "| WebView2 runtime | $runtime |"
    "| processes in the tree | $($tree.Count) ($($webviews.Count) WebView2) |"
    "| watched, first start | $($first.Watched) s ($Seconds s after the page loaded) |"
    "| remote TCP connections, all starts | $($remote.Count) |"
    "| page loaded, first start (creates the WebView2 profile, not budgeted) | $($first.Ms) ms |"
    "| page loaded, regular starts | $($warmMs -join ', ') ms (median $warmMedian ms, budget $MaxWarmStartMs ms) |"
    "| private memory, idle after the page loaded | $privateMB MB |"
    ''
)
$summary | Write-Host
if ($env:GITHUB_STEP_SUMMARY) { $summary | Add-Content $env:GITHUB_STEP_SUMMARY }

if ($failures.Count -gt 0) {
    $failures | ForEach-Object { Write-Host "::error::$_" }
    exit 1
}
Write-Host 'no remote connections'
