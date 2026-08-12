# 横切关注点实施计划：hutool-observability / hutool-compat-hutool

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking。

**Goal:** 确认 hutool-observability（Rust 独有）完整，记录 hutool-compat-hutool 冻结决策。

**Architecture:** hutool-observability 是 Rust 独有的可观测性模块（tracing + metrics）。hutool-compat-hutool 是早期设想的 Hutool 风格门面，已被用户明确否决，当前冻结。

**Tech Stack:** `tracing`（日志）, `tracing-subscriber`（日志订阅）, `metrics`（指标）。

## Global Constraints

- hutool-observability 是 Rust 独有扩展，无 Java 对应。
- hutool-compat-hutool 冻结维持现状（114 行，不扩展、不删除、不新增依赖）。

---

## 1. 当前状态（代码事实）

| 指标 | 当前值 | 证据 |
|---|---|---|
| hutool-observability Rust 文件数 | 34 | `find crates/hutool-observability/src -name "*.rs"` |
| hutool-observability Java 文件数 | 0（Rust 独有） | 无 Java 对应 |
| hutool-compat-hutool Rust 文件数 | 1 | `find crates/hutool-compat-hutool/src -name "*.rs"` |
| hutool-compat-hutool 行数 | 114 | `IMPLEMENTATION_PLAN.md` |

---

## 2. 已确认需求与非目标

### 2.1 已确认需求

1. hutool-observability 提供 tracing + metrics 集成。
2. hutool-compat-hutool 冻结决策记录。

### 2.2 非目标

- 不扩展 hutool-compat-hutool。
- 不在 hutool-observability 中实现完整 APM。

---

### Task 1: hutool-observability 收尾验证

- [ ] **Step 1: 确认 hutool-observability 功能完整**
- [ ] **Step 2: 确认 tracing + metrics 集成正确**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交验证结果**

### Task 2: hutool-compat-hutool 冻结决策记录

- [x] **Step 1: 记录冻结决策**（用户明确否决双 API 表面方案）
- [x] **Step 2: 确认当前 114 行 / 1 文件维持现状**
- [x] **Step 3: 不扩展、不删除、不新增依赖**

---

## 3. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| hutool-observability 功能完整 | 测试通过 |
| hutool-compat-hutool 冻结 | 1 文件 / 114 行不变 |
| `cargo test -p hutool-observability` | 全绿 |
