# 生产就绪设计规格

日期：2026-08-12
来源：重定位 `docs/production-readiness.md`

## 1. 背景

本规格检查实现是否符合原始 Hutool-Rust 架构要求，描述当前 0.1 线，不宣称方法级 Hutool 兼容。

## 2. 架构与 API

| 要求 | 状态 | 证据或边界 |
|---|---|---|
| Workspace 结构 | 完成 | 26 workspace crate：20 个已实现能力 crate、facade、兼容层、宏、测试支持和 Rust 扩展（hutool-poi 占位已移除） |
| 最小 facade 默认 | 完成 | hutool 默认 core 和 json；full 是显式的 |
| 累加 feature 模型 | 完成 | 每个 facade 和组件 feature 在 CI 中独立检查 |
| 无反向/循环组件依赖 | 完成 | 仅 facade 聚合；组件永不依赖 hutool |
| 惯用和兼容 API 分离 | 完成 | hutool-compat-hutool 委托到 core/JSON 实现 |
| 无隐藏全局客户端/池/配置/运行时 | 完成 | 有状态资源显式构造和注入 |
| 完整 Hutool 功能对等 | 仅账本 | pinned v5.8.46 API 已注册，但包含 planned 和 unportable 条目 |

## 3. 运行时边界

| 要求 | 状态 | 证据或边界 |
|---|---|---|
| HTTP 超时、重定向、body 限制、流、SSRF hook | 完成 | hutool-http 有界 Rustls 客户端和 UrlPolicy |
| 仅幂等 HTTP 重试 | 完成 | 显式有界 jitter/backoff 和 Retry-After 处理 |
| 显式 SQLx 池和事务 | 完成 | 驱动 feature + 真实 PostgreSQL/MySQL/SQLite 事务测试 |
| Cron 外部运行时、关闭、超时、不重叠、追踪 | 完成 | spawn_on、JobHandle、JobPolicy、每运行 span |
| Cron 重试独立于 job 实现 | 完成 | RetryPolicy 和 fallible spawn API + 有界指数延迟 |
| 有界网络/解析器/媒体输入 | 完成 | HTTP、SSE、套接字、ZIP、图片、邮件、CAPTCHA、Rhai |
| 有界二进制序列化 | 完成（启用时） | 16 MiB 默认负载限制、精确长度、schema/version、codec、flags、尾随字节和 CRC32 校验 |
| 默认可观测性 | 完成 | hutool-observability 默认可重载 tracing、显式 Prometheus recorder 安装和应用持有的健康注册表 |
| 诊断后端 | feature 和授权门控 | pprof、tokio-console 和 DHAT 各需非默认 Cargo feature + 匹配运行时许可 |

## 4. 安全与质量

| 要求 | 状态 | 证据或边界 |
|---|---|---|
| 无 workspace unsafe 代码 | 完成 | 每个 crate 禁止 unsafe；CI 构建所有 feature |
| 安全加密默认 | 完成 | AES-256-GCM、HMAC-SHA-256、Argon2id、作用域 HS256 JWT |
| 遗留加密隔离 | 完成 | Hutool 命名 MD5/SHA1/DSA/SM4-CMAC 返回类型拒绝错误 |
| 供应链策略 | 完成 | cargo-deny advisories、licenses、bans 和 sources 检查 |
| 默认/无默认/全 feature 测试 | 完成 | CI 质量 job 和本地发布门控 |
| 每 feature 编译 | 完成 | cargo hack --each-feature --locked |
| Stable/MSRV/Nightly/GNU/MUSL/macOS/Windows | 完成 | CI 矩阵；MSRV 是 Rust 1.94 |
| 属性、编译失败、fuzz 和集成测试 | 完成 | codec 属性；编译失败文档；结构化解析器 fuzz 目标 |
| 100% 测试覆盖 | 进行中 | 当前全 feature 基线：行 98.73%、区域 98.71%、函数 98.30% |
| SemVer 回归检查 | 首次发布后就绪 | tag workflow 运行 cargo-semver-checks |

## 5. 可选二进制序列化门控

- Musli 精确锁定到 0.0.149，声明的 MSRV 不超过 Hutool-Rust 的 Rust 1.94 基线
- Musli 和每个具体格式是累加的、非默认 Cargo feature
- `full` 故意不激活格式或静默选择 wire 兼容契约
- 生产采用需在代表性应用模型上记录 Criterion 结果

## 6. 公开 1.0 前的门控

- 按依赖顺序保留和发布 crates.io 名称
- 完成独立安全和许可证/来源审查
- 定义精确 1.0 Hutool 能力范围并冻结公共 API
- 建立已发布 SemVer 基线和支持版本策略
- 运行长时间 fuzz 战役并记录语料库、时长和工具链
- 为生产形状基准展示的热点添加性能基线

## 7. 参考

- 原始文档：`docs/production-readiness.md`（已删除，内容已重定位至此）
