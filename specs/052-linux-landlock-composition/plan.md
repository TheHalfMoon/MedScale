# Spec 052 plan

1. Add `OsSandboxTarget::LinuxLandlockComposition` + plan factory.
2. Implement `apply_landlock_composition_linux`: Landlock FS + AccessNet
   BindTcp|ConnectTcp (no NetPort allow rules) + `libc::setrlimit(RLIMIT_NOFILE)`.
3. Extend `medscale-os-sandbox-probe` Linux path to measure composition.
4. Doctor + inventory axis `linux_landlock_composition`.
5. Evidence + limitations (seccomp still open; platform_qualified false).
6. Contract/integration tests; qualify on ubuntu CI.
