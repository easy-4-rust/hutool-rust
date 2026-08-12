//! Spring POJO facade，对齐 hutool 的 `cn.hutool.extra.spring.*`。
//!
//! **仅提供 trait 抽象**。具体 Spring Framework 依赖（ApplicationContext / `BeanFactory`）
//! 是 Java-only，属于 unsafe-to-copy。Rust 用户应使用依赖注入框架（如 `axum::Extension`、
//! shaku、self-rs 等）替代。

use std::sync::{Arc, OnceLock};

mod application_context;
mod application_context_ext;
mod configurable_bean_factory;
mod spring_util;

pub use application_context::ApplicationContext;
pub use application_context_ext::ApplicationContextExt;
pub use configurable_bean_factory::ConfigurableBeanFactory;
pub use spring_util::SpringUtil;

static APPLICATION_CONTEXT: OnceLock<Arc<dyn ApplicationContext>> = OnceLock::new();

/// 启用 SpringUtil（占位）：Rust 中用户须显式调用 [`SpringUtil::set_application_context`]。
pub fn enable_spring_util() {
    // 占位函数：在 Spring 中由 @EnableSpringUtil 注解触发，
    // 在 Rust 中用户必须显式 set_application_context。
}
