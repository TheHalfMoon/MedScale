# Plan — Spec 033 Windows AppContainer FS

## Approach

1. Add `OsSandboxTarget::WindowsAppContainerFs` + `windows_appcontainer_fs_ready_base()`.
2. Implement `windows_appcontainer` module: profile create/derive, CreateProcess with SECURITY_CAPABILITIES, child FS-deny against host-temp marker.
3. Extend probe: `appcontainer-fs` / `appcontainer-fs-child`.
4. Doctor: `windows_appcontainer_fs_measured=true`; `platform_qualified=false`.
5. Evidence + BUILD_QUEUE / EXTERNAL_GATES honesty.

## Non-goals

PlatformQualified gate close; network/LPAC; elevation-required packaging.
