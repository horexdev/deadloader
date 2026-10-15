# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = 닫기
library-apply = 적용
library-apply-profile = { $name } 적용
library-discard-all = 모두 버리기
library-keep = { $name } 유지
library-keep-applied = 적용된 변경 사항 유지
library-undo-this = 이 변경 취소
library-undo-all = 모두 취소
library-close-to-apply = Deadlock을 닫으면 변경 사항이 자동으로 적용됩니다
library-play-applies = 플레이를 눌러도 적용됩니다
library-applied-title = 변경 사항 적용됨
library-nothing-pending = 대기 중인 항목 없음
library-all-in-game = { $name } 프로필의 모든 항목이 이미 게임에 있습니다.
library-running-sub = Deadlock이 실행 중이라 Windows가 게임 파일을 잠그고 있습니다. 게임을 닫는 즉시 이 변경 사항이 자동으로 적용됩니다.
library-waiting-sub = 이 변경 사항은 아직 게임 폴더에 없습니다. 지금 적용하거나 플레이를 누르세요.
library-switch-sub = 프로필 전환은 많은 파일을 한꺼번에 바꾸므로 직접 적용할 때까지 기다립니다. 여기서 적용하거나 플레이를 누르세요. deadloader가 실행 전에 적용합니다.
library-applied-sub = { $time }에 Deadlock이 닫혀서, 게임이 실행되는 동안 한 변경 사항 { $count }개를 적용했습니다. 다음 변경 전까지 실행 취소할 수 있습니다.
library-undoing-sub = { $time }에 적용된 변경 사항 { $count }개를 취소했지만 Deadlock이 다시 실행 중입니다. 게임을 닫으면 자동으로 되돌립니다.
library-group-enable = 켜기 · { $count }
library-group-disable = 끄기 · { $count }
library-group-order = 순서 · { $count }
library-group-variant = 변형 · { $count }
library-group-version = 버전 · { $count }
library-group-switch = 프로필 전환 · { $count }
library-group-undo = { $time }에 적용된 변경 사항 { $count }개 취소
library-change-on = 꺼짐 → 켜짐
library-change-off = 켜짐 → 꺼짐
library-change-above = { $name } 위로 이동
library-change-above-wins = { $name } 위로 이동 — 이제 충돌에서 이김
library-change-below = { $name } 아래로 이동
library-switch-what-happens = 모드 { $on }개 켜짐 · { $off }개 꺼짐
library-what-happens = 게임 폴더에서 일어나는 일
library-folder-summary = 파일 { $added }개 추가 · { $removed }개 제거 · { $renumbered }개 번호 변경

## Foreign files
library-foreign-title = 게임의 모드 폴더에서 발견됨
library-foreign-note = deadloader가 넣은 것이 아니며, 직접 선택하지 않는 한 건드리지 않습니다.
library-collapse = 접기
library-expand = 펼치기
library-take = 관리 대상으로 가져오기
library-leave = 그대로 두기
library-left = 그대로 둠
library-foreign-open = { $name }이(가) 있는 폴더 열기
library-foreign-vpk = VPK, 파일 { $count }개
library-foreign-vpk-hero = VPK, 파일 { $count }개 — { $hero } 스킨으로 보입니다
library-foreign-unreadable = deadloader가 읽을 수 없는 모드 파일
library-foreign-replaced = deadloader가 넣은 파일을 대신 차지한 파일
library-foreign-dmm = Deadlock Mod Manager가 만든 설정 파일이며 모드가 아닙니다
library-foreign-other = 모드가 아닌 파일
library-foreign-folder = 폴더, 파일 { $count }개 — 모드 아님
library-foreign-meta = { $size } · { $date }에 추가됨
library-taken = 이제 { $name }이(가) 라이브러리에 있습니다.

## D11
library-d11-title = { $name }을(를) 관리 대상으로 가져올까요?
library-d11-lead = deadloader가 다음을 합니다:
library-d11-step-copy = 로컬 모드로 라이브러리에 복사합니다({ $size }).
library-d11-step-replace = 게임 폴더의 파일을 deadloader가 링크한 사본으로 바꿉니다.
library-d11-step-enable = { $profile } 프로필에서 목록 맨 아래에 두고 켭니다.
library-d11-name = 라이브러리에서의 이름
library-d11-hint = 라이브러리에 원본 파일의 정확한 사본이 보관됩니다.
library-d11-suggested = { $file }({ $hero } 스킨)

## D10
library-d10-title = 다른 프로그램이 게임 파일을 바꿨습니다
library-d10-what = Deadlock이 불러올 폴더를 정하는 게임 파일을 다른 프로그램이 바꿨습니다. 대개 Deadlock Mod Manager입니다. deadloader는 다른 프로그램의 변경을 덮어쓰지 않으므로, 이 문제가 해결될 때까지 모드가 로드되지 않습니다.
library-d10-do = Steam으로 원본 파일을 복원하세요(게임 파일 무결성 확인). 그러면 deadloader가 자기 줄을 추가하고 알려 드립니다.
library-d10-diff = 차이점 보기
library-d10-expected = 예상 내용(원본 + deadloader의 줄)
library-d10-found = 현재 내용
library-d10-diff-note = 강조 표시: deadloader가 추가하는 줄과 다른 프로그램이 추가한 줄.
library-d10-help = 이 파일은 무엇인가요?
library-d10-help-text = gameinfo.gi는 Deadlock이 불러올 폴더를 알려 줍니다. 모드 관리자는 줄 하나를 추가해 게임이 addons 폴더에서 모드를 불러오게 합니다. 게임 업데이트가 이 줄을 초기화하면 deadloader가 자동으로 고칩니다. 단, 파일이 원본 상태일 때만 그렇습니다.
library-d10-wait-title = Steam이 Deadlock의 파일을 확인하고 있습니다.
library-d10-wait-text = Steam이 끝나면 deadloader로 돌아오세요. 파일을 자동으로 다시 확인합니다. 지금 확인할 수도 있습니다.
library-d10-fixed = gameinfo.gi가 설정되었습니다 — 모드가 다시 로드됩니다.
library-d10-fixed-text = Steam이 원본 파일을 복원했고 deadloader가 자기 줄을 추가했습니다. 자세한 내용은 활동 → 파일에 있습니다.
library-open-file = 파일 열기
library-not-now = 나중에
library-verify-steam = Steam에서 게임 파일 확인
library-check-again = 다시 확인
library-show-differences = 차이점 보기

## D5
library-update = 업데이트
library-update-on-gb = GameBanana에 새 버전이 있습니다.
library-later = 나중에
library-d5-title = { $name }을(를) 업데이트할까요?
library-d5-installed = 설치됨
library-d5-new = GameBanana의 새 버전
library-d5-downloaded = 다운로드됨
library-d5-arrived = 라이브러리에 있지만 아직 사용하지 않음
library-d5-new-meta = { $date }에 업데이트됨 · 다운로드 { $size }
library-d5-download = 다운로드 { $size }
library-d5-changed = 새 버전은 파일 구성이 다릅니다. deadloader가 다음과 같이 짝지었습니다:
library-d5-same = 이 파일들은 새 버전으로 교체됩니다:
library-d5-map-text = deadloader가 모든 파일에 대해 어떤 새 파일이 대신하는지 알 수 없었습니다. 파일마다 새 파일을 고르거나 이전 파일을 유지하세요.
library-d5-map-label = 파일 짝짓기
library-d5-col-old = 설치된 파일
library-d5-col-new = 새 파일
library-d5-pick = { $file }의 새 파일
library-d5-choose = 선택…
library-d5-keep = 이전 파일 유지
library-d5-tag-replaces = 교체
library-d5-tag-same = 같은 이름
library-d5-tag-new = 새 파일 · 모드 페이지에 있음
library-d5-profiles = { $old }을(를) 쓰는 모든 프로필에서 { $new }이(가) 사용됩니다. { $old }은(는) 라이브러리에 남아 있어 모드 세부 정보에서 다시 전환할 수 있습니다.
library-d5-install = 업데이트 설치
library-d5-confirm-map = 이 파일 사용
library-update-downloading = { $name } 업데이트를 다운로드하는 중입니다. 도착하면 설치됩니다.
library-updated = { $name }을(를) 업데이트했습니다.
library-updated-text = 이전 버전을 쓰던 모든 프로필에서 사용됩니다. 이전 버전은 보관되므로 모드 세부 정보에서 다시 전환할 수 있습니다.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name } 켜짐
library-n12-off = { $name } 꺼짐
library-n12-above = { $name }: { $other } 위로 이동
library-n12-below = { $name }: { $other } 아래로 이동
library-n12-switched = { $name }: { $to }(으)로 전환
library-n12-files = { $name }: 파일 변경됨

## S9: the file list of a pending apply
library-show-file-list = 파일 목록 보기
library-hide-file-list = 파일 목록 숨기기
library-file-list = 게임 폴더의 파일
library-files-added = 추가 · { $count }
library-files-removed = 제거 · { $count }
library-files-renumbered = 번호 변경 · { $count }
