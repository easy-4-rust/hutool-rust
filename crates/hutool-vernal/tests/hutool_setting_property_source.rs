//! Hutool `.setting` 到 Vernal Environment 的真实文件装载合同测试。

use std::{fs, sync::Arc};

use hutool_setting::{DEFAULT_ENCODING, Profile, Setting};
use hutool_vernal::HutoolSettingPropertySource;
use vernal_context::{ApplicationEnvironment, EnvironmentError};

#[test]
fn profile_setting_becomes_typed_immutable_vernal_environment() {
    let directory = tempfile::tempdir().expect("temporary setting directory");
    let profile_directory = directory.path().join("dev");
    fs::create_dir(&profile_directory).expect("profile directory");
    fs::write(
        profile_directory.join("application.setting"),
        concat!(
            "application_name=vernal-demo\n",
            "base_url=https://example.test\n",
            "api_url=${base_url}/v1\n",
            "\n",
            "[database]\n",
            "host=127.0.0.1\n",
            "port=5432\n",
        ),
    )
    .expect("setting document");

    let mut profile = Profile::with_options(directory.path(), "dev", DEFAULT_ENCODING, true);
    let source = HutoolSettingPropertySource::from_profile(
        "hutool:application",
        &mut profile,
        "application",
    )
    .expect("Hutool property source");
    assert_eq!(source.len(), 5);

    let mut builder = ApplicationEnvironment::builder();
    builder
        .add_last(Arc::new(source))
        .expect("Vernal property source")
        .active_profile("dev")
        .expect("Vernal active profile");
    let environment = builder.build();

    assert_eq!(
        environment
            .property("application_name")
            .expect("application name")
            .as_deref(),
        Some("vernal-demo")
    );
    assert_eq!(
        environment
            .property("api_url")
            .expect("expanded URL")
            .as_deref(),
        Some("https://example.test/v1")
    );
    assert_eq!(
        environment
            .property("database.host")
            .expect("database host")
            .as_deref(),
        Some("127.0.0.1")
    );
    assert_eq!(
        environment
            .get::<u16>("database.port")
            .expect("typed database port"),
        Some(5432)
    );
    assert_eq!(environment.active_profiles(), ["dev"]);
}

#[test]
fn flattened_key_collision_fails_closed_without_exposing_values() {
    let directory = tempfile::tempdir().expect("temporary setting directory");
    let setting_path = directory.path().join("collision.setting");
    fs::write(
        &setting_path,
        concat!(
            "database.host=default-secret-host\n",
            "\n",
            "[database]\n",
            "host=group-secret-host\n",
        ),
    )
    .expect("collision setting");
    let setting = Setting::from_path(setting_path).expect("Hutool setting");

    let error = HutoolSettingPropertySource::new("hutool:collision", &setting)
        .expect_err("flattened duplicate must be rejected");
    assert!(matches!(
        error,
        EnvironmentError::DuplicatePropertyKey { .. }
    ));
    let debug = format!("{error:?}");
    assert!(!debug.contains("default-secret-host"));
    assert!(!debug.contains("group-secret-host"));
}
