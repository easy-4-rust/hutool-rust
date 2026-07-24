# hutool-vernal

`hutool-vernal` is the consumer-owned integration between Hutool-Rust utilities
and Vernal Framework. It keeps the dependency direction explicit:

```text
application -> hutool-vernal -> hutool-http + Vernal public contracts
```

The bridge currently installs Hutool-Rust `HttpConfig` and its Tokio/Reqwest
`HttpClient` as context-local Vernal singleton components. The client keeps the
configuration dependency visible in Vernal's validated graph and can use a
custom `UrlPolicy` without any process-global registry.

```rust
use hutool_vernal::HutoolHttpComponents;
use vernal_context::VernalApplicationBuilder;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let mut application = VernalApplicationBuilder::current()?;
HutoolHttpComponents::default()
    .deny_local_targets()
    .install(&mut application)?;

let context = application.build()?;
context.refresh().await?;
let client = context
    .container()
    .resolve::<hutool_http::HttpClient>()?;
context.close().await?;
# Ok(())
# }
```

## 中文说明

`hutool-vernal` 是 Hutool-Rust 主动消费 Vernal 公共合同的桥接 crate。它不会把
工具对象复制进 Vernal 内核，也不会让 Vernal 反向依赖 Hutool-Rust。

当前桥接会把 `HttpConfig` 与基于 Tokio/Reqwest 的 `HttpClient` 作为
Context-local Singleton 原生组件原子注册：配置依赖会进入 Vernal 的启动期依赖图，
构造失败保留原始错误，URL 安全策略由应用显式选择，全程不使用全局 Service
Locator。
