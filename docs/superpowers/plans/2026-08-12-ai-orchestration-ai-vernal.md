# AI 与编排实施计划：hutool-ai / hutool-vernal

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking。

**Goal:** 确认 hutool-ai 的 7 个 provider 全部实现、确认 hutool-vernal 完整。

**Architecture:** hutool-ai 基于 reqwest + SSE（eventsource-stream），提供统一 AI Provider 抽象。hutool-vernal 是 Rust 独有扩展（spring-equinox 农历）。依赖方向：hutool-ai -> hutool-http + hutool-json + hutool-core。

**Tech Stack:** `reqwest`（HTTP）, `eventsource-stream`（SSE）, `serde_json`（JSON）, `chrono`（日期）。

## Global Constraints

- hutool-ai 的 provider 必须对齐 Java 7 个 provider（OpenAI/DeepSeek/Doubao/Gemini/Grok/Hutool/Ollama）。
- hutool-vernal 是 Rust 独有扩展，无 Java 对应。

---

## 1. 当前状态（代码事实）

| 指标 | 当前值 | 证据 |
|---|---|---|
| hutool-ai Rust 文件数 | 28 | `find crates/hutool-ai/src -name "*.rs"` |
| hutool-ai Java 文件数 | 58 | `find hutool/hutool-ai/src/main/java -name "*.java"` |
| hutool-ai API 覆盖率 | 100% (281/281) | `PHASE_BASELINE.md` |
| hutool-ai providers | openai, deepseek, doubao, gemini, grok, hutool, ollama | `find crates/hutool-ai/src/providers -name "*.rs"` |
| hutool-vernal Rust 文件数 | 8 | 代码事实 |
| hutool-vernal Java 文件数 | 0（Rust 独有） | 无 Java 对应 |

---

## 2. 已确认需求与非目标

### 2.1 已确认需求

1. hutool-ai 7 个 provider 全部实现（openai/deepseek/doubao/gemini/grok/hutool/ollama）。
2. hutool-ai API 覆盖率 100%。
3. hutool-vernal 为 Rust 独有扩展，确认功能完整。

### 2.2 非目标

- 不在 hutool-ai 中实现模型训练。
- 不为 hutool-vernal 添加 Java 对应。

---

### Task 1: hutool-ai 状态确认（DONE）

- [x] **Step 1: 确认 7 个 provider 全部实现**（openai/deepseek/doubao/gemini/grok/hutool/ollama）
- [x] **Step 2: 确认 API 覆盖率 100% (281/281)**
- [x] **Step 3: 确认 28 个 Rust 文件覆盖 Java 58 个文件的 API**

### Task 2: hutool-vernal 收尾验证

- [ ] **Step 1: 确认 hutool-vernal 功能完整（spring-equinox 农历）**
- [ ] **Step 2: 测试**
- [ ] **Step 3: 提交验证结果**

### Task 3: hutool-ai SSE 增强（可选）

- [ ] **Step 1: 确认 `eventsource-stream` 已集成**
- [ ] **Step 2: 确认流式响应解析正确**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

---

## 3. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| hutool-ai DONE | 100% API, 7 provider, 28 文件 |
| hutool-vernal 完整 | 功能测试通过 |
| `cargo test -p hutool-ai` | 全绿 |
| `cargo test -p hutool-vernal` | 全绿 |
