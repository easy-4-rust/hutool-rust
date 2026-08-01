//! 对齐: `cn.hutool.core.bean.copier.AbsCopier`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/AbsCopier.java
//!
//! 中文说明: 抽象的对象拷贝封装，提供来源对象、目标对象持有。
//! Java 侧 `AbsCopier<S, T>` 是抽象类，持有 `source`、`target`、`copyOptions`，
//! 子类实现 `copy()` 方法。Rust 中映射为 trait，由各具体拷贝器实现。

use super::copy_options::CopyOptions;

/// 对齐 Java abstract class: `cn.hutool.core.bean.copier.AbsCopier<S, T>`
///
/// 中文说明: 抽象拷贝器接口，所有具体拷贝器都实现此 trait。
/// `copy` 方法执行实际的属性拷贝操作，返回目标对象。
///
/// Java 中 `AbsCopier` 实现了 `Copier<T>` 接口，Rust 中直接作为 trait 定义。
pub trait AbsCopier<T> {
    /// 对齐 Java: `T copy()`
    ///
    /// 中文说明: 执行拷贝操作，将来源对象的属性复制到目标对象，返回目标对象。
    fn copy(&self) -> T;

    /// 对齐 Java: 获取拷贝选项引用
    ///
    /// 中文说明: 获取此拷贝器使用的拷贝选项。
    fn copy_options(&self) -> &CopyOptions;
}

/// 通用辅助函数：判断一个 serde_json::Value 是否为"空值"（null 或缺失）
///
/// 对齐 Java 语义：`ObjectUtil.isEmpty` / `value == null`
#[allow(dead_code)] // 对齐 Java ObjectUtil.isEmpty，供后续拷贝流程接线使用，暂未调用
pub(crate) fn is_empty_value(v: &serde_json::Value) -> bool {
    v.is_null()
}

/// 通用辅助函数：将 Value 转为 camelCase 字段名对应的目标类型表示
///
/// 对齐 Java: CopyOptions 内部的 `convertField` + `editFieldValue` 组合
#[allow(dead_code)] // 对齐 Java CopyOptions 内部流程，预留供后续接线使用
pub(crate) fn apply_value_transform(
    copy_options: &CopyOptions,
    field_name: &str,
    value: &serde_json::Value,
) -> serde_json::Value {
    let converted = copy_options.convert_field("", value);
    copy_options.edit_field_value(field_name, &converted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn is_empty_value_null() {
        assert!(is_empty_value(&Value::Null));
        assert!(!is_empty_value(&Value::Bool(false)));
        assert!(!is_empty_value(&Value::String(String::new())));
    }

    #[test]
    fn apply_value_transform_identity() {
        let opts = CopyOptions::create();
        let v = Value::Number(42.into());
        assert_eq!(apply_value_transform(&opts, "age", &v), v);
    }
}
