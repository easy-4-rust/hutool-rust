//! Hutool 消费方 `ApplicationModule` 的原子装配合同测试。

use std::sync::Arc;

use hutool_http::{HttpClient, HttpConfig};
use hutool_setting::Setting;
use hutool_vernal::{HutoolApplicationModule, HutoolHttpComponents, HutoolSettingPropertySource};
use vernal_context::{ApplicationModuleError, VernalApplicationBuilder};
use vernal_ioc::ComponentDefinition;

/// 创建一份不依赖文件系统的 Hutool Setting 属性快照。
fn setting_source(name: &str, value: &str) -> HutoolSettingPropertySource {
    let setting = Setting::new();
    setting.set("bridge.value", value);
    HutoolSettingPropertySource::new(name, &setting).expect("valid Hutool property source")
}

#[tokio::test]
async fn named_module_atomically_installs_http_setting_and_profiles() {
    let mut config = HttpConfig::default();
    config.max_response_bytes = 8192;
    let module = HutoolApplicationModule::new()
        .with_http_components(HutoolHttpComponents::new(config).deny_local_targets())
        .setting_last(setting_source("hutool:application-module", "ready"))
        .active_profile("dev")
        .default_profile("default");
    let mut application =
        VernalApplicationBuilder::current().expect("Tokio runtime should be available");
    application
        .register_module(module)
        .expect("Hutool application module should install atomically");

    let context = application.build().expect("application should build");
    context.refresh().await.expect("context should refresh");
    let client = context
        .container()
        .resolve::<HttpClient>()
        .expect("HTTP client should resolve");
    let resolved_config = context
        .container()
        .resolve::<HttpConfig>()
        .expect("HTTP configuration should resolve");

    assert_eq!(resolved_config.max_response_bytes, 8192);
    assert_eq!(
        context
            .environment()
            .property("bridge.value")
            .expect("bridge property")
            .as_deref(),
        Some("ready")
    );
    assert_eq!(context.environment().active_profiles(), ["dev"]);
    assert_eq!(context.environment().default_profiles(), ["default"]);
    assert!(Arc::strong_count(&client) >= 2);
    context.close().await.expect("context should close");
}

#[tokio::test]
async fn definition_failure_rolls_back_setting_profile_and_module_identity() {
    let mut application =
        VernalApplicationBuilder::current().expect("Tokio runtime should be available");
    application
        .register(ComponentDefinition::shared_value(HttpConfig::default()))
        .expect("pre-existing HTTP configuration");

    let failing = HutoolApplicationModule::new()
        .with_default_http()
        .setting_last(setting_source("hutool:retry", "first"))
        .active_profile("failed-profile");
    let Err(error) = application.register_module(failing) else {
        panic!("duplicate HttpConfig should reject the whole module");
    };
    assert!(matches!(
        error,
        ApplicationModuleError::Definition {
            module: "hutool.application",
            ..
        }
    ));

    // 第一次失败不得提交 Setting、Profile 或模块身份，因此相同来源名和模块名可以
    // 在去掉冲突 Definition 后立即作为一个修正后的模块重试。
    application
        .register_module(
            HutoolApplicationModule::new()
                .setting_last(setting_source("hutool:retry", "second"))
                .active_profile("retry-profile"),
        )
        .expect("corrected module should reuse every rolled-back identity");
    let context = application.build().expect("retried application");

    assert_eq!(
        context
            .environment()
            .property("bridge.value")
            .expect("retried bridge property")
            .as_deref(),
        Some("second")
    );
    assert_eq!(context.environment().active_profiles(), ["retry-profile"]);
}
