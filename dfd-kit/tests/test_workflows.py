"""Structural checks of the GitHub Actions workflows (standard library only, no YAML parser)."""
import re
import unittest
from pathlib import Path

WORKFLOWS = Path(__file__).resolve().parents[2] / ".github" / "workflows"


def jobs(text):
    """Job name -> body, for top-level jobs indented by two spaces under `jobs:`."""
    body = text.split("\njobs:\n", 1)[1]
    parts = re.split(r"^  ([A-Za-z0-9_-]+):\n", body, flags=re.M)
    return dict(zip(parts[1::2], parts[2::2]))


class WorkflowsTest(unittest.TestCase):
    def test_ci_runs_on_main_pull_requests_and_as_reusable_workflow(self):
        ci = (WORKFLOWS / "dfd-kit.yml").read_text()
        for trigger in ("pull_request:", "workflow_dispatch:", "workflow_call:"):
            self.assertIn(trigger, ci)
        self.assertRegex(ci, r"push:\n\s+branches: \[main\]")
        self.assertRegex(ci, r"(?m)^permissions:\n  contents: read$")
        self.assertEqual(set(jobs(ci)), {"quality", "mutations", "binaries"})
        self.assertNotIn("contents: write", ci)

    def test_release_checks_first_and_only_publish_can_write(self):
        release = (WORKFLOWS / "release.yml").read_text()
        self.assertRegex(release, r"push:\n\s+tags: \['v\*\.\*\.\*'\]")
        self.assertRegex(release, r"(?m)^permissions:\n  contents: read$")
        found = jobs(release)
        self.assertEqual(list(found), ["check", "ci", "publish"])
        self.assertIn("release_notes.py --check", found["check"])
        self.assertIn("uses: ./.github/workflows/dfd-kit.yml", found["ci"])
        self.assertIn("needs: check", found["ci"])
        self.assertRegex(found["publish"], r"needs: \[check, ci\]")
        self.assertEqual(release.count("contents: write"), 1)
        self.assertIn("contents: write", found["publish"])
        self.assertIn("SHA256SUMS", found["publish"])
        self.assertIn("gh release create", found["publish"])
        self.assertIn("--notes-file", found["publish"])
        self.assertNotIn("--clobber", release)

    def test_third_party_actions_have_a_fixed_version(self):
        for path in WORKFLOWS.glob("*.yml"):
            for action in re.findall(r"uses: ([^\s]+)", path.read_text()):
                if not action.startswith("./"):
                    with self.subTest(workflow=path.name, action=action):
                        self.assertRegex(action, r"@v\d+$|@[0-9a-f]{40}$|@stable$")


if __name__ == "__main__":
    unittest.main()
