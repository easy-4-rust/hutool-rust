# 计算与调度实施计划：hutool-cron / hutool-script / hutool-dfa

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 确认 hutool-cron 已完成（标注 DONE），补全 hutool-script 和 hutool-dfa 剩余 API。

**Architecture:** hutool-cron 自研 cron 表达式解析 + TimingWheel 调度。hutool-script 基于 rhai 沙箱脚本。hutool-dfa 基于 Aho-Corasick（敏感词/关键词匹配）。

**Tech Stack:** `cron`（cron 表达式解析）, `tokio`（异步调度）, `rhai`（脚本引擎）, `aho-corasick`（DFA）。

## Global Constraints

- hutool-cron 已完成，不再修改（仅验证）。
- hutool-script 使用 rhai 作为脚本引擎（对齐 Java JSR-223）。
- hutool-dfa 使用 aho-corasick（对齐 Java DFA 算法）。

---

## 1. 当前状态（代码事实）

| 指标 | 当前值 | 证据 |
|---|---|---|
| hutool-cron Rust 文件数 | 40 | `find crates/hutool-cron/src -name "*.rs"` |
| hutool-cron Java 文件数 | 41 | `find hutool/hutool-cron/src/main/java -name "*.java"` |
| hutool-cron API 覆盖率 | 100% (208/208) | `PHASE_BASELINE.md` |
| hutool-cron 子包 | pattern/, listener/, task/, timingwheel/ | 目录结构确认 |
| hutool-script Rust 文件数 | 8 | 代码事实 |
| hutool-script Java 文件数 | 5 | 代码事实 |
| hutool-script API 覆盖率 | 100% (79/79) | `PHASE_BASELINE.md` |
| hutool-dfa Rust 文件数 | 14 | 代码事实 |
| hutool-dfa Java 文件数 | 6 | 代码事实 |
| hutool-dfa API 覆盖率 | 100% (43/43) | `PHASE_BASELINE.md` |

---

## 2. 已确认需求与非目标

### 2.1 已确认需求

1. hutool-cron 状态标注为 DONE（100% API 覆盖率，40 文件完整）。
2. hutool-script 和 hutool-dfa 确认 100% 覆盖率无遗漏。

### 2.2 非目标

- 不为 hutool-cron 添加新功能。
- 不替换 hutool-script 的 rhai 引擎。

---

### Task 1: hutool-cron 状态确认（DONE）

- [x] **Step 1: 确认 40 个 Rust 文件覆盖 Java 41 个文件**（Rust 文件数 >= Java 文件数 - 1，允许合并）
- [x] **Step 2: 确认 API 覆盖率 100% (208/208)**
- [x] **Step 3: 确认子包完整（pattern/timingwheel/task/listener）**
- [x] **Step 4: 确认行覆盖率 99.66%**

### Task 2: hutool-script 收尾验证

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 确认 100%**
- [ ] **Step 2: 确认 rhai 引擎集成测试通过**
- [ ] **Step 3: 提交验证结果**

### Task 3: hutool-dfa 收尾验证

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 确认 100%**
- [ ] **Step 2: 确认 Aho-Corasick 集成测试通过**
- [ ] **Step 3: 提交验证结果**

---

## 3. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| hutool-cron DONE | 100% API, 40 文件, 99.66% 行覆盖率 |
| hutool-script 100% | `verify-parity.py --by-module` |
| hutool-dfa 100% | `verify-parity.py --by-module` |
