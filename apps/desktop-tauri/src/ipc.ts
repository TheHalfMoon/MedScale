export type ShellStatus = {
  schemaVersion: 1;
  coreConnection: "unavailable";
  detail: string;
  syntheticOnly: true;
};

// Reject a mismatched native protocol instead of rendering an assumed safe state.
export function parseShellStatus(value: unknown): ShellStatus {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("Invalid shell status response");
  }
  const status = value as Record<string, unknown>;
  if (status.schemaVersion !== 1 || status.coreConnection !== "unavailable" ||
      status.syntheticOnly !== true || typeof status.detail !== "string" ||
      status.detail.length === 0 || status.detail.length > 256) {
    throw new Error("Unsupported shell status response");
  }
  return { schemaVersion: 1, coreConnection: "unavailable", detail: status.detail, syntheticOnly: true };
}
