//! Extended utilities that do not belong in the core dependency surface.

#![forbid(unsafe_code)]

#[cfg(feature = "archive")]
pub mod archive;
pub mod exceptions;
pub mod expression;
pub mod ftp;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "mail")]
pub mod mail;
#[cfg(feature = "mail")]
pub mod mail_facade;
pub mod prelude;
#[cfg(feature = "qrcode")]
pub mod qrcode;
pub mod spring;
pub mod ssh;
pub mod template;
pub mod tokenizer;
pub mod validation;

#[cfg(feature = "emoji")]
pub mod emoji;
#[cfg(feature = "pinyin")]
pub mod pinyin;

#[cfg(feature = "emoji")]
pub use emoji::{Emoji, EmojiUtil, FitzpatrickAction};
#[cfg(feature = "pinyin")]
pub use pinyin::{
    Bopomofo4jEngine, DefaultPinyinEngine, HoubbPinyinEngine, JPinyinEngine, Pinyin4jEngine,
    PinyinEngine, PinyinException, PinyinFactory, PinyinUtil, TinyPinyinEngine,
};

#[cfg(feature = "archive")]
pub use archive::{CompressUtil, ZipUtil};
pub use exceptions::{
    CompressException, ExpressionException, HutoolException, MailException, QrCodeException,
    TokenizerException,
};
pub use expression::{ExpressionEngine, ExpressionUtil};
pub use ftp::{AbstractFtp, FtpConfig, FtpException, FtpMode};
#[cfg(feature = "image")]
pub use image::ImgUtil;
#[cfg(feature = "mail")]
pub use mail_facade::{Mail, MailAccount, MailUtil};
#[cfg(feature = "qrcode")]
pub use qrcode::{QrCodeUtil, QrConfig};
pub use spring::{
    ApplicationContext, ApplicationContextExt, ConfigurableBeanFactory, SpringUtil,
    enable_spring_util,
};
pub use ssh::{
    ChannelType, Connector, GanymedUtil, JschRuntimeException, JschSessionPool, JschUtil,
};
pub use template::{
    AbstractTemplate, ResourceMode, Template, TemplateBinding, TemplateConfig, TemplateEngine,
    TemplateException, TemplateFactory, TemplateUtil, TemplateValue, default_config,
};
pub use tokenizer::{AbstractResult, TokenizerEngine, TokenizerResult, TokenizerUtil, Word};
pub use validation::{BeanValidationResult, ErrorMessage, ValidationUtil};

/// Errors returned by extra utilities.
#[derive(Debug, thiserror::Error)]
pub enum ExtraError {
    /// QR input could not be encoded.
    #[error(transparent)]
    #[cfg(feature = "qrcode")]
    Qr(#[from] ::qrcode::types::QrError),
    /// ZIP structure is malformed or unsupported.
    #[error(transparent)]
    #[cfg(feature = "archive")]
    Zip(#[from] zip::result::ZipError),
    /// Filesystem I/O failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An archive entry attempted to escape the destination root.
    #[error("unsafe archive path: {0}")]
    UnsafeArchivePath(String),
    /// Symbolic links are not extracted.
    #[error("symbolic link archive entry is not allowed: {0}")]
    SymlinkEntry(String),
    /// Archive resource limits were exceeded.
    #[error("archive limit exceeded: {0}")]
    ArchiveLimit(&'static str),
    /// Image decoding or encoding failed.
    #[cfg(feature = "image")]
    #[error(transparent)]
    Image(#[from] ::image::ImageError),
    /// Image input or decoded dimensions exceeded configured limits.
    #[cfg(feature = "image")]
    #[error("image limit exceeded: {0}")]
    ImageLimit(&'static str),
    /// SMTP transport construction or delivery failed.
    #[cfg(feature = "mail")]
    #[error(transparent)]
    Smtp(#[from] lettre::transport::smtp::Error),
    /// MIME message construction failed.
    #[cfg(feature = "mail")]
    #[error(transparent)]
    MailMessage(#[from] lettre::error::Error),
    /// Mail configuration or message content is invalid.
    #[cfg(feature = "mail")]
    #[error("invalid mail configuration: {0}")]
    InvalidMail(&'static str),
    /// Mail attachments exceeded configured limits.
    #[cfg(feature = "mail")]
    #[error("mail attachment limit exceeded: {0}")]
    MailAttachmentLimit(&'static str),
}

/// Result type for this crate.
pub type Result<T> = std::result::Result<T, ExtraError>;
