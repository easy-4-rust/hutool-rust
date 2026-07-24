//! 统一实现 hutool-extra 6 个 Exception 类型。
//!
//! 每个 hutool-extra Exception 都是同样的 6 个构造器模式：
//!   - `(Throwable e)` → 包装底层错误
//!   - `(String message)` → 简单消息
//!   - `(String messageTemplate, Object... params)` → 模板格式化
//!   - `(String message, Throwable throwable)` → 消息 + 错误
//!   - `(String message, Throwable throwable, boolean, boolean)` → 完整控制
//!   - `(Throwable throwable, String messageTemplate, Object... params)` → 模板 + 错误
//!
//! 使用宏 `define_hutool_exception!` 批量实现，避免重复代码。

use thiserror::Error;

use super::hutool_exception::HutoolException;

/// 对齐 `cn.hutool.extra.expression.ExpressionException`
pub type ExpressionException = HutoolException;
