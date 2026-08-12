# 对象级与语义迁移对照聚合设计规格

日期：2026-08-12
来源：聚合 21 个模块 x2 份文档（对象级对照表 + 语义迁移对照表）= 42 份，按模块分节提炼

## 1. 背景

每个 hutool 模块有两份对照文档：对象级对照表（Java 对象到 Rust 类型的结构映射）和语义迁移对照表（行为合同、类型系统、并发模型等语义差异）。
本规格将 42 份文档聚合为统一参考，每个模块列出对象级映射要点和关键语义差异。

## 2. 通用语义迁移原则

### 2.1 类型、所有权与可空性

| Java 语义 | Rust 映射原则 |
|---|---|
| nullable reference / null | Option<T>，不得用空串/零值偷偷代替 |
| mutable bean | struct + 受控 setter/builder |
| interface/abstract class | trait + 组合状态/默认方法 |
| Java 泛型/通配符 | Rust 泛型、trait bound、dyn Trait |

### 2.2 方法、重载与参数

- 一个基础语义保留 canonical snake_case 名
- 差异使用 `_with_*`、`_from_*`、options、trait 或 builder
- Java 参数名按 snake_case 保持顺序，可空性显式使用 Option

### 2.3 错误体系

- Java checked exception -> Result<T, E>
- Java unchecked exception -> panic（仅在不可恢复时）
- 所有失败分支不得 panic 化

### 2.4 并发、异步与生命周期

- 所有权优先，必要时 Arc/锁/atomic/Tokio/channel
- 必须验证：原子复合操作、取消、背压、超时、关闭、资源泄漏

## 3. 按模块对照

### hutool-ai

**对象级**：48 个 Java 对象，14 个 Rust 候选。核心抽象（AIConfig、AIService、Message、AiProvider trait）已迁移。
7 个 provider（OpenAI/DeepSeek/Doubao/Gemini/Grok/Hutool/Ollama）已全部实现。

**语义差异**：
- Java 使用接口+实现类分离；Rust 使用 trait + 泛型
- SSE 流式使用 eventsource-stream 替代 Java 的 InputStream
- API key 通过 secrecy::SecretString 脱敏

### hutool-aop

**对象级**：15 个 Java 对象，21 个 Rust 类型。Aspect/BeforeAdvice/AfterAdvice/AroundAdvice/ProxyFactory 已迁移。

**语义差异**：
- Java CGLIB/JDKProxy -> Rust trait object + 拦截器链
- 无运行时代理，编译期确定
- Rust 无反射，JoinPoint 通过闭包传递

### hutool-bloom-filter

**对象级**：22 个 Java 对象，6 个 Rust 类型。BitMap/BloomFilter/Hasher trait 已迁移。

**语义差异**：
- Java 多种 Filter 实现 -> Rust 保留通用 BloomFilter + Hasher trait
- 多种 hash 函数过滤器未迁移（可后续通过 trait 扩展）

### hutool-cache

**对象级**：22 个 Java 对象，28 个 Rust 类型。全部 Java 对象已迁移。

**语义差异**：
- Java AbstractCache 的 striped lock -> Rust per-key striped locks（已实现）
- moka 替代 Caffeine 作为底层引擎
- FIFO/LFU/LRU/Timed/Weak/NoOp 缓存通过 macro 生成
- GlobalPruneTimer 通过 tokio::spawn 替代 Java Timer

### hutool-captcha

**对象级**：13 个 Java 对象，19 个 Rust 类型。全部已迁移。

**语义差异**：
- Java BufferedImage -> Rust image crate
- SVG 渲染纯 Rust 实现
- 音频 CAPTCHA 通过注入语音引擎实现

### hutool-core

**对象级**：648 个 Java 对象，849 个 Rust 类型。641 个已匹配。

**语义差异**：
- String -> &str/String（参数倾向 &str，返回 String）
- Collection<T> -> &[T]（只读场景用 slice）
- Optional<T> -> Option<T>
- Date/Calendar -> chrono::NaiveDateTime
- InputStream/OutputStream -> R: Read/W: Write（trait 泛型）
- Throwable -> Result<T, E>
- Class<T> -> TypeId
- Object -> serde_json::Value

**已知问题**：
- 双重路径：顶层 23 个 .rs 文件需迁移到子目录
- 缺失 facade 类：SetUtil、URLDecodeUtil、URLEncodeUtil、RegexUtil、SecureUtil、DigestUtil

### hutool-cron

**对象级**：34 个 Java 对象，32 个 Rust 类型。31 个完全匹配。

**语义差异**：
- Java Quartz -> Rust 自研 cron 表达式解析 + TimingWheel
- Scheduler 使用 tokio::spawn 替代 Java Thread
- L 哨兵语义已验证（DayOfMonthMatcher.match）
- 99.66% 行覆盖率

### hutool-crypto

**对象级**：70 个 Java 对象，51 个 Rust 类型。34 个完全匹配。

**语义差异**：
- BouncyCastle -> RustCrypto（纯 Rust + 国密 sm2/sm3/sm4）
- Java OO 抽象层（AsymmetricCrypto/SymmetricCrypto）-> Rust 函数式 API
- DES/RC4 等历史算法放 legacy feature
- RSA 时序侧信道风险已文档说明
- AES-GCM 替代 AES-CBC 作为默认

### hutool-db

**对象级**：107 个 Java 对象，79 个 Rust 类型。72 个完全匹配。

**语义差异**：
- JDBC/HikariCP -> sqlx（async + 编译期检查）
- 不造 ORM，使用 sqlx 直接构建
- Entity -> 动态实体（需补）
- Dialect 体系 -> sqlx 内置方言支持
- DSFactory -> sqlx::Pool（连接池内置）

### hutool-dfa

**对象级**：6 个 Java 对象，14 个 Rust 类型。全部匹配。

**语义差异**：
- Java 自实现 DFA -> Rust aho-corasick（Aho-Corasick 算法）
- 左最 longest 匹配语义保留

### hutool-extra

**对象级**：179 个 Java 对象，92 个 Rust 类型。76 个完全匹配。

**语义差异**：
- 邮件：javax.mail -> lettre（feature-gated）
- 拼音：pinyin4j -> pinyin crate
- QR 码：zxing -> qrcode crate（svg）
- 压缩：Commons IO -> zip/flate2
- 分词：IK/Jieba -> jieba-rs（feature-gated）
- FTP：Commons Net -> suppaftp（feature-gated）
- SSH：JSch -> ssh2（feature-gated）
- 模板：Freemarker/Velocio -> minijinja（feature-gated）
- Spring/Servlet：无 Rust 对应（planned/unsafe-to-copy）

### hutool-http

**对象级**：72 个 Java 对象，87 个 Rust 类型。62 个完全匹配。

**语义差异**：
- OkHttp -> reqwest + rustls
- HttpServer 系列未迁移（hutool-http 是客户端优先）
- SSL 系列由 reqwest TLS 替代
- SOAP 系列未迁移
- UserAgent 解析使用 Woothee

### hutool-json

**对象级**：33 个 Java 对象，26 个 Rust 类型。26 个完全匹配。

**语义差异**：
- Jackson/Gson -> serde_json
- JSONBeanParser 由 serde 承担
- XML 转换通过 quick-xml 实现
- 类型化/动态 JSON + Hutool 对齐配置对象/数组

### hutool-jwt

**对象级**：17 个 Java 对象，28 个 Rust 类型。全部匹配。

**语义差异**：
- jjwt -> jsonwebtoken
- 动态 claims、HS/RS/ES/none signers
- PEM 工厂、类型化验证
- 显式拒绝过时/非 JOSE 算法

### hutool-log

**对象级**：46 个 Java 对象，29 个 Rust 类型。29 个完全匹配。

**语义差异**：
- SLF4J/Logback -> tracing（跨 crate 统一）
- dialect 适配层被有意简化（全部走 tracing）
- 日志工厂/级别/记录通过 tracing-subscriber 实现

### hutool-poi（已移除）

2026-08-04 已移除。Excel/Word/OFD 由 easyexcel-rust 承接。

### hutool-script

**对象级**：5 个 Java 对象，8 个 Rust 类型。全部匹配。

**语义差异**：
- JSR-223 -> rhai（沙箱友好）
- 无 JSR-223 全局，有界 Rhai 引擎
- 操作/深度/容器限制；动态 eval 禁用

### hutool-setting

**对象级**：16 个 Java 对象，14 个 Rust 类型。14 个完全匹配。

**语义差异**：
- Commons Config -> config crate + serde_yaml_ng
- 有序分组设置和集、变量展开
- Java properties 和类型转换
- 可注入自动重载、profiles、分层文件/环境

### hutool-socket

**对象级**：24 个 Java 对象，26 个 Rust 类型。全部匹配。

**语义差异**：
- Java NIO/AIO -> tokio async TCP/UDP
- 有界 AIO/NIO 会话、协议 trait、超时和管理关闭

### hutool-system

**对象级**：16 个 Java 对象，25 个 Rust 类型。全部匹配。

**语义差异**：
- Java System/Runtime -> sysinfo crate
- CPU/进程/内存/磁盘/网络/传感器快照
- OS/宿主/用户/运行时信息 + 显式可选 Java/JVM 属性

## 4. 参考

- 原始文档：`docs/hutool-*/对象级对照表.md` + `docs/hutool-*/语义迁移对照表.md`（42 份，已删除，内容已聚合至此）
- `specs/2026-08-12-java-rust-naming-mapping-rules.md` 命名映射规则
- `specs/2026-08-12-method-level-comparison.md` 方法级对照
