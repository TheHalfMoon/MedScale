#!/usr/bin/env python3
"""Generate MedScale CSS and Slint token artifacts from the canonical JSON source."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "assets/brand/tokens/medscale.tokens.json"
CSS_OUT = ROOT / "assets/brand/tokens/medscale.css"
SLINT_OUT = ROOT / "crates/medscale-desktop/ui/theme_tokens.slint"


def kebab(name: str) -> str:
    out: list[str] = []
    for ch in name:
        if ch.isupper():
            out.extend(("-", ch.lower()))
        else:
            out.append(ch)
    return "".join(out)


def render_css(data: dict) -> str:
    lines = [
        "/* GENERATED from assets/brand/tokens/medscale.tokens.json. Do not edit by hand. */",
        ":root {",
    ]
    for name, value in data["brand"].items():
        lines.append(f"  --brand-{kebab(name)}: {value};")
    for name, value in data["fixed"].items():
        lines.append(f"  --fixed-{kebab(name)}: {value};")
    for name, value in data["light"].items():
        lines.append(f"  --{kebab(name)}: {value};")
    for name, value in data["semantic"]["light"].items():
        lines.append(f"  --{kebab(name)}: {value};")
    for name, value in data["geometry"].items():
        lines.append(f"  --{kebab(name)}: {value}px;")
    for name, value in data["typography"].items():
        lines.append(f"  --type-{kebab(name)}: {value}px;")
    for name, value in data["spacing"].items():
        lines.append(f"  --space-{kebab(name)}: {value}px;")
    for name, value in data["layout"].items():
        lines.append(f"  --layout-{kebab(name)}: {value}px;")
    for name, value in data["motionMs"].items():
        lines.append(f"  --motion-{kebab(name)}: {value}ms;")
    lines.extend(
        [
            "  --brand-gradient: linear-gradient(135deg, var(--brand-azure) 0%, var(--brand-iris) 50%, var(--brand-pink) 100%);",
            "  --brand-gradient-wide: linear-gradient(105deg, var(--brand-azure) 0%, var(--brand-cobalt) 25%, var(--brand-violet) 55%, var(--brand-magenta) 78%, var(--brand-pink) 100%);",
            "}",
            "",
            ".theme-dark, .reference-dark {",
        ]
    )
    for name, value in data["dark"].items():
        lines.append(f"  --{kebab(name)}: {value};")
    for name, value in data["semantic"]["dark"].items():
        lines.append(f"  --{kebab(name)}: {value};")
    lines.extend(["}", ""])
    return "\n".join(lines)


def render_slint(data: dict) -> str:
    lines = [
        "// GENERATED from assets/brand/tokens/medscale.tokens.json. Do not edit by hand.",
        "export global BrandTokens {",
    ]
    for group in ("brand", "fixed", "light", "dark"):
        for name, value in data[group].items():
            lines.append(f"    out property <color> {group}-{kebab(name)}: {value};")
    for mode in ("light", "dark"):
        for name, value in data["semantic"][mode].items():
            lines.append(f"    out property <color> {mode}-{kebab(name)}: {value};")
    for name, value in data["geometry"].items():
        lines.append(f"    out property <length> {kebab(name)}: {value}px;")
    for name, value in data["typography"].items():
        lines.append(f"    out property <length> type-{kebab(name)}: {value}px;")
    for name, value in data["spacing"].items():
        lines.append(f"    out property <length> space-{kebab(name)}: {value}px;")
    for name, value in data["layout"].items():
        lines.append(f"    out property <length> layout-{kebab(name)}: {value}px;")
    for name, value in data["motionMs"].items():
        lines.append(f"    out property <duration> motion-{kebab(name)}: {value}ms;")
    lines.extend(["}", ""])
    return "\n".join(lines)


def check_or_write(path: Path, expected: str, check: bool) -> bool:
    if check:
        current = path.read_text(encoding="utf-8") if path.exists() else ""
        if current != expected:
            print(f"stale generated token artifact: {path.relative_to(ROOT)}", file=sys.stderr)
            return False
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(expected, encoding="utf-8")
    return True


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if generated artifacts are stale")
    args = parser.parse_args()
    data = json.loads(SOURCE.read_text(encoding="utf-8"))
    ok = True
    ok &= check_or_write(CSS_OUT, render_css(data), args.check)
    ok &= check_or_write(SLINT_OUT, render_slint(data), args.check)
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
