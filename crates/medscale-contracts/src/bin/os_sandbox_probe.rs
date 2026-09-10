//! Spec 030/031/033/038 probe: apply OS sandbox READY_BASE, then prove a denied ambient capability.
//!
//! Exit codes:
//! - 0: apply succeeded and measured deny observed (PASS)
//! - 1: apply failed
//! - 2: apply succeeded but ambient capability was still allowed (FAIL)
//! - 3: not applicable on this host OS
//!
//! Windows AppContainer modes:
//! - `appcontainer-fs-child <marker>`: child entry inside AppContainer (FS)
//! - `appcontainer-fs`: parent measure FS deny
//! - `appcontainer-net-child`: child entry inside AppContainer (network)
//! - `appcontainer-net`: parent measure network deny

fn main() {
    let args: Vec<String> = std::env::args().collect();

    #[cfg(windows)]
    {
        use medscale_contracts::os_sandbox::{
            APPCONTAINER_FS_CHILD_ARG, APPCONTAINER_FS_PARENT_ARG, APPCONTAINER_NET_CHILD_ARG,
            APPCONTAINER_NET_PARENT_ARG, OsSandboxPlan, appcontainer_fs_child_exit_code,
            appcontainer_net_child_exit_code, measure_appcontainer_fs_deny,
            measure_appcontainer_net_deny, try_apply_os_sandbox,
        };
        use std::path::Path;
        use std::process::Command;

        if args.get(1).map(String::as_str) == Some(APPCONTAINER_FS_CHILD_ARG) {
            let marker = args.get(2).map(Path::new).unwrap_or_else(|| Path::new(""));
            std::process::exit(appcontainer_fs_child_exit_code(marker));
        }

        if args.get(1).map(String::as_str) == Some(APPCONTAINER_NET_CHILD_ARG) {
            std::process::exit(appcontainer_net_child_exit_code());
        }

        if args.get(1).map(String::as_str) == Some(APPCONTAINER_FS_PARENT_ARG) {
            let child = std::env::current_exe().unwrap_or_default();
            match measure_appcontainer_fs_deny(&child) {
                Ok(()) => {
                    eprintln!("OK: AppContainer FS deny measured");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("apply/measure failed: {e:?}");
                    std::process::exit(1);
                }
            }
        }

        if args.get(1).map(String::as_str) == Some(APPCONTAINER_NET_PARENT_ARG) {
            let child = std::env::current_exe().unwrap_or_default();
            match measure_appcontainer_net_deny(&child) {
                Ok(()) => {
                    eprintln!("OK: AppContainer network deny measured");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("apply/measure failed: {e:?}");
                    std::process::exit(1);
                }
            }
        }

        let plan = OsSandboxPlan::windows_job_object_ready_base();
        if let Err(e) = try_apply_os_sandbox(&plan) {
            eprintln!("apply failed: {e:?}");
            std::process::exit(1);
        }

        // ACTIVE_PROCESS=1: this process is the sole member; CreateProcess must fail.
        match Command::new("cmd.exe").args(["/C", "exit", "0"]).status() {
            Ok(status) => {
                eprintln!(
                    "FAIL: child process was allowed (exit={status:?}); Job Object did not deny spawn"
                );
                std::process::exit(2);
            }
            Err(err) => {
                eprintln!("OK: child process denied ({err})");
                std::process::exit(0);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use medscale_contracts::os_sandbox::{OsSandboxPlan, try_apply_os_sandbox};
        use std::io::ErrorKind;
        use std::net::{SocketAddr, TcpStream};
        use std::time::Duration;

        let _ = args;

        let plan = OsSandboxPlan::macos_seatbelt_ready_base();
        if let Err(e) = try_apply_os_sandbox(&plan) {
            eprintln!("apply failed: {e:?}");
            std::process::exit(1);
        }

        // Seatbelt (deny network*): connect must fail as EPERM / PermissionDenied.
        // ConnectionRefused would mean the network stack was still reachable (measured FAIL).
        let addr: SocketAddr = "127.0.0.1:9".parse().expect("static loopback discard port");
        match TcpStream::connect_timeout(&addr, Duration::from_millis(800)) {
            Ok(_) => {
                eprintln!("FAIL: TCP connect succeeded; Seatbelt did not deny network");
                std::process::exit(2);
            }
            Err(err) => {
                let denied = err.kind() == ErrorKind::PermissionDenied
                    || err.raw_os_error() == Some(1) // EPERM
                    || err.to_string().contains("Operation not permitted")
                    || err.to_string().contains("Permission denied");
                if denied {
                    eprintln!("OK: network denied ({err})");
                    std::process::exit(0);
                }
                eprintln!(
                    "FAIL: connect error was not a sandbox deny (got {err:?}); network may still be ambient"
                );
                std::process::exit(2);
            }
        }
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    {
        let _ = args;
        eprintln!("medscale-os-sandbox-probe: not applicable on this host");
        std::process::exit(3);
    }
}
