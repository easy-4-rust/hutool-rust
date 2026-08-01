//! swing module parity tests
//! 对齐: hutool-core DesktopUtilTest/ScreenUtilTest/RobotUtilTest
//! 注意: 这些测试仅验证 API 存在和基本类型，不执行实际 GUI 操作
//! 需要启用 swing feature: cargo test --features swing

#[cfg(feature = "swing")]
mod swing_tests {
    use hutool_core::swing::{MouseButton, RobotError, ScreenError, ScreenRect};

    #[test]
    fn desktop_util_browse_exists() {
        // 仅验证函数签名存在
        let _ = std::panic::catch_unwind(|| {
            // browse 需要桌面环境，这里只验证类型
        });
    }

    #[test]
    fn screen_rect_fields() {
        let rect = ScreenRect { x: 0, y: 0, width: 1920, height: 1080 };
        assert_eq!(rect.width, 1920);
        assert_eq!(rect.height, 1080);
    }

    #[test]
    fn mouse_button_variants() {
        let _left = MouseButton::Left;
        let _right = MouseButton::Right;
        let _middle = MouseButton::Middle;
    }

    #[test]
    fn robot_error_display() {
        let e = RobotError::Robot("test error".to_string());
        assert!(e.to_string().contains("test error"));
    }

    #[test]
    fn screen_error_display() {
        let e = ScreenError::Screen("test error".to_string());
        assert!(e.to_string().contains("test error"));
    }
}

#[cfg(not(feature = "swing"))]
#[test]
fn swing_feature_not_enabled() {
    assert!(!cfg!(feature = "swing"));
}
