import assert from "node:assert/strict";
import test from "node:test";
import { parseShellStatus } from "./ipc.ts";

const preview = { schemaVersion: 1, coreConnection: "unavailable", detail: "Preview has no workspace connection.", syntheticOnly: true };

test("shell response preserves the explicitly limited native protocol", () => {
  assert.deepEqual(parseShellStatus(preview), preview);
});

test("mismatched or malformed responses cannot imply connected or private-data readiness", () => {
  for (const invalid of [null, [], "ready", {}, { ...preview, schemaVersion: 2 },
    { ...preview, coreConnection: "connected" }, { ...preview, syntheticOnly: false },
    { ...preview, detail: "" }, { ...preview, detail: "x".repeat(257) }]) {
    assert.throws(() => parseShellStatus(invalid));
  }
});
