# 迁移状态基线设计规格

日期：2026-08-12
来源：合并 `docs/PHASE_BASELINE.md` + `MIGRATION_STATUS.md` + `MIGRATION_AUDIT_REPORT.md`，以代码事实核对

## 1. 背景

hutool-rust 的迁移进度需要一个权威状态基线。此前存在三份独立文档（PHASE_BASELINE、MIGRATION_STATUS、MIGRATION_AUDIT_REPORT），
数据口径不一致。本规格以 2026-08-12 代码事实为准，合并去重，提供单一权威源。

## 2. API 覆盖率基线

执行 `python3 scripts/verify-parity.py`：

```
Hutool API parity: 11587/13871 (83.53%), registered=13871/13871 (100.00%)
```

| 指标 | 当前 | 目标 | 差距 |
|---|---:|---:|---:|
| API 总体覆盖 | 83.53% | 100% | -16.47% |
| API 注册率 | 100% | 100% | done |
| feasible_covered（排除 unsafe-to-copy） | 97.55% | 100% | -2.45% |
| unportable_excluded | 2203 | -- | -- |

## 3. 测试覆盖率基线

执行 `python3 scripts/verify-test-parity.py`：

```
TEST registration: 3292/3292 (100.00%)
TEST behavioral:   3266/3292 (99.21%)
```

| 指标 | 当前 | 目标 |
|---|---:|---:|
| TEST 注册率 | 100% | 100% |
| TEST 行为覆盖 | 99.21% | 100% |
| Planned tests | 26 | -- |

## 4. 按模块状态（代码事实核对，2026-08-12）

> **已纠正**：以下数据基于 crates/ 实际文件数和 superpowers README 已有数据交叉验证。
> 旧文档中部分"完成度百分比"基于文件数而非 API 覆盖率，本表统一使用 verify-parity.py 数据。

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

## 5. hutool-core 子包覆盖

| 子包 | 覆盖 | 缺口 |
|---|---:|---:|
| util | 1237/1407 (88%) | 170 |
| lang | 678/915 (74%) | 237 |
| thread | 189/222 (85%) | 33 |
| annotation | 142/192 (74%) | 50 |
| exceptions | 89/96 (93%) | 7 |
| comparator | 80/87 (92%) | 7 |
| swing | 0/68 (0%) | 68（unsafe-to-copy） |

其他子包（io/text/map/collection/net/img/convert/bean/codec/builder/math/compress/stream/compiler/clone/getter）均 100%。

## 6. hutool-core 双重路径问题（已纠正）

旧文档声称"顶层 59 个 .rs 平铺"。代码事实核对：**当前顶层 23 个 .rs 文件**（含 lib.rs）。
已从 59 个减少到 23 个，但目标是 <=2（仅 lib.rs + error.rs），仍有 21 个实现文件需要迁移到子目录。

双重路径示例：
- `coll_util.rs`（顶层）+ `collection/coll_util.rs`（子目录）
- `file_util.rs`（顶层）+ `io/file/file_util.rs`（子目录）
- `io_util.rs`（顶层）+ `io/io_util.rs`（子目录）
- `iter_util.rs`（顶层）+ `collection/iter_util.rs`（子目录）
- `list_util.rs`（顶层）+ `collection/list_util.rs`（子目录）

## 7. 剩余工作量估算

| Phase | 已覆盖 | 剩余缺口 | 估算工作量 |
|---|---|---|---|
| Phase 1（hutool-extra） | 388/1082 (36%) | 694 API | 4~6 周 |
| Phase 2（hutool-db） | 831/1041 (80%) | 210 API | 2~3 周 |
| Phase 2（hutool-core 收尾） | 7027/7605 (92%) | 578 API | 3~4 周 |
| Phase 3（hutool-http） | 458/695 (66%) | 237 API | 2~3 周 |
| Phase 3（hutool-crypto） | 735/745 (99%) | 10 API | 0.5 周 |

总剩余工作量估算：8~12 周（不包含 hutool-poi 实现）。

## 8. hutool-poi 处置

`crates/hutool-poi` 于 2026-08-04 通过 `git rm -r` 移除。Excel/Word/OFD 能力由独立项目 `easyexcel-rust` 承接。
hutool-poi 不计入本仓库完成度。

## 9. 已知状态可信度

以下状态来自近期工作，经代码事实核对可信：
- hutool-cron：40 文件完整，行覆盖率 99.66%
- hutool-ai：28 文件，7 个 provider 全实现，API 覆盖率 100%
- hutool-extra：15 模块全整合（mail/image/qrcode/pinyin/emoji/archive 默认启用；expression/tokenizer/ftp/ssh/template 可选 feature）
- hutool-cache：per-key striped locks 并发修复，API 覆盖率 100%
- hutool-core：892 文件含双重遗留路径，需重组

## 10. 验收方式

任何 Phase 完成时，运行以下命令验证：

```bash
python3 scripts/verify-parity.py
python3 scripts/verify-parity.py --by-module
python3 scripts/verify-parity.py --feasible
python3 scripts/verify-test-parity.py
cargo test --workspace
```

Phase 6 完成条件：`feasible_covered == 100%` 且 `cargo test --workspace` 全绿。

## 11. 参考

- 原始文档：`docs/PHASE_BASELINE.md`、`MIGRATION_STATUS.md`、`MIGRATION_AUDIT_REPORT.md`（已删除，内容已合并至此）
- `parity/` 目录下的测试资产
- `scripts/verify-parity.py` 和 `scripts/verify-test-parity.py`
