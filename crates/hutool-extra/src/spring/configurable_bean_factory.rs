//! Spring POJO facade，对齐 hutool 的 `cn.hutool.extra.spring.*`。
//!
//! **仅提供 trait 抽象**。具体 Spring Framework 依赖（ApplicationContext / `BeanFactory`）
//! 是 Java-only，属于 unsafe-to-copy。Rust 用户应使用依赖注入框架（如 `axum::Extension`、
//! shaku、self-rs 等）替代。

use std::any::Any;
use std::sync::Arc;

/// Bean 注册接口，对齐 `org.springframework.beans.factory.config.ConfigurableListableBeanFactory`。
pub trait ConfigurableBeanFactory: Send + Sync {
    /// 对齐 `ConfigurableListableBeanFactory.registerSingleton(String, Object)`
    fn register_bean(&mut self, name: &str, bean: Arc<dyn Any + Send + Sync>) -> bool;

    /// 对齐 `ConfigurableListableBeanFactory.destroyBean(String)` 或 unregister
    fn unregister_bean(&mut self, name: &str) -> bool;

    /// 对齐 `ConfigurableListableBeanFactory.getProperty(String)`：读取配置
    fn get_property(&self, key: &str) -> Option<String>;
}
