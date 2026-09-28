; Gradient was called "Grade 9 Tracker" until v1.29.0. The NSIS installer
; keys its install folder, shortcuts and uninstall entry off the product name,
; so updating from an old build installs Gradient alongside the old copy.
; After Gradient is installed, quietly remove the old copy: its folder, its
; Start menu and desktop shortcuts and its "Installed apps" entry.
;
; Study data is safe. It lives under the bundle identifier
; (uk.alastair.grade9tracker), which did not change, and the old uninstaller
; only deletes it when its "delete app data" box is ticked, which never
; happens in a silent (/S) run. The main binary name (grade9-tracker.exe) is
; also unchanged, and the installer has already closed it before this runs.

!macro NSIS_HOOK_POSTINSTALL
  Push $R0
  Push $R1
  ReadRegStr $R0 SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\Grade 9 Tracker" "UninstallString"
  ReadRegStr $R1 SHCTX "${MANUKEY}\Grade 9 Tracker" ""
  ${If} $R0 != ""
  ${AndIf} $R1 != ""
  ${AndIf} $R1 != $INSTDIR
    ; _?= runs the uninstaller in place and makes ExecWait really wait.
    ExecWait '$R0 /S _?=$R1'
    ; In-place uninstallers cannot delete themselves; tidy up after them.
    Delete "$R1\uninstall.exe"
    RMDir "$R1"
  ${EndIf}
  Pop $R1
  Pop $R0
!macroend
