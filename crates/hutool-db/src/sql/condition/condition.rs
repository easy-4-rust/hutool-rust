//! 条件对象 —— 对齐 Hutool `cn.hutool.db.sql.Condition`。

use crate::sql::logical_operator::LogicalOperator;
use rust_decimal::Decimal;
use serde_json::Value;
use std::fmt::Write as _;

use super::like_type::LikeType;

/// SQL WHERE 条件 —— 对齐 Hutool `Condition`。
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    field: String,
    operator: String,
    value: Value,
    second_value: Option<Value>,
    place_holder: bool,
    link_operator: LogicalOperator,
}

impl Condition {
    const OPERATOR_LIKE: &'static str = "LIKE";
    const OPERATOR_IN: &'static str = "IN";
    const OPERATOR_IS: &'static str = "IS";
    const OPERATOR_IS_NOT: &'static str = "IS NOT";
    const OPERATOR_BETWEEN: &'static str = "BETWEEN";
    const VALUE_NULL: &'static str = "NULL";

    /// 对齐 Java: `Condition.parse(String, Object)`.
    #[must_use]
    pub fn parse(field: impl Into<String>, expression: impl Into<Value>) -> Self {
        Self::new(field, expression)
    }

    /// 对齐 Java: `Condition(String field, Object value)`.
    #[must_use]
    pub fn new(field: impl Into<String>, value: impl Into<Value>) -> Self {
        let mut condition = Self {
            field: field.into(),
            operator: "=".to_string(),
            value: value.into(),
            second_value: None,
            place_holder: true,
            link_operator: LogicalOperator::And,
        };
        condition.parse_value();
        condition
    }

    /// 对齐 Java: `Condition(String field, String operator, Object value)`.
    #[must_use]
    pub fn with_operator(
        field: impl Into<String>,
        operator: impl Into<String>,
        value: impl Into<Value>,
    ) -> Self {
        Self {
            field: field.into(),
            operator: operator.into(),
            value: value.into(),
            second_value: None,
            place_holder: true,
            link_operator: LogicalOperator::And,
        }
    }

    /// 对齐 Java: `Condition(String field, String value, LikeType likeType)`.
    #[must_use]
    pub fn like(field: impl Into<String>, value: impl Into<String>, like_type: LikeType) -> Self {
        let value = value.into();
        Self {
            field: field.into(),
            operator: Self::OPERATOR_LIKE.to_string(),
            value: Value::String(crate::sql::sql_util::build_like_value(
                &value, like_type, false,
            )),
            second_value: None,
            place_holder: true,
            link_operator: LogicalOperator::And,
        }
    }

    /// 对齐 Java: BETWEEN 构造。
    #[must_use]
    pub fn between(
        field: impl Into<String>,
        left: impl Into<Value>,
        right: impl Into<Value>,
    ) -> Self {
        Self {
            field: field.into(),
            operator: Self::OPERATOR_BETWEEN.to_string(),
            value: left.into(),
            second_value: Some(right.into()),
            place_holder: true,
            link_operator: LogicalOperator::And,
        }
    }

    /// 对齐 Java: `Condition.setPlaceHolder(boolean)`.
    pub fn set_place_holder(&mut self, place_holder: bool) -> &mut Self {
        self.place_holder = place_holder;
        self
    }

    /// 对齐 Java: `Condition.setLinkOperator(LogicalOperator)`.
    pub fn set_link_operator(&mut self, link_operator: LogicalOperator) -> &mut Self {
        self.link_operator = link_operator;
        self
    }

    /// 对齐 Java: `Condition.getLinkOperator()`.
    #[must_use]
    pub fn link_operator(&self) -> LogicalOperator {
        self.link_operator
    }

    /// 对齐 Java: `Condition.getField()`.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// 设置字段名（Wrapper 包装时使用）。
    pub fn set_field(&mut self, field: impl Into<String>) {
        self.field = field.into();
    }

    /// 对齐 Java: `Condition.getValue()`.
    #[must_use]
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// 对齐 Java: `Condition.isPlaceHolder()`.
    #[must_use]
    pub fn is_place_holder(&self) -> bool {
        self.place_holder
    }

    /// 对齐 Java: `Condition.toString(List<Object> paramValues)`.
    pub fn to_sql(&self, param_values: &mut Vec<Value>) -> String {
        let mut condition = self.clone();
        condition.check_value_null();
        let mut out = String::new();
        write!(out, "{} {}", condition.field, condition.operator).unwrap();

        if condition.is_operator_between() {
            condition.build_between(&mut out, param_values);
        } else if condition.is_operator_in() {
            condition.build_in(&mut out, param_values);
        } else if condition.place_holder
            && !condition.is_operator_is()
            && !condition.is_operator_is_not()
        {
            out.push_str(" ?");
            param_values.push(condition.value.clone());
        } else {
            out.push(' ');
            let value_str = condition.value_to_literal();
            if condition.is_operator_like() {
                out.push('\'');
                out.push_str(&value_str);
                out.push('\'');
            } else {
                out.push_str(&value_str);
            }
        }
        out
    }

    fn check_value_null(&mut self) {
        if self.value.is_null() {
            if self.operator == "!=" || self.operator == "<>" {
                self.operator = Self::OPERATOR_IS_NOT.to_string();
            } else {
                self.operator = Self::OPERATOR_IS.to_string();
            }
            self.value = Value::String(Self::VALUE_NULL.to_string());
        }
    }

    fn is_operator_between(&self) -> bool {
        self.operator.eq_ignore_ascii_case(Self::OPERATOR_BETWEEN)
    }

    fn is_operator_in(&self) -> bool {
        self.operator.eq_ignore_ascii_case(Self::OPERATOR_IN)
    }

    fn is_operator_is(&self) -> bool {
        self.operator.eq_ignore_ascii_case(Self::OPERATOR_IS)
    }

    fn is_operator_is_not(&self) -> bool {
        self.operator.eq_ignore_ascii_case(Self::OPERATOR_IS_NOT)
    }

    fn is_operator_like(&self) -> bool {
        self.operator.eq_ignore_ascii_case(Self::OPERATOR_LIKE)
    }

    fn build_between(&self, out: &mut String, param_values: &mut Vec<Value>) {
        if self.place_holder {
            out.push_str(" ?");
            param_values.push(self.value.clone());
        } else {
            out.push(' ');
            out.push_str(&self.value_to_literal());
        }
        out.push_str(" AND");
        if self.place_holder {
            out.push_str(" ?");
            param_values.push(self.second_value.clone().unwrap_or(Value::Null));
        } else {
            out.push(' ');
            out.push_str(
                &self
                    .second_value
                    .as_ref()
                    .map(|v| json_scalar(v))
                    .unwrap_or_default(),
            );
        }
    }

    fn build_in(&self, out: &mut String, param_values: &mut Vec<Value>) {
        out.push_str(" (");
        if self.place_holder {
            let values = in_values(&self.value);
            out.push_str(
                &std::iter::repeat("?")
                    .take(values.len())
                    .collect::<Vec<_>>()
                    .join(","),
            );
            param_values.extend(values);
        } else {
            out.push_str(&in_literal(&self.value));
        }
        out.push(')');
    }

    fn value_to_literal(&self) -> String {
        json_scalar(&self.value)
    }

    fn parse_value(&mut self) {
        if self.value.is_null() {
            self.operator = Self::OPERATOR_IS.to_string();
            self.value = Value::String(Self::VALUE_NULL.to_string());
            return;
        }

        if self.value.is_array() {
            self.operator = Self::OPERATOR_IN.to_string();
            return;
        }

        let Value::String(value_str) = self.value.clone() else {
            return;
        };
        if value_str.trim().is_empty() {
            return;
        }
        let value_str = value_str.trim().to_string();

        if value_str.eq_ignore_ascii_case("= null") || value_str.eq_ignore_ascii_case("is null") {
            self.operator = Self::OPERATOR_IS.to_string();
            self.value = Value::String(Self::VALUE_NULL.to_string());
            self.place_holder = false;
            return;
        }
        if value_str.eq_ignore_ascii_case("!= null")
            || value_str.eq_ignore_ascii_case("is not null")
        {
            self.operator = Self::OPERATOR_IS_NOT.to_string();
            self.value = Value::String(Self::VALUE_NULL.to_string());
            self.place_holder = false;
            return;
        }

        let Some((first, rest)) = split_once_space(&value_str) else {
            return;
        };
        let first_upper = first.to_ascii_uppercase();

        if matches!(
            first_upper.as_str(),
            "<>" | "<=" | "<" | ">=" | ">" | "=" | "!=" | "IN"
        ) {
            self.operator = first_upper;
            self.value = if self.is_operator_in() {
                Value::String(rest.to_string())
            } else {
                try_to_number(rest)
            };
            return;
        }

        if first_upper == Self::OPERATOR_LIKE {
            self.operator = Self::OPERATOR_LIKE.to_string();
            self.value = Value::String(unwrap_quote(rest));
            return;
        }

        if first_upper == Self::OPERATOR_BETWEEN {
            let parts: Vec<_> = rest
                .split(" AND ")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .collect();
            if parts.len() >= 2 {
                self.operator = Self::OPERATOR_BETWEEN.to_string();
                self.value = Value::String(unwrap_quote(parts[0]));
                self.second_value = Some(Value::String(unwrap_quote(parts[1])));
            }
        }
    }
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_sql(&mut Vec::new()))
    }
}

fn json_scalar(value: &Value) -> String {
    match value {
        Value::Null => "NULL".to_string(),
        Value::Bool(v) => v.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn split_once_space(input: &str) -> Option<(&str, &str)> {
    input
        .find(' ')
        .map(|idx| (&input[..idx], input[idx + 1..].trim()))
}

fn in_literal(value: &Value) -> String {
    in_values(value)
        .into_iter()
        .map(|v| json_scalar(&v))
        .collect::<Vec<_>>()
        .join(",")
}

fn try_to_number(value: &str) -> Value {
    let value = value.trim();
    if let Ok(v) = value.parse::<i64>() {
        return Value::Number(v.into());
    }
    if let Ok(v) = value.parse::<Decimal>() {
        return Value::String(v.to_string());
    }
    Value::String(value.to_string())
}

fn in_values(value: &Value) -> Vec<Value> {
    match value {
        Value::Array(items) => items.clone(),
        Value::String(s) => s
            .split(',')
            .map(|part| try_to_number(part.trim()))
            .collect(),
        _ => vec![value.clone()],
    }
}

fn unwrap_quote(value: &str) -> String {
    let value = value.trim();
    if value.len() >= 2 {
        let start = value.as_bytes()[0];
        let end = value.as_bytes()[value.len() - 1];
        if (start == b'\'' && end == b'\'') || (start == b'"' && end == b'"') {
            return value[1..value.len() - 1].to_string();
        }
    }
    value.to_string()
}
