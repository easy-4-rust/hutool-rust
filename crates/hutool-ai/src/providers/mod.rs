//! `cn.hutool.ai.model.*` 子包对齐。
//!
//! 每个厂商一个文件：`XxxProvider`（SPI 工厂，经 `AIServiceFactory::registry()`
//! 注册后按名称路由）+ Java `XxxCommon`/`XxxConfig`/`XxxService` 镜像别名；
//! 厂商端点请求由通用 `ProviderService` + `Operation` 枚举承载。

pub mod deepseek;
pub mod doubao;
pub mod gemini;
pub mod grok;
pub mod hutool;
pub mod ollama;
pub mod openai;

pub use deepseek::DeepSeekProvider;
pub use doubao::DoubaoProvider;
pub use gemini::GeminiProvider;
pub use grok::GrokProvider;
pub use hutool::HutoolProvider;
pub use ollama::OllamaProvider;
pub use openai::OpenAiCompatibleProvider;
