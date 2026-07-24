//! Hutool HTTP 原生对象与 Vernal Context 的真实桥接合同测试。

use std::sync::Arc;

use hutool_http::{
    HttpClient, HttpConfig, HttpError, UrlPolicyError,
};
use hutool_vernal::HutoolHttpComponents;
use vernal_context::VernalApplicationBuilder;

#[tokio::test]
async fn context_resolves_singleton_http_objects_and_applies_url_policy() {
    let mut config = HttpConfig::default();
    config.max_response_bytes = 4096;
    let components = HutoolHttpComponents::new(config).deny_local_targets();
    let mut application =
        VernalApplicationBuilder::current().expect("Tokio runtime should be available");
    components
        .install(&mut application)
        .expect("Hutool components should install atomically");

    let context = application.build().expect("application should build");
    context.refresh().await.expect("context should refresh");
    context.start().await.expect("context should start");
    let first_client = context
        .container()
        .resolve::<HttpClient>()
        .expect("HTTP client should resolve");
    let second_client = context
        .container()
        .resolve::<HttpClient>()
        .expect("HTTP client should stay singleton");
    let resolved_config = context
        .container()
        .resolve::<HttpConfig>()
        .expect("HTTP configuration should resolve");

    assert!(Arc::ptr_eq(&first_client, &second_client));
    assert_eq!(resolved_config.max_response_bytes, 4096);
    assert!(matches!(
        first_client.get_text("http://127.0.0.1/private").await,
        Err(HttpError::UrlPolicy(UrlPolicyError::DeniedTarget))
    ));

    context.close().await.expect("context should close");
}
