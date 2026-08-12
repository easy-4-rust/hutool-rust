# 依赖治理策略设计规格

日期：2026-08-12

## 1. 背景

hutool-rust workspace 有 25 个 crate，直接依赖 3~63 个不等。
依赖治理是持续过程：版本锁定、feature 裁剪、组件选型、安全审计都需要统一策略。
本规格从 `IMPLEMENTATION_PLAN.md` 第九节和 `PHASE_BASELINE.md` 提炼，固化为可执行规范。

## 2. 目标与非目标

### 2.1 目标

- 建立精确锁基线（`Cargo.lock` + `cargo deny`），保证可重复构建。
- 每个 crate 的 `[features]` 设计遵循"默认零、driver 显式、legacy opt-in"原则。
- 组件选型有明确的评估记录（为什么选 A 不选 B）。
- 安全审计（RustSec advisory）纳入 CI。

### 2.2 非目标

- 不强制所有 crate 使用同一版本的传递依赖（允许 `cargo tree --duplicates` 在合理范围内）。
- 不为 hutool-rust 建立私有 registry。
- 不在 v1.0 前引入 `sea-orm` / `diesel` 等 ORM（违反"不造 ORM"原则）。

## 3. 版本锁定策略

### 3.1 Cargo.lock

- `Cargo.lock` 必须提交到版本控制。
- workspace 级别统一管理：所有 crate 通过 `[workspace.dependencies]` 声明版本。
- 单个 crate 的 `Cargo.toml` 只写 `dep.workspace = true`，不硬编码版本号。

### 3.2 cargo deny 配置

```toml
# deny.toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Zlib", "Unicode-3.0"]

[bans]
multiple-versions = "warn"   # 允许合理范围内的多版本
wildcards = "deny"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
allow-git = []
```

## 4. Feature 设计原则

### 4.1 核心规则

1. **默认零**：`default = []`（facade `hutool` 除外，默认 `core` + `json`）。
2. **driver 显式**：数据库驱动、TLS 后端、IO 模式全部 feature-gated。
3. **legacy opt-in**：DES/RC4/MD5 等历史算法放 `legacy` feature。

### 4.2 Facade crate feature 设计

```toml
# crates/hutool/Cargo.toml
[features]
default = ["core", "json"]
core = ["dep:hutool-core"]
json = ["core", "dep:hutool-json"]
crypto = ["core", "dep:hutool-crypto"]
crypto-legacy = ["crypto", "hutool-crypto/legacy"]
http = ["core", "json", "dep:hutool-http"]
http-blocking = ["http", "hutool-http/blocking"]
db = ["core", "json", "dep:hutool-db"]
db-postgres = ["db", "hutool-db/postgres"]
db-mysql = ["db", "hutool-db/mysql"]
db-sqlite = ["db", "hutool-db/sqlite"]
ai = ["http", "json", "dep:hutool-ai"]
extra = ["core", "dep:hutool-extra"]
full = ["crypto", "http", "db", "ai", "extra", "dep:hutool-cron", ...]
```

### 4.3 单 crate feature 设计

| crate | default features | opt-in features |
|---|---|---|
| hutool-core | `[]` | `swing`, `img`, `async` |
| hutool-extra | `[archive, qrcode, emoji, pinyin]` | `tokenizer`, `ftp`, `ssh`, `template`, `image`, `mail` |
| hutool-http | `[useragent, html]` | `blocking` |
| hutool-db | `[sqlite]` | `postgres`, `mysql` |
| hutool-crypto | `[]` | `legacy` |
| hutool-captcha | `[raster]` | `audio` |

## 5. 组件选型表

| 领域 | Java 依赖 | Rust crate | 选型理由 |
|---|---|---|---|
| JSON | Jackson/Gson | `serde_json` | Rust 生态标准 |
| YAML | SnakeYAML | `serde_yaml_ng` | 同上 |
| HTTP | OkHttp | `reqwest` | 生态最成熟 |
| DB | JDBC/HikariCP | `sqlx` | async + 编译期检查 |
| 缓存 | Caffeine | `moka` | 高性能进程内 |
| 定时 | Quartz | `cron` + 自研解析 | 双轨策略 |
| 加密 | BouncyCastle | RustCrypto 系列 | 纯 Rust + 国密 |
| JWT | jjwt | `jsonwebtoken` | 生态标准 |
| 脚本 | JSR-223 | `rhai` | 沙箱友好 |
| 日志 | SLF4J/Logback | `tracing` | 跨 crate 统一 |
| 配置 | Commons Config | `config` | 多格式支持 |
| 邮件 | javax.mail | `lettre` | Rust SMTP 主流 |
| 拼音 | pinyin4j | `pinyin` | 最成熟中文拼音库 |
| QR 码 | zxing | `qrcode` (svg) | 无 C 依赖 |
| 文件监控 | Commons IO | `notify` | 跨平台 FS 事件 |
| 图片 | Java ImageIO | `image` | 纯 Rust codec |
| 分词 | IK/Jieba | `jieba-rs` | feature-gated |
| FTP | Commons Net | `suppaftp` | feature-gated |
| SSH | JSch | `ssh2` | feature-gated |
| 模板 | Freemarker/Velocio | `minijinja` | feature-gated |

## 6. 依赖审计

每次 PR 触发以下检查：

```bash
cargo deny check              # license + advisory + bans + sources
cargo tree --duplicates       # 重复依赖审查
cargo audit                   # RustSec advisory
cargo outdated --workspace    # 版本新鲜度
```

## 7. 不建议引入的依赖

| 依赖 | 原因 |
|---|---|
| `sea-orm` / `diesel` | 违反"不造 ORM"原则 |
| `ring` | OpenSSL 绑定优先，会失去国密 sm2/sm3/sm4 |
| `axum`（在 hutool-http 内） | HTTP crate 是客户端，Server 应独立 crate |
| `nacos-rust`（默认） | 国内中间件 Rust 客户端不成熟，放可选 |

## 8. 参考

- `IMPLEMENTATION_PLAN.md` 第九节"关键技术决策汇总"、第十一节"依赖清单"
- `PHASE_BASELINE.md` 第五节 API 覆盖率基线
- `feature-matrix.md` Feature 矩阵
