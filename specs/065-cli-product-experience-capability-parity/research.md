# Research — Spec 065

The CLI already uses `CliSession` / `CoreFacade`, exposes doctor, vault, ingest, timeline, Brief, coverage, Packs, minimum lovable journey, and Host IPC, and has stable exit-code categories. The missing product layer is discoverability and read parity for contracts already present in the authority envelope.

Safe parity candidates are `GetTimeline`, `GetBrief`, `GetCoverage`, `ListOutbox`, `ListDisclosures`, `GetFhirSupportMatrix`, doctor truth, Packs, and Host IPC. Population Insights aggregation and Workflow Studio layout are presentation concerns; the CLI should expose their underlying evidence/action contracts rather than manufacture a second visual or clinical authority.

The canonical topology allows a CLI command to own a transient host when no persistent host exists. The product guide must state this honestly: transient commands do not imply a long-running shared-vault session unless Host IPC is explicitly used.
