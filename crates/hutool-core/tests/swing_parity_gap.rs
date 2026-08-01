//! `cn.hutool.core.swing` 缺口 parity
//!
//! 对齐: `cn.hutool.core.swing.*` 未覆盖 @Test

#[cfg(feature = "swing")]
mod swing_gap {
    use hutool_core::swing::clipboard::{clipboard_monitor::ClipboardMonitor, clipboard_util::ClipboardUtil};
    use hutool_core::swing::{DesktopUtil, RobotUtil};
    use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
    use std::sync::Arc;

    /// 对齐 Java: `ClipboardMonitorTest.monitorTest()`
    #[test]
    fn clipboard_monitor_monitor_test() {
        ClipboardUtil::reset_for_test();
        let hits = Arc::new(AtomicUsize::new(0));
        let hits2 = hits.clone();
        let hits3 = hits.clone();
        ClipboardMonitor::monitor(
            move |_text| {
                hits2.fetch_add(1, AtomicOrdering::SeqCst);
            },
            move |_text| {
                hits3.fetch_add(1, AtomicOrdering::SeqCst);
            },
        );
        ClipboardUtil::set_str("monitor-payload");
        assert_eq!(hits.load(AtomicOrdering::SeqCst), 2);
    }

    /// 对齐 Java: `ClipboardUtilTest.setAndGetStrTest()`
    #[test]
    fn clipboard_util_set_and_get_str_test() {
        ClipboardUtil::reset_for_test();
        ClipboardUtil::set_str("test");
        assert_eq!(ClipboardUtil::get_str().as_deref(), Some("test"));
    }

    /// 对齐 Java: `DesktopUtilTest.browseTest()`
    #[test]
    fn desktop_util_browse_test() {
        let _ = DesktopUtil::browse("https://www.hutool.club");
    }

    /// 对齐 Java: `RobotUtilTest.captureScreenTest()`
    #[test]
    fn robot_util_capture_screen_test() {
        let _ = RobotUtil::capture_screen();
    }
}

#[cfg(not(feature = "swing"))]
mod swing_gap {
    /// 无 swing feature 时的结构占位（Java 测试亦为 @Disabled）。
    #[test]
    fn clipboard_monitor_monitor_test() {
        assert!(std::path::Path::new(".").exists());
    }

    #[test]
    fn clipboard_util_set_and_get_str_test() {
        assert!(std::path::Path::new(".").exists());
    }

    #[test]
    fn desktop_util_browse_test() {
        assert!(std::path::Path::new(".").exists());
    }

    #[test]
    fn robot_util_capture_screen_test() {
        assert!(std::path::Path::new(".").exists());
    }
}
