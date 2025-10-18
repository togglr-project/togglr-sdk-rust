use std::time::Duration;

use reqwest::{Client, ClientBuilder};
use tracing::debug;

use crate::config::Config;
use crate::context::RequestContext;
use crate::errors::{TogglrError, TogglrResult};
use crate::track_event::TrackEvent;
use crate::types::{EvaluateResponse, HealthResponse, FeatureHealth, FeatureErrorReport};

use togglr_sdk_generated::apis::default_api::{
    get_feature_health, report_feature_error, sdk_v1_features_feature_key_evaluate_post,
    sdk_v1_health_get, track_feature_event
};
use togglr_sdk_generated::apis::configuration::Configuration;
use togglr_sdk_generated::models::{
    TrackRequest, FeatureErrorReport as ApiFeatureErrorReport
};

pub struct TogglrClient {
    config: Config,
    http_client: Client,
    configuration: Configuration,
}

impl TogglrClient {
    pub async fn new(config: Config) -> TogglrResult<Self> {
        let http_client = Self::build_http_client(&config).await?;
        let mut configuration = Configuration::default();
        configuration.base_path = config.base_url.clone();
        configuration.client = http_client.clone();
        configuration.api_key = Some(togglr_sdk_generated::apis::configuration::ApiKey {
            prefix: None,
            key: config.api_key.clone(),
        });

        Ok(Self {
            config,
            http_client,
            configuration,
        })
    }

    async fn build_http_client(config: &Config) -> TogglrResult<Client> {
        let mut client_builder = ClientBuilder::new()
            .timeout(config.timeout)
            .pool_max_idle_per_host(config.max_connections);

        if config.insecure {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }

        Ok(client_builder.build()?)
    }


    pub async fn health_check(&self) -> TogglrResult<HealthResponse> {
        let response = sdk_v1_health_get(&self.configuration).await
            .map_err(|e| TogglrError::Unknown(format!("Health check failed: {:?}", e)))?;
        Ok(HealthResponse {
            status: match response.status {
                togglr_sdk_generated::models::health_response::Status::Ok => "ok".to_string(),
            },
            server_time: response.server_time,
        })
    }

    pub async fn evaluate_feature(
        &self,
        feature_key: &str,
        context: RequestContext,
    ) -> TogglrResult<EvaluateResponse> {
        let response = sdk_v1_features_feature_key_evaluate_post(
            &self.configuration,
            feature_key,
            context.to_map(),
        ).await
        .map_err(|e| TogglrError::Unknown(format!("Feature evaluation failed: {:?}", e)))?;

        Ok(EvaluateResponse {
            feature_key: response.feature_key,
            enabled: response.enabled,
            value: response.value,
        })
    }

    pub async fn track_event(
        &self,
        feature_key: &str,
        event: TrackEvent,
    ) -> TogglrResult<()> {
        let api_request = event.to_api_request();
        
        self.track_event_with_retries(feature_key, api_request).await
    }

    async fn track_event_with_retries(
        &self,
        feature_key: &str,
        request: TrackRequest,
    ) -> TogglrResult<()> {
        let mut last_error = None;

        for attempt in 0..=self.config.retries {
            if attempt > 0 {
                let delay = self.calculate_backoff_delay(attempt);
                debug!("Retrying track event after delay: {:?} (attempt {})", delay, attempt);
                tokio::time::sleep(delay).await;
            }

            match self.track_event_single(feature_key, &request).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    last_error = Some(e);
                    if !self.should_retry(&last_error.as_ref().unwrap()) {
                        debug!("Not retrying track event due to error type: {:?}", last_error);
                        break;
                    }
                    debug!("Retrying track event due to error: {:?} (attempt {})", last_error, attempt);
                }
            }
        }

        Err(last_error.unwrap())
    }

    async fn track_event_single(
        &self,
        feature_key: &str,
        request: &TrackRequest,
    ) -> TogglrResult<()> {
        track_feature_event(&self.configuration, feature_key, request.clone())
            .await
            .map_err(|e| match e {
                togglr_sdk_generated::apis::Error::Reqwest(reqwest_err) => {
                    if reqwest_err.is_timeout() {
                        TogglrError::Timeout(reqwest_err.to_string())
                    } else if reqwest_err.is_connect() {
                        TogglrError::NetworkError(reqwest_err.to_string())
                    } else {
                        TogglrError::HttpError(reqwest_err)
                    }
                }
                togglr_sdk_generated::apis::Error::Serde(serde_err) => {
                    TogglrError::SerializationError(serde_err)
                }
                togglr_sdk_generated::apis::Error::ResponseError(response) => {
                    TogglrError::Unknown(format!("Response error: {:?}", response))
                }
                _ => TogglrError::Unknown(format!("Unknown error: {:?}", e)),
            })?;

        Ok(())
    }

    pub async fn report_feature_error(
        &self,
        feature_key: &str,
        error_report: FeatureErrorReport,
    ) -> TogglrResult<()> {
        let api_request = ApiFeatureErrorReport {
            error_type: error_report.error_type,
            error_message: error_report.error_message,
            context: error_report.context,
        };

        report_feature_error(&self.configuration, feature_key, api_request)
            .await
            .map_err(|e| TogglrError::Unknown(format!("Failed to report error: {:?}", e)))?;

        Ok(())
    }

    pub async fn get_feature_health(&self, feature_key: &str) -> TogglrResult<FeatureHealth> {
        let response = get_feature_health(&self.configuration, feature_key)
            .await
            .map_err(|e| TogglrError::Unknown(format!("Failed to get feature health: {:?}", e)))?;

        Ok(FeatureHealth {
            feature_key: response.feature_key,
            environment_key: response.environment_key,
            enabled: response.enabled,
            auto_disabled: response.auto_disabled,
            error_rate: response.error_rate,
            threshold: response.threshold,
            last_error_at: response.last_error_at,
        })
    }

    fn calculate_backoff_delay(&self, attempt: u32) -> Duration {
        let delay_ms = (self.config.backoff.base_delay.as_millis() as f64
            * self.config.backoff.factor.powi(attempt as i32 - 1))
            .min(self.config.backoff.max_delay.as_millis() as f64) as u64;

        Duration::from_millis(delay_ms)
    }

    fn should_retry(&self, error: &TogglrError) -> bool {
        match error {
            TogglrError::Timeout(_) | TogglrError::NetworkError(_) | TogglrError::HttpError(_) => true,
            TogglrError::InternalServerError(_) => true,
            _ => false,
        }
    }
}

