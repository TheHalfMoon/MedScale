//! Spec 032 OS privacy probes (existence / leftover scan only).
//!
//! Never claims `PRIVATE_DATA_READY` or measured swap/hibernate/snapshot qualification.

use std::io::ErrorKind;
use std::path::Path;

use crate::encrypted_vault::EncryptedVault;

/// Best-effort existence result for OS residual privacy surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsFileProbeResult {
    Detected,
    NotFound,
    NotReadable,
    NotApplicable,
}

impl OsFileProbeResult {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Detected => "detected",
            Self::NotFound => "not_found",
            Self::NotReadable => "not_readable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Aggregated privacy probe report for doctor honesty (Spec 032).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivacyProbeReport {
    pub probes_present: bool,
    pub pagefile_existence: OsFileProbeResult,
    pub hibernate_file_existence: OsFileProbeResult,
    pub residual_risk_classes_open: Vec<&'static str>,
    pub vault_leftover_scan_available: bool,
    pub crash_sidecar_detect_available: bool,
}

/// Residual risk classes that remain OPEN after Spec 032 READY_BASE.
#[must_use]
pub fn residual_risk_classes_open() -> Vec<&'static str> {
    vec!["swap", "hibernate", "snapshot", "pagefile"]
}

/// Probe OS residual privacy surfaces (existence only; fail-soft).
#[must_use]
pub fn probe_os_privacy_surfaces() -> PrivacyProbeReport {
    let (pagefile, hibernate) = platform_file_probes();
    PrivacyProbeReport {
        probes_present: true,
        pagefile_existence: pagefile,
        hibernate_file_existence: hibernate,
        residual_risk_classes_open: residual_risk_classes_open(),
        vault_leftover_scan_available: true,
        crash_sidecar_detect_available: true,
    }
}

/// Vault temp/work leftover scan helper (Spec 017 crash leftovers).
#[must_use]
pub fn scan_vault_work_leftovers(root: &Path) -> bool {
    EncryptedVault::leftover_work_present(root)
}

/// Crash sidecar detection alias (WAL/SHM/journal beside meta.work).
#[must_use]
pub fn crash_sidecar_leftovers_present(root: &Path) -> bool {
    EncryptedVault::leftover_work_present(root)
}

fn probe_path(path: &Path) -> OsFileProbeResult {
    match std::fs::metadata(path) {
        Ok(_) => OsFileProbeResult::Detected,
        Err(e) if e.kind() == ErrorKind::NotFound => OsFileProbeResult::NotFound,
        // Windows often returns PermissionDenied for existing pagefile/hiberfil.
        Err(e) if e.kind() == ErrorKind::PermissionDenied => OsFileProbeResult::Detected,
        Err(_) => OsFileProbeResult::NotReadable,
    }
}

#[cfg(windows)]
fn platform_file_probes() -> (OsFileProbeResult, OsFileProbeResult) {
    let pagefile = probe_path(Path::new(r"C:\pagefile.sys"));
    let hiber = probe_path(Path::new(r"C:\hiberfil.sys"));
    (pagefile, hiber)
}

#[cfg(target_os = "linux")]
fn platform_file_probes() -> (OsFileProbeResult, OsFileProbeResult) {
    // Linux has no Windows pagefile; report swap via /proc/swaps as the pagefile-class residual.
    let pagefile = match std::fs::read_to_string("/proc/swaps") {
        Ok(s) => {
            let data_lines = s.lines().skip(1).filter(|l| !l.trim().is_empty()).count();
            if data_lines > 0 {
                OsFileProbeResult::Detected
            } else {
                OsFileProbeResult::NotFound
            }
        }
        Err(e) if e.kind() == ErrorKind::NotFound => OsFileProbeResult::NotFound,
        Err(e) if e.kind() == ErrorKind::PermissionDenied => OsFileProbeResult::NotReadable,
        Err(_) => OsFileProbeResult::NotReadable,
    };
    // Hibernate may use swap; `/sys/power/disk` presence is informational only.
    let hibernate = match std::fs::read_to_string("/sys/power/disk") {
        Ok(s) if s.trim().is_empty() => OsFileProbeResult::NotFound,
        Ok(_) => OsFileProbeResult::Detected,
        Err(e) if e.kind() == ErrorKind::NotFound => OsFileProbeResult::NotFound,
        Err(e) if e.kind() == ErrorKind::PermissionDenied => OsFileProbeResult::NotReadable,
        Err(_) => OsFileProbeResult::NotReadable,
    };
    (pagefile, hibernate)
}

#[cfg(target_os = "macos")]
fn platform_file_probes() -> (OsFileProbeResult, OsFileProbeResult) {
    let vm = Path::new("/private/var/vm");
    let pagefile = match std::fs::read_dir(vm) {
        Ok(entries) => {
            let any_swap = entries
                .filter_map(Result::ok)
                .any(|e| e.file_name().to_string_lossy().starts_with("swapfile"));
            if any_swap {
                OsFileProbeResult::Detected
            } else {
                OsFileProbeResult::NotFound
            }
        }
        Err(e) if e.kind() == ErrorKind::NotFound => OsFileProbeResult::NotFound,
        Err(e) if e.kind() == ErrorKind::PermissionDenied => OsFileProbeResult::NotReadable,
        Err(_) => OsFileProbeResult::NotReadable,
    };
    let hibernate = probe_path(Path::new("/private/var/vm/sleepimage"));
    (pagefile, hibernate)
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
fn platform_file_probes() -> (OsFileProbeResult, OsFileProbeResult) {
    (
        OsFileProbeResult::NotApplicable,
        OsFileProbeResult::NotApplicable,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn probe_report_lists_open_residual_classes_and_never_claims_ready() {
        let r = probe_os_privacy_surfaces();
        assert!(r.probes_present);
        assert!(r.vault_leftover_scan_available);
        assert!(r.crash_sidecar_detect_available);
        assert_eq!(
            r.residual_risk_classes_open,
            vec!["swap", "hibernate", "snapshot", "pagefile"]
        );
        // Existence results are host-dependent; only require a known enum variant.
        assert!(matches!(
            r.pagefile_existence,
            OsFileProbeResult::Detected
                | OsFileProbeResult::NotFound
                | OsFileProbeResult::NotReadable
                | OsFileProbeResult::NotApplicable
        ));
    }

    #[test]
    fn leftover_scan_detects_planted_crash_sidecar() {
        let root = std::env::temp_dir().join(format!("medscale-032-probe-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("meta.work.sqlite3"), b"leftover").unwrap();
        assert!(scan_vault_work_leftovers(&root));
        assert!(crash_sidecar_leftovers_present(&root));
        let _ = fs::remove_dir_all(&root);
    }
}
