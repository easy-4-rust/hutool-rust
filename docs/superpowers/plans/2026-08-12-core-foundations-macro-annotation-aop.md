# 核心基础实施计划：hutool-core / hutool-macro / hutool-annotation / hutool-aop

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 完成 hutool-core 目录结构重组（解决双重路径）、补齐缺失 facade 类、补全 hutool-macro proc-macro、hutool-annotation 和 hutool-aop 的剩余 API，使核心基础层达到 100% API 覆盖率。

**Architecture:** hutool-core 是零依赖的核心工具库（仅 std + 第三方 crate），包含 24 个子包对齐 Java `cn.hutool.core.*`。hutool-macro 提供编译期 proc-macro。hutool-annotation 提供元注解。hutool-aop 提供 trait-based AOP。依赖方向严格单向：hutool-aop -> hutool-core，其余无环依赖。

**Tech Stack:** Rust 2024 edition, resolver 3, `thiserror`（错误处理）, `chrono`（日期）, `regex`（正则）, `uuid`（ID）, `serde`/`serde_json`（序列化）, `proc-macro2`/`syn`/`quote`（宏）。

## Global Constraints

- 计划基线为 hutool-rust workspace 当前 HEAD；执行前必须重新检查未提交改动。
- Java 权威源为 Hutool v5.8.x（`/Users/wandl/workspaces/workspace-github/hutool`）。
- 已有实现不删减，通过 `pub use` 在 lib.rs 重新导出。
- `hutool-core/src/lib.rs` 最终不超过 80 行（仅 mod 声明 + re-export）。
- 顶层 `src/` 目录不允许 `.rs` 实现文件（除 `lib.rs` 和 `error.rs`）。
- 每个 hutool Java 子包 = 一个 Rust 子目录。
- 命名严格对齐 Java（见 `specs/2026-08-12-java-rust-naming-mapping-rules.md`）。
- 测试门槛：行覆盖率 >= 95%（见 `specs/2026-08-12-testing-coverage-strategy.md`）。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. hutool-core 双重路径问题必须解决（保留全部，pub use 重新导出）。
2. 缺失的 facade 类必须补齐（SetUtil、URLEncodeUtil、URLDecodeUtil、RegexUtil、SecureUtil、DigestUtil、Base32Util、Base64Util）。
3. hutool-macro 补齐计划中的 proc-macro（BeanDesc、ToJson、FromJson、Setting、TreeNode、function!）。
4. hutool-annotation 补齐剩余 50 个 API（当前覆盖率 74%）。
5. hutool-aop 补齐剩余 9 个文件（Java 15 文件，Rust 21 文件但 API 覆盖率 100%）。

### 1.2 非目标

- 不迁移 `cn.hutool.core.swing`（标记 unsafe-to-copy，0% 覆盖率）。
- 不在 hutool-core 内实现加密功能（委托到 hutool-crypto）。
- 不修改已有测试的行为。

---

## 2. 当前状态（代码事实）

| 指标 | 当前值 | 证据 |
|---|---|---|
| hutool-core Rust 文件数 | 892 | `find crates/hutool-core/src -name "*.rs" | wc -l` |
| hutool-core Java 文件数 | 713 | `find hutool/hutool-core/src/main/java -name "*.java" | wc -l` |
| hutool-core API 覆盖率 | 92% (7027/7605) | `PHASE_BASELINE.md` |
| 顶层 .rs 实现文件 | 23 个（应 <= 2） | `ls crates/hutool-core/src/*.rs` |
| 双重路径模块数 | >= 8 | `coll_util`, `file_util`, `io_util`, `iter_util`, `list_util`, `map_util`, `id` |
| lib.rs 行数 | ~280（应 <= 80） | `wc -l crates/hutool-core/src/lib.rs` |
| hutool-macro 文件数 | 86 | `find crates/hutool-macro/src -name "*.rs" | wc -l` |
| hutool-annotation 文件数 | 17 | `find crates/hutool-annotation/src -name "*.rs" | wc -l` |
| hutool-aop 文件数 | 21 | `find crates/hutool-aop/src -name "*.rs" | wc -l` |

---

## 3. 文件职责总览

| 文件/目录 | 职责 |
|---|---|
| `crates/hutool-core/src/lib.rs` | 模块索引（<= 80 行） |
| `crates/hutool-core/src/collection/` | CollUtil、ListUtil、IterUtil、CollStreamUtil |
| `crates/hutool-core/src/util/` | 全部 `*_util.rs` 收编 |
| `crates/hutool-core/src/io/` | FileUtil、IoUtil、Resource |
| `crates/hutool-core/src/text/` | StrUtil（完整 facade） |
| `crates/hutool-core/src/net/` | URLEncodeUtil、URLDecodeUtil |
| `crates/hutool-macro/src/lib.rs` | proc-macro 入口 |
| `crates/hutool-annotation/src/` | 元注解 |
| `crates/hutool-aop/src/` | trait-based AOP |

---

### Task 1: 解决 hutool-core 双重路径 -- collection 子包

**Files:**
- Modify: `crates/hutool-core/src/coll_util.rs` -> 移动到 `crates/hutool-core/src/collection/coll_util.rs`
- Modify: `crates/hutool-core/src/iter_util.rs` -> 移动到 `crates/hutool-core/src/collection/iter_util.rs`
- Modify: `crates/hutool-core/src/list_util.rs` -> 移动到 `crates/hutool-core/src/collection/list_util.rs`
- Modify: `crates/hutool-core/src/collection/mod.rs` -- 添加 `pub use` 重新导出
- Modify: `crates/hutool-core/src/lib.rs` -- 保留旧路径的 `pub use` 兼容

- [ ] **Step 1: 移动文件并保留旧路径 re-export**

```bash
git mv crates/hutool-core/src/coll_util.rs crates/hutool-core/src/collection/coll_util.rs
git mv crates/hutool-core/src/iter_util.rs crates/hutool-core/src/collection/iter_util.rs
git mv crates/hutool-core/src/list_util.rs crates/hutool-core/src/collection/list_util.rs
```

在 `lib.rs` 中保留：
```rust
pub use collection::coll_util::CollUtil;
pub use collection::iter_util::IterUtil;
pub use collection::list_util::ListUtil;
```

- [ ] **Step 2: 运行测试确认无回归**

```bash
cargo test -p hutool-core
```

- [ ] **Step 3: 提交**

```bash
git commit -m "refactor(core): resolve dual-path for collection sub-package"
```

### Task 2: 解决 hutool-core 双重路径 -- io / map / text 子包

**Files:**
- Move: `file_util.rs` -> `io/file_util.rs`
- Move: `io_util.rs` -> `io/io_util.rs`
- Move: `map_util.rs` -> `map/map_util.rs`（如果存在于顶层）
- Move: `str_util.rs` -> `text/str_util.rs`（如果存在于顶层）
- Move: `id.rs` -> `util/id_util.rs`

- [ ] **Step 1: 移动文件**
- [ ] **Step 2: 更新 mod.rs 和 lib.rs re-export**
- [ ] **Step 3: 运行测试**
- [ ] **Step 4: 提交**

### Task 3: 收编顶层 util 文件到 util/ 子目录

**范围:** 将 `array_util.rs`、`boolean_util.rs`、`byte_util.rs`、`char_util.rs`、`charset_util.rs`、`class_util.rs`、`number_util.rs`、`random_util.rs`、`re_util.rs`、`reflect_util.rs`、`runtime_util.rs`、`validator.rs`、`zip_util.rs`、`escape_util.rs`、`enum_util.rs`、`phone_util.rs`、`primitive_array_util.rs` 等移动到 `util/` 子目录。

- [ ] **Step 1: 批量移动（保持 git 历史）**
- [ ] **Step 2: 更新 lib.rs（顶层只留 mod 声明 + re-export）**
- [ ] **Step 3: 运行 `cargo test -p hutool-core`**
- [ ] **Step 4: 提交**

### Task 4: 收编其余顶层文件

**范围:** `mutable.rs` -> `lang/mutable/`，`opt.rs` -> `lang/`，`validator.rs` -> `lang/`，`dict.rs` -> `lang/` 等。

- [ ] **Step 1: 移动**
- [ ] **Step 2: 更新 re-export**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 5: 精简 lib.rs 到 <= 80 行

- [ ] **Step 1: 重构 lib.rs，只保留 `pub mod` 声明和 `pub use` re-export**
- [ ] **Step 2: 确保所有公开类型仍可通过 `hutool_core::*` 访问**
- [ ] **Step 3: 运行全量测试**
- [ ] **Step 4: 提交**

### Task 6: 补齐缺失 facade 类

**范围:** SetUtil、URLEncodeUtil、URLDecodeUtil、RegexUtil（re-export re_util）、SecureUtil（re-export hutool-crypto）、DigestUtil（re-export hutool-crypto::digest）、Base32Util、Base64Util。

- [ ] **Step 1: 创建 `util/set_util.rs`**
- [ ] **Step 2: 创建 `net/url_encode_util.rs` 和 `net/url_decode_util.rs`**
- [ ] **Step 3: 创建 `util/secure_util.rs`（facade 委托到 hutool-crypto）**
- [ ] **Step 4: 创建 `util/digest_util.rs`（facade 委托到 hutool-crypto）**
- [ ] **Step 5: 更新 lib.rs re-export**
- [ ] **Step 6: 运行测试**
- [ ] **Step 7: 提交**

### Task 7: hutool-macro 补齐 proc-macro

**范围:** 补齐 `#[derive(BeanDesc)]`、`#[derive(ToJson)]`、`#[derive(FromJson)]`、`#[derive(Setting)]`、`#[derive(TreeNode)]`、`function!{}` 宏。

- [ ] **Step 1: 实现 `#[derive(BeanDesc)]`**
- [ ] **Step 2: 实现 `#[derive(ToJson)]` / `#[derive(FromJson)]`**
- [ ] **Step 3: 实现 `function!{}` 模板字符串宏**
- [ ] **Step 4: 编写测试**
- [ ] **Step 5: 提交**

### Task 8: hutool-annotation 和 hutool-aop 收尾

- [ ] **Step 1: 补齐 hutool-annotation 剩余 API（50 个缺口）**
- [ ] **Step 2: 确认 hutool-aop API 100% 覆盖（当前已是 100%，验证无遗漏）**
- [ ] **Step 3: 运行全量测试**
- [ ] **Step 4: 提交**

---

## 4. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| 顶层 .rs 文件 <= 2 | `ls crates/hutool-core/src/*.rs | wc -l` |
| lib.rs <= 80 行 | `wc -l crates/hutool-core/src/lib.rs` |
| 双重路径模块 = 0 | 无同名文件在顶层和子目录同时存在 |
| hutool-core API >= 97% | `verify-parity.py --by-module` |
| hutool-macro 新增 proc-macro | 编译通过 + 测试通过 |
| `cargo test -p hutool-core` | 全绿 |
| `cargo test -p hutool-macro` | 全绿 |
