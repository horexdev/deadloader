# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = 설정
settings-nav = 설정 섹션
settings-reset-nav = 모든 설정 초기화…
settings-default = 기본값: { $value }
settings-restore = 기본값으로 되돌리기
settings-on = 켜짐
settings-off = 꺼짐
settings-cancel = 취소
settings-remove = 제거
settings-remove-item = { $item } 제거
settings-add = 추가
settings-open-folder = 폴더 열기
settings-open-folder-at = 폴더 열기: { $path }
settings-copied = 클립보드에 복사했습니다.
settings-folder = 폴더
settings-example = 예:

settings-section-general = 일반
settings-section-general-lead = 언어, 시작, 확대/축소.
settings-section-game = 게임
settings-section-game-lead = Deadlock이 설치된 위치와 deadloader가 그곳에서 바꿀 수 있는 것.
settings-section-catalog = 카탈로그
settings-section-catalog-lead = GameBanana 카탈로그, 민감한 콘텐츠, 이미지와 링크.
settings-section-library = 라이브러리
settings-section-library-lead = 모드 업데이트, 모드 삭제, 이전 버전.
settings-section-downloads = 다운로드
settings-section-downloads-lead = 다운로드 대기열, 압축 파일, 압축 풀기.
settings-section-network = 네트워크
settings-section-network-lead = deadloader가 GameBanana와 통신하는 방식.
settings-section-cache = 캐시
settings-section-cache-lead = 이 PC에 저장된 썸네일과 카탈로그 페이지.
settings-section-logs = 로그
settings-section-logs-lead = 활동이 무엇을 얼마나 오래 보관하는지.
settings-section-advanced = 고급
settings-section-advanced-lead = 설정 파일과 추가 허용 도메인.
settings-section-about = deadloader 정보
settings-section-about-lead = 버전, 라이선스, 이 다운로드를 확인하는 방법, 데이터 위치.

# General
settings-general-group = 언어와 시작
settings-language = 언어
settings-language-help = 시스템 언어: deadloader가 지원하면 Windows 표시 언어, 아니면 영어입니다.
settings-language-system = 시스템 언어 ({ $language })
settings-startup-profile = 시작 시 프로필
settings-startup-profile-help = deadloader가 시작될 때 선택되는 프로필입니다. 선택만으로는 적용되지 않습니다.
settings-startup-last = 마지막으로 선택한 프로필 ({ $name })
settings-startup-last-plain = 마지막으로 선택한 프로필
settings-app-updates = 시작 시 deadloader 업데이트 확인
settings-app-updates-help = 시작할 때 GitHub(api.github.com)에 새 버전이 있는지 한 번 확인합니다. 업데이트는 항상 직접 해야 합니다. 처음 실행할 때 선택한 항목입니다.
settings-app-updates-local = 사용할 수 없음: 로컬 모드에서는 네트워크 요청을 전혀 하지 않습니다. 카탈로그를 켜면 사용할 수 있습니다(설정 → 카탈로그).
settings-zoom = 확대/축소
settings-zoom-keys-1 = Ctrl + 마우스 휠로도 조절하며,
settings-zoom-keys-2 = 로 확대/축소,
settings-zoom-keys-3 = 으로 초기화합니다.
settings-ctrl = Ctrl
settings-percent = { $value }%

# Game
settings-installs-group = 게임 설치
settings-installs-lead = deadloader는 Steam의 Deadlock 설치를 스스로 찾습니다. 각 프로필은 그중 하나를 사용하며, 프로필에서 바꿀 수 있습니다.
settings-install-library = Steam 라이브러리 { $library }
settings-install-build = 빌드 { $build }
settings-install-used =
    { $count ->
        [0] 사용하는 프로필 없음
       *[other] 프로필 { $count }개가 사용 중: { $names }
    }
settings-install-selected = { $name } (선택됨)
settings-install-applied = { $name } (여기에 적용됨)
settings-install-steam = Steam에서 찾음
settings-install-manual = 직접 추가함
settings-install-known = 아직 deadloader 파일이 남아 있음
settings-install-manual-meta = 직접 추가함
settings-gi-ours = gameinfo.gi: 정상 — deadloader의 줄이 있습니다
settings-gi-wait = gameinfo.gi: 프로필이 사용할 때 설정됩니다
settings-gi-rolled = gameinfo.gi: 원본 — 적용하면 deadloader의 줄을 다시 추가합니다
settings-gi-foreign = gameinfo.gi: 다른 프로그램이 변경함
settings-gi-missing = gameinfo.gi: 찾을 수 없음 — 폴더가 옮겨졌을 수 있습니다
settings-installs-none = 아직 찾은 설치가 없습니다.
settings-add-manual = 직접 추가…
settings-add-manual-hint = 찾지 못한 Deadlock 폴더를 선택하세요.
settings-add-manual-title = Deadlock 폴더 선택
settings-install-added = 설치를 추가했습니다.
settings-install-removed = 목록에서 설치를 제거했습니다.
settings-install-removed-text = 그 폴더에서는 아무것도 바뀌지 않았습니다.
settings-gameinfo-group = 게임 업데이트 후
settings-autofix = gameinfo.gi 자동 수정
settings-autofix-help = Deadlock이 업데이트되면 모드를 불러오라고 알려 주는 게임 파일이 초기화됩니다. deadloader는 파일을 백업하고 자신의 줄을 다시 넣은 뒤 알려 드립니다. 끄면 먼저 배너로 물어봅니다.
settings-danger-group = 되돌리기와 제거
settings-rollback = 선택한 프로필의 게임 폴더 롤백
settings-rollback-help = 선택한 프로필 { $profile }의 설치({ $drive }): deadloader가 넣은 모드 파일 { $files }개와 gameinfo.gi의 줄을 제거합니다. 라이브러리와 프로필은 그대로이며, 적용하면 모두 돌아옵니다.
settings-rollback-help-others = 다른 게임 폴더는 건드리지 않습니다. 롤백하려면 그 폴더를 사용하는 프로필을 선택하세요.
settings-rollback-nothing = { $drive }에는 롤백할 것이 없습니다 — 지금 deadloader 파일이 없습니다.
settings-rollback-no-game = Deadlock을 찾지 못해 롤백할 게임 폴더가 없습니다.
settings-rollback-button = 롤백…
settings-rolled-title = { $drive }의 게임 폴더를 롤백했습니다.
settings-rolled-text =
    { $foreign ->
        [0] { $drive }에서 모드 파일 { $files }개와 gameinfo.gi의 deadloader 줄을 제거했습니다. 라이브러리와 프로필은 그대로입니다 — 적용 또는 플레이로 모드가 돌아옵니다.
       *[other] { $drive }에서 모드 파일 { $files }개, gameinfo.gi의 deadloader 줄, 다른 파일 { $foreign }개를 제거했습니다. 라이브러리와 프로필은 그대로입니다 — 적용 또는 플레이로 모드가 돌아옵니다.
    }
settings-wipe = deadloader 데이터 모두 삭제
settings-wipe-help = 라이브러리, 프로필, 로그, 캐시, 설정({ $size })을 삭제하고 deadloader를 닫습니다. deadloader.exe는 남으니 나중에 직접 삭제하세요.
settings-wipe-button = 모든 데이터 삭제…

# Catalog
settings-catalog-group = GameBanana 카탈로그
settings-catalog = 카탈로그 사용
settings-catalog-help-on = GameBanana의 모드를 찾아보고 설치합니다. deadloader는 GameBanana하고만 통신하며 모든 요청이 활동에 표시됩니다. 꺼짐은 로컬 모드입니다.
settings-catalog-help-off = 꺼짐 — 로컬 모드: deadloader는 네트워크 요청을 전혀 하지 않습니다. 디스크에서 모드를 추가하는 것은 계속 가능합니다.
settings-consent-title = 먼저 한 단계.
settings-consent-text = 카탈로그를 켜기 전에 deadloader가 연결할 호스트와 GameBanana의 약관을 보여 드립니다.
settings-consent-go = 확인하고 켜기
settings-local-on = 로컬 모드가 켜져 있습니다.
settings-local-on-text = 카탈로그가 꺼져 있고 deadloader는 네트워크 요청을 하지 않습니다.
settings-ttl = 불러온 카탈로그 페이지 재사용 시간
settings-ttl-help = 이 시간 안에는 페이지로 돌아가도 GameBanana에 다시 묻지 않습니다.
settings-minutes = { $count }분
settings-hours = { $count }시간
settings-prefetch = 다음 페이지 미리 불러오기
settings-prefetch-help = 결과의 약 70%를 스크롤하면 그리드가 이어지도록 다음 결과를 불러옵니다.
settings-nsfw-group = 민감한 콘텐츠
settings-nsfw = 콘텐츠 등급이 있는 모드(NSFW)
settings-nsfw-help = 숨김: 전혀 표시하지 않습니다. 흐림: "민감한 콘텐츠" 표시가 있는 흐린 이미지이며, 클릭하면 보입니다. 표시: 흐림 없음.
settings-nsfw-age-done = { $date }에 나이를 확인했습니다.
settings-nsfw-age-ask = "숨김"에서 바꾸면 나이 확인을 한 번 요청합니다.
settings-nsfw-hidden = 숨김
settings-nsfw-blurred = 흐림
settings-nsfw-shown = 표시
settings-images-group = 이미지와 링크
settings-description-images = 모드 설명의 이미지
settings-description-images-help = GameBanana가 아닌 호스트(예: i.imgur.com)에서 옵니다. 클릭 시: 클릭할 때까지 자리표시자를 보여 줍니다.
settings-images-click = 클릭 시
settings-images-always = 항상
settings-prechecked = "이 호스트의 이미지를 항상 허용" 미리 선택
settings-prechecked-help = 설명 이미지가 새 호스트에서 처음 올 때 나타나는 대화 상자에서.
settings-hosts = 허용한 이미지 호스트
settings-hosts-help = 이 호스트의 이미지는 묻지 않고 불러옵니다. 활동에서 해당 요청은 "허용한 이미지 호스트"로 표시됩니다.
settings-hosts-built-in = 기본 제공
settings-hosts-none = 추가한 호스트가 없습니다 — 새 호스트는 매번 먼저 묻습니다.
settings-host-removed = { $host } 제거됨
settings-host-removed-text = 이 호스트의 이미지는 불러오기 전에 다시 묻습니다.
settings-undo = 실행 취소
settings-gif = GIF 자동 재생
settings-gif-help = 꺼짐: GIF는 재생 버튼과 함께 첫 프레임을 보여 줍니다.
settings-links = 외부 링크
settings-links-help = 링크는 기본 브라우저에서 열립니다. "먼저 묻기"는 열기 전에 전체 주소를 보여 줍니다.
settings-links-ask = 먼저 묻기
settings-links-open = 바로 열기

# Library
settings-updates-group = 모드 업데이트
settings-check-updates = 시작 시 모드 업데이트 확인
settings-check-updates-help = GameBanana에서 받은 모드를 카탈로그와 비교합니다. 로컬 모드에서는 하지 않습니다.
settings-auto-download = 업데이트 자동 다운로드
settings-auto-download-help = 다운로드만 합니다 — 모드를 새 버전으로 바꾸기 전에는 항상 묻습니다.
settings-disk-group = 삭제와 이전 버전
settings-deletion = 모드를 삭제할 때
settings-deletion-help = 삭제 대화 상자에서 매번 다르게 고를 수 있습니다.
settings-deletion-permanent = 영구 삭제
settings-deletion-bin = 휴지통으로
settings-old-versions = 이전 버전
settings-old-versions-help = 받은 버전은 모두 보관되어 되돌아갈 수 있습니다. 모드의 이전 버전은 세부 정보에서 삭제합니다.
settings-old-keep-all = 모두 보관

# Downloads
settings-queue-group = 대기열
settings-parallel = 동시 다운로드
settings-parallel-help = 동시에 받는 파일 수입니다.
settings-speed = 속도 제한
settings-speed-help = 모드 다운로드에만 적용됩니다 — 카탈로그와 썸네일은 제한하지 않습니다.
settings-speed-none = 제한 없음
settings-speed-value = { $value } MB/s
settings-keep-archives = 설치 후 압축 파일 보관
settings-keep-archives-help = 꺼짐: .vpk 파일이 라이브러리에 들어가면 .zip, .7z, .rar 파일을 삭제합니다.
settings-unpack-group = 압축 풀기
settings-limits = 압축 풀기 한도
settings-limits-help = 풀면 거대해지는 압축 파일로부터 보호합니다. 신뢰하는 압축 파일이 거부되면 한도를 높이세요.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = 푼 크기
settings-limit-ratio = 압축률
settings-limit-files = 압축 파일당 모드 파일 수
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = 개
settings-unit-s = 초
settings-number-error = { $field }: { $min }~{ $max } 사이의 정수를 입력하세요. 그때까지는 이전 값이 쓰입니다.

# Network
settings-network-local = 로컬 모드가 켜져 있습니다.
settings-network-local-text = deadloader는 네트워크 요청을 하지 않습니다. 이 설정은 카탈로그를 켜면 적용됩니다.
settings-requests-group = GameBanana 요청
settings-api-parallel = 동시 카탈로그 요청
settings-api-parallel-help = GameBanana API에 동시에 보내는 요청 수입니다. 이미지와 다운로드는 따로 셉니다.
settings-timeouts = 시간 제한
settings-timeouts-help = 연결: 이 시간 안에 연결되지 않으면 포기합니다. 응답: 서버가 보내기를 멈추면 포기합니다.
settings-timeouts-default = 10초 · 30초
settings-timeout-connect = 연결
settings-timeout-read = 응답
settings-retries = 재시도
settings-retries-help = 실패한 요청을 점점 길어지는 간격으로 반복하는 횟수입니다. 그 뒤 deadloader가 오류를 표시합니다.
settings-connection-group = 연결
settings-proxy = 프록시
settings-proxy-help = 없음: 직접 연결. 시스템: Windows에 설정된 프록시. 수동: 아래에 입력합니다.
settings-proxy-none = 없음
settings-proxy-system = 시스템
settings-proxy-manual = 수동
settings-proxy-host = 주소
settings-proxy-port = 포트
settings-proxy-hint = HTTP 프록시. deadloader 자신의 요청만 이 프록시를 거칩니다.
settings-proxy-invalid = 사용자 이름이나 암호 없이 호스트 이름 또는 주소를 입력하세요(예: proxy.example.com).

# Cache
settings-thumbs-group = 썸네일과 카탈로그 페이지
settings-cache-limit = 캐시 한도
settings-cache-limit-help = 캐시가 가득 차면 가장 오래 보지 않은 썸네일부터 지웁니다.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = 현재 크기
settings-cache-breakdown = 썸네일 { $thumbnails }개 · 카탈로그 페이지 { $pages }개
settings-cache-empty = 비어 있음 — 아직 캐시된 것이 없습니다.
settings-cache-used = { $used } / { $limit }
settings-cache-clear = 캐시 비우기
settings-cache-cleared = 캐시를 비웠습니다 — { $size } 확보. 썸네일은 필요할 때 다시 불러옵니다.
settings-cache-cleared-partly = 사용 중이던 파일 { $failed }개는 다음에 비울 때까지 남습니다.

# Logs
settings-keep-group = 로그 보관
settings-log-days = 로그 보관 기간
settings-log-days-help = 더 오래된 항목은 deadloader가 시작할 때 삭제됩니다.
settings-days = { $count ->
       *[other] { $count }일
    }
settings-year = 1년
settings-log-size = 최대 크기
settings-log-size-help = 이 크기를 넘으면 가장 오래된 로그 파일부터 삭제합니다.
settings-privacy-group = 개인 정보
settings-mask = Windows 사용자 이름 숨기기
settings-mask-help = 로그의 경로에 이름 대신 { $masked }를 표시합니다 — 로그를 공유할 때 더 안전합니다.
settings-logs-folder-group = 로그 폴더
settings-logs-folder = 로그 위치
settings-logs-now = { $size } · 파일 { $files }개 · 가장 오래된 것은 { $date }
settings-open-logs = 로그 폴더 열기

# Advanced
settings-advanced-warning = 숙련된 사용자용입니다.
settings-advanced-warning-text = 여기서의 실수는 deadloader의 동작을 바꿀 수 있습니다. 저장하기 전에 파일을 검사하고 이전 버전을 백업해 둡니다.
settings-files-group = 설정 파일 편집
settings-file-settings = 설정
settings-file-manifest = 게임 폴더 매니페스트
settings-file-label = 편집할 파일
settings-file-settings-lead = 이 페이지들의 모든 내용을 한 파일에 담았습니다. 저장하면 변경 사항이 페이지에 나타납니다.
settings-file-manifest-lead = deadloader가 게임 폴더에 넣은 것의 목록입니다. 롤백이 이 목록에 의존하므로 잘못된 항목이 있으면 파일이 남을 수 있습니다.
settings-json-error = 올바른 JSON이 아니어서 저장하지 않았습니다.
settings-json-where = { $line }행 { $column }열: 이곳이나 바로 앞에 무언가 빠졌거나 남습니다 — 흔히 쉼표나 따옴표입니다.
settings-json-fix = 그 부분을 고친 뒤 다시 저장하세요 — 그때까지 deadloader는 현재 파일을 계속 씁니다.
settings-json-refused = 이 값들은 쓸 수 없어서 아무것도 저장하지 않았습니다.
settings-json-newer = 이 파일은 더 새로운 deadloader가 만든 것이라(형식 { $found }, 이 버전은 { $supported }까지 읽음) 저장하지 않았습니다.
settings-json-invalid = deadloader가 이 파일을 쓸 수 없어서 저장하지 않았습니다.
settings-json-details = 세부 정보
settings-problem-unknown-field = { $path }: 이 버전의 설정이 아닙니다.
settings-problem-invalid-value = { $path }: 이 값은 여기에 쓸 수 없습니다.
settings-problem-out-of-range = { $path }: { $found }이(가) { $min }–{ $max } 범위를 벗어났습니다.
settings-problem-invalid-entry = { $path }: "{ $entry }"은(는) 여기서 유효하지 않습니다.
settings-problem-duplicate-entry = { $path }: "{ $entry }"이(가) 두 번 있습니다.
settings-problem-needs-age = { $path }: NSFW 콘텐츠를 보려면 먼저 나이를 확인해야 합니다(설정 → 카탈로그).
settings-problem-local-mode = { $path }: 로컬 모드에서는 사용할 수 없습니다.
settings-json-idle = 저장할 때 검사합니다.
settings-json-saved = 올바르며 저장했습니다. 이전 버전은 { $backup }(으)로 보관했습니다.
settings-json-saved-plain = 올바르며 저장했습니다.
settings-json-save = 검사 후 저장
settings-json-discard = 편집 내용 버리기
settings-json-backup = 저장하기 전에 이전 파일을 백업합니다. 최근 10개가 남습니다.
settings-domains-group = 추가 허용 도메인
settings-domains-lead = deadloader는 GameBanana하고만 통신합니다. 다운로드가 다른 호스트로 리디렉션되어 차단되면 여기서 그 호스트를 허용할 수 있습니다. 활동에서 그 호스트로의 요청은 "사용자가 추가함"으로 표시됩니다.
settings-domains-built-in = 기본 제공:
settings-domain-user = 사용자가 추가함
settings-domains-none = 추가 도메인이 없습니다.
settings-domain-add = 도메인 추가
settings-domain-hint = 호스트 이름만 입력합니다(예: files.example.com).
settings-domain-empty = 먼저 호스트 이름을 입력하세요(예: files.example.com).
settings-domain-not-host = https://나 경로 없이 호스트 이름만 입력하세요(예: files.example.com).
settings-domain-invalid = "{ $host }"은(는) 호스트 이름이 아닙니다. 철자를 확인하세요(예: files.example.com).
settings-domain-built-in = { $host }은(는) 기본 제공이라 항상 허용됩니다.
settings-domain-listed = { $host }은(는) 이미 목록에 있습니다.

# About
settings-version = 버전 { $version }
settings-commit = 빌드한 커밋
settings-commit-local = 로컬 빌드 — GitHub Actions가 빌드하지 않음
settings-copy-commit = 커밋 ID 복사
settings-release-page = 릴리스 페이지
settings-licence-group = 라이선스
settings-licence-help = deadloader는 자유 소프트웨어입니다. GPL-3에 따라 사용, 연구, 공유, 수정할 수 있습니다. 어떠한 보증도 없습니다.
settings-read-licence = 라이선스 읽기
settings-source = 소스 코드
settings-verify-group = 이 다운로드를 확인하는 방법
settings-verify-lead = 이 deadloader.exe가 GitHub에 게시된 파일인지 확인하는 두 가지 독립적인 방법입니다.
settings-verify-hash = SHA-256 체크섬 비교
settings-verify-hash-help = 릴리스 페이지의 체크섬과 같아야 합니다. 변조된 복사본은 여기에 가짜 값을 보여 줄 수 있으니 직접 계산도 해 보세요:
settings-verify-hash-cmd = exe가 있는 폴더의 PowerShell에서:
settings-verify-attest = 빌드 증명 확인
settings-verify-attest-help = GitHub Actions가 게시된 소스 코드로 이 파일을 빌드했음을 증명합니다. GitHub CLI가 필요합니다:
settings-copy-hash = 체크섬 복사
settings-copy-command = 명령 복사
settings-unsigned = deadloader.exe는 코드 서명되어 있지 않습니다.
settings-unsigned-text = 처음 실행할 때 Windows SmartScreen이 "Windows의 PC 보호"를 표시할 수 있습니다. 위의 두 가지 확인으로 파일이 진짜인지 확인할 수 있습니다.
settings-webview-group = 창 엔진
settings-webview-help = deadloader의 창은 Windows의 일부인 WebView2가 그립니다. Windows가 알아서 업데이트합니다. deadloader가 끄는 것:
settings-webview-off-1 = DNS-over-HTTPS 조회
settings-webview-off-2 = 창 안에서의 Microsoft 계정 로그인
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = 백그라운드 네트워크와 구성 요소 업데이트
settings-webview-off-5 = 양식 자동 완성 서버 호출
settings-webview-off-6 = Microsoft로 보내는 오류 보고(domain reliability)
settings-webview-no-route = 그리고 이 창은 인터넷으로 가는 경로가 전혀 없습니다.
settings-webview-unknown = 버전 알 수 없음
settings-data-group = 내 데이터
settings-data-folder = 데이터 폴더
settings-data-folder-help = deadloader가 보관하는 모든 것은 exe 옆의 이 폴더 하나에 있습니다.
settings-notice = 첫 실행 안내
settings-notice-help = 버전 { $version }, { $date }에 { $language }(으)로 동의함. 선택: 카탈로그 { $catalog }, 업데이트 확인 { $updates }.
settings-notice-none = 아직 동의하지 않았습니다.
settings-notice-again = 다시 읽기

# D7
settings-d7-title = { $drive }의 게임 폴더를 롤백할까요?
settings-d7-from =
    { $others ->
        [0] 선택한 프로필 { $profile }의 설치입니다. deadloader가 다음 위치에서 제거합니다:
       *[other] 선택한 프로필 { $profile }의 설치입니다({ $others }도 사용). deadloader가 다음 위치에서 제거합니다:
    }
settings-d7-files =
    { $count ->
       *[other] 추가한 모드 파일 { $count }개 —
    }
settings-d7-line = gameinfo.gi의 줄
settings-d7-line-why = — 파일이 Steam이 설치한 상태로 돌아갑니다.
settings-d7-stays = 그대로 남는 것:
settings-d7-library = 내 라이브러리 — 모드 { $mods }개, { $size }
settings-d7-profiles =
    { $count ->
       *[other] 내 프로필 { $count }개 — { $names }
    }
settings-d7-other = { $path }의 설치({ $profile }) — 건드리지 않습니다. 롤백하려면 그 설치를 사용하는 프로필을 선택하고 다시 롤백하세요.
settings-d7-other-plain = { $path }의 설치 — 건드리지 않습니다.
settings-d7-back = 적용 또는 플레이로 모두 돌아옵니다.
settings-d7-foreign =
    { $count ->
       *[other] deadloader가 만들지 않은 파일 { $count }개도 제거
    }
settings-d7-foreign-why = 다른 프로그램이나 직접 설치한 파일입니다. 영구 삭제되며 deadloader가 되살릴 수 없습니다.
settings-d7-confirm = 롤백
settings-d7-confirm-foreign =
    { $count ->
       *[other] 롤백하고 파일 { $count }개 삭제
    }

# D8
settings-d8-title = deadloader 데이터를 모두 삭제할까요?
settings-d8-from = deadloader가 다음 폴더 안의 모든 것을 삭제하고
settings-d8-from-2 = 종료합니다.
settings-d8-what = 항목
settings-d8-contains = 내용
settings-d8-size = 크기
settings-d8-total = 합계
settings-d8-library = 라이브러리
settings-d8-library-has = 모드 { $mods }개, 파일 { $files }개
settings-d8-profiles = 프로필
settings-d8-logs = 로그
settings-d8-logs-has = 파일 { $files }개, { $date }부터
settings-d8-logs-none = 로그 파일 없음
settings-d8-cache = 캐시
settings-d8-cache-has = 썸네일과 카탈로그 페이지
settings-d8-settings = 설정
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
       *[other] 백업 { $backups }개 포함
    }
settings-d8-other = 기타
settings-d8-other-has = 덜 받은 다운로드, gameinfo.gi 백업, 창 엔진 데이터
settings-d8-exe = 삭제하지 않음: deadloader.exe 자체 — 더 필요 없으면 직접 삭제하세요.
settings-d8-rollback = 먼저 deadloader가 사용하는 모든 게임 폴더 롤백(권장)
settings-d8-rollback-why = 삭제하기 전에 모든 게임 폴더에서 deadloader의 모드 파일과 gameinfo.gi의 줄을 제거합니다. 다른 파일은 남습니다.
settings-d8-rollback-off = 선택하지 않음: deadloader의 모드 파일이 게임 폴더에 남고 Deadlock이 계속 불러옵니다 — 그것을 제거할 deadloader는 더 이상 없습니다.
settings-d8-type-1 = 확인하려면
settings-d8-type-2 = 을(를) 입력하세요
settings-d8-word = 삭제
settings-d8-hint = 단어가 일치하면 삭제 버튼이 활성화됩니다.
settings-d8-hint-ok = 확인했습니다. 되돌릴 수 없습니다.
settings-d8-type-first = 먼저 "{ $word }"을(를) 입력하세요
settings-d8-deleting = 삭제하는 중…
settings-d8-confirm = 삭제하고 닫기
settings-d8-confirm-rollback = 롤백, 삭제 후 닫기

# Reset
settings-reset-title = 모든 설정을 초기화할까요?
settings-reset-body = 허용한 이미지 호스트와 추가 도메인을 포함해 모든 설정이 기본값으로 돌아갑니다.
settings-reset-keeps = 라이브러리, 프로필, 로그는 그대로이고 게임 폴더도 바뀌지 않습니다.
settings-reset-backup = 현재 설정의 백업을 settings.json.bak-…으로 보관합니다.
settings-reset-confirm = 설정 초기화
settings-reset-done = 모든 설정을 기본값으로 초기화했습니다.
settings-reset-done-text = 백업: { $backup }
