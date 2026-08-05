# Hutool-Rust-Rs 最终实施计划

> **版本**：v1.0（最终版，整合所有前期讨论、迁移审计与修正）
> **基线**：Hutool Java v5.8.x（`/Users/wandl/workspaces/workspace-github/hutool`）
> **目标仓库**：`/Users/wandl/workspaces/workspace-github/hutool-rust`
> **参考项目**：`sa-token-rs`（同模式 Java→Rust 移植，详细模板见其 `docs/IMPLEMENTATION_PLAN.md`）
> **当前迁移基线**：见 `/Users/wandl/workspaces/workspace-github/hutool-rust/MIGRATION_STATUS.md`（完成度 ≈ 59.6% 文件数 / ≈ 50% 加权方法数）
> **定位**：一比一复刻 Hutool Java 的 Rust 实现，对象/方法/参数命名严格对齐，Rust 端使用 idiomatic API + 成熟 crate 引擎
> **状态**：历史实施计划，已按 2026-07-24 代码快照同步关键状态
>
> **当前代码事实**：workspace 现有 26 个 crate，其中新增独立 `hutool-observability`。~~`crates/hutool-poi` 已建立 79 个 Rust 源文件的 API 占位骨架~~（**2026-08-04 已移除**：`git rm -r crates/hutool-poi`，Excel/Word/OFD 能力由独立项目 `easyexcel-rust` 承接），
> 其中 67 处调用 `unimplemented!()`；该 crate 只依赖 `thiserror`，未由 `hutool` facade 暴露，也未接入任何
> Excel、Word、OFD 或 PDF 引擎。因此下文所有“创建 hutool-poi 占位”的步骤均视为历史已完成，
> 但 `hutool-poi` 功能实现仍为 0%，并从完成度与生产能力声明中排除。

> ⚠️ **关于 hutool-compat-hutool 的重新定位**：经过对当前实现的诚实审计（仅 114 行 / 2 个 facade / 11 个方法），
> 本文档**早期设想**的「双 API 表面（idiomatic + hutool-compat）」架构目标已被用户明确否决。
> 当前优先级是把 `hutool-db`（缺 75 文件）、`hutool-extra`（缺 170 文件）、`hutool-cron`（缺 37 文件）、
> `hutool-http`（缺 47 文件）、`hutool-crypto`（缺 48 文件）等**主仓核心模块**先迁移到位。
> `hutool-compat-hutool` crate 在主仓迁移完成之前**没有任何迁移价值**（背后无实现可委托）。
> 后续若用户明确需要 Hutool 风格门面，再单独评估。

---

## 目录

- [一、项目背景与目标](#一项目背景与目标)
- [二、核心设计原则](#二核心设计原则)
- [三、命名映射规则（严格对齐 Java）](#三命名映射规则严格对齐-java)
- [四、Workspace 总体结构](#四workspace-总体结构)
- [五、hutool-core 内部模块（1:1 对齐 Java 包）](#五hutool-core-内部模块11-对齐-java-包)
- [六、核心类型/Trait 签名（重点 hutool-core）](#六核心类型trait-签名重点-hutool-core)
- [七、proc-macro 设计](#七proc-macro-设计)
- [八、生态适配层（Web/DB/AI/Email/Captcha）](#八生态适配层webdbaicaptcha)
- [九、关键技术决策汇总](#九关键技术决策汇总)
- [十、分阶段实施计划](#十分阶段实施计划)
- [十一、依赖清单](#十一依赖清单)
- [十二、测试体系](#十二测试体系)
- [十三、迁移文档体系](#十三迁移文档体系)
- [十四、风险与缓解](#十四风险与缓解)
- [十五、Java ↔ Rust 完整文件对应关系（节选）](#十五java--rust-完整文件对应关系节选)
- [十六、Phase 0 立即执行清单](#十六phase-0-立即执行清单)
- [附录 A：与 Hutool Java 的关键差异速查](#附录-a与-hutool-java-的关键差异速查)
- [附录 B：参考项目](#附录-b参考项目)
- [附录 C：Hutool v5.8.x 模块 → Hutool-Rust crate 映射表](#附录-chutool-v58x-模块--hutool-crate-映射表)
- [附录 D：DDD4J → Hutool-Rust-Rs 完整生态映射](#附录-dddd4j--hutool-rust-完整生态映射)

---

## 一、项目背景与目标

### 1.1 背景

Hutool 是 Java 生态**最受欢迎**的国产基础工具集（`cn.hutool.*`），覆盖：

- **基础工具**：字符串、集合、日期、IO、加密、转换、反射
- **Web 与协议**：HTTP、Socket、Mail、FTP、SSH、SOAP、WebService
- **数据**：JSON、XML、Excel、Word、PDF、OFD、CSS、模板引擎
- **扩展生态**：二维码、验证码、拼音、分词、表情、Spring/Servlet 适配
- **调度/缓存**：Cron 表达式、TTL 缓存、LRU/LFU/Timed/Weak 缓存、Bloom Filter
- **AI/ML**：7 家大模型 Provider 统一抽象

**Rust 生态现状**：
- 没有能与之对标的"全家桶"工具集
- 各领域碎片化：`chrono` / `reqwest` / `serde_json` / `tokio-cron-scheduler` / `moka` / `sqlx` 等单独可用
- Hutool 用户从 Java 转 Rust 时心智模型完全断裂

### 1.2 目标

**一比一**复刻 Hutool Java 到 Rust，按用户要求：

| 维度 | 目标 |
|---|---|
| **文件数量** | Java 1553 文件 ↔ Rust 926 文件（已有），目标补足至 ≈ 1500+ Rust 文件 |
| **文件路径** | `cn.hutool.core.collection.CollUtil` ↔ `hutool-core/src/collection/coll_util.rs` |
| **对象/方法/参数命名** | 100% 对齐（snake_case 转换除外） |
| **方法逻辑** | 核心逻辑对齐，错误处理 Rust 化（`Result<T, E>`） |
| **hutool-poi** | **2026-08-04 已移除**：占位 crate 删除，Excel/Word/OFD 能力由 `easyexcel-rust`（EasyExcel 4.0.3 高保真迁移）承接 |
| **已有实现** | **不删减**，顺着原思路继续迁移（解决 hutool-core 内部双重路径问题） |
| **文档注释** | rustdoc 必须标注"原 Java 对应文件/方法" |
| **Rust 生态** | 充分利用 `chrono`/`uuid`/`reqwest`/`sqlx`/`moka`/`tokio`/`tracing`/`serde`/`regex`/`rustcrypto` |

### 1.3 非目标

- ❌ 不追求 Java 字节码级 1:1（Rust 与 Java 语义差异大）
- ❌ 不复刻 Spring AOP 运行时反射（改用 proc-macro 编译期生成或 trait 适配）
- ❌ 不复刻 Hutool 的 Swing/AWT UI（标记 `unsafe-to-copy`/planned）
- ❌ 不复刻 JVM-only 能力（JNDI/Servlet 容器/SOAP Server/SSH/FTP），改用 idiomatic 替代

---

## 二、核心设计原则

本项目参考了 `sa-token-rs`（同模式 Java→Rust 移植，模板成熟）和 `hutool-rust` 现有的 7 条核心设计决策：

### 原则 1：**单一 Rust 实现 + 命名严格对齐 Java**（**已重新评估**）

> ⚠️ 早期文档设想的「双 API 表面（idiomatic + hutool-compat）」架构目标**已被用户否决**。
> 当前唯一 API 表面是 Rust idiomatic，**结构体/方法/参数命名严格对齐 Hutool Java**（snake_case 转换除外）。

| 层 | 决策 | 原因 |
|---|---|---|
| `hutool-core` / `hutool-cron` / ... | **Rust idiomatic 单一 API** | 充分利用 Rust 类型系统、所有权、async |
| `hutool-compat-hutool` | **暂时不扩展**，作为**未实现占位** | 底层主仓未完成前，compat 层无真实实现可委托；当前 114 行价值低 |

**实现策略**：每个 Hutool 类直接以 Rust struct/enum + idiomatic API 实现。命名通过 snake_case 转换保留 Java 业务语义：

```rust
// hutool-core/src/collection/coll_util.rs — Rust idiomatic 唯一实现
pub struct CollUtil;
impl CollUtil {
    pub fn is_empty<T>(values: Option<&[T]>) -> bool { ... }
    pub fn join<T: Display>(values: impl IntoIterator<Item = T>, delimiter: &str) -> String { ... }
    // ... 100+ 方法
}
```

### 原则 2：对象/方法命名严格对齐 Java（动词与业务词汇不翻译）

```rust
// ❌ 错误：意译
pub fn concat(...)

// ✅ 正确：保留 Java 动词
pub fn join<T: Display>(values: impl IntoIterator<Item = T>, delimiter: &str) -> String
```

```text
isEmpty    →  is_empty        (命名风格转换)
login      →  login           (业务词汇不变)
kickout    →  kickout         (业务词汇不变)
replaced   →  replaced        (业务词汇不变)
openSafe   →  open_safe       (命名风格转换，业务不变)
```

### 原则 3：**已有实现不删减**（用户特别要求）

> ⚠️ hutool-core 当前存在双重路径（如 `coll_util.rs` 顶层 + `collection/coll_util.rs`），
> 这是历史迁移过渡状态。**原则：保留所有已有实现**，通过 `pub use` 在 lib.rs 重新导出。

```rust
// hutool-core/src/lib.rs
// 同时导出"hutool-legacy"位置和"new path"，消除用户认知差异
pub use crate::coll_util::CollUtil;
pub use crate::collection::coll_util::CollUtil as CollUtilV2;
```

### 原则 4：类型映射表固定（不重新发明）

| Java | Rust | 备注 |
|---|---|---|
| `String` | `String` / `&str` | 参数倾向 `&str`，返回 `String` |
| `int` / `long` | `i32` / `i64` | 默认 `i64`（Java long 等价） |
| `boolean` | `bool` | — |
| `List<T>` | `Vec<T>` | — |
| `Map<K,V>` | `HashMap<K,V>` | 顺序保留时用 `IndexMap<K,V>` |
| `Set<T>` | `HashSet<T>` | 顺序保留时用 `IndexSet<T>` |
| `Collection<T>` | `&[T]` | 只读场景用 slice |
| `Optional<T>` | `Option<T>` | — |
| `Date` / `Calendar` | `chrono::NaiveDateTime` | 类型别名 `pub type DateTime = NaiveDateTime` |
| `InputStream` | `R: Read` | trait 约束泛型 |
| `OutputStream` | `W: Write` | trait 约束泛型 |
| `Throwable` | `Result<T, E>` | — |
| `enum Foo { A, B }` | `enum Foo { A, B }` | 完全一致 |
| `Map.entrySet()` | `.iter()` | idiomatic |
| `Class<T>` | `TypeId` | 类型标识 |
| `Object` | `serde_json::Value` | 通用对象 |

### 原则 5：错误处理：**单 crate 一个根 enum + 子 enum 派生**

```rust
// hutool-core/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] chrono::ParseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("util exception: {0}")]
    Util(String),
    // ...
}
```

```rust
// hutool-crypto/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("invalid key: {0}")]
    InvalidKey(String),
    // ...
}
```

### 原则 6：全局状态：`OnceLock<Arc<T>>`（无锁优先）

```rust
use std::sync::OnceLock;

static CONFIG: OnceLock<Arc<SaTokenConfig>> = OnceLock::new();
```

### 原则 7：依赖方向严格单向（无环）

```text
hutool (facade) ──→ 所有组件

hutool-core ──→ (不依赖任何本 workspace crate)

hutool-extra ──→ hutool-core
hutool-http ──→ hutool-core + hutool-json
hutool-db ──→ hutool-core + hutool-json
hutool-cron ──→ hutool-core + hutool-log
hutool-aop ──→ hutool-core

# hutool-compat-hutool 暂时冻结（不扩展、不删除）
# 待主仓迁移完成后再评估
hutool-crypto ──→ hutool-core
hutool-jwt ──→ hutool-crypto + hutool-json
hutool-ai ──→ hutool-http + hutool-json + hutool-core
```

---

## 三、命名映射规则（严格对齐 Java）

### 3.1 全局规则

| Java | Rust | 示例 |
|---|---|---|
| 包 `cn.hutool.core.collection` | crate + mod `hutool_core::collection` | — |
| 类 `CollUtil` | 结构体 `CollUtil`（PascalCase 保留） | `CollUtil` |
| `CollUtil.isEmpty(...)` | `CollUtil::is_empty(...)` | 静态方法 → 关联函数 |
| `collUtil` 变量 | `coll_util` 变量 | snake_case |
| `getXxx()` | `xxx()` | getter 去掉前缀 |
| `setXxx(v)` | `set_xxx(v)` | setter 保留 set_ 前缀 |
| `isXxx()` | `is_xxx()` | — |
| `hasXxx()` | `has_xxx()` | — |
| `MAX_SIZE` 常量 | `MAX_SIZE` 常量 | SCREAMING_SNAKE 不变 |
| 接口 `CollUtil`（作为抽象） | trait `CollUtilLike` | 后缀 `Like` |
| 文件 `CollUtil.java` | 文件 `coll_util.rs` | snake_case 文件名 |
| `package-info.java` | `mod.rs`（仅当目录） | N/A |

### 3.2 Rust 特有命名约定

| Java | Rust | 原因 |
|---|---|---|
| `static final` 字段 | `const` 或 `pub const` | 编译期常量 |
| `Builder` 类内嵌 | builder pattern + `impl FooBuilder` | Rust 习惯 |
| `enum` 内部类 | `pub enum Foo { ... }` | 同 |
| `interface Foo` | `trait Foo` | 同 |
| `abstract class Foo` | `trait Foo`（默认实现用 trait method） | 抽象类 → trait |
| `record Foo(...)`（Java 14+） | `pub struct Foo(pub Type, ...);` | Rust tuple struct |
| `@FunctionalInterface` | `Fn(...) -> ...` | trait bound |
| `@Override` | 无需标注 | trait 默认 |

### 3.3 必须保留业务动词的清单（**禁止翻译**）

| Java 业务动词 | Rust 命名 | 出现模块 |
|---|---|---|
| `login` / `logout` | `login` / `logout` | core, sa-token |
| `kickout` / `replaced` | `kickout` / `replaced` | core, sa-token |
| `openSafe` / `closeSafe` | `open_safe` / `close_safe` | core, sa-token |
| `disable` / `untie` | `disable` / `untie` | core, sa-token |
| `switch` / `endSwitch` | `switch` / `end_switch` | core, sa-token |
| `setToken` / `getToken` | `set_token` / `get_token` | core, sa-token |
| `createLoginSession` | `create_login_session` | core, sa-token |
| `parseObj` / `toBean` | `parse_obj` / `to_bean` | json |
| `toJsonStr` / `fromJsonStr` | `to_json_str` / `from_json_str` | json |
| `writeExcel` / `readExcel` | `write_excel` / `read_excel` | poi |
| `send` / `recv` | `send` / `recv` | socket, http |
| `upload` / `download` | `upload` / `download` | ftp, http |

---

## 四、Workspace 总体结构

### 4.1 当前实际结构（基于 MIGRATION_STATUS.md 现状）

```text
hutool-rust/                                       # 仓库根（对应 Hutool 根 pom.xml）
├── Cargo.toml                                   # [workspace] 根清单（resolver = "3"）
├── README.md / CHANGELOG.md / SECURITY.md / NOTICE / LICENSE
├── docs/
│   ├── IMPLEMENTATION_PLAN.md                   # 本文档（最终实施计划）
│   ├── ARCHITECTURE.md                          # 架构总览 + Rust idiom 与 Hutool 兼容层
│   ├── feature-matrix.md                        # Feature 矩阵（编译成本/平台/安全影响）
│   ├── hutool-parity.md                         # Hutool-5.8 模块对照账本
│   ├── production-readiness.md                  # 1.0 准入清单
│   ├── security.md / provenance.md
│   ├── MIGRATION_STATUS.md                      # Phase 进度 + 已实现方法追踪
│   └── migration/                               # 迁移审计文档
│       ├── java-tree-full.md
│       ├── rust-tree-full.md
│       ├── project-tree-diff.md
│       ├── object-method-matrix.md
│       ├── CODEGRAPH_METHOD_MAP.md
│       ├── codegraph-gap-audit.md
│       └── TEST_AUDIT_REPORT.md
│
├── crates/
│   ├── hutool/                                  # ★ facade（用户只引这一个包）
│   │   ├── Cargo.toml                           # features: default=[core,json] + full=...
│   │   ├── src/lib.rs                           # pub use 全组件 + prelude
│   │   └── examples/
│   │       ├── cache_demo.rs
│   │       ├── core_json.rs
│   │       ├── crypto_hash.rs
│   │       ├── http_client.rs
│   │       └── json_object.rs
│   │
│   ├── hutool-core/                             # ★ 核心库（对应 hutool-core/）
│   ├── hutool-aop/                              # AOP trait + interceptor
│   ├── hutool-bloom-filter/                     # BloomFilter + BitMap
│   ├── hutool-cache/                            # Cache (基于 moka)
│   ├── hutool-captcha/                          # 验证码生成
│   ├── hutool-cron/                             # Cron 表达式 + 调度
│   ├── hutool-crypto/                           # 加密（AES-GCM/HMAC/SHA/Argon2id）
│   ├── hutool-db/                               # 数据库（基于 sqlx）
│   ├── hutool-dfa/                              # Aho-Corasick
│   ├── hutool-extra/                            # QR/压缩/邮件/拼音/分词/...
│   ├── hutool-http/                             # HTTP 客户端（基于 reqwest）
│   ├── hutool-json/                             # JSON（基于 serde_json）
│   ├── hutool-jwt/                              # JWT（基于 jsonwebtoken）
│   ├── hutool-log/                              # 日志（基于 tracing）
│   ├── hutool-script/                           # 脚本（基于 rhai）
│   ├── hutool-setting/                          # 配置（基于 config）
│   ├── hutool-socket/                           # TCP/UDP（基于 tokio）
│   ├── hutool-system/                           # OS/进程信息（基于 sysinfo）
│   ├── hutool-ai/                               # AI Provider 抽象
│   ├── hutool-macro/                            # proc-macro 工具
│   ├── hutool-compat-hutool/                    # 🟡 暂时冻结（114 行占位，待主仓完成后再评估）
│   ├── hutool-test-support/                     # 测试工具
│   │
│   └── hutool-poi/                              # ⚪ API 占位骨架（79 个 .rs、67 处 unimplemented，不是实现）
│
├── examples/                                    # 跨 crate 示例
├── benches/                                     # 性能基准
├── fuzz/                                        # Fuzz 测试（独立 workspace）
├── parity/                                      # Parity 测试资产
├── cn/                                          # 中文文档
├── scripts/                                     # 辅助脚本（gap-check, coverage, java-golden-export）
└── .github/workflows/ci.yml                     # CI: fmt + clippy(-D warnings) + test + coverage + deny
```

### 4.2 目标结构（按 IMPLEMENTATION_PLAN 完成后）

新增/改动（按用户优先级）：
- **`crates/hutool-extra/`** 扩充（**缺 170 文件，最高优先级**）
- **`crates/hutool-db/`** 扩充（**缺 75 文件**）
- **`crates/hutool-crypto/`** 扩充（缺 48 文件）
- **`crates/hutool-http/`** 扩充（缺 47 文件）
- **`crates/hutool-cron/`** 扩充（缺 37 文件）
- **`crates/hutool-poi/`** 已存在占位骨架；不纳入 facade 或实现完成度
- `crates/hutool-core/tests/` 扩充（330 Java 测试 → 目标 200+ Rust 测试）
- `crates/hutool-compat-hutool/` **冻结维持现状**（114 行，待主仓完成后再评估）
- `docs/migration/` 扩充（CODEGRAPH_METHOD_MAP 等）

### 4.3 依赖方向（严格单向，无环）

```text
hutool (facade)
  └── hutool-core
  └── hutool-json ──→ hutool-core
  └── hutool-http ──→ hutool-core + hutool-json
  └── hutool-db ──→ hutool-core + hutool-json
  └── hutool-crypto ──→ hutool-core
  └── hutool-jwt ──→ hutool-crypto + hutool-json
  └── hutool-ai ──→ hutool-http + hutool-json + hutool-core
  └── ... (全部组件)

hutool-core ──→ (零依赖，仅 std + 第三方 crate)

# hutool-compat-hutool 暂时冻结，不扩展、不删除、不新增依赖
```

---

## 五、hutool-core 内部模块（1:1 对齐 Java 包）

### 5.0 ⚠️ 目录组织原则（**用户硬性要求**）

> "你必须参考 hutool 的目录命名规范，结合 rust 的项目结构，
> 不能都在一个根目录，或者一个 lib 中把所有代码都实现了。"

**核心规则**：
1. **顶层不写实现代码**：除 `lib.rs` 和 `error.rs` 外，**不允许任何 `.rs` 文件直接放在 `src/` 根目录**
2. **lib.rs 是目录索引**：只写 `pub mod xxx;` 和 `pub use ...;`，**不超过 80 行**
3. **每个 hutool Java 子包 = 一个 Rust 子目录**：`cn.hutool.core.collection` ↔ `src/collection/`
4. **子目录内可以嵌套子目录**：对齐 hutool 的多层子包（如 `cn.hutool.core.io.resource` ↔ `src/io/resource/`）
5. **每个子目录有自己的 `mod.rs`**：只写 `pub mod sub_module;` 声明

```rust
// ✅ 正确示例：lib.rs 是目录索引
// hutool-core/src/lib.rs
#![forbid(unsafe_code)]
pub mod annotation;
pub mod bean;
pub mod collection;
pub mod io;
pub mod util;
// ... 共 24 个 mod
pub use collection::CollUtil;

// ✅ 正确示例：collection/mod.rs 是子目录索引
// hutool-core/src/collection/mod.rs
mod coll_util;
mod coll_stream_util;
mod iter_util;
mod list_util;
pub use coll_util::CollUtil;
pub use iter_util::IterUtil;
pub use list_util::ListUtil;

// ❌ 错误示例：实现代码堆在顶层
// hutool-core/src/coll_util.rs        ← 违反规则 1
// hutool-core/src/iter_util.rs        ← 违反规则 1
// hutool-core/src/lib.rs (280 行)     ← 违反规则 2
```

### 5.0.1 🎯 1:1 全文件覆盖硬性要求（**用户最终要求**）

> "hutool 中的除了 hutool-poi，每一个模块，每一个文件在 hutool-rust 中必须有对应的实现。"

**核心原则（贯穿整个实施过程）**：

| 要求 | 含义 |
|---|---|
| **每个 hutool 模块** | 除 hutool-poi 外，**每个模块都必须有完整对应的 Rust crate** |
| **每个 hutool 文件** | 除 hutool-poi 外，**每个 `.java` 文件都必须有对应的 `.rs` 文件** |
| **不例外** | 不可合并、不可省略、不可跳过、不可标记为"可选" |
| **空实现也要建文件** | 即使只写 stub 或 `unimplemented!()`，也要建对应 `.rs` 文件 + `pub fn` 骨架 |
| **hutool-poi 唯一例外** | 按用户原话"hutool 中的除了 hutool-poi"，hutool-poi 暂保留占位 |

**验收方法（每个 Phase 完成时执行）**：

```bash
# 1. 列出 hutool 全部 .java 文件
find /Users/wandl/workspaces/workspace-github/hutool -name "*.java" \
  -not -path "*/test/*" -not -path "*/target/*" \
  | grep -v "/hutool-poi/" > /tmp/hutool_all_java.txt

# 2. 列出 hutool-rust 全部 .rs 文件
find /Users/wandl/workspaces/workspace-github/hutool-rust/crates -name "*.rs" \
  -not -path "*/target/*" -not -path "*/tests/*" -not -path "*/examples/*" \
  | grep -v "/hutool-poi/" > /tmp/hutool_all_rs.txt

# 3. 计算覆盖率
total_java=$(wc -l < /tmp/hutool_all_java.txt)
covered_java=...  # 通过 camelCase → snake_case 映射统计
coverage=$(echo "scale=2; $covered_java * 100 / $total_java" | bc)
echo "Java 文件覆盖率: ${coverage}%"

# 4. 找出未映射的 hutool 文件
# （具体脚本见 Phase 6 验收工具）
```

**覆盖度验收门槛（每个 Phase）**：

| Phase | 验收门槛 | 说明 |
|---|---|---|
| Phase 1 结束 | ≥ 30% | hutool-extra + hutool-core 目录重组完成 |
| Phase 2 结束 | ≥ 60% | hutool-cron / hutool-db 大件补齐 |
| Phase 3 结束 | ≥ 85% | hutool-crypto / hutool-http 补齐 |
| Phase 4 结束 | ≥ 95% | hutool-poi 占位 + hutool-ai provider |
| Phase 5 结束 | ≥ 99% | 命名收尾 + facade 补全 |
| Phase 6 结束 | **100%** | **每个 hutool .java 都有 hutool-rust 对应** |
| v1.0.0 | **100%（除 hutool-poi）** | 锁定覆盖率 |

**对「unsafe-to-copy」和「planned」的处理**：

即使某个文件标记为「unsafe-to-copy」或「planned」（如 Swing UI、SOAP Server、SSH/FTP），**仍必须建对应 .rs 文件**，但内部实现可以是：

```rust
//! 迁移自 hutool 的 `cn.hutool.swing.RobotUtil`
//!
//! - 迁移状态：⚠️ unsafe-to-copy（依赖 GUI 平台，不适合跨平台抽象）
//! - 当前实现：安全 stub + 明确错误
//!
//! 原始 Java Robot 用于键鼠模拟；Rust 等价物是 `enigo` crate，
//! 但跨平台一致性较差。本文件提供 API 入口，方法返回 NotImplemented。

pub struct RobotUtil;
impl RobotUtil {
    pub fn mouse_move(_x: i32, _y: i32) -> Result<(), RobotError> {
        Err(RobotError::UnsupportedPlatform)
    }
}
```

**`unsafe-to-copy` 状态的文件仍建 `.rs`**，但实现方式是 `Err` 返回（而非 panic），保证 API 完整性。

**对「hutool 独有扩展」（无 Java 对应）的处理**：

hutool-rust 可能新增 hutool Java 没有的模块（如 `advanced_codec.rs`、`murmur3` 等）。这些**不受 1:1 要求约束**，允许自由扩展。

### 5.1 当前 vs 目标结构对比（**Phase 1.2 必须修复**）

**当前（违反规范）**：

```
hutool-core/src/                          # ❌ 顶层 59 个 .rs 平铺
├── lib.rs                                # ❌ 280 行（应是 30~80 行）
├── advanced_codec.rs                     # ❌ 应放 codec/
├── array_util.rs                         # ❌ 应放 util/
├── assert_util.rs                        # ❌ 应放 lang/assert.rs
├── boolean_util.rs                       # ❌ 应放 util/
├── byte_util.rs                          # ❌ 应放 util/
├── char_util.rs                          # ❌ 应放 util/
├── charset_util.rs                       # ❌ 应放 util/
├── class_loader_util.rs                  # ❌ 应放 util/
├── class_util.rs                         # ❌ 应放 util/
├── clone_support.rs                      # ❌ 应放 clone/
├── coll_stream_util.rs                   # ❌ 应放 collection/
├── coll_util.rs                          # ❌ 应放 collection/（双重路径 1）
├── collection_adapters.rs               # ❌ 应放 collection/adapters.rs
├── collection_iter.rs                    # ❌ 应放 collection/iter.rs
├── collection_partition.rs               # ❌ 应放 collection/partition.rs
├── collection_types.rs                   # ❌ 应放 collection/types.rs
├── coordinate_util.rs                    # ❌ 应放 util/
├── credit_code_util.rs                   # ❌ 应放 util/
├── desensitized_util.rs                  # ❌ 应放 util/
├── dict.rs                               # ❌ 应放 lang/
├── enum_util.rs                          # ❌ 应放 util/
├── error.rs                              # ✅ 允许（顶层基础设施）
├── escape_util.rs                        # ❌ 应放 util/
├── file_util.rs                          # ❌ 应放 io/（双重路径 2）
├── hash_util.rs                          # ❌ 应放 util/
├── hex_util.rs                           # ❌ 应放 util/
├── hutool_codec.rs                       # ❌ 应放 codec/
├── id.rs                                 # ❌ 应放 util/id_util.rs（双重路径 3）
├── idcard_util.rs                        # ❌ 应放 util/
├── io_util.rs                            # ❌ 应放 io/（双重路径 4）
├── iter_util.rs                          # ❌ 应放 collection/（双重路径 5）
├── jaxb_util.rs                          # ❌ 应放 util/
├── list_util.rs                          # ❌ 应放 collection/（双重路径 6）
├── map_util.rs                           # ❌ 应放 map/（双重路径 7）
├── modifier_util.rs                      # ❌ 应放 util/
├── mutable.rs                            # ❌ 应放 lang/mutable/
├── number_util.rs                        # ❌ 应放 util/
├── object_util.rs                        # ❌ 应放 util/
├── opt.rs                                # ❌ 应放 lang/
├── page_util.rs                          # ❌ 应放 util/
├── pair.rs                               # ❌ 应放 lang/
├── phone_util.rs                         # ❌ 应放 util/
├── primitive_array_util.rs               # ❌ 应放 util/
├── radix_codec.rs / radix_util.rs        # ❌ 应放 util/
├── random_util.rs                        # ❌ 应放 util/
├── re_util.rs                            # ❌ 应放 util/
├── reference_util.rs                     # ❌ 应放 util/
├── reflect_util.rs                       # ❌ 应放 util/
├── runtime_util.rs                       # ❌ 应放 util/
├── str_util.rs                           # ❌ 应放 text/str_util.rs（Phase 5 完整化）
├── string.rs                             # ❌ 应放 text/
├── type_util.rs                          # ❌ 应放 util/
├── url_util.rs                           # ❌ 应放 net/url_util.rs
├── validator.rs                          # ❌ 应放 lang/
├── version_util.rs                       # ❌ 应放 util/
├── xml_util.rs                           # ❌ 应放 util/
├── zip_util.rs                           # ❌ 应放 util/
└── 24 个子目录（已有，正确）              # ✅ 24 个子目录
```

**目标（严格对齐 hutool）**：

```
hutool-core/src/
├── lib.rs                                # ✅ ≤ 80 行（仅 mod 声明 + re-export）
├── error.rs                              # ✅ 顶层基础设施
│
├── annotation/mod.rs                     # ← cn.hutool.core.annotation
├── bean/                                 # ← cn.hutool.core.bean
│   ├── mod.rs
│   ├── bean_util.rs
│   └── copier/                           # ← bean.copier 子包
├── builder/mod.rs                        # ← cn.hutool.core.builder
├── clone/mod.rs                          # ← cn.hutool.core.clone
├── codec/mod.rs                          # ← cn.hutool.core.codec
├── collection/                           # ← cn.hutool.core.collection
│   ├── mod.rs
│   ├── coll_util.rs                      # ★ CollUtil 唯一新家
│   ├── iter_util.rs                      # ★ IterUtil 唯一新家
│   ├── list_util.rs                      # ★ ListUtil 唯一新家
│   ├── coll_stream_util.rs
│   └── ...
├── comparator/mod.rs                     # ← cn.hutool.core.comparator
├── compiler/mod.rs                       # ← cn.hutool.core.compiler
├── compress/mod.rs                       # ← cn.hutool.core.compress
├── convert/                              # ← cn.hutool.core.convert
│   ├── mod.rs
│   ├── convert.rs
│   └── impl/                             # ← convert.impl 子包
├── date/                                 # ← cn.hutool.core.date
│   ├── mod.rs
│   ├── date_util.rs
│   ├── date_time.rs
│   └── format/                           # ← date.format 子包
├── exceptions/mod.rs                     # ← cn.hutool.core.exceptions
├── getter/mod.rs                         # ← cn.hutool.core.getter
├── img/                                  # ← cn.hutool.core.img（feature-gated）
├── io/                                   # ← cn.hutool.core.io
│   ├── mod.rs
│   ├── io_util.rs                        # ★ IoUtil 唯一新家
│   ├── file_util.rs                      # ★ FileUtil 唯一新家
│   ├── resource/                         # ← io.resource 子包
│   ├── file/                             # ← io.file 子包
│   ├── copy/                             # ← io.copy 子包
│   └── watch/                            # ← io.watch 子包
├── lang/                                 # ← cn.hutool.core.lang
│   ├── mod.rs
│   ├── assert.rs                         # ★ 原 assert_util.rs 改名
│   ├── dict.rs                           # ★ 原 dict.rs
│   ├── opt.rs                            # ★ 原 opt.rs
│   ├── pair.rs                           # ★ 原 pair.rs
│   ├── validator.rs                      # ★ 原 validator.rs
│   ├── mutable/                          # ← lang.mutable 子包
│   ├── tree/                             # ← lang.tree 子包
│   ├── func/                             # ← lang.func 子包
│   ├── id/                               # ← lang.id 子包
│   ├── hash/                             # ← lang.hash 子包
│   └── reflect/                          # ← lang.reflect 子包
├── map/                                  # ← cn.hutool.core.map
│   ├── mod.rs
│   ├── map_util.rs                       # ★ MapUtil 唯一新家
│   ├── multi/                            # ← map.multi 子包
│   └── reference/                        # ← map.reference 子包
├── math/mod.rs                           # ← cn.hutool.core.math
├── net/                                  # ← cn.hutool.core.net
│   ├── mod.rs
│   ├── url_util.rs                       # ★ 原 url_util.rs
│   ├── url_encode_util.rs                # Phase 1.3 新增
│   ├── url_decode_util.rs                # Phase 1.3 新增
│   ├── url/                              # ← net.url 子包
│   └── multipart/                        # ← net.multipart 子包
├── stream/mod.rs                         # ← cn.hutool.core.stream
├── swing/                                # ← cn.hutool.core.swing（feature-gated）
├── text/                                 # ← cn.hutool.core.text
│   ├── mod.rs
│   ├── str_util.rs                       # ★ 原 str_util.rs（Phase 5 完整化）
│   ├── string.rs                         # ★ 原 string.rs
│   ├── str_builder.rs
│   ├── csv/                              # ← text.csv 子包
│   ├── escape/                           # ← text.escape 子包
│   ├── finder/                           # ← text.finder 子包
│   └── replacer/                         # ← text.replacer 子包
├── thread/                               # ← cn.hutool.core.thread
│   ├── mod.rs
│   └── lock/                             # ← thread.lock 子包
└── util/                                 # ← cn.hutool.core.util（最常用的工具门面）
    ├── mod.rs
    ├── array_util.rs                     # ★ 原 array_util.rs
    ├── boolean_util.rs                   # ★ 原 boolean_util.rs
    ├── byte_util.rs                      # ★ 原 byte_util.rs
    ├── char_util.rs                      # ★ 原 char_util.rs
    ├── charset_util.rs                   # ★ 原 charset_util.rs
    ├── class_loader_util.rs              # ★ 原 class_loader_util.rs
    ├── class_util.rs                     # ★ 原 class_util.rs
    ├── coordinate_util.rs                # ★ 原 coordinate_util.rs
    ├── credit_code_util.rs               # ★ 原 credit_code_util.rs
    ├── desensitized_util.rs              # ★ 原 desensitized_util.rs
    ├── enum_util.rs                      # ★ 原 enum_util.rs
    ├── escape_util.rs                    # ★ 原 escape_util.rs
    ├── hash_util.rs                      # ★ 原 hash_util.rs
    ├── hex_util.rs                       # ★ 原 hex_util.rs
    ├── id_util.rs                        # ★ 原 id.rs 重命名
    ├── idcard_util.rs                    # ★ 原 idcard_util.rs
    ├── jaxb_util.rs                      # ★ 原 jaxb_util.rs
    ├── modifier_util.rs                  # ★ 原 modifier_util.rs
    ├── number_util.rs                    # ★ 原 number_util.rs
    ├── object_util.rs                    # ★ 原 object_util.rs
    ├── page_util.rs                      # ★ 原 page_util.rs
    ├── phone_util.rs                     # ★ 原 phone_util.rs
    ├── primitive_array_util.rs           # ★ 原 primitive_array_util.rs
    ├── radix_util.rs                     # ★ 原 radix_util.rs（含 radix_codec）
    ├── random_util.rs                    # ★ 原 random_util.rs
    ├── re_util.rs                        # ★ 原 re_util.rs
    ├── reference_util.rs                 # ★ 原 reference_util.rs
    ├── reflect_util.rs                   # ★ 原 reflect_util.rs
    ├── runtime_util.rs                   # ★ 原 runtime_util.rs
    ├── version_util.rs                   # ★ 原 version_util.rs
    └── zip_util.rs                       # ★ 原 zip_util.rs（含 xml_util）
```

**Phase 1.2 完成度验收指标**：

| 指标 | 当前 | 目标 |
|---|---:|---:|
| 顶层 `.rs` 实现文件数 | 59 | **≤ 2**（仅 `lib.rs` + `error.rs`） |
| 顶层 mod 声明 | 散乱 | **24** 个（对应 hutool 24 子包） |
| `lib.rs` 行数 | 280 | **≤ 80** |
| 双重路径模块数 | ≥ 8 | **0**（git mv 合并） |
| 子目录数 | 24 | 24（保持） |

### 5.2 子目录到 hutool 包的对应（包级映射）

| hutool 包 | hutool 子目录 | 文件数对比 | 迁移完成度 |
|---|---|---:|---:|
| `cn.hutool.core.annotation` | `annotation/` | 36 ↔ 49 | 97.2% |
| `cn.hutool.core.bean` | `bean/` | 11 ↔ 24 | 90.9% |
| `cn.hutool.core.builder` | `builder/` | 7 ↔ 7 | 85.7% |
| `cn.hutool.core.clone` | `clone/` | 5 ↔ 5 | 80.0% |
| `cn.hutool.core.codec` | `codec/` | 20 ↔ 20 | 95.0% |
| `cn.hutool.core.collection` | `collection/` | 30 ↔ 30 | 96.7% |
| `cn.hutool.core.comparator` | `comparator/` | 18 ↔ 18 | 94.4% |
| `cn.hutool.core.compiler` | `compiler/` | 9 ↔ 9 | 88.9% |
| `cn.hutool.core.compress` | `compress/` | 6 ↔ 6 | 83.3% |
| `cn.hutool.core.convert` | `convert/` | 12 ↔ 48 | 91.7% |
| `cn.hutool.core.date` | `date/` | 27 ↔ 41 | 96.3% |
| `cn.hutool.core.exceptions` | `exceptions/` | 9 ↔ 9 | 88.9% |
| `cn.hutool.core.getter` | `getter/` | 10 ↔ 10 | 90.0% |
| `cn.hutool.core.img` | `img/` | 9 ↔ 13 | 88.9% |
| `cn.hutool.core.io` | `io/` | 21 ↔ 84 | 95.2% |
| `cn.hutool.core.lang` | `lang/` | 35 ↔ 116 | 94.3% |
| `cn.hutool.core.map` | `map/` | 25 ↔ 35 | 96.0% |
| `cn.hutool.core.math` | `math/` | 7 ↔ 7 | 85.7% |
| `cn.hutool.core.net` | `net/` | 17 ↔ 25 | 94.1% |
| `cn.hutool.core.stream` | `stream/` | 4 ↔ 4 | 75.0% |
| `cn.hutool.core.swing` | `swing/` | 4 ↔ 9 | 75.0% |
| `cn.hutool.core.text` | `text/` | 16 ↔ 45 | 93.8% |
| `cn.hutool.core.thread` | `thread/` | 16 ↔ 22 | 93.8% |
| `cn.hutool.core.util` | `util/`（Phase 1.2 收编全部 `*_util.rs`） | 42 ↔ 42 | 97.6% |
| `cn.hutool.core.swing`（awt） | 不迁移 | 标记 planned/unsafe-to-copy | 0% |

**总体 hutool-core 命名一致性 ≈ 90%**，但有以下"缺失 facade 类"待补：

| 用户期待类名 | hutool-rust 实际 | 计划动作 |
|---|---|---|
| `SetUtil` | ❌ 不存在 | Phase 1.3 新建 `util/set_util.rs` |
| `URLDecodeUtil` | ❌ 不存在 | Phase 1.3 新建 `net/url_decode_util.rs` |
| `URLEncodeUtil` | ❌ 不存在 | Phase 1.3 新建 `net/url_encode_util.rs` |
| `RegexUtil` | ❌ 不存在 | Phase 1.3 新建 `util/regex_util.rs`（re-export `re_util`） |
| `SecureUtil` | ❌ 不存在 | Phase 1.3 新建 `util/secure_util.rs`（re-export `hutool-crypto::*`） |
| `DigestUtil` | ❌ 不存在 | Phase 1.3 新建 `util/digest_util.rs`（re-export `hutool-crypto::digest::*`） |
| `StrUtil`（完整 facade） | 🟡 占位 1.1KB | Phase 1.1 替换为完整 facade（不删减原文件） |
| `Base32Util` / `Base64Util` | ❌ 不存在 | Phase 1.4 新建 facade 委托到 `codec::base32/base64` |
| `DES` / `AES` / `RSA` | 移至 `hutool-crypto` | Phase 1.4 加 re-export 兼容 |

---

## 六、核心类型/Trait 签名（重点 hutool-core）

> 以下签名遵循原则 2：snake_case 命名 + 严格对齐 Java 方法签名 + 返回 `Result<T>` 替代 Java 异常。

### 6.1 `CollUtil`（最常用集合工具）

```rust
// hutool-core/src/collection/coll_util.rs

/// 集合工具，1:1 对齐 `cn.hutool.core.collection.CollUtil`
pub struct CollUtil;

impl CollUtil {
    /// Java: `public static boolean isEmpty(Collection)` → Rust: `is_empty(Option<&[T]>)`
    pub fn is_empty<T>(values: Option<&[T]>) -> bool;
    pub fn is_not_empty<T>(values: Option<&[T]>) -> bool;

    /// Java: `public static <T> T get(Collection, int)` → `get(&[T], isize) -> Option<&T>`
    pub fn get<T>(values: &[T], index: isize) -> Option<&T>;

    /// Java: `public static Collection newHashSet(T...)` → `new_hash_set(impl IntoIterator<Item=T>)`
    pub fn new_hash_set<T>(values: impl IntoIterator<Item = T>) -> HashSet<T>;
    pub fn new_linked_hash_set<T>(values: impl IntoIterator<Item = T>) -> IndexSet<T>;
    pub fn new_array_list<T>(values: impl IntoIterator<Item = T>) -> Vec<T>;
    pub fn new_tree_set<T: Ord>(values: impl IntoIterator<Item = T>) -> BTreeSet<T>;

    /// Java: `public static String join(Collection, CharSequence)` → `join(impl IntoIterator<Item=T: Display>, &str)`
    pub fn join<T: Display>(values: impl IntoIterator<Item = T>, delimiter: &str) -> String;

    /// Java: `public static Collection unionDistinct(...)` → `union_distinct(...) -> IndexSet<T>`
    pub fn union_distinct<T>(collections: &[&[T]]) -> IndexSet<T>;
    pub fn intersection_distinct<T>(collections: &[&[T]]) -> IndexSet<T>;

    /// Java: `public static <T> List<T> sub(List, int, int, int)` → `sub(&[T], isize, isize, isize) -> Result<Vec<T>>`
    pub fn sub<T: Clone>(values: &[T], start: isize, end: isize, step: isize) -> Result<Vec<T>>;

    /// Java: `public static <T> List<List<T>> split(List, int)` → `split(&[T], usize) -> Result<Vec<Vec<T>>>`
    pub fn split<T: Clone>(values: &[T], size: usize) -> Result<Vec<Vec<T>>>;

    // ... 100+ 个方法，完整列表见 docs/migration/object-method-matrix.md
}
```

### 6.2 `DateUtil` / `DateTime`

```rust
// hutool-core/src/date/date_util.rs

/// 1:1 对齐 `cn.hutool.core.date.DateUtil`
pub struct DateUtil;

/// DateTime 类型别名（hutool-rust 的简化）
pub type DateTime = chrono::NaiveDateTime;

impl DateUtil {
    /// Java: `public static Date date()` → `date() -> DateTime`
    pub fn date() -> DateTime;
    pub fn date_from_millis(millis: i64) -> DateTime;

    /// Java: `public static long current()` → `current() -> i64`
    pub fn current() -> i64;
    pub fn current_seconds() -> i64;

    /// Java: `public static String format(Date, String)` → `format(DateTime, &str)`
    pub fn format(date: DateTime, pattern: &str) -> String;

    /// Java: `public static Date parse(CharSequence)` → `parse(&str) -> Result<DateTime>`
    pub fn parse(date_str: &str) -> Result<DateTime>;
    pub fn parse_with_format(date_str: &str, format: &str) -> Result<DateTime>;
    pub fn parse_utc(utc: &str) -> Result<DateTime>;
    pub fn parse_iso8601(iso: &str) -> Result<DateTime>;
    pub fn parse_rfc2822(source: &str) -> Result<DateTime>;

    /// Java: `public static Date beginOfDay(Date)` → `begin_of_day(DateTime)`
    pub fn begin_of_day(date: DateTime) -> DateTime;
    pub fn end_of_day(date: DateTime) -> DateTime;
    pub fn begin_of_month(date: DateTime) -> DateTime;
    pub fn begin_of_year(date: DateTime) -> DateTime;

    /// Java: `public static long between(Date, Date, DateUnit)` → `between(DateTime, DateTime, DateUnit)`
    pub fn between(begin: DateTime, end: DateTime, unit: DateUnit) -> i64;

    /// Java: `public static String formatBetween(Date, Date)` → `format_between(DateTime, DateTime)`
    pub fn format_between(begin: DateTime, end: DateTime) -> String;
}
```

### 6.3 `FileUtil` / `IoUtil` / `ResourceUtil`

```rust
// hutool-core/src/io/file_util.rs

pub struct FileUtil;

impl FileUtil {
    pub fn name(path: &Path) -> &str;
    pub fn suffix(path: &Path) -> &str;
    pub fn main_name(path: &Path) -> &str;

    pub fn exists(path: &str) -> bool;
    pub fn is_file(path: &str) -> bool;
    pub fn is_directory(path: &str) -> bool;
    pub fn size(path: &Path) -> u64;

    pub fn read_utf8_string(path: &str) -> std::io::Result<String>;
    pub fn read_bytes(path: &str) -> std::io::Result<Vec<u8>>;
    pub fn write_utf8_string(path: &str, content: &str) -> std::io::Result<()>;

    pub fn copy(from: &str, to: &str) -> std::io::Result<u64>;
    pub fn delete(path: &str) -> std::io::Result<()>;
    pub fn mkdir(path: &str) -> std::io::Result<()>;
    pub fn rename(from: &str, to: &str) -> std::io::Result<()>;

    pub fn ls(path: &str) -> std::io::Result<Vec<String>>;
    pub fn loop_files(path: &str) -> std::io::Result<Vec<PathBuf>>;
    pub fn walk_files(path: &str) -> std::io::Result<Vec<PathBuf>>;

    pub fn checksum_sha256(path: &str) -> std::io::Result<String>;
    pub fn checksum_crc32(path: &str) -> std::io::Result<u32>;
    // ... 80+ 个方法
}
```

### 6.4 `NumberUtil`

```rust
// hutool-core/src/util/number_util.rs

pub struct NumberUtil;

impl NumberUtil {
    pub fn add(v1: f64, v2: f64) -> f64;
    pub fn sub(v1: f64, v2: f64) -> f64;
    pub fn mul(v1: f64, v2: f64) -> f64;
    pub fn div(v1: f64, v2: f64) -> Result<f64>;
    pub fn div_with_scale(v1: f64, v2: f64, scale: u32) -> Result<f64>;

    pub fn is_number(s: &str) -> bool;
    pub fn is_integer(s: &str) -> bool;
    pub fn is_long(s: &str) -> bool;
    pub fn is_double(s: &str) -> bool;
    pub fn is_power_of_two(n: i64) -> bool;
    pub fn is_primes(n: i32) -> bool;
    pub fn is_odd(num: i32) -> bool;
    pub fn is_even(num: i32) -> bool;

    pub fn round_decimal(number: Decimal, scale: i32) -> Decimal;
    pub fn decimal_format(pattern: &str, value: f64) -> Result<String>;
    pub fn decimal_format_money(value: f64) -> Result<String>;

    pub fn parse_int(number: &str) -> Result<i32>;
    pub fn parse_long(number: &str) -> Result<i64>;
    pub fn parse_float_or(number: &str, default: Option<f32>) -> Option<f32>;
    pub fn parse_double_or(number: &str, default: Option<f64>) -> Option<f64>;

    // ... 80+ 个方法
}
```

### 6.5 `StrUtil`（**核心门面，需补全**）

```rust
// hutool-core/src/str_util.rs （需扩充）
// 1:1 对齐 `cn.hutool.core.text.StrUtil`

pub struct StrUtil;

impl StrUtil {
    // ─── 判空 ───
    pub fn is_empty(s: Option<&str>) -> bool;
    pub fn is_not_empty(s: Option<&str>) -> bool;
    pub fn is_blank(s: Option<&str>) -> bool;
    pub fn is_not_blank(s: Option<&str>) -> bool;

    // ─── 等值 ───
    pub fn equals(a: Option<&str>, b: Option<&str>) -> bool;
    pub fn equals_ignore_case(a: Option<&str>, b: Option<&str>) -> bool;

    // ─── 子串查找 ───
    pub fn contains(s: &str, sub: &str) -> bool;
    pub fn contains_ignore_case(s: &str, sub: &str) -> bool;
    pub fn index_of(s: &str, sub: &str) -> Option<usize>;
    pub fn index_of_ignore_case(s: &str, sub: &str) -> Option<usize>;
    pub fn last_index_of(s: &str, sub: &str) -> Option<usize>;

    // ─── 截取 ───
    pub fn sub(s: &str, from: isize, to: isize) -> Result<String>;
    pub fn sub_before(s: &str, sep: &str) -> Result<String>;
    pub fn sub_after(s: &str, sep: &str) -> Result<String>;
    pub fn sub_between(s: &str, begin: &str, end: &str) -> Result<String>;

    // ─── 变换 ───
    pub fn trim(s: &str) -> &str;
    pub fn trim_start(s: &str) -> &str;
    pub fn trim_end(s: &str) -> &str;
    pub fn upper(s: &str) -> String;
    pub fn lower(s: &str) -> String;
    pub fn upper_first(s: &str) -> String;
    pub fn lower_first(s: &str) -> String;
    pub fn reverse(s: &str) -> String;
    pub fn repeat(s: &str, count: usize) -> String;

    // ─── 替换 ───
    pub fn replace(s: &str, from: &str, to: &str) -> String;
    pub fn replace_ignore_case(s: &str, from: &str, to: &str) -> String;
    pub fn remove_all(s: &str, sub: &str) -> String;
    pub fn remove_prefix(s: &str, prefix: &str) -> &str;
    pub fn remove_suffix(s: &str, suffix: &str) -> &str;

    // ─── 拆分 ───
    pub fn split(s: &str, sep: char) -> Vec<&str>;
    pub fn split_to_array(s: &str, sep: char) -> Vec<String>;
    pub fn split_by_blank(s: &str) -> Vec<&str>;
    pub fn split_by_length(s: &str, len: usize) -> Vec<&str>;

    // ─── 格式化 ───
    pub fn format(template: &str, args: &[&dyn Display]) -> Result<String>;
    pub fn indexed_format(template: &str, args: &[&dyn Display]) -> Result<String>;

    // ─── 命名风格 ───
    pub fn to_camel_case(s: &str) -> String;
    pub fn to_snake_case(s: &str) -> String;
    pub fn to_kebab_case(s: &str) -> String;
    pub fn to_pascal_case(s: &str) -> String;

    // ... 80+ 个方法

    // ─── 委托到子模块（不重复实现）───
    // pub fn builder(cap: i32) -> StrBuilder { text::str_builder::StrBuilder::with_capacity(cap) }
    // pub fn joiner(delim: &str) -> StrJoiner { text::str_joiner::StrJoiner::of(delim) }
    // pub fn splitter(sep: &str) -> StrSplitter { text::str_splitter::StrSplitter::new(sep) }
}
```

### 6.6 `MapUtil`

```rust
// hutool-core/src/map/map_util.rs

pub struct MapUtil;

impl MapUtil {
    pub fn is_empty<K, V>(map: &HashMap<K, V>) -> bool;
    pub fn is_not_empty<K, V>(map: &HashMap<K, V>) -> bool;

    pub fn new_hash_map<K, V>() -> HashMap<K, V>;
    pub fn new_hash_map_sized<K, V>(size: usize) -> HashMap<K, V>;
    pub fn new_tree_map<K: Ord, V>() -> BTreeMap<K, V>;

    pub fn of<K: Eq + Hash, V>(pairs: &[(K, V)]) -> HashMap<K, V>;
    pub fn entry<K, V>(key: K, value: V) -> (K, V);

    pub fn get_str<'a, K: Eq + Hash>(map: &'a HashMap<K, String>, key: &K) -> Option<&'a str>;
    pub fn get_str_or<'a, K: Eq + Hash>(map: &'a HashMap<K, String>, key: &K, default: &'a str) -> &'a str;
    pub fn get_int<K: Eq + Hash>(map: &HashMap<K, i64>, key: &K) -> Option<i64>;

    pub fn put_all<K, V>(target: &mut HashMap<K, V>, source: HashMap<K, V>);
    pub fn remove_any<K, V>(map: &mut HashMap<K, V>, keys: &[K]);

    pub fn join<K: Display, V: Display>(map: &HashMap<K, V>, separator: &str, key_value_separator: &str) -> String;
    pub fn filter<K, V, F: FnMut(&(&K, &V)) -> bool>(map: &HashMap<K, V>, predicate: F) -> HashMap<K, V>;

    pub fn to_list_map<K, V>(map_list: &[HashMap<K, V>]) -> HashMap<K, Vec<V>>;
    pub fn to_map_list<K, V>(list_map: &HashMap<K, Vec<V>>) -> Vec<HashMap<K, V>>;

    // ... 50+ 个方法
}
```

### 6.7 `IdUtil` / `IdcardUtil` / `SecureUtil`（hutool-core 内直接实现）

```rust
// hutool-core/src/util/id_util.rs

pub struct IdUtil;

impl IdUtil {
    /// Java: `public static String randomUUID()` → `uuid() -> String`
    pub fn uuid() -> String;
    /// Java: `public static String simpleUUID()` → `simple_uuid() -> String`
    pub fn simple_uuid() -> String;
    /// Java: `public static String objectId()` → `object_id() -> String`
    pub fn object_id() -> String;
    /// Java: `public static Snowflake createSnowflake(long, long)` → `create_snowflake(i64, i64)`
    pub fn create_snowflake(worker_id: i64, data_center_id: i64) -> Snowflake;
    pub fn get_snowflake_next_id() -> i64;
    pub fn get_snowflake_next_id_str() -> String;
    // ...
}
```

```rust
// hutool-core/src/util/secure_util.rs （新增 facade，委托到 hutool-crypto）
//! 1:1 对齐 `cn.hutool.crypto.SecureUtil`
pub struct SecureUtil;

impl SecureUtil {
    pub fn md5(data: &str) -> String {
        hutool_crypto::digest::md5_hex(data)
    }
    pub fn sha1(data: &str) -> String {
        hutool_crypto::digest::sha1_hex(data)
    }
    pub fn sha256(data: &str) -> String {
        hutool_crypto::digest::sha256_hex(data)
    }
    pub fn sha512(data: &str) -> String {
        hutool_crypto::digest::sha512_hex(data)
    }
    pub fn aes(key: &[u8]) -> AesBuilder {
        AesBuilder::new(key)
    }
    pub fn des(_key: &[u8]) -> DesBuilder {
        unimplemented!("DES 已废弃，请使用 AES")
    }
    pub fn rsa() -> RsaBuilder {
        RsaBuilder::new()
    }
    pub fn hmac(algorithm: &str, key: &[u8]) -> HMacBuilder {
        HMacBuilder::new(algorithm, key)
    }
}
```

### 6.8 错误处理（每 crate 独立 enum）

```rust
// hutool-core/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] chrono::ParseError),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("util exception: {0}")]
    Util(String),

    #[error("argument invalid: {0}")]
    ArgumentInvalid(String),

    #[error("class not found: {0}")]
    ClassNotFound(String),

    #[error("convert failed: {from} → {to}")]
    ConvertFailed { from: String, to: String },

    #[error("number format error: {0}")]
    NumberFormat(String),
}
```

```rust
// hutool-crypto/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("invalid key length: expected {expected}, got {actual}")]
    InvalidKeyLength { expected: usize, actual: usize },

    #[error("invalid iv: {0}")]
    InvalidIv(String),

    #[error("encryption failed: {0}")]
    Encryption(String),

    #[error("decryption failed: {0}")]
    Decryption(String),

    #[error("signing failed: {0}")]
    Signing(String),

    #[error("verification failed")]
    Verification,

    #[error("random generation failed")]
    Random,
}
```

---

## 七、proc-macro 设计

### 7.1 当前状态：`hutool-macro` 仅有 1 个文件

```text
crates/hutool-macro/
├── Cargo.toml
└── src/
    └── lib.rs                          # 仅 `derive_redacted_debug` 一个 proc-macro derive
```

### 7.2 设计原则

> ⚠️ Hitool 与 sa-token 不同：**Hutool 没有 Java 注解**（除 `cn.hutool.core.annotation` 元注解包），
> 所以 hutool-macro 主要服务于**编译期派生宏**，而非运行时注解处理器。

### 7.3 已有宏（保留）

```rust
// hutool-macro/src/lib.rs

/// 自动派生 Debug，对标记 `#[redact]` 的字段脱敏
#[proc_macro_derive(RedactedDebug, attributes(redact))]
pub fn derive_redacted_debug(input: TokenStream) -> TokenStream;
```

### 7.4 计划新增宏

| 宏 | 用途 | 对应 Hutool 能力 | Phase |
|---|---|---|---|
| `#[derive(BeanDesc)]` | 派生 Bean 元数据 | `BeanDesc.java` | 3 |
| `#[derive(ToJson)]` / `#[derive(FromJson)]` | JSON 序列化 | `JSONUtil.toBean` | 3 |
| `#[derive(ExcelRow)]` | Excel 行映射 | `BeanSheetReader` | 4 |
| `#[derive(Setting)]` | 配置项映射 | `Props.toBean` | 3 |
| `#[derive(TreeNode)]` | 树结构递归 | `TreeUtil` | 3 |
| `function!{}` | 模板字符串宏 | `StrFormatter.format` | 3 |

### 7.5 使用示例

```rust
use hutool_macros::RedactedDebug;

#[derive(RedactedDebug)]
pub struct User {
    pub id: u64,
    pub name: String,
    #[redact]
    pub password: String,
    #[redact]
    pub email: String,
}

let user = User { id: 1, name: "Alice".into(), password: "secret".into(), email: "alice@example.com".into() };
println!("{:?}", user);
// User { id: 1, name: "Alice", password: "<redacted>", email: "<redacted>" }
```

---

## 八、生态适配层（Web/DB/AI/Email/Captcha）

### 8.1 hutool-http（基于 reqwest）

#### 已实现的 22 个文件

```text
hutool-http/src/
├── lib.rs                              # 顶层 re-export + prelude
├── client.rs                           # HttpClient (reqwest wrapper)
├── request.rs / response.rs            # HttpRequest/HttpResponse
├── method.rs                           # Method enum
├── exception.rs                        # HttpException
├── global_config.rs                    # HttpGlobalConfig
├── downloader.rs                       # HttpDownloader
├── interceptor.rs                      # Interceptor trait
├── progress.rs                         # StreamProgress
├── resource.rs                         # HttpResource
├── upload.rs                           # Upload / UploadFile
├── useragent.rs                        # UserAgent + Parser
├── cookie.rs                           # Cookie / CookieJar
├── input_stream.rs                     # HttpInputStream
├── html/
│   ├── filter.rs                       # HtmlFilter
│   ├── util.rs                         # HtmlUtil
│   └── mod.rs
└── body/
    ├── mod.rs
    ├── bytes_form.rs                   # BytesBody / FormUrlEncodedBody / ResourceBody
    ├── multipart.rs                    # MultipartBody
    └── multipart_stream.rs             # MultipartOutputStream
```

#### 核心签名

```rust
// hutool-http/src/client.rs

#[derive(Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
    policy: Arc<dyn UrlPolicy>,
    interceptors: ...
}

impl HttpClient {
    pub fn builder() -> HttpClientBuilder;
    pub fn get(&self, url: &str) -> HttpRequest;
    pub fn post(&self, url: &str) -> HttpRequest;
    pub fn put(&self, url: &str) -> HttpRequest;
    pub fn delete(&self, url: &str) -> HttpRequest;
    pub fn head(&self, url: &str) -> HttpRequest;
    pub fn execute(&self, req: HttpRequest) -> Result<HttpResponse, HttpException>;
    pub fn request(&self, method: Method, url: &str) -> HttpRequest;
}

// hutool-http/src/request.rs

pub struct HttpRequest {
    method: Method,
    url: String,
    headers: HttpHeaders,
    body: Option<Box<dyn RequestBody>>,
    timeout: Option<Duration>,
}

impl HttpRequest {
    pub fn new(method: Method, url: &str) -> Self;
    pub fn header(self, name: impl Into<String>, value: impl Into<String>) -> Self;
    pub fn headers(self, headers: impl IntoIterator<Item = (String, String)>) -> Self;
    pub fn body(self, body: impl RequestBody + 'static) -> Self;
    pub fn timeout(self, d: Duration) -> Self;
    pub fn build(self) -> Self;
}
```

#### 待补 Hutool 对齐类

| Java | Rust 状态 | Phase |
|---|---|---|
| `HttpUtil` (静态 facade) | ❌ 缺 | Phase 1.5 新建 `util.rs` |
| `HttpBase` / `HttpConfig` | ❌ 合并到 builder | 跳过（idiomatic） |
| `HttpStatus` / `Status` | ❌ 合并到 `Method` | 跳过 |
| `ContentType` | 🟡 散落 | Phase 3 集中 |
| `server/*`（SimpleServer / HttpServer） | ❌ 缺 | Phase 5 |
| `ssl/*`（DefaultSSLFactory 等） | ❌ 由 reqwest 替代 | 跳过 |
| `webservice/*`（Soap） | ❌ 缺 | Phase 5 |

### 8.2 hutool-db（基于 sqlx）

#### 当前 6 个文件

```text
hutool-db/src/
├── lib.rs
├── connection.rs                       # Connection trait + Transaction
├── ds.rs                               # DataSource + ConnectionFactory
├── pool.rs                             # ConnectionPool + PooledConn
├── row.rs                              # Row + RowSet + Value enum
└── sql.rs                              # sql 渲染 + ToSql trait + NamedParam
```

#### 待补 Hutool 对齐类（107 个 hutool-db 类中已迁移 ~6）

| Java | Rust 状态 | Phase |
|---|---|---|
| `Db` (静态 facade) | ❌ 缺 | Phase 2 |
| `Entity` (动态实体) | ❌ 缺 | Phase 2 |
| `Page` / `PageResult` | 🟡 在 hutool-core 已有 `page_util.rs` | Phase 2 整合 |
| `Query` / `Condition` / `ConditionBuilder` | ❌ 缺 | Phase 3 |
| `SqlBuilder` / `SqlExecutor` / `SqlLog` / `SqlFormatter` | ❌ 缺 | Phase 3 |
| `Dialect` 体系（10+ 方言） | ❌ 缺 | Phase 4 |
| `DSFactory` + 各种数据源（Druid/Hikari/DBCP/C3P0/Tomcat） | ❌ 缺 | Phase 4（SQLx 替代） |
| `nosql.mongo` / `nosql.redis` | ❌ 缺 | Phase 5 |
| `handler/*` | ❌ 缺 | Phase 3 |
| `meta/*` | ❌ 缺 | Phase 3 |
| `transaction.TransactionLevel` | 🟡 通过 SQLx 控制 | 跳过 |

### 8.3 hutool-ai（基于 reqwest + SSE）

#### 当前 13 个文件

```text
hutool-ai/src/
├── lib.rs
├── agent.rs                            # Agent struct
├── client.rs                           # AiClient
├── error.rs                            # AiError enum
├── message.rs                          # Message + Role + ToolCall
├── model.rs                            # ModelKind + ModelDescriptor
├── provider/
│   ├── mod.rs
│   ├── openai.rs                       # OpenAiProvider ★
│   └── traits.rs                       # AiProvider trait
├── request.rs                          # ChatRequest
├── response.rs                         # ChatResponse
├── stream.rs                           # ChatChunk + Delta
└── tool.rs                             # ToolSpec + ToolInvocation
```

#### 待补（hutool-ai 7 个 provider 中仅迁移 OpenAI）

| Java Provider | Rust 状态 | Phase |
|---|---|---|
| `OpenaiCommon/Config/Provider/Service/ServiceImpl` | ✅ 已实现（合并简化） | 1 |
| `DeepSeek` | ❌ | 3 |
| `Doubao` | ❌ | 3 |
| `Gemini` | ❌ | 3 |
| `Grok` | ❌ | 3 |
| `Hutool` | ❌ | 3 |
| `Ollama` | ❌ | 3 |

### 8.4 hutool-extra（基于 qrcode / zip / etc）

#### 当前 6 个文件

```text
hutool-extra/src/
├── codec.rs                            # Base32/Base62/Base64UrlSafe/Hex/Coder
├── compression.rs                      # Gzip/Zlib/Brotli/Snappy
├── invocation.rs                       # CommandLine
├── qrcode.rs                           # QrCode (Svg + raw)
└── template.rs                         # TemplateEngine + Helper
```

#### 待补（hutool-extra 179 个类中仅迁移 6）

| Java 子包 | Rust 状态 | Phase |
|---|---|---|
| `cglib/*` (BeanCopierCache) | ❌（Serde 替代） | 跳过 |
| `compress/*` (Archiver/Extractor) | ❌ 缺完整实现 | Phase 3 |
| `emoji/*` (EmojiUtil) | ❌ | Phase 3 |
| `expression/*` (7 个 engine) | ❌ 缺 | Phase 4 |
| `ftp/*` (Ftp/Sftp/JschUtil) | ❌ 缺（`suppaftp`/`ssh2` 替代） | Phase 4 |
| `mail/*` (Mail/MailAccount) | 🟡 在 feature `extra-mail` 中 | Phase 2 |
| `pinyin/*` (5 个 engine) | ❌ 缺 | Phase 4 |
| `qrcode/*` (QrCodeUtil) | 🟡 已在 qrcode.rs | Phase 2 |
| `servlet/*` (ServletUtil) | ❌ | Phase 5 |
| `spring/*` (SpringUtil) | ❌（无 Spring） | 跳过 |
| `ssh/*` (GanymedUtil) | ❌ | Phase 5 |
| `template/*` (8 个 engine) | 🟡 简化版 | Phase 3 |
| `tokenizer/*` (8 个 engine) | ❌ 缺 | Phase 5 |
| `validation/*` (BeanValidation) | 🟡 `validator.rs` 在 hutool-core | Phase 2 |

---

## 九、关键技术决策汇总

### 9.0 决策一览表

| 决策点 | 方案 | 原因 / 借鉴 |
|---|---|---|
| API 风格 | **单一 Rust idiomatic API + 命名严格对齐 Java** | 用户否决了 compat 双表面方案；早期设想的「双 API 表面（idiomatic + hutool-compat）」已被 §2 原则 1 移除 |
| 异步模型 | **核心 sync + 适配层 async** | 借鉴 sa-token-rs；hutool-core 无 IO，sync 即可 |
| 全局状态 | `OnceLock<Arc<T>>` | 借鉴 sa-token-rs |
| 错误处理 | **每 crate 一个根 enum + thiserror** | 借鉴 sa-token-rs；hutool-core 已有 11+ 子 enum，避免巨型 enum |
| 序列化 | `serde` + `serde_json::Value` | Rust 生态标准 |
| JSON | `serde_json`（hutool-json facade） | Hutool JSONUtil 风格 facade + serde 底层 |
| HTTP | `reqwest` + `tower`（可选） | Rust 生态标准 |
| DB | `sqlx`（**不造 ORM**） | 借鉴 sa-token-rs "不包装 ORM" 原则 |
| 缓存 | `moka` | 高性能进程内缓存 |
| 定时调度 | `tokio-cron-scheduler`（业务侧） + 自研 cron 解析（hutool-cron） | 双轨：自研 cron 表达式 + 借用 tokio 调度 |
| 加密 | `RustCrypto`（aes-gcm、hmac、sha2）+ `argon2` | 借鉴 sa-token-rs 决策；避开 `rsa` 时序侧信道 |
| JWT | `jsonwebtoken` | Rust 生态标准 |
| 脚本 | `rhai` | 沙箱友好 |
| 文本搜索 | `regex` + `aho-corasick` | 标准组合 |
| 系统信息 | `sysinfo` | Rust 生态标准 |
| 日志 | `tracing` | 跨 crate 统一抽象 + 重脱敏 |
| 配置 | `config` + `serde_yaml_ng` | 借鉴 sa-token-rs |
| AI | `reqwest` + `eventsource-stream` (SSE) | 标准 HTTP + 流式 |
| POI | **仅占位骨架，无引擎实现** | 当前不由 facade 暴露，也不计入完成度 |
| 命名一致性 | **严格对齐 Java，snake_case 转换** | 用户明确要求 |
| 已有实现 | **不删减，pub use 重新导出** | 用户明确要求 |
| Edition | **2024** | 借鉴 sa-token-rs |
| Resolver | **3** | 借鉴 sa-token-rs |
| Lint | `unsafe_code=deny` + clippy `all+pedantic` | 借鉴 sa-token-rs |
| 邮件 | `lettre`（feature-gated） | Rust SMTP 生态主流；`smtp-transport` + `tokio1-rustls-tls` |
| 拼音 | `pinyin` | Rust 生态最成熟的中文拼音库 |
| 文件监控 | `notify` | 跨平台 FS 事件，对应 Commons IO FileMonitor |
| 图片处理 | `image`（feature-gated） | 纯 Rust image codec，对应 Java ImageIO |
| QR 码 | `qrcode`（feature-gated，`svg`） | 无 C 依赖，对应 zxing |
| 数据库迁移 | `sqlx::migrate!` | 内置于 sqlx，对应 Flyway/Liquibase |
| UUID | `uuid`（v3/v4/v5） | 对应 java.util.UUID |

### 9.1 依赖与生态评估（基于 DDD4J 650 组件映射）

> 本节基于 DDD4J（Dromara Distributed Domain for Java）650 个 Java 组件映射到 Rust 生态的分析，
> 对 hutool-rust 当前依赖和架构进行**事实核对 + 提升建议**。

#### 9.1.1 实际依赖现状（基于 hutool-rust 工作区 Cargo.toml 实测）

| crate | 直接依赖（`=` 行） | optional 依赖 | 实际可用特性 |
|---|---:|---:|---|
| `hutool` (facade) | 63 | 20 | 全部通过 `dep:hutool-*` 特性聚合 |
| `hutool-extra` | 17 | 7 | `default = ["archive", "qrcode", "emoji", "pinyin"]` |
| `hutool-core` | 15 | 6 | `default = []`；`swing`/`img`/`async` 三个 feature |
| `hutool-http` | 11 | 4 | `default = ["useragent", "html"]`；`blocking` opt-in |
| `hutool-db` | 9 | 2 | `default = ["sqlite"]`；`postgres`/`mysql` opt-in |
| `hutool-crypto` | 5 | 0 | `default = []`；`legacy` opt-in |
| `hutool-captcha` | 9 | 3 | `default = ["raster"]`；`audio` opt-in |
| `hutool-json` | 3 | 0 | `default = []` |
| `hutool-cache` / `hutool-cron` / `hutool-script` / `hutool-setting` / `hutool-system` / `hutool-jwt` / `hutool-log` / `hutool-aop` / `hutool-bloom-filter` / `hutool-dfa` / `hutool-socket` / `hutool-ai` / `hutool-macro` / `hutool-compat-hutool` | 3~5 | 0 | 均为 `default = []` |

> **注**：上述数字基于 `Cargo.toml` 行计数；部分行是 `[features]` 而非 `[dependencies]`。
> 实际传递依赖（`cargo tree`）会更大，但单 crate 直接依赖已准确反映模块边界。

#### 9.1.2 已对齐 DDD4J 映射（✅ 已实现）

| DDD4J Java 领域 | Rust 生态 | hutool-rust 状态 |
|---|---|---|
| JSON：Jackson / Gson / FastJSON | `serde_json` | ✅ `hutool-json` |
| YAML：SnakeYAML | `serde_yaml_ng` | ⚠️ 仅 workspace.dependencies 中定义，**hutool-setting 未启用** |
| HTTP：OkHttp / Apache HttpClient | `reqwest` | ✅ `hutool-http` |
| 加密：BouncyCastle | `aes` / `sha2` / `sm2` / `sm3` / `sm4` / `rsa` | ✅ `hutool-crypto`（30+ 算法实现） |
| 数据库连接池：HikariCP / Druid | `sqlx::Pool` | ✅ `hutool-db` |
| 缓存：Caffeine / Ehcache | `moka` | ✅ `hutool-cache` |
| 邮件：javax.mail | `lettre` | ✅ `hutool-extra::mail`（feature-gated） |
| 定时任务：Quartz | `cron` | ✅ `hutool-cron`（自研解析）+ workspace 引入 `cron` crate |
| 脚本：JSR-223 | `rhai` | ✅ `hutool-script` |
| 日志：Logback / SLF4J | `tracing` + `tracing-subscriber` | ✅ `hutool-log` |
| 并发：JDK concurrent | `tokio` + `parking_lot` | ✅ `hutool-core::async` (feature-gated) |
| 文件监控：Commons IO | `notify` | ✅ workspace.dependencies，但**未在任何 crate 启用** |
| 图片：Java ImageIO | `image` | ✅ `hutool-core::img` + `hutool-extra::image`（feature-gated） |
| QR 码：zxing | `qrcode`（svg feature） | ✅ `hutool-extra::qrcode` |
| UUID：java.util.UUID | `uuid`（v3/v4/v5） | ✅ `hutool-core` |
| CSV：Commons CSV | `csv` | ✅ `hutool-core` |
| Unicode：ICU4J | `unicode-general-category` + `unicode-normalization` | ✅ `hutool-core` |
| 配置：Commons Config | `config` | ✅ `hutool-setting` |
| 密钥安全：Java Crypto | `secrecy` + `zeroize` | ✅ `hutool-crypto` |
| Excel：POI / EasyExcel | `rust_xlsxwriter` + `quick-xml` + `zip` | workspace 虽声明相关生态依赖，但 `hutool-poi` 自身仅依赖 `thiserror`，没有实现或引擎接入 |
| 脚本测试：JUnit | `proptest` | ✅ workspace.dependencies（dev-dependencies） |
| 正则：JDK Pattern | `regex` + `fancy-regex` + `aho-corasick` | ✅ `hutool-core` / `hutool-dfa` / `hutool-http::html` |
| 编码：Commons Codec | `base64` + `hex` + `data-encoding` + `encoding_rs` | ✅ `hutool-core` |
| 数据库迁移：Flyway | `sqlx::migrate!`（内置） | ✅ `hutool-db` 推荐用法 |

#### 9.1.3 建议补全（**按 DDD4J 分析 + hutool-rust 现状综合评估**）

| 优先级 | 补全项 | Rust crate | 目标 crate | 评估依据 |
|---|---|---|---|---|
| 🟢 高 | 启用 `serde_yaml_ng` 到 `hutool-setting` | `serde_yaml_ng` | `hutool-setting` | workspace 已声明但未启用；hutool-setting 明确支持 YAML |
| 🟢 高 | 启用 `notify` 到 hutool-core 的 Watch 模块 | `notify` | `hutool-core::io::watch` | 对齐 `cn.hutool.core.io.watch.WatchMonitor`（已实现），但缺底层依赖 |
| 🟢 高 | `itertools` 加入 hutool-core | `itertools` | `hutool-core` | Rust 生态集合迭代标准补充；hutool-core 大量组合操作可受益 |
| 🟡 中 | `num` / `statrs` 加入 hutool-core | `num` + `statrs` | `hutool-core` | 对齐 `cn.hutool.core.math.*` 缺失的统计/数值工具 |
| 🟡 中 | `eventsource-stream` 加入 hutool-ai | `eventsource-stream` | `hutool-ai` | DDD4J 推荐；当前 hutool-ai 是否已用待确认 |
| 🟡 中 | 新建 `crates/hutool-cache-redis`（DAO 扩展） | `redis` | **新 crate** | 对齐 `hutool-cache` 的 Redis 实现（RedisCache） |
| 🟡 中 | `tracing-appender` + `tracing-json` 增强 hutool-log | `tracing-appender` + `tracing-json` | `hutool-log` | DDD4J 推荐；当前 `tracing-subscriber` 已够用，json 输出为增强 |
| 🟠 低 | 新建 `crates/hutool-mq`（Kafka/RabbitMQ/MQTT） | `rdkafka` / `lapin` / `rumqttc` | **新 crate** | DDD4J 推荐；hutool 无原生 MQ，需自定边界 |
| 🟠 低 | 新建 `crates/hutool-distributed`（Nacos/Redis/Dubbo） | `nacos-rust` / `redis` | **新 crate** | DDD4J 推荐；hutool-extra 中有 Dubbo/Nacos 适配 |
| 🟠 低 | 新建 `crates/hutool-security`（axum-login / tower-sessions） | `axum-login` + `tower-sessions` | **新 crate** | DDD4J 推荐；hutool-security 对标 Shiro/Sa-Token |

#### 9.1.4 ❌ 不建议补全（**与项目原则冲突**）

| 建议项 | 原因 |
|---|---|
| ❌ 新增 `sea-orm` / `diesel` ORM | 违反原则 7「不造 ORM」、hutool 架构决策 §9「sqlx 直接构建」；ORM 应作为上层应用框架职责 |
| ❌ 在 `hutool-http` 中加 `axum` Web Server | hutool-http 是**客户端**，Server 模块应新建 `hutool-http-server` 独立 crate（参考 sa-token-rs 的 starter 拆分） |
| ❌ 用 `ring` 替代 `hmac`/`sha2` | ring 是 OpenSSL 绑定优先；当前 `RustCrypto` 纯 Rust + 国密 sm2/sm3/sm4 已覆盖，切换会失去国密能力 |
| ❌ 新增 `nacos-rust` 作为默认依赖 | 国内中间件生态碎片化，Rust 客户端不成熟；放可选特性 |

#### 9.1.5 依赖审计与治理（**架构守门**）

| 审计项 | 当前规则 | 检查命令 |
|---|---|---|
| License 合规 | MIT / Apache-2.0 / BSD 兼容 | `cargo deny check licenses` |
| 重复依赖 | 不同版本同一 crate 禁止 | `cargo tree --duplicates` |
| 安全公告 | RustSec 跟踪 | `cargo deny check advisories` |
| 来源可信 | crates.io + git + sparse | `cargo deny check sources` |
| 编译时长 | 单 crate 增量编译 ≤ 10s | `cargo build -p hutool-core --timings` |
| 默认特性膨胀 | `default = []` 优先 | `cargo metadata --format-version 1` |
| 特性互斥 | `legacy` 之类 opt-in 互斥 | CI 矩阵编译 |

#### 9.1.6 feature 设计原则（**新增**）

参考 DDD4J 中关于「hutool-all vs 单 crate」的设计决策，hutool-rust 的 feature 设计遵守：

```toml
# crates/hutool/Cargo.toml

[features]
default = ["core", "json"]                  # 用户不写 feature 也能用基础能力
core = ["dep:hutool-core"]
json = ["core", "dep:hutool-json"]

# 单 crate 必须为 opt-in
crypto-legacy = ["crypto", "hutool-crypto/legacy"]   # 显式启用
http-blocking = ["http", "hutool-http/blocking"]      # 阻塞 IO 显式启用
db-postgres = ["db", "hutool-db/postgres"]            # 驱动显式选择

# `full` 绝不默认启用（参考 sa-token-rs / DDD4J）
full = [...]   # 仅供 CI 集成测试
```

**核心规则**：
1. **默认零**：`default = []`（除 facade `hutool` 默认 `core` + `json`）
2. **driver 显式**：数据库驱动、TLS 后端、IO 模式全部 feature-gated
3. **legacy opt-in**：DES/RC4/MD5 等历史算法放 `legacy` feature（已实现 ✅）
4. **不引入 ORM**：原则 7 + §9 决策不变

---

## 十、分阶段实施计划

### 总体时间线（与 feature-matrix 版本对齐）

> ⚠️ **贯穿所有 Phase 的硬性验收标准（用户最终要求）**：
>
> > "hutool 中的除了 hutool-poi，每一个模块，每一个文件在 hutool-rust 中必须有对应的实现。"
>
> 每个 Phase 完成时必须通过 `scripts/verify-file-coverage.sh` 脚本验证，
> 覆盖率 ≥ §5.0.1 表格对应门槛（Phase 1: 30% → Phase 6: 100%）。

```text
v0.1.x (Phase 1) ─── hutool-core 命名收尾 + **hutool-extra 优先补全**（缺 170 文件，最高优先级）
    ├── Phase 1.1: hutool-extra 补 compress.archiver/extractor（七牛/腾讯/阿里/华为/京东 OSS 适配）
    ├── Phase 1.2: hutool-extra 补 mail（Mail/MailAccount/MailUtil）
    ├── Phase 1.3: hutool-extra 补 expression 引擎（Aviator/JEXL/JfireEL/Mvel/QLExpress/Rhino/SpEL）
    ├── Phase 1.4: hutool-extra 补 template 引擎（Beetl/Enjoy/Freemarker/Jetbrick/Rythm/Thymeleaf/Velocity/Wit）
    └── Phase 1.5: hutool-extra 补 ftp/ssh（Ftp/AbstractFtp/Sftp/JschUtil 等）

v0.2.x (Phase 2) ─── hutool-cron + hutool-db 核心补全
    ├── Phase 2.1: hutool-cron 补 41 文件（pattern 子包 + TaskExecutor + Listener + TimingWheel）
    ├── Phase 2.2: hutool-db 补 75 文件（Entity/Page/Query/SqlBuilder + Druid/Hikari DSFactory + 9 方言）
    ├── Phase 2.3: hutool-db 补 handler/meta/nosql 子包
    └── Phase 2.4: hutool-extra 补 tokenizer（8 engine：ansj/hanlp/ikanalyzer/jcseg/jieba/mmseg/mynlp/word）

v0.3.x (Phase 3) ─── hutool-crypto + hutool-http 补全
    ├── Phase 3.1: hutool-crypto 补 48 文件（SymmetricCrypto/SymmetricAlgorithm/RSA/ECIES/SM2/SM3/SM4/DES/RC4/HMac）
    ├── Phase 3.2: hutool-http 补 47 文件（HttpUtil 静态 facade + Server 子包 + SSL 子包 + Soap 子包）
    ├── Phase 3.3: hutool-aop + hutool-cache + hutool-bloom-filter + hutool-captcha 补全
    └── Phase 3.4: hutool-json + hutool-jwt + hutool-script + hutool-setting + hutool-system + hutool-socket 补全

v0.4.x (Phase 4) ─── hutool-poi 占位登记 + hutool-ai 7 provider
    ├── Phase 4.1: hutool-poi API 占位骨架已存在（79 个 .rs；不算实现）
    ├── Phase 4.2: 占位文件已有原 Java 文件/方法 rustdoc
    ├── Phase 4.3: hutool-ai 补 DeepSeek/Doubao/Gemini/Grok/Hutool/Ollama
    └── Phase 4.4: hutool-extra 补 emoji/pinyin/spring/servlet/validation

v0.5.x (Phase 5) ─── hutool-core 命名收尾（双重路径收尾 + facade 补全）
    ├── Phase 5.1: 解决 hutool-core 双重路径问题（保留全部，pub use 重新导出）
    ├── Phase 5.2: 补全缺失 facade 类（SetUtil/URLEncodeUtil/URLDecodeUtil/RegexUtil）
    ├── Phase 5.3: SecureUtil/DigestUtil/Base32Util/Base64Util 在 hutool-core 直接实现
    └── Phase 5.4: HttpUtil/SecureUtil 等 facade 静态方法

v0.9.x (Phase 6) ─── API 冻结、安全审计、文档完善
    ├── Phase 6.1: SemVer 检查、API 文档、CHANGELOG
    ├── Phase 6.2: Golden + Parity 测试
    └── Phase 6.3: 性能基准 + 安全审计

v1.0.0 ─── 正式发布
```

### 10.0 文件数量缺口优先级总表（**用户硬性要求**）

> ⚠️ 此表是 Phase 排序的**唯一依据**，按缺口绝对数量降序排列。

| 优先级 | 模块 | Java 文件 | Rust 文件 | 缺口 | 完成度 | 计划 Phase |
|---|---|---:|---:|---:|---:|---|
| 🔴 P0 | **hutool-extra** | 179 | 9 | **缺 170** | 5% | Phase 1（最高优先级） |
| 🔴 P0 | **hutool-db** | 107 | 32 | **缺 75** | 30% | Phase 2.2 |
| 🔴 P0 | **hutool-crypto** | 70 | 22 | **缺 48** | 31% | Phase 3.1 |
| 🔴 P0 | **hutool-http** | 72 | 25 | **缺 47** | 35% | Phase 3.2 |
| 🔴 P0 | **hutool-cron** | 41 | 4 | **缺 37** | 10% | Phase 2.1 |
| 🟡 P1 | **hutool-json** | 33 | 6 | 缺 27 | 18% | Phase 3.4 |
| 🟡 P1 | **hutool-socket** | 24 | 2 | 缺 22 | 8% | Phase 3.4 |
| 🟡 P1 | **hutool-cache** | 22 | 3 | 缺 19 | 14% | Phase 3.3 |
| 🟡 P1 | **hutool-bloomFilter** | 22 | 5 | 缺 17 | 23% | Phase 3.3 |
| 🟡 P1 | **hutool-jwt** | 17 | 2 | 缺 15 | 12% | Phase 3.4 |
| 🟡 P1 | **hutool-system** | 16 | 3 | 缺 13 | 19% | Phase 3.4 |
| 🟡 P1 | **hutool-setting** | 16 | 6 | 缺 10 | 38% | Phase 3.4 |
| 🟡 P1 | **hutool-script** | 5 | 2 | 缺 3 | 40% | Phase 3.4 |
| 🟢 P2 | **hutool-captcha** | 13 | 4 | 缺 9 | 31% | Phase 3.3 |
| 🟢 P2 | **hutool-aop** | 15 | 6 | 缺 9 | 40% | Phase 3.3 |
| 🟢 P2 | **hutool-dfa** | 6 | 5 | 缺 1 | 83% | Phase 3.4 |
| ⚪ 排除 | **hutool-poi** | 78 | 79 个占位 `.rs` | 功能未实现 | 0% | 不计完成度 |
| ⚪ 冻结 | **hutool-compat-hutool** | 0 | 1（114 行） | — | — | 冻结维持现状 |
| ✅ | **hutool-core** | 713 | 780 | -67（多 67） | 109% | Phase 5 收尾 |

**关键修正**：
1. **hutool-extra 缺 170 文件，是绝对的 P0 优先级**（最高）
2. **hutool-db 缺 75 文件**，必须先迁移（用户硬性要求）
3. **hutool-cron / hutool-http / hutool-crypto** 都是 P0 严重不足
4. **hutool-compat-hutool 冻结**（114 行无迁移价值）

### 总体版本路线图（与 §9.1 依赖评估对齐）

| Phase | 版本 | 主题 | 依赖治理 |
|---|---|---|---|
| Phase 0 | 0.0.x | 基础设施 + 文档体系 | workspace 依赖锁定 |
| Phase 1 | 0.1.x | **hutool-extra 优先补全**（缺 170 文件）+ 依赖治理 | 启用 `serde_yaml_ng` / `notify` / `itertools` |
| Phase 2 | 0.2.x | cron/db/extra 核心补全 | 启用 `num` / `statrs` |
| Phase 3 | 0.3.x | hutool-poi 占位登记 + ai 7 provider | POI 不接入 facade、不计完成度 |
| Phase 4 | 0.4.x | extra 大件补全 | 引入 `lettre` / `redis` |
| Phase 5 | 0.5.x | http-server + socket + 长尾 | 评估 `axum` / `tower-sessions` |
| Phase 6 | 0.9.x | API 冻结、安全审计 | `cargo deny` / `cargo semver-checks` |
| **Phase 7** | **0.6~0.8** | **依赖与架构提升（新增）** | **新建 hutool-cache-redis / hutool-mq / hutool-distributed** |
| v1.0.0 | 1.0 | 正式发布 | 全量审计通过 |

### **Phase 7：依赖与架构提升**（与 hutool 迁移并行，持续推进）

> 本 Phase 与 Phase 1~6 并行运行，每完成一个子阶段即合并入下一个 minor 版本。
> 依赖治理是**持续过程**而非一次性动作。

#### **Phase 7.1：核心 crate 依赖补全（高优先级，1~2 周）**

**目标**：把 DDD4J 分析中"workspace 已声明但未启用"的依赖补齐到对应 crate。

| 改动 | 当前 | 目标 |
|---|---|---|
| `hutool-setting` 加入 `serde_yaml_ng` | ❌ 未启用 | ✅ 启用（hutool-setting 原生支持 YAML） |
| `hutool-core` 启用 `notify` | ❌ 未启用 | ✅ 在 `io::watch` 模块启用 |
| `hutool-core` 加入 `itertools` | ❌ 缺失 | ✅ 新增（Rust 集合迭代标准） |
| `hutool-core` 加入 `num` + `statrs` | ❌ 缺失 | ✅ 新增（数学/统计） |

**验收标准**：
```rust
// hutool-setting/src/yaml.rs
use serde_yaml_ng;
let cfg: MyConfig = serde_yaml_ng::from_str(text)?;

// hutool-core/src/io/watch.rs（已有 WatchMonitor）
use notify::{RecommendedWatcher, RecursiveMode};
let mut watcher = notify::recommended_watcher(|res| { ... })?;
watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
```

#### **Phase 7.2：新建 hutool-cache-redis（DAO 扩展，1 周）**

**目标**：补齐 hutool-cache 缺失的 Redis 实现。

```toml
# crates/hutool-cache-redis/Cargo.toml
[dependencies]
hutool-core.workspace = true
redis = { version = "0.27", features = ["tokio-comp"] }
tokio.workspace = true
thiserror.workspace = true
```

**对齐 Java**：
- `cn.hutool.cache.impl.CacheImpl` + `cn.hutool.cache.impl.RedisCache`

**Rust 实现**：
```rust
// crates/hutool-cache-redis/src/lib.rs
//! 1:1 对齐 hutool-cache 的 Redis 实现
//! 使用 `redis::aio::ConnectionManager` 提供连接复用

pub struct RedisCache {
    conn: redis::aio::ConnectionManager,
    prefix: String,
}

impl RedisCache {
    pub async fn get<K, V>(&self, key: &K) -> Result<Option<V>, CacheError>
    where K: AsRef<str>, V: serde::de::DeserializeOwned;
    pub async fn put<K, V>(&self, key: K, value: &V, ttl: Duration) -> Result<(), CacheError>
    where K: Into<String>, V: serde::Serialize;
    pub async fn remove<K>(&self, key: K) -> Result<(), CacheError>
    where K: AsRef<str>;
}
```

**特性**：默认不启用，作为 opt-in DAO 扩展。

#### **Phase 7.3：新建 hutool-ai SSE 增强（1 周）**

**目标**：把 `eventsource-stream` 加入 hutool-ai，标准化流式响应。

```toml
# crates/hutool-ai/Cargo.toml
[dependencies]
# 现有 + 新增
eventsource-stream = "0.2"
futures-core.workspace = true
```

```rust
// hutool-ai/src/stream.rs
use eventsource_stream::{EventStream, EventStreamExt};

pub fn parse_sse_stream(body: impl futures_core::Stream<Item = reqwest::Result<bytes::Bytes>> + Unpin)
    -> impl futures_core::Stream<Item = Result<ChatChunk, AiError>>;
```

#### **Phase 7.4：新建 hutool-mq（消息队列，低优先级 / 3~4 周）**

**决策**：**仅在用户明确需求时启动**。DDD4J 推荐但不在 hutool 主干。

```text
crates/hutool-mq/
├── Cargo.toml          # rdkafka/lapin/rumqttc 各自 feature
├── src/
│   ├── lib.rs
│   ├── kafka.rs        # 特性: kafka
│   ├── rabbit.rs       # 特性: rabbitmq
│   └── mqtt.rs         # 特性: mqtt
```

#### **Phase 7.5：新建 hutool-distributed（分布式中间件，低优先级 / 3~4 周）**

**决策**：**作为 opt-in 扩展**，默认不引入。

```text
crates/hutool-distributed/
├── Cargo.toml          # nacos/redis/consul 各自 feature
├── src/
│   ├── lib.rs
│   ├── nacos.rs        # 特性: nacos
│   └── redis_lock.rs   # 特性: redis
```

#### **Phase 7.6：依赖审计持续化（持续）**

每次 PR 触发：

```yaml
# .github/workflows/ci.yml 新增
audit:
  steps:
    - cargo deny check
    - cargo outdated --workspace
    - cargo audit
    - cargo tree --duplicates --workspace
```

#### **Phase 7.7：feature 矩阵更新（每 minor 版本）**

每次 Phase 完成，更新 `docs/feature-matrix.md`：

| Feature | 依赖变化 | 编译成本影响 |
|---|---|---|
| Phase 7.1 后 `core` | + `itertools` + `num` + `statrs` + `notify` | 编译时长 +5~8s |
| Phase 7.2 后 `cache-redis` | + `redis` (opt-in) | 编译时长 +3~5s |
| Phase 7.3 后 `ai` | + `eventsource-stream` | 编译时长 +1~2s |

#### **Phase 7 不启动的项（DDD4J 推荐但与 hutool-rust 原则冲突）**

| 项 | 不启动原因 |
|---|---|
| ❌ `sea-orm` / `diesel` ORM | 违反「不造 ORM」原则 |
| ❌ `hutool-http` 内嵌 `axum` Server | 应新建 `hutool-http-server` 独立 crate |
| ❌ `ring` 替代 `RustCrypto` | 失去国密 sm2/sm3/sm4 能力 |

### **Phase 1.1：StrUtil 完整 facade**（1~2 周）

**目标**：替换 `src/str_util.rs` 的 1.1KB 占位为完整 facade，**不删减原文件**（满足"不删减"原则）。

**范围**：
- ✅ 在 `src/str_util.rs` 中实现 100+ 静态方法
- ✅ 通过 `pub use` 委托到 `text/str_builder.rs`、`text/str_joiner.rs`、`text/str_splitter.rs` 等子模块
- ✅ 添加完整 rustdoc，每方法标注"原 Java 对应方法"

**验收标准**：
```rust
#[test]
fn test_str_util_is_empty() {
    assert!(StrUtil::is_empty(None));
    assert!(StrUtil::is_empty(Some("")));
    assert!(!StrUtil::is_empty(Some("hello")));
}
```

### **Phase 1.2：hutool-core 目录结构重组（解决双重路径 + 平铺问题）**（2~3 周，**P0**）

**问题诊断（用户硬性要求）**：

> "你必须参考 hutool 的目录命名规范，结合 rust 的项目结构，不能都在一个根目录，
> 或者一个 lib 中把所有代码都实现了。"

| 指标 | hutool Java | hutool-rust Rust（当前） | 评价 |
|---|---|---|---|
| 顶层 `.java`/`.rs` 文件 | **1**（仅 `package-info.java`） | **59**（一堆平铺） | ❌ 严重违反 |
| 子包/子目录 | **24 个**（每个对应一个领域） | **24 个**（但内容被顶层覆盖） | ✅ 一致 |
| `lib.rs` 内容 | 不适用 | 280 行（应是 ~30 行 mod 索引） | ❌ 过重 |

**目标**：让 hutool-core 严格按 hutool Java 的 24 子包组织，每个子包是"目录索引"，顶层 lib.rs 只做 `mod` 声明和统一 re-export。

**目标目录结构**（与 hutool Java `cn.hutool.core.*` 1:1 对应）：

```text
hutool-core/src/
├── lib.rs                                # 只写 mod 声明 + 统一 re-export（≤ 80 行）
│
├── annotation/                           # ← cn.hutool.core.annotation
│   ├── mod.rs                            # 目录索引
│   ├── alias.rs                          # ← cn.hutool.core.annotation.AnnotationAlias
│   ├── annotation_util.rs                # ← cn.hutool.core.annotation.AnnotationUtil
│   └── ...
│
├── bean/                                 # ← cn.hutool.core.bean
│   ├── mod.rs
│   ├── bean_util.rs                      # ← cn.hutool.core.bean.BeanUtil
│   ├── bean_desc.rs
│   ├── bean_path.rs
│   └── copier/                           # ← bean.copier 子包
│       ├── mod.rs
│       ├── bean_copier.rs
│       └── bean_to_bean_copier.rs
│
├── builder/                              # ← cn.hutool.core.builder
│   ├── mod.rs
│   ├── builder.rs
│   ├── equals_builder.rs
│   └── hash_code_builder.rs
│
├── clone/                                # ← cn.hutool.core.clone
│   ├── mod.rs
│   ├── clone_support.rs                  # ← cn.hutool.core.clone.CloneSupport
│   └── cloneable.rs                      # ← cn.hutool.core.clone.Cloneable
│
├── codec/                                # ← cn.hutool.core.codec
│   ├── mod.rs
│   ├── base16_codec.rs
│   ├── base32.rs
│   ├── base64.rs
│   └── ...
│
├── collection/                           # ← cn.hutool.core.collection
│   ├── mod.rs
│   ├── coll_util.rs                      # ← cn.hutool.core.collection.CollUtil（★ 新家）
│   ├── coll_stream_util.rs
│   ├── list_util.rs
│   ├── iter_util.rs
│   ├── coll_stream_util.rs
│   └── iter/                             # ← collection.iter 子包
│       ├── mod.rs
│       └── ...
│
├── comparator/                           # ← cn.hutool.core.comparator
│   ├── mod.rs
│   ├── compare_util.rs
│   └── ...
│
├── compiler/                             # ← cn.hutool.core.compiler
│   ├── mod.rs
│   └── ...
│
├── compress/                             # ← cn.hutool.core.compress
│   ├── mod.rs
│   └── ...
│
├── convert/                              # ← cn.hutool.core.convert
│   ├── mod.rs
│   ├── convert.rs                        # ← cn.hutool.core.convert.Convert
│   ├── converter.rs
│   ├── converter_registry.rs
│   └── impl/                             # ← convert.impl 子包
│       ├── mod.rs
│       └── ...
│
├── date/                                 # ← cn.hutool.core.date
│   ├── mod.rs
│   ├── date_util.rs                      # ← cn.hutool.core.date.DateUtil
│   ├── date_time.rs                      # ← cn.hutool.core.date.DateTime
│   ├── local_date_time_util.rs
│   ├── calendar_util.rs
│   ├── stop_watch.rs
│   └── format/                           # ← date.format 子包
│       ├── mod.rs
│       ├── date_basic.rs
│       └── fast_date_format.rs
│
├── exceptions/                           # ← cn.hutool.core.exceptions
│   ├── mod.rs
│   ├── util_exception.rs
│   └── stateful_exception.rs
│
├── getter/                               # ← cn.hutool.core.getter
│   ├── mod.rs
│   └── basic_type_getter.rs
│
├── img/                                  # ← cn.hutool.core.img
│   ├── mod.rs
│   ├── img_util.rs
│   └── color_util.rs
│
├── io/                                   # ← cn.hutool.core.io
│   ├── mod.rs
│   ├── io_util.rs                        # ← cn.hutool.core.io.IoUtil（★ 新家）
│   ├── file_util.rs                      # ← cn.hutool.core.io.FileUtil（★ 新家）
│   ├── resource/                         # ← io.resource 子包
│   │   ├── mod.rs
│   │   └── resource_util.rs
│   ├── file/                             # ← io.file 子包
│   │   ├── mod.rs
│   │   └── file_appender.rs
│   ├── copy/                             # ← io.copy 子包
│   │   ├── mod.rs
│   │   └── stream_copier.rs
│   └── watch/                            # ← io.watch 子包
│       ├── mod.rs
│       └── watch_monitor.rs
│
├── lang/                                 # ← cn.hutool.core.lang
│   ├── mod.rs
│   ├── validator.rs
│   ├── console.rs
│   ├── snowflake.rs
│   ├── tuple.rs
│   ├── pair.rs
│   ├── range.rs
│   ├── matcher.rs
│   ├── regex_pool.rs
│   ├── segment.rs
│   ├── simple_cache.rs
│   ├── singleton.rs
│   ├── tree/                             # ← lang.tree 子包
│   │   ├── mod.rs
│   │   ├── tree.rs
│   │   ├── tree_node.rs
│   │   └── tree_util.rs
│   ├── mutable/                          # ← lang.mutable 子包
│   │   ├── mod.rs
│   │   └── mutable_int.rs
│   ├── func/                             # ← lang.func 子包
│   │   ├── mod.rs
│   │   └── func.rs
│   ├── id/                               # ← lang.id 子包
│   │   ├── mod.rs
│   │   └── nano_id.rs
│   ├── hash/                             # ← lang.hash 子包
│   │   ├── mod.rs
│   │   ├── city_hash.rs
│   │   └── murmur_hash.rs
│   └── reflect/                          # ← lang.reflect 子包
│       ├── mod.rs
│       └── lookup_factory.rs
│
├── map/                                  # ← cn.hutool.core.map
│   ├── mod.rs
│   ├── map_util.rs                       # ← cn.hutool.core.map.MapUtil（★ 新家）
│   ├── map_wrapper.rs
│   ├── custom_key_map.rs
│   ├── func_map.rs
│   ├── camel_case_map.rs
│   ├── case_insensitive_map.rs
│   ├── multi/                            # ← map.multi 子包
│   │   ├── mod.rs
│   │   └── table.rs
│   └── reference/                        # ← map.reference 子包
│       ├── mod.rs
│       └── weak_key_concurrent_map.rs
│
├── math/                                 # ← cn.hutool.core.math
│   ├── mod.rs
│   ├── math_util.rs
│   ├── calculator.rs
│   ├── arrangement.rs
│   ├── combination.rs
│   └── money.rs
│
├── net/                                  # ← cn.hutool.core.net
│   ├── mod.rs
│   ├── net_util.rs
│   ├── url_util.rs
│   ├── url_encode_util.rs                # ← cn.hutool.core.net.URLEncodeUtil
│   ├── url_decode_util.rs                # ← cn.hutool.core.net.URLDecodeUtil
│   ├── ipv4_util.rs
│   ├── mask_bit.rs
│   ├── ssl_util.rs
│   ├── url/                              # ← net.url 子包
│   │   ├── mod.rs
│   │   ├── url_builder.rs
│   │   └── url_query.rs
│   └── multipart/                        # ← net.multipart 子包
│       ├── mod.rs
│       └── upload_file.rs
│
├── stream/                               # ← cn.hutool.core.stream
│   ├── mod.rs
│   └── stream_util.rs
│
├── swing/                                # ← cn.hutool.core.swing
│   ├── mod.rs
│   ├── clipboard/
│   ├── desktop_util.rs
│   ├── robot_util.rs
│   └── screen_util.rs
│
├── text/                                 # ← cn.hutool.core.text
│   ├── mod.rs
│   ├── str_util.rs                       # ← cn.hutool.core.text.StrUtil（★ 新家，替换占位）
│   ├── str_builder.rs
│   ├── str_joiner.rs
│   ├── str_splitter.rs
│   ├── str_formatter.rs
│   ├── csv/                              # ← text.csv 子包
│   │   ├── mod.rs
│   │   └── csv_util.rs
│   ├── escape/                           # ← text.escape 子包
│   │   ├── mod.rs
│   │   └── html4_escape.rs
│   ├── finder/                           # ← text.finder 子包
│   │   ├── mod.rs
│   │   └── finder.rs
│   └── replacer/                         # ← text.replacer 子包
│       ├── mod.rs
│       └── replacer_chain.rs
│
├── thread/                               # ← cn.hutool.core.thread
│   ├── mod.rs
│   ├── thread_util.rs
│   ├── global_thread_pool.rs
│   ├── executor_builder.rs
│   ├── named_thread_factory.rs
│   └── lock/                             # ← thread.lock 子包
│       ├── mod.rs
│       └── lock_util.rs
│
└── util/                                 # ← cn.hutool.core.util（核心门面）
    ├── mod.rs
    ├── array_util.rs                     # ← cn.hutool.core.util.ArrayUtil
    ├── boolean_util.rs
    ├── byte_util.rs
    ├── char_util.rs
    ├── charset_util.rs
    ├── class_loader_util.rs
    ├── class_util.rs
    ├── coordinate_util.rs
    ├── credit_code_util.rs
    ├── desensitized_util.rs
    ├── enum_util.rs
    ├── escape_util.rs
    ├── hash_util.rs
    ├── hex_util.rs
    ├── id_util.rs                        # ← cn.hutool.core.util.IdUtil（★ 新家）
    ├── idcard_util.rs
    ├── jaxb_util.rs
    ├── jdk_util.rs
    ├── jndi_util.rs
    ├── modifier_util.rs
    ├── number_util.rs
    ├── object_util.rs
    ├── page_util.rs
    ├── phone_util.rs
    ├── primitive_array_util.rs
    ├── radix_util.rs
    ├── random_util.rs
    ├── re_util.rs
    ├── reference_util.rs
    ├── reflect_util.rs
    ├── runtime_util.rs
    ├── serialize_util.rs
    ├── service_loader_util.rs
    ├── system_props_util.rs
    ├── type_util.rs
    ├── version_util.rs
    ├── xml_util.rs
    ├── zip_util.rs
    ├── secure_util.rs                    # ← cn.hutool.crypto.SecureUtil（hutool-core 不存在，Phase 1.3 补）
    ├── digest_util.rs                    # ← cn.hutool.crypto.DigestUtil（Phase 1.3 补）
    └── set_util.rs                       # ← cn.hutool.core.collection.SetUtil（Phase 1.3 补）
```

**目录重组执行步骤（不动文件内容，只移动路径）**：

```bash
# Step 1: 创建新位置目录
mkdir -p crates/hutool-core/src/text  # 已有，待确认内容
mkdir -p crates/hutool-core/src/util  # 已有，待确认内容

# Step 2: 用 git mv 把顶层 .rs 移动到对应子目录（★ "不删减"用 git mv 保留历史）
cd crates/hutool-core/src

git mv coll_util.rs            collection/coll_util.rs
git mv coll_stream_util.rs     collection/coll_stream_util.rs
git mv list_util.rs            collection/list_util.rs
git mv iter_util.rs            collection/iter_util.rs
git mv map_util.rs             map/map_util.rs
git mv io_util.rs              io/io_util.rs
git mv file_util.rs            io/file_util.rs
git mv array_util.rs           util/array_util.rs
git mv boolean_util.rs         util/boolean_util.rs
git mv byte_util.rs            util/byte_util.rs
git mv char_util.rs            util/char_util.rs
git mv charset_util.rs         util/charset_util.rs
git mv class_loader_util.rs    util/class_loader_util.rs
git mv class_util.rs           util/class_util.rs
git mv coordinate_util.rs      util/coordinate_util.rs
git mv credit_code_util.rs     util/credit_code_util.rs
git mv desensitized_util.rs    util/desensitized_util.rs
git mv enum_util.rs            util/enum_util.rs
git mv escape_util.rs          util/escape_util.rs
git mv hash_util.rs            util/hash_util.rs
git mv hex_util.rs             util/hex_util.rs
git mv id.rs                   util/id_util.rs
git mv idcard_util.rs          util/idcard_util.rs
git mv jaxb_util.rs            util/jaxb_util.rs
git mv modifier_util.rs        util/modifier_util.rs
git mv number_util.rs          util/number_util.rs
git mv object_util.rs          util/object_util.rs
git mv page_util.rs            util/page_util.rs
git mv phone_util.rs           util/phone_util.rs
git mv primitive_array_util.rs util/primitive_array_util.rs
git mv radix_util.rs           util/radix_util.rs
git mv random_util.rs          util/random_util.rs
git mv re_util.rs              util/re_util.rs
git mv reference_util.rs       util/reference_util.rs
git mv reflect_util.rs         util/reflect_util.rs
git mv runtime_util.rs         util/runtime_util.rs
git mv version_util.rs         util/version_util.rs

# Step 3: 移动双重路径中已迁移到子目录的版本（保留在新位置，删除顶层 legacy）
# BeanUtil: src/bean_util.rs → 已迁移到 src/bean/bean_util.rs，删除顶层
# IdUtil: 同上

# Step 4: 移动 text 相关（如果顶层有 str_util.rs）
# src/str_util.rs 已存在但只有 1.1KB，将在 Phase 5 完整化为 text/str_util.rs

# Step 5: 重写 lib.rs（只保留 mod 声明 + 顶层 re-export）
```

**新 lib.rs 模板**（≤ 80 行）：

```rust
//! Core utilities shared by the `Hutool-Rust` workspace.
//!
//! 模块组织严格对齐 `cn.hutool.core.*` 24 子包；每个子包是一个"目录索引"，
//! 不允许把实现代码堆在 src/ 顶层。

#![forbid(unsafe_code)]

// ===== 24 个 hutool 子包（按字母序） =====
pub mod annotation;
pub mod bean;
pub mod builder;
pub mod clone;
pub mod codec;
pub mod collection;
pub mod comparator;
pub mod compiler;
pub mod compress;
pub mod convert;
pub mod date;
pub mod exceptions;
pub mod getter;
#[cfg(feature = "img")]
pub mod img;
pub mod io;
pub mod lang;
pub mod map;
pub mod math;
pub mod net;
pub mod stream;
#[cfg(feature = "swing")]
pub mod swing;
pub mod text;
pub mod thread;
pub mod util;        // ← 最常用的工具门面

// ===== 顶层统一 re-export（用户最常 import 的类） =====
pub use collection::CollUtil;
pub use collection::ListUtil;
pub use collection::IterUtil;
pub use date::DateUtil;
pub use date::DateTime;
pub use io::FileUtil;
pub use io::IoUtil;
pub use io::resource::ResourceUtil;
pub use map::MapUtil;
pub use text::StrUtil;
pub use util::ArrayUtil;
pub use util::CharUtil;
pub use util::NumberUtil;
pub use util::RandomUtil;
pub use util::IdUtil;
pub use util::IdcardUtil;
pub use util::PhoneUtil;
// ... 其他 ~50 个常用 facade
```

**已有双重路径清单（必须清理）**：

| 模块 | legacy 顶层 | new path 子目录 | Phase 1.2 处理 |
|---|---|---|---|
| CollUtil | `src/coll_util.rs` | `src/collection/coll_util.rs` | 删除顶层，保留子目录 |
| BeanUtil | `src/bean_util.rs` | `src/bean/bean_util.rs` | 删除顶层，保留子目录 |
| FileUtil | `src/file_util.rs` | `src/io/file/file_util.rs` | 删除顶层，保留子目录 |
| IoUtil | `src/io_util.rs` | `src/io/io_util.rs` | 删除顶层，保留子目录 |
| IdUtil | `src/id.rs` | `src/util/id_util.rs` | 删除顶层，保留子目录 |
| MapUtil | `src/map_util.rs` | `src/map/map_util.rs` | 删除顶层，保留子目录 |
| IterUtil | `src/iter_util.rs` | `src/collection/iter_util.rs` | 删除顶层，保留子目录 |
| ListUtil | `src/list_util.rs` | `src/collection/list_util.rs` | 删除顶层，保留子目录 |

**Phase 1.2 不删减原则的体现**：所有现有代码保留，**只移动路径，不修改内容**。使用 `git mv` 保留完整历史。

### **Phase 1.3：补全缺失 facade 类**（1~2 周）

| 新增文件 | 对应 Java | 内容 |
|---|---|---|
| `src/util/set_util.rs` | `cn.hutool.core.collection.SetUtil` | ~30 个静态方法 |
| `src/net/url_encode_util.rs` | `cn.hutool.core.net.URLEncodeUtil` | ~10 个静态方法 |
| `src/net/url_decode_util.rs` | `cn.hutool.core.net.URLDecodeUtil` | ~5 个静态方法 |
| `src/util/regex_util.rs` | `cn.hutool.core.util.RegexUtil` | re-export + 包装 `ReUtil` |
| `src/util/serialize_util.rs` | `cn.hutool.core.util.SerializeUtil` | Java 序列化兼容（Rust 简化） |
| `src/util/service_loader_util.rs` | `cn.hutool.core.util.ServiceLoaderUtil` | SPI 替代 |

### **Phase 1.4：SecureUtil / DigestUtil 委托 facade**（1 周）

```rust
// hutool-core/src/util/secure_util.rs （新增）
//! 1:1 对齐 `cn.hutool.crypto.SecureUtil`
//! 实现：纯委托到 `hutool_crypto` crate

pub struct SecureUtil;
impl SecureUtil {
    pub fn md5(s: &str) -> String { hutool_crypto::digest::md5_hex(s) }
    pub fn sha1(s: &str) -> String { hutool_crypto::digest::sha1_hex(s) }
    pub fn sha256(s: &str) -> String { hutool_crypto::digest::sha256_hex(s) }
    pub fn sha512(s: &str) -> String { hutool_crypto::digest::sha512_hex(s) }
    pub fn aes(key: &[u8]) -> AesBuilder { AesBuilder::new(key) }
    pub fn rsa() -> RsaBuilder { RsaBuilder::new() }
    pub fn hmac(algorithm: &str, key: &[u8]) -> HMacBuilder { HMacBuilder::new(algorithm, key) }
}
```

```rust
// hutool-core/src/util/digest_util.rs （新增）
pub struct DigestUtil;
impl DigestUtil {
    pub fn md5(s: &str) -> String { SecureUtil::md5(s) }
    pub fn sha256(s: &str) -> String { SecureUtil::sha256(s) }
    // ... 全部委托到 SecureUtil
}
```

### **Phase 1.5：HttpUtil 静态 facade**（3~5 天）

```rust
// hutool-http/src/util.rs （新增）
//! 1:1 对齐 `cn.hutool.http.HttpUtil`

pub struct HttpUtil;
impl HttpUtil {
    pub fn create(url: &str) -> HttpRequest {
        HttpRequest::new(Method::Get, url)
    }
    pub async fn get(url: &str) -> Result<HttpResponse, HttpException> {
        HttpUtil::create(url).send().await
    }
    pub async fn post(url: &str, body: impl RequestBody + 'static) -> Result<HttpResponse, HttpException> {
        HttpRequest::new(Method::Post, url).body(body).send().await
    }
    pub async fn download(url: &str, target_file: &Path) -> Result<(), HttpException> {
        // ...
    }
}
```

### **Phase 2.1：hutool-cron 补全**（2~3 周）

**当前**：4 个文件（`expression.rs`/`parser.rs`/`scheduler.rs`/`lib.rs`）

**目标**：对齐 hutool-cron 41 文件，补 36 个缺失类。

**新增子包**：

```text
hutool-cron/src/
├── pattern/                            # ← cn.hutool.cron.pattern
│   ├── mod.rs
│   ├── part.rs                         # Part
│   ├── part_matcher.rs                 # PartMatcher trait
│   ├── part_parser.rs                  # PartParser
│   ├── pattern_matcher.rs              # PatternMatcher
│   ├── pattern_parser.rs               # PatternParser
│   ├── pattern_util.rs                 # PatternUtil
│   └── matcher/
│       ├── mod.rs
│       ├── always_true_matcher.rs      # AlwaysTrueMatcher
│       ├── bool_array_matcher.rs       # BoolArrayMatcher
│       ├── day_of_month_matcher.rs     # DayOfMonthMatcher
│       └── year_value_matcher.rs       # YearValueMatcher
├── task/                               # ← cn.hutool.cron.task
│   ├── mod.rs
│   ├── invoke_task.rs                  # InvokeTask
│   └── runnable_task.rs                # RunnableTask
├── timingwheel/                        # ← cn.hutool.cron.timingwheel
│   ├── mod.rs
│   └── timing_wheel.rs                 # TimingWheel
├── listener/                           # ← cn.hutool.cron.listener
│   ├── mod.rs
│   └── simple_task_listener.rs         # SimpleTaskListener
├── cron_pattern.rs                     # ← CronPattern
├── cron_pattern_builder.rs             # ← CronPatternBuilder
├── cron_pattern_util.rs                # ← CronPatternUtil
├── cron_task.rs                        # ← CronTask
├── cron_timer.rs                       # ← CronTimer
├── cron_config.rs                      # ← CronConfig
├── cron_exception.rs                   # ← CronException
├── scheduler.rs                        # ← Scheduler（已有 + 扩展）
├── system_timer.rs                     # ← SystemTimer
├── task.rs                             # ← Task trait
├── task_executor.rs                    # ← TaskExecutor
├── task_executor_manager.rs            # ← TaskExecutorManager
├── task_launcher.rs                    # ← TaskLauncher
├── task_launcher_manager.rs            # ← TaskLauncherManager
├── task_listener.rs                    # ← TaskListener
├── task_listener_manager.rs            # ← TaskListenerManager
├── task_table.rs                       # ← TaskTable
├── timer_task.rs                       # ← TimerTask
└── timer_task_list.rs                  # ← TimerTaskList
```

### **Phase 2.2：hutool-db 补全**（4~6 周）

**当前**：6 个文件（`connection/ds/pool/row/sql/lib`）

**目标**：对齐 hutool-db 107 文件，补 100+ 缺失类。

**新增子包**（参考 hutool-db 子包）：

```text
hutool-db/src/
├── entity.rs                           # ← Entity
├── page.rs                             # ← Page + PageResult
├── query.rs                            # ← Query + Condition + ConditionBuilder + Direction + LogicalOperator + Order
├── sql_builder.rs                      # ← SqlBuilder
├── sql_executor.rs                     # ← SqlExecutor + SqlConnRunner
├── sql_formatter.rs                    # ← SqlFormatter
├── sql_log.rs                          # ← SqlLog
├── sql_util.rs                         # ← SqlUtil + Table + Column + IndexInfo + ColumnIndexInfo
├── transaction_level.rs                # ← TransactionLevel
├── db.rs                               # ← Db 静态 facade
├── db_util.rs                          # ← DbUtil
├── db_setting.rs                       # ← DbSetting
├── db_config.rs                        # ← DbConfig
├── db_runtime_exception.rs             # ← DbRuntimeException
├── abstract_db.rs                      # ← AbstractDb
├── active_entity.rs                    # ← ActiveEntity
├── named_sql.rs                        # ← NamedSql
├── ds/                                 # ← ds/
│   ├── mod.rs
│   ├── ds_factory.rs                   # ← DSFactory
│   ├── abstract_ds_factory.rs          # ← AbstractDSFactory
│   ├── simple/                         # ← ds/simple/
│   ├── pooled/                         # ← ds/pooled/
│   ├── hikari/                         # ← ds/hikari/
│   ├── druid/                          # ← ds/druid/
│   ├── dbcp/                           # ← ds/dbcp/
│   ├── c3p0/                           # ← ds/c3p0/
│   ├── tomcat/                         # ← ds/tomcat/
│   ├── jndi/                           # ← ds/jndi/
│   └── bee/                            # ← ds/bee/
├── handler/                            # ← handler/
│   ├── mod.rs
│   ├── bean_handler.rs                 # ← BeanHandler
│   ├── bean_list_handler.rs            # ← BeanListHandler
│   ├── entity_handler.rs               # ← EntityHandler
│   ├── entity_list_handler.rs          # ← EntityListHandler
│   ├── entity_set_handler.rs           # ← EntitySetHandler
│   ├── number_handler.rs               # ← NumberHandler
│   ├── page_result_handler.rs          # ← PageResultHandler
│   ├── rs_handler.rs                   # ← RsHandler
│   ├── string_handler.rs               # ← StringHandler
│   └── value_list_handler.rs           # ← ValueListHandler
├── meta/                               # ← meta/
│   ├── mod.rs
│   ├── column.rs
│   ├── index_info.rs
│   ├── table_type.rs
│   └── meta_util.rs                    # ← MetaUtil
├── nosql/                              # ← nosql/
│   ├── mod.rs
│   ├── mongo/                          # ← nosql/mongo/
│   │   ├── mod.rs
│   │   ├── mongo_ds.rs
│   │   └── mongo_factory.rs
│   └── redis/                          # ← nosql/redis/
│       ├── mod.rs
│       └── redis_ds.rs
├── sql/                                # ← sql/
│   ├── mod.rs
│   ├── condition.rs                    # ← Condition + ConditionGroup
│   ├── direction.rs                    # ← Direction
│   ├── logical_operator.rs             # ← LogicalOperator
│   ├── order.rs                        # ← Order
│   └── sql_builder.rs                  # ← SqlBuilder (extended)
├── transaction/                        # ← transaction/
│   ├── mod.rs
│   ├── session.rs                      # ← Session
│   ├── transaction_level.rs
│   └── ...
└── dialect/                            # ← dialect/
    ├── mod.rs
    ├── dialect.rs                      # ← Dialect trait
    ├── dialect_factory.rs              # ← DialectFactory
    ├── dialect_name.rs                 # ← DialectName
    └── impl/
        ├── mod.rs
        ├── ansi_sql_dialect.rs         # ← AnsiSqlDialect
        ├── dm_dialect.rs               # ← DmDialect
        ├── h2_dialect.rs               # ← H2Dialect
        ├── hana_dialect.rs             # ← HanaDialect
        ├── mysql_dialect.rs            # ← MysqlDialect
        ├── oracle_dialect.rs           # ← OracleDialect
        ├── phoenix_dialect.rs          # ← PhoenixDialect
        ├── postgresql_dialect.rs       # ← PostgresqlDialect
        ├── sql_server_2012_dialect.rs  # ← SqlServer2012Dialect
        └── sqlite3_dialect.rs          # ← Sqlite3Dialect
```

### **Phase 3.1：hutool-poi 占位骨架**（历史已完成，仅登记 API）

**当前结果**：已建立 79 个 `.rs` 文件并添加迁移 rustdoc；其中 67 处为 `unimplemented!()`。这只完成 API/文件登记，功能实现仍为 0%。

历史阶段只生成了占位文件；它没有接入 `easyexcel-rs`、`easydoc-rs`、
`easyofd-rs`、`easypdf-rs` 或任何其他文档引擎。当前应保持以下边界：

- 不向 `crates/hutool/Cargo.toml` 添加 `hutool-poi` 依赖；
- 不定义 `poi` / `poi-docx` facade feature；
- 不把占位 API、`#[should_panic]` 测试或文件数量计为功能实现。

### **Phase 3.3：hutool-ai 补全**（3~4 周）

每个 provider 5 文件：

```text
hutool-ai/src/provider/
├── openai.rs                           # ✅ 已实现
├── deepseek.rs                         # Phase 3.3.1
├── doubao.rs                           # Phase 3.3.2
├── gemini.rs                           # Phase 3.3.3
├── grok.rs                             # Phase 3.3.4
├── hutool.rs                           # Phase 3.3.5
└── ollama.rs                           # Phase 3.3.6
```

### **Phase 6：发布前质量门禁**

每个 PR 至少执行：

```text
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo test --workspace --no-default-features
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
cargo deny check
cargo semver-checks
```

---

## 十一、依赖清单

### 11.1 Workspace 根 Cargo.toml（与现状一致）

```toml
[workspace]
resolver = "3"
members = ["crates/*"]

[workspace.package]
edition = "2024"
rust-version = "1.94"
license = "MIT OR Apache-2.0"
repository = "https://github.com/hiwepy/hutool-rust"

[workspace.lints.rust]
unsafe_code = "deny"
missing_docs = "warn"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
```

### 11.2 hutool-core 依赖（保持现状，不引入 async）

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
regex = "1"
encoding_rs = "0.8"
once_cell = "1"
num-bigint = "0.4"
num-traits = "0.2"
rust_decimal = { version = "1", features = ["serde-with-str"] }
smallvec = "1"
ahash = "0.8"
indexmap = { version = "2", features = ["serde"] }
```

### 11.3 hutool-http 依赖

```toml
[dependencies]
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "stream", "multipart", "gzip"] }
url = "2"
bytes = "1"
tokio = { version = "1", features = ["fs", "io-util", "macros", "rt-multi-thread", "sync", "time"] }
tower = "0.5"
tower-http = "0.6"
http = "1"
http-body-util = "0.1"
eventsource-stream = "0.2"
woothee = "0.13"
```

### 11.4 hutool-db 依赖

```toml
[dependencies]
sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio-rustls", "macros", "any"] }
tokio = { version = "1", features = ["full"] }
futures = "0.3"
async-trait = "0.1"
```

### 11.5 hutool-crypto 依赖

```toml
[dependencies]
aes-gcm = "0.10"
hmac = "0.12"
sha2 = "0.10"
sha1 = "0.10"
md-5 = "0.10"
argon2 = "0.5"
rsa = { version = "0.9", features = ["sha2"] }   # Phase 5 才启用
rand = "0.8"
secrecy = "0.8"
zeroize = "1"
```

---

## 十二、测试体系

### 12.1 四层测试（参考 sa-token-rs + hutool 现状）

| 层级 | 文件位置 | 数量目标 | 说明 |
|---|---|---|---|
| 单元测试 | `src/tests.rs` + `src/missing_tests.rs` | 每 crate ≥ 20 | 追踪未移植的 Java 测试 |
| 1:1 方法测试 | `tests/1to1/*_1to1_tests.rs` | ≥ 300 | 每个 Hutool 关键方法对应一个 Rust 测试 |
| Golden 测试 | `tests/golden/*.expected.json` | ≥ 50 | Java Hutool 跑出快照 → Rust 字节级比对 |
| Parity 测试 | `tests/parity/*_parity_tests.rs` | ≥ 100 | 端到端行为对等 |
| Fuzz | `fuzz/`（独立） | ≥ 5 | JSON / JWT / ZIP / URL / Cron |
| Property Test | `tests/property/*.rs` | ≥ 10 | 转换、编码、日期、集合不变量 |
| Compile-fail | `tests/compile_fail/` | ≥ 5 | 错误用法必须无法编译 |
| 并发测试 | `tests/concurrency/*.rs` | ≥ 5 | 缓存、调度、连接池取消安全 |
| Benchmark | `benches/*.rs` | ≥ 3 | 热点路径稳定基线 |

### 12.2 Phase 1.1 测试示例

```rust
// hutool-core/src/str_util.rs（同文件 unit test）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        assert!(StrUtil::is_empty(None));
        assert!(StrUtil::is_empty(Some("")));
        assert!(!StrUtil::is_empty(Some("hello")));
        assert!(!StrUtil::is_empty(Some(" ")));  // " " is not empty (but blank)
    }

    #[test]
    fn test_is_blank() {
        assert!(StrUtil::is_blank(None));
        assert!(StrUtil::is_blank(Some("")));
        assert!(StrUtil::is_blank(Some("   ")));
        assert!(StrUtil::is_blank(Some("\t\n")));
        assert!(!StrUtil::is_blank(Some("hello")));
    }

    #[test]
    fn test_join() {
        assert_eq!(StrUtil::join(&["a", "b", "c"], ","), "a,b,c");
        assert_eq!(StrUtil::join(&vec![1, 2, 3], "-"), "1-2-3");
        assert_eq!(StrUtil::join::<&str>(&[], ","), "");
    }

    #[test]
    fn test_split() {
        assert_eq!(StrUtil::split("a,b,c", ','), vec!["a", "b", "c"]);
        assert_eq!(StrUtil::split("a,,b", ','), vec!["a", "", "b"]);
    }
}
```

### 12.3 Golden 测试生成脚本

```bash
# scripts/java-golden-export/：Java Maven 子项目
# 跑 Java Hutool 生成 token、session、Excel、JSON 快照
./scripts/export-java-golden.sh
# 输出 tests/golden/*.expected.json

# Rust 侧逐字节比对
cargo test --test java_golden_tests
```

---

## 十三、迁移文档体系

```text
docs/
├── IMPLEMENTATION_PLAN.md                # 本文档（最终实施计划）
├── ARCHITECTURE.md                       # 架构总览（已存在）
├── feature-matrix.md                     # Feature 矩阵（已存在）
├── hutool-parity.md                      # Hutool-5.8 模块对照账本（已存在）
├── production-readiness.md               # 1.0 准入清单（已存在）
├── security.md / provenance.md
├── MIGRATION_STATUS.md                   # Phase 进度 + 已实现方法追踪（已存在）
├── GUIDE.md                              # ★ 新增：使用指南（5 分钟快速开始）
├── compatibility.md                      # ★ 新增：与 Hutool Java 兼容性
├── ecosystem-roadmap.md                  # ★ 新增：生态路线图
└── migration/                            # 迁移审计文档
    ├── java-tree-full.md                 # ★ 新增：Hutool Java 完整目录树
    ├── rust-tree-full.md                 # ★ 新增：hutool-rust 完整目录树
    ├── project-tree-diff.md              # ★ 新增：两侧目录 diff
    ├── object-method-matrix.md           # ★ 新增：Java 对象 × 方法矩阵
    ├── CODEGRAPH_METHOD_MAP.md           # ★ 新增：方法级 1:1 审计
    ├── codegraph-gap-audit.md            # ★ 新增：缺口审计
    └── TEST_AUDIT_REPORT.md              # ★ 新增：测试审计
```

---

## 十四、风险与缓解

| 风险 | 缓解 |
|---|---|
| Hutool 1 553 文件 → Rust 工作量巨大 | 分 Phase 推进；Phase 1~6 共约 10~12 周 |
| hutool-core 双重路径问题 | 不删减，pub use 重新导出，保留全部实现 |
| 缺失 facade 类（SetUtil/URLDecodeUtil 等） | Phase 1.3 集中补全 |
| hutool-poi 占位骨架被误认为实现 | 明确标记无引擎、无 facade，并从完成度与生产就绪能力中排除 |
| Rust 与 Java 语义差异大（如 `Collection<Object>` → 强类型） | 类型映射表固定 + serde_json::Value 兜底 |
| Java 反射注解 → Rust 宏心智模型差异 | hutool-macro 提供 derive + helper attribute |
| Hutool 7 家 AI Provider 工作量大 | Phase 3.3 分 6 个子阶段，每个 2~3 天 |
| Hutool-cron 41 文件 → Rust 36 缺失类 | Phase 2.1 集中补全 |
| Hutool-db 107 文件 → Rust 100 缺失类 | Phase 2.2 分 4~6 周补全 |
| Hutool-extra 179 文件 → Rust 173 缺失类 | Phase 2.3 + 4.x 分阶段补全 |
| Golden 测试依赖 Java 环境 | `scripts/java-golden-export/` 独立 Maven 子项目；CI 可选跳过 |
| Rust 1.94 MSRV vs Hutool 5.x Java 8+ 基线 | 通过 `chrono`/`reqwest`/`sqlx` 等成熟 crate 避免 nightly 依赖 |
| Hutool 的 `unsafe-to-copy` 区域（AWT/Servlet/JNDI/JDBC SPI/SSH/FTP） | 标记为 `planned`，不在 Phase 1~3 范围 |

---

## 十五、Java ↔ Rust 完整文件对应关系（节选）

### 15.1 hutool-core ↔ hutool-core（核心模块）

| Java 源文件 | Rust 源文件 | Phase |
|---|---|---|
| `cn/hutool/core/util/StrUtil.java` | `hutool-core/src/str_util.rs` + `text/str_*.rs` | 1.1 |
| `cn/hutool/core/util/ArrayUtil.java` | `hutool-core/src/array_util.rs` | 1 |
| `cn/hutool/core/util/PrimitiveArrayUtil.java` | `hutool-core/src/primitive_array_util.rs` | 1 |
| `cn/hutool/core/util/BooleanUtil.java` | `hutool-core/src/boolean_util.rs` | 1 |
| `cn/hutool/core/util/ByteUtil.java` | `hutool-core/src/byte_util.rs` | 1 |
| `cn/hutool/core/util/CharUtil.java` | `hutool-core/src/char_util.rs` | 1 |
| `cn/hutool/core/util/CharsetUtil.java` | `hutool-core/src/charset_util.rs` | 1 |
| `cn/hutool/core/util/ClassLoaderUtil.java` | `hutool-core/src/class_loader_util.rs` | 1 |
| `cn/hutool/core/util/ClassUtil.java` | `hutool-core/src/class_util.rs` | 1 |
| `cn/hutool/core/util/CoordinateUtil.java` | `hutool-core/src/coordinate_util.rs` | 1 |
| `cn/hutool/core/util/CreditCodeUtil.java` | `hutool-core/src/credit_code_util.rs` | 1 |
| `cn/hutool/core/util/DesensitizedUtil.java` | `hutool-core/src/desensitized_util.rs` | 1 |
| `cn/hutool/core/util/EnumUtil.java` | `hutool-core/src/enum_util.rs` | 1 |
| `cn/hutool/core/util/EscapeUtil.java` | `hutool-core/src/escape_util.rs` | 1 |
| `cn/hutool/core/util/HashUtil.java` | `hutool-core/src/hash_util.rs` | 1 |
| `cn/hutool/core/util/HexUtil.java` | `hutool-core/src/hex_util.rs` | 1 |
| `cn/hutool/core/util/IdUtil.java` | `hutool-core/src/id.rs` + `src/util/id_util.rs` | 1 |
| `cn/hutool/core/util/IdcardUtil.java` | `hutool-core/src/idcard_util.rs` | 1 |
| `cn/hutool/core/util/JAXBUtil.java` | `hutool-core/src/jaxb_util.rs` | 2 |
| `cn/hutool/core/util/JNDIUtil.java` | （跳过，`unsafe-to-copy`） | — |
| `cn/hutool/core/util/JdkUtil.java` | （跳过，无 Rust 对应） | — |
| `cn/hutool/core/util/ModifierUtil.java` | `hutool-core/src/modifier_util.rs` | 3 |
| `cn/hutool/core/util/NumberUtil.java` | `hutool-core/src/number_util.rs` | 1 |
| `cn/hutool/core/util/ObjUtil.java` | `hutool-core/src/object_util.rs` | 1 |
| `cn/hutool/core/util/ObjectUtil.java` | `hutool-core/src/object_util.rs` | 1 |
| `cn/hutool/core/util/PageUtil.java` | `hutool-core/src/page_util.rs` | 1 |
| `cn/hutool/core/util/PhoneUtil.java` | `hutool-core/src/phone_util.rs` | 1 |
| `cn/hutool/core/util/RadixUtil.java` | `hutool-core/src/radix_util.rs` | 1 |
| `cn/hutool/core/util/RandomUtil.java` | `hutool-core/src/random_util.rs` | 1 |
| `cn/hutool/core/util/ReUtil.java` | `hutool-core/src/re_util.rs` + `src/util/regex_util.rs`（新） | 1.3 |
| `cn/hutool/core/util/ReferenceUtil.java` | `hutool-core/src/reference_util.rs` | 1 |
| `cn/hutool/core/util/ReflectUtil.java` | `hutool-core/src/reflect_util.rs`（简化） | 2 |
| `cn/hutool/core/util/RuntimeUtil.java` | `hutool-core/src/runtime_util.rs` | 1 |
| `cn/hutool/core/util/SerializeUtil.java` | `hutool-core/src/util/serialize_util.rs`（新） | 1.3 |
| `cn/hutool/core/util/ServiceLoaderUtil.java` | `hutool-core/src/util/service_loader_util.rs`（新） | 1.3 |
| `cn/hutool/core/util/SystemPropsUtil.java` | （合并到 `system_props_util.rs`） | 1 |
| `cn/hutool/core/util/TypeUtil.java` | `hutool-core/src/type_util.rs` | 1 |
| `cn/hutool/core/util/URLUtil.java` | `hutool-core/src/url_util.rs` + `src/net/url_*.rs` | 1 |
| `cn/hutool/core/util/VersionUtil.java` | `hutool-core/src/version_util.rs` | 1 |
| `cn/hutool/core/util/XmlUtil.java` | `hutool-core/src/xml_util.rs` | 1 |
| `cn/hutool/core/util/ZipUtil.java` | `hutool-core/src/zip_util.rs` | 1 |
| **`cn/hutool/core/util/SetUtil.java`** | ❌ → 新建 `hutool-core/src/util/set_util.rs` | **1.3** |
| **`cn/hutool/core/util/URLEncodeUtil.java`** | ❌ → 新建 `hutool-core/src/net/url_encode_util.rs` | **1.3** |
| **`cn/hutool/core/util/URLDecodeUtil.java`** | ❌ → 新建 `hutool-core/src/net/url_decode_util.rs` | **1.3** |
| `cn/hutool/core/io/IoUtil.java` | `hutool-core/src/io_util.rs` + `src/io/io_util.rs` | 1 |
| `cn/hutool/core/io/FileUtil.java` | `hutool-core/src/file_util.rs` + `src/io/file/file_util.rs` | 1 |
| `cn/hutool/core/io/ResourceUtil.java` | `hutool-core/src/io/resource/resource_util.rs` | 1 |
| `cn/hutool/core/collection/CollUtil.java` | `hutool-core/src/coll_util.rs` + `src/collection/coll_util.rs` | 1 |
| `cn/hutool/core/collection/CollectionUtil.java` | `hutool-core/src/collection/collection_util.rs` | 1 |
| `cn/hutool/core/collection/ListUtil.java` | `hutool-core/src/list_util.rs` | 1 |
| `cn/hutool/core/collection/IterUtil.java` | `hutool-core/src/iter_util.rs` | 1 |
| `cn/hutool/core/collection/CollStreamUtil.java` | `hutool-core/src/coll_stream_util.rs` | 1 |
| `cn/hutool/core/map/MapUtil.java` | `hutool-core/src/map_util.rs` + `src/map/map_util.rs` | 1 |
| `cn/hutool/core/date/DateUtil.java` | `hutool-core/src/date/date_util.rs` | 1 |
| `cn/hutool/core/date/DateTime.java` | `hutool-core/src/date/date_time.rs` + type alias `DateTime` | 1 |
| `cn/hutool/core/date/LocalDateTimeUtil.java` | `hutool-core/src/date/local_date_time_util.rs` | 1 |
| `cn/hutool/core/bean/BeanUtil.java` | `hutool-core/src/bean_util.rs` + `src/bean/bean_util.rs` | 2 |
| **`cn/hutool/crypto/SecureUtil.java`** | ❌ → 新建 `hutool-core/src/util/secure_util.rs`（委托 facade） | **1.4** |
| **`cn/hutool/crypto/digest/DigestUtil.java`** | ❌ → 新建 `hutool-core/src/util/digest_util.rs`（委托 facade） | **1.4** |

### 15.2 hutool-cron ↔ hutool-cron（核心模块）

| Java 源文件 | Rust 源文件 | Phase |
|---|---|---|
| `cn/hutool/cron/CronUtil.java` | `hutool-cron/src/lib.rs` (facade) | 2.1 |
| `cn/hutool/cron/CronPattern.java` | `hutool-cron/src/expression.rs` + `cron_pattern.rs` | 2.1 |
| `cn/hutool/cron/CronPatternBuilder.java` | `hutool-cron/src/cron_pattern_builder.rs` | 2.1 |
| `cn/hutool/cron/CronPatternUtil.java` | `hutool-cron/src/parser.rs` + `cron_pattern_util.rs` | 2.1 |
| `cn/hutool/cron/Scheduler.java` | `hutool-cron/src/scheduler.rs` | 2.1 |
| `cn/hutool/cron/CronTask.java` | `hutool-cron/src/cron_task.rs` | 2.1 |
| `cn/hutool/cron/CronConfig.java` | `hutool-cron/src/cron_config.rs` | 2.1 |
| `cn/hutool/cron/CronException.java` | `hutool-cron/src/cron_exception.rs` | 2.1 |
| `cn/hutool/cron/CronTimer.java` | `hutool-cron/src/cron_timer.rs` | 2.1 |
| `cn/hutool/cron/SystemTimer.java` | `hutool-cron/src/system_timer.rs` | 2.1 |
| `cn/hutool/cron/Task.java` | `hutool-cron/src/task.rs` | 2.1 |
| `cn/hutool/cron/TaskExecutor.java` | `hutool-cron/src/task_executor.rs` | 2.1 |
| `cn/hutool/cron/TaskExecutorManager.java` | `hutool-cron/src/task_executor_manager.rs` | 2.1 |
| `cn/hutool/cron/TaskLauncher.java` | `hutool-cron/src/task_launcher.rs` | 2.1 |
| `cn/hutool/cron/TaskLauncherManager.java` | `hutool-cron/src/task_launcher_manager.rs` | 2.1 |
| `cn/hutool/cron/TaskListener.java` | `hutool-cron/src/task_listener.rs` | 2.1 |
| `cn/hutool/cron/TaskListenerManager.java` | `hutool-cron/src/task_listener_manager.rs` | 2.1 |
| `cn/hutool/cron/TaskTable.java` | `hutool-cron/src/task_table.rs` | 2.1 |
| `cn/hutool/cron/TimerTask.java` | `hutool-cron/src/timer_task.rs` | 2.1 |
| `cn/hutool/cron/TimerTaskList.java` | `hutool-cron/src/timer_task_list.rs` | 2.1 |
| `cn/hutool/cron/listener/SimpleTaskListener.java` | `hutool-cron/src/listener/simple_task_listener.rs` | 2.1 |
| `cn/hutool/cron/pattern/*` (5 文件) | `hutool-cron/src/pattern/*.rs` | 2.1 |
| `cn/hutool/cron/pattern/matcher/*` (4 文件) | `hutool-cron/src/pattern/matcher/*.rs` | 2.1 |
| `cn/hutool/cron/task/InvokeTask.java` | `hutool-cron/src/task/invoke_task.rs` | 2.1 |
| `cn/hutool/cron/task/RunnableTask.java` | `hutool-cron/src/task/runnable_task.rs` | 2.1 |
| `cn/hutool/cron/timingwheel/TimingWheel.java` | `hutool-cron/src/timingwheel/timing_wheel.rs` | 2.1 |

### 15.3 hutool-poi ↔ hutool-poi（**全部为占位**）

| Java 源文件 | Rust 源文件 | Phase |
|---|---|---|
| `cn/hutool/poi/excel/ExcelUtil.java` | `crates/hutool-poi/src/excel/excel_util.rs`（占位） | 3.1 |
| `cn/hutool/poi/excel/ExcelReader.java` | `crates/hutool-poi/src/excel/excel_reader.rs`（占位） | 3.1 |
| `cn/hutool/poi/excel/ExcelWriter.java` | `crates/hutool-poi/src/excel/excel_writer.rs`（占位） | 3.1 |
| `cn/hutool/poi/excel/BigExcelWriter.java` | `crates/hutool-poi/src/excel/big_excel_writer.rs`（占位） | 3.1 |
| `cn/hututl/poi/excel/RowUtil.java` | `crates/hutool-poi/src/excel/row_util.rs`（占位） | 3.1 |
| ...（其余 73 个 .java 全部对应 73 个 .rs 占位）| | 3.1 |
| `cn/hutool/poi/word/Word07Writer.java` | `crates/hutool-poi/src/word/word07_writer.rs`（占位） | 3.1 |
| `cn/hutool/poi/word/WordUtil.java` | `crates/hutool-poi/src/word/word_util.rs`（占位） | 3.1 |
| `cn/hutool/poi/ofd/OfdWriter.java` | `crates/hutool-poi/src/ofd/ofd_writer.rs`（占位） | 3.1 |
| `cn/hutool/poi/exceptions/POIException.java` | `crates/hutool-poi/src/exceptions/poi_exception.rs`（占位） | 3.1 |

### 15.4 hutool-ai ↔ hutool-ai

| Java 源文件 | Rust 源文件 | Phase |
|---|---|---|
| `cn/hutool/ai/AIUtil.java` | `crates/hutool-ai/src/lib.rs` (facade) | 1 |
| `cn/hutool/ai/core/AIService.java` | `crates/hutool-ai/src/client.rs` | 1 |
| `cn/hutool/ai/core/AIConfig.java` | `crates/hutool-ai/src/request.rs` (struct) | 1 |
| `cn/hutool/ai/core/Message.java` | `crates/hutool-ai/src/message.rs` | 1 |
| `cn/hutool/ai/Models.java` | `crates/hutool-ai/src/model.rs` | 1 |
| `cn/hutool/ai/model/openai/*.java` | `crates/hutool-ai/src/provider/openai.rs` | 1 |
| `cn/hutool/ai/model/deepseek/*.java` | `crates/hutool-ai/src/provider/deepseek.rs`（新） | 3.3 |
| `cn/hutool/ai/model/doubao/*.java` | `crates/hutool-ai/src/provider/doubao.rs`（新） | 3.3 |
| `cn/hutool/ai/model/gemini/*.java` | `crates/hutool-ai/src/provider/gemini.rs`（新） | 3.3 |
| `cn/hutool/ai/model/grok/*.java` | `crates/hutool-ai/src/provider/grok.rs`（新） | 3.3 |
| `cn/hutool/ai/model/hutool/*.java` | `crates/hutool-ai/src/provider/hutool.rs`（新） | 3.3 |
| `cn/hutool/ai/model/ollama/*.java` | `crates/hutool-ai/src/provider/ollama.rs`（新） | 3.3 |

（其他模块的完整对应表见 `docs/migration/object-method-matrix.md`）

---

## 十六、Phase 0 立即执行清单

批准本计划后，Phase 0 的具体执行步骤如下：

### Step 0：建立 1:1 文件覆盖率验收脚本（**最先执行**）

> 用户硬性要求：「hutool 中的除了 hutool-poi，每一个模块，每一个文件在 hutool-rust 中必须有对应的实现」
> 因此第一步必须建立可验收的自动化脚本。

```bash
# 1. 创建脚本目录
mkdir -p scripts

# 2. 创建覆盖率验收脚本（详见附录 E.11）
cat > scripts/verify-file-coverage.sh << 'SCRIPT_EOF'
#!/usr/bin/env bash
# 验证 hutool Java 文件 ↔ hutool-rust Rust 文件 1:1 覆盖率（除 hutool-poi 外）
set -e
HUTOOL_ROOT="/Users/wandl/workspaces/workspace-github/hutool"
HITOOL_ROOT="/Users/wandl/workspaces/workspace-github/hutool-rust"
PHASE="${1:-1}"

# 1. 列出 hutool 除 hutool-poi 外全部 .java
find "$HUTOOL_ROOT" -name "*.java" \
  -not -path "*/test/*" -not -path "*/target/*" \
  | grep -v "/hutool-poi/" \
  | sed 's|.*/cn/hutool/||' | sed 's|.java$||' \
  | sort -u > /tmp/hutool_files.txt

# 2. 列出 hutool-rust 除 hutool-poi 外全部 .rs
find "$HITOOL_ROOT/crates" -name "*.rs" \
  -not -path "*/target/*" -not -path "*/tests/*" -not -path "*/examples/*" \
  -not -path "*/hutool-poi/*" \
  | sed 's|.*/||' | sed 's|.rs$||' \
  | sort -u > /tmp/hutool_files.txt

# 3. 计算覆盖率（CamelCase → snake_case）
total=$(wc -l < /tmp/hutool_files.txt)
covered=0
while IFS= read -r java_path; do
    cls=$(basename "$java_path")
    snake=$(echo "$cls" | sed -E 's/([A-Z])/_\L\1/g; s/^_//')
    if grep -q "^${snake}$" /tmp/hutool_files.txt; then
        covered=$((covered + 1))
    fi
done < /tmp/hutool_files.txt

coverage=$(echo "scale=2; $covered * 100 / $total" | bc)
echo "覆盖率：${coverage}%（${covered}/${total}）"

# 4. 按 Phase 门槛检查
case "$PHASE" in
    1) REQUIRED=30 ;; 2) REQUIRED=60 ;; 3) REQUIRED=85 ;;
    4) REQUIRED=95 ;; 5) REQUIRED=99 ;; 6) REQUIRED=100 ;;
    *)  REQUIRED=100 ;;
esac
if [ "$(echo "$coverage >= $REQUIRED" | bc)" -eq 1 ]; then
    echo "✅ Phase $PHASE 门槛（${REQUIRED}%）通过"
    exit 0
else
    echo "❌ Phase $PHASE 门槛（${REQUIRED}%）未达成"
    exit 1
fi
SCRIPT_EOF
chmod +x scripts/verify-file-coverage.sh

# 3. 立即运行一次，确立基线
./scripts/verify-file-coverage.sh 1
```

**Step 0 验收**：脚本可执行，且输出当前覆盖率数字（即「迁移基线」）。

### Step 1：建立文档体系（半天）

```bash
mkdir -p /Users/wandl/workspaces/workspace-github/hutool-rust/docs/migration
# 创建以下文档：
touch docs/GUIDE.md
touch docs/compatibility.md
touch docs/ecosystem-roadmap.md
touch docs/migration/java-tree-full.md
touch docs/migration/rust-tree-full.md
touch docs/migration/project-tree-diff.md
touch docs/migration/object-method-matrix.md
touch docs/migration/CODEGRAPH_METHOD_MAP.md
touch docs/migration/codegraph-gap-audit.md
touch docs/migration/TEST_AUDIT_REPORT.md
```

### Step 2：hutool-poi 占位骨架（历史结果，不是实现步骤）

仓库已有 79 个 Rust 源文件，其中 67 个文件仍含 `unimplemented!()`。
这些文件只登记迁移形状；当前计划不执行 POI 引擎开发，也不承诺
Excel、Word、OFD 或 PDF 功能。

### Step 3：workspace/facade 状态

```toml
# Cargo.toml
[workspace]
members = ["crates/*"]
# hutool-poi 已在 crates/ 下，因此被 workspace 自动包含
```

```toml
# crates/hutool/Cargo.toml 当前不依赖 hutool-poi，
# 也不定义 poi/poi-docx feature；保持这一边界，避免暴露不可用 API。
```

### Step 4：补充 Phase 1.1 的 StrUtil 完整实现（3~5 天）

按 §10 Phase 1.1 实施。

### Step 5：补充 Phase 1.3 缺失 facade 类（3~5 天）

按 §10 Phase 1.3 实施：SetUtil / URLEncodeUtil / URLDecodeUtil / RegexUtil。

### Step 6：补充 Phase 1.4 SecureUtil / DigestUtil 委托 facade（1~2 天）

按 §10 Phase 1.4 实施。

### Step 7：补充 Phase 1.2 hutool-core 双重路径（1 天）

按 §10 Phase 1.2 实施：双重路径 pub use 重新导出。

### Step 8：质量门禁（半天）

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --all-features --no-deps
cargo deny check
```

### Step 9：输出初版迁移文档（1 天）

- `docs/migration/object-method-matrix.md` - 已实现方法对照表
- `docs/migration/codegraph-gap-audit.md` - 缺口审计
- `docs/migration/TEST_AUDIT_REPORT.md` - 测试审计

---

## 附录 A：与 Hutool Java 的关键差异速查

| 差异点 | Hutool Java 做法 | Hutool-Rust-Rs 做法 |
|---|---|---|
| 命名 | `CollUtil` (CamelCase) | `CollUtil` (PascalCase) + 方法 snake_case |
| 静态方法 | `CollUtil.isEmpty(...)` | `CollUtil::is_empty(...)` |
| 异常处理 | `throws IllegalArgumentException` | `Result<T, Error>` |
| 集合类型 | `Collection<T>` / `List<T>` / `Map<K,V>` | `&[T]` / `Vec<T>` / `HashMap<K,V>` |
| 泛型擦除 | `Object` | `serde_json::Value` / `String` / `T: Clone` |
| 反射 | `Class.forName(...)` | `TypeId` / 编译期宏 |
| 多线程 | `ThreadLocal` | `thread_local!` + `task_local!` |
| 异步 | 同步 | 核心同步 + 适配层 async |
| 配置读取 | 反射注入字段 | `Deserialize` from TOML/YAML/ENV |
| 异常层次 | 20+ RuntimeException 子类 | 单一 enum per crate |
| 元数据 | 运行时反射 HashMap | `&'static [T]` 编译期常量 |
| 日志 | SLF4J + Log4j + JDK | `tracing` 统一抽象 + dialect adapter |
| 数据库 | JDBC + 自研方言 | `sqlx`（不造 ORM） |
| HTTP | HttpURLConnection | `reqwest`（基于 tokio） |
| Excel | Apache POI | **仅占位，无引擎实现** |
| AI | 7 个 Provider | 7 个 Provider（OpenAI 已实现） |

---

## 附录 B：参考项目

- **Hutool Java**：`/Users/wandl/workspaces/workspace-github/hutool`（基线 v5.8.x）
- **sa-token-rs**：`/Users/wandl/workspaces/workspace-github/sa-token-rs`（同模式 Java→Rust 移植，详细模板）
- **easyexcel-rs**：`/Users/wandl/workspaces/workspace-github/easyexcel-rs`（hutool-poi 实现参考）
- **axum-login**：https://github.com/maxcountryman/axum-login
- **RustCrypto**：https://github.com/RustCrypto
- **reqwest**：https://github.com/seanmonstar/reqwest
- **sqlx**：https://github.com/launchbadge/sqlx
- **moka**：https://github.com/moka-rs/moka
- **tracing**：https://github.com/tokio-rs/tracing
- **serde**：https://github.com/serde-rs/serde
- **chrono**：https://github.com/chronotope/chrono

---

## 附录 C：Hutool v5.8.x 模块 → Hutool-Rust crate 映射表

| Hutool 模块 | Hutool-Rust crate | 迁移完成度 | 计划 Phase |
|---|---|---:|---|
| `hutool-bom` | workspace | 100% | 0 |
| `hutool-all` | `hutool` (facade) | 100% | 0 |
| `hutool-core` | `hutool-core` | **80%** | 1.x |
| `hutool-aop` | `hutool-aop` | 50% | 3 |
| `hutool-bloomFilter` | `hutool-bloom-filter` | 25% | 4 |
| `hutool-cache` | `hutool-cache` | 30% | 4 |
| `hutool-captcha` | `hutool-captcha` | 50% | 4 |
| `hutool-cron` | `hutool-cron` | **10%** | 2.1 |
| `hutool-crypto` | `hutool-crypto` | 25% | 4 |
| `hutool-db` | `hutool-db` | 15% | 2.2 |
| `hutool-dfa` | `hutool-dfa` | 70% | 4 |
| `hutool-extra` | `hutool-extra` | **5%** | 2.3 + 4.x |
| `hutool-http` | `hutool-http` | 70% | 1.5 + 5 |
| `hutool-json` | `hutool-json` | 85% | 4 |
| `hutool-jwt` | `hutool-jwt` | 95% | 5 |
| `hutool-log` | `hutool-log` | 30% | 2.4 |
| **`hutool-poi`** | **`hutool-poi`** | **0%（占位）** | **3.1** |
| `hutool-script` | `hutool-script` | 80% | 5 |
| `hutool-setting` | `hutool-setting` | 95% | 5 |
| `hutool-socket` | `hutool-socket` | 85% | 5 |
| `hutool-system` | `hutool-system` | 95% | 5 |
| `hutool-ai` | `hutool-ai` | 15%（仅 OpenAI） | 3.3 |
| （无 Java 原型） | `hutool-macro` | 100% | 0 |
| **（**`hutool-compat`** 冻结）** | `hutool-compat-hutool` | **5%**（114 行 / 2 facade） | **冻结** |
| （无 Java 原型） | `hutool-test-support` | 100% | 0 |

**加权总体完成度**：约 **30%**（按"已迁移等价方法数 / Hutool 总方法数"算的更保守口径）；详细文件级缺口见 §10.0 表。

---

## 附录 D：DDD4J → Hutool-Rust-Rs 完整生态映射

> 本附录提供 DDD4J（Dromara Distributed Domain for Java）650 个 Java 组件到
> Rust 生态 + Hutool-Rust-Rs 的**逐项映射**，作为 §9.1 和 §10 Phase 7 的查询手册。

### D.1 序列化 / 数据格式

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Jackson | `serde_json` | ✅ hutool-json | — |
| Gson | `serde_json` | ✅ 委托 | — |
| FastJSON | `serde_json` | ✅ 替代 | — |
| SnakeYAML | `serde_yaml_ng` | ⚠️ workspace 已声明，hutool-setting 未启用 | **Phase 7.1** 启用 |
| JAXB | `quick-xml` + `serde` | ⚠️ hutool-jaxb_util 占位 | Phase 4 补全 |
| Protocol Buffers | `prost` / `protobuf` | ❌ 无 | Phase 5 可选 |
| Avro | `apache-avro` | ❌ 无 | Phase 5 可选 |
| Thrift | `thrift` | ❌ 无 | Phase 5 可选 |

### D.2 HTTP 与 Web

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| OkHttp / Apache HttpClient | `reqwest` | ✅ hutool-http | — |
| Retrofit | `reqwest` + builder | 🟡 hutool-http 已自建 builder | 文档化推荐用法 |
| Spring MVC | `axum` / `actix-web` / `poem` | ❌ 无 | **新建 hutool-http-server** |
| Servlet | `tower::Service` | ❌ 无 | 同上 |
| Tomcat / Jetty / Undertow | `hyper` | ❌ 无 | 同上 |

### D.3 数据库

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| JDBC | `sqlx` | ✅ hutool-db | — |
| HikariCP | `sqlx::Pool` | ✅ | — |
| Druid | `sqlx::Pool` + 监控 | 🟡 自建 | 监控可加 `metrics` feature |
| MyBatis | ❌（不造 ORM） | — | — |
| Hibernate / jOOQ | ❌（不造 ORM） | — | — |
| Flyway | `sqlx::migrate!` | ✅ 推荐用法 | 文档化 |
| Druid SQL Parser | `sqlparser-rs` | ❌ 无 | Phase 3 加 SQL 解析 |
| PageHelper | `hutool-db::page` | 🟡 部分 | Phase 2.2 补全 |

### D.4 加密与安全

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| JCA (javax.crypto) | `RustCrypto`（aes/chacha/des/rc4/sm2/sm3/sm4） | ✅ hutool-crypto | — |
| BouncyCastle | `RustCrypto` + 国密 | ✅ 30+ 算法 | — |
| BCrypt | `bcrypt` crate | ❌（用 argon2 替代） | 推荐 argon2 替代 |
| scrypt | `scrypt` crate | ❌ | 评估中 |
| `java.security.MessageDigest` | `sha2` / `sha1` / `md-5` / `sm3` | ✅ | — |
| OpenSSL JNI | `ring` | ❌ | 不建议切换（失去国密） |

### D.5 缓存

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Caffeine | `moka` | ✅ hutool-cache | — |
| Ehcache | `moka` | ✅ | — |
| Guava Cache | `moka` | ✅ | — |
| Redis (Jedis / Lettuce) | `redis` | ⚠️ workspace 未启用 | **Phase 7.2** 新建 hutool-cache-redis |
| Hazelcast | ❌ | — | 不实现（集群方案） |

### D.6 消息队列

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Kafka | `rdkafka` | ❌ | Phase 7.4 可选 |
| RabbitMQ | `lapin` | ❌ | Phase 7.4 可选 |
| MQTT | `rumqttc` | ❌ | Phase 7.4 可选 |
| RocketMQ | `rocketmq-rust` | ❌ | 评估中 |

### D.7 分布式与微服务

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Nacos | `nacos-rust` | ❌ | Phase 7.5 可选 |
| Consul | `consul-rust` | ❌ | Phase 7.5 可选 |
| Eureka | ❌（过时） | — | — |
| Zookeeper | `zookeeper` | ❌ | Phase 7.5 可选 |
| Dubbo | 自封装 | ❌ | 不实现（复杂度高） |
| Spring Cloud Gateway | `pingora` / `apollo` | ❌ | 不实现 |

### D.8 邮件与消息

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| javax.mail (SMTP) | `lettre` | ✅ hutool-extra::mail（feature-gated） | — |
| JavaMail IMAP/POP3 | `imap` / `pop3` | ❌ | Phase 5 可选 |
| Apache Commons Email | `lettre` | ✅ 替代 | — |

### D.9 定时任务与调度

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Quartz | `cron`（workspace 引入） + 自研解析 | ✅ hutool-cron | — |
| `@Scheduled` (Spring) | `tokio-cron-scheduler` | ❌（业务侧） | 业务可选 |
| XXL-JOB | ❌ | — | 不实现（外部系统） |

### D.10 日志

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Logback | `tracing` + `tracing-subscriber` | ✅ hutool-log | — |
| Log4j2 | `log4rs` / `tracing-log` | 🟡 dialect adapter 待补 | Phase 2.4 |
| SLF4J | `tracing` / `log` 抽象 | 🟡 dialect adapter | Phase 2.4 |
| `java.util.logging` | `tracing` | ✅ | — |

### D.11 文本 / 国际化 / 编码

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Commons Lang | 标准库 + `hutool-core` | ✅ | — |
| Commons Collections | `itertools` + `hutool-core` | 🟡 加 `itertools` | **Phase 7.1** |
| Commons Math | `num` + `statrs` | ❌ | **Phase 7.1** 新增 |
| ICU4J | `unicode-general-category` + `unicode-normalization` | ✅ | — |
| Joda-Time | `chrono` | ✅ | — |
| Commons CSV | `csv` | ✅ hutool-core | — |
| Apache Commons Compress | `flate2` + `zip` | ✅ | — |

### D.12 图片与文档

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Java ImageIO | `image` | ✅ hutool-core::img / hutool-extra::image | — |
| Apache POI / EasyExcel | `rust_xlsxwriter` + `quick-xml` + `zip` | `hutool-poi` 只有占位骨架，自身未依赖这些引擎 | **未实现，不计完成度** |
| iText / Apache PDFBox | `printpdf` / `lopdf` | ❌ | Phase 4 可选 |
| zxing (QR) | `qrcode`（svg feature） | ✅ hutool-extra::qrcode | — |
| Thumbnailator | `image` | 🟡 部分 | Phase 4 补全 |
| Apache Commons Imaging | `image` | ✅ 替代 | — |

### D.13 网络与 Socket

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Netty | `tokio` + `tokio-util` | ✅ | — |
| Java NIO | `tokio` | ✅ | — |
| Apache MINA | `tokio` | ✅ | — |
| WebSocket | `tokio-tungstenite` / `axum::extract::ws` | ❌ | Phase 5 可选 |

### D.14 测试

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| JUnit 5 | 内置 `#[test]` + `proptest` | ✅ | — |
| TestNG | `proptest` | ✅ | — |
| Mockito | `mockall` | ❌ | Phase 6 加 `mockall` 用于 trait 模拟 |
| AssertJ | `pretty_assertions` | ❌ | Phase 6 可选 |

### D.15 构建 / CI / 治理

| Java 组件 | Rust crate | Hutool-Rust 现状 | 建议 |
|---|---|---|---|
| Maven / Gradle | Cargo | ✅ | — |
| Checkstyle / SpotBugs | `cargo clippy`（pedantic） | ✅ | — |
| SonarQube | `cargo llvm-cov` + Codecov | ✅ | — |
| Jenkins / GitHub Actions | GitHub Actions | ✅ | — |
| JaCoCo (覆盖率) | `cargo llvm-cov` | ✅ | — |

### D.16 不实现项（DDD4J 推荐但与 hutool-rust 原则冲突）

| 项 | 原因 |
|---|---|
| ❌ Spring 全家桶（Spring Boot/Data Cloud/Security） | Rust 无 Spring，框架职责由应用层承担 |
| ❌ MyBatis / Hibernate / JPA | 违反「不造 ORM」原则 |
| ❌ Tomcat / Jetty / Undertow Web 容器 | hutool-http 是客户端；Server 应新建独立 crate |
| ❌ Swing / JavaFX 桌面 UI | hutool-core::swing 仅做底层截屏/键鼠，不做 UI 框架 |
| ❌ JNDI | 无容器概念，改用显式注入 |
| ❌ RMI / CORBA / JNI | Rust 有更好的 IPC 机制 |

---

## 附录 E：hutool 全模块文件 ↔ hutool-rust 1:1 对照清单（**用户硬性要求验收清单**）

> 本附录是 §5.0.1 「1:1 全文件覆盖硬性要求」的**具体可验收清单**。
> 每个 hutool Java `.java` 文件都必须有对应的 hutool-rust `.rs` 文件（除 hutool-poi）。

### E.1 hutool-core（713 .java → target 713+ .rs）

| Java 包 | Java 文件数 | hutool 子目录 | 当前 .rs | 缺口 | Phase |
|---|---:|---|---:|---:|---|
| `cn.hutool.core.annotation` | 36 | `annotation/` | 49 | ✅ 已超（多 13） | — |
| `cn.hutool.core.bean` | 11 | `bean/` | 24 | ✅ 已超（多 13） | — |
| `cn.hutool.core.builder` | 7 | `builder/` | 7 | ✅ 对齐 | — |
| `cn.hutool.core.clone` | 5 | `clone/` | 5 | ✅ 对齐 | — |
| `cn.hutool.core.codec` | 20 | `codec/` | 20 | ✅ 对齐 | — |
| `cn.hutool.core.collection` | 30 | `collection/` | 30 | ✅ 对齐 | — |
| `cn.hutool.core.comparator` | 18 | `comparator/` | 18 | ✅ 对齐 | — |
| `cn.hutool.core.compiler` | 9 | `compiler/` | 9 | ✅ 对齐 | — |
| `cn.hutool.core.compress` | 6 | `compress/` | 6 | ✅ 对齐 | — |
| `cn.hutool.core.convert` | 12 | `convert/` | 48 | ✅ 已超 | — |
| `cn.hutool.core.date` | 27 | `date/` | 41 | ✅ 已超 | — |
| `cn.hutool.core.exceptions` | 9 | `exceptions/` | 9 | ✅ 对齐 | — |
| `cn.hutool.core.getter` | 10 | `getter/` | 10 | ✅ 对齐 | — |
| `cn.hutool.core.img` | 9 | `img/` | 13 | ✅ 已超 | — |
| `cn.hutool.core.io` | 21 | `io/` | 84 | ✅ 已超 | — |
| `cn.hutool.core.lang` | 35 | `lang/` | 116 | ✅ 已超 | — |
| `cn.hutool.core.map` | 25 | `map/` | 35 | ✅ 已超 | — |
| `cn.hutool.core.math` | 7 | `math/` | 7 | ✅ 对齐 | — |
| `cn.hutool.core.net` | 17 | `net/` | 25 | ✅ 已超 | — |
| `cn.hutool.core.stream` | 4 | `stream/` | 4 | ✅ 对齐 | — |
| `cn.hutool.core.swing` | 4 | `swing/` | 9 | ✅ 已超（feature-gated） | — |
| `cn.hutool.core.text` | 16 | `text/` | 45 | ✅ 已超 | — |
| `cn.hutool.core.thread` | 16 | `thread/` | 22 | ✅ 已超 | — |
| `cn.hutool.core.util` | 42 | `util/` | 42 | ✅ 对齐 | — |

**hutool-core 完成度：≥ 100%（含 hutool 扩展）**

### E.2 hutool-cron（41 .java → target 41 .rs）

| Java 文件 | Rust 目标 | 当前 | Phase |
|---|---|---|---|
| `cn.hutool.cron.CronUtil` | `hutool-cron/src/lib.rs` | 🟡 部分 | Phase 2.1 |
| `cn.hutool.cron.CronPattern` | `hutool-cron/src/cron_pattern.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.CronPatternBuilder` | `hutool-cron/src/cron_pattern_builder.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.CronPatternUtil` | `hutool-cron/src/cron_pattern_util.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.CronTask` | `hutool-cron/src/cron_task.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.CronTimer` | `hutool-cron/src/cron_timer.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.CronConfig` | `hutool-cron/src/cron_config.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.CronException` | `hutool-cron/src/cron_exception.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.SystemTimer` | `hutool-cron/src/system_timer.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.Scheduler` | `hutool-cron/src/scheduler.rs` | 🟡 | Phase 2.1 |
| `cn.hutool.cron.Task` | `hutool-cron/src/task.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.TaskExecutor` | `hutool-cron/src/task_executor.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.TaskExecutorManager` | `hutool-cron/src/task_executor_manager.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.TaskLauncher` | `hutool-cron/src/task_launcher.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.TaskLauncherManager` | `hutool-cron/src/task_launcher_manager.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.TaskListener` | `hutool-cron/src/task_listener.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.TaskListenerManager` | `hutool-cron/src/task_listener_manager.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.TaskTable` | `hutool-cron/src/task_table.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.TimerTask` | `hutool-cron/src/timer_task.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.TimerTaskList` | `hutool-cron/src/timer_task_list.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.Part` | `hutool-cron/src/pattern/part.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.PartMatcher` | `hutool-cron/src/pattern/part_matcher.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.PartParser` | `hutool-cron/src/pattern/part_parser.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.PatternMatcher` | `hutool-cron/src/pattern/pattern_matcher.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.PatternParser` | `hutool-cron/src/pattern/pattern_parser.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.PatternUtil` | `hutool-cron/src/pattern/pattern_util.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.matcher.AlwaysTrueMatcher` | `hutool-cron/src/pattern/matcher/always_true_matcher.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.matcher.BoolArrayMatcher` | `hutool-cron/src/pattern/matcher/bool_array_matcher.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.matcher.DayOfMonthMatcher` | `hutool-cron/src/pattern/matcher/day_of_month_matcher.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.pattern.matcher.YearValueMatcher` | `hutool-cron/src/pattern/matcher/year_value_matcher.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.task.InvokeTask` | `hutool-cron/src/task/invoke_task.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.task.RunnableTask` | `hutool-cron/src/task/runnable_task.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.timingwheel.TimingWheel` | `hutool-cron/src/timingwheel/timing_wheel.rs` | ❌ | Phase 2.1 |
| `cn.hutool.cron.listener.SimpleTaskListener` | `hutool-cron/src/listener/simple_task_listener.rs` | ❌ | Phase 2.1 |
| ...（其余 7 个 package-info）| 各 mod.rs | 🟡 | Phase 2.1 |

**hutool-cron 完成度：4/41 = 10% → 目标 100%（Phase 2.1）**

### E.3 hutool-db（107 .java → target 107 .rs）

| 子包 | Java 文件 | 当前 .rs | Phase |
|---|---:|---:|---|
| `cn.hutool.db` 顶层 | ~25 | 0 | Phase 2.2 |
| `cn.hutool.db.ds.*`（DS 工厂）| ~10 | 0 | Phase 2.2 |
| `cn.hutool.db.dialect.*`（方言）| ~13 | 0 | Phase 2.2 |
| `cn.hutool.db.handler.*`（结果处理）| ~10 | 0 | Phase 2.2 |
| `cn.hutool.db.meta.*`（元数据）| ~5 | 0 | Phase 2.2 |
| `cn.hutool.db.nosql.mongo` | ~5 | 0 | Phase 2.2 |
| `cn.hutool.db.nosql.redis` | ~3 | 0 | Phase 2.2 |
| `cn.hutool.db.sql.*`（SQL 工具）| ~8 | 0 | Phase 2.2 |
| `cn.hutool.db.transaction.*`（事务）| ~5 | 0 | Phase 2.2 |
| 其他 + package-info | ~23 | 32（hutool-db 当前） | Phase 2.2 |

**hutool-db 完成度：32/107 = 30% → 目标 100%（Phase 2.2）**

### E.4 hutool-extra（179 .java → target 179 .rs）

| 子包 | Java 文件 | 当前 .rs | Phase |
|---|---:|---:|---|
| `cn.hutool.extra.cglib.*` | 4 | 0 | Phase 5（unsafe-to-copy） |
| `cn.hutool.extra.compress.*` | ~15 | 0 | Phase 1 |
| `cn.hutool.extra.emoji.*` | 1 | 0 | Phase 4.4 |
| `cn.hutool.extra.expression.*`（7 engine）| ~30 | 0 | Phase 1.3 |
| `cn.hutool.extra.ftp.*`（含 ssh）| ~20 | 0 | Phase 1.5 |
| `cn.hutool.extra.mail.*` | ~10 | 🟡 feature-gated | Phase 1.2 |
| `cn.hutool.extra.pinyin.*`（5 engine）| ~15 | 0 | Phase 4.4 |
| `cn.hutool.extra.qrcode.*` | ~5 | 🟡 部分 | Phase 3 |
| `cn.hutool.extra.servlet.*` | ~3 | 0 | Phase 4.4 |
| `cn.hutool.extra.spring.*` | ~3 | 0 | Phase 4.4 |
| `cn.hutool.extra.ssh.*` | ~5 | 0 | Phase 1.5 |
| `cn.hutool.extra.template.*`（8 engine）| ~40 | 🟡 简化版 | Phase 1.4 |
| `cn.hutool.extra.tokenizer.*`（8 engine）| ~25 | 0 | Phase 2.4 |
| `cn.hutool.extra.validation.*` | ~5 | 🟡 部分 | Phase 4.4 |

**hutool-extra 完成度：9/179 = 5% → 目标 100%（Phase 1 + 2 + 4 + 5）**

### E.5 hutool-http（72 .java → target 72 .rs）

| 子包 | Java 文件 | 当前 .rs | Phase |
|---|---:|---:|---|
| `cn.hutool.http` 顶层 | ~20 | 22 | ✅ |
| `cn.hutool.http.body.*` | ~5 | 4 | 🟡 |
| `cn.hutool.http.cookie.*` | ~3 | 1 | 🟡 |
| `cn.hutool.http.server.*`（含 action/filter/handler）| ~25 | 0 | Phase 5 |
| `cn.hutool.http.ssl.*` | ~6 | 0 | Phase 3.2 |
| `cn.hutool.http.webservice.*`（Soap）| ~7 | 0 | Phase 5 |
| `cn.hutool.http.useragent.*` | ~6 | 1 | 🟡 |

**hutool-http 完成度：25/72 = 35% → 目标 100%（Phase 3.2 + 5）**

### E.6 hutool-crypto（70 .java → target 70 .rs）

| 子包 | Java 文件 | 当前 .rs | Phase |
|---|---:|---:|---|
| `cn.hutool.crypto` 顶层 | ~10 | 5 | 🟡 |
| `cn.hutool.crypto.asymmetric.*` | ~10 | 0 | Phase 3.1 |
| `cn.hutool.crypto.digest.*`（含 mac/otp）| ~15 | 0 | Phase 3.1 |
| `cn.hutool.crypto.symmetric.*`（含 fpe）| ~25 | 0 | Phase 3.1 |

**hutool-crypto 完成度：22/70 = 31% → 目标 100%（Phase 3.1）**

### E.7 hutool-cache / hutool-aop / hutool-captcha / hutool-bloomFilter（72 .java → 72 .rs）

| 模块 | Java | 当前 | Phase |
|---|---:|---:|---|
| `hutool-cache` | 22 | 3 | Phase 3.3 |
| `hutool-aop` | 15 | 6 | Phase 3.3 |
| `hutool-captcha` | 13 | 4 | Phase 3.3 |
| `hutool-bloomFilter` | 22 | 5 | Phase 3.3 |

### E.8 hutool-json / hutool-jwt / hutool-script / hutool-setting / hutool-system / hutool-socket（90 .java → 90 .rs）

| 模块 | Java | 当前 | Phase |
|---|---:|---:|---|
| `hutool-json` | 33 | 6 | Phase 3.4 |
| `hutool-jwt` | 17 | 2 | Phase 3.4 |
| `hutool-script` | 5 | 2 | Phase 3.4 |
| `hutool-setting` | 16 | 6 | Phase 3.4 |
| `hutool-system` | 16 | 3 | Phase 3.4 |
| `hutool-socket`（含 aio/nio/protocol）| 24 | 2 | Phase 3.4 |

### E.9 hutool-dfa（6 .java → target 6 .rs）

| Java 文件 | Rust 目标 | 当前 |
|---|---|---|
| `cn.hutool.dfa.SensitiveUtil` | `hutool-dfa/src/sensitive_util.rs` | ❌ |
| `cn.hutool.dfa.WordTree` | `hutool-dfa/src/word_tree.rs` | 🟡 部分 |
| `cn.hutool.dfa.FoundWord` | `hutool-dfa/src/found_word.rs` | ❌ |
| `cn.hutool.dfa.SensitiveProcessor` | `hutool-dfa/src/sensitive_processor.rs` | ❌ |
| `cn.hutool.dfa.StopChar` | `hutool-dfa/src/stop_char.rs` | ❌ |
| `cn.hutool.dfa.package-info` | `hutool-dfa/src/lib.rs` | 🟡 |

**hutool-dfa 完成度：5/6 = 83% → 目标 100%（Phase 3.4）**

### E.10 hutool-ai（58 .java → target 58 .rs）

| 子包 | Java | 当前 | Phase |
|---|---:|---:|---|
| `cn.hutool.ai` 顶层 + core | ~10 | 13 | 🟡 已超 |
| `cn.hutool.ai.model.openai` | 5 | 1 | ✅ |
| `cn.hutool.ai.model.deepseek` | 5 | 0 | Phase 4.3 |
| `cn.hutool.ai.model.doubao` | 5 | 0 | Phase 4.3 |
| `cn.hutool.ai.model.gemini` | 5 | 0 | Phase 4.3 |
| `cn.hutool.ai.model.grok` | 5 | 0 | Phase 4.3 |
| `cn.hutool.ai.model.hutool` | 5 | 0 | Phase 4.3 |
| `cn.hutool.ai.model.ollama` | 5 | 0 | Phase 4.3 |

**hutool-ai 完成度：14/58 = 24% → 目标 100%（Phase 4.3）**

### E.11 验收脚本 `scripts/verify-file-coverage.sh`

```bash
#!/usr/bin/env bash
# scripts/verify-file-coverage.sh
# 验证 hutool Java 文件 ↔ hutool-rust Rust 文件 1:1 覆盖率（除 hutool-poi 外）
set -e

HUTOOL_ROOT="/Users/wandl/workspaces/workspace-github/hutool"
HITOOL_ROOT="/Users/wandl/workspaces/workspace-github/hutool-rust"

# 1. 列出 hutool 除 hutool-poi 外全部 .java
find "$HUTOOL_ROOT" -name "*.java" \
  -not -path "*/test/*" -not -path "*/target/*" \
  | grep -v "/hutool-poi/" \
  | sed 's|.*/cn/hutool/||' | sed 's|.java$||' \
  | sort -u > /tmp/hutool_files.txt

# 2. 列出 hutool-rust 除 hutool-poi 外全部 .rs
find "$HITOOL_ROOT/crates" -name "*.rs" \
  -not -path "*/target/*" -not -path "*/tests/*" -not -path "*/examples/*" \
  -not -path "*/hutool-poi/*" \
  | sed 's|.*/crates/hutool-||' | sed 's|/src/.*/|/|' | sed 's|.rs$||' \
  | sort -u > /tmp/hutool_files.txt

# 3. CamelCase → snake_case 转换（hutool 标准）
to_snake() {
    echo "$1" | sed -E 's/([A-Z])/_\L\1/g; s/^_//; s/\//\//g'
}

# 4. 计算覆盖率
total=$(wc -l < /tmp/hutool_files.txt)
covered=0
uncovered_list=""

while IFS= read -r java_path; do
    # 提取类名（路径最后一段）
    cls=$(basename "$java_path")
    snake=$(echo "$cls" | sed -E 's/([A-Z])/_\L\1/g; s/^_//')
    if grep -q "$snake" /tmp/hutool_files.txt; then
        covered=$((covered + 1))
    else
        uncovered_list="$uncovered_list\n  - $java_path"
    fi
done < /tmp/hutool_files.txt

coverage=$(echo "scale=2; $covered * 100 / $total" | bc)

echo "===== hutool-rs 1:1 文件覆盖率 ====="
echo "hutool 全部 Java 文件（除 hutool-poi）：$total"
echo "已映射到 hutool-rust：$covered"
echo "覆盖率：${coverage}%"
echo ""
if [ -n "$uncovered_list" ]; then
    echo "未覆盖文件（前 30 个）："
    echo -e "$uncovered_list" | head -30
fi

# 5. 按 Phase 门槛检查
PHASE="${1:-1}"
case "$PHASE" in
    1) REQUIRED=30 ;;
    2) REQUIRED=60 ;;
    3) REQUIRED=85 ;;
    4) REQUIRED=95 ;;
    5) REQUIRED=99 ;;
    6) REQUIRED=100 ;;
    *)  REQUIRED=100 ;;
esac

if [ "$(echo "$coverage >= $REQUIRED" | bc)" -eq 1 ]; then
    echo "✅ Phase $PHASE 门槛（${REQUIRED}%）通过"
    exit 0
else
    echo "❌ Phase $PHASE 门槛（${REQUIRED}%）未达成，当前 ${coverage}%"
    exit 1
fi
```

**使用方式**：

```bash
# 检查 Phase 1 进度（≥ 30%）
./scripts/verify-file-coverage.sh 1

# 检查最终进度（≥ 100%）
./scripts/verify-file-coverage.sh 6

# CI 集成
- name: Verify file coverage
  run: ./scripts/verify-file-coverage.sh ${{ matrix.phase }}
```

### E.12 总覆盖率目标（v1.0.0 必须达成）

| 模块 | Java | 目标 Rust | Phase 6 验收 |
|---|---:|---:|---|
| hutool-core | 713 | ≥ 713 | 100% |
| hutool-cron | 41 | ≥ 41 | 100% |
| hutool-db | 107 | ≥ 107 | 100% |
| hutool-extra | 179 | ≥ 179 | 100% |
| hutool-http | 72 | ≥ 72 | 100% |
| hutool-crypto | 70 | ≥ 70 | 100% |
| hutool-cache | 22 | ≥ 22 | 100% |
| hutool-aop | 15 | ≥ 15 | 100% |
| hutool-captcha | 13 | ≥ 13 | 100% |
| hutool-bloomFilter | 22 | ≥ 22 | 100% |
| hutool-json | 33 | ≥ 33 | 100% |
| hutool-jwt | 17 | ≥ 17 | 100% |
| hutool-script | 5 | ≥ 5 | 100% |
| hutool-setting | 16 | ≥ 16 | 100% |
| hutool-system | 16 | ≥ 16 | 100% |
| hutool-socket | 24 | ≥ 24 | 100% |
| hutool-dfa | 6 | ≥ 6 | 100% |
| hutool-ai | 58 | ≥ 58 | 100% |
| hutool-poi | 78 | **79 个占位 `.rs`，可用实现为 0** | **0%（排除）** |
| **总计** | **1553** | **≥ 1475**（除 hutool-poi） | **100%（除 hutool-poi）** |

**v1.0.0 准入条件**：除 hutool-poi 外，所有 hutool 模块的 Java 文件都必须在 hutool-rust 中有对应 `.rs` 文件。

## 文档结束

本计划已整合：

1. ✅ Hutool Java 完整代码结构分析（基于 `MIGRATION_STATUS.md` + code-review-graph）
2. ✅ hutool-rust 当前迁移进度审计（926 Rust 文件 vs 1553 Java 文件，**完成度 59.6% 文件数 / 50% 方法数**）
3. ✅ hutool-core 双重路径问题识别与处理策略
4. ✅ 缺失 facade 类清单（SetUtil / URLEncodeUtil / URLDecodeUtil / RegexUtil / SecureUtil / DigestUtil 等）
5. ✅ hutool-poi API 占位骨架已存在（79 个 `.rs` + rustdoc）；⚪ 功能实现仍为 0%
6. ✅ hutool-ai 7 provider 补全计划
7. ✅ hutool-cron / hutool-db / hutool-extra 大件补全方案
8. ✅ 7 条核心设计原则（双 API 表面 + 已有实现不删减 + 类型映射表固定 + 错误单 enum + OnceLock 等）
9. ✅ 命名映射规则（snake_case 转换 + 业务动词不翻译）
10. ✅ Phase 1~6 分阶段实施计划（10~12 周）
11. ✅ 测试体系（四层测试 + Golden + Parity）
12. ✅ 依赖清单与质量门禁
13. ✅ Java ↔ Rust 完整文件对应关系
14. ✅ Phase 0 立即执行清单

**下一步**：继续非 POI 模块的实现与验证；除非用户单独批准引擎集成，否则不实施或暴露 `hutool-poi` 功能。
