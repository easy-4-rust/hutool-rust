//! Hutool HTTP 组件包对象。

use std::{error::Error, sync::Arc};

use hutool_http::{AllowAllUrls, DenyLocalTargets, HttpClient, HttpConfig, UrlPolicy};
use vernal_context::{
    ApplicationModule, ApplicationModuleError, ApplicationModuleRegistrar, VernalApplicationBuilder,
};
use vernal_ioc::ComponentDefinition;

type BridgeFactoryError = Box<dyn Error + Send + Sync + 'static>;

/// 将 Hutool-Rust HTTP 配置与客户端装入 Vernal 的组件包。
///
/// 组件包属于消费方集成层：Hutool-Rust 继续负责安全、有限内存的 HTTP 工具，
/// Vernal 只负责组件定义、依赖图、作用域和应用上下文。默认 URL 策略与
/// `hutool_http::HttpClient` 一致，允许全部合法 HTTP/HTTPS 地址；生产应用可显式
/// 选择 [`Self::deny_local_targets`] 或注入自己的 [`UrlPolicy`]。
#[derive(Clone)]
pub struct HutoolHttpComponents {
    config: HttpConfig,
    url_policy: Arc<dyn UrlPolicy>,
}

impl HutoolHttpComponents {
    /// 使用完整 Hutool HTTP 配置创建组件包。
    #[must_use]
    pub fn new(config: HttpConfig) -> Self {
        Self {
            config,
            url_policy: Arc::new(AllowAllUrls),
        }
    }

    /// 拒绝回环、私网、链路本地、多播和未指定的字面量 IP 地址。
    ///
    /// 该策略在网络 I/O 前执行，适合处理用户可控 URL 的服务端应用。域名解析后的
    /// DNS rebinding 防护仍应由应用提供环境感知的自定义策略。
    #[must_use]
    pub fn deny_local_targets(mut self) -> Self {
        self.url_policy = Arc::new(DenyLocalTargets);
        self
    }

    /// 使用应用自定义的 URL/SSRF 策略。
    #[must_use]
    pub fn with_url_policy<P>(mut self, policy: P) -> Self
    where
        P: UrlPolicy + 'static,
    {
        self.url_policy = Arc::new(policy);
        self
    }

    /// 使用应用已经共享的 URL/SSRF 策略。
    #[must_use]
    pub fn with_url_policy_arc(mut self, policy: Arc<dyn UrlPolicy>) -> Self {
        self.url_policy = policy;
        self
    }

    /// 返回将进入 Vernal Context 的 HTTP 配置。
    #[must_use]
    pub const fn config(&self) -> &HttpConfig {
        &self.config
    }

    /// 生成 `HttpConfig -> HttpClient` 两个显式组件定义。
    ///
    /// 配置和客户端都使用每个 Container 独立的 Singleton。客户端工厂只能解析
    /// 已声明的 `HttpConfig` 依赖；Reqwest/Rustls 构造错误会被 Vernal 保留为
    /// [`vernal_ioc::ResolveError::Construction`] 的原始 source。
    #[must_use]
    pub fn definitions(&self) -> [ComponentDefinition; 2] {
        let config = self.config.clone();
        let url_policy = Arc::clone(&self.url_policy);

        // 配置通过工厂克隆进入具体 Container，避免多个 Container 共享同一实例
        // 身份，同时仍保留 HttpConfig 内部证书和拦截器的既有 Arc 共享语义。
        let config_definition =
            ComponentDefinition::singleton::<HttpConfig, _>(move |_| config.clone());

        // HttpClient 的真实构造延迟到首次解析，并把显式配置依赖交给 Vernal 图校验。
        let client_definition =
            ComponentDefinition::try_singleton::<HttpClient, _>(move |resolver| {
                let config = resolver
                    .resolve::<HttpConfig>()
                    .map_err(|source| Box::new(source) as BridgeFactoryError)?;
                HttpClient::builder()
                    .with_config(config.as_ref().clone())
                    .url_policy_arc(Arc::clone(&url_policy))
                    .build()
                    .map_err(|source| Box::new(source) as BridgeFactoryError)
            })
            .depends_on::<HttpConfig>();

        [config_definition, client_definition]
    }

    /// 将完整组件包原子安装到高层 Vernal 应用建造器。
    ///
    /// # Errors
    ///
    /// 模块身份重复，或应用已注册 `HttpConfig`/`HttpClient` 时返回
    /// [`ApplicationModuleError`]。Vernal 会在修改真实建造器前校验模块身份与整个
    /// Definition 批次，因此失败不会留下半个组件包或占用未成功提交的模块名。
    pub fn install<'a>(
        &self,
        application: &'a mut VernalApplicationBuilder,
    ) -> Result<&'a mut VernalApplicationBuilder, ApplicationModuleError> {
        application.register_module(self.clone())
    }
}

impl ApplicationModule for HutoolHttpComponents {
    /// 返回 HTTP 组件包的稳定模块身份。
    fn name(&self) -> &'static str {
        "hutool.http"
    }

    /// 在隔离 Registrar 中暂存 `HttpConfig` 与 `HttpClient` 定义。
    fn configure(
        self,
        registrar: &mut ApplicationModuleRegistrar,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        registrar.register_all(self.definitions());
        Ok(())
    }
}

impl Default for HutoolHttpComponents {
    fn default() -> Self {
        Self::new(HttpConfig::default())
    }
}
