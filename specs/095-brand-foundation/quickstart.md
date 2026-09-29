# Native qualification — Spec 095

Use one qualified CI portable artifact, never parallel cold builds. Verify its manifest source tree equals the candidate tree. The GitHub pull-request merge commit may differ from the candidate commit; the tree equality and required CI head binding must both be recorded.

Run the bundled Python/Pillow runtime with scripts/capture-native-ui.py, an absolute packaged executable, --candidate-head, --expected-tree and a fresh repository output directory. Select Home/light/dark and 1440×900/1100×720; compact is explicit. Each call launches one normal native window, captures it once and exits. The binary uses a separate synthetic vault next to raw captures and no real patient data. PPM/raw output stays ignored; retain only necessary PNGs and small metadata records.

The runner checks actual pixel dimensions and writes binary/source/tree/platform/theme/route/size bindings. Review the resulting native pixels and record inspection separately; a successful capture flag does not mark visual acceptance, WCAG or release readiness. Unknown/denied/unavailable states and keyboard behavior are further qualified by the owning units.
