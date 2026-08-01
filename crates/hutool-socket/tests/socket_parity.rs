//! Socket 配置对齐测试。

use std::time::Duration;

use hutool_socket as hs;

#[test]
fn socket_config_test() {
    let config = hs::SocketConfig::new();
    assert!(config.read_timeout() > Duration::ZERO, "read_timeout 应存在");
}
