# 网络与 IO 实施计划：hutool-http / hutool-socket

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 补全 hutool-http 的 server/SSL/soap 子包（已部分实现）、补齐 hutool-socket 剩余 API。

**Architecture:** hutool-http 基于 reqwest，提供客户端 + 简易服务器。hutool-socket 基于 tokio，提供 TCP/UDP。依赖方向：hutool-http -> hutool-core + hutool-json；hutool-socket -> hutool-core。

**Tech Stack:** `reqwest`（HTTP 客户端）, `hyper`（HTTP 服务器）, `tokio`（异步 IO）, `native-tls` / `rustls`（TLS）。

## Global Constraints

- hutool-http 是客户端优先，Server 模块为可选能力。
- hutool-socket 使用 tokio async。
- 命名严格对齐 Java `HttpUtil` / `HttpServer` / `SimpleServer`。

---

## 1. 当前状态（代码事实）

| 指标 | 当前值 | 证据 |
|---|---|---|
| hutool-http Rust 文件数 | 87 | `find crates/hutool-http/src -name "*.rs"` |
| hutool-http Java 文件数 | 72 | `find hutool/hutool-http/src/main/java -name "*.java"` |
| hutool-http API 覆盖率 | 66% (458/695) | `PHASE_BASELINE.md` |
| hutool-http 已有子包 | base/, body/, config/, cookie/, html/, http_connection/, http_util/, progress/, response/, server/, ssl/, useragent/, webservice/ | 目录结构 |
| hutool-socket Rust 文件数 | 26 | 代码事实 |
| hutool-socket Java 文件数 | 24 | 代码事实 |
| hutool-socket API 覆盖率 | 100% (102/102) | `PHASE_BASELINE.md` |

---

## 2. 已确认需求与非目标

### 2.1 已确认需求

1. hutool-http 补齐剩余 237 个 API（695 - 458 = 237）。
2. server/ 子包已有 SimpleServer 实现，需补全。
3. ssl/ 子包已有自定义实现。
4. webservice/ 子包已有 SOAP 客户端。
5. hutool-socket 确认 100% 覆盖率。

### 2.2 非目标

- 不在 hutool-http 内嵌 axum 作为 Web 服务器框架。
- 不实现 Java Servlet 容器。

---

### Task 1: hutool-http HttpUtil 静态 facade 补全

**当前状态:** `http_util/http_util.rs` 已存在，需补齐与 Java `HttpUtil` 对齐的静态方法。

- [ ] **Step 1: 对比 Java `HttpUtil.java` 与 Rust `http_util.rs` 方法清单**
- [ ] **Step 2: 补齐缺失方法（get/post/put/delete/createGet/createPost 等）**
- [ ] **Step 3: 编写测试**
- [ ] **Step 4: 提交**

### Task 2: hutool-http Server 子包完善

**当前状态:** `server/` 已有 simple_server.rs、filter_chain.rs、http_server_base.rs 等 11 个文件。

- [ ] **Step 1: 对比 Java `server/*` 子包**
- [ ] **Step 2: 补齐缺失的 Server API（路由注册、中间件、静态文件等）**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 3: hutool-http SSL 子包完善

**当前状态:** `ssl/` 已有 5 个文件。

- [ ] **Step 1: 确认 SSL/TLS 配置 API 对齐**
- [ ] **Step 2: 补齐缺失的 SSL 工厂方法**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 4: hutool-http WebService/SOAP 子包完善

**当前状态:** `webservice/` 已有 8 个文件（soap_client/soap_util/jakarta_soap_*）。

- [ ] **Step 1: 确认 SOAP 客户端 API 对齐**
- [ ] **Step 2: 补齐缺失方法**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 5: hutool-http 剩余 API 补齐

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 找出全部缺失 API**
- [ ] **Step 2: 按优先级逐个补齐**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 6: hutool-socket 收尾验证

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 确认 100%**
- [ ] **Step 2: 确认 TCP/UDP 测试通过**
- [ ] **Step 3: 提交验证结果**

---

## 3. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| hutool-http >= 80% | `verify-parity.py --by-module` |
| hutool-socket = 100% | `verify-parity.py --by-module` |
| `cargo test -p hutool-http` | 全绿 |
| `cargo test -p hutool-socket` | 全绿 |
