# MedScale Research OS Implementation Posture

When future implementation starts:

- prefer extending existing Rust crates/contracts before creating parallel frameworks;
- keep native Desktop and CLI as first-class clients of the same Core;
- put dynamic Python/JS/model ecosystems behind versioned process interfaces when they are materially useful;
- use source code permissions selectively rather than importing applications wholesale;
- require exact receipts for AI/browser/analytics/audio/compute operations that influence research outputs;
- keep external network access deny-by-default unless the active product policy explicitly grants it;
- preserve deterministic, testable paths alongside model-assisted UX.

This posture is intended to maximize long-term replacement freedom: any model, search engine, speech engine, BI tool or collaboration implementation should be replaceable without rewriting the Project/Artifact/Authority model.
