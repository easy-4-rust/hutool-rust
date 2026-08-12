# 数据与存储实施计划：hutool-db / hutool-cache / hutool-json / hutool-setting

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 补全 hutool-db 的 Entity/Query/SqlBuilder/dialect/nosql 子包、确认 hutool-cache 并发语义完整、补齐 hutool-json 和 hutool-setting 剩余 API，使数据与存储层达到 100% API 覆盖率。

**Architecture:** hutool-db 基于 sqlx，提供 Entity/Query/SqlBuilder/Dialect/Handler 体系。hutool-cache 基于 moka，提供 LRU/LFU/Timed/Weak/FIFO 缓存。hutool-json 基于 serde_json。hutool-setting 基于 config。依赖方向：hutool-db -> hutool-core + hutool-json；hutool-cache -> hutool-core。

**Tech Stack:** `sqlx`（async DB）, `moka`（缓存）, `serde_json`（JSON）, `config`（配置）, `serde_yaml_ng`（YAML，待启用）。

## Global Constraints

- hutool-db 不造 ORM，使用 sqlx 直接构建。
- hutool-cache 的并发语义必须对齐 Java `AbstractCache` 的 striped lock。
- hutool-json 的 API 命名严格对齐 Java `JSONUtil` / `JSONObject` / `JSONArray`。
- hutool-setting 必须支持 YAML（`serde_yaml_ng` 待启用）。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. hutool-db 补齐 Entity（动态实体）、Query（查询构建）、SqlBuilder、Condition 体系。
2. hutool-db 补齐 Dialect 体系（MySQL/PostgreSQL/SQLite/Oracle/SQLServer 等）。
3. hutool-db 补齐 handler/meta/nosql 子包。
4. hutool-json 补齐剩余 27 个 API（当前 294/294 = 100%，需确认）。
5. hutool-setting 启用 YAML 支持。
6. hutool-cache 确认 per-key striped locks 已实现。

### 1.2 非目标

- 不引入 `sea-orm` / `diesel` 等 ORM。
- 不在 hutool-db 中实现连接池管理（委托 sqlx::Pool）。
- 不为 NoSQL（MongoDB/Redis）实现完整客户端（仅 facade 占位）。

---

## 2. 当前状态（代码事实）

| 指标 | 当前值 | 证据 |
|---|---|---|
| hutool-db Rust 文件数 | 79 | `find crates/hutool-db/src -name "*.rs"` |
| hutool-db Java 文件数 | 107 | `find hutool/hutool-db/src/main/java -name "*.java"` |
| hutool-db 已实现子包 | db/, dialect/, ds/, handler/, meta_types/, nosql/, runner/, sql/, dao_template/ | 目录结构 |
| hutool-cache Rust 文件数 | 28 | 代码事实 |
| hutool-cache Java 文件数 | 22 | 代码事实 |
| hutool-cache 并发语义 | per-key striped locks 已实现 | `grep "striped" crates/hutool-cache/src/compat.rs` |
| hutool-json Rust 文件数 | 26 | 代码事实 |
| hutool-json API 覆盖率 | 100% (294/294) | `PHASE_BASELINE.md` |
| hutool-setting Rust 文件数 | 14 | 代码事实 |
| hutool-setting API 覆盖率 | 100% (225/225) | `PHASE_BASELINE.md` |

---

## 3. 文件职责总览

| 文件/目录 | 职责 |
|---|---|
| `hutool-db/src/entity.rs` | Entity 动态实体 |
| `hutool-db/src/sql/query.rs` | Query 查询构建 |
| `hutool-db/src/sql/sql_builder/` | SqlBuilder |
| `hutool-db/src/sql/condition/` | Condition 体系 |
| `hutool-db/src/dialect/` | Dialect 方言 |
| `hutool-db/src/handler/` | ResultSet Handler |
| `hutool-db/src/meta_types/` | 元数据类型 |
| `hutool-db/src/nosql/` | NoSQL facade |
| `hutool-cache/src/` | 缓存实现 |
| `hutool-json/src/` | JSON 工具 |
| `hutool-setting/src/` | 配置工具 |

---

### Task 1: hutool-db Entity 完善

**当前状态:** `entity.rs` 已存在，需确认与 Java `Entity` 类的 API 对齐度。

- [ ] **Step 1: 对比 Java `Entity.java` 与 Rust `entity.rs` 的方法清单**
- [ ] **Step 2: 补齐缺失方法（set/get/remove/size/isEmpty/entrySet 等）**
- [ ] **Step 3: 编写单元测试**
- [ ] **Step 4: 提交**

### Task 2: hutool-db Query / Condition 体系完善

**当前状态:** `sql/query.rs` 和 `sql/condition/` 已存在。

- [ ] **Step 1: 对比 Java `Query.java` / `Condition.java` / `ConditionBuilder.java`**
- [ ] **Step 2: 补齐缺失的条件操作（and/or/in/like/between/gt/lt 等）**
- [ ] **Step 3: 补齐 `condition_builder.rs`**
- [ ] **Step 4: 测试**
- [ ] **Step 5: 提交**

### Task 3: hutool-db Dialect 体系完善

**当前状态:** `dialect/` 已有 factory/impls/name 等文件。

- [ ] **Step 1: 确认 MySQL/PostgreSQL/SQLite 方言已实现**
- [ ] **Step 2: 补齐 Oracle/SQLServer/H2 方言（如需）**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 4: hutool-db Handler 子包完善

**当前状态:** `handler/` 已有 bean_handler/bean_list_handler/entity_handler 等 12 个文件。

- [ ] **Step 1: 对比 Java handler 子包**
- [ ] **Step 2: 补齐缺失 handler**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 5: hutool-db Meta / NoSQL 子包

**当前状态:** `meta_types/` 和 `nosql/` 已存在。

- [ ] **Step 1: 确认 meta_types 与 Java 对齐**
- [ ] **Step 2: 确认 nosql/mongo_ds.rs 和 nosql/redis_ds.rs 为 facade 占位**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 6: hutool-setting 启用 YAML

- [ ] **Step 1: 在 `hutool-setting/Cargo.toml` 添加 `serde_yaml_ng` 依赖**
- [ ] **Step 2: 实现 YAML 解析支持**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 7: hutool-json 和 hutool-cache 收尾验证

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 确认 json/cache 100%**
- [ ] **Step 2: 确认 hutool-cache per-key striped locks 语义完整**
- [ ] **Step 3: 运行全量测试**
- [ ] **Step 4: 提交**

---

## 4. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| hutool-db API >= 90% | `verify-parity.py --by-module` |
| hutool-cache API = 100% | `verify-parity.py` |
| hutool-json API = 100% | `verify-parity.py` |
| hutool-setting API = 100% + YAML | `verify-parity.py` + YAML 测试 |
| `cargo test -p hutool-db` | 全绿 |
| `cargo test -p hutool-cache` | 全绿 |
