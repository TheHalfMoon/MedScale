# Spec 024 requirements checklist

- [x] Strict mutating session enforcement (default / IPC)
- [x] Local OS IPC transport (pipe/UDS) with length-prefixed JSON
- [x] Two-client revoke/deny evidence across IPC
- [x] WriterLock exclusivity for IPC host
- [x] Doctor honesty: os_ipc_qualified=true; multi_client_release_ready=false
- [x] No MULTI_CLIENT_RELEASE_READY / RELEASE_READY / PRIVATE_DATA_READY claims
- [x] Planning docs: 024 CLOSED; deferred 025+
