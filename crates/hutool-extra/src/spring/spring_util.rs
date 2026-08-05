//! Spring POJO facade，对齐 hutool 的 `cn.hutool.extra.spring.*`。
//!
//! **仅提供 trait 抽象**。具体 Spring Framework 依赖（ApplicationContext / BeanFactory）
//! 是 Java-only，属于 unsafe-to-copy。Rust 用户应使用依赖注入框架（如 axum::Extension、
//! shaku、self-rs 等）替代。

use super::application_context_ext::ApplicationContextExt;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use crate::HutoolException;

use super::application_context::ApplicationContext;
use super::configurable_bean_factory::ConfigurableBeanFactory;

/// SpringUtil 工具类（Java 版是 BeanFactoryPostProcessor + ApplicationContextAware），
/// 对齐 `cn.hutool.extra.spring.SpringUtil`。
///
/// Rust 版用 trait + 全局 OnceLock 持有 `dyn ApplicationContext`，避免依赖 Spring。
pub struct SpringUtil;

impl SpringUtil {
    /// 对齐 `SpringUtil.setApplicationContext(ApplicationContext)`
    pub fn set_application_context(
        ctx: Arc<dyn ApplicationContext>,
    ) -> Result<(), HutoolException> {
        APPLICATION_CONTEXT.set(ctx).map_err(|_| {
            HutoolException::Message("SpringUtil: ApplicationContext already set".into())
        })
    }

    /// 对齐 `SpringUtil.getApplicationContext()`
    pub fn get_application_context() -> Option<Arc<dyn ApplicationContext>> {
        APPLICATION_CONTEXT.get().cloned()
    }

    /// 对齐 `SpringUtil.getBeanFactory()`
    pub fn get_bean_factory() -> Result<Arc<dyn ApplicationContext>, HutoolException> {
        Self::get_application_context().ok_or_else(|| {
            HutoolException::Message(
                "No ConfigurableListableBeanFactory or ApplicationContext injected, maybe not in the Spring environment?".into(),
            )
        })
    }

    /// 对齐 `SpringUtil.getBean(Class)`：按类型获取 Bean
    pub fn get_bean(type_name: &str) -> Result<Arc<dyn Any>, HutoolException> {
        let ctx = Self::get_bean_factory()?;
        ctx.get_bean(type_name)
            .ok_or_else(|| HutoolException::Message(format!("No bean of type: {}", type_name)))
    }

    /// 对齐 `SpringUtil.getBean(String)`：按名称获取 Bean
    pub fn get_bean_by_name(name: &str) -> Result<Arc<dyn Any>, HutoolException> {
        let ctx = Self::get_bean_factory()?;
        ctx.get_bean_by_name(name)
            .ok_or_else(|| HutoolException::Message(format!("No bean named: {}", name)))
    }

    /// 对齐 `SpringUtil.getBeansOfType(Class)`
    pub fn get_beans_of_type(
        type_name: &str,
    ) -> Result<HashMap<String, Arc<dyn Any>>, HutoolException> {
        Ok(Self::get_bean_factory()?.get_beans_of_type(type_name))
    }

    /// 对齐 `SpringUtil.getBeanNamesForType(Class)`
    pub fn get_bean_names_for_type(type_name: &str) -> Result<Vec<String>, HutoolException> {
        Ok(Self::get_bean_factory()?.get_bean_names_for_type(type_name))
    }

    /// 对齐 `SpringUtil.getApplicationName()`
    pub fn get_application_name() -> Result<String, HutoolException> {
        Ok(Self::get_bean_factory()?.get_application_name().to_string())
    }

    /// 对齐 `SpringUtil.getActiveProfiles()`
    pub fn get_active_profiles() -> Result<Vec<String>, HutoolException> {
        Ok(Self::get_bean_factory()?.get_active_profiles())
    }

    /// 对齐 `SpringUtil.getActiveProfile()`：取第一个
    pub fn get_active_profile() -> Result<String, HutoolException> {
        let profiles = Self::get_active_profiles()?;
        profiles
            .into_iter()
            .next()
            .ok_or_else(|| HutoolException::Message("No active profile set".into()))
    }

    /// 对齐 `SpringUtil.registerBean(String, Object)`
    pub fn register_bean(
        _name: &str,
        _bean: Arc<dyn Any + Send + Sync>,
    ) -> Result<(), HutoolException> {
        Err(HutoolException::Message(
            "SpringUtil::register_bean requires a mutable ConfigurableBeanFactory; consider using ApplicationContext directly".into(),
        ))
    }

    /// 对齐 `SpringUtil.unregisterBean(String)`
    pub fn unregister_bean(_name: &str) -> Result<(), HutoolException> {
        Err(HutoolException::Message(
            "SpringUtil::unregister_bean requires a mutable ConfigurableBeanFactory; consider using ApplicationContext directly".into(),
        ))
    }

    /// 对齐 `SpringUtil.publishEvent(Object)`
    pub fn publish_event(event: Arc<dyn Any + Send + Sync>) -> Result<(), HutoolException> {
        let ctx = Self::get_bean_factory()?;
        ctx.publish_event(event);
        Ok(())
    }

    /// 对齐 `SpringUtil.getProperty(String)`：读取配置（委托到 ApplicationContext）
    pub fn get_property(key: &str) -> Result<String, HutoolException> {
        let ctx = Self::get_bean_factory()?;
        ctx.get_property_or(key)
            .ok_or_else(|| HutoolException::Message(format!("No property: {}", key)))
    }

    /// 对齐 `SpringUtil.postProcessBeanFactory(ConfigurableListableBeanFactory)`
    ///
    /// Rust 版无 Spring 生命周期回调；此方法返回 Err 提示用户应直接调用 `set_application_context`。
    pub fn post_process_bean_factory(
        _factory: &dyn ConfigurableBeanFactory,
    ) -> Result<(), HutoolException> {
        Err(HutoolException::Message(
            "SpringUtil::post_process_bean_factory is Spring-only; call set_application_context instead".into(),
        ))
    }
}

use super::APPLICATION_CONTEXT;
