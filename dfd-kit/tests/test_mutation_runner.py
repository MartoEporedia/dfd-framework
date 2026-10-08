"""Verify that runner failures cannot inflate the mutation score."""
import importlib.util
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("mutation_runner", Path(__file__).resolve().parents[1] / "scripts/mutation_test.py")
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


class MutationRunnerTests(unittest.TestCase):
    def test_build_errors_are_not_killed_mutants(self):
        self.assertEqual(runner.outcome(101, "error[E0308]: mismatched types"), "build-error")
        self.assertEqual(runner.outcome(101, "test result: FAILED. 0 passed; 1 failed;"), "killed")
        self.assertEqual(runner.outcome(0, "test result: ok. 1 passed;"), "survived")

    def test_timeout_is_reported_and_log_is_kept(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            status, output, _ = runner.run(
                [sys.executable, "-c", "import time; print('started', flush=True); time.sleep(30)"],
                root, os.environ.copy(), 0.5, root / "timeout.log")
            self.assertEqual(status, "timeout")
            self.assertIn("started", output)
            self.assertEqual((root / "timeout.log").read_text(), output)

    def test_ambiguous_mutations_and_source_escape_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "src").mkdir()
            (root / "src/lib.rs").write_text("if false {}\nif false {}\n")
            mutation = {"id": "sample", "file": "src/lib.rs", "before": "if false", "after": "if true", "tests": ["sample"]}
            with patch.object(runner, "PACKAGE", root):
                with self.assertRaisesRegex(ValueError, "ambiguous"):
                    runner.validate_mutations([mutation])
                mutation["file"] = "../outside.rs"
                with self.assertRaisesRegex(ValueError, "outside supported sources"):
                    runner.validate_mutations([mutation])

    def test_ineffective_mutation_is_rejected(self):
        mutation = {"id": "sample", "file": "src/lib.rs", "before": "x", "after": "x", "tests": ["sample"]}
        with self.assertRaisesRegex(ValueError, "ineffective"):
            runner.validate_mutations([mutation])


if __name__ == "__main__":
    unittest.main()
