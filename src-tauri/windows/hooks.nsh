; The app has been renamed twice: "Grade 9 Tracker", then "Gradient", now
; "Zelinx Study Planner". The NSIS installer keys its install folder,
; shortcuts and uninstall entry off the product name, so an update installs the
; new name alongside the old copy. Once the new one is in, quietly remove any
; old copy: its folder, its Start menu and desktop shortcuts and its
; "Installed apps" entry.
;
; The old copy is found from its own uninstall entry: UninstallString to run
; and InstallLocation for the folder (written in quotes, which are stripped).
; The publisher changed as well, so the old ${MANUKEY} path cannot be used.
;
; Study data is safe. It lives in a folder named after the bundle identifier,
; which the app itself moves across when that changes, and the old uninstaller
; only deletes it when its "delete app data" box is ticked, which never
; happens in a silent (/S) run. The main binary name (grade9-tracker.exe) is
; also unchanged, and the installer has already closed it before this runs.
;
; Updates never create shortcuts: Tauri's template only re-points ones that
; already exist. So make sure a Start menu shortcut exists after every install
; or update, and replace a desktop shortcut if an old copy had one.

!macro REMOVE_OLD_COPY NAME
  ${If} ${FileExists} "$DESKTOP\${NAME}.lnk"
    StrCpy $R2 1
  ${EndIf}
  ReadRegStr $R0 SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${NAME}" "UninstallString"
  ReadRegStr $R1 SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${NAME}" "InstallLocation"
  StrCpy $R3 $R1 1
  ${If} $R3 == '"'
    StrCpy $R1 $R1 "" 1
    StrCpy $R1 $R1 -1
  ${EndIf}
  ${If} $R0 != ""
  ${AndIf} $R1 != ""
  ${AndIf} $R1 != $INSTDIR
    ; _?= runs the uninstaller in place and makes ExecWait really wait.
    ExecWait '$R0 /S _?=$R1'
    ; In-place uninstallers cannot delete themselves; tidy up after them.
    Delete "$R1\uninstall.exe"
    RMDir "$R1"
  ${EndIf}
!macroend

!macro NSIS_HOOK_POSTINSTALL
  Push $R0
  Push $R1
  Push $R2
  Push $R3

  StrCpy $R2 0
  !if "${PRODUCTNAME}" != "Grade 9 Tracker"
    !insertmacro REMOVE_OLD_COPY "Grade 9 Tracker"
  !endif
  !if "${PRODUCTNAME}" != "Gradient"
    !insertmacro REMOVE_OLD_COPY "Gradient"
  !endif

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

  Pop $R3
  Pop $R2
  Pop $R1
  Pop $R0
!macroend
