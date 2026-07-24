//! 对齐: `cn.hutool.core.lang.id.NanoId`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/id/NanoId.java

use rand::RngCore;

mod nano_id;
mod java_random;

pub use nano_id::NanoId;
pub use java_random::JavaRandom;

pub const DEFAULT_SIZE: usize = 21;

const DEFAULT_ALPHABET: &[u8] =
    b"_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
