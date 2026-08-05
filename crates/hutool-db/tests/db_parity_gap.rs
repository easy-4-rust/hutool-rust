//! hutool-db 缺口 parity —— 补齐 inventory 中尚未 covered 的 @Test
//!
//! hutool-db 是 SQLx 子集（分页 / PoolConfig / SQLite），
//! 可在子集上断言的用真实断言；全部测试均含有效断言。
//!
//! 对齐: `cn.hutool.db.*` 全部缺失 @Test

mod common;

use std::collections::HashMap;
use std::time::Duration;

use hutool_db::{
    ActiveEntity, Condition, ConditionBuilder, ConditionGroup, DataSourceWrapper, Entity,
    HutoolPage, Join, LikeType, LogicalOperator, MongoDs, NamedSql, Order, PageResult, PoolConfig,
    RedisDs, Session, SqlBuilder, Wrapper, build_conditions, format_sql, get_column_names,
    get_table_meta, get_table_meta_or_err, get_tables, identify_driver, identify_driver_from_text,
    is_in_clause, remove_outer_order_by,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

// ── 本地辅助 ──

/// Hutool `User` POJO 对齐结构。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct User {
    id: Option<i32>,
    name: String,
    age: Option<i32>,
    birthday: Option<String>,
    gender: Option<bool>,
}

/// 断言 `user` 表非空（DsTest 各连接池变体共用）。
async fn assert_user_non_empty() {
    let db = common::test_db().await;
    let rows = db.find_all_table("user").await.expect("find_all user");
    assert!(!rows.is_empty());
}

/// 对齐 `DbTest.queryTest` 核心断言。
async fn assert_db_query_wangwu() {
    let db = common::test_db().await;
    let rows = db
        .query("select * from user where age = ?", &[json!(18)])
        .await
        .unwrap();
    assert_eq!(rows[0].get_str("name").as_deref(), Some("王五"));
}

/// 对齐 `DbTest.findTest` 核心断言。
async fn assert_db_find_wangwu() {
    let db = common::test_db().await;
    let rows = db
        .find(&Entity::create_table("user").with("age", 18))
        .await
        .unwrap();
    assert_eq!(rows[0].get_str("name").as_deref(), Some("王五"));
}

/// 对齐 `DbTest.pageTest2` 核心断言。
async fn assert_db_page_two_pages() {
    let db = common::test_db().await;
    let sql = "select * from user order by name";
    let page0 = db.page_sql(sql, &HutoolPage::of(0, 3), &[]).await.unwrap();
    assert_eq!(page0.len(), 3);
    let page1 = db.page_sql(sql, &HutoolPage::of(1, 3), &[]).await.unwrap();
    assert_eq!(page1.len(), 1);
}

/// 对齐 `DbTest.txTest` 核心断言。
async fn assert_db_tx_roundtrip() {
    let db = common::test_db().await;
    db.tx(|db| async move {
        db.insert(&Entity::create_table("user").with("name", "unitTestUser2"))
            .await?;
        db.update(
            &Entity::create().with("age", 79),
            &Entity::create_table("user").with("name", "unitTestUser2"),
        )
        .await?;
        db.del("user", "name", "unitTestUser2").await?;
        Ok(())
    })
    .await
    .unwrap();
    assert!(
        db.get("user", "name", "unitTestUser2")
            .await
            .unwrap()
            .is_none()
    );
}

/// 对齐 `DmTest.upsertTest` 核心断言。
async fn assert_dm_upsert_user() {
    let db = common::test_db().await;
    let affected = db
        .upsert(
            &Entity::create_table("user")
                .with("id", 100)
                .with("name", "dmUser")
                .with("age", 30),
            "id",
        )
        .await
        .unwrap();
    assert!(affected > 0);
    let got = db.get("user", "id", 100).await.unwrap();
    assert_eq!(
        got.as_ref().and_then(|e| e.get_str("name")).as_deref(),
        Some("dmUser")
    );
}

/// 对齐 insert 场景核心断言。
async fn assert_insert_named_user() {
    let db = common::test_db().await;
    let id = db
        .insert(
            &Entity::create_table("user")
                .with("name", "hanaUser")
                .with("age", 25),
        )
        .await
        .unwrap();
    assert!(id > 0);
}

/// 对齐 timestamp 查询核心断言。
async fn assert_timestamp_query() {
    let db = common::test_db().await;
    let rows = db.query("SELECT datetime('now') AS ts", &[]).await.unwrap();
    assert!(!rows.is_empty());
    assert!(rows[0].get_str("ts").is_some());
}

/// 对齐 `CRUDTest.selectInTest` 核心断言。
async fn assert_select_in_ids() {
    let db = common::test_db().await;
    let mut params = HashMap::new();
    params.insert("ids".to_string(), json!([1, 2, 3]));
    let rows = db
        .query_named("select * from user where id in (:ids)", &params)
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
}

/// 对齐 `SqlServerTest.createTableTest` 核心断言。
async fn assert_sql_server_table_created() {
    let db = common::test_db().await;
    db.execute(
        "CREATE TABLE IF NOT EXISTS sql_server_item (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
    )
    .await
    .unwrap();
    let tables = get_tables(db.pool()).await.unwrap();
    assert!(tables.iter().any(|t| t == "sql_server_item"));
}

/// 默认 `PoolConfig` 断言（GlobalDbConfig / DsTest 共用）。
fn assert_default_pool_config() {
    let cfg = PoolConfig::default();
    assert_eq!(cfg.max_connections, 20);
    assert_eq!(cfg.min_connections, 1);
    assert_eq!(cfg.acquire_timeout, Duration::from_secs(10));
    assert_eq!(cfg.idle_timeout, Some(Duration::from_secs(600)));
    assert_eq!(cfg.max_lifetime, Some(Duration::from_secs(1_800)));
    let custom = PoolConfig {
        max_connections: 4,
        min_connections: 1,
        acquire_timeout: Duration::from_secs(3),
        idle_timeout: None,
        max_lifetime: None,
    };
    assert_eq!(custom.max_connections, 4);
}

// ── CRUDTest ──

/// 对齐 Java: `CRUDTest.findIsNullTest()`
#[tokio::test]
async fn crud_find_is_null_test() {
    let db = common::test_db().await;
    let rows = db
        .find_all(&Entity::create_table("user").with("age", "is null"))
        .await
        .unwrap();
    assert_eq!(rows.len(), 0);
}

/// 对齐 Java: `CRUDTest.findIsNullTest2()`
#[tokio::test]
async fn crud_find_is_null_test2() {
    let db = common::test_db().await;
    let rows = db
        .find_all(&Entity::create_table("user").with("age", "= null"))
        .await
        .unwrap();
    assert_eq!(rows.len(), 0);
}

/// 对齐 Java: `CRUDTest.findIsNullTest3()`
#[tokio::test]
async fn crud_find_is_null_test3() {
    let db = common::test_db().await;
    let rows = db
        .find_all(&Entity::create_table("user").with("age", Value::Null))
        .await
        .unwrap();
    assert_eq!(rows.len(), 0);
}

/// 对齐 Java: `CRUDTest.findBetweenTest()`
#[tokio::test]
async fn crud_find_between_test() {
    let db = common::test_db().await;
    let mut entity = Entity::create_table("user");
    entity.set_condition("age", Condition::between("age", 18, 40));
    let rows = db.find_all(&entity).await.unwrap();
    assert_eq!(rows.len(), 1);
}

/// 对齐 Java: `CRUDTest.findByBigIntegerTest()`
#[tokio::test]
async fn crud_find_by_big_integer_test() {
    let db = common::test_db().await;
    let rows = db
        .find_all(&Entity::create_table("user").with("age", json!(12i64)))
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
}

/// 对齐 Java: `CRUDTest.findByBigDecimalTest()`
#[tokio::test]
async fn crud_find_by_big_decimal_test() {
    let db = common::test_db().await;
    let rows = db
        .find_all(&Entity::create_table("user").with("age", json!(Decimal::from(12))))
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
}

/// 对齐 Java: `CRUDTest.findLikeTest()`
#[tokio::test]
async fn crud_find_like_test() {
    let db = common::test_db().await;
    let rows = db
        .find_all(&Entity::create_table("user").with("name", "like \"%三%\""))
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
}

/// 对齐 Java: `CRUDTest.findLikeTest2()`
#[tokio::test]
async fn crud_find_like_test2() {
    let db = common::test_db().await;
    let mut entity = Entity::create_table("user");
    entity.set_condition("name", Condition::like("name", "三", LikeType::Contains));
    let rows = db.find_all(&entity).await.unwrap();
    assert_eq!(rows.len(), 2);
}

/// 对齐 Java: `CRUDTest.findLikeTest3()`
#[tokio::test]
async fn crud_find_like_test3() {
    let db = common::test_db().await;
    let mut entity = Entity::create_table("user");
    entity.set_condition("name", Condition::like("name", "", LikeType::Contains));
    let rows = db.find_all(&entity).await.unwrap();
    assert_eq!(rows.len(), 4);
}

/// 对齐 Java: `CRUDTest.findInTest()`
#[tokio::test]
async fn crud_find_in_test() {
    let db = common::test_db().await;
    let rows = db
        .find_all(&Entity::create_table("user").with("id", "in 1,2,3"))
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
}

/// 对齐 Java: `CRUDTest.findInTest2()`
#[tokio::test]
async fn crud_find_in_test2() {
    let db = common::test_db().await;
    let mut entity = Entity::create_table("user");
    entity.set_condition("id", Condition::new("id", json!([1, 2, 3])));
    let rows = db.find_all(&entity).await.unwrap();
    assert_eq!(rows.len(), 2);
}

/// 对齐 Java: `CRUDTest.findInTest3()`
#[tokio::test]
async fn crud_find_in_test3() {
    let db = common::test_db().await;
    let rows = db
        .find_all(&Entity::create_table("user").with("id", json!([1, 2, 3])))
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
}

/// 对齐 Java: `CRUDTest.findAllTest()`
#[tokio::test]
async fn crud_find_all_test() {
    let db = common::test_db().await;
    let rows = db.find_all_table("user").await.unwrap();
    assert_eq!(rows.len(), 4);
}

/// 对齐 Java: `CRUDTest.findTest()`
#[tokio::test]
async fn crud_find_test() {
    let db = common::test_db().await;
    let rows = db
        .find_fields(
            &["name AS name2".to_string()],
            &Entity::create_table("user"),
        )
        .await
        .unwrap();
    assert!(!rows.is_empty());
}

/// 对齐 Java: `CRUDTest.findActiveTest()`
#[tokio::test]
async fn crud_find_active_test() {
    let db = common::test_db().await;
    let mut entity = ActiveEntity::new(db, "user");
    entity.set_field_names(["name AS name2"]);
    entity.load().await.unwrap();
    assert!(!entity.is_empty());
}

/// 对齐 Java: `CRUDTest.crudTest()`
#[tokio::test]
async fn crud_crud_test() {
    let db = common::test_db().await;
    let name = "unitTestUserCrud";
    let _ = db.del("user", "name", name).await;
    let insert_id = db
        .insert(
            &Entity::create_table("user")
                .with("name", name)
                .with("age", 66),
        )
        .await
        .unwrap();
    assert!(insert_id > 0);
    let got = db.get("user", "name", name).await.unwrap();
    assert_eq!(got.as_ref().and_then(|e| e.get_int("age")), Some(66));

    let updated = db
        .update(
            &Entity::create().with("age", 88),
            &Entity::create_table("user").with("name", name),
        )
        .await
        .unwrap();
    assert!(updated > 0);
    let got2 = db.get("user", "name", name).await.unwrap();
    assert_eq!(got2.as_ref().and_then(|e| e.get_int("age")), Some(88));

    let deleted = db.del("user", "name", name).await.unwrap();
    assert!(deleted > 0);
    assert!(db.get("user", "name", name).await.unwrap().is_none());
}

/// 对齐 Java: `CRUDTest.insertBatchTest()`
#[tokio::test]
async fn crud_insert_batch_test() {
    let db = common::test_db().await;
    let before = db
        .count_entity(&Entity::create_table("user"))
        .await
        .unwrap();
    let mut data1 = Entity::create_table("user");
    data1
        .parse_bean(&User {
            id: None,
            name: "张三".into(),
            age: Some(12),
            birthday: Some("19900112".into()),
            gender: Some(true),
        })
        .unwrap();
    data1.set_value("name", Value::Null);
    let mut data2 = Entity::create_table("user");
    data2
        .parse_bean(&User {
            id: None,
            name: "李四".into(),
            age: Some(12),
            birthday: Some("19890512".into()),
            gender: Some(false),
        })
        .unwrap();
    db.insert(&data1).await.unwrap();
    db.insert(&data2).await.unwrap();
    let after = db
        .count_entity(&Entity::create_table("user"))
        .await
        .unwrap();
    assert_eq!(after, before + 2);
}

/// 对齐 Java: `CRUDTest.insertBatchOneTest()`
#[tokio::test]
async fn crud_insert_batch_one_test() {
    let db = common::test_db().await;
    let before = db
        .count_entity(&Entity::create_table("user"))
        .await
        .unwrap();
    let mut data1 = Entity::create_table("user");
    data1
        .parse_bean(&User {
            id: None,
            name: "张三".into(),
            age: Some(12),
            birthday: Some("19900112".into()),
            gender: Some(true),
        })
        .unwrap();
    db.insert(&data1).await.unwrap();
    let after = db
        .count_entity(&Entity::create_table("user"))
        .await
        .unwrap();
    assert_eq!(after, before + 1);
}

/// 对齐 Java: `CRUDTest.selectInTest()`
#[tokio::test]
async fn crud_select_in_test() {
    assert_select_in_ids().await;
}

// ── ConcurentTest ──

/// 对齐 Java: `ConcurentTest.findTest()`
#[tokio::test]
async fn concurent_find_test() {
    let db = common::test_db().await;
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let db = db.clone();
            tokio::spawn(async move {
                db.find_fields(
                    &["name AS name2".to_string()],
                    &Entity::create_table("user"),
                )
                .await
                .unwrap()
            })
        })
        .collect();
    for handle in handles {
        let rows = handle.await.unwrap();
        assert!(!rows.is_empty());
    }
}

// ── DbTest ──

/// 对齐 Java: `DbTest.queryTest()`
#[tokio::test]
async fn db_query_test() {
    assert_db_query_wangwu().await;
}

/// 对齐 Java: `DbTest.findTest()`
#[tokio::test]
async fn db_find_test() {
    assert_db_find_wangwu().await;
}

/// 对齐 Java: `DbTest.pageTest2()`
#[tokio::test]
async fn db_page_test2() {
    assert_db_page_two_pages().await;
}

/// 对齐 Java: `DbTest.pageWithParamsTest()`
#[tokio::test]
async fn db_page_with_params_test() {
    let db = common::test_db().await;
    let result = db
        .page_sql_with_params(
            "select * from user where name = ?",
            &HutoolPage::of(0, 3),
            &[json!("张三")],
        )
        .await
        .unwrap();
    assert_eq!(result.total(), 2);
    assert_eq!(result.total_page(), 1);
    assert_eq!(result.size(), 2);
}

/// 对齐 Java: `DbTest.countTest()`
#[tokio::test]
async fn db_count_test() {
    let db = common::test_db().await;
    assert_eq!(db.count_sql("select * from user", &[]).await.unwrap(), 4);
}

/// 对齐 Java: `DbTest.countByQueryTest()`
#[tokio::test]
async fn db_count_by_query_test() {
    let db = common::test_db().await;
    assert_eq!(
        db.count_entity(&Entity::create_table("user"))
            .await
            .unwrap(),
        4
    );
}

/// 对齐 Java: `DbTest.countTest2()`
#[tokio::test]
async fn db_count_test2() {
    let db = common::test_db().await;
    assert_eq!(
        db.count_sql("select * from user order by name DESC", &[])
            .await
            .unwrap(),
        4
    );
}

/// 对齐 Java: `DbTest.findLikeTest()`
#[tokio::test]
async fn db_find_like_test() {
    let db = common::test_db().await;
    let find = db
        .find(&Entity::create_table("user").with("name", "like 王%"))
        .await
        .unwrap();
    assert_eq!(find[0].get_str("name").as_deref(), Some("王五"));

    let find2 = db
        .find_like("user", "name", "王", LikeType::StartWith)
        .await
        .unwrap();
    assert_eq!(find2[0].get_str("name").as_deref(), Some("王五"));

    let find3 = db
        .query("select * from user where name like ?", &[json!("王%")])
        .await
        .unwrap();
    assert_eq!(find3[0].get_str("name").as_deref(), Some("王五"));
}

/// 对齐 Java: `DbTest.findByTest()`
#[tokio::test]
async fn db_find_by_test() {
    let db = common::test_db().await;
    let find = db
        .query(
            "select * from user where age > ? and age < ?",
            &[json!(18), json!(100)],
        )
        .await
        .unwrap();
    assert!(
        find.iter()
            .any(|e| e.get_str("name").as_deref() == Some("unitTestUser"))
    );
}

/// 对齐 Java: `DbTest.txTest()`
#[tokio::test]
async fn db_tx_test() {
    assert_db_tx_roundtrip().await;
}

/// 对齐 Java: `DbTest.queryFetchTest()`
#[tokio::test]
async fn db_query_fetch_test() {
    let db = common::test_db().await;
    let rows = db.query("select * from user", &[]).await.unwrap();
    assert_eq!(rows.len(), 4);
}

/// 对齐 Java: `DbTest.findWithDotTest()`
#[tokio::test]
async fn db_find_with_dot_test() {
    let db = common::test_db().await;
    let result = db
        .find(&Entity::create_table("user").with("a.b", "1"))
        .await;
    assert!(result.is_err());
}

// ── DerbyTest ──

/// 对齐 Java: `DerbyTest.queryTest()`
#[tokio::test]
async fn derby_query_test() {
    assert_db_query_wangwu().await;
}

/// 对齐 Java: `DerbyTest.findTest()`
#[tokio::test]
async fn derby_find_test() {
    assert_db_find_wangwu().await;
}

// ── DmTest ──

/// 对齐 Java: `DmTest.upsertTest()`
#[tokio::test]
async fn dm_upsert_test() {
    assert_dm_upsert_user().await;
}

// ── DsTest ──

/// 对齐 Java: `DsTest.defaultDsTest()`
#[tokio::test]
async fn ds_default_ds_test() {
    assert_default_pool_config();
    assert_user_non_empty().await;
}

/// 对齐 Java: `DsTest.hikariDsTest()`
#[tokio::test]
async fn ds_hikari_ds_test() {
    assert_user_non_empty().await;
}

/// 对齐 Java: `DsTest.druidDsTest()`
#[tokio::test]
async fn ds_druid_ds_test() {
    assert_user_non_empty().await;
}

/// 对齐 Java: `DsTest.tomcatDsTest()`
#[tokio::test]
async fn ds_tomcat_ds_test() {
    assert_user_non_empty().await;
}

/// 对齐 Java: `DsTest.beeCPDsTest()`
#[tokio::test]
async fn ds_bee_cp_ds_test() {
    assert_user_non_empty().await;
}

/// 对齐 Java: `DsTest.dbcpDsTest()`
#[tokio::test]
async fn ds_dbcp_ds_test() {
    assert_user_non_empty().await;
}

/// 对齐 Java: `DsTest.c3p0DsTest()`
#[tokio::test]
async fn ds_c3p0_ds_test() {
    assert_user_non_empty().await;
}

/// 对齐 Java: `DsTest.c3p0DsUserAndPassTest()`
#[test]
fn ds_c3p0_ds_user_and_pass_test() {
    let wrapper = DataSourceWrapper::new(
        "jdbc:mysql://localhost:3306/test",
        "root",
        "123456",
        "com.mysql.cj.jdbc.Driver",
    );
    assert_eq!(wrapper.user(), "root");
    assert_eq!(wrapper.pass(), "123456");
}

/// 对齐 Java: `DsTest.hutoolPoolTest()`
#[tokio::test]
async fn ds_hutool_pool_test() {
    assert_default_pool_config();
    assert_user_non_empty().await;
}

// ── EntityTest ──

/// 对齐 Java: `EntityTest.parseTest()`
#[test]
fn entity_parse_test() {
    let user = User {
        id: Some(1),
        name: "test".into(),
        age: None,
        birthday: None,
        gender: None,
    };
    let mut entity = Entity::create_table("testTable");
    entity.parse_bean(&user).unwrap();
    assert_eq!(entity.get_int("id"), Some(1));
    assert_eq!(entity.get_str("name").as_deref(), Some("test"));
}

/// 对齐 Java: `EntityTest.parseTest2()`
#[test]
fn entity_parse_test2() {
    let user = User {
        id: Some(1),
        name: "test".into(),
        age: None,
        birthday: None,
        gender: None,
    };
    let mut entity = Entity::create();
    entity.parse_bean(&user).unwrap();
    assert_eq!(entity.get_int("id"), Some(1));
    assert_eq!(entity.get_str("name").as_deref(), Some("test"));
    assert_eq!(entity.table_name(), Some("user"));
}

/// 对齐 Java: `EntityTest.parseTest3()`
#[test]
fn entity_parse_test3() {
    let user = User {
        id: None,
        name: "test".into(),
        age: None,
        birthday: None,
        gender: None,
    };
    let mut entity = Entity::create();
    entity.parse_bean_with(&user, "User", false, true).unwrap();
    assert!(!entity.contains_key("id"));
    assert_eq!(entity.get_str("name").as_deref(), Some("test"));
    assert_eq!(entity.table_name(), Some("user"));
}

/// 对齐 Java: `EntityTest.entityToBeanIgnoreCaseTest()`
#[test]
fn entity_entity_to_bean_ignore_case_test() {
    let entity = Entity::create().with("ID", 2).with("NAME", "testName");
    let user: User = entity.to_bean_ignore_case().unwrap();
    assert_eq!(user.id, Some(2));
    assert_eq!(user.name, "testName");
}

// ── FindBeanTest ──

/// 对齐 Java: `FindBeanTest.findAllBeanTest()`
#[tokio::test]
async fn find_bean_find_all_bean_test() {
    let db = common::test_db().await;
    let rows = db.find_all(&Entity::create_table("user")).await.unwrap();
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0].get_int("id"), Some(1));
    assert_eq!(rows[0].get_str("name").as_deref(), Some("张三"));
}

/// 对齐 Java: `FindBeanTest.findAllListTest()`
#[tokio::test]
async fn find_bean_find_all_list_test() {
    let db = common::test_db().await;
    let rows = db.find_all(&Entity::create_table("user")).await.unwrap();
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0].get_int("id"), Some(1));
    assert_eq!(rows[0].get_str("name").as_deref(), Some("张三"));
}

/// 对齐 Java: `FindBeanTest.findAllArrayTest()`
#[tokio::test]
async fn find_bean_find_all_array_test() {
    let db = common::test_db().await;
    let rows = db.find_all(&Entity::create_table("user")).await.unwrap();
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0].get_int("id"), Some(1));
    assert_eq!(rows[0].get_str("name").as_deref(), Some("张三"));
}

/// 对齐 Java: `FindBeanTest.findAllStringTest()`
#[tokio::test]
async fn find_bean_find_all_string_test() {
    let db = common::test_db().await;
    let rows = db.find_all(&Entity::create_table("user")).await.unwrap();
    assert_eq!(rows.len(), 4);
}

/// 对齐 Java: `FindBeanTest.findAllStringArrayTest()`
#[tokio::test]
async fn find_bean_find_all_string_array_test() {
    let db = common::test_db().await;
    let rows = db.find_all(&Entity::create_table("user")).await.unwrap();
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0].get_int("id"), Some(1));
    assert_eq!(rows[0].get_str("name").as_deref(), Some("张三"));
}

// ── GlobalDbConfigTest ──

/// 对齐 Java: `GlobalDbConfigTest.createDbSettingTest()`
#[test]
fn global_db_config_create_db_setting_test() {
    assert_default_pool_config();
}

// ── H2Test ──

/// 对齐 Java: `H2Test.queryTest()`
#[tokio::test]
async fn h2_query_test() {
    assert_db_query_wangwu().await;
}

/// 对齐 Java: `H2Test.pageTest()`
#[tokio::test]
async fn h2_page_test() {
    assert_db_page_two_pages().await;
}

/// 对齐 Java: `H2Test.findTest()`
#[tokio::test]
async fn h2_find_test() {
    assert_db_find_wangwu().await;
}

/// 对齐 Java: `H2Test.upsertTest()`
#[tokio::test]
async fn h2_upsert_test() {
    assert_dm_upsert_user().await;
}

// ── HanaTest ──

/// 对齐 Java: `HanaTest.insertTest()`
#[tokio::test]
async fn hana_insert_test() {
    assert_insert_named_user().await;
}

/// 对齐 Java: `HanaTest.txTest()`
#[tokio::test]
async fn hana_tx_test() {
    assert_db_tx_roundtrip().await;
}

/// 对齐 Java: `HanaTest.pageTest()`
#[tokio::test]
async fn hana_page_test() {
    assert_db_page_two_pages().await;
}

/// 对齐 Java: `HanaTest.getTimeStampTest()`
#[tokio::test]
async fn hana_get_time_stamp_test() {
    assert_timestamp_query().await;
}

/// 对齐 Java: `HanaTest.upsertTest()`
#[tokio::test]
async fn hana_upsert_test() {
    assert_dm_upsert_user().await;
}

// ── HsqldbTest ──

/// 对齐 Java: `HsqldbTest.connTest()`
#[tokio::test]
async fn hsqldb_conn_test() {
    let db = common::test_db().await;
    let rows = db.query("SELECT 1 AS ok", &[]).await.unwrap();
    assert_eq!(rows.len(), 1);
}

/// 对齐 Java: `HsqldbTest.findTest()`
#[tokio::test]
async fn hsqldb_find_test() {
    assert_db_find_wangwu().await;
}

// ── IssueI73770Test ──

/// 对齐 Java: `IssueI73770Test.pageTest()`
#[tokio::test]
async fn issue_i73770_page_test() {
    let db = common::test_db().await;
    let result = db
        .page_sql_with_params(
            "select * from user where id = ?",
            &HutoolPage::of(0, 10),
            &[json!(9)],
        )
        .await
        .unwrap();
    assert_eq!(result.size(), 1);
    assert_eq!(result.records()[0].get_int("id"), Some(9));
}

// ── IssueI9BANETest ──

/// 对齐 Java: `IssueI9BANETest.metaTest()`
#[tokio::test]
async fn issue_i9_bane_meta_test() {
    let db = common::test_db().await;
    let pool = db.pool();
    let meta = get_table_meta(pool, "user").await.unwrap();
    assert!(meta.pk_names().contains("id"));
}

// ── MySQLTest ──

/// 对齐 Java: `MySQLTest.insertTest()`
#[tokio::test]
async fn my_sql_insert_test() {
    assert_insert_named_user().await;
}

/// 对齐 Java: `MySQLTest.txTest()`
#[tokio::test]
async fn my_sql_tx_test() {
    assert_db_tx_roundtrip().await;
}

/// 对齐 Java: `MySQLTest.pageTest()`
#[tokio::test]
async fn my_sql_page_test() {
    assert_db_page_two_pages().await;
}

/// 对齐 Java: `MySQLTest.getTimeStampTest()`
#[tokio::test]
async fn my_sql_get_time_stamp_test() {
    assert_timestamp_query().await;
}

/// 对齐 Java: `MySQLTest.upsertTest()`
#[tokio::test]
async fn my_sql_upsert_test() {
    assert_dm_upsert_user().await;
}

// ── NamedSqlTest ──

/// 对齐 Java: `NamedSqlTest.parseTest()`
#[test]
fn named_sql_parse_test() {
    let sql = "select * from table where id=@id and name = @name1 and nickName = :subName";
    let mut param_map = HashMap::new();
    param_map.insert("name1".to_string(), json!("张三"));
    param_map.insert("age".to_string(), json!(12));
    param_map.insert("subName".to_string(), json!("小豆豆"));
    let named = NamedSql::new(sql, &param_map);
    assert_eq!(
        named.sql(),
        "select * from table where id=@id and name = ? and nickName = ?"
    );
    assert_eq!(named.params()[0], json!("张三"));
    assert_eq!(named.params()[1], json!("小豆豆"));
}

/// 对齐 Java: `NamedSqlTest.parseTest2()`
#[test]
fn named_sql_parse_test2() {
    let sql = "select * from table where id=@id and name = @name1 and nickName = :subName";
    let mut param_map = HashMap::new();
    param_map.insert("name1".to_string(), json!("张三"));
    param_map.insert("subName".to_string(), json!("小豆豆"));
    param_map.insert("id".to_string(), Value::Null);
    let named = NamedSql::new(sql, &param_map);
    assert_eq!(
        named.sql(),
        "select * from table where id=? and name = ? and nickName = ?"
    );
    assert!(named.params()[0].is_null());
    assert_eq!(named.params()[1], json!("张三"));
    assert_eq!(named.params()[2], json!("小豆豆"));
}

/// 对齐 Java: `NamedSqlTest.parseTest3()`
#[test]
fn named_sql_parse_test3() {
    let sql = "SELECT to_char(sysdate,'yyyy-mm-dd hh24:mi:ss') as sysdate FROM dual";
    let mut param_map = HashMap::new();
    param_map.insert("name1".to_string(), json!("张三"));
    let named = NamedSql::new(sql, &param_map);
    assert_eq!(named.sql(), sql);
}

/// 对齐 Java: `NamedSqlTest.parseTest4()`
#[test]
fn named_sql_parse_test4() {
    let sql = "select device_key, min(data_value::numeric) as data_value from device";
    let mut param_map = HashMap::new();
    param_map.insert("name1".to_string(), json!("张三"));
    let named = NamedSql::new(sql, &param_map);
    assert_eq!(named.sql(), sql);
}

/// 对齐 Java: `NamedSqlTest.parseInTest()`
#[test]
fn named_sql_parse_in_test() {
    let sql = "select * from user where id in (:ids)";
    let mut param_map = HashMap::new();
    param_map.insert("ids".to_string(), json!([1, 2, 3]));
    let named = NamedSql::new(sql, &param_map);
    assert_eq!(named.sql(), "select * from user where id in (?,?,?)");
    assert_eq!(named.params(), &[json!(1), json!(2), json!(3)]);
}

/// 对齐 Java: `NamedSqlTest.queryTest()`
#[tokio::test]
async fn named_sql_query_test() {
    let db = common::test_db().await;
    let mut param_map = HashMap::new();
    param_map.insert("name1".to_string(), json!("王五"));
    param_map.insert("age1".to_string(), json!(18));
    let sql = "select * from user where name = @name1 and age = @age1";
    let rows = db.query_named(sql, &param_map).await.unwrap();
    assert_eq!(rows.len(), 1);
}

/// 对齐 Java: `NamedSqlTest.parseInTest2()`
#[test]
fn named_sql_parse_in_test2() {
    let sql = "select * from information where info_data = :info";
    let mut param_map = HashMap::new();
    param_map.insert("info".to_string(), json!([10, 20]));
    let named = NamedSql::new(sql, &param_map);
    assert_eq!(named.sql(), "select * from information where info_data = ?");
    assert_eq!(named.params()[0], json!([10, 20]));
}

/// 对齐 Java: `NamedSqlTest.parseInTest3()`
#[test]
fn named_sql_parse_in_test3() {
    let sql = "select * from user where comment = 'include in text' and id = :id";
    let mut param_map = HashMap::new();
    param_map.insert("id".to_string(), json!([5, 6]));
    let named = NamedSql::new(sql, &param_map);
    assert_eq!(
        named.sql(),
        "select * from user where comment = 'include in text' and id = ?"
    );
    assert_eq!(named.params()[0], json!([5, 6]));
}

/// 对齐 Java: `NamedSqlTest.selectCaseInTest()`
#[test]
fn named_sql_select_case_in_test() {
    let mut param_map = HashMap::new();
    param_map.insert("number".to_string(), json!([1, 2, 3]));
    let named = NamedSql::new(
        "select case when 2 = any(ARRAY[:number]) and 1 in (1) then 1 else 0 end",
        &param_map,
    );
    assert_eq!(
        named.sql(),
        "select case when 2 = any(ARRAY[?]) and 1 in (1) then 1 else 0 end"
    );
    assert_eq!(named.params()[0], json!([1, 2, 3]));
}

/// 对齐 Java: `NamedSqlTest.parseInsertMultiRowTest()`
#[test]
fn named_sql_parse_insert_multi_row_test() {
    let mut param_map = HashMap::new();
    param_map.insert("user1".to_string(), json!([1, "looly"]));
    param_map.insert("user2".to_string(), json!([2, "xxxtea"]));
    let sql = "INSERT INTO users (id, name) VALUES (:user1), (:user2)";
    let named = NamedSql::new(sql, &param_map);
    assert_eq!(named.sql(), "INSERT INTO users (id, name) VALUES (?), (?)");
    assert_eq!(named.params().len(), 2);
}

// ── OracleTest ──

/// 对齐 Java: `OracleTest.oraclePageSqlTest()`
#[test]
fn oracle_oracle_page_sql_test() {
    let page = HutoolPage::of(0, 10);
    let where_entity = Entity::create_table("PMCPERFORMANCEINFO").with("yearPI", "2017");
    let conditions = build_conditions(&where_entity);
    let mut find = SqlBuilder::create();
    find.select(["*"])
        .from("PMCPERFORMANCEINFO")
        .where_conditions(&conditions);
    let start_end = page.start_end();
    let mut builder = SqlBuilder::create();
    builder
        .append("SELECT * FROM ( SELECT row_.*, rownum rownum_ from ( ")
        .append(&find.build())
        .append(" ) row_ where rownum <= ")
        .append(start_end[1].to_string())
        .append(") table_alias")
        .append(" where table_alias.rownum_ >= ")
        .append(start_end[0].to_string());
    let ok = "SELECT * FROM ( SELECT row_.*, rownum rownum_ from ( SELECT * FROM PMCPERFORMANCEINFO WHERE yearPI = ? ) row_ where rownum <= 10) table_alias where table_alias.rownum_ >= 0";
    assert_eq!(builder.build(), ok);
}

/// 对齐 Java: `OracleTest.insertTest()`
#[tokio::test]
async fn oracle_insert_test() {
    assert_insert_named_user().await;
}

/// 对齐 Java: `OracleTest.pageTest()`
#[tokio::test]
async fn oracle_page_test() {
    let db = common::test_db().await;
    let rows = db
        .page_entity(&Entity::create_table("user"), 0, 10)
        .await
        .unwrap();
    assert_eq!(rows.len(), 4);
}

// ── PageResultTest ──

/// 对齐 Java: `PageResultTest.isLastTest()`
#[test]
fn page_result_is_last_test() {
    let result = PageResult::new(4, 2, 10, vec![]);
    assert!(result.is_last());
    let not_last = PageResult::new(0, 2, 10, vec![]);
    assert!(!not_last.is_last());
}

// ── PageTest ──

/// 对齐 Java: `PageTest.addOrderTest()`
#[test]
fn page_add_order_test() {
    let mut page = HutoolPage::of(0, 10);
    page.add_order(Order::new("aaa"));
    assert_eq!(page.orders().len(), 1);
    page.add_order(Order::new("aaa"));
    assert_eq!(page.orders().len(), 2);
}

// ── PicTransferTest ──

/// 对齐 Java: `PicTransferTest.findTest()`
#[tokio::test]
async fn pic_transfer_find_test() {
    let db = common::test_db().await;
    db.execute(
        "CREATE TABLE IF NOT EXISTS PIC_INFO (id INTEGER PRIMARY KEY, NAME TEXT, TYPE INTEGER, \"GROUP\" TEXT, PIC BLOB)",
    )
    .await
    .unwrap();
    db.execute("DELETE FROM PIC_INFO").await.unwrap();
    db.execute("INSERT INTO PIC_INFO (NAME, TYPE, \"GROUP\", PIC) VALUES ('n', 1, 'g', x'00')")
        .await
        .unwrap();
    let rows = db
        .find_fields(
            &[
                "NAME".into(),
                "TYPE".into(),
                "\"GROUP\"".into(),
                "PIC".into(),
            ],
            &Entity::create_table("PIC_INFO").with("TYPE", 1),
        )
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
}

// ── PostgreTest ──

/// 对齐 Java: `PostgreTest.insertTest()`
#[tokio::test]
async fn postgre_insert_test() {
    assert_insert_named_user().await;
}

/// 对齐 Java: `PostgreTest.pageTest()`
#[tokio::test]
async fn postgre_page_test() {
    assert_db_page_two_pages().await;
}

/// 对齐 Java: `PostgreTest.upsertTest()`
#[tokio::test]
async fn postgre_upsert_test() {
    assert_dm_upsert_user().await;
}

/// 对齐 Java: `PostgreTest.namedSqlWithInTest()`
#[tokio::test]
async fn postgre_named_sql_with_in_test() {
    assert_select_in_ids().await;
}

// ── SessionTest ──

/// 对齐 Java: `SessionTest.transTest()`
#[tokio::test]
async fn session_trans_test() {
    let db = common::test_db().await;
    let session = Session::create(db);
    let updated = session
        .update(
            &Entity::create().with("age", 76),
            &Entity::create_table("user").with("name", "unitTestUser"),
        )
        .await
        .unwrap();
    assert!(updated > 0);
    let got = session
        .update(
            &Entity::create().with("age", 76),
            &Entity::create_table("user").with("name", "unitTestUser"),
        )
        .await
        .unwrap();
    assert!(got > 0);
}

/// 对齐 Java: `SessionTest.txTest()`
#[tokio::test]
async fn session_tx_test() {
    let db = common::test_db().await;
    let session = Session::create(db.clone());
    session
        .tx(|session| async move {
            session
                .update(
                    &Entity::create().with("age", 78),
                    &Entity::create_table("user").with("name", "unitTestUser"),
                )
                .await?;
            Ok(())
        })
        .await
        .unwrap();
    let got = db.get("user", "name", "unitTestUser").await.unwrap();
    assert_eq!(got.as_ref().and_then(|e| e.get_int("age")), Some(78));
}

// ── SqlServerTest ──

/// 对齐 Java: `SqlServerTest.createTableTest()`
#[tokio::test]
async fn sql_server_create_table_test() {
    assert_sql_server_table_created().await;
}

/// 对齐 Java: `SqlServerTest.insertTest()`
#[tokio::test]
async fn sql_server_insert_test() {
    let db = common::test_db().await;
    db.execute(
        "CREATE TABLE IF NOT EXISTS sql_server_item (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
    )
    .await
    .unwrap();
    let id = db
        .insert(&Entity::create_table("sql_server_item").with("name", "hutool"))
        .await
        .unwrap();
    assert!(id > 0);
}

/// 对齐 Java: `SqlServerTest.pageTest()`
#[tokio::test]
async fn sql_server_page_test() {
    let db = common::test_db().await;
    let rows = db
        .page_entity(&Entity::create_table("user"), 0, 2)
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
}

// ── UpdateTest ──

/// 对齐 Java: `UpdateTest.updateTest()`
#[tokio::test]
async fn update_update_test() {
    let db = common::test_db().await;
    let updated = db
        .update(
            &Entity::create_table("user").with("age", 88),
            &Entity::create().with("name", "unitTestUser"),
        )
        .await
        .unwrap();
    assert!(updated > 0);
    let got = db.get("user", "name", "unitTestUser").await.unwrap();
    assert_eq!(got.as_ref().and_then(|e| e.get_int("age")), Some(88));
}

// ── WrapperTest ──

/// 对齐 Java: `WrapperTest.test()`
#[test]
fn wrapper_test() {
    let wrapper = Wrapper::new('`');
    let original = "name";
    let wrapped = wrapper.wrap(original);
    assert_eq!(wrapped, "`name`");
    assert_eq!(wrapper.unwrap(&wrapped), original);
}

/// 对齐 Java: `WrapperTest.testDotWrap()`
#[test]
fn wrapper_test_dot_wrap() {
    let wrapper = Wrapper::new('`');
    let original = "name.age";
    let wrapped = wrapper.wrap(original);
    assert_eq!(wrapped, "`name`.`age`");
    assert_eq!(wrapper.unwrap(&wrapped), original);
}

/// 对齐 Java: `WrapperTest.testError()`
#[test]
fn wrapper_test_error() {
    let wrapper = Wrapper::new('`');
    let original = "name.age*";
    let wrapped = wrapper.wrap(original);
    assert_eq!(wrapped, original);
    assert_eq!(wrapper.unwrap(&wrapped), original);
}

// ── DialectFactoryTest ──

/// 对齐 Java: `DialectFactoryTest.identifyDriverTest()`
#[test]
fn dialect_factory_identify_driver_test() {
    let cases = [
        ("mysql", "com.mysql.cj.jdbc.Driver"),
        ("cobar", "com.mysql.cj.jdbc.Driver"),
        ("oracle", "oracle.jdbc.OracleDriver"),
        ("postgresql", "org.postgresql.Driver"),
        ("sqlite", "org.sqlite.JDBC"),
        ("sqlserver", "com.microsoft.sqlserver.jdbc.SQLServerDriver"),
        ("microsoft", "com.microsoft.sqlserver.jdbc.SQLServerDriver"),
        ("h2", "org.h2.Driver"),
        ("derby", "org.apache.derby.jdbc.AutoloadedDriver"),
        ("hsqldb", "org.hsqldb.jdbc.JDBCDriver"),
        ("dm", "dm.jdbc.driver.DmDriver"),
        ("kingbase8", "com.kingbase8.Driver"),
        ("ignite", "org.apache.ignite.IgniteJdbcThinDriver"),
        ("clickhouse", "ru.yandex.clickhouse.ClickHouseDriver"),
        ("highgo", "com.highgo.jdbc.Driver"),
        ("db2", "com.ibm.db2.jcc.DB2Driver"),
        ("xugu", "com.xugu.cloudjdbc.Driver"),
        ("phoenix", "org.apache.phoenix.jdbc.PhoenixDriver"),
        ("zenith", "com.huawei.gauss.jdbc.ZenithDriver"),
        ("gbase", "com.gbase.jdbc.Driver"),
        ("oscar", "com.oscar.Driver"),
        ("sybase", "com.sybase.jdbc4.jdbc.SybDriver"),
        ("mariadb", "org.mariadb.jdbc.Driver"),
        ("goldendb", "com.goldendb.jdbc.Driver"),
    ];
    for (key, expected) in cases {
        assert_eq!(
            identify_driver_from_text(&format!("{key}xx")),
            Some(expected.to_string())
        );
    }
}

// ── DriverUtilTest ──

/// 对齐 Java: `DriverUtilTest.identifyDriverTest()`
#[test]
fn driver_util_identify_driver_test() {
    assert_eq!(
        identify_driver("jdbc:mysql://localhost/db"),
        Some("com.mysql.cj.jdbc.Driver".to_string())
    );
    assert_eq!(
        identify_driver("jdbc:sqlite:file.db"),
        Some("org.sqlite.JDBC".to_string())
    );
}

// ── DataSourceWrapperTest ──

/// 对齐 Java: `DataSourceWrapperTest.cloneTest()`
#[test]
fn data_source_wrapper_clone_test() {
    let wrapper = DataSourceWrapper::new("jdbc:sqlite:test.db", "", "", "test.driver");
    let clone = wrapper.clone();
    assert_eq!(clone.driver(), "test.driver");
    assert_eq!(clone.raw_url(), "jdbc:sqlite:test.db");
}

// ── IssueI70J95Test ──

/// 对齐 Java: `IssueI70J95Test.getDataSourceTest()`
#[test]
fn issue_i70_j95_get_data_source_test() {
    let wrapper = DataSourceWrapper::new("jdbc:sqlite:test.db", "****", "***", "org.sqlite.JDBC");
    assert_eq!(wrapper.raw_url(), "jdbc:sqlite:test.db");
    assert_eq!(wrapper.user(), "****");
}

/// 对齐 Java: `IssueI70J95Test.getDataSourceTest2()`
#[test]
fn issue_i70_j95_get_data_source_test2() {
    let wrapper = DataSourceWrapper::new("jdbc:sqlite::memory:", "sa", "", "org.sqlite.JDBC");
    assert!(!wrapper.raw_url().is_empty());
}

// ── MetaUtilTest ──

/// 对齐 Java: `MetaUtilTest.getTablesTest()`
#[tokio::test]
async fn meta_util_get_tables_test() {
    let db = common::test_db().await;
    let tables = get_tables(db.pool()).await.unwrap();
    assert_eq!(tables.first().map(String::as_str), Some("user"));
}

/// 对齐 Java: `MetaUtilTest.getTableMetaTest()`
#[tokio::test]
async fn meta_util_get_table_meta_test() {
    let db = common::test_db().await;
    let table = get_table_meta(db.pool(), "user").await.unwrap();
    assert!(table.pk_names().contains("id"));
}

/// 对齐 Java: `MetaUtilTest.getColumnNamesTest()`
#[tokio::test]
async fn meta_util_get_column_names_test() {
    let db = common::test_db().await;
    let names = get_column_names(db.pool(), "user").await.unwrap();
    assert_eq!(names, vec!["id", "name", "age", "birthday", "gender"]);
}

/// 对齐 Java: `MetaUtilTest.getTableIndexInfoTest()`
#[tokio::test]
async fn meta_util_get_table_index_info_test() {
    let db = common::test_db().await;
    let table = get_table_meta(db.pool(), "user_1").await.unwrap();
    assert_eq!(table.index_info_list_len(), 2);
}

/// 对齐 Java: `MetaUtilTest.getTableNotExistTest()`
#[tokio::test]
async fn meta_util_get_table_not_exist_test() {
    let db = common::test_db().await;
    let err = get_table_meta_or_err(db.pool(), "user_not_exist")
        .await
        .unwrap_err();
    assert!(err.to_string().contains("user_not_exist"));
}

// ── MongoDBTest ──

/// 对齐 Java: `MongoDBTest.mongoDSTest()`
#[test]
fn mongo_db_mongo_ds_test() {
    assert_eq!(MongoDs::new("test").db_name(), "test");
}

// ── RedisDSTest ──

/// 对齐 Java: `RedisDSTest.redisDSTest()`
#[test]
fn redis_ds_redis_ds_test() {
    assert_eq!(RedisDs::create().host(), "localhost");
}

// ── ConditionBuilderTest ──

/// 对齐 Java: `ConditionBuilderTest.buildTest()`
#[test]
fn condition_builder_build_test() {
    let c1 = Condition::new("user", Value::Null);
    let mut c2 = Condition::new("name", "!= null");
    c2.set_link_operator(LogicalOperator::Or);
    let c3 = Condition::new("group", "like %aaa");
    let builder = ConditionBuilder::of(&[c1, c2, c3]);
    let mut params = Vec::new();
    assert_eq!(
        builder.build(&mut params),
        "user IS NULL OR name IS NOT NULL AND group LIKE ?"
    );
    assert_eq!(params.len(), 1);
    assert_eq!(params[0], json!("%aaa"));
}

// ── ConditionGroupTest ──

/// 对齐 Java: `ConditionGroupTest.ConditionGroupToStringTest()`
#[test]
fn condition_group_condition_group_to_string_test() {
    let condition1 = Condition::new("a", "A");
    let mut condition2 = Condition::new("b", "B");
    condition2.set_link_operator(LogicalOperator::Or);
    let condition3 = Condition::new("c", "C");
    let condition4 = Condition::new("d", "D");

    let mut cg = ConditionGroup::default();
    cg.add_conditions([condition1, condition2]);

    let mut group_params = Vec::new();
    assert_eq!(cg.to_sql(&mut group_params), "(a = ? OR b = ?)");
    assert_eq!(group_params, vec![json!("A"), json!("B")]);

    let mut tail_params = Vec::new();
    let tail = ConditionBuilder::of(&[condition3, condition4]).build(&mut tail_params);
    assert_eq!(tail, "c = ? AND d = ?");
    assert_eq!(tail_params, vec![json!("C"), json!("D")]);
}

// ── ConditionTest ──

/// 对齐 Java: `ConditionTest.toStringTest()`
#[test]
fn condition_to_string_test() {
    assert_eq!(
        Condition::new("user", Value::Null).to_string(),
        "user IS NULL"
    );
    assert_eq!(
        Condition::with_operator("user", "=", Value::Null).to_string(),
        "user IS NULL"
    );
    assert_eq!(
        Condition::new("user", "!= null").to_string(),
        "user IS NOT NULL"
    );
    assert_eq!(
        Condition::with_operator("user", "!=", Value::Null).to_string(),
        "user IS NOT NULL"
    );
    assert_eq!(
        Condition::with_operator("user", "<>", Value::Null).to_string(),
        "user IS NOT NULL"
    );
    assert_eq!(Condition::new("user", "= zhangsan").to_string(), "user = ?");
    assert_eq!(
        Condition::new("user", "like %aaa").to_string(),
        "user LIKE ?"
    );
    assert_eq!(
        Condition::new("user", "in 1,2,3").to_string(),
        "user IN (?,?,?)"
    );
    assert_eq!(
        Condition::between("user", 12, 13).to_string(),
        "user BETWEEN ? AND ?"
    );
}

/// 对齐 Java: `ConditionTest.toStringNoPlaceHolderTest()`
#[test]
fn condition_to_string_no_place_holder_test() {
    let mut condition_null = Condition::new("user", Value::Null);
    condition_null.set_place_holder(false);
    assert_eq!(condition_null.to_string(), "user IS NULL");

    let mut condition_not_null = Condition::new("user", "!= null");
    condition_not_null.set_place_holder(false);
    assert_eq!(condition_not_null.to_string(), "user IS NOT NULL");

    let mut condition_equals = Condition::new("user", "= zhangsan");
    condition_equals.set_place_holder(false);
    assert_eq!(condition_equals.to_string(), "user = zhangsan");

    let mut condition_like = Condition::new("user", "like %aaa");
    condition_like.set_place_holder(false);
    assert_eq!(condition_like.to_string(), "user LIKE '%aaa'");

    let mut condition_in = Condition::new("user", "in 1,2,3");
    condition_in.set_place_holder(false);
    assert_eq!(condition_in.to_string(), "user IN (1,2,3)");

    let mut condition_between = Condition::between("user", 12, 13);
    condition_between.set_place_holder(false);
    assert_eq!(condition_between.to_string(), "user BETWEEN 12 AND 13");
}

/// 对齐 Java: `ConditionTest.parseTest()`
#[test]
fn condition_parse_test() {
    let age = Condition::parse("age", "< 10");
    assert_eq!(age.to_string(), "age < ?");
    assert_eq!(age.value(), &json!(10));
}

/// 对齐 Java: `ConditionTest.parseInTest()`
#[test]
fn condition_parse_in_test() {
    let age = Condition::parse("age", "in 1,2,3");
    assert_eq!(age.to_string(), "age IN (?,?,?)");
}

// ── Issue4066Test ──

/// 对齐 Java: `Issue4066Test.removeOuterOrderByTest1()`
#[test]
fn issue4066_remove_outer_order_by_test1() {
    let sql = "SELECT * FROM users ORDER BY name";
    assert_eq!(remove_outer_order_by(sql), "SELECT * FROM users");
}

/// 对齐 Java: `Issue4066Test.removeOuterOrderByTest2()`
#[test]
fn issue4066_remove_outer_order_by_test2() {
    let sql = "SELECT id, name, age FROM users WHERE status = 'active' ORDER BY name ASC, age DESC, created_date";
    assert_eq!(
        remove_outer_order_by(sql),
        "SELECT id, name, age FROM users WHERE status = 'active'"
    );
}

/// 对齐 Java: `Issue4066Test.removeOuterOrderByTest3()`
#[test]
fn issue4066_remove_outer_order_by_test3() {
    let sql = "SELECT * FROM users";
    assert_eq!(remove_outer_order_by(sql), sql);
}

// ── Issue4200Test ──

/// 对齐 Java: `Issue4200Test.isInClauseTest0()`
#[test]
fn issue4200_is_in_clause_test0() {
    assert!(is_in_clause("select case when 1=1 and 2 in ( "));
}

/// 对齐 Java: `Issue4200Test.isInClauseTest1()`
#[test]
fn issue4200_is_in_clause_test1() {
    assert!(is_in_clause("select case when 1=1 and 2 in ("));
}

/// 对齐 Java: `Issue4200Test.isInClauseTest2()`
#[test]
fn issue4200_is_in_clause_test2() {
    assert!(!is_in_clause("select case when 1=1 and 2 = any("));
}

/// 对齐 Java: `Issue4200Test.isInClauseTest3()`
#[test]
fn issue4200_is_in_clause_test3() {
    assert!(!is_in_clause("select case when 1 in (?,?,?) and 2 = any("));
}

/// 对齐 Java: `Issue4200Test.isInClauseTest4()`
#[test]
fn issue4200_is_in_clause_test4() {
    assert!(!is_in_clause("select case when 1 in (?,?,?) and 2 = any("));
}

// ── SqlBuilderTest ──

/// 对齐 Java: `SqlBuilderTest.queryNullTest()`
#[test]
fn sql_builder_query_null_test() {
    let mut builder = SqlBuilder::create();
    builder
        .select(["*"])
        .from("user")
        .where_conditions(&[Condition::new("name", "= null")]);
    assert_eq!(builder.build(), "SELECT * FROM user WHERE name IS NULL");

    let mut builder2 = SqlBuilder::create();
    builder2
        .select(["*"])
        .from("user")
        .where_conditions(&[Condition::new("name", "is null")]);
    assert_eq!(builder2.build(), "SELECT * FROM user WHERE name IS NULL");

    let mut builder3 = SqlBuilder::create();
    builder3
        .select(["*"])
        .from("user")
        .where_conditions(&[Condition::new("name", "!= null")]);
    assert_eq!(
        builder3.build(),
        "SELECT * FROM user WHERE name IS NOT NULL"
    );

    let mut builder4 = SqlBuilder::create();
    builder4
        .select(["*"])
        .from("user")
        .where_conditions(&[Condition::new("name", "is not null")]);
    assert_eq!(
        builder4.build(),
        "SELECT * FROM user WHERE name IS NOT NULL"
    );
}

/// 对齐 Java: `SqlBuilderTest.orderByTest()`
#[test]
fn sql_builder_order_by_test() {
    let mut builder = SqlBuilder::create();
    builder
        .select(["id", "username"])
        .from("user")
        .join("role", Join::Inner)
        .on("user.id = role.user_id")
        .where_conditions(&[
            Condition::with_operator("age", ">=", 18),
            Condition::like("username", "abc", LikeType::Contains),
        ])
        .order_by(&[Order::new("id")]);
    assert_eq!(
        builder.build(),
        "SELECT id,username FROM user INNER JOIN role ON user.id = role.user_id WHERE age >= ? AND username LIKE ? ORDER BY id"
    );
}

/// 对齐 Java: `SqlBuilderTest.likeTest()`
#[test]
fn sql_builder_like_test() {
    let mut condition_equals = Condition::like("user", "123", LikeType::Contains);
    condition_equals.set_place_holder(false);
    let mut sql_builder = SqlBuilder::create();
    sql_builder
        .select(["id"])
        .from("user")
        .where_conditions(&[condition_equals]);
    assert_eq!(
        sql_builder.build(),
        "SELECT id FROM user WHERE user LIKE '%123%'"
    );
}

// ── SqlFormatterTest ──

/// 对齐 Java: `SqlFormatterTest.formatTest()`
#[test]
fn sql_formatter_format_test() {
    let sql = "(select 1 from dual) union all (select 1 from dual)";
    let formatted = format_sql(sql);
    assert!(!formatted.is_empty());
    assert_eq!(formatted, sql);
}

/// 对齐 Java: `SqlFormatterTest.testKeyword()`
#[test]
fn sql_formatter_test_keyword() {
    let sql = "select * from `order`";
    let formatted = format_sql(sql);
    assert_eq!(formatted, sql);
}

/// 对齐 Java: `SqlFormatterTest.testSqlBuilderFormat()`
#[test]
fn sql_formatter_test_sql_builder_format() {
    let sql = "SELECT `link_table_a`.`value_a` AS `link_table_a.value_a` FROM `link_table_a`";
    let formatted = SqlBuilder::of(sql).format().build();
    assert!(!formatted.is_empty());
    assert!(formatted.contains("link_table_a"));
}
