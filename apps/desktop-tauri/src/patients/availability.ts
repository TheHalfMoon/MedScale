import type { ShellStatus } from "../ipc";

export type ShellDiagnostic = { kind: "loading" } | { kind: "available"; value: ShellStatus } | { kind: "error"; message: string };

export type PatientAvailability = {
  kind: "checking" | "unavailable";
  total: null;
  searchSupported: false;
};

// A shell diagnostic never establishes a successful patient query or roster.
export function patientAvailability(status: ShellDiagnostic): PatientAvailability {
  return { kind: status.kind === "loading" ? "checking" : "unavailable", total: null, searchSupported: false };
}
