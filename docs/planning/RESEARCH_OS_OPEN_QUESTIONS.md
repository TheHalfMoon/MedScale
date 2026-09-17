# MedScale Research OS Open Questions

**Status:** Planning questions that must be answered by evidence before relevant implementation is promoted.

## Core / Project Graph

1. Which current MedScale artifact/domain types should become generic `Artifact` relations, and which must remain specialized Core types?
2. What migration path preserves existing CLI/Desktop semantics without creating two competing project models?
3. Which graph relations are canonical versus derived/indexed projections?

## Collaboration / Hub

4. Does MedScale need cryptographically signed user events for all collaboration state, or is a tamper-evident authenticated server event model sufficient for the initial Hub?
5. Which Buzz components are materially better to adapt than reimplement under MedScale contracts?
6. How much collaboration content should be end-to-end encrypted versus server-readable for search/workflow features?
7. What offline conflict model is acceptable for notes/tasks/rooms/artifact metadata?

## Identity / authorization

8. Can the existing MedScale authority model extend cleanly to organization/team/project/agent relationships, or is a ReBAC engine justified?
9. How are institutional SSO identities bound to local cryptographic/user identities and offline use?
10. What is the revocation behavior for an agent or user with an in-flight run or compute job?

## MedAgent / fleet

11. Which model tasks need same-prompt comparison versus role-specialized lanes?
12. Which comparison metrics are deterministic/observable enough to productize?
13. What context-budget and caching model avoids hidden sensitive retention?
14. Which external delegate adapters are acceptable for public/deidentified work, and how are their capabilities exposed honestly?

## Privacy Gate

15. Which jurisdictions/data classes are product semantics versus deployment policy configured by institutions?
16. When is pseudonymization reversible, who holds mapping keys, and how is re-identification audited?
17. What benchmark corpus can test names, IDs, dates, contacts, locations, medical record identifiers, free text and multilingual PHI without using unauthorized real PHI?

## AudioFlow

18. Which native capture stack is the minimal cross-platform foundation?
19. Which live/offline/medical/Arabic speech routes win measured qualification on target devices?
20. Is a Python speech worker justified for high-quality lanes, and what is the default minimal native runtime?
21. What Audio Pack format reuses the existing MedScale Pack trust model most cleanly?
22. What consent/capture UX is required for meetings and clinical/research conversations in each supported deployment?
23. Which TTS/voice-cloning capabilities belong in the core product versus optional later Packs?

## Analytics

24. Does DataFusion meet required statistical/query features and performance for target workloads, or is a secondary engine required?
25. What notebook model preserves reproducibility without embedding an unrestricted arbitrary-code environment into the trusted Desktop?
26. Which statistical methods should be deterministic native operations versus sandboxed Python/R workers?
27. What is the safe boundary for natural-language-to-SQL and generated analytical code?

## Knowledge / RAG

28. What indexing stack is sufficient for Personal/Lab before OpenSearch-class infrastructure is justified?
29. Which vector database, if any, is needed beyond local embedded indexes?
30. How are re-indexing, stale embeddings and model changes represented in provenance?
31. How does retrieval preserve project/team permissions at query time and cache time?

## Compute

32. Is OpenSandbox the right isolation layer across supported platforms, or should MedScale define a smaller native worker first?
33. How will institutional HPC/Slurm jobs receive and return artifacts without broad vault access?
34. Which workloads need containers, WASM, OS sandboxing, or bare native execution?

## Research Packs

35. Which Pack should be the first proof of domain extensibility after Clinical Research — AI Research, Imaging, Systematic Review, Omics, or Wet Lab?
36. What extension points are required in Desktop without allowing arbitrary untrusted UI code inside the trusted process?
37. How are Pack schemas/version migrations and uninstall behavior handled?

## Federation

38. Is federation required for the first institutional release, or should it remain a later research program?
39. Which computations can be federated while preserving statistical validity and privacy?
40. What trust and policy vocabulary can cross institutions without assuming identical governance?

## Business/product boundary

41. Which capabilities must remain fully open/local versus optional paid institutional packaging/support without compromising the core promise?
42. What is the smallest feature set that makes a lab adopt MedScale daily before institution-scale features exist?

These questions are intentionally unresolved. Future specs should close them with measured evidence rather than architecture-by-preference.
