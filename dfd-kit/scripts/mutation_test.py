#!/usr/bin/env python3
"""Run curated Rust mutations in an isolated copy, using Python's standard library."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import tempfile
import time

PACKAGE = Path(__file__).resolve().parents[1]
SOURCES = {"src/lib.rs", "src/store.rs", "src/development.rs", "src/release.rs"}


def outcome(returncode, output):
    if returncode == 0:
        return "survived"
    if "test result: FAILED." in output:
        return "killed"
    return "build-error"


def run(command, workspace, env, timeout, log):
    start = time.monotonic()
    process = subprocess.Popen(command, cwd=workspace, env=env, stdout=subprocess.PIPE,
                               stderr=subprocess.STDOUT, text=True, start_new_session=os.name == "posix")

    def stop():
        try:
            if os.name == "posix":
                os.killpg(process.pid, signal.SIGKILL)
            else:
                process.kill()
        except ProcessLookupError:
            pass

    try:
        output, _ = process.communicate(timeout=timeout)
        status = outcome(process.returncode, output)
    except subprocess.TimeoutExpired:
        stop()
        output, _ = process.communicate()
        status = "timeout"
    except BaseException:
        stop()
        process.communicate()
        raise
    log.write_text(output, encoding="utf-8")
    return status, output, round(time.monotonic() - start, 2)


def validate_mutations(mutations):
    seen = set()
    if not mutations:
        raise ValueError("No mutations selected")
    for mutation in mutations:
        ident = mutation["id"]
        if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", ident) or ident in seen:
            raise ValueError(f"Invalid or duplicate mutation ID: {ident}")
        seen.add(ident)
        if mutation["file"] not in SOURCES:
            raise ValueError(f"Mutation outside supported sources: {ident}")
        if not mutation["before"] or mutation["before"] == mutation["after"]:
            raise ValueError(f"Empty or ineffective replacement: {ident}")
        source = (PACKAGE / mutation["file"]).read_text(encoding="utf-8")
        count = source.count(mutation["before"])
        if count != 1:
            raise ValueError(f"Stale or ambiguous mutation {ident}: {count} matches (expected 1)")
        if not mutation["tests"] or len(set(mutation["tests"])) != len(mutation["tests"]):
            raise ValueError(f"Missing or duplicate tests: {ident}")


def execute(args, output):
    manifest = (PACKAGE / "mutations/gates.json").read_bytes()
    mutations = json.loads(manifest)
    if args.only:
        selected = set(args.only)
        unknown = selected - {mutation["id"] for mutation in mutations}
        if unknown:
            raise ValueError(f"Unknown mutations: {sorted(unknown)}")
        mutations = [mutation for mutation in mutations if mutation["id"] in selected]
    validate_mutations(mutations)
    output.mkdir(parents=True, exist_ok=True)
    logs = output / "logs"
    logs.mkdir(exist_ok=True)
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(PACKAGE / "target/mutation-build")
    env["CARGO_TERM_COLOR"] = "never"
    cargo = ["cargo", "test", "--locked"]
    if not args.online:
        cargo.append("--offline")
    report = {"scope": "curated gate mutations", "mutations": [], "baseline": "not-run", "complete": False}
    report_path = output / "report.json"

    def save():
        report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

    (PACKAGE / "target").mkdir(parents=True, exist_ok=True)
    # Copy only build inputs; never mutate the user's sources or binary artifacts.
    with tempfile.TemporaryDirectory(prefix="dfd-mutants-", dir=PACKAGE / "target") as directory:
        workspace = Path(directory)
        for folder in ["src", "tests", "skills", "templates"]:
            shutil.copytree(PACKAGE / folder, workspace / folder)
        for file in ["Cargo.toml", "Cargo.lock", "build.rs"]:
            shutil.copy2(PACKAGE / file, workspace / file)
        for file in PACKAGE.glob("*.md"):
            shutil.copy2(file, workspace / file.name)
        # Cargo uses mtimes: copy2 would preserve timestamps older than a cached
        # mutant build. Touch every copied input so the baseline is recompiled.
        for copied in workspace.rglob("*"):
            if copied.is_file():
                os.utime(copied, None)
        report["source_sha256"] = {name: hashlib.sha256((workspace / name).read_bytes()).hexdigest()
                                   for name in sorted(SOURCES | {"tests/workflow.rs"})}
        report["manifest_sha256"] = hashlib.sha256(manifest).hexdigest()
        listed, listing, _ = run(cargo + ["--test", "workflow", "--", "--list"], workspace,
                                 env, args.timeout, logs / "test-list.log")
        available = {line.removesuffix(": test") for line in listing.splitlines() if line.endswith(": test")}
        if listed != "survived" or not available:
            report["baseline"] = "test-discovery-failed"
            save()
            raise RuntimeError("Cannot discover workflow tests; see test-list.log")
        for mutation in mutations:
            missing = set(mutation["tests"]) - available
            if missing:
                raise ValueError(f"Unknown tests for {mutation['id']}: {sorted(missing)}")
        print("Baseline: running the complete unmodified suite", flush=True)
        baseline, _, seconds = run(cargo, workspace, env, args.timeout, logs / "baseline.log")
        report["baseline"] = "passed" if baseline == "survived" else baseline
        report["baseline_seconds"] = seconds
        save()
        if baseline != "survived":
            raise RuntimeError("Baseline did not pass; mutation results would be invalid")

        for index, mutation in enumerate(mutations, 1):
            path = workspace / mutation["file"]
            original = path.read_text(encoding="utf-8")
            if original.count(mutation["before"]) != 1:
                raise ValueError(f"Snapshot changed for {mutation['id']}: replacement must match exactly once")
            row = {"id": mutation["id"], "file": mutation["file"], "purpose": mutation["purpose"],
                   "tests": mutation["tests"], "status": "survived", "seconds": 0, "logs": []}
            try:
                path.write_text(original.replace(mutation["before"], mutation["after"], 1), encoding="utf-8")
                for test in mutation["tests"]:
                    log = logs / f"{mutation['id']}-{test}.log"
                    status, result, seconds = run(cargo + ["--test", "workflow", test, "--", "--exact"],
                                                  workspace, env, args.timeout, log)
                    row["seconds"] += seconds
                    row["logs"].append(str(log.relative_to(output)))
                    if status == "survived" and "1 passed;" not in result:
                        status = "test-discovery-failed"
                    if status != "survived":
                        row["status"] = status
                        break
                # A weak targeted test is not necessarily a gap in the entire suite.
                if row["status"] == "survived":
                    log = logs / f"{mutation['id']}-full-suite.log"
                    status, _, seconds = run(cargo, workspace, env, args.timeout, log)
                    row["seconds"] += seconds
                    row["logs"].append(str(log.relative_to(output)))
                    row["status"] = status
                    row["full_suite_fallback"] = True
            finally:
                path.write_text(original, encoding="utf-8")
            row["seconds"] = round(row["seconds"], 2)
            report["mutations"].append(row)
            save()
            print(f"[{index}/{len(mutations)}] {row['status']}: {row['id']}", flush=True)

    counts = {status: sum(row["status"] == status for row in report["mutations"])
              for status in sorted({row["status"] for row in report["mutations"]})}
    viable = counts.get("killed", 0) + counts.get("survived", 0)
    report["complete"] = True
    report["summary"] = counts
    report["score"] = counts.get("killed", 0) / viable if viable else None
    save()
    print(f"Results: {counts}; report: {report_path}", flush=True)
    return 0 if counts.get("killed", 0) == len(mutations) else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--only", nargs="+", help="Run only the specified mutation IDs")
    parser.add_argument("--output", type=Path, default=PACKAGE / "target/mutation-results")
    parser.add_argument("--timeout", type=int, default=180, help="Seconds per Cargo invocation")
    parser.add_argument("--online", action="store_true", help="Allow Cargo to fetch dependencies; default offline")
    args = parser.parse_args()
    if args.timeout <= 0:
        parser.error("--timeout must be positive")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    lock = output / ".lock"
    try:
        handle = os.open(lock, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
    except FileExistsError:
        parser.error(f"Mutation runner already active: {lock}")
    try:
        os.close(handle)
        return execute(args, output)
    except (ValueError, RuntimeError, OSError) as error:
        print(f"Mutation run failed: {error}", flush=True)
        return 1
    finally:
        lock.unlink(missing_ok=True)


if __name__ == "__main__":
    raise SystemExit(main())
