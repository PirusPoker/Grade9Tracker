//! Are we running as administrator, and how to get back down.
//!
//! Nothing here asks for elevation: the app's manifest is asInvoker and the
//! installer is per-user into AppData, so a normal launch is a normal
//! process. But a token is inherited, not chosen - anything started by an
//! elevated shell, installer or updater comes up elevated too, silently and
//! with no visible difference.
//!
//! It matters because an elevated process cannot see a normal-integrity
//! Outlook in the Running Object Table. `GetActiveObject` fails with
//! MK_E_UNAVAILABLE, which reads as "Outlook is not running", so the Outlook
//! panel insists Outlook is closed while it sits open on the taskbar. The
//! user cannot be expected to guess that. So we notice, say so plainly, and
//! offer the one-click way out.
//!
//! Relaunching happens through Explorer, which runs at normal integrity as
//! the shell: it starts the new copy as a child of itself rather than of us,
//! so the new window comes up unelevated however this one was started.

/// True when this process holds an elevated token.
#[cfg(windows)]
pub fn is_elevated() -> bool {
    use std::mem::size_of;
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    use windows_sys::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token: HANDLE = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return false; // no token to read: assume the ordinary case
        }
        let mut info = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut len = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            &mut info as *mut _ as *mut std::ffi::c_void,
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut len,
        );
        CloseHandle(token);
        ok != 0 && info.TokenIsElevated != 0
    }
}

#[cfg(not(windows))]
pub fn is_elevated() -> bool {
    false
}

/// Start a fresh, unelevated copy through Explorer. The caller quits once
/// this returns; the new process is Explorer's child, not ours.
#[cfg(windows)]
pub fn restart_unelevated() -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    let exe = std::env::current_exe().map_err(|e| format!("Could not find the app: {e}"))?;
    std::process::Command::new("explorer.exe")
        .arg(&exe)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("Could not restart through Explorer: {e}"))?;
    Ok(())
}

#[cfg(not(windows))]
pub fn restart_unelevated() -> Result<(), String> {
    Err("Only Windows runs the app elevated.".into())
}

#[cfg(test)]
mod tests {
    /// The check has to be cheap: it runs on every start and the answer is
    /// almost always "no". If this ever takes real time, cache it.
    #[test]
    fn asking_is_quick() {
        let t = std::time::Instant::now();
        for _ in 0..100 {
            let _ = super::is_elevated();
        }
        assert!(t.elapsed().as_millis() < 100, "elevation check is too slow: {:?}", t.elapsed());
    }
}
