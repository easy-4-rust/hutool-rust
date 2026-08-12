# 安全与加密实施计划：hutool-crypto / hutool-jwt / hutool-bloom-filter

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 补全 hutool-crypto 的 symmetric/asymmetric/digest 子包细化、确认 hutool-jwt 和 hutool-bloom-filter 完整。

**Architecture:** hutool-crypto 基于 RustCrypto（aes-gcm/hmac/sha2/sm2/sm3/sm4）+ argon2。hutool-jwt 基于 jsonwebtoken。hutool-bloom-filter 自研 BloomFilter + BitMap。依赖方向：hutool-crypto -> hutool-core；hutool-jwt -> hutool-crypto + hutool-json。

**Tech Stack:** `aes-gcm`, `hmac`, `sha2`, `argon2`, `sm2`, `sm3`, `sm4`, `rsa`, `jsonwebtoken`, `serde_json`。

## Global Constraints

- DES/RC4 等历史算法放 `legacy` feature（已实现）。
- RSA 时序侧信道风险需在文档中说明。
- 国密 sm2/sm3/sm4 是必须支持的能力。

---

## 1. 当前状态（代码事实）

| 指标 | 当前值 | 证据 |
|---|---|---|
| hutool-crypto Rust 文件数 | 51 | `find crates/hutool-crypto/src -name "*.rs"` |
| hutool-crypto Java 文件数 | 70 | `find hutool/hutool-crypto/src/main/java -name "*.java"` |
| hutool-crypto API 覆盖率 | 99% (735/745) | `PHASE_BASELINE.md` |
| hutool-crypto 已有子包 | asymmetric/, bc_util/, cipher_wrapper/, hutool_facade/, sm2_util/, spec_util/, symmetric_legacy/ | 目录结构 |
| hutool-jwt Rust 文件数 | 28 | 代码事实 |
| hutool-jwt Java 文件数 | 17 | 代码事实 |
| hutool-jwt API 覆盖率 | 100% (121/121) | `PHASE_BASELINE.md` |
| hutool-bloom-filter Rust 文件数 | 6 | 代码事实 |
| hutool-bloom-filter Java 文件数 | 22 | 代码事实 |
| hutool-bloom-filter API 覆盖率 | 100% (72/72) | `PHASE_BASELINE.md` |

---

## 2. 已确认需求与非目标

### 2.1 已确认需求

1. hutool-crypto 补齐剩余 10 个 API（745 - 735 = 10）。
2. hutool-jwt 确认 100% 覆盖率。
3. hutool-bloom-filter 确认 100% 覆盖率。

### 2.2 非目标

- 不替换 RustCrypto 为 ring。
- 不实现 DES/RC4 的非 legacy 版本。

---

### Task 1: hutool-crypto 补齐剩余 API

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 找出 10 个缺失 API**
- [ ] **Step 2: 逐个补齐（可能在 symmetric/asymmetric/digest 子包）**
- [ ] **Step 3: 编写测试**
- [ ] **Step 4: 提交**

### Task 2: hutool-jwt 收尾验证

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 确认 100%**
- [ ] **Step 2: 确认 JWT 签名/验证测试通过**
- [ ] **Step 3: 提交验证结果**

### Task 3: hutool-bloom-filter 收尾验证

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 确认 100%**
- [ ] **Step 2: 确认 BloomFilter 性能测试通过**
- [ ] **Step 3: 提交验证结果**

---

## 3. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| hutool-crypto >= 99% | `verify-parity.py --by-module` |
| hutool-jwt = 100% | `verify-parity.py --by-module` |
| hutool-bloom-filter = 100% | `verify-parity.py --by-module` |
| `cargo test -p hutool-crypto` | 全绿 |
| `cargo test -p hutool-jwt` | 全绿 |
