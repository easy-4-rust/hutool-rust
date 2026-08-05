//! 对齐: `cn.hutool.core.lang.id.NanoId`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/id/NanoId.java

#![allow(dead_code)] // 对齐 Java NanoId，暂未接线，预留

mod java_random;
mod nano_id;

pub const DEFAULT_SIZE: usize = 21;

const DEFAULT_ALPHABET: &[u8] = b"_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
