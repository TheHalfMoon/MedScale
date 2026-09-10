//! Spec 030 probe: apply Windows Job Object READY_BASE, then prove child spawn is denied.
//!
//! Exit codes:
//! - 0: apply succeeded and child process creation was denied (measured PASS)
//! - 1: apply failed
//! - 2: apply succeeded but child process creation was allowed (measured FAIL)
//! - 3: not Windows (probe not applicable)

fn main() {
    #[cfg(not(windows))]
    {
        eprintln!("medscale-os-sandbox-probe: not applicable off Windows");
        std::process::exit(3);
    }

    #[cfg(windows)]
    {
        use medscale_contracts::os_sandbox::{OsSandboxPlan, try_apply_os_sandbox};
        use std::process::Command;

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
}
