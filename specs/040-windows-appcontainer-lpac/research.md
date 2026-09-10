# Research — Spec 040 Windows AppContainer LPAC

## Decision

Use Microsoft-documented LPAC launch:

1. `SECURITY_CAPABILITIES` with AppContainer SID, `CapabilityCount=0`
2. `PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY` =
   `PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT` (`1`)

## Identity proof

LPAC tokens omit ALL APPLICATION PACKAGES (`WinBuiltinAnyPackageSid` / S-1-15-2-1).
Child checks `TokenIsAppContainer` and that ALL_APPLICATION_PACKAGES is **not** an
enabled group — distinguishing LPAC from Spec 033/038 regular AppContainer.

## Threat-model measures (worker)

| Check | Expected |
|---|---|
| LPAC identity | AppContainer + no ALL_APPLICATION_PACKAGES |
| Filesystem | Host-temp marker PermissionDenied |
| Network | TCP to 203.0.113.1:9 → WSAEACCES / access denied |
| Setup failure | Parent ApplyFailed (fail-closed) |

## Out of scope this unit

Full Chromium-style LPAC capability allowlist matrix; parent-process confinement;
named-object ACL suite beyond identity; PLATFORM_QUALIFIED composition.
