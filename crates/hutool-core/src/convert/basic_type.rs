//! 对齐: `cn.hutool.core.convert.BasicType`

#![allow(dead_code)]

/// 对齐 Java 类: `cn.hutool.core.convert.BasicType`
#[derive(Debug, Clone, Default)]
pub struct BasicType;

impl BasicType {
    /// 兼容 sentinel。
    pub fn pending_alignment() -> &'static str {
        "pending"
    }

    /// 对齐 Java: `BasicType.wrap(Class)` — 以类型名字符串表达
    pub fn wrap(type_name: &str) -> &'static str {
        match type_name {
            "int" | "Integer" => "Integer",
            "long" | "Long" => "Long",
            "boolean" | "Boolean" => "Boolean",
            "byte" | "Byte" => "Byte",
            "short" | "Short" => "Short",
            "char" | "Character" => "Character",
            "float" | "Float" => "Float",
            "double" | "Double" => "Double",
            "void" | "Void" => "Void",
            _ => "Object",
        }
    }

    /// 对齐 Java: `BasicType.unWrap(Class)`
    pub fn un_wrap(type_name: &str) -> &'static str {
        match type_name {
            "Integer" | "int" => "int",
            "Long" | "long" => "long",
            "Boolean" | "boolean" => "boolean",
            "Byte" | "byte" => "byte",
            "Short" | "short" => "short",
            "Character" | "char" => "char",
            "Float" | "float" => "float",
            "Double" | "double" => "double",
            "Void" | "void" => "void",
            _ => "Object",
        }
    }
}
