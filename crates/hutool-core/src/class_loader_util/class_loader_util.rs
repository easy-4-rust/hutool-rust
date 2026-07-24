//! 对齐: `cn.hutool.core.util.ClassLoaderUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ClassLoaderUtil.java
//!
//! Rust 侧对齐 Java 类名规范化（内部类 `.` → `$`）与 `Class.getName()` 语义。

use crate::{CoreError, Result};

use super::loaded_class::LoadedClass;

/// 对齐 Java: `cn.hutool.core.util.ClassLoaderUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ClassLoaderUtil;

impl ClassLoaderUtil {
    /// 对齐 Java: `ClassLoaderUtil.loadClass(String)`
    pub fn load_class(name: &str) -> Result<LoadedClass> {
        Self::load_class_with_init(name, true)
    }

    /// 对齐 Java: `ClassLoaderUtil.loadClass(String, boolean)`
    pub fn load_class_with_init(name: &str, _is_initialized: bool) -> Result<LoadedClass> {
        let normalized = normalize_class_name(name)?;
        Ok(LoadedClass { name: normalized })
    }
}

fn normalize_class_name(name: &str) -> Result<String> {
    let name = name.trim().replace('/', ".");
    if name.is_empty() {
        return Err(CoreError::InvalidArgument {
            name: "className",
            reason: "must not be blank",
        });
    }

    if name.contains('$') {
        return Ok(name);
    }

    if let Some(canonical) = known_class_aliases().get(name.as_str()) {
        return Ok(canonical.to_string());
    }

    if let Some(idx) = name.rfind('.') {
        if idx > 0 {
            let inner = format!("{}${}", &name[..idx], &name[idx + 1..]);
            if known_class_aliases().contains_key(inner.as_str()) || inner.contains('$') {
                return Ok(inner);
            }
        }
    }

    Ok(name)
}
