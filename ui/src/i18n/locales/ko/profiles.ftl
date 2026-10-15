# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = 프로필
profiles-count =
    { $count ->
       *[other] 프로필 { $count }개
    }
profiles-in-game = 지금 게임에 적용됨: { $list }
profiles-in-game-item = { $drive }의 “{ $name }”
profiles-in-game-none = 지금 게임에 적용된 프로필이 없습니다.
profiles-import = 가져오기…
profiles-import-title = .json 파일에서 프로필 가져오기
profiles-import-dialog = 프로필 가져오기
profiles-export-dialog = 프로필 내보내기
profiles-file-filter = deadloader 프로필
profiles-new = 새 프로필
profiles-selected = 선택됨
profiles-applied = 적용됨
profiles-not-applied = 아직 적용 안 됨
profiles-selected-title = 라이브러리에서 편집하고 플레이하는 프로필
profiles-pending-title = 적용 또는 플레이를 누르면 게임 폴더에 들어갑니다
profiles-applied-title = 이 프로필의 파일이 지금 { $drive }의 게임 폴더에 있습니다
profiles-select = 선택
profiles-select-of = “{ $name }” 선택
profiles-more-of = “{ $name }”의 다른 작업
profiles-rename = 이름 바꾸기
profiles-duplicate = 복제
profiles-export = 파일로 내보내기…
profiles-change-installation = 게임 설치 위치 변경…
profiles-delete = 삭제
profiles-delete-confirm = 삭제…
profiles-delete-last = deadloader에는 프로필이 하나 이상 있어야 합니다.
profiles-name-label = 프로필 이름
profiles-save = 저장
profiles-cancel = 취소
profiles-close = 닫기
profiles-close-esc = 닫기 (Esc)
profiles-rename-hint = Enter로 저장 · Esc로 취소
profiles-name-empty = 프로필 이름을 입력하세요.
profiles-mods =
    { $count ->
       *[other] 모드 { $count }개 켜짐
    }
profiles-mods-none = 켜진 모드 없음
profiles-applied-when = { $when } 적용됨
profiles-last-applied = 마지막 적용: { $when }
profiles-never-applied = 적용한 적 없음
profiles-just-now = 방금
profiles-preview-empty = 이 프로필을 선택한 다음 라이브러리에서 모드를 켜세요.
profiles-footnote = 프로필은 켜진 모드와 그 순서, 변형, 버전의 목록입니다. 선택해도 아무것도 다운로드하거나 삭제하지 않으며, 적용하면 다른 파일만 바뀝니다.
profiles-copy-name = { $name } (사본)
profiles-copy-name-n = { $name } (사본 { $number })
profiles-new-title = 새 프로필
profiles-start-with = 시작 내용
profiles-empty = 비어 있음
profiles-empty-hint = 켜진 모드 없음 — 나중에 라이브러리에서 고르세요.
profiles-copy-of = “{ $name }”의 사본
profiles-copy-hint = { $mods } — 순서, 변형, 버전이 같습니다.
profiles-installation = 게임 설치 위치
profiles-installation-hint = “{ $name }”와(과) 같습니다. 나중에 프로필 메뉴에서 바꿀 수 있습니다.
profiles-new-note = 프로필을 만들어도 선택되거나 적용되지 않습니다 — 게임은 바뀌지 않습니다.
profiles-create = 프로필 만들기
profiles-pending-what = 프로필 “{ $name }”이(가) 아직 게임에 적용되지 않았습니다.
profiles-pending-why-other = { $drive }의 게임 폴더에는 아직 “{ $other }”이(가) 있습니다. 지금 적용하거나 플레이를 누르세요 — deadloader가 실행 전에 적용합니다.
profiles-pending-why-empty = { $drive }의 게임 폴더에는 지금 deadloader 모드가 없습니다. 지금 적용하거나 플레이를 누르세요 — deadloader가 실행 전에 적용합니다.
profiles-keep = “{ $name }” 유지
profiles-apply = “{ $name }” 적용
profiles-other-what = “{ $name }”은(는) 다른 게임 설치 위치를 사용합니다.
profiles-other-why = 적용과 플레이는 { $path }을(를) 사용합니다. { $drive }의 게임은 “{ $other }”을(를) 유지합니다.
profiles-other-why-empty = 적용과 플레이는 { $path }을(를) 사용합니다. { $drive }의 게임은 바뀌지 않습니다.
profiles-delete-title = 프로필 “{ $name }”을(를) 삭제할까요?
profiles-delete-applied =
    { $count ->
       *[other] “{ $name }”이(가) 지금 게임에 적용되어 있습니다. deadloader가 게임 폴더에서 모드 { $count }개를 제거합니다:
    }
profiles-delete-applied-empty = “{ $name }”이(가) 지금 게임에 적용되어 있지만 모드가 없어 게임 폴더는 바뀌지 않습니다.
profiles-delete-stay = 모드는 라이브러리와 다른 프로필에 남습니다. 그곳에 다른 프로필을 적용할 때까지 Deadlock은 deadloader 모드 없이 실행됩니다.
profiles-delete-next = “{ $name }”이(가) 선택된 프로필이 됩니다. 적용 또는 플레이를 누를 때까지 적용되지 않습니다.
profiles-delete-undoable = 되돌릴 수 없습니다. 사본을 남기려면 먼저 프로필을 내보내세요.
profiles-export-first = 먼저 내보내기
profiles-delete-profile = 프로필 삭제
profiles-installation-title = “{ $name }”의 게임 설치 위치
profiles-installation-lead = 이 프로필을 적용할 Deadlock 폴더를 고르세요.
profiles-used-by =
    { $count ->
       *[other] 프로필 { $count }개가 사용
    }
profiles-used-by-none = 사용하는 프로필 없음
profiles-installation-applied = “{ $name }”이(가) 지금 { $drive }의 게임에 적용되어 있습니다. { $drive }에 다른 프로필을 적용할 때까지 파일은 그대로 남습니다.
profiles-installation-not-applied = “{ $name }”을(를) 적용할 때까지 어느 게임 폴더도 바뀌지 않습니다.
profiles-manage-installations = 설정에서 설치 위치 관리
profiles-use-installation = 이 설치 위치 사용
profiles-d9-title =
    { $count ->
       *[other] “{ $name }”의 모드 { $count }개가 라이브러리에 없습니다
    }
profiles-d9-file = 파일
profiles-d9-lead = 에는 모드가 { $listed }개 있습니다. { $present }개는 이미 라이브러리에 있지만 다음 { $count }개는 없습니다:
profiles-d9-list = 없는 모드
profiles-d9-from-gamebanana = GameBanana { $count }개
profiles-d9-local-count = 로컬 { $count }개
profiles-gamebanana = GameBanana
profiles-can-download = 다운로드 가능
profiles-local-mod = 로컬 모드
profiles-cant-download = 다운로드 불가
profiles-local-why = 다른 PC에서 디스크로 추가한 모드라 GameBanana에 없습니다. 프로필을 공유한 사람에게 파일을 요청하세요.
profiles-d9-local-mode = 카탈로그가 꺼져 있어(로컬 모드) deadloader가 모드를 다운로드할 수 없습니다. 이 모드 없이 가져온 뒤 디스크에서 추가하거나, 설정에서 카탈로그를 켜세요.
profiles-d9-after = 어느 쪽이든 “{ $name }”이(가) 선택된 프로필이 되지만 적용되지는 않습니다 — 적용 또는 플레이를 누를 때까지 게임 폴더는 바뀌지 않습니다.
profiles-import-without = 없이 가져오기
profiles-download-missing = 없는 모드 다운로드 ({ $count })
profiles-turned-off =
    { $count ->
       *[other] 모드 { $count }개 꺼짐
    }
profiles-turned-on =
    { $count ->
       *[other] 모드 { $count }개 켜짐
    }
profiles-differ = 다른 파일만 바꿨습니다.
profiles-same = 같은 모드가 이미 게임에 있었습니다. 순서와 변형만 확인했습니다.
profiles-deleted = 프로필 “{ $name }”을(를) 삭제했습니다.
profiles-deleted-not-applied = 적용되지 않았던 프로필이라 게임 폴더는 바뀌지 않았습니다.
profiles-deleted-applied =
    { $count ->
       *[other] 게임 폴더에서 모드 { $count }개를 제거했습니다. 라이브러리에는 그대로 있습니다.
    }
profiles-exported = 프로필 “{ $name }”을(를) 내보냈습니다.
profiles-imported = 프로필 “{ $name }”을(를) 가져왔습니다.
profiles-imported-text = 선택되었지만 아직 적용되지 않았습니다.
profiles-imported-without =
    { $count ->
       *[other] 프로필 “{ $name }”을(를) 모드 { $count }개 없이 가져왔습니다.
    }
profiles-imported-without-text = 선택되었지만 아직 적용되지 않았습니다. 없는 모드는 나중에 추가할 수 있습니다.
profiles-imported-downloading =
    { $count ->
       *[other] 프로필 “{ $name }”을(를) 가져왔습니다 — 모드 { $count }개를 다운로드하는 중입니다.
    }
profiles-imported-downloading-text = 선택되었지만 아직 적용되지 않았습니다. 다운로드가 끝나면 적용하세요.
profiles-open-downloads = 다운로드 열기
profiles-undo = 실행 취소
profiles-check-update = deadloader 업데이트 확인
profiles-choose-another = 다른 파일 선택
profiles-switcher = 프로필 관리
