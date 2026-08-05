//! 对齐: `cn.hutool.core.util.RuntimeUtilTest`

use hutool_core::RuntimeUtil;

/// 对齐 Java: `RuntimeUtilTest.execTest()` — Java `@Disabled`；Unix 用 echo 做等价 smoke。
#[test]
fn exec_test() {
    #[cfg(unix)]
    {
        let output = RuntimeUtil::exec_for_str(&["echo", "hello"]).expect("exec");
        assert!(output.contains("hello"));
    }
    #[cfg(not(unix))]
    {
        let output = RuntimeUtil::exec_for_str(&["cmd", "/C", "echo hello"]).expect("exec");
        assert!(output.to_lowercase().contains("hello"));
    }
}

/// 对齐 Java: `RuntimeUtilTest.execCmdTest()` — Java `@Disabled`；Unix 用 sh -c。
#[test]
fn exec_cmd_test() {
    #[cfg(unix)]
    {
        let output = RuntimeUtil::exec_for_str(&["sh", "-c", "echo hello"]).expect("exec");
        assert!(output.contains("hello"));
    }
    #[cfg(not(unix))]
    {
        let output = RuntimeUtil::exec_for_str(&["cmd", "/c", "dir"]).expect("exec");
        assert!(!output.is_empty());
    }
}

/// 对齐 Java: `RuntimeUtilTest.execCmdTest2()` — Java `@Disabled`。
#[test]
fn exec_cmd_test2() {
    #[cfg(unix)]
    {
        let output = RuntimeUtil::exec_for_str(&["sh", "-c", "cd /tmp && pwd"]).expect("exec");
        assert!(output.contains("tmp"));
    }
    #[cfg(not(unix))]
    {
        let output =
            RuntimeUtil::exec_for_str(&["cmd", "/c", "cd", "C:\\", "&&", "cd"]).expect("exec");
        assert!(!output.is_empty());
    }
}

/// 对齐 Java: `RuntimeUtilTest.getUsableMemoryTest()`
#[test]
fn get_usable_memory_test() {
    assert!(RuntimeUtil::get_usable_memory() > 0);
}

/// 对齐 Java: `RuntimeUtilTest.getPidTest()`
#[test]
fn get_pid_test() {
    assert!(RuntimeUtil::get_pid() > 0);
}

/// 对齐 Java: `RuntimeUtilTest.getProcessorCountTest()`
#[test]
fn get_processor_count_test() {
    assert!(RuntimeUtil::get_processor_count() > 0);
}

/// 对齐 Java: `RuntimeUtilTest.issueIAB5LWTest()` — Java `@Disabled`；Unix 用 netstat/lsof smoke。
#[test]
fn issue_i_a_b5_l_w_test() {
    #[cfg(target_os = "macos")]
    {
        let output = RuntimeUtil::exec_for_str(&["netstat", "-an"]).expect("netstat");
        assert!(!output.is_empty());
    }
    #[cfg(target_os = "linux")]
    {
        let output = RuntimeUtil::exec_for_str(&["ss", "-tln"]).expect("ss");
        assert!(!output.is_empty());
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        let output = RuntimeUtil::exec_for_str(&["cmd", "/c", "netstat", "-aon"]).expect("netstat");
        assert!(!output.is_empty());
    }
}
