//! `cn.hutool.ai.core` 包对齐。
//!
//! 按照 Java 1:1 子包路径组织：
//! - [`ai_config`] - 配置接口；
//! - [`base_config`] - 默认配置实现；
//! - [`ai_config_builder`] - 链式 builder；
//! - [`ai_service`] - 异步服务 trait；
//! - [`ai_service_provider`] - SPI 注册入口；
//! - [`provider_service`] - 通用 reqwest 实现，承载 Hutool 全部能力。

pub mod ai_config;
pub mod ai_config_builder;
pub mod ai_service;
pub mod ai_service_provider;
pub mod base_config;
pub mod provider_service;

pub use ai_config::AIConfig;
pub use ai_config_builder::AIConfigBuilder;
pub use ai_service::AIService;
pub use ai_service_provider::AIServiceProvider;
pub use base_config::BaseConfig;
pub use provider_service::ProviderService;
