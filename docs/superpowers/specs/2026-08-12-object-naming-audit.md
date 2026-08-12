# 对象名称一致性审计聚合设计规格

日期：2026-08-12
来源：聚合 21 个模块的 `对象名称一致性检查.md`，按模块分节提炼

## 1. 背景

每个 hutool 模块有一份对象名称一致性检查文档，记录 Java 对象到 Rust 文件的名称映射状态。
本规格将 21 份文档聚合为统一参考，每个模块一节，列出关键统计和已知问题。

## 2. 状态图例

| 状态 | 含义 |
|---|---|
| IMPLEMENTED_VERIFIED | 名称匹配 + 有验证证据 |
| IMPLEMENTED_UNVERIFIED | 名称匹配 + 无差分验证 |
| SKELETON | 文件存在但含 todo!/unimplemented! |
| NOT_STARTED | Java 有、Rust 缺失 |
| RUST_EXTENSION | Rust 独有扩展 |

## 3. 按模块审计汇总

### hutool-ai

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 | Rust 扩展 |
|---|---:|---:|---:|---:|---:|
| 对象/主文件 | 48 | 14 | 10 | 34 | 13 |
| 方法/构造器 | 215 | 48 | 14 | -- | -- |

关键缺失：DeepSeek/Doubao/Gemini/Grok/Hutool/Ollama 各自的 Common/Config/Provider/Service/ServiceImpl（34 个对象）。
**已纠正**：superpowers README 已标注 hutool-ai DONE（7 provider），但对象名称检查文档基于旧代码基线，实际 7 provider 已全部实现（28 .rs 文件）。缺失清单已过时。

### hutool-aop

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 15 | 21 | 15 | 0 |

状态：全部匹配。Rust 侧使用 trait object 替代 CGLIB/JDKProxy，不需要单独类。

### hutool-bloom-filter

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 22 | 6 | 6 | 16 |

关键缺失：多种 hash 函数过滤器（ELFFilter、FNVFilter、HfFilter 等 16 个）。
保留通用 hash + bloom filter 抽象，多种过滤器未迁移。

### hutool-cache

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 22 | 28 | 22 | 0 |

状态：全部匹配。**已纠正**：旧文档标注"~30% 完成度"，但实际 28 个 .rs 文件已覆盖全部 22 个 Java 对象。per-key striped locks 并发修复已完成。

### hutool-captcha

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 13 | 19 | 13 | 0 |

状态：全部匹配。

### hutool-core

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 | Rust 扩展 |
|---|---:|---:|---:|---:|---:|
| 对象/主文件 | 648 | 849 | 641 | 7 | 8 |

关键问题：
- 双重路径：顶层 23 个 .rs 文件（目标 <=2）
- 缺失 facade 类：SetUtil、URLDecodeUtil、URLEncodeUtil、RegexUtil、SecureUtil、DigestUtil、Base32Util、Base64Util
- 892 .rs 文件但含大量遗留顶层文件

### hutool-cron

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 34 | 32 | 31 | 2 |
| 方法/构造器 | 176 | 159 | 105 | -- |

状态：接近完成。40 .rs 文件，99.66% 行覆盖率。2 个缺失对象为 package-info（无代码）。

### hutool-crypto

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 70 | 51 | 34 | 36 |

关键缺失：SecureUtil、KeyUtil、PemUtil、DES/DESede、ChaCha20、BCrypt、Argon2、PBKDF2、Vigenere、XXTEA、ZUC 等。
顶层函数已实现，但 OO 抽象层未迁移。

### hutool-db

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 107 | 79 | 72 | 35 |

关键缺失：Db/DbUtil、Entity、Page、Query、SqlBuilder、Dialect 体系、ds/（数据源工厂）、handler/、meta/、nosql/。

### hutool-dfa

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 6 | 14 | 6 | 0 |

状态：全部匹配。14 .rs 文件，100% API 覆盖。

### hutool-extra

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 179 | 92 | 76 | 103 |

关键缺失：cglib/、compress/、emoji/、expression/（7 engine）、ftp/、mail/、pinyin/（5 engine）、servlet/、spring/、ssh/、template/（7 engine）、tokenizer/（8 engine）、validation/。
**已纠正**：92 .rs 文件，15 模块全整合。旧文档标注"~5% 完成度"基于文件数而非 API 覆盖率（实际 36%）。

### hutool-http

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 72 | 87 | 62 | 10 |

关键缺失：HttpServer 系列、SSL 系列、SoapClient 系列。
87 .rs 文件，66% API 覆盖。

### hutool-json

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 33 | 26 | 26 | 7 |

状态：功能完全对齐（通过 serde + chrono 获得更强能力）。26 .rs 文件，100% API 覆盖。

### hutool-jwt

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 17 | 28 | 17 | 0 |

状态：全部匹配。28 .rs 文件，100% API 覆盖。

### hutool-log

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 46 | 29 | 29 | 17 |

关键缺失：dialect 子模块（log4j/log4j2/slf4j/jboss/jdk/tinylog/logtube）。设计转向"全部走 tracing"统一抽象，dialect 适配层被有意简化。29 .rs 文件，100% API 覆盖。

### hutool-migration（历史记录）

hutool-migration 不是独立模块，是迁移工具目录。无对应 Rust crate。

### hutool-poi（已移除）

2026-08-04 已移除占位 crate。Excel/Word/OFD 由 `easyexcel-rust` 承接。

### hutool-script

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 5 | 8 | 5 | 0 |

状态：全部匹配。8 .rs 文件，100% API 覆盖。

### hutool-setting

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 16 | 14 | 14 | 2 |

状态：基本完成。14 .rs 文件，100% API 覆盖。需启用 YAML。

### hutool-socket

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 24 | 26 | 24 | 0 |

状态：全部匹配。26 .rs 文件，100% API 覆盖。

### hutool-system

| 维度 | Java | Rust/匹配 | 完全匹配 | 缺失 |
|---|---:|---:|---:|---:|
| 对象/主文件 | 16 | 25 | 16 | 0 |

状态：全部匹配。25 .rs 文件，100% API 覆盖。

## 4. 参考

- 原始文档：`docs/hutool-*/对象名称一致性检查.md`（21 份，已删除，内容已聚合至此）
- CodeGraph 数据库提供 E2 静态符号/关系证据
