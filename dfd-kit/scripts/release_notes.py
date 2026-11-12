#!/usr/bin/env python3
"""Release notes for a tag from CHANGELOG.md and en/CHANGELOG.md, and the release checks.

  release_notes.py [--root DIR] vX.Y.Z           print the notes (Italian, then English)
  release_notes.py [--root DIR] --check vX.Y.Z   exit 1 when the tag cannot be released

The tag must be v<major>.<minor>.<patch>, equal to the version in Cargo.toml, and both
changelogs need a non-empty section `## [X.Y.Z]`. Standard library only.
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

TAG = re.compile(r"^v(\d+\.\d+\.\d+)$")
CHANGELOGS = (("CHANGELOG.md", "Italiano"), ("en/CHANGELOG.md", "English"))


def section(text: str, version: str) -> str | None:
    """Body of `## [version]` up to the next `## [` heading; None if absent."""
    match = re.search(rf"^## \[{re.escape(version)}\][^\n]*\n", text, flags=re.M)
    if not match:
        return None
    rest = text[match.end():]
    following = re.search(r"^## \[", rest, flags=re.M)
    return (rest[: following.start()] if following else rest).strip()


def cargo_version(root: Path) -> str | None:
    match = re.search(r'^version\s*=\s*"([^"]+)"', (root / "Cargo.toml").read_text(), flags=re.M)
    return match.group(1) if match else None


def check(root: Path, tag: str) -> list[str]:
    match = TAG.match(tag)
    if not match:
        return [f"tag non valido: {tag} (atteso v<major>.<minor>.<patch>)"]
    version = match.group(1)
    errors = []
    declared = cargo_version(root)
    if declared != version:
        errors.append(f"il tag {tag} non corrisponde alla versione {declared} di Cargo.toml")
    for name, _ in CHANGELOGS:
        path = root / name
        if not path.is_file():
            errors.append(f"{name} assente")
            continue
        body = section(path.read_text(), version)
        if body is None:
            errors.append(f"{name}: manca la sezione [{version}]")
        elif not re.search(r"^\s*[-*] \S", body, flags=re.M):
            errors.append(f"{name}: la sezione [{version}] non elenca cambiamenti")
    return errors


def notes(root: Path, tag: str) -> str:
    version = TAG.match(tag).group(1)
    parts = []
    for name, language in CHANGELOGS:
        parts.append(f"## {language}\n\n{section((root / name).read_text(), version)}")
    return "\n\n".join(parts) + "\n"


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parent.parent)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("tag")
    args = parser.parse_args(argv)
    errors = check(args.root, args.tag)
    if errors:
        for error in errors:
            print(f"release: {error}", file=sys.stderr)
        return 1
    if args.check:
        print(f"release: {args.tag} pronto")
    else:
        sys.stdout.write(notes(args.root, args.tag))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
