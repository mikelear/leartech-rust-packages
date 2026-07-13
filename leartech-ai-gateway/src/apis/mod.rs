use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod chat_api;
pub mod embeddings_api;
pub mod models_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn chat_api(&self) -> &dyn chat_api::ChatApi;
    fn embeddings_api(&self) -> &dyn embeddings_api::EmbeddingsApi;
    fn models_api(&self) -> &dyn models_api::ModelsApi;
}

pub struct ApiClient {
    chat_api: Box<dyn chat_api::ChatApi>,
    embeddings_api: Box<dyn embeddings_api::EmbeddingsApi>,
    models_api: Box<dyn models_api::ModelsApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            chat_api: Box::new(chat_api::ChatApiClient::new(configuration.clone())),
            embeddings_api: Box::new(embeddings_api::EmbeddingsApiClient::new(configuration.clone())),
            models_api: Box::new(models_api::ModelsApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn chat_api(&self) -> &dyn chat_api::ChatApi {
        self.chat_api.as_ref()
    }
    fn embeddings_api(&self) -> &dyn embeddings_api::EmbeddingsApi {
        self.embeddings_api.as_ref()
    }
    fn models_api(&self) -> &dyn models_api::ModelsApi {
        self.models_api.as_ref()
    }
}

#[cfg(feature = "mockall")]
pub struct MockApiClient {
    pub chat_api_mock: chat_api::MockChatApi,
    pub embeddings_api_mock: embeddings_api::MockEmbeddingsApi,
    pub models_api_mock: models_api::MockModelsApi,
}

#[cfg(feature = "mockall")]
impl MockApiClient {
    pub fn new() -> Self {
        Self {
            chat_api_mock: chat_api::MockChatApi::new(),
            embeddings_api_mock: embeddings_api::MockEmbeddingsApi::new(),
            models_api_mock: models_api::MockModelsApi::new(),
        }
    }
}

#[cfg(feature = "mockall")]
impl Api for MockApiClient {
    fn chat_api(&self) -> &dyn chat_api::ChatApi {
        &self.chat_api_mock
    }
    fn embeddings_api(&self) -> &dyn embeddings_api::EmbeddingsApi {
        &self.embeddings_api_mock
    }
    fn models_api(&self) -> &dyn models_api::ModelsApi {
        &self.models_api_mock
    }
}

