# 测试与覆盖率策略设计规格

日期：2026-08-12

## 1. 背景

hutool-rust 的验证体系已在方法级别建立（`verify-parity.py` + `verify-test-parity.py`），
基线数据显示 API 覆盖率 83.53%、测试行为覆盖率 99.21%。
本规格固化测试策略、覆盖率门槛和验证工具链，作为所有 Phase 的验收权威源。

## 2. 目标与非目标

### 2.1 目标

- 每个 crate 行覆盖率 >= 95%（`llvm-cov`）。
- 每个 hutool Java 测试方法在 Rust 侧有对应测试（`verify-test-parity.py` 100% 注册）。
- 测试分为：单元测试、集成测试、parity 测试、golden 测试。
- CI 自动运行全部测试 + 覆盖率报告。

### 2.2 非目标

- 不追求 Java 字节码级行为等价（Rust 语义不同）。
- 不为 `unsafe-to-copy` 的 Java 方法编写功能测试（只需注册 + stub）。
- 不在 v1.0 前建立 fuzz 测试的 CI 集成（已有独立 `fuzz/` workspace，手动运行）。

## 3. 测试分层

### 3.1 单元测试（crate 内）

```rust
// hutool-core/src/collection/coll_util.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        assert!(CollUtil::is_empty(None));
        assert!(CollUtil::is_empty(Some(&[])));
        assert!(!CollUtil::is_empty(Some(&[1, 2, 3])));
    }
}
```

规则：
- 每个公开方法至少 1 个正向 + 1 个反向测试。
- 边界条件（空输入、单元素、溢值）必须覆盖。
- 测试函数命名：`test_<method_name>_<scenario>`。

### 3.2 集成测试（tests/ 目录）

```rust
// hutool-core/tests/coll_util_integration.rs
use hutool_core::CollUtil;

#[test]
fn test_join_with_collection() {
    let result = CollUtil::join(&["a", "b", "c"], ",");
    assert_eq!(result, "a,b,c");
}
```

规则：
- 跨模块交互测试放在 `tests/` 目录。
- 测试 public API 的组合使用场景。

### 3.3 Parity 测试（对标 Java 行为）

```rust
// parity/ 目录下的测试资产
// scripts/verify-test-parity.py 验证每个 Java 测试方法有 Rust 对应
```

规则：
- `parity/hutool-v5.8.46-tests.csv` 列出全部 3292 个 Java 测试方法。
- `parity/test-decisions.csv` 记录每个 Java 测试的 Rust 对应（covered/planned/ignored-stub）。
- `verify-test-parity.py` 确保 100% 注册率。

### 3.4 Golden 测试（输出快照）

```rust
// 用于验证格式化输出、日期解析、编码结果等
#[test]
fn test_date_format_golden() {
    let dt = NaiveDateTime::parse_from_str("2026-01-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let formatted = DateUtil::format(dt, "%Y年%m月%d日");
    assert_eq!(formatted, "2026年01月15日");
}
```

## 4. 覆盖率工具与门槛

### 4.1 工具链

```bash
# 行覆盖率（llvm-cov）
cargo llvm-cov --workspace --html --output-dir target/coverage

# 按 crate 细分
cargo llvm-cov --workspace --json --output-path coverage.json

# 方法级 parity
python3 scripts/verify-parity.py --by-module
python3 scripts/verify-test-parity.py
```

### 4.2 门槛

| 维度 | 门槛 | 工具 |
|---|---|---|
| 行覆盖率（每 crate） | >= 95% | `cargo llvm-cov` |
| API 覆盖率（排除 unsafe-to-copy） | >= 97.55%（当前基线） -> 100% | `verify-parity.py --feasible` |
| 测试注册率 | 100% | `verify-test-parity.py` |
| 测试行为覆盖率 | >= 99.21%（当前基线） -> 100% | `verify-test-parity.py` |
| `cargo test --workspace` | 全绿 | CI |
| `cargo clippy --workspace -- -D warnings` | 零警告 | CI |
| `cargo fmt --check` | 通过 | CI |

## 5. Mock 策略

### 5.1 mockall（trait mock）

```rust
#[cfg(test)]
use mockall::automock;

#[automock]
trait Database {
    fn query(&self, sql: &str) -> Result<Vec<Row>>;
}

// 测试中
let mut mock = MockDatabase::new();
mock.expect_query()
    .returning(|_| Ok(vec![]));
```

### 5.2 mockall_double（依赖注入 mock）

```rust
use mockall_double::double;

#[double]
mod db {
    use super::MockDatabase as Database;
}
```

### 5.3 异步测试

```rust
#[tokio::test]
async fn test_http_get() {
    let response = HttpClient::new().get("https://example.com").send().await;
    assert!(response.is_ok());
}
```

## 6. CI 集成

```yaml
# .github/workflows/ci.yml
test:
  steps:
    - cargo fmt --check
    - cargo clippy --workspace -- -D warnings
    - cargo test --workspace
    - cargo llvm-cov --workspace --lcov --output-path lcov.info
    - python3 scripts/verify-parity.py --require-complete
    - python3 scripts/verify-test-parity.py
    - cargo deny check
```

## 7. 参考

- `PHASE_BASELINE.md` 测试覆盖率基线
- `IMPLEMENTATION_PLAN.md` 第十二节"测试体系"
- `parity/` 目录下的测试资产
- `scripts/verify-parity.py` 和 `scripts/verify-test-parity.py`
