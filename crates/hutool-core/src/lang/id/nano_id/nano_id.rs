//! 对齐: `cn.hutool.core.lang.id.NanoId`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/id/NanoId.java

use rand::RngCore;

/// 对齐 Java: `cn.hutool.core.lang.id.NanoId`
#[derive(Debug, Clone, Copy, Default)]
pub struct NanoId;

use super::{DEFAULT_ALPHABET, DEFAULT_SIZE};
