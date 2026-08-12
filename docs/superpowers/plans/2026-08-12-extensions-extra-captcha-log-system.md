# 扩展能力实施计划：hutool-extra / hutool-captcha / hutool-log / hutool-system

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking。

**Goal:** 补全 hutool-extra 的 15 个模块（mail/image/qrcode/pinyin/emoji/archive/expression/tokenizer/ftp/ssh/template 等），确认 hutool-captcha/hutool-log/hutool-system 完整。

**Architecture:** hutool-extra 是扩展能力集合，每个模块 feature-gated。hutool-captcha 基于 image crate 生成验证码。hutool-log 基于 tracing。hutool-system 基于 sysinfo。依赖方向：hutool-extra -> hutool-core；hutool-log -> hutool-core。

**Tech Stack:** `lettre`（邮件）, `image`（图片）, `qrcode`（QR 码）, `pinyin`（拼音）, `emojis`（表情）, `zip`（压缩）, `jieba-rs`（分词）, `suppaftp`（FTP）, `ssh2`（SSH）, `minijinja`（模板）, `rhai`（表达式）, `tracing`（日志）, `sysinfo`（系统信息）。

## Global Constraints

- hutool-extra 的模块全部 feature-gated，不默认全启。
- hutool-log 使用 tracing 统一抽象。
- hutool-system 使用 sysinfo 获取 OS/进程信息。

---

## 1. 当前状态（代码事实）

| 指标 | 当前值 | 证据 |
|---|---|---|
| hutool-extra Rust 文件数 | 92 | `find crates/hutool-extra/src -name "*.rs"` |
| hutool-extra Java 文件数 | 179 | `find hutool/hutool-extra/src/main/java -name "*.java"` |
| hutool-extra API 覆盖率 | 36% (388/1082) | `PHASE_BASELINE.md` |
| hutool-extra 已有模块 | archive/, emoji/, expression/, ftp/, image/, mail/, mail_facade/, pinyin/, qrcode.rs, spring/, ssh/, template/, tokenizer/, validation/ | 目录结构 |
| hutool-extra features | default=[archive,qrcode,emoji,pinyin]; opt-in: tokenizer,ftp,ssh,template,image,mail | Cargo.toml |
| hutool-captcha Rust 文件数 | 19 | 代码事实 |
| hutool-captcha API 覆盖率 | 100% (87/87) | `PHASE_BASELINE.md` |
| hutool-log Rust 文件数 | 29 | 代码事实 |
| hutool-log API 覆盖率 | 100% (283/283) | `PHASE_BASELINE.md` |
| hutool-system Rust 文件数 | 25 | 代码事实 |
| hutool-system API 覆盖率 | 100% (189/189) | `PHASE_BASELINE.md` |

---

## 2. 已确认需求与非目标

### 2.1 已确认需求

1. hutool-extra 是最大缺口（36% API），需按模块逐步补全。
2. 15 个模块已整合（mail/image/qrcode/pinyin/emoji/archive 默认启用；expression/tokenizer/ftp/ssh/template 可选 feature）。
3. hutool-captcha/log/system 确认 100% 覆盖率。

### 2.2 非目标

- 不实现 `cglib/*`（Serde 替代）。
- 不实现 `spring/*`（无 Spring）。
- 不实现 `servlet/*`（无 Servlet 容器）。

---

### Task 1: hutool-extra expression 引擎补全

**当前状态:** `expression/` 目录已存在，需确认 rhai 引擎集成。

- [ ] **Step 1: 对比 Java `expression/*`（Aviator/JEXL/Mvel/QLExpress/Rhino/SpEL）**
- [ ] **Step 2: 实现 rhai-based 表达式引擎（对齐 Java JSR-223）**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 2: hutool-extra tokenizer 引擎补全

**当前状态:** `tokenizer/` 目录已存在，feature-gated（jieba-rs）。

- [ ] **Step 1: 确认 jieba-rs 集成完整**
- [ ] **Step 2: 补齐缺失的分词 API**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 3: hutool-extra ftp/ssh 模块补全

**当前状态:** `ftp/` 和 `ssh/` 目录已存在，feature-gated。

- [ ] **Step 1: 确认 suppaftp（FTP）和 ssh2（SSH）集成**
- [ ] **Step 2: 补齐缺失的 FTP/SSH API**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 4: hutool-extra template 引擎补全

**当前状态:** `template/` 目录已存在，feature-gated（minijinja）。

- [ ] **Step 1: 确认 minijinja 集成完整**
- [ ] **Step 2: 补齐模板 API**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 5: hutool-extra mail 模块补全

**当前状态:** `mail/` 和 `mail_facade/` 目录已存在，feature-gated（lettre）。

- [ ] **Step 1: 确认 lettre 集成完整**
- [ ] **Step 2: 补齐 MailUtil/MailAccount API**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 6: hutool-extra 剩余模块补全

**范围:** spring/, validation/, image/ 等。

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 找出全部缺失 API**
- [ ] **Step 2: 按优先级逐个补齐**
- [ ] **Step 3: 测试**
- [ ] **Step 4: 提交**

### Task 7: hutool-captcha/log/system 收尾验证

- [ ] **Step 1: 运行 `verify-parity.py --by-module` 确认三者 100%**
- [ ] **Step 2: 运行全量测试**
- [ ] **Step 3: 提交验证结果**

---

## 3. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| hutool-extra >= 60% | `verify-parity.py --by-module` |
| hutool-captcha = 100% | `verify-parity.py --by-module` |
| hutool-log = 100% | `verify-parity.py --by-module` |
| hutool-system = 100% | `verify-parity.py --by-module` |
| `cargo test -p hutool-extra --features full` | 全绿 |
