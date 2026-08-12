# hutool-rust Superpowers 规格驱动开发（SDD）体系

> **建立日期**：2026-08-12
> **基线**：Hutool Java v5.8.x + hutool-rust workspace 当前 HEAD
> **方法论**：Superpowers SDD（Spec-Driven Development）

---

## 概述

本目录是 hutool-rust 项目的规格驱动开发中心。所有迁移工作的设计依据（specs/）和执行清单（plans/）均在此管理。

### 如何使用

- **specs/** 是设计依据：命名规则、依赖策略、测试策略、架构总览、迁移状态等横切关注点的权威规格。
- **plans/** 是执行清单：按模块分组的实施计划，每个计划含 checkbox 跟踪的分阶段任务。
- **状态基于代码事实**：每处 DONE 标注引用具体 crate 路径或文件数或覆盖率；TODO 引用 Java 侧缺失文件清单。

---

## 设计规格（specs/）

| 日期 | 文件 | 主题 | 状态 |
|---|---|---|---|
| 2026-08-12 | [java-rust-naming-mapping-rules.md](specs/2026-08-12-java-rust-naming-mapping-rules.md) | Java -> Rust 命名映射规则 | 设计完成 |
| 2026-08-12 | [dependency-governance-strategy.md](specs/2026-08-12-dependency-governance-strategy.md) | 依赖治理策略（锁基线、feature 裁剪、组件选型） | 设计完成 |
| 2026-08-12 | [testing-coverage-strategy.md](specs/2026-08-12-testing-coverage-strategy.md) | 测试与覆盖率策略（llvm-cov、parity、CI） | 设计完成 |
| 2026-08-12 | [hutool-rust-Architecture.md](../hutool-rust-Architecture.md) / [.zh_CN.md](../hutool-rust-Architecture.zh_CN.md) | 项目架构契约（分层/crate/依赖/安全/演进），独立于 superpowers 的权威架构文档 | 已批准 |
| 2026-08-12 | [migration-status-baseline.md](specs/2026-08-12-migration-status-baseline.md) | 迁移状态基线（API 覆盖率、测试覆盖率、按模块状态） | 设计完成 |
| 2026-08-12 | [feature-matrix.md](specs/2026-08-12-feature-matrix.md) | Feature 矩阵（编译成本/平台/安全影响） | 设计完成 |
| 2026-08-12 | [production-readiness.md](specs/2026-08-12-production-readiness.md) | 生产就绪审计（架构/API/运行时/安全/质量） | 设计完成 |
| 2026-08-12 | [security-baseline.md](specs/2026-08-12-security-baseline.md) | 安全基线（默认设置、调用方责任、遗留算法） | 设计完成 |
| 2026-08-12 | [observability-strategy.md](specs/2026-08-12-observability-strategy.md) | 可观测性策略（tracing/metrics/health/诊断后端） | 设计完成 |
| 2026-08-12 | [provenance-and-attribution.md](specs/2026-08-12-provenance-and-attribution.md) | 来源与归属（yimi-rutool、Hutool Java） | 设计完成 |
| 2026-08-12 | [xml-support.md](specs/2026-08-12-xml-support.md) | XML 支持（流式/DOM/Serde、安全默认） | 设计完成 |
| 2026-08-12 | [java-rust-parity-methodology.md](specs/2026-08-12-java-rust-parity-methodology.md) | Java-Rust 对等方法论（能力账本、unportable 矩阵） | 设计完成 |
| 2026-08-12 | [method-level-comparison.md](specs/2026-08-12-method-level-comparison.md) | 方法级对照聚合（StrUtil/DateUtil/CollUtil/HTTP/Setting） | 设计完成 |
| 2026-08-12 | [object-naming-audit.md](specs/2026-08-12-object-naming-audit.md) | 对象名称一致性审计聚合（21 模块） | 设计完成 |
| 2026-08-12 | [object-and-semantic-mapping.md](specs/2026-08-12-object-and-semantic-mapping.md) | 对象级与语义迁移对照聚合（21 模块 x2） | 设计完成 |

---

## 实施计划（plans/）

### 模块覆盖总表

| 模块分组 | plan 文件 | 覆盖模块 | 状态 |
|---|---|---|---|
| 核心基础 | [core-foundations-macro-annotation-aop.md](plans/2026-08-12-core-foundations-macro-annotation-aop.md) | hutool-core, hutool-macro, hutool-annotation, hutool-aop | 进行中 |
| 数据与存储 | [data-storage-db-cache-json-setting.md](plans/2026-08-12-data-storage-db-cache-json-setting.md) | hutool-db, hutool-cache, hutool-json, hutool-setting | 进行中 |
| 计算与调度 | [computation-scheduling-cron-script-dfa.md](plans/2026-08-12-computation-scheduling-cron-script-dfa.md) | hutool-cron, hutool-script, hutool-dfa | hutool-cron DONE |
| 安全与加密 | [security-crypto-jwt-bloom-filter.md](plans/2026-08-12-security-crypto-jwt-bloom-filter.md) | hutool-crypto, hutool-jwt, hutool-bloom-filter | 进行中 |
| 网络与 IO | [network-io-http-socket.md](plans/2026-08-12-network-io-http-socket.md) | hutool-http, hutool-socket | 进行中 |
| 扩展能力 | [extensions-extra-captcha-log-system.md](plans/2026-08-12-extensions-extra-captcha-log-system.md) | hutool-extra, hutool-captcha, hutool-log, hutool-system | 进行中 |
| AI 与编排 | [ai-orchestration-ai-vernal.md](plans/2026-08-12-ai-orchestration-ai-vernal.md) | hutool-ai, hutool-vernal | hutool-ai DONE |
| 横切关注点 | [cross-cutting-observability-compat.md](plans/2026-08-12-cross-cutting-observability-compat.md) | hutool-observability, hutool-compat-hutool | 冻结 |

### 按模块状态明细

| 模块 | Rust 文件 | Java 文件 | API 覆盖率 | 状态 | 所属 plan |
|---|---:|---:|---:|---|---|
| hutool-core | 892 | 713 | 92% | 需重组目录 + 补 facade | 核心基础 |
| hutool-macro | 86 | 0 | N/A（Rust 独有） | 需补 proc-macro | 核心基础 |
| hutool-annotation | 17 | 36 | 74% | 需补 50 API | 核心基础 |
| hutool-aop | 21 | 15 | 100% | DONE（验证中） | 核心基础 |
| hutool-db | 79 | 107 | 80% | 需补 Entity/Query/Dialect | 数据与存储 |
| hutool-cache | 28 | 22 | 100% | DONE（并发语义已确认） | 数据与存储 |
| hutool-json | 26 | 33 | 100% | DONE | 数据与存储 |
| hutool-setting | 14 | 16 | 100% | 需启用 YAML | 数据与存储 |
| hutool-cron | 40 | 41 | 100% | DONE（99.66% 行覆盖率） | 计算与调度 |
| hutool-script | 8 | 5 | 100% | DONE | 计算与调度 |
| hutool-dfa | 14 | 6 | 100% | DONE | 计算与调度 |
| hutool-crypto | 51 | 70 | 99% | 需补 10 API | 安全与加密 |
| hutool-jwt | 28 | 17 | 100% | DONE | 安全与加密 |
| hutool-bloom-filter | 6 | 22 | 100% | DONE | 安全与加密 |
| hutool-http | 87 | 72 | 66% | 需补 237 API | 网络与 IO |
| hutool-socket | 26 | 24 | 100% | DONE | 网络与 IO |
| hutool-extra | 92 | 179 | 36% | 需补 694 API（最大缺口） | 扩展能力 |
| hutool-captcha | 19 | 13 | 100% | DONE | 扩展能力 |
| hutool-log | 29 | 46 | 100% | DONE | 扩展能力 |
| hutool-system | 25 | 16 | 100% | DONE | 扩展能力 |
| hutool-ai | 28 | 58 | 100% | DONE（7 provider） | AI 与编排 |
| hutool-vernal | 8 | 0 | N/A（Rust 独有） | DONE | AI 与编排 |
| hutool-observability | 34 | 0 | N/A（Rust 独有） | DONE | 横切关注点 |
| hutool-compat-hutool | 1 | 0 | N/A（冻结） | 冻结 | 横切关注点 |

### 状态图例

- DONE：API 覆盖率 100% 或有代码事实确认完成
- 进行中：有已知缺口，需继续迁移
- 冻结：用户明确决策不扩展

---

## 设计决策记录

### hutool-poi 已移除

`crates/hutool-poi` 于 2026-08-04 通过 `git rm -r` 移除。Excel/Word/OFD 能力由独立项目 `easyexcel-rust` 承接。hutool-poi 不计入本仓库完成度。

### hutool-compat-hutool 冻结

经过对当前实现的诚实审计（仅 114 行 / 2 个 facade / 11 个方法），早期设想的"双 API 表面（idiomatic + hutool-compat）"架构目标已被用户明确否决。当前优先级是把主仓核心模块先迁移到位。hutool-compat-hutool 在主仓迁移完成之前没有任何迁移价值。

### 已知状态可信度

以下状态来自近期工作，经代码事实核对可信：
- hutool-cron：40 文件完整，行覆盖率 99.66%
- hutool-ai：28 文件，7 个 provider 全实现，API 覆盖率 100%
- hutool-extra：15 模块全整合（mail/image/qrcode/pinyin/emoji/archive 默认启用；expression/tokenizer/ftp/ssh/template 可选 feature）
- hutool-cache：并发语义修复（per-key striped locks），API 覆盖率 100%
- hutool-core：892 文件含双重遗留路径，需重组

---

## 收敛记录（2026-08-12）

本轮完成 docs/ 和根目录下所有计划/规范/对照类文档向 superpowers specs/plans 的转化：

- **A 类（docs/ 顶层规范）**：11 个文档 -> 11 个 specs（architecture 合并中英两份）
- **B 类（根目录迁移产物）**：6 个文档 -> 2 个 specs（migration-status-baseline 合并 3 个、method-level-comparison 合并 4 个）
- **C 类（模块子目录）**：84 个文档 -> 2 个聚合 specs（object-naming-audit 聚合 21 个、object-and-semantic-mapping 聚合 42 个）
- **删除旧文件**：docs/ 下除 superpowers/ 外全部删除，根目录 6 个迁移产物 .md 删除
- **状态核对**：以 crates/ 实际代码为准，纠正旧文档中过时数据

### 最终目录结构

```
docs/
└── superpowers/
    ├── README.md                    # 本文件
    ├── specs/                       # 15 个设计规格
    └── plans/                       # 8 个实施计划
```
