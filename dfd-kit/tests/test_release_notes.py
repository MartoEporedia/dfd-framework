"""Release notes and release checks taken from CHANGELOG.md / en/CHANGELOG.md."""
import importlib.util
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parent.parent / "scripts" / "release_notes.py"


def load():
    spec = importlib.util.spec_from_file_location("release_notes", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


IT = """# Changelog

## [Unreleased]

### Aggiunto

- Lavoro non ancora rilasciato.

## [0.2.0] - 2026-11-20

### Aggiunto

- Casi di accettazione nel design.

## [0.1.0] - 2026-11-12

### Aggiunto

- Prima release.
"""
EN = IT.replace("Aggiunto", "Added").replace("Lavoro non ancora rilasciato.", "Unreleased work.").replace(
    "Casi di accettazione nel design.", "Acceptance cases in the design."
).replace("Prima release.", "First release.")


class ReleaseNotesTest(unittest.TestCase):
    def project(self, version="0.1.0", it=IT, en=EN):
        root = Path(tempfile.mkdtemp())
        (root / "en").mkdir()
        (root / "Cargo.toml").write_text(f'[package]\nname = "dfd-kit"\nversion = "{version}"\n')
        if it is not None:
            (root / "CHANGELOG.md").write_text(it)
        if en is not None:
            (root / "en" / "CHANGELOG.md").write_text(en)
        return root

    def test_notes_take_the_section_in_both_languages(self):
        notes = load().notes(self.project(), "v0.1.0")
        self.assertIn("Prima release.", notes)
        self.assertIn("First release.", notes)
        self.assertLess(notes.index("Prima release."), notes.index("First release."))
        self.assertNotIn("Casi di accettazione", notes)
        self.assertNotIn("## [0.1.0]", notes)

    def test_section_stops_at_the_next_version(self):
        notes = load().notes(self.project(version="0.2.0"), "v0.2.0")
        self.assertIn("Casi di accettazione nel design.", notes)
        self.assertNotIn("Prima release.", notes)
        self.assertNotIn("Lavoro non ancora rilasciato.", notes)

    def test_check_accepts_a_consistent_tag(self):
        self.assertEqual(load().check(self.project(), "v0.1.0"), [])

    def test_check_rejects_bad_tags_versions_and_sections(self):
        rn = load()
        cases = [
            (self.project(), "0.1.0", "tag"),
            (self.project(), "v0.1", "tag"),
            (self.project(), "vUnreleased", "tag"),
            (self.project(), "v0.3.0", "Cargo.toml"),
            (self.project(version="0.3.0"), "v0.3.0", "CHANGELOG.md"),
            (self.project(it=None), "v0.1.0", "CHANGELOG.md"),
            (self.project(en=None), "v0.1.0", "en/CHANGELOG.md"),
            (self.project(en=EN.replace("- First release.\n", "")), "v0.1.0", "en/CHANGELOG.md"),
        ]
        for root, tag, expected in cases:
            with self.subTest(tag=tag, expected=expected):
                errors = rn.check(root, tag)
                self.assertTrue(errors, f"{tag}: nessun errore")
                self.assertIn(expected, " ".join(errors))

    def test_command_line_exit_codes(self):
        root = self.project()
        ok = subprocess.run([sys.executable, str(SCRIPT), "--root", str(root), "--check", "v0.1.0"], capture_output=True, text=True)
        self.assertEqual(ok.returncode, 0, ok.stderr)
        ko = subprocess.run([sys.executable, str(SCRIPT), "--root", str(root), "--check", "v9.9.9"], capture_output=True, text=True)
        self.assertEqual(ko.returncode, 1)
        self.assertIn("Cargo.toml", ko.stderr)
        notes = subprocess.run([sys.executable, str(SCRIPT), "--root", str(root), "v0.1.0"], capture_output=True, text=True)
        self.assertEqual(notes.returncode, 0, notes.stderr)
        self.assertIn("First release.", notes.stdout)


if __name__ == "__main__":
    unittest.main()
