# MedScale desktop (Tauri)

The Tauri 2 desktop shell over the real MedScale Core. Rust owns the Core session (`CliSession`, in-process `CoreFacade`), the same way the Slint desktop does. The React/TypeScript frontend only renders projections and sends bounded requests. No vault key, passphrase or Core handle crosses into the WebView after a command returns.

Feature surfaces reuse the Slint desktop view models through `crates/medscale-desktop-vm`, so both desktops present the same Core truth:

| Area | Routes |
| --- | --- |
| Clinical | Patients (brief, timeline, labs, coverage, sources), Documents, Insights, Evidence |
| Research | Projects, Data, Browse, Analytics, Knowledge, Research OS |
| Intelligence | Models, MedAgent, Model Fleet |
| Operations | Workflows, Tasks, Messages, Collaboration, Audio |
| Governance | Privacy, Audit Trail, Exports, Integrations, Settings, About |

Workspaces:

- **Synthetic workspace.** Opens a synthetic vault in app-local data. On first open, two synthetic subjects are seeded through governed FHIR ingest and proposal promotion, using the repository fixtures. Browse, ASR and privacy keys use Core's offline fixtures.
- **Encrypted vault.** Created and unlocked with a passphrase, which is not an identity, and gets recovery codes. Core accepts clinical FHIR ingest into synthetic vaults only, so an encrypted vault starts without subjects.

From this directory:

```text
npm ci --ignore-scripts
npm test
npm run build
npm run tauri -- dev
```

Native builds on Windows need Strawberry Perl first on `PATH`, with `PERL` pointing at it, for the vendored OpenSSL used by SQLCipher. The Git-for-Windows msys perl does not work.

## Tests

- `cargo test` in `src-tauri` runs the command layer against a real Core through Tauri's `MockRuntime`.
  - `feature_tests.rs` drives every surface end to end: seed, read, create, execute, compare, lock.
  - It also covers encrypted create, lock, unlock and the wrong-passphrase path.
  - `permission_tests.rs` checks that the capability grants exactly the generated command list.
- `npm test` runs the route and IPC contract tests.

## Visual QA

`feature_tests.rs` can record every command response:

```text
MEDSCALE_UI_FIXTURES=/path/ui-fixtures.json cargo test every_surface_runs_against_real_core
npm run build
node qa/serve.mjs /path/ui-fixtures.json 47124
```

`qa/` replays those real responses into the built frontend, so pages can be captured in a browser without a native shell. The hash selects the start state:

- `#route=Patients&subject=synthetic-subject-ada`
- `&theme=light`
- `&density=compact`
- `&locked=1&gate=access` for the unlock screen

The QA files are never bundled into the app.

## Capability and navigation policy

- Only the main local window has a capability, and it grants exactly the commands listed in `build.rs`.
- No Tauri plugin, filesystem, HTTP, shell, process, clipboard or window-control permission is granted.
- Navigation accepts only the packaged application origins. Debug builds also accept the loopback Vite origin.
- New windows are denied.
- The production CSP has no remote connection origin.
- WebView incognito mode is a mitigation. It does not by itself establish cache, crash, log or storage containment.
- Fonts (Inter, JetBrains Mono NL), icons and the MedScale mark are bundled locally.
- Preferences (theme, density, last project) are the only values kept in WebView storage.

## Qualification status

- Real PHI is not authorized.
- Signing, installers, cross-platform behavior, physical keyboard and screen-reader passes, performance and privacy containment are not qualified.
- Do not infer product or private-data readiness from this desktop.
