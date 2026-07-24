//! Spring POJO facade，对齐 hutool 的 `cn.hutool.extra.spring.*`。
//!
//! **仅提供 trait 抽象**。具体 Spring Framework 依赖（ApplicationContext / BeanFactory）
//! 是 Java-only，属于 unsafe-to-copy。Rust 用户应使用依赖注入框架（如 axum::Extension、
//! shaku、self-rs 等）替代。

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use crate::HutoolException;

/// Spring 应用上下文 trait，对齐 `org.springframework.context.ApplicationContext`。
///
/// Rust 没有原生 Spring，此 trait 用作"通用 IoC 容器"抽象。
/// 用户可以提供自己的实现（基于任何 Rust DI 框架）。
pub trait ApplicationContext: Send + Sync {
    /// 对齐 `ApplicationContext.getBean(Class)`：按类型获取 Bean
    fn get_bean(&self, type_name: &str) -> Option<Arc<dyn Any>>;

    /// 对齐 `ApplicationContext.getBean(String)`：按名称获取 Bean
    fn get_bean_by_name(&self, name: &str) -> Option<Arc<dyn Any>>;

    /// 对齐 `ApplicationContext.getBeansOfType(Class)`
    fn get_beans_of_type(&self, type_name: &str) -> HashMap<String, Arc<dyn Any>>;

    /// 对齐 `ApplicationContext.getBeanNamesForType(Class)`
    fn get_bean_names_for_type(&self, type_name: &str) -> Vec<String>;

    /// 对齐 `ApplicationContext.getApplicationName()`
    fn get_application_name(&self) -> &str;

    /// 对齐 `ApplicationContext.getActiveProfiles()`
    fn get_active_profiles(&self) -> Vec<String>;

    /// 对齐 `ApplicationContext.containsBean(String)`
    fn contains_bean(&self, name: &str) -> bool;

    /// 对齐 `ApplicationContext.publishEvent(Object)`：发布事件
    fn publish_event(&self, event: Arc<dyn Any + Send + Sync>);
}

use super::{APPLICATION_CONTEXT, enable_spring_util};
