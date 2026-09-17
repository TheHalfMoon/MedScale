# MedScale Research OS Lab Adoption Journeys

**Status:** Planning candidate — used to test whether the architecture serves real research workflows.

## Journey 1 — Solo health-data researcher

Topology: Personal, offline.

1. Create a Project.
2. Import protocol, papers and synthetic/deidentified dataset.
3. Ask MedAgent to explain schema and propose cohort query.
4. Review generated SQL in Analytics and run it locally.
5. Compare two local model lanes against the derived cohort.
6. Record a voice note through AudioFlow and attach it to the analysis Run.
7. Export a provenance-preserving report bundle.

Success: no Hub or cloud account required.

## Journey 2 — Five-person lab

Topology: Lab Hub + shared GPU workstation.

1. PI creates Project and invites members.
2. Data scientist adds governed dataset artifact.
3. Researcher opens a Room bound to the dataset.
4. MedAgent runs on local or lab GPU worker under scoped capability.
5. Team discusses findings in the same Project history.
6. Audio Huddle captures a meeting; transcript produces tasks and evidence-linked decisions.
7. Privacy Gate blocks an attempted external browse using sensitive context and offers a deidentified route.
8. PI approves an export package.

Success: collaboration does not require a MedScale-hosted cloud and the GPU worker never receives ambient vault access.

## Journey 3 — Medical AI model evaluation team

1. Create AI Research Project.
2. Register benchmark Dataset and exact evaluation protocol.
3. Admit several model Packs.
4. Create a FleetRun with equal input/evidence policy.
5. Compare outputs, abstentions, latency, resource use, evidence coverage and structured validity.
6. Keep unknown dimensions unknown rather than imputing scores.
7. Publish evaluation artifacts and reproducibility manifest.

Success: model consensus is not represented as truth or clinical authority.

## Journey 4 — Interview / qualitative research

1. Create consent-aware Audio Session.
2. Capture/import interviews.
3. Run live or offline transcription and diarization.
4. Preserve transcript revisions and exact source-time mapping.
5. Researchers annotate transcript/audio spans.
6. Project retrieval searches across transcripts, notes and documents under permissions.
7. MedAgent proposes themes with linked excerpts; researcher reviews/accepts findings.

Success: qualitative findings remain traceable to exact audio evidence.

## Journey 5 — Clinical research project

1. Import protocol and permitted/synthetic FHIR data.
2. Build cohort through Analytics Gate.
3. Privacy Gate creates deidentified analytical view.
4. MedAgent reviews evidence and papers; browser access is separately governed.
5. Team opens a Room for an anomalous result.
6. Audio Huddle discusses it; tasks and evidence links are captured.
7. A corrected cohort becomes Dataset v2 with explicit derivation from v1.

Success: conversation, AI output and analytics cannot silently mutate the clinical source record.

## Journey 6 — Wet-lab future Pack

1. Project uses Wet Lab Pack.
2. Create Experiment with protocol, samples, batch and instrument run.
3. Instrument output arrives as Artifact.
4. Researcher records voice observations during bench work.
5. AudioFlow attaches timestamped note to Experiment step/batch.
6. Analytics processes measurements through a bounded pipeline.
7. MedAgent links anomalies to protocol deviations and prior experiments.

Success: domain extension uses Pack schemas while Project/Artifact/Evidence semantics remain unchanged.

## Journey 7 — Institutional research center

Topology: Institutional Hub + identity + worker pool/HPC adapter.

1. Organization identity maps teams/projects without eliminating offline local identity.
2. Projects enforce relationship-aware access.
3. Large datasets stay within institution-approved storage.
4. MedScale sends scoped compute manifests to worker/HPC infrastructure.
5. Only approved outputs return as artifacts.
6. Audit and provenance cover user, agent, workflow and worker actions.
7. Optional BI/RAG services consume approved views/indexes, not unrestricted vault data.

Success: scale changes infrastructure, not authority semantics.

## Product test

Any future major feature should answer: which journey becomes materially better, and can that improvement be achieved without adding a new authority plane? If neither answer is clear, the feature should remain deferred.
