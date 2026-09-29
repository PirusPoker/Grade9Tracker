; Gradient was called "Grade 9 Tracker" until v1.29.0. The NSIS installer
; keys its install folder, shortcuts and uninstall entry off the product name,
; so updating from an old build installs Gradient alongside the old copy.
; After Gradient is installed, quietly remove the old copy: its folder, its
; Start menu and desktop shortcuts and its "Installed apps" entry.
;
; Study data is safe. It lives in a folder named after the bundle identifier,
; which the app itself moves across when that changes, and the old uninstaller
; only deletes it when its "delete app data" box is ticked, which never
; happens in a silent (/S) run. The main binary name (grade9-tracker.exe) is
; also unchanged, and the installer has already closed it before this runs.
;
; Updates never create shortcuts: Tauri's template only re-points ones that
; already exist. So the first Gradient install, which arrives as an update,
; made no "Gradient" shortcut, and removing the old copy took away the only
; one there was - leaving nothing to open the app with. Make sure a Start menu
; shortcut exists after every install or update, and replace a desktop
; shortcut if the old copy had one.

!macro NSIS_HOOK_POSTINSTALL
  Push $R0
  Push $R1
  Push $R2

  StrCpy $R2 0
  ${If} ${FileExists} "$DESKTOP\Grade 9 Tracker.lnk"
    StrCpy $R2 1
  ${EndIf}

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

  ${If} $NoShortcutMode <> 1
    !if "${STARTMENUFOLDER}" != ""
      ${IfNot} ${FileExists} "$SMPROGRAMS\$AppStartMenuFolder\${PRODUCTNAME}.lnk"
        CreateDirectory "$SMPROGRAMS\$AppStartMenuFolder"
        CreateShortcut "$SMPROGRAMS\$AppStartMenuFolder\${PRODUCTNAME}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe"
        !insertmacro SetLnkAppUserModelId "$SMPROGRAMS\$AppStartMenuFolder\${PRODUCTNAME}.lnk"
      ${EndIf}
    !else
      ${IfNot} ${FileExists} "$SMPROGRAMS\${PRODUCTNAME}.lnk"
        CreateShortcut "$SMPROGRAMS\${PRODUCTNAME}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe"
        !insertmacro SetLnkAppUserModelId "$SMPROGRAMS\${PRODUCTNAME}.lnk"
      ${EndIf}
    !endif
    ${If} $R2 = 1
    ${AndIfNot} ${FileExists} "$DESKTOP\${PRODUCTNAME}.lnk"
      CreateShortcut "$DESKTOP\${PRODUCTNAME}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe"
      !insertmacro SetLnkAppUserModelId "$DESKTOP\${PRODUCTNAME}.lnk"
    ${EndIf}
  ${EndIf}

  Pop $R2
  Pop $R1
  Pop $R0
!macroend
