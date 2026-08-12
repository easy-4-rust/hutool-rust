//! Hutool-named mail facades over injectable SMTP/MIME helpers.
//!
//! 对齐: `cn.hutool.extra.mail.MailUtil`
//! 对齐: `cn.hutool.extra.mail.Mail`
//! 对齐: `cn.hutool.extra.mail.MailAccount`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/mail/

mod mail;
mod mail_account;
mod mail_util;

pub use mail::Mail;
pub use mail_account::MailAccount;
pub use mail_util::MailUtil;
