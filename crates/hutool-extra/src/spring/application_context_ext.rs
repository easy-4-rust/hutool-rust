//! Spring POJO facade，对齐 hutool 的 `cn.hutool.extra.spring.*`。
//!
//! **仅提供 trait 抽象**。具体 Spring Framework 依赖（ApplicationContext / `BeanFactory`）
//! 是 Java-only，属于 unsafe-to-copy。Rust 用户应使用依赖注入框架（如 `axum::Extension`、
//! shaku、self-rs 等）替代。

use super::application_context::ApplicationContext;

/// `ApplicationContext` 扩展 trait，提供 hutool-rs 独有的便捷方法。
pub trait ApplicationContextExt: ApplicationContext {
    /// `get_property_or：返回配置或默认值`
    fn get_property_or(&self, key: &str) -> Option<String> {
        // 默认从 BeanFactory 读取；具体实现可覆盖
        self.get_bean_by_name(&format!("__property_{key}"))
            .and_then(|v| v.downcast_ref::<String>().cloned())
    }
}

impl<T: ApplicationContext + ?Sized> ApplicationContextExt for T {}
