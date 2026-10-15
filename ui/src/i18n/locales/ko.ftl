## Notifications

n1 = Deadlock이 업데이트되었습니다 — 모드를 복원했습니다.
    .why = 업데이트로 gameinfo.gi가 초기화되어 deadloader가 자체 줄을 다시 추가했습니다.
n2 = { $name } — 설치하고 활성화했습니다.
n3 = 모드 업데이트 { $count }개를 사용할 수 있습니다.
n4 = Deadlock이 실행 중입니다.
    .why = 게임을 종료하면 변경 사항이 자동으로 적용됩니다.
    .pending = 대기 중인 변경 사항 { $count }개
n5 =
    { $reason ->
        [completed] 중단된 변경 작업을 완료했습니다.
       *[undone] 중단된 변경 작업을 취소했습니다. 게임 폴더는 이전 상태 그대로입니다.
    }
n6 = 게임의 모드 폴더에서 deadloader가 만들지 않은 파일 { $count }개를 발견했습니다.
n7 =
    { $when ->
        [today] 오프라인 — { $saved }에 저장된 카탈로그를 표시합니다.
       *[earlier] 오프라인 — { $saved }에 저장된 카탈로그를 표시합니다.
    }
    .why = 다시 온라인 상태가 되면 다운로드가 재개됩니다.
n8 = ‘{ $name }’ 프로필을 적용했습니다 — 모드 { $count }개 활성화됨.
n9 = 새 deadloader 버전({ $version })을 사용할 수 있습니다.
n10 = 로컬 모드가 켜져 있습니다.
    .why = 카탈로그가 꺼져 있으며 deadloader는 네트워크 요청을 전혀 보내지 않습니다.
n11 = deadloader와 게임이 서로 다른 드라이브에 있어서 모드를 링크하지 않고 복사합니다.
    .why = 그래서 디스크 공간을 { $size } 더 사용합니다.
n12 = Deadlock이 종료되었습니다 — 변경 사항 { $count }개를 적용했습니다.

## Errors: what happened, why, what to do

e1 = deadloader가 데이터를 저장할 수 없습니다.
    .why =
        { $reason ->
            [access_denied] deadloader.exe가 있는 폴더가 읽기 전용입니다. 예를 들어 Windows는 프로그램이 Program Files 안에 쓰는 것을 허용하지 않습니다.
            [in_use] 다른 프로그램(주로 백신 프로그램)이 deadloader의 파일 하나를 열고 있습니다.
           *[other] Windows가 deadloader.exe가 있는 폴더에 쓰지 못했습니다.
        }
    .todo =
        { $reason ->
            [in_use] 다른 프로그램을 닫고 다시 시도하세요.
           *[other] deadloader를 쓰기 권한이 있는 폴더(예: D:\Games\deadloader)로 옮기세요.
        }
e2 = Deadlock을 찾을 수 없습니다.
    .why = Steam 또는 게임이 설치되어 있지 않거나 게임 폴더가 이동되었습니다.
    .todo = 게임 폴더를 직접 선택하세요.
e3 = 다른 프로그램이 게임 파일을 변경했습니다.
    .why = gameinfo.gi가 원본과 다릅니다. 대개 Deadlock Mod Manager가 변경한 것입니다. deadloader는 다른 프로그램이 변경한 내용을 덮어쓰지 않습니다.
    .todo = Steam에서 게임 파일 무결성을 확인하세요. 그러면 deadloader가 자체 줄을 추가합니다.
e4 = 카탈로그를 사용할 수 없습니다.
    .why = GameBanana의 API가 바뀌어 deadloader가 읽을 수 없습니다. 라이브러리와 프로필은 평소처럼 작동합니다.
    .todo = deadloader 업데이트가 있는지 확인하거나 나중에 다시 시도하세요.
e5 = GameBanana가 혼잡합니다.
    .why =
        { $reason ->
            [rate_limited] 짧은 시간에 요청이 너무 많습니다(HTTP 429).
            [server] GameBanana 서버에서 오류가 발생했습니다.
           *[other] 요청이 너무 많거나 서버 오류가 발생했습니다.
        }
    .todo =
        { $wait ->
            [seconds] deadloader가 { $retry_in_s }초 후에 자동으로 다시 시도합니다.
           *[later] deadloader가 곧 자동으로 다시 시도합니다.
        }
e6 = 다운로드하지 못했습니다.
    .why =
        { $reason ->
            [local_mode] deadloader가 로컬 모드이므로 네트워크 요청을 보내지 않습니다.
            [dns] 다운로드 서버의 주소를 찾을 수 없습니다.
            [connect] deadloader가 다운로드 서버에 연결하지 못했습니다.
            [tls] 다운로드 서버와 보안 연결을 설정하지 못했습니다.
            [timeout] 서버 응답이 너무 오래 걸렸습니다.
            [connection_lost] 연결이 끊어졌습니다.
            [http_status] 서버가 오류로 응답했습니다(HTTP { $status }).
           *[other] 다운로드가 중단되었습니다.
        }
    .todo =
        { $reason ->
            [local_mode] 모드를 다운로드하려면 설정에서 카탈로그를 켜세요.
           *[other] 중단된 지점부터 다시 이어서 받습니다.
        }
e7 = 다운로드한 파일이 손상되었습니다.
    .why =
        { $reason ->
            [size] 파일 크기가 GameBanana에 표시된 값과 일치하지 않습니다.
           *[checksum] 체크섬이 GameBanana에 표시된 값과 일치하지 않습니다.
        }
    .todo = 다시 다운로드하세요.
e8 = 압축 파일을 풀 수 없습니다.
    .why =
        { $reason ->
            [not_an_archive] ZIP, 7z, RAR 압축 파일이 아닙니다. 다운로드가 완료되지 않았을 수 있습니다.
            [damaged] 압축 파일이 손상되었습니다.
            [encrypted] 압축 파일이 암호로 보호되어 있습니다.
            [unsupported] deadloader가 지원하지 않는 압축 방식을 사용합니다.
            [not_a_vpk] 올바른 .vpk 파일이 아닙니다: { $entry }
           *[other] 손상되었거나 지원하지 않는 형식입니다.
        }
    .todo = 이 모드의 다른 파일을 사용해 보거나 제작자에게 문제를 알리세요.
e9 = 압축 파일이 너무 커서 안전하게 풀 수 없습니다.
    .why =
        { $reason ->
            [too_large] 압축을 풀면 크기가 제한({ $limit })을 넘습니다.
            [too_many_files] 모드 파일이 { $found }개 들어 있지만 제한은 { $limit }개입니다.
            [ratio] { $entry }의 압축률이 의심스러울 만큼 높습니다({ $limit }배 초과).
            [memory] 압축을 풀려면 메모리가 { $needed } 필요하지만 제한은 { $limit }입니다.
           *[other] 압축 파일 안전 제한 중 하나를 초과합니다.
        }
    .todo =
        { $reason ->
            [memory] 이 모드의 다른 파일을 사용해 보세요.
           *[other] 신뢰할 수 있는 파일이라면 설정 → 다운로드에서 제한을 늘리세요.
        }
e10 = 이 압축 파일에 모드 파일이 없습니다.
    .why =
        { $listing ->
            [files] .vpk 파일은 없고 다음 항목만 들어 있습니다: { $contents }
           *[empty] 압축 파일이 비어 있습니다.
        }
    .todo = 이 파일은 deadloader가 설치할 수 있는 형식의 Deadlock 모드가 아닙니다.
e11 = 이 PC에서는 RAR 압축 파일을 풀 수 없습니다.
    .why = Windows 기본 도구로는 이 RAR 파일을 읽을 수 없으며, 7-Zip과 WinRAR도 찾지 못했습니다.
    .todo = 7-Zip(무료)을 설치한 후 다시 시도하세요. 다운로드한 파일은 보관됩니다.
e12 = 디스크 공간이 부족합니다.
    .why =
        { $reason ->
            [space] 필요한 공간은 { $needed }, 사용 가능한 공간은 { $available }뿐입니다.
           *[full] 디스크가 가득 찼습니다.
        }
    .todo = 공간을 확보하거나 라이브러리에서 오래된 모드 버전을 삭제한 후 다시 시도하세요.
e13 = Deadlock이 실행 중일 때는 변경 사항을 적용할 수 없습니다.
    .why = 게임이 실행되는 동안 Windows가 게임 파일을 잠급니다.
    .todo = Deadlock을 종료하세요. 종료 직후 deadloader가 변경 사항을 적용합니다.
e14 = 라이브러리의 모드 파일이 없거나 손상되었습니다.
    .why = deadloader 외부에서 변경되거나 삭제되었습니다.
    .todo = 모드를 복구해 파일을 다시 다운로드하거나 모드를 제거하세요.
e15 = 링크를 열 수 없습니다.
    .why = Windows에 기본 브라우저가 설정되어 있지 않습니다.
    .todo = 링크를 복사해 직접 여세요.
e16 = 요청이 차단되었습니다.
    .why =
        { $reason ->
            [not_https] 서버가 HTTPS가 아닌 주소로 리디렉션했습니다. deadloader는 이런 리디렉션을 따르지 않습니다.
            [bad_redirect] 서버가 deadloader가 따를 수 없는 리디렉션을 보냈습니다.
            [too_many_redirects] 서버가 너무 여러 번 리디렉션했습니다.
           *[not_listed] { $host } 호스트가 허용된 호스트 목록에 없어 deadloader가 연결하지 않았습니다.
        }
    .todo =
        { $reason ->
            [not_listed] 아무것도 하지 않아도 됩니다. deadloader는 GameBanana와만 통신합니다. 이 호스트를 신뢰한다면 설정 → 고급에서 추가하세요.
           *[not_https] 아무것도 하지 않아도 됩니다. deadloader는 안전한 리디렉션만 따릅니다. 나중에 다시 시도하세요.
        }
e17 = 프로필을 가져올 수 없습니다.
    .why =
        { $reason ->
            [newer] 더 새로운 버전의 deadloader로 만든 프로필입니다.
            [unreadable] 파일을 읽을 수 없습니다. 이동 또는 삭제되었거나 다른 프로그램이 파일을 열고 있을 수 있습니다.
           *[not_a_profile] 이 파일은 deadloader 프로필이 아닙니다.
        }
    .todo =
        { $reason ->
            [newer] deadloader를 업데이트한 후 프로필을 다시 가져오세요.
            [unreadable] 파일이 그대로 있고 다른 프로그램에서 열려 있지 않은지 확인한 후 다시 시도하세요.
           *[not_a_profile] 올바른 파일을 선택했는지 확인하세요.
        }
e18 = 게임 폴더의 파일을 변경할 수 없습니다.
    .why =
        { $reason ->
            [access_denied] Windows가 게임 폴더에 대한 액세스를 거부했습니다.
            [in_use] 백신 프로그램이나 다른 모드 관리자 등 다른 프로그램이 파일을 사용 중입니다.
           *[other] Windows가 파일을 변경하지 못했습니다.
        }
    .todo = 다른 프로그램을 닫고 다시 시도하세요.
e19 = 이미지를 불러올 수 없습니다.
    .why =
        { $reason ->
            [no_response] 호스트가 응답하지 않았습니다.
            [not_an_image] 이미지 파일이 아닙니다.
            [too_large] 이미지가 너무 큽니다.
           *[other] 호스트가 사용할 수 있는 이미지를 보내지 않았습니다.
        }
    .todo = 자리 표시자는 그대로 유지됩니다. 다시 시도할 수 있습니다.
e20 = 활성화된 모드가 너무 많습니다.
    .why = 게임이 추가로 불러올 수 있는 모드 파일은 { $free }개인데, 이 프로필에는 { $wanted }개가 필요합니다.
    .todo = 일부 모드를 끄거나 애드온을 더 적게 선택하세요.
e21 = ‘{ $name }’ 이름의 프로필이 이미 있습니다.
    .why = 파일 이름과 마찬가지로 이름은 대소문자를 구분하지 않고 비교합니다.
    .todo = 다른 이름을 선택하세요.
e22 = 이 이름은 프로필 이름으로 사용할 수 없습니다: ‘{ $name }’
    .why = Windows에서 파일 이름으로 허용하지 않는 이름입니다(예: CON, 끝에 오는 마침표, \ / : * ? " < > | 중 하나).
    .todo = 다른 이름을 선택하세요.
e23 = deadloader 자체 파일을 읽을 수 없습니다: { $file }
    .why =
        { $reason ->
            [damaged_at] 파일이 손상되었습니다({ $line }행, { $column }열).
            [damaged] 파일이 손상되었습니다.
            [newer] 더 새로운 버전의 deadloader로 만든 파일입니다.
           *[invalid] 파일 내용이 deadloader가 예상한 것과 다릅니다.
        }
    .todo =
        { $kind ->
            [settings] 우선 deadloader가 기본 설정을 사용하고 파일은 그대로 둡니다.
            [manifest] manifest.json.bak을 복원하거나 deadloader를 업데이트하세요.
           *[other] 백업에서 파일을 복원하거나 deadloader를 업데이트하세요.
        }
internal = deadloader 내부에서 문제가 발생했습니다.
    .why = 사용자의 잘못이 아니라 deadloader의 오류입니다.
    .todo = 다시 시도하세요. 문제가 반복되면 세부 정보에 있는 로그 항목과 함께 신고해 주세요.

## Buttons named in the texts above

action-undo = 실행 취소
action-review = 검토
action-retry = 다시 시도
action-retry-now = 지금 다시 시도
action-browse = 찾아보기…
action-verify-in-steam = Steam에서 확인
action-show-differences = 차이점 보기
action-details = 세부 정보
action-repair = 복구
action-copy-link = 링크 복사
action-open-folder = 폴더 열기
action-open-release-page = 릴리스 페이지 열기
action-enable-catalog = 카탈로그 켜기
action-learn-more = 자세히 알아보기
action-view-log = 로그 보기

## Shared pieces

log-entry = 로그 항목 { $id }
list-more = { $count }개 더

## Sizes, labelled like Windows Explorer labels them

size-bytes = { $count }바이트
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
