# Clarifications — Spec 033

| Question | Resolution |
|---|---|
| Does try_apply sandbox the parent? | No — AppContainer applies to launched child; documented in limitations. |
| Probe required? | Yes for measured CreateProcess child; doctor still reports in-tree ReadyBaseMeasured. |
| Clear PLATFORM_QUALIFIED? | No. |
| Reuse Job Object target? | No — new `WindowsAppContainerFs` target. |
