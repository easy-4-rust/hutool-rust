# hutool-rust 迁移基线（Baseline）

> **生成时间**：2026-07-21
> **基线来源**：`scripts/verify-parity.py` + `scripts/verify-test-parity.py`
> **目的**：记录 IMPLEMENTATION_PLAN.md 实施启动时的真实基线状态，让后续 Phase 工作有量化目标。

---

## 一、API 覆盖率基线

执行 `python3 scripts/verify-parity.py`：

```
Hutool API parity: 11587/13871 (83.53%), registered=13871/13871 (100.00%), missing=0, invalid_decisions=0
```

| 指标 | 当前 | 目标 | 差距 |
|---|---:|---:|---:|
| **API 总体覆盖** | 83.53% | 100% | -16.47% |
| **API 注册率**（每个 hutool API 都有 decision row） | **100%** | 100% | ✅ 已达成 |
| **Missing API**（未记录决策） | **0** | 0 | ✅ 已达成 |
| **Invalid decisions** | **0** | 0 | ✅ 已达成 |
| **feasible_covered**（排除 unsafe-to-copy 后） | 97.55% | 100% | -2.45% |
| **unportable_excluded**（标记为 unsafe-to-copy 的 API 数） | 2 203 | — | — |

---

## 二、测试覆盖率基线

执行 `python3 scripts/verify-test-parity.py`：

```
Hutool TEST registration: 3292/3292 (100.00%), missing=0, invalid_or_orphan=0
Hutool TEST behavioral:   3266/3292 (99.21%), ignored_stubs=0, planned=26
```

| 指标 | 当前 | 目标 | 差距 |
|---|---:|---:|---:|
| **TEST 注册率**（每个 hutool 测试方法都有 Rust 对应） | **100%** | 100% | ✅ 已达成 |
| **TEST 行为覆盖**（runnable covered asserts） | 99.21% | 100% | -0.79% |
| **Planned tests**（接受延期的） | 26 | — | — |
| **Missing tests**（未注册） | **0** | 0 | ✅ 已达成 |
| **Invalid or orphan** | **0** | 0 | ✅ 已达成 |

---

## 三、按 hutool 模块细分（API 覆盖率）

| 模块 | 覆盖 | 状态 |
|---|---|---|
| `hutool-ai` | 281/281 (100%) | ✅ |
| `hutool-all` | 3/3 (100%) | ✅ |
| `hutool-aop` | 37/37 (100%) | ✅ |
| `hutool-bloomFilter` | 72/72 (100%) | ✅ |
| `hutool-cache` | 124/124 (100%) | ✅ |
| `hutool-captcha` | 87/87 (100%) | ✅ |
| `hutool-core` | 7027/7605 (92%) | 🟡 主要缺口：util/thread/annotation/exceptions/comparator/swing |
| `hutool-cron` | 208/208 (100%) | ✅ |
| `hutool-crypto` | 735/745 (99%) | ✅ |
| `hutool-db` | 831/1041 (80%) | 🟡 主要缺口 |
| `hutool-dfa` | 43/43 (100%) | ✅ |
| `hutool-extra` | 388/1082 (36%) | 🔴 缺口最大 |
| `hutool-http` | 458/695 (66%) | 🟡 |
| `hutool-json` | 294/294 (100%) | ✅ |
| `hutool-jwt` | 121/121 (100%) | ✅ |
| `hutool-log` | 283/283 (100%) | ✅ |
| `hutool-poi` | 0/555 (0%) | ✅ **2026-08-04 已移除**：占位 crate 删除，Excel/Word/OFD 能力由 `easyexcel-rust` 承接，不再计入本仓库 |
| `hutool-script` | 79/79 (100%) | ✅ |
| `hutool-setting` | 225/225 (100%) | ✅ |
| `hutool-socket` | 102/102 (100%) | ✅ |
| `hutool-system` | 189/189 (100%) | ✅ |

---

## 四、按 hutool 子包细分（hutool-core）

| 子包 | 覆盖 | 缺口 |
|---|---:|---:|
| `util` | 1237/1407 (88%) | 170 |
| `lang` | 678/915 (74%) | 237 |
| `thread` | 189/222 (85%) | 33 |
| `annotation` | 142/192 (74%) | 50 |
| `exceptions` | 89/96 (93%) | 7 |
| `comparator` | 80/87 (92%) | 7 |
| `swing` | 0/68 (0%) | 68（unsafe-to-copy） |

其他子包（io / text / map / collection / net / img / convert / bean / codec / builder / math / compress / stream / compiler / clone / getter）均 100%。

---

## 五、状态决策

### 5.1 评估结论

hutool-rust 已经拥有**完整的迁移验证体系**：
- `parity/hutool-v5.8.46-api.csv`：14 082 行 API 清单（每个公开方法 1 行）
- `parity/decisions.csv`：13 871 行决策记录（每个 API 1 行决策 + Rust 符号 + 测试证据）
- `parity/hutool-v5.8.46-tests.csv`：3 293 行测试方法清单
- `parity/test-decisions.csv`：每个 hutool 测试方法的 Rust 对应记录
- `scripts/verify-parity.py`：API 覆盖率验证脚本
- `scripts/verify-test-parity.py`：测试覆盖率验证脚本

**这比 IMPLEMENTATION_PLAN.md §16 Step 0 规划的 `verify-file-coverage.sh` 更精细**——已经在方法级别验证，而非仅文件级别。

### 5.2 IMPLEMENTATION_PLAN.md 文档与实际状态差异

| 维度 | IMPLEMENTATION_PLAN.md 描述 | 实际状态 |
|---|---|---|
| 总体完成度 | "≈ 59.6% 文件数 / ≈ 50% 加权方法数" | **API 83.53% / feasible 97.55% / TEST 99.21%** |
| 缺口最大模块 | "hutool-extra 缺 170 文件" | **hutool-extra 36% API 缺口（确认）** |
| hutool-poi | ~~79 个 Rust 源文件、67 处 `unimplemented!()`~~（2026-08-04 已移除） | **555 个 API 由 `easyexcel-rust` 承接** |

**结论**：IMPLEMENTATION_PLAN.md 是基于早期估算的规划，实际迁移进度远超文档描述。后续 Phase 应以**本文档基线**为起点。

### 5.3 后续 Phase 起点

按 IMPLEMENTATION_PLAN.md 的 6 Phase 框架，**Phase 1~6 的工作已完成大部分**，剩余主要缺口：

| Phase | 已覆盖 | 剩余缺口 | 估算工作量 |
|---|---|---|---|
| Phase 1（hutool-extra） | 388/1082 (36%) | 694 API | 4~6 周 |
| Phase 2（hutool-db） | 831/1041 (80%) | 210 API | 2~3 周 |
| Phase 2（hutool-core 收尾） | 7027/7605 (92%) | 578 API | 3~4 周 |
| Phase 3（hutool-http） | 458/695 (66%) | 237 API | 2~3 周 |
| Phase 3（hutool-crypto） | 735/745 (99%) | 10 API | 0.5 周 |
| Phase 4（hutool-ai provider） | 281/281 (100%) | ✅ 已完成 |
| Phase 4（hutool-poi） | 0/555 (0%) | 555 API（**实现范围外**） | 不做 |
| Phase 5（命名收尾） | — | — | 1~2 周 |

**总剩余工作量估算**：**8~12 周**（不包含 hutool-poi 实现）

---

## 六、验收方式

任何 Phase 完成时，运行以下命令验证：

```bash
# 1. API 覆盖率
python3 scripts/verify-parity.py

# 2. 按模块细分
python3 scripts/verify-parity.py --by-module

# 3. 排除 unsafe-to-copy 后的可达覆盖率
python3 scripts/verify-parity.py --feasible

# 4. 严格模式（CI 集成）
python3 scripts/verify-parity.py --require-complete

# 5. 测试覆盖率
python3 scripts/verify-test-parity.py

# 6. 编译 + 测试
cargo test --workspace
```

**Phase 6 完成条件**：`feasible_covered == 100%` 且 `cargo test --workspace` 全绿。
