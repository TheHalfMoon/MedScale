# MedScale Research OS Product Metrics Framework

**Status:** Planning candidate. Metrics are design inputs, not launch targets until baselines exist.

## Principle

Measure whether MedScale reduces fragmented work while preserving trust. Avoid vanity metrics that reward more AI activity regardless of usefulness.

## Candidate user-value metrics

- time from Project open to first useful/reproducible result;
- proportion of important outputs with inspectable evidence/provenance;
- proportion of analysis/model/audio runs reproducible from retained manifests;
- number of external tools/tabs required for a defined research journey;
- task/decision traceability to artifacts/evidence;
- successful offline completion rate for Personal workflows;
- successful reconnect/recovery for Lab workflows.

## Trust/quality metrics

- unauthorized capability attempt denial rate (must fail closed);
- sensitive-egress prevention coverage;
- unknown/partial states incorrectly rendered as success (target zero);
- stale evidence/index detection;
- model/runtime provenance completeness;
- transcript evidence time-mapping integrity;
- cross-project authorization leakage (target zero in qualification suites).

## MedAgent metrics

- grounded evidence coverage;
- unsupported claim rate on qualified corpora;
- abstention behavior;
- tool-call validity;
- cancel/steer latency;
- FleetRun partial-failure correctness;
- per-lane latency/resource envelope.

## Analytics metrics

- query/result reproducibility;
- generated-query validation failure rate;
- cohort definition round-trip stability;
- source revision binding completeness;
- large-dataset latency/resource envelope.

## AudioFlow metrics

- first partial/final latency;
- WER plus domain-sensitive error classes;
- Arabic/code-switch quality where supported;
- diarization accuracy/stability;
- interruption latency;
- long-session memory growth/dropout/recovery;
- transcript/source alignment integrity.

## Collaboration/Hub metrics

- message/event delivery under declared load;
- reconnect backlog recovery;
- conflict resolution correctness;
- membership/revocation propagation;
- search authorization correctness;
- Hub outage effect on local work.

## Anti-metrics

Do not optimize for:

- raw prompt count;
- total agent messages;
- model agreement percentage as correctness;
- number of integrations;
- number of models installed;
- number of donor projects copied;
- dashboard count.

The product should become simpler to operate even as its capability grows.
