#!/usr/bin/env python3
"""Check the repository's paired public documentation without external packages."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import sys
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "dfd-kit/i18n/documents.json"


def pairs():
    result = []
    for base in (ROOT, ROOT / "dfd-kit"):
        sources = [p for p in base.glob("*.md") if p.name != "AGENTS.md"]
        for folder in ("templates", "examples", "skills"):
            sources.extend((base / folder).rglob("*.md"))
        for source in sorted(sources):
            result.append((source, base / "en" / source.relative_to(base)))
    return result


def anchors(text):
    result = set(re.findall(r'<a\s+id="([^"]+)"', text))
    seen = {}
    fence = False
    for line in text.splitlines():
        if line.startswith("```"):
            fence = not fence
        match = re.match(r"^#{1,6} (.+)$", line)
        if match and not fence:
            slug = re.sub(r"[`*_~]", "", match[1]).lower()
            slug = re.sub(r"[^\w\- ]", "", slug).replace(" ", "-")
            index = seen.get(slug, 0)
            seen[slug] = index + 1
            result.add(slug if index == 0 else f"{slug}-{index}")
    return result


def links(path, text):
    errors = []
    # Ignore illustrative code, inline placeholders and external schemes.
    text = re.sub(r"```[^\n]*\n.*?```", "", text, flags=re.S)
    for target in re.findall(r"\[[^\]\n]+\]\(([^)\n]+)\)", text):
        target = target.strip("<>")
        if re.match(r"^[a-zA-Z][a-zA-Z0-9+.-]*:", target):
            continue
        file, _, fragment = unquote(target).partition("#")
        linked = (path.parent / file).resolve() if file else path
        if not linked.is_relative_to(ROOT):
            errors.append(f"{path.relative_to(ROOT)}: link escapes repository: {target}")
        elif not linked.exists():
            errors.append(f"{path.relative_to(ROOT)}: missing link: {target}")
        elif fragment and linked.suffix == ".md" and fragment not in anchors(linked.read_text()):
            errors.append(f"{path.relative_to(ROOT)}: missing anchor: {target}")
    return errors


def check(update=False):
    errors = []
    snapshot = {}
    paired = pairs()
    expected_english = {english for _, english in paired}
    for base in (ROOT / "en", ROOT / "dfd-kit/en"):
        for english in base.rglob("*.md"):
            if english not in expected_english:
                errors.append(f"Unpaired English document: {english.relative_to(ROOT)}")
    for source, english in paired:
        if not english.is_file():
            errors.append(f"Missing English counterpart: {english.relative_to(ROOT)}")
            continue
        it, en = source.read_text(), english.read_text()
        ids = lambda text: set(re.findall(r"\b(?:OBS|SEC|SLO|COST|UX|COMP)-\d{2,}\b", text))
        if ids(it) != ids(en):
            errors.append(f"Criterion mismatch {source.relative_to(ROOT)}: IT only {sorted(ids(it)-ids(en))}; EN only {sorted(ids(en)-ids(it))}")
        for path, text in ((source, it), (english, en)):
            if text.count("<!-- dfd-language -->") != 1:
                errors.append(f"Missing/duplicate language navigation: {path.relative_to(ROOT)}")
            if len(re.findall(r"^# ", re.sub(r"```[^\n]*\n.*?```", "", text, flags=re.S), re.M)) != 1:
                errors.append(f"Expected one H1: {path.relative_to(ROOT)}")
            if text.count("```") % 2:
                errors.append(f"Unclosed code fence: {path.relative_to(ROOT)}")
            strip_navigation = lambda value: re.sub(r"<!-- dfd-language -->[^\n]*\n|<a id=\"[^\"]+\"></a>\s*", "", value).strip()
            if strip_navigation(it) == strip_navigation(en):
                errors.append(f"Untranslated document: {path.relative_to(ROOT)}")
            errors.extend(links(path, text))
        snapshot[str(source.relative_to(ROOT))] = {
            "english": str(english.relative_to(ROOT)),
            "it_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
            "en_sha256": hashlib.sha256(english.read_bytes()).hexdigest(),
        }
    catalog = json.loads((ROOT / "dfd-kit/i18n/en.json").read_text())
    fields = lambda text: re.findall(r"\{[^{}]*\}", text)
    for source, translated in catalog.items():
        if fields(source) != fields(translated):
            errors.append(f"Message placeholders differ: {source!r}")
    # Compile-time localized format calls must agree with the shared catalog.
    literal = r'"(?:[^"\\]|\\.)*"'
    for path in (ROOT / "dfd-kit/src").glob("*.rs"):
        for match in re.finditer(r"localized_format!\(\s*(" + literal + r")\s*,\s*(" + literal + r")", path.read_text()):
            it, en = json.loads(match[1]), json.loads(match[2])
            if catalog.get(it) != en:
                errors.append(f"Format/catalog mismatch: {path.name}: {it!r}")
    for path in (ROOT / "dfd-kit/src").glob("*.rs"):
        for match in re.finditer(r"(?:Error::message|i18n::text)\(\s*(" + literal + r")", path.read_text()):
            message = json.loads(match[1])
            if message not in catalog:
                errors.append(f"Missing static message translation: {path.name}: {message!r}")
    cli = (ROOT / "dfd-kit/src/cli.rs").read_text()
    for message in re.findall(r"^\s*/// (.*)$", cli, re.M):
        if message not in catalog:
            errors.append(f"Missing help translation: {message!r}")
    if update and not errors:
        MANIFEST.write_text(json.dumps({"schema_version": 1, "documents": snapshot}, ensure_ascii=False, indent=2) + "\n")
    elif not update:
        old = json.loads(MANIFEST.read_text())["documents"] if MANIFEST.exists() else {}
        for name in sorted(snapshot.keys() | old.keys()):
            if snapshot.get(name) != old.get(name):
                errors.append(f"Pair needs semantic review / manifest refresh: {name}")
    for error in errors:
        print(error, file=sys.stderr)
    print(f"{len(paired)} document pairs; {len(catalog)} messages; {len(errors)} errors")
    return 1 if errors else 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--update-manifest", action="store_true", help="Record alignment after semantic review of both languages")
    sys.exit(check(parser.parse_args().update_manifest))
