# Spec 080 Promotion — Governed Browse

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Promotion date:** 2026-09-23
**Canonical base:** `561f97feabe6beb7f079e2af679a4119b50368ce`
**Target branch:** `spec/080-governed-browse`

## Authority

The founder's standing continuation directive requires promoting the next
dependency-ready Research OS unit after each closure without routine
approval. `IMPLEMENTATION_AUTHORITY.md` remains active.

Live verification at promotion time (2026-09-23, `gh pr view` / `gh run view`):

- Spec 079 is `CLOSED_CANONICAL`: final head `0168f05` passed exact-head run
  `35806329807` (6/6); PR #137 merged as `e2a90ba`; post-merge main run
  `35809030069` passed 6/6. Closure PR #138 passed exact-head run
  `35816916622` (6/6 on `945d213`) and merged as `561f97f`.
- Specs 077 and 013 are `CLOSED_CANONICAL` (see `BUILD_QUEUE.md`).

Dependency proof: `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` and
`RESEARCH_OS_EXECUTION_ROADMAP.md` number Governed Browse **080**, with hard
dependency **077 + 079 + the existing Network Broker authority** (Spec 013).
All are closed. `RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md` Q43-Q47 fix the
routing order, credential rule, private-network rule, side-effect rule and
isolation rule this promotion follows.

Review policy: `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Product-network decision recorded here

Spec 080 is the first unit that can send product traffic to the public
network. It does so only when all of these hold, which keeps the
constitutional default-deny posture:

1. the destination host and path prefix are on the Project's Browse
   allowlist, added explicitly by the operator (empty allowlist = every
   request denied before any socket);
2. only `https` on port 443 with a DNS host name; IP-literal and numeric
   hosts, userinfo, other schemes and other ports are denied;
3. every resolved address is re-checked at connect time: loopback,
   private, link-local (including cloud metadata `169.254.169.254`),
   shared/CGNAT, multicast, documentation, benchmarking, reserved,
   unspecified, and IPv6 unique-local/link-local/site-local addresses, plus
   IPv4-mapped, IPv4-compatible and NAT64 forms of them, are refused
   (DNS-rebinding safe: the filtered answer is the one used to connect);
4. redirects are never followed automatically; each hop is re-evaluated
   against rules 1-3, with a limit of 5 redirects;
5. Project content sent out needs a Spec 079 `allow` egress decision for
   boundary `browse`; the URL query string and any search query are
   scanned by the Spec 079 recognizers and denied if a sensitive span is
   found;
6. requests carry no cookies, credentials, proxy or ambient headers.

## Authorized scope

- Contracts: `BrowseRequest`, `BrowseIntentKind`, `BrowsePolicyDecision`,
  `BrowseRoute`, `BrowseRouteStatus`, `BrowseSession`, `BrowseSessionState`,
  `BrowseNavigationStep`, `BrowseDownloadCandidate`, `BrowseEvidenceItem`,
  `BrowseReceipt`, `BrowseSessionView`, `CredentialHandleRef`,
  `HumanTakeoverRequest`, `BrowseAllowlistEntry`.
- Route `http_fetch` implemented through `medscale-network`. Routes
  `search`, `deterministic_browser` and `agentic_browser` exist in the
  vocabulary and report `unavailable` (no admitted search provider; browser
  execution needs the bounded worker of Spec 085, Q47).
- Response limits: 2 MiB, 15 s, content-type allowlist. `text/html`,
  `text/plain` and `application/json` become evidence items (digest, final
  URL, byte length, bounded inert excerpt with instruction-like content
  flagged). `application/pdf` and `text/csv` become quarantined download
  candidates (bytes held, never admitted as artifacts).
- Login walls (401, 407, or 403 with an auth challenge) produce a
  `HumanTakeoverRequest` and state `awaiting_human_takeover`; no credential
  is ever used. `CredentialHandleRef` is an opaque type only.
- Receipts bind the request digest, route, step count, evidence and
  download ids and fixed limitations.
- Storage schema v8 -> v9 (additive), backup/restore, consistency checks.
- CLI vertical slice (`medscale browse ...`) and a Desktop Browse route.

## Explicitly not authorized

- Form submission, purchasing, clinical-system writes or any side effect
  (Q46); only GET exists.
- Browser engines in Core; deterministic or agentic browser execution.
- A search provider (would require provider terms; recorded residual).
- Credential storage or use; cookie jars; browser profile import.
- Institutional or private-network destinations (Q45; needs Spec 087+).
- Model execution over fetched content; content is evidence only and can
  never change instructions or capabilities.
- Real PHI; new third-party dependencies.

## Frozen acceptance requirements

1. Empty allowlist, non-allowlisted host or path, non-https, non-443 port,
   IP literal, userinfo, and private/loopback/link-local/metadata
   resolution all deny before any request is sent, with a persisted
   session.
2. A redirect to a denied target stops the session at that hop; the
   redirect limit is enforced.
3. A URL query or search query with a detected sensitive span is denied;
   Project content goes out only with a Spec 079 `allow` decision.
4. An allowed fetch (scripted transport in CI) produces a receipt, steps,
   a digest-bound evidence item with inert text; instruction-like page text
   is flagged and changes nothing.
5. Oversized, disallowed-type, timed-out and failed responses end in
   explicit states; PDF/CSV downloads stay quarantined candidates.
6. Login walls yield a human-takeover request, never a credential use.
7. Sessions, evidence, downloads and receipts survive reopen and
   backup/restore; tampered rows and content are refused.
8. CLI and Desktop reach Browse only through Core; no network code outside
   `medscale-network`.
9. Exact-head and post-main CI pass.

Recorded residuals: the live `ureq` path to the real public internet is not
exercised in CI (hermetic tests use a scripted transport; the address
filter and URL validation are unit-tested without sockets); URL path
segments are not scanned for sensitive spans because research sites use
identifier-shaped article ids.

## Completion rule

`CLOSED_CANONICAL` only after merge on a green exact head and recorded
post-main verification. Closure of 080 does not authorize 081.
