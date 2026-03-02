from __future__ import annotations

import json
import subprocess
from pathlib import Path
from typing import Any, Dict, Iterable, Optional

RESULT_MARKER = "__FIXER_RESULT__"


def _normalize_test_cases(test_cases: Optional[Iterable[str]]) -> list[str]:
    if not test_cases:
        return []
    return [item.strip() for item in test_cases if item and item.strip()]


def repair_rust_project(
    rust_project_dir: str,
    output_rust_project_dir: str,
    source_project_dir: Optional[str] = None,
    test_cases: Optional[Iterable[str]] = None,
    max_iterations: int = 35,
    time_budget_ms: Optional[int] = None,
    run_tests_when_check_pass: bool = True,
    fixer_dir: Optional[str] = None,
    bun_cmd: str = "bun",
    show_logs: bool = True,
) -> Dict[str, Any]:
    """
    Python bridge for opencode-rust-fix.

    It calls `src/python-bridge-cli.ts` and returns a JSON-like dict.
    """
    base_dir = Path(fixer_dir) if fixer_dir else Path(__file__).resolve().parents[1]
    payload = {
        "rustProjectDir": rust_project_dir,
        "outputRustProjectDir": output_rust_project_dir,
        "sourceProjectDir": source_project_dir,
        "testCases": _normalize_test_cases(test_cases),
        "maxIterations": max_iterations,
        "timeBudgetMs": time_budget_ms,
        "runTestsWhenCheckPass": run_tests_when_check_pass,
    }

    proc = subprocess.run(
        [bun_cmd, "run", "src/python-bridge-cli.ts", json.dumps(payload, ensure_ascii=False)],
        cwd=str(base_dir),
        text=True,
        capture_output=True,
        check=False,
    )

    stdout_lines = proc.stdout.splitlines()
    marker_line = next((line for line in reversed(stdout_lines) if line.startswith(RESULT_MARKER)), "")

    if show_logs:
        for line in stdout_lines:
            if not line.startswith(RESULT_MARKER):
                print(line)
        if proc.stderr:
            print(proc.stderr, end="" if proc.stderr.endswith("\n") else "\n")

    if not marker_line:
        raise RuntimeError(
            "Fixer output missing result marker. "
            f"exit={proc.returncode}, stderr={proc.stderr.strip()}"
        )

    result = json.loads(marker_line[len(RESULT_MARKER) :])
    return result
