# 可观测性策略设计规格

日期：2026-08-12
来源：重定位 `docs/observability.md`

## 1. 背景

`hutool-observability` 是 Hutool-Rust 的独立可观测性组件，遵循固定边界。

## 2. 固定边界

- tracing、metrics、health 是 crate 的默认 feature
- pprof、tokio-console、heap-profiler 必须显式编译
- 三个诊断后端即使已编译，也必须取得运行时 `DiagnosticPermit`
- samply、perf、eBPF、bpftrace、gdb 和 lldb 永远是外部工具，不进入依赖树
- crate 不启动 HTTP 管理服务，不创建隐藏 Tokio runtime，不选择全局 allocator

## 3. Feature 契约

| Feature | 默认 | 需运行时授权 | 边界 |
|---|:---:|:---:|---|
| tracing | 是 | 否 | 提供 reloadable filter；只有显式调用 install 才安装全局 subscriber |
| metrics | 是 | 否 | 显式安装 Prometheus recorder；只返回渲染 handle，不启动 scrape server |
| health | 是 | 否 | 应用拥有的线程安全健康注册表 |
| pprof | 否 | 是 | 启动采样和读取 protobuf 都要求 CpuProfile permit |
| tokio-console | 否 | 是 | 只接受 loopback 地址；应用自行运行 console server |
| heap-profiler | 否 | 是 | 应用必须显式声明 DhatAllocator 为全局 allocator |

hutool facade 的 `full` 只包含默认的 `observability`，不会隐式启用三个诊断 feature。

## 4. 默认接入

```rust
use hutool_observability::{
    HealthRegistry, HealthStatus,
    metrics::{PrometheusMetrics, counter},
    tracing::{TracingConfig, install},
};

let _reload = install(&TracingConfig::default())?;
let prometheus = PrometheusMetrics::install()?;
let health = HealthRegistry::new("orders")?;

health.set("database", HealthStatus::Healthy, None)?;
counter!("orders_started_total").increment(1);

let metrics_body = prometheus.render();
let health_report = health.report()?;
```

全局 subscriber 和 metrics recorder 都只能安装一次。库代码只应输出 span/metric，不应调用上述 install。

## 5. 诊断授权

默认 `DiagnosticsAccess` 拒绝所有诊断操作。共享令牌至少为 16 字节，以常量时间比较，并在 Debug 输出中脱敏。

## 6. CPU Profiling

需启用 `observability-pprof` feature。采样窗口由应用设置上限。推荐使用专用 profiling 构建：
```bash
RUSTFLAGS="-C force-frame-pointers=yes" cargo build --profile profiling --features observability-pprof
```

## 7. Tokio Console

需 `--cfg tokio_unstable`。组件拒绝 0.0.0.0、公网 IP 等非 loopback 地址。

## 8. Heap Profiling

只有最终 binary 可以选择全局 allocator；任何 hutool-* 库都不得替应用做此决定。DHAT 会显著降低性能，只用于专用诊断构建。

## 9. 外部工具链

samply/perf（CPU 采样）、eBPF/bpftrace（无侵入 uprobe）、lldb/gdb（崩溃/死锁/底层状态）不会成为 Cargo 依赖或 facade feature。

## 10. 参考

- 原始文档：`docs/observability.md`（已删除，内容已重定位至此）
