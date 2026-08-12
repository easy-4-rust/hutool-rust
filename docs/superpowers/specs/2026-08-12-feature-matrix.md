# Feature 矩阵设计规格

日期：2026-08-12
来源：重定位 `docs/feature-matrix.md`

## 1. 背景

hutool features 是累加的。默认集合仅 `core` 和 `json`；`full` 提供便利但永不隐式启用。

## 2. Feature 矩阵

| Feature | 添加 | 编译成本 | 运行时/平台说明 | 安全影响 |
|---|---|---:|---|---|
| `core` | hutool-core | 低 | 可移植 | 解析器强制类型错误 |
| `xml-serde` | quick-xml 直接类型序列化/反序列化 | 低 | 可移植；需 core | 避免 JSON-value 中间层 |
| `xml-encoding` | 非 UTF-8 XML 解码 | 低 | 可移植；需 core | 输入/深度/节点/属性/文本限制仍适用 |
| `xml-async` | 上游 Tokio XML 适配器 | 中 | Tokio；需 core | 传输和超时策略由调用方持有 |
| `json` | hutool-json | 低 | 可移植 | 全输入解析 |
| `aop` | 拦截器链 | 低 | 同步 | advice 可按设计观察上下文 |
| `bloom-filter` | 概率查找 | 低 | 可移植 | 误报是预期行为 |
| `cache` | Moka 缓存 | 中 | 多线程 | 应用须选择容量/TTL |
| `cron` | cron 解析器和 Tokio jobs | 中 | Tokio runtime 由调用方提供 | 取消、超时、追踪、不重叠和独立有界重试策略 |
| `crypto` | AES-GCM、HMAC、SHA-256、Argon2id | 中 | OS 随机数 | 仅安全默认 |
| `crypto-legacy` | 保留遗留命名空间 | 无 | 显式 opt-in | 永不用于密码/认证 |
| `db` | 共享分页/连接池策略 | 中 | 无驱动选择 | 无全局池或隐藏事务 |
| `db-postgres` | SQLx PostgreSQL | 高 | 网络/数据库 | 显式驱动和 URL |
| `db-mysql` | SQLx MySQL | 高 | 网络/数据库 | 显式驱动和 URL |
| `db-sqlite` | SQLx SQLite | 高 | 内置 SQLite | 文件权限由调用方持有 |
| `dfa` | Aho-Corasick 匹配 | 低 | 可移植 | 模式/输入大小影响内存 |
| `extra` | QR SVG 和 ZIP | 中 | 文件系统提取 | 路径遍历、符号链接和展开有界 |
| `extra-image` | 解码、尺寸、缩放、裁剪、编码 | 高 | 纯 Rust 编解码 | 编码字节、尺寸、像素和输出字节有界 |
| `extra-mail` | SMTP 和 MIME 消息 | 高 | Tokio、网络、Rustls | 凭证脱敏；明文 SMTP 是显式选择 |
| `http` | 异步 Reqwest/Rustls | 高 | Tokio runtime 由调用方提供 | 限制和 URL 策略 |
| `http-blocking` | 阻塞 Reqwest 客户端 | 高 | 阻塞当前线程 | 显式 opt-in |
| `log` | tracing 设置/脱敏 | 中 | 可移植 | 敏感值需 Redacted 包装 |
| `observability` | hutool-observability 默认 tracing/metrics/health | 中 | 显式请求前不安装全局状态 | 管理传输由应用持有 |
| `observability-pprof` | 进程内 CPU 采样器 | 高 | Unix 导向 | 编译时 opt-in + CpuProfile 许可 |
| `observability-tokio-console` | Tokio 任务/资源控制台 | 高 | 需 --cfg tokio_unstable；仅 loopback | 编译时 opt-in + TokioConsole 许可 |
| `observability-heap-profiler` | DHAT 堆分析适配器 | 高 | 最终 binary 须选择 allocator | 编译时 opt-in + HeapProfile 许可 |
| `script` | 受限 Rhai | 中 | 进程内 | 操作/深度/容器限制；动态 eval 禁用 |
| `setting` | 分层配置 | 中 | 文件系统/环境 | 不记录秘密配置值 |
| `system` | 宿主指标 | 中 | OS 特定后端 | 可能暴露宿主元数据 |
| `captcha` | 代码生成、验证和 SVG 渲染 | 低 | OS 随机数 | 存储/速率限制由调用方持有 |
| `captcha-raster` | 随机位图字形 PNG 渲染 | 高 | 纯 Rust 编解码 | 尺寸、像素和代码长度有界 |
| `captcha-audio` | 注入语音 PCM 到噪声 WAV | 低 | 调用方提供语音引擎 | 采样率、时长和采样数有界 |
| `socket` | 有界异步 TCP/UDP 辅助 | 中 | Tokio、OS 套接字 | 连接超时和帧限制 |
| `jwt` | 作用域 HS256 JWT | 中 | 可移植 | issuer/audience/leeway/expiry 是强制策略 |
| `ai` | provider 抽象/OpenAI 兼容 JSON 和 SSE 客户端 | 高 | HTTP/Tokio | API key 脱敏；HTTP 和 SSE 事件大小有界 |
| `hutool-compat` | StrUtil/JsonUtil 迁移 API | 低 | 可移植 | 与惯用 API 隔离 |

## 3. 约定

- MSRV 是 Rust 1.94
- Feature 添加不得改变已启用函数的语义
- 数据库驱动和替代阻塞行为保持显式 opt-in
- `full` 启用 `observability`，但永不启用 `observability-pprof`、`observability-tokio-console` 或 `observability-heap-profiler`
- XML DOM 和有界流 API 包含在 `core` 中；`xml-serde`、`xml-encoding` 和 `xml-async` 是累加的，不由 `full` 启用
- hutool-poi 不在本表中：hutool facade 未定义 `poi` 或 `poi-docx` feature，占位 crate 已于 2026-08-04 移除

## 4. 参考

- 原始文档：`docs/feature-matrix.md`（已删除，内容已重定位至此）
- `docs/xml.md` XML API 层、防御默认和性能验证边界
