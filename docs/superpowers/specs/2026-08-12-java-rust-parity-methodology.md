# Java-Rust 对等方法论设计规格

日期：2026-08-12
来源：重定位 `docs/hutool-parity.md`

## 1. 背景

Hutool-Rust 对齐 Hutool 的能力模型，而非 Java 的反射/运行时模型。
状态值为 `native`、`idiomatic`、`compatible`、`unsafe-to-copy` 和 `planned`。

## 2. 对等账本

| Hutool 模块 | Hutool-Rust crate | 状态 | 当前映射 |
|---|---|---|---|
| hutool-all | hutool | idiomatic | Cargo feature-gated facade + 确定性编译时能力注册 |
| hutool-bom | workspace | native | 锁步版本和 workspace 依赖 |
| hutool-core | hutool-core | idiomatic | 字符串、集合、编解码、日期、ID、闭包/Serde 支持构建器、所有权感知可变包装等 |
| hutool-json | hutool-json | complete | Serde 类型化/动态 JSON + Hutool 对齐配置对象/数组等 |
| hutool-http | hutool-http | idiomatic | Reqwest/Rustls 客户端 + 限制、URL 策略、流和显式幂等重试 |
| hutool-crypto | hutool-crypto | idiomatic | RustCrypto AEAD/HMAC/SHA-256/Argon2id |
| hutool-jwt | hutool-jwt | idiomatic | 动态 claims、HS/RS/ES/none signers、PEM 工厂等 |
| hutool-cache | hutool-cache | idiomatic | Moka 原生缓存 + 确定性 FIFO/LFU/LRU/timed/weak/no-op 兼容缓存 |
| hutool-bloomFilter | hutool-bloom-filter | idiomatic | 类型化概率成员 |
| hutool-dfa | hutool-dfa | idiomatic | Aho-Corasick 最左最长匹配 |
| hutool-setting | hutool-setting | complete | 有序分组设置和集、变量展开、Java properties 和类型转换等 |
| hutool-cron | hutool-cron | complete | Hutool 对齐 5/6/7 部分模式、解析器和匹配器；任务表、监听器和执行器等 |
| hutool-log | hutool-log | complete | Hutool 对齐级别、记录、工厂、静态 facade 和后端别名 |
| hutool-system | hutool-system | idiomatic | sysinfo 支持的 CPU/进程/内存/磁盘/网络/传感器快照 |
| hutool-aop | hutool-aop | idiomatic | 显式拦截器链替代运行时代理 |
| hutool-script | hutool-script | idiomatic | 有界 Rhai 引擎；无 JSR-223 全局 |
| hutool-socket | hutool-socket | idiomatic | Tokio TCP/UDP、有界 AIO/NIO 会话、协议 trait、超时和管理关闭 |
| hutool-extra | hutool-extra | idiomatic | QR SVG、安全 ZIP、有界图片变换、可注入 Rustls SMTP/MIME 邮件 |
| hutool-poi | removed 2026-08-04 | removed | 占位 crate 已删除；Excel/Word/OFD 由 easyexcel-rust 承接 |
| hutool-captcha | hutool-captcha | idiomatic | 生成器/挑战验证、随机 SVG/PNG 和可注入语音到 WAV 音频渲染 |
| hutool-db | hutool-db | idiomatic | SQLx 池、显式事务和分页，非自定义 ORM |
| hutool-ai | hutool-ai | complete | provider 中立核心 + Hutool 对齐配置、模型、工厂和七个 provider 的穷举操作 |
| StrUtil/JSONUtil | hutool-compat-hutool | compatible | 聚焦迁移 facade |
| Bean 反射拷贝 | none | unsafe-to-copy | 使用 Serde、From/TryFrom 或 derive 宏 |
| 全局 HTTP/配置/DB 单例 | none | unsafe-to-copy | 客户端和池显式注入 |

## 3. Unportable 矩阵

| 标签 | 典型 Hutool 区域 | Hutool-Rust 处置 |
|---|---|---|
| awt_swing | core.swing、桌面 UI | planned -- 无 AWT |
| javax_servlet | extra.servlet | planned -- 无 Servlet 容器 |
| jndi | JNDI 工厂 | planned -- 显式注入替代 |
| reflection | ReflectUtil、ClassUtil、BeanDesc、代理 | unsafe-to-copy -- Serde / From / 宏 |
| javax_sql_spi | JDBC Statement/Connection 包装、全局 | planned -- 仅 SQLx 池 |
| bouncycastle_only | ZUC、Cipher SPI、纯 BC 参数类型 | planned -- RustCrypto 子集 |
| soap_server | SOAP 客户端、SimpleServer、HttpConnection | planned -- reqwest 客户端模型 |
| jvm_only | SSH/FTP/Spring/CGLIB/模板/分词器、poi 引擎 | planned -- 或延后到 easy* |
| portable | 其他所有 | 实现时 idiomatic；仅作为临时 Wave 账本行时 planned |

## 4. 迁移 DoD（账本）

1. registered = 100% 的 pinned v5.8.46 API
2. 对 portable 标签最大化 idiomatic
3. 保持 TEST 注册 100%；行为 planned 仅用于声明的 unportable 测试
4. 永不删除已有 idiomatic 实现；facade 必须委托
5. hutool-poi 已于 2026-08-04 从 workspace 移除

## 5. 参考

- 原始文档：`docs/hutool-parity.md`（已删除，内容已重定位至此）
- `scripts/classify-unportable.py` 分类器
