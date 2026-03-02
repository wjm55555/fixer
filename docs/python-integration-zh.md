# Python 项目调用 Rust Fixer 指南

本文说明如何在其他 Python 项目中调用 `fixer` 修复模块。

## 1. 你会得到什么

- 输入 Rust 项目目录（必填）
- 可选输入 C 源码目录（用于辅助 LLM 理解）
- 输出修复后的 Rust 项目目录（无论成功失败都会生成）
- 自动执行：
  - `cargo fmt`
  - `cargo check --all-targets`
  - `cargo test`（默认开启）
  - 编译/测试失败时进入 LLM 修复循环（`maxIterations > 0` 时）
- 返回统一 JSON 结果（`status`, `metrics`, `changedFiles`, `artifacts` 等）

---

## 2. 前置条件

调用方机器需要满足：

1. 安装 `bun`（用于运行 TypeScript bridge）
2. 安装 Rust 工具链（`cargo`, `rustc`）
3. `fixer` 目录可访问
4. Provider 配置已设置（任选其一）：
   - 环境变量：`FIXER_BASE_URL` + `FIXER_API_KEY`
   - 本地文件：`fixer/config/provider.local.json`（可参考 `fixer/config/provider.example.json`）
5. 默认运行时为 vendored 完整内核；仅排障时可设置 `FIXER_ENGINE=shim`

> 注意：`outputDir` 必须是空目录或不存在，fixer 会先复制 workspace 到输出目录再修复。

---

## 3. 推荐方式：直接调用 Python 函数

文件：`python/fixer_bridge.py`  
函数：`repair_rust_project(...)`

### 3.1 最小示例

```python
from python.fixer_bridge import repair_rust_project

result = repair_rust_project(
    rust_project_dir="/path/to/rust-project",
    output_rust_project_dir="/path/to/output",
    source_project_dir="/path/to/c-source",   # 可选
    max_iterations=35,                        # >0 才会进入 LLM 修复
    run_tests_when_check_pass=True,           # 默认 True
    test_cases=[],                            # 可选：["module::test_name"]
)

print(result["status"])
print(result["metrics"])
print(result["artifacts"]["outputDir"])
```

### 3.2 参数说明

- `rust_project_dir`（必填）：Rust 项目根目录（含 `Cargo.toml`）
- `output_rust_project_dir`（必填）：修复输出目录（必须为空或不存在）
- `source_project_dir`（可选）：C 源码目录，仅用于辅助修复
- `test_cases`（可选）：测试过滤列表；内部执行为 `cargo test <filter>`
- `max_iterations`（默认 `35`）：
  - `0`：仅跑编译/测试，不调用 LLM（适合无额度时验证流程）
  - `>0`：允许 LLM 修复
- `time_budget_ms`（可选）：总时间预算
- `run_tests_when_check_pass`（默认 `True`）：`check` 通过后是否执行测试
- `fixer_dir`（可选）：fixer 根目录，不传则自动推断
- `bun_cmd`（默认 `"bun"`）：bun 命令名
- `show_logs`（默认 `True`）：是否打印 fixer 过程日志

### 3.3 返回结果关键字段

- `status`
  - `success`：编译和测试都通过
  - `partial`：编译通过但测试失败
  - `failed`：编译未通过或流程异常
- `metrics`
  - `iterations`：LLM 迭代次数
  - `cargoCheckPass`：编译是否通过
  - `cargoTestPass`：测试是否通过
- `artifacts.outputDir`：实际输出目录
- `changedFiles`：发生修改的文件列表

---

## 4. 方式二：通过 CLI bridge（subprocess）调用

入口：`src/python-bridge-cli.ts`  
输出最后会带一行标记：`__FIXER_RESULT__{...json...}`

### 4.1 Python subprocess 示例

```python
import json
import subprocess

payload = {
    "rustProjectDir": "/path/to/rust-project",
    "outputRustProjectDir": "/path/to/output",
    "sourceProjectDir": "/path/to/c-source",
    "maxIterations": 0,
    "runTestsWhenCheckPass": True,
}

proc = subprocess.run(
    ["bun", "run", "src/python-bridge-cli.ts", json.dumps(payload, ensure_ascii=False)],
    cwd="/path/to/fixer",
    text=True,
    capture_output=True,
    check=False,
)

marker = "__FIXER_RESULT__"
line = next((x for x in proc.stdout.splitlines()[::-1] if x.startswith(marker)), None)
if not line:
    raise RuntimeError(proc.stderr)
result = json.loads(line[len(marker):])
print(result["status"], result["metrics"])
```

---

## 5. 方式三：通过 HTTP 服务调用

服务文件：`python/fixer_service.py`

### 5.1 启动服务

```bash
cd /Users/wujiaming/Desktop/rust修复/fixer
python3 python/fixer_service.py --host 127.0.0.1 --port 8787
```

### 5.2 健康检查

```bash
curl -sS http://127.0.0.1:8787/health
```

### 5.3 修复请求示例

```bash
curl -sS http://127.0.0.1:8787/repair \
  -H "Content-Type: application/json" \
  -d '{
    "rustProjectDir": "/path/to/rust-project",
    "outputRustProjectDir": "/path/to/output",
    "sourceProjectDir": "/path/to/c-source",
    "maxIterations": 35,
    "runTestsWhenCheckPass": true,
    "testCases": []
  }'
```

返回格式：

```json
{
  "ok": true,
  "result": {
    "status": "success",
    "metrics": {
      "iterations": 2,
      "cargoCheckPass": true,
      "cargoTestPass": true,
      "clippyFixApplied": false
    }
  }
}
```

---

## 6. 无 API 额度时如何测试

将 `max_iterations` / `maxIterations` 设为 `0`：

- 会执行 `fmt/check/test`
- 不会触发 LLM 调用
- 可用于验证流程、输出目录、测试闭环是否正确

---

## 7. 常见坑位与建议

1. **不要把 C 源码目录当 workspace 传入**
   - `workspaceDir` 必须是 Rust 项目目录
   - C 源码目录应传 `sourceDir`

2. **输出目录必须为空**
   - 已有文件会直接报错

3. **测试失败会影响最终状态**
   - 当前默认要求测试通过才算 `success`

4. **日志默认会显示**
   - Python 函数层 `show_logs=True` 会透传修复过程

5. **建议固定 test_cases（可选）**
   - 大项目可只跑关键用例，提高迭代速度

---

## 8. 针对你当前项目的调用模板

```python
from python.fixer_bridge import repair_rust_project

result = repair_rust_project(
    rust_project_dir="/Users/wujiaming/Desktop/rust修复/fixer/inputs/rust项目/translate_cjson",
    output_rust_project_dir="/Users/wujiaming/Desktop/rust修复/fixer/outputs/translate_cjson_from_python",
    source_project_dir="/Users/wujiaming/Desktop/rust修复/fixer/inputs/源码/translate_cjson",
    max_iterations=35,   # 无额度时改成 0
    run_tests_when_check_pass=True,
)

print(result["status"])
print(result["metrics"])
print(result["artifacts"]["outputDir"])
```
