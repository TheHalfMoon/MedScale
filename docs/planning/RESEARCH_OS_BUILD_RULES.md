# MedScale Research OS Build Rules

**Status:** Planning candidate — not implementation authority

These rules constrain future implementation so the Research OS does not become an unmaintainable aggregation of donor projects.

## 1. Contract before donor

Define the MedScale-owned interface and threat boundary before selecting/copying donor implementation.

## 2. Minimal trusted core

Keep trusted in-process code small. Heavy, dynamic, custom-code, or broad-dependency systems belong behind workers/adapters unless evidence justifies trusted inclusion.

## 3. Local baseline first

Every major feature should have a useful local path before optional server/cloud/institutional acceleration is admitted.

## 4. No silent fallback

Local-to-cloud, GPU-to-CPU, model/provider changes, de-identification failure, and missing evidence must be explicit states.

## 5. Immutable run identity

A run is not identified only by a model name. Bind exact model/runtime, inputs, project context, tool capabilities, policy, and environment facts required by the declared reproducibility level.

## 6. Events do not become facts

Collaboration/activity events can report or reference facts; canonical state changes require Core authority.

## 7. External tools see the minimum

Adapters, browser agents, analytics servers, speech workers, and compute nodes receive only scoped artifacts/capabilities needed for the task.

## 8. Derived indexes are disposable

Search/vector/graph indexes are projections. Canonical artifacts/evidence must be sufficient to rebuild them.

## 9. One product vocabulary

Desktop, CLI, Hub, workers, Packs, agents and adapters use consistent object/authority terms. Do not let donor terminology leak into the product model unless deliberately adopted.

## 10. Preserve partial failure

Fleet runs, workflows, huddles, analytics and compute must preserve successful partial evidence while clearly representing failed/cancelled/denied components.

## 11. Optimize adoption surface

Prefer dependency or selective adaptation over wholesale copying. Prefer a small native implementation when it is simpler to secure and maintain.

## 12. Measure before scale claims

Personal, Lab and Institutional envelopes are separate qualifications. Never infer institution readiness from a working demo.

## 13. Privacy is architecture

PII/PHI handling is not a post-processing plugin. Data classification, egress, logs, caches, workers, collaboration and exports all participate.

## 14. Audio source is evidence

Transcription/cleanup/summarization never destroys the original source relationship. Audio and transcript revisions remain distinguishable.

## 15. Agents remain interruptible

Long-running intelligent actions need cancel/steer/permission transitions that do not depend on the model voluntarily complying.

## 16. Research extensibility uses Packs

New scientific domains should extend stable Core contracts through versioned Packs and adapters rather than adding permanent one-off branches throughout Core.

## 17. Planning language stays honest

Use candidate/proposed/research language until canonical promotion. A planning branch cannot declare a feature delivered, benchmark qualified, or release-ready.
