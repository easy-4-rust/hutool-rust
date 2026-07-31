//! 对齐: `cn.hutool.aop.aspect.SimpleAspect`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/aspect/SimpleAspect.java
//! 中文说明: 简单切面实现，三个回调均允许正常处理（默认放行）。

use super::aspect::Aspect;

/// 对齐: `cn.hutool.aop.aspect.SimpleAspect`
/// 中文说明: 简单切面，所有回调均返回默认值（放行），不做任何拦截处理。
#[derive(Debug, Default, Clone, Copy)]
pub struct SimpleAspect;

impl<T, A, R, E> Aspect<T, A, R, E> for SimpleAspect {}
