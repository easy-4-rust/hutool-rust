//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

use crate::{generate_random_key_bytes, CryptoError};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use num_bigint::BigUint;

mod key_spec_bytes;
mod pbe_key_spec;
mod pbe_parameter_spec;
mod rsa_private_crt_key_spec;
mod spec_util;

pub use key_spec_bytes::KeySpecBytes;
pub use pbe_key_spec::PbeKeySpec;
pub use pbe_parameter_spec::PbeParameterSpec;
pub use rsa_private_crt_key_spec::RsaPrivateCrtKeySpec;
pub use spec_util::SpecUtil;
