//! `MedScale` CLI bootstrap entrypoint.

use medscale_core::CoreFacade;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None | Some("doctor") => {
            print_doctor();
            ExitCode::SUCCESS
        }
        Some("--version" | "-V" | "version") => {
            println!("{}", CoreFacade::version());
            ExitCode::SUCCESS
        }
        Some("--help" | "-h" | "help") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("error: unknown command '{other}'");
            print_help();
            ExitCode::from(2)
        }
    }
}

fn print_help() {
    println!("MedScale CLI (Spec 001 bootstrap)");
    println!();
    println!("Usage:");
    println!("  medscale --version");
    println!("  medscale doctor");
    println!("  medscale help");
}

fn print_doctor() {
    let report = CoreFacade::bootstrap_report();
    println!("{} doctor (bootstrap)", report.identity.product_name);
    println!("version: {}", report.identity.version);
    println!("local_only: {}", report.local_only);
    println!("medical_functionality: {}", report.medical_functionality);
    println!("vault: not_implemented");
    println!("network_broker: not_implemented");
    println!("packs: not_implemented");
    println!("note: product runtime egress remains DEFAULT_DENY");
}

#[cfg(test)]
mod tests {
    use medscale_core::CoreFacade;

    #[test]
    fn version_is_non_empty() {
        assert!(!CoreFacade::version().is_empty());
    }
}
