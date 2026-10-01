import { MedScaleButton, MedScaleEmptyState, MedScaleRow, MedScaleSearch, MedScaleStatus, MedScaleTable, type LiteralState } from "../components/MedScale";
import { patientAvailability, type ShellDiagnostic } from "./availability";

// A display primitive for a future validated Core projection, never a fixture source.
export type PatientSummary = {
  subjectRef: string;
  displayName: string | null;
  conditions: readonly string[];
  evidenceState: LiteralState;
};

export function PatientRow({ patient }: { patient: PatientSummary }) {
  return <MedScaleRow><th scope="row"><span className="patient-name">{patient.displayName ?? "Name unavailable"}</span></th><td className="exact-identity">{patient.subjectRef}</td><td>{patient.conditions.length ? patient.conditions.join(" · ") : "No supported condition facts"}</td><td><MedScaleStatus state={patient.evidenceState} /></td></MedScaleRow>;
}

export function PatientIndex({ status, onNavigate }: { status: ShellDiagnostic; onNavigate: (id: string) => void }) {
  const availability = patientAvailability(status);
  return <div className="page-container patient-index">
    <div className="patient-index-heading"><div><h1>Patients</h1><p>Patient context, evidence and review.</p></div><div className="patient-total"><span>Patient total</span><MedScaleStatus state="Unknown" explanation="No patient roster has been read." /></div></div>
    <div className="patient-toolbar"><MedScaleSearch label="Search patients" unavailableReason="Patient search is unavailable in this preview." /><MedScaleStatus state="Unavailable" explanation="The workspace is not connected." /></div>
    <MedScaleTable label="Patients — workspace unavailable; total unknown" columns={["Patient", "Subject identifier", "Known conditions", "Evidence state"]}>
      <tr><td className="patient-table-unavailable" colSpan={4}><MedScaleEmptyState icon="patients" title={availability.kind === "checking" ? "Checking workspace availability" : "Patient list unavailable"}><p>No workspace is connected. Patient identity and evidence are unavailable; no patient records have been read.</p></MedScaleEmptyState></td></tr>
    </MedScaleTable>
    <div className="patient-availability-note"><span>Patient total and evidence state remain unknown.</span><span>Synthetic-only preview</span></div>
    <details className="availability-inspector"><summary>Why is the list unavailable?</summary><div><p>This preview has no patient data connection. An unavailable list does not mean the workspace contains no patients.</p><dl><div><dt>Patient records</dt><dd><MedScaleStatus state="Unavailable" /></dd></div><div><dt>Patient search</dt><dd><MedScaleStatus state="Unsupported" /></dd></div><div><dt>Clinical review state</dt><dd><MedScaleStatus state="Unknown" /></dd></div></dl><p>Reference patients are not substituted for workspace records. Workspace access is not available in this build.</p></div></details>
    <MedScaleButton className="return-home" onClick={() => onNavigate("Home")}>Return to Home</MedScaleButton>
  </div>;
}
