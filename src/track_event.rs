use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

use crate::context::RequestContext;
use crate::generated::models::{TrackRequest, track_request::EventType as ApiEventType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    Success,
    Failure,
    Error,
}

impl From<EventType> for ApiEventType {
    fn from(event_type: EventType) -> Self {
        match event_type {
            EventType::Success => ApiEventType::Success,
            EventType::Failure => ApiEventType::Failure,
            EventType::Error => ApiEventType::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackEvent {
    pub variant_key: String,
    pub event_type: EventType,
    pub reward: Option<f32>,
    pub context: RequestContext,
    pub created_at: Option<DateTime<Utc>>,
    pub dedup_key: Option<String>,
}

impl TrackEvent {
    pub fn new(variant_key: String, event_type: EventType) -> Self {
        Self {
            variant_key,
            event_type,
            reward: None,
            context: RequestContext::new(),
            created_at: None,
            dedup_key: None,
        }
    }

    pub fn with_reward(mut self, reward: f32) -> Self {
        self.reward = Some(reward);
        self
    }

    pub fn with_context(mut self, key: &str, value: Value) -> Self {
        self.context = self.context.set(key, value);
        self
    }

    pub fn with_contexts(mut self, contexts: HashMap<String, Value>) -> Self {
        self.context = self.context.set_many(contexts);
        self
    }

    pub fn with_request_context(mut self, context: RequestContext) -> Self {
        self.context = context;
        self
    }

    pub fn with_created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn with_dedup_key(mut self, dedup_key: String) -> Self {
        self.dedup_key = Some(dedup_key);
        self
    }

    pub fn to_api_request(&self) -> TrackRequest {
        let mut request = TrackRequest::new(
            self.variant_key.clone(),
            self.event_type.clone().into(),
        );

        if let Some(reward) = self.reward {
            request.reward = Some(reward);
        }

        if !self.context.is_empty() {
            request.context = Some(self.context.to_map());
        }

        if let Some(created_at) = self.created_at {
            request.created_at = Some(created_at.to_rfc3339());
        }

        if let Some(dedup_key) = &self.dedup_key {
            request.dedup_key = Some(dedup_key.clone());
        }

        request
    }
}

pub struct TrackEventBuilder {
    event: TrackEvent,
}

impl TrackEventBuilder {
    pub fn new(variant_key: String, event_type: EventType) -> Self {
        Self {
            event: TrackEvent::new(variant_key, event_type),
        }
    }

    pub fn with_reward(mut self, reward: f32) -> Self {
        self.event = self.event.with_reward(reward);
        self
    }

    pub fn with_context(mut self, key: &str, value: Value) -> Self {
        self.event = self.event.with_context(key, value);
        self
    }

    pub fn with_contexts(mut self, contexts: HashMap<String, Value>) -> Self {
        self.event = self.event.with_contexts(contexts);
        self
    }

    pub fn with_request_context(mut self, context: RequestContext) -> Self {
        self.event = self.event.with_request_context(context);
        self
    }

    pub fn with_created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.event = self.event.with_created_at(created_at);
        self
    }

    pub fn with_dedup_key(mut self, dedup_key: String) -> Self {
        self.event = self.event.with_dedup_key(dedup_key);
        self
    }

    pub fn build(self) -> TrackEvent {
        self.event
    }
}

pub fn create_track_event(variant_key: String, event_type: EventType) -> TrackEventBuilder {
    TrackEventBuilder::new(variant_key, event_type)
}
