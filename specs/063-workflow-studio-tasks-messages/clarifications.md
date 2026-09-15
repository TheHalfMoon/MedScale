# Clarifications — Spec 063

- Workflow Studio is review/presentation UX in this slice, not a new executable workflow runtime.
- Tasks are derived from existing outbox/effect state and are not a new canonical object class.
- Messages are local previews/status explanations only; no SMS/email/WhatsApp/NPHIES transport is authorized.
- UNKNOWN always requires reconciliation; blind retry remains forbidden.
- Payload digest is identity evidence, not proof that an action was clinically appropriate.
- Spec 012/MESC remains optional/deferred and untouched.
