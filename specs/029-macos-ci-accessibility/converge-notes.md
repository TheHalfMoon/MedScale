# Converge notes — Spec 029

READY_BASE closed when:

1. CI matrix includes macos-latest with --locked (and macOS build deps step)
2. Doctor accessibility READY_BASE honest (no WCAG / no RELEASE_READY)
3. macos_ci_present=true; macos_qualified=false; release_ready=false
4. FixtureUi/CLI honesty tests green
5. Evidence + BUILD_QUEUE mark 029 CLOSED_CANONICAL; deferred 030+
6. Workspace fmt/clippy/test --locked PASS on Windows; macOS proven in CI

Does not converge RELEASE_READY, WCAG, PRIVATE_DATA_READY, or MULTI_CLIENT_RELEASE_READY.
