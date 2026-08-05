//! Facade parity for planned→idiomatic Hutool-named surfaces.

use hutool_db::{
    AbstractDb, AnsiSqlDialect, BeanHandler, Column, ColumnIndexInfo, DbConfig, DbSetting,
    DialectName, DialectRunner, DsFactory, Entity, EntityHandler, EntityListHandler,
    EntitySetHandler, HandleHelper, HutoolPage, IndexInfo, MysqlDialect, NumberHandler,
    OracleDialect, PageResultHandler, PooledDataSource, Query, SimpleDataSource, SqlConnRunner,
    SqlExecutor, SqlLog, StringHandler, TableType, TransactionLevel, ValueListHandler, Wrapper,
    dialect::Dialect,
};
use serde::Deserialize;
use serde_json::json;

#[test]
fn dialect_name_and_impls_align() {
    assert!(DialectName::Mysql.match_name("mysql"));
    assert_eq!(
        DialectName::from_product_info("jdbc:postgresql:"),
        Some(DialectName::Postgresql)
    );
    let mysql = MysqlDialect::new();
    assert_eq!(mysql.dialect_name(), DialectName::Mysql);
    assert_eq!(mysql.wrapper().wrap("id"), "`id`");
    assert!(OracleDialect::is_next_val("seq.nextval"));
    let ansi = AnsiSqlDialect::new();
    assert!(ansi.wrap_page_sql("SELECT 1", 10, 0).contains("LIMIT 10"));
}

#[test]
fn query_and_sql_log_facades() {
    let entity = Entity::create_table("user").with("age", 18);
    let mut q = Query::of(&entity);
    q.set_page(HutoolPage::of(0, 10));
    assert_eq!(q.first_table_name(), Some("user"));
    assert!(!q.where_conditions().is_empty());
    assert_eq!(q.page().map(HutoolPage::page_size), Some(10));

    let mut log = SqlLog::new();
    log.init(true, false, true);
    let line = log.log_with_params("select 1", &[json!(1)]).unwrap();
    assert!(line.contains("[SQL]"));
    assert!(
        log.log_for_batch("insert", 3)
            .unwrap()
            .contains("batchSize=3")
    );
}

#[test]
fn ds_config_and_factory_are_explicit() {
    let mut cfg = DbConfig::of("jdbc:sqlite::memory:", "u", "p");
    cfg.set_driver("org.sqlite.JDBC")
        .set_max_active(8)
        .set_min_idle(1)
        .add_conn_props("a", "b");
    assert_eq!(cfg.to_pool_config().max_connections, 8);
    let simple = SimpleDataSource::from_config(cfg.clone());
    assert_eq!(simple.url(), "jdbc:sqlite::memory:");
    let pooled = PooledDataSource::get_data_source(cfg);
    assert_eq!(pooled.config().user(), "u");

    let mut setting = DbSetting::new();
    setting
        .put("db.url", "jdbc:sqlite:x")
        .put("db.user", "root")
        .put("db.pass", "secret")
        .put("db.driver", "org.sqlite.JDBC");
    let factory = DsFactory::create(setting);
    let ds = factory.get_data_source("db");
    assert_eq!(ds.user(), "root");
    assert_eq!(ds.connection_meta().driver(), "org.sqlite.JDBC");
}

#[test]
fn meta_column_index_and_handlers() {
    let mut col = Column::create("user", "id");
    col.init("INTEGER", 11, false, true)
        .set_auto_increment(true)
        .set_comment("pk");
    assert!(col.is_pk());
    assert_eq!(col.type_enum(), hutool_db::JdbcType::Integer);
    assert_eq!(TableType::from_jdbc("TABLE"), TableType::Table);

    let idx_col = ColumnIndexInfo::create("id", "A");
    let mut idx = IndexInfo::new("pk_user", "user");
    idx.set_non_unique(false)
        .set_column_index_info_list(vec![idx_col]);
    assert!(!idx.is_non_unique());
    assert_eq!(idx.column_index_info_list().len(), 1);

    let rows = vec![
        HandleHelper::handle_row([("name", json!("a")), ("age", json!(1))]),
        HandleHelper::handle_row([("name", json!("b")), ("age", json!(2))]),
    ];
    assert_eq!(
        EntityHandler::create()
            .handle(&rows)
            .unwrap()
            .get_str("name")
            .as_deref(),
        Some("a")
    );
    assert_eq!(EntityListHandler::create().handle(&rows).len(), 2);
    assert_eq!(EntitySetHandler::create().handle(&rows).len(), 2);
    let page = PageResultHandler::create(0, 10, 2).handle(rows.clone());
    assert_eq!(page.total(), 2);
    let num_rows = vec![HandleHelper::handle_row([("c", json!(9))])];
    assert_eq!(NumberHandler::create().handle(&num_rows), Some(9));
    assert_eq!(
        StringHandler::create().handle(&num_rows).as_deref(),
        Some("9")
    );
    assert_eq!(ValueListHandler::create().handle(&num_rows)[0].len(), 1);

    #[derive(Debug, Deserialize, PartialEq)]
    struct User {
        name: String,
    }
    let bean = BeanHandler::create()
        .handle::<User>(&rows)
        .unwrap()
        .unwrap();
    assert_eq!(bean.name, "a");
    assert_eq!(TransactionLevel::ReadCommitted.jdbc_level(), 2);
}

#[tokio::test]
async fn runners_delegate_to_db() {
    let pool = hutool_db::memory_pool().await.unwrap();
    let abs = AbstractDb::new(pool.clone());
    let rows = abs.find_all("user").await.unwrap();
    assert!(!rows.is_empty());

    let runner = SqlConnRunner::create(pool.clone());
    let found = runner
        .find(&Entity::create_table("user").with("age", 18))
        .await
        .unwrap();
    assert_eq!(found[0].get_str("name").as_deref(), Some("王五"));

    let dialect_runner = DialectRunner::from_dialect(pool.clone(), &MysqlDialect::new());
    assert_eq!(dialect_runner.dialect_name(), DialectName::Mysql);
    let page = dialect_runner
        .page("select * from user", &HutoolPage::of(0, 2), &[])
        .await
        .unwrap();
    assert_eq!(page.size(), 2);

    let db = hutool_db::Db::new(pool);
    let via_exec = SqlExecutor::query(&db, "select * from user where age = ?", &[json!(18)])
        .await
        .unwrap();
    assert_eq!(via_exec.len(), 1);
    let _ = Wrapper::new('"');
}
