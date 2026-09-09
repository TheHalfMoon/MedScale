# Spec 017 evidence (in progress)

**PRIVATE_DATA_READY = FALSE**  
Wipe/detect reduces leftover plaintext after EncryptedVault close; open-while-unlocked work file and OS snapshot/key-custody gaps remain.

## Commands

```text
cargo test -p medscale-storage --test vault_privacy_017
cargo test --workspace
```
