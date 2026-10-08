# Research and decisions — Spec 095

## Identity principles, reference only

Public Pentagram work consulted on 2026-09-29:

- https://www.pentagram.com/work/mastercard — preserve approved equity and derive one identity system from a small central geometry.
- https://www.pentagram.com/work/mit-media-lab/story — a shared grid can keep research applications coherent.
- https://www.pentagram.com/work/slack — qualify identity across scale and context.

MedScale applies reduction, systematic proportions, editorial hierarchy and whitespace. No client artwork, glyph library, custom font or layout is copied.

## Zero-cost typography evaluation

| Candidate | Decision for MedScale |
|---|---|
| Inter | Selected UI. Text-oriented optical sizing, tabular numerals and clear letter/number differentiation fit dense clinical rows and the approved reference. |
| Geist Sans | Credible technical UI family; no demonstrated advantage over the approved Inter direction for this product. |
| Instrument Sans | Existing licensed native baseline and viable alternative; more expressive forms depart from the new reference. Historical assets remain preserved. |
| IBM Plex Sans | Strong research/data readability; more pronounced character changes the approved direction. |
| JetBrains Mono | Selected NL Regular for exact evidence IDs, scientific operators and hashes; distinguishable characters with code ligatures absent in the NL build. |
| Geist Mono | Coherent with Geist; not selected because UI uses Inter. |
| IBM Plex Mono | Credible with Plex; not selected because UI uses Inter. |

Official sources: https://rsms.me/inter/ ; https://www.jetbrains.com/lp/mono/ ; https://vercel.com/font ; https://github.com/Instrument/instrument-sans ; https://github.com/IBM/plex . All selected families use SIL OFL. Admission/provenance binds exact binaries and rights separately from these evaluation summaries. Native rendering is measured by the actual candidate captures, not inferred from the font name.

## Runtime approach

Existing Slint 1.16.1 supports registered native fonts, OS theme and Window::take_snapshot. Keep the admitted renderer/runtime. Explicit evidence route/theme/size selects the actual AppWindow. Shared tokens remain native source; documentation/web reference exports are checked against the source.
