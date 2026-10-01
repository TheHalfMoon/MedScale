import assert from "node:assert/strict";
import test from "node:test";
import { patientAvailability } from "./availability.ts";

test("a successful shell invoke cannot become a zero-patient successful query", () => {
  const state = patientAvailability({ kind: "available", value: { schemaVersion: 1, coreConnection: "unavailable", detail: "No connection", syntheticOnly: true } });
  assert.equal(state.kind, "unavailable");
  assert.equal(state.total, null);
  assert.equal(state.searchSupported, false);
});

test("loading and bridge failure keep membership unknown", () => {
  assert.equal(patientAvailability({ kind: "loading" }).kind, "checking");
  const failure = patientAvailability({ kind: "error", message: "Unavailable" });
  assert.equal(failure.kind, "unavailable");
  assert.equal(failure.total, null);
  assert.equal(failure.searchSupported, false);
});
