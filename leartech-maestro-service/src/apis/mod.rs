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

pub mod event_maestro_api;
pub mod event_registration_api;
pub mod health_api;
pub mod latest_events_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn event_maestro_api(&self) -> &dyn event_maestro_api::EventMaestroApi;
    fn event_registration_api(&self) -> &dyn event_registration_api::EventRegistrationApi;
    fn health_api(&self) -> &dyn health_api::HealthApi;
    fn latest_events_api(&self) -> &dyn latest_events_api::LatestEventsApi;
}

pub struct ApiClient {
    event_maestro_api: Box<dyn event_maestro_api::EventMaestroApi>,
    event_registration_api: Box<dyn event_registration_api::EventRegistrationApi>,
    health_api: Box<dyn health_api::HealthApi>,
    latest_events_api: Box<dyn latest_events_api::LatestEventsApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            event_maestro_api: Box::new(event_maestro_api::EventMaestroApiClient::new(configuration.clone())),
            event_registration_api: Box::new(event_registration_api::EventRegistrationApiClient::new(configuration.clone())),
            health_api: Box::new(health_api::HealthApiClient::new(configuration.clone())),
            latest_events_api: Box::new(latest_events_api::LatestEventsApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn event_maestro_api(&self) -> &dyn event_maestro_api::EventMaestroApi {
        self.event_maestro_api.as_ref()
    }
    fn event_registration_api(&self) -> &dyn event_registration_api::EventRegistrationApi {
        self.event_registration_api.as_ref()
    }
    fn health_api(&self) -> &dyn health_api::HealthApi {
        self.health_api.as_ref()
    }
    fn latest_events_api(&self) -> &dyn latest_events_api::LatestEventsApi {
        self.latest_events_api.as_ref()
    }
}

#[cfg(feature = "mockall")]
pub struct MockApiClient {
    pub event_maestro_api_mock: event_maestro_api::MockEventMaestroApi,
    pub event_registration_api_mock: event_registration_api::MockEventRegistrationApi,
    pub health_api_mock: health_api::MockHealthApi,
    pub latest_events_api_mock: latest_events_api::MockLatestEventsApi,
}

#[cfg(feature = "mockall")]
impl MockApiClient {
    pub fn new() -> Self {
        Self {
            event_maestro_api_mock: event_maestro_api::MockEventMaestroApi::new(),
            event_registration_api_mock: event_registration_api::MockEventRegistrationApi::new(),
            health_api_mock: health_api::MockHealthApi::new(),
            latest_events_api_mock: latest_events_api::MockLatestEventsApi::new(),
        }
    }
}

#[cfg(feature = "mockall")]
impl Api for MockApiClient {
    fn event_maestro_api(&self) -> &dyn event_maestro_api::EventMaestroApi {
        &self.event_maestro_api_mock
    }
    fn event_registration_api(&self) -> &dyn event_registration_api::EventRegistrationApi {
        &self.event_registration_api_mock
    }
    fn health_api(&self) -> &dyn health_api::HealthApi {
        &self.health_api_mock
    }
    fn latest_events_api(&self) -> &dyn latest_events_api::LatestEventsApi {
        &self.latest_events_api_mock
    }
}

