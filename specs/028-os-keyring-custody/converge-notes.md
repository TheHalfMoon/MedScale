# Converge notes — Spec 028

READY_BASE closed when:

1. OsKeyStore + FakeOsKeyStore + Memory tests green
2. Doctor honesty fields present; private_data_ready=false
3. Admission + evidence committed
4. BUILD_QUEUE marks 028 CLOSED_CANONICAL; deferred 029+
5. Workspace fmt/clippy/test --locked PASS

Does not converge PRIVATE_DATA_READY or clear EXTERNAL_GATES fully.
