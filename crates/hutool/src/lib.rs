//! Feature-gated facade for the `HiTool` workspace.
//!
//! Default features include only `core` and `json`. Applications can disable
//! defaults and opt into individual Hutool-aligned capability crates.

#![forbid(unsafe_code)]

use std::{collections::BTreeSet, io};

#[cfg(feature = "ai")]
pub use hutool_ai as ai;
#[cfg(feature = "annotation")]
pub use hutool_annotation as annotation;
#[cfg(feature = "aop")]
pub use hutool_aop as aop;
#[cfg(feature = "bloom-filter")]
pub use hutool_bloom_filter as bloom_filter;
#[cfg(feature = "cache")]
pub use hutool_cache as cache;
#[cfg(feature = "captcha")]
pub use hutool_captcha as captcha;
#[cfg(feature = "hutool-compat")]
pub use hutool_compat_hutool as compat;
#[cfg(feature = "core")]
pub use hutool_core as core;
#[cfg(feature = "cron")]
pub use hutool_cron as cron;
#[cfg(feature = "crypto")]
pub use hutool_crypto as crypto;
#[cfg(feature = "db")]
pub use hutool_db as db;
#[cfg(feature = "dfa")]
pub use hutool_dfa as dfa;
#[cfg(feature = "extra")]
pub use hutool_extra as extra;
#[cfg(feature = "http")]
pub use hutool_http as http;
#[cfg(feature = "json")]
pub use hutool_json as json;
#[cfg(feature = "jwt")]
pub use hutool_jwt as jwt;
#[cfg(feature = "log")]
pub use hutool_log as log;
#[cfg(feature = "observability")]
pub use hutool_observability as observability;
#[cfg(feature = "script")]
pub use hutool_script as script;
#[cfg(feature = "setting")]
pub use hutool_setting as setting;
#[cfg(feature = "socket")]
pub use hutool_socket as socket;
#[cfg(feature = "system")]
pub use hutool_system as system;
#[cfg(feature = "vernal")]
pub use hutool_vernal as vernal;

/// Common extension traits and data types enabled by facade features.
pub mod prelude {
    #[cfg(feature = "core")]
    pub use hutool_core::prelude::*;
    #[cfg(feature = "json")]
    pub use hutool_json::prelude::*;
}

/// Compile-time view of the `HiTool` capabilities enabled for this build.
///
/// This is the Rust counterpart of Hutool's classpath-scanning `Hutool`
/// helper. Cargo features provide a deterministic registry without runtime
/// reflection or filesystem scanning.
#[derive(Debug, Clone, Copy, Default)]
pub struct Hutool;

impl Hutool {
    /// Returns enabled capability crate names in stable lexical order.
    #[must_use]
    pub fn get_all_utils() -> BTreeSet<&'static str> {
        #[allow(unused_mut)]
        let mut modules = BTreeSet::new();
        macro_rules! enabled {
            ($feature:literal, $name:literal) => {
                #[cfg(feature = $feature)]
                modules.insert($name);
            };
        }
        enabled!("ai", "ai");
        enabled!("aop", "aop");
        enabled!("bloom-filter", "bloom_filter");
        enabled!("cache", "cache");
        enabled!("captcha", "captcha");
        enabled!("core", "core");
        enabled!("cron", "cron");
        enabled!("crypto", "crypto");
        enabled!("db", "db");
        enabled!("dfa", "dfa");
        enabled!("extra", "extra");
        enabled!("http", "http");
        enabled!("hutool-compat", "compat");
        enabled!("json", "json");
        enabled!("jwt", "jwt");
        enabled!("log", "log");
        enabled!("observability", "observability");
        enabled!("script", "script");
        enabled!("setting", "setting");
        enabled!("socket", "socket");
        enabled!("system", "system");
        enabled!("vernal", "vernal");
        modules
    }

    /// Writes enabled capability names, one per line, to an injected sink.
    pub fn write_all_utils(writer: &mut dyn io::Write) -> io::Result<()> {
        for module in Self::get_all_utils() {
            writeln!(writer, "{module}")?;
        }
        Ok(())
    }

    /// Prints enabled capability names to standard output.
    pub fn print_all_utils() -> io::Result<()> {
        Self::write_all_utils(&mut io::stdout().lock())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(all(feature = "core", feature = "json"))]
    #[test]
    fn default_facade_exposes_core_and_json() {
        use crate::prelude::*;

        assert!(" \t".is_blank());
        assert_eq!(crate::json::parse(r#"{"ok":true}"#).unwrap()["ok"], true);
    }

    #[test]
    fn hutool_registry_is_feature_aware_stable_and_writable() {
        #[cfg(feature = "core")]
        struct FailingWriter;

        #[cfg(feature = "core")]
        impl std::io::Write for FailingWriter {
            fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("injected writer failure"))
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let modules = super::Hutool::get_all_utils();
        #[cfg(feature = "core")]
        assert!(modules.contains("core"));
        #[cfg(feature = "json")]
        assert!(modules.contains("json"));
        #[cfg(feature = "observability")]
        assert!(modules.contains("observability"));

        let mut output = Vec::new();
        super::Hutool::write_all_utils(&mut output).unwrap();
        let text = String::from_utf8(output).unwrap();
        let lines: Vec<_> = text.lines().collect();
        assert!(lines.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(lines.len(), modules.len());
        #[cfg(feature = "core")]
        {
            let mut failing = FailingWriter;
            std::io::Write::flush(&mut failing).unwrap();
            assert!(super::Hutool::write_all_utils(&mut failing).is_err());
        }
        super::Hutool::print_all_utils().unwrap();
        assert!(format!("{:?}", super::Hutool).contains("Hutool"));
    }
}
