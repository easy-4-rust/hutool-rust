# 安全基线设计规格

日期：2026-08-12
来源：重定位 `docs/security.md`

## 1. 背景

Hutool-Rust 将网络、加密、归档、脚本和解析器边界视为敌意输入面。

## 2. 默认安全设置

- HTTP 使用 Rustls、连接/总超时、重定向限制和有界 body
- HTTP 重试是 opt-in 且拒绝非幂等方法和不可克隆请求 body；可重试状态/传输故障使用有界抖动延迟
- `DenyLocalTargets` 拒绝私有和本地字面 IP 目标。接受任意主机名的应用应安装感知解析器的 `UrlPolicy` 以防 DNS 重绑定
- AES 使用认证 AES-256-GCM + 新鲜随机 nonce
- 密码使用 Argon2id + 新鲜随机盐 + PHC 输出
- JWT 验证固定 HS256 并要求显式 issuer、audience、时钟偏差和过期策略
- ZIP 提取拒绝遍历路径和符号链接，并强制条目和未压缩字节限制
- 图片操作在返回分配前绑定编码输入、解码尺寸和像素、目标尺寸和编码输出
- POI/Office 处理由 `easyexcel-rust` 独立项目承接，本 workspace 不存在 XLSX/DOCX/OFD/PDF 面
- SMTP 凭证使用脱敏秘密容器。TLS 和强制 STARTTLS 是显式模式
- Rhai 脚本有操作、调用深度、表达式深度、字符串、数组和 map 限制。动态 `eval` 禁用
- OpenAI 兼容流增量解析 SSE 并拒绝超大事件
- 可失败 cron job 使用显式有界重试计数、有界指数延迟、可选执行超时和追踪 span
- hutool-observability 默认仅 tracing、metrics 和 health。CPU、Tokio Console 和堆分析需单独 Cargo feature + 匹配不可伪造运行时许可
- Tokio Console 限制为 loopback。可观测性 crate 不启动管理 HTTP 服务器、隐藏运行时或全局 allocator
- workspace 拒绝 Rust unsafe 代码

## 3. 调用方责任

- 保持 HTTP/DNS 策略与部署网络对齐
- 在服务边界应用认证、授权、速率限制和请求大小限制
- 服务端存储 CAPTCHA 答案 + 单次语义 + 速率限制
- 将注入音频 CAPTCHA 语音引擎视为可信依赖
- 净化不可信 HTML 邮件内容
- 保持数据库事务边界显式并保护连接 URL
- 选择适合下游服务预算的 cron 重试和超时值
- 用 `secrecy` 或 `hutool_log::Redacted` 包装秘密
- 认证诊断管理路由、强制有界分析时长和响应大小

## 4. 遗留算法

MD5、SHA-1、ECB 加密和未认证加密不由默认 `crypto` feature 暴露。
`crypto-legacy` feature 保留用于显式迁移助手，当前不添加算法。

安全漏洞请按 `SECURITY.md` 报告。

## 5. 参考

- 原始文档：`docs/security.md`（已删除，内容已重定位至此）
- `SECURITY.md` 根目录安全策略门面（保留）
