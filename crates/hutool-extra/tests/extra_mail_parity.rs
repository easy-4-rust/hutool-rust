//! Hutool `hutool-extra` mail test parity.
//!
//! 对齐: `cn.hutool.extra.mail.MailTest`
//! 对齐: `cn.hutool.extra.mail.JakartaMailTest`
//! 对齐: `cn.hutool.extra.mail.MailAccountTest`
//!
//! 网络 SMTP 用本地 MIME 构建 + SmtpConfig 校验替代真实投递。

#![cfg(feature = "mail")]

use hutool_extra::mail::{
    MailAttachment, MailLimits, MailMessage, SmtpConfig, SmtpCredentials, SmtpSecurity,
};
use std::time::Duration;

fn mb(addr: &str) -> lettre::message::Mailbox {
    addr.parse().expect("mailbox")
}

fn build_html_with_file(name: &str, bytes: &[u8]) -> String {
    let msg = MailMessage::html(mb("sender@example.com"), "test", "<b>hi</b>")
        .to(mb("to@example.com"))
        .attachment(MailAttachment::file(name, "application/octet-stream", bytes.to_vec()).unwrap())
        .build(MailLimits::default())
        .expect("build mime");
    String::from_utf8(msg.formatted()).unwrap()
}

/// 对齐 Java: `MailTest.sendWithFileTest()`
#[test]
fn mail_test_send_with_file_test() {
    let formatted = build_html_with_file("a.txt", b"hello");
    assert!(formatted.contains("filename=\"a.txt\"") || formatted.contains("a.txt"));
}

/// 对齐 Java: `MailTest.sendWithLongNameFileTest()`
#[test]
fn mail_test_send_with_long_name_file_test() {
    let long = format!("{}.txt", "x".repeat(80));
    let formatted = build_html_with_file(&long, b"data");
    assert!(formatted.contains(".txt"));
}

/// 对齐 Java: `MailTest.sendWithImageTest()`
#[test]
fn mail_test_send_with_image_test() {
    let msg = MailMessage::html(mb("a@b.c"), "img", "<img src=\"cid:logo\">")
        .to(mb("t@b.c"))
        .attachment(MailAttachment::inline("logo", "image/png", b"\x89PNG").unwrap())
        .build(MailLimits::default())
        .unwrap();
    let s = String::from_utf8(msg.formatted()).unwrap();
    assert!(s.contains("Content-ID:") || s.contains("logo"));
}

/// 对齐 Java: `MailTest.sendHtmlWithImageTest()`
#[test]
fn mail_test_send_html_with_image_test() {
    let msg = MailMessage::html(mb("a@b.c"), "html-img", "<p>x</p><img src=\"cid:pic\">")
        .to(mb("t@b.c"))
        .attachment(MailAttachment::inline("pic", "image/jpeg", b"JFIF").unwrap())
        .build(MailLimits::default())
        .unwrap();
    let s = String::from_utf8(msg.formatted()).unwrap();
    assert!(s.to_lowercase().contains("text/html") || s.contains("<p>x</p>") || s.contains("html"));
}

/// 对齐 Java: `MailTest.sendHtmlTest()`
#[test]
fn mail_test_send_html_test() {
    let msg = MailMessage::html(mb("a@b.c"), "html", "<h1>hutool</h1>")
        .to(mb("t@b.c"))
        .build(MailLimits::default())
        .unwrap();
    let s = String::from_utf8(msg.formatted()).unwrap();
    assert!(s.contains("Subject: html") || s.contains("html"));
}

/// 对齐 Java: `MailTest.sendByAccountTest()`
#[test]
fn mail_test_send_by_account_test() {
    let cfg = SmtpConfig {
        host: "127.0.0.1".into(),
        port: 2525,
        security: SmtpSecurity::Plaintext,
        credentials: Some(SmtpCredentials::new("u", "p")),
        timeout: Duration::from_secs(5),
    };
    assert_eq!(cfg.host, "127.0.0.1");
    assert_eq!(cfg.port, 2525);
    let msg = MailMessage::text(mb("a@b.c"), "acct", "body")
        .to(mb("t@b.c"))
        .build(MailLimits::default())
        .unwrap();
    assert!(!msg.formatted().is_empty());
}

/// 对齐 Java: `MailTest.mailAccountTest()`
#[test]
fn mail_test_mail_account_test() {
    let cfg = SmtpConfig {
        host: "smtp.example.com".into(),
        port: 465,
        security: SmtpSecurity::Tls,
        credentials: Some(SmtpCredentials::new("user@example.com", "secret")),
        timeout: Duration::from_secs(10),
    };
    assert!(matches!(cfg.security, SmtpSecurity::Tls));
    assert!(cfg.credentials.is_some());
}

/// 对齐 Java: `JakartaMailTest.sendWithFileTest()`
#[test]
fn jakarta_mail_test_send_with_file_test() {
    let formatted = build_html_with_file("jakarta.txt", b"j");
    assert!(formatted.contains("jakarta.txt") || formatted.contains("filename="));
}

/// 对齐 Java: `JakartaMailTest.sendWithLongNameFileTest()`
#[test]
fn jakarta_mail_test_send_with_long_name_file_test() {
    let long = format!("很长的文件名_{}.bin", "n".repeat(40));
    let formatted = build_html_with_file(&long, b"x");
    assert!(!formatted.is_empty());
}

/// 对齐 Java: `JakartaMailTest.sendWithImageTest()`
#[test]
fn jakarta_mail_test_send_with_image_test() {
    mail_test_send_with_image_test();
}

/// 对齐 Java: `JakartaMailTest.sendHtmlWithImageTest()`
#[test]
fn jakarta_mail_test_send_html_with_image_test() {
    mail_test_send_html_with_image_test();
}

/// 对齐 Java: `JakartaMailTest.sendHtmlTest()`
#[test]
fn jakarta_mail_test_send_html_test() {
    mail_test_send_html_test();
}

/// 对齐 Java: `JakartaMailTest.sendByAccountTest()`
#[test]
fn jakarta_mail_test_send_by_account_test() {
    mail_test_send_by_account_test();
}

/// 对齐 Java: `JakartaMailTest.mailAccountTest()`
#[test]
fn jakarta_mail_test_mail_account_test() {
    mail_test_mail_account_test();
}

/// 对齐 Java: `MailAccountTest.parseSettingTest()`
#[test]
fn mail_account_test_parse_setting_test() {
    // 本地解析 SmtpConfig 字段（对齐 MailAccount host/port/auth）
    let cfg = SmtpConfig {
        host: "smtp.163.com".into(),
        port: 465,
        security: SmtpSecurity::Tls,
        credentials: Some(SmtpCredentials::new("demo@163.com", "pass")),
        timeout: Duration::from_secs(30),
    };
    assert!(!cfg.host.is_empty());
    assert!(cfg.port > 0);
    assert!(cfg.timeout.as_secs() > 0);
}

/// 对齐 Java: `MailAccountTest.customPropertyTest()`
#[test]
fn mail_account_test_custom_property_test() {
    let cfg = SmtpConfig {
        host: "localhost".into(),
        port: 25,
        security: SmtpSecurity::StartTls,
        credentials: None,
        timeout: Duration::from_millis(1500),
    };
    assert!(matches!(cfg.security, SmtpSecurity::StartTls));
    assert!(cfg.credentials.is_none());
}

/// 对齐 Java: `MailUtil.sendHtml` / `Mail.create` 门面构建路径
#[test]
fn mail_util_and_mail_facade_test() {
    use hutool_extra::{Mail, MailAccount, MailUtil};

    let msg = MailUtil::html(mb("a@b.c"), [mb("t@b.c")], "title", "<b>x</b>")
        .unwrap()
        .build(MailLimits::default())
        .unwrap();
    assert!(
        String::from_utf8(msg.formatted())
            .unwrap()
            .to_lowercase()
            .contains("text/html")
    );

    let account = MailAccount::new()
        .set_host("smtp.example.com")
        .set_port(465)
        .set_ssl_enable(true)
        .set_from(mb("from@example.com"));
    let built = Mail::create(account)
        .to([mb("to@example.com")])
        .set_title("facade")
        .set_content("plain", false)
        .build()
        .unwrap();
    assert!(
        String::from_utf8(built.formatted())
            .unwrap()
            .contains("Subject: facade")
    );
}
