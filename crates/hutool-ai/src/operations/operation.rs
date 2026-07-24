//! Provider capability model used by the thin Hutool-compatible facade.

#![allow(missing_docs, clippy::enum_glob_use, clippy::match_same_arms)]

use crate::Message;
use serde_json::{Map, Value, json};
use std::{path::PathBuf, sync::Arc};

use super::video_parameter::VideoParameter;

/// All capabilities exposed by Hutool AI providers.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Operation {
    /// Chat completion.
    Chat { messages: Vec<Message> },
    /// Chat with image URLs or data URLs.
    Vision {
        prompt: String,
        images: Vec<String>,
        detail: String,
    },
    /// Provider-specific beta/reasoning prompt.
    Beta { prompt: String },
    /// Grok message completion with a token ceiling.
    Message {
        messages: Vec<Message>,
        max_tokens: u32,
    },
    /// Provider model catalogue.
    ListModels,
    /// Fetch one model.
    GetModel { id: String },
    /// Grok language-model catalogue.
    ListLanguageModels,
    /// Fetch one Grok language model.
    GetLanguageModel { id: String },
    /// `DeepSeek` account balance.
    Balance,
    /// Text embedding.
    EmbedText { inputs: Vec<String> },
    /// Multimodal embedding.
    EmbedVision { text: String, image: String },
    /// Tokenization.
    Tokenize { texts: Vec<String> },
    /// Image generation.
    GenerateImage { prompt: String },
    /// `OpenAI` image editing.
    EditImage {
        prompt: String,
        image: PathBuf,
        mask: Option<PathBuf>,
    },
    /// `OpenAI` image variation.
    VaryImage { image: PathBuf },
    /// Text-to-speech.
    TextToSpeech { input: String, voice: String },
    /// Speech-to-text.
    SpeechToText { file: PathBuf },
    /// Content moderation.
    Moderate {
        text: String,
        image_url: Option<String>,
    },
    /// Video generation task.
    CreateVideo {
        text: String,
        image: Option<String>,
        parameters: Vec<VideoParameter>,
    },
    /// Poll video task or Gemini operation.
    GetVideo { id: String },
    /// Doubao bot chat.
    BotChat { messages: Vec<Message> },
    /// Doubao batch chat.
    BatchChat { messages: Vec<Message> },
    /// Doubao context creation.
    CreateContext {
        messages: Vec<Message>,
        mode: String,
    },
    /// Doubao context chat.
    ContextChat {
        messages: Vec<Message>,
        context_id: String,
    },
    /// Gemini multimodal generation.
    Multimodal { prompt: String, media: Vec<String> },
    /// Gemini JSON-mode chat.
    JsonChat { messages: Vec<Message> },
    /// Gemini file upload.
    UploadFile { file: PathBuf },
    /// Ollama generation.
    Generate {
        prompt: String,
        format: Option<String>,
    },
    /// Ollama embeddings.
    Embeddings { prompt: String },
    /// Ollama model inspection.
    ShowModel { name: String },
    /// Ollama model pull.
    PullModel { name: String },
    /// Ollama model deletion.
    DeleteModel { name: String },
    /// Ollama model copy.
    CopyModel { source: String, destination: String },
    /// Fetch an already-deferred Grok completion.
    DeferredCompletion { request_id: String },
}

impl Operation {
    pub(crate) fn endpoint(&self, provider: crate::ModelName, model: &str) -> String {
        use Operation::*;
        if provider == crate::ModelName::Gemini {
            return match self {
                Chat { .. } | Multimodal { .. } | JsonChat { .. } => {
                    format!("/models/{model}:generateContent")
                }
                GenerateImage { .. } => format!("/models/{model}:predict"),
                CreateVideo { .. } => format!("/models/{model}:predictLongRunning"),
                GetVideo { id } => format!("/{id}"),
                TextToSpeech { .. } => format!("/models/{model}:generateContent"),
                UploadFile { .. } => "/upload/v1beta/files".into(),
                _ => "/models".into(),
            };
        }
        if provider == crate::ModelName::Ollama {
            return match self {
                Chat { .. } => "/api/chat",
                Generate { .. } => "/api/generate",
                Embeddings { .. } | EmbedText { .. } => "/api/embeddings",
                ListModels => "/api/tags",
                ShowModel { .. } => "/api/show",
                PullModel { .. } => "/api/pull",
                DeleteModel { .. } => "/api/delete",
                CopyModel { .. } => "/api/copy",
                _ => "/api/generate",
            }
            .into();
        }
        match self {
            Chat { .. } | Vision { .. } | Message { .. } | BotChat { .. } | ContextChat { .. } => {
                "/chat/completions"
            }
            Beta { .. } => "/beta/completions",
            ListModels => "/models",
            GetModel { id } => return format!("/models/{id}"),
            ListLanguageModels => "/language-models",
            GetLanguageModel { id } => return format!("/language-models/{id}"),
            Balance => "/user/balance",
            EmbedText { .. } | Embeddings { .. } => "/embeddings",
            EmbedVision { .. } => "/embeddings/multimodal",
            Tokenize { .. } => "/tokenization",
            GenerateImage { .. } => "/images/generations",
            EditImage { .. } => "/images/edits",
            VaryImage { .. } => "/images/variations",
            TextToSpeech { .. } => "/audio/speech",
            SpeechToText { .. } => "/audio/transcriptions",
            Moderate { .. } => "/moderations",
            CreateVideo { .. } => "/contents/generations/tasks",
            GetVideo { id } => return format!("/contents/generations/tasks/{id}"),
            BatchChat { .. } => "/batch/chat/completions",
            CreateContext { .. } => "/context/create",
            Multimodal { .. } | JsonChat { .. } => "/chat/completions",
            UploadFile { .. } => "/files",
            Generate { .. } => "/completions",
            ShowModel { .. } | PullModel { .. } | DeleteModel { .. } | CopyModel { .. } => {
                "/models"
            }
            DeferredCompletion { request_id } => {
                return format!("/chat/deferred-completion/{request_id}");
            }
        }
        .into()
    }

    pub(crate) fn payload(
        &self,
        model: &str,
        additional: &Map<String, Value>,
        stream: bool,
    ) -> Value {
        use Operation::*;
        let mut value = match self {
            Chat { messages } | BotChat { messages } | BatchChat { messages } => {
                json!({"model": model, "messages": messages, "stream": stream})
            }
            Vision {
                prompt,
                images,
                detail,
            } => {
                let mut content = vec![json!({"type":"text", "text":prompt})];
                content.extend(images.iter().map(
                    |image| json!({"type":"image_url", "image_url":{"url":image,"detail":detail}}),
                ));
                json!({"model":model,"messages":[{"role":"user","content":content}],"stream":stream})
            }
            Beta { prompt } | Generate { prompt, .. } => {
                json!({"model":model,"prompt":prompt,"stream":stream})
            }
            Message {
                messages,
                max_tokens,
            } => json!({"model":model,"messages":messages,"max_tokens":max_tokens,"stream":stream}),
            EmbedText { inputs } => json!({"model":model,"input":inputs}),
            EmbedVision { text, image } => {
                json!({"model":model,"input":[{"type":"text","text":text},{"type":"image_url","image_url":image}]})
            }
            Tokenize { texts } => json!({"model":model,"text":texts}),
            GenerateImage { prompt } => json!({"model":model,"prompt":prompt}),
            EditImage {
                prompt,
                image,
                mask,
            } => json!({"model":model,"prompt":prompt,"image":image,"mask":mask}),
            VaryImage { image } => json!({"model":model,"image":image}),
            TextToSpeech { input, voice } => json!({"model":model,"input":input,"voice":voice}),
            SpeechToText { file } | UploadFile { file } => json!({"model":model,"file":file}),
            Moderate { text, image_url } => {
                json!({"model":model,"input":{"text":text,"image_url":image_url}})
            }
            CreateVideo {
                text,
                image,
                parameters,
            } => {
                json!({"model":model,"text":text,"image":image,"parameters":parameters.iter().map(|p| json!({"type":p.kind,"value":p.value})).collect::<Vec<_>>() })
            }
            GetVideo { id } | GetModel { id } | GetLanguageModel { id } => json!({"id":id}),
            CreateContext { messages, mode } => {
                json!({"model":model,"messages":messages,"mode":mode})
            }
            ContextChat {
                messages,
                context_id,
            } => json!({"model":model,"messages":messages,"context_id":context_id,"stream":stream}),
            Multimodal { prompt, media } => {
                json!({"contents":[{"parts":[{"text":prompt},{"media":media}]}]})
            }
            JsonChat { messages } => {
                json!({"model":model,"messages":messages,"response_format":{"type":"json_object"}})
            }
            Embeddings { prompt } => json!({"model":model,"prompt":prompt}),
            ShowModel { name } | PullModel { name } | DeleteModel { name } => json!({"name":name}),
            CopyModel {
                source,
                destination,
            } => json!({"source":source,"destination":destination}),
            DeferredCompletion { request_id } => json!({"request_id":request_id}),
            ListModels | ListLanguageModels | Balance => Value::Object(Map::new()),
        };
        let object = value
            .as_object_mut()
            .expect("all operation payloads are JSON objects");
        for (key, item) in additional {
            object.entry(key.clone()).or_insert_with(|| item.clone());
        }
        if let Generate {
            format: Some(format),
            ..
        } = self
        {
            object.insert("format".into(), Value::String(format.clone()));
        }
        value
    }
}
