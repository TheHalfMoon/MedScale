# MedScale Product Truth

## Product
MedScale is a local-first healthcare intelligence workspace for clinicians, care teams, operators, and technical builders. Desktop and CLI are the launch surfaces. Mobile applications come only after Desktop + CLI launch quality is established.

## Core promise
MedScale turns trusted longitudinal health data into understandable, source-aware workflows without weakening authority, privacy, or evidence boundaries.

## Launch surfaces
- **Desktop**: the primary visual product for daily clinical and operational work.
- **CLI**: a first-class, scriptable product with the same authority and capability model as Desktop.
- **Mobile**: deferred until Desktop + CLI are launched; no mobile work is part of Specs 060-067 unless explicitly promoted later.

## Product principles
1. Local-first and private by default.
2. One Rust authority path for Desktop, CLI, and future clients.
3. Evidence and provenance are visible product concepts, not hidden implementation detail.
4. AI may assist, summarize, and propose; authority-changing actions remain explicit.
5. Fast enough to feel immediate, calm enough to support consequential work.
6. Advanced power is discoverable through command palette, keyboard, and CLI rather than visual clutter.
7. Synthetic/permitted fixtures only until REAL_PHI authorization is explicitly granted.

## Experience target
The product should feel as considered as the best modern technical applications: precise, quiet, premium, fast, and trustworthy. The visual direction is founder-approved and combines:
- an original rounded MedScale `M` mark in a Discord-like blurple;
- a Cohere-inspired supporting palette of pine green, coral, lavender, and soft neutrals;
- restrained, Apple-like hierarchy and spacing;
- Linear-like information density and interaction precision;
- modern AI-product discoverability without generic chatbot-first UI.

## User jobs
Desktop and CLI must ultimately cover the same trusted product capabilities: status/privacy inspection, vault/open lifecycle, ingest/import, patient/subject search, longitudinal timeline, brief/summary, coverage, documents/labs, evidence/insights, care-plan workflows, tasks/messages where authorized, audit, exports, backup/recovery, integrations, packs, and operator diagnostics.

## Non-goals for this phase
- No real PHI authorization.
- No product runtime cloud dependency.
- No Tauri/WebView.
- No mobile app implementation before Desktop + CLI launch.
- No claim of WCAG conformance until final product qualification evidence exists.
- No production signing/notarization claims without real credentials.

## Success definition
The Desktop and CLI should feel like two expressions of one product: same data semantics, same authority rules, same vocabulary, same privacy posture, and equivalent power appropriate to each interaction model.
