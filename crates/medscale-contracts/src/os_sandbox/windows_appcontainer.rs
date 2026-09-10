//! Windows AppContainer filesystem ReadyBaseMeasured (Spec 033).
//!
//! Measures: create/derive per-user AppContainer profile, launch a child with
//! `SECURITY_CAPABILITIES`, prove the child cannot read a host-temp marker file
//! outside the profile folder. Does **not** claim multi-OS PLATFORM_QUALIFIED
//! or clear EXTERNAL_GATES `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED`.

#![allow(unsafe_code)]

use super::OsSandboxApplyError;
use std::ffi::OsStr;
use std::fs;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;

/// Child argv token: read marker path and exit 0 on PermissionDenied, 2 if readable.
pub const APPCONTAINER_FS_CHILD_ARG: &str = "appcontainer-fs-child";

/// Parent argv token: run measured AppContainer FS deny using this executable as child.
pub const APPCONTAINER_FS_PARENT_ARG: &str = "appcontainer-fs";

fn wide_null(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn path_wide_null(path: &Path) -> Vec<u16> {
    path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Child entry: attempt to read `marker`; exit codes are for the parent CreateProcess wait.
#[must_use]
pub fn appcontainer_fs_child_exit_code(marker: &Path) -> i32 {
    match fs::read(marker) {
        Ok(_) => 2, // FAIL: ambient host file was readable inside AppContainer
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => 0,
        Err(e) if e.raw_os_error() == Some(5) => 0, // ERROR_ACCESS_DENIED
        Err(_) => 1,
    }
}

/// Locate the AppContainer FS child helper executable.
#[must_use]
pub fn resolve_appcontainer_child_exe() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("CARGO_BIN_EXE_medscale-os-sandbox-probe") {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Some(pb);
        }
    }
    if let Ok(cur) = std::env::current_exe() {
        if cur
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("medscale-os-sandbox-probe"))
        {
            return Some(cur);
        }
        let sibling = cur.with_file_name("medscale-os-sandbox-probe.exe");
        if sibling.is_file() {
            return Some(sibling);
        }
    }
    None
}

/// Create AppContainer profile, launch `child_exe` with FS-child args, expect deny.
pub fn measure_appcontainer_fs_deny(child_exe: &Path) -> Result<(), OsSandboxApplyError> {
    if !child_exe.is_file() {
        return Err(OsSandboxApplyError::ApplyFailed(format!(
            "AppContainer FS child exe missing: {}",
            child_exe.display()
        )));
    }

    let pid = std::process::id();
    let profile_name = format!("medscale.ac.033.{pid}");
    let marker = std::env::temp_dir().join(format!("medscale-ac-033-{pid}.marker"));
    fs::write(&marker, b"medscale-033-host-marker").map_err(|e| {
        OsSandboxApplyError::ApplyFailed(format!("failed to plant host marker: {e}"))
    })?;

    let result = measure_with_profile(&profile_name, child_exe, &marker);
    let _ = fs::remove_file(&marker);
    let _ = delete_profile(&profile_name);
    result
}

/// ReadyBaseMeasured apply: resolve probe helper and measure FS deny.
pub(super) fn apply_appcontainer_fs_windows() -> Result<(), OsSandboxApplyError> {
    let child = resolve_appcontainer_child_exe().ok_or_else(|| {
        OsSandboxApplyError::ApplyFailed(
            "AppContainer FS measure needs medscale-os-sandbox-probe on PATH/sibling/CARGO_BIN_EXE"
                .to_owned(),
        )
    })?;
    measure_appcontainer_fs_deny(&child)
}

fn delete_profile(profile_name: &str) -> Result<(), OsSandboxApplyError> {
    use windows_sys::Win32::Security::Isolation::DeleteAppContainerProfile;

    let name = wide_null(profile_name);
    // SAFETY: profile name is a valid NUL-terminated UTF-16 string.
    let hr = unsafe { DeleteAppContainerProfile(name.as_ptr()) };
    if hr < 0 {
        // Best-effort cleanup; ignore "not found".
        let _ = hr;
    }
    Ok(())
}

fn measure_with_profile(
    profile_name: &str,
    child_exe: &Path,
    marker: &Path,
) -> Result<(), OsSandboxApplyError> {
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, LocalFree};
    use windows_sys::Win32::Security::Authorization::ConvertSidToStringSidW;
    use windows_sys::Win32::Security::Isolation::{
        CreateAppContainerProfile, DeriveAppContainerSidFromAppContainerName,
    };
    use windows_sys::Win32::Security::{FreeSid, PSID, SECURITY_CAPABILITIES};
    use windows_sys::Win32::System::Threading::{
        CreateProcessW, DeleteProcThreadAttributeList, EXTENDED_STARTUPINFO_PRESENT,
        GetExitCodeProcess, InitializeProcThreadAttributeList,
        PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, PROCESS_INFORMATION, STARTUPINFOEXW,
        UpdateProcThreadAttribute, WaitForSingleObject,
    };

    let name_w = wide_null(profile_name);
    let display_w = wide_null("MedScale Spec 033 AppContainer");
    let desc_w = wide_null("Synthetic AppContainer FS ReadyBaseMeasured profile");

    let mut sid: PSID = ptr::null_mut();

    // SAFETY: Win32 AppContainer profile APIs; SID freed via FreeSid on all paths below.
    unsafe {
        let hr = CreateAppContainerProfile(
            name_w.as_ptr(),
            display_w.as_ptr(),
            desc_w.as_ptr(),
            ptr::null(),
            0,
            &mut sid,
        );
        // 0x800700B7 = HRESULT_FROM_WIN32(ERROR_ALREADY_EXISTS)
        if hr < 0 {
            if hr == -2147024713i32 {
                let hr2 = DeriveAppContainerSidFromAppContainerName(name_w.as_ptr(), &mut sid);
                if hr2 < 0 || sid.is_null() {
                    return Err(OsSandboxApplyError::ApplyFailed(format!(
                        "DeriveAppContainerSidFromAppContainerName failed: HRESULT=0x{hr2:08X}"
                    )));
                }
            } else {
                return Err(OsSandboxApplyError::ApplyFailed(format!(
                    "CreateAppContainerProfile failed: HRESULT=0x{hr:08X}"
                )));
            }
        } else if sid.is_null() {
            return Err(OsSandboxApplyError::ApplyFailed(
                "CreateAppContainerProfile returned null SID".to_owned(),
            ));
        }

        // Optional: prove folder path API works (existence of profile store).
        let mut sid_str: windows_sys::core::PWSTR = ptr::null_mut();
        if ConvertSidToStringSidW(sid, &mut sid_str) == 0 || sid_str.is_null() {
            FreeSid(sid);
            return Err(OsSandboxApplyError::ApplyFailed(
                "ConvertSidToStringSidW failed".to_owned(),
            ));
        }
        LocalFree(sid_str.cast());

        let mut caps = SECURITY_CAPABILITIES {
            AppContainerSid: sid,
            Capabilities: ptr::null_mut(),
            CapabilityCount: 0,
            Reserved: 0,
        };

        let mut attr_size: usize = 0;
        let _ = InitializeProcThreadAttributeList(ptr::null_mut(), 1, 0, &mut attr_size);
        if attr_size == 0 {
            FreeSid(sid);
            return Err(OsSandboxApplyError::ApplyFailed(
                "InitializeProcThreadAttributeList size query failed".to_owned(),
            ));
        }
        let mut attr_buf = vec![0u8; attr_size];
        let attr_list = attr_buf.as_mut_ptr().cast();
        if InitializeProcThreadAttributeList(attr_list, 1, 0, &mut attr_size) == 0 {
            FreeSid(sid);
            return Err(OsSandboxApplyError::ApplyFailed(
                "InitializeProcThreadAttributeList failed".to_owned(),
            ));
        }

        if UpdateProcThreadAttribute(
            attr_list,
            0,
            PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES as usize,
            (&raw mut caps).cast(),
            size_of::<SECURITY_CAPABILITIES>(),
            ptr::null_mut(),
            ptr::null(),
        ) == 0
        {
            DeleteProcThreadAttributeList(attr_list);
            FreeSid(sid);
            return Err(OsSandboxApplyError::ApplyFailed(
                "UpdateProcThreadAttribute(SECURITY_CAPABILITIES) failed".to_owned(),
            ));
        }

        let mut si: STARTUPINFOEXW = std::mem::zeroed();
        si.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;
        si.lpAttributeList = attr_list;

        let mut pi = PROCESS_INFORMATION {
            hProcess: ptr::null_mut(),
            hThread: ptr::null_mut(),
            dwProcessId: 0,
            dwThreadId: 0,
        };

        let app = path_wide_null(child_exe);
        // Quote paths for CreateProcess command line.
        let cmdline_str = format!(
            "\"{}\" {} \"{}\"",
            child_exe.display(),
            APPCONTAINER_FS_CHILD_ARG,
            marker.display()
        );
        let mut cmdline = wide_null(&cmdline_str);

        let created = CreateProcessW(
            app.as_ptr(),
            cmdline.as_mut_ptr(),
            ptr::null(),
            ptr::null(),
            0,
            EXTENDED_STARTUPINFO_PRESENT,
            ptr::null(),
            ptr::null(),
            &raw const si.StartupInfo,
            &mut pi,
        );

        DeleteProcThreadAttributeList(attr_list);
        FreeSid(sid);

        if created == 0 {
            return Err(OsSandboxApplyError::ApplyFailed(
                "CreateProcessW into AppContainer failed".to_owned(),
            ));
        }

        let process: HANDLE = pi.hProcess;
        let thread: HANDLE = pi.hThread;
        let _ = WaitForSingleObject(process, 30_000);
        let mut code: u32 = 1;
        let _ = GetExitCodeProcess(process, &mut code);
        let _ = CloseHandle(thread);
        let _ = CloseHandle(process);

        match code {
            0 => Ok(()),
            2 => Err(OsSandboxApplyError::ApplyFailed(
                "AppContainer child could read host marker (FS isolation not observed)".to_owned(),
            )),
            other => Err(OsSandboxApplyError::ApplyFailed(format!(
                "AppContainer FS child exited unexpectedly: {other}"
            ))),
        }
    }
}
