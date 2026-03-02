from __future__ import annotations

import argparse
import json
from http import HTTPStatus
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from typing import Any, Dict

try:
    from .fixer_bridge import repair_rust_project
except ImportError:
    from fixer_bridge import repair_rust_project


def _pick(payload: Dict[str, Any], *keys: str, default: Any = None) -> Any:
    for key in keys:
        if key in payload:
            return payload[key]
    return default


class FixerHandler(BaseHTTPRequestHandler):
    server_version = "RustFixerHTTP/0.1"

    def _send_json(self, status: int, body: Dict[str, Any]) -> None:
        data = json.dumps(body, ensure_ascii=False).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def _read_json(self) -> Dict[str, Any]:
        content_length = int(self.headers.get("Content-Length", "0"))
        if content_length <= 0:
            return {}
        raw = self.rfile.read(content_length).decode("utf-8")
        if not raw.strip():
            return {}
        return json.loads(raw)

    def do_GET(self) -> None:
        if self.path != "/health":
            self._send_json(HTTPStatus.NOT_FOUND, {"ok": False, "error": "Not found"})
            return
        self._send_json(HTTPStatus.OK, {"ok": True, "service": "rust-fixer"})

    def do_POST(self) -> None:
        if self.path != "/repair":
            self._send_json(HTTPStatus.NOT_FOUND, {"ok": False, "error": "Not found"})
            return

        try:
            payload = self._read_json()
            rust_project_dir = _pick(payload, "rustProjectDir", "rust_project_dir")
            output_rust_project_dir = _pick(payload, "outputRustProjectDir", "output_rust_project_dir")
            source_project_dir = _pick(payload, "sourceProjectDir", "source_project_dir")
            test_cases = _pick(payload, "testCases", "test_cases", default=[])
            max_iterations = int(_pick(payload, "maxIterations", "max_iterations", default=35))
            time_budget_ms = _pick(payload, "timeBudgetMs", "time_budget_ms")
            run_tests_when_check_pass = bool(
                _pick(payload, "runTestsWhenCheckPass", "run_tests_when_check_pass", default=True)
            )
            fixer_dir = _pick(payload, "fixerDir", "fixer_dir")
            bun_cmd = str(_pick(payload, "bunCmd", "bun_cmd", default="bun"))
            show_logs = bool(_pick(payload, "showLogs", "show_logs", default=True))

            if not rust_project_dir or not output_rust_project_dir:
                self._send_json(
                    HTTPStatus.BAD_REQUEST,
                    {
                        "ok": False,
                        "error": "Missing required fields: rustProjectDir and outputRustProjectDir",
                    },
                )
                return

            result = repair_rust_project(
                rust_project_dir=rust_project_dir,
                output_rust_project_dir=output_rust_project_dir,
                source_project_dir=source_project_dir,
                test_cases=test_cases,
                max_iterations=max_iterations,
                time_budget_ms=time_budget_ms,
                run_tests_when_check_pass=run_tests_when_check_pass,
                fixer_dir=fixer_dir,
                bun_cmd=bun_cmd,
                show_logs=show_logs,
            )
            self._send_json(HTTPStatus.OK, {"ok": True, "result": result})
        except Exception as exc:
            self._send_json(
                HTTPStatus.INTERNAL_SERVER_ERROR,
                {"ok": False, "error": str(exc)},
            )

    def log_message(self, format: str, *args: Any) -> None:
        # Keep service quiet by default; request/repair logs already come from fixer.
        return


def serve(host: str = "127.0.0.1", port: int = 8787) -> None:
    server = ThreadingHTTPServer((host, port), FixerHandler)
    print(f"Rust fixer HTTP service listening on http://{host}:{port}")
    print("Endpoints: GET /health, POST /repair")
    server.serve_forever()


def main() -> None:
    parser = argparse.ArgumentParser(description="Rust fixer minimal HTTP service")
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8787)
    args = parser.parse_args()
    serve(host=args.host, port=args.port)


if __name__ == "__main__":
    main()
