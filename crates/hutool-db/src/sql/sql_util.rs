//! SQL 工具 —— 对齐 Hutool `cn.hutool.db.sql.SqlUtil`。

use crate::entity::Entity;
use crate::sql::condition::{Condition, ConditionValue, LikeType};
use regex::Regex;
use std::sync::LazyLock;

static PATTERN_ORDER_BY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(.*)\s+order\s+by\s+[^\s]+").expect("order by pattern"));

static PATTERN_IN_CLAUSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\s+in\s+\(\s*$").expect("in clause pattern"));

/// 对齐 Java: `SqlUtil.removeOuterOrderBy(String)`.
#[must_use]
pub fn remove_outer_order_by(select_sql: &str) -> String {
    PATTERN_ORDER_BY
        .captures(select_sql)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| select_sql.to_string())
}

/// 对齐 Java: `SqlUtil.isInClause(CharSequence)`.
#[must_use]
pub fn is_in_clause(sql: &str) -> bool {
    PATTERN_IN_CLAUSE.is_match(sql)
}

/// 对齐 Java: `SqlUtil.buildLikeValue(String, LikeType, boolean)`.
#[must_use]
pub fn build_like_value(value: &str, like_type: LikeType, with_like_keyword: bool) -> String {
    let mut out = String::new();
    if with_like_keyword {
        out.push_str("LIKE ");
    }
    match like_type {
        LikeType::StartWith => {
            out.push_str(value);
            out.push('%');
        }
        LikeType::EndWith => {
            out.push('%');
            out.push_str(value);
        }
        LikeType::Contains => {
            out.push('%');
            out.push_str(value);
            out.push('%');
        }
    }
    out
}

/// 对齐 Java: `SqlUtil.buildConditions(Entity)`.
#[must_use]
pub fn build_conditions(entity: &Entity) -> Vec<Condition> {
    entity
        .iter_conditions()
        .map(|(field, value)| match value {
            ConditionValue::Condition(condition) => condition.clone(),
            ConditionValue::Raw(raw) => Condition::new(field.clone(), raw.clone()),
        })
        .collect()
}
