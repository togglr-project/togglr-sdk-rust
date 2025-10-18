use std::collections::HashMap;
use chrono::Utc;
use serde_json::Value;
use togglr_sdk::{
    TogglrClient, ConfigBuilder, RequestContext, EventType,
    create_track_event, FeatureErrorReport
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api_key = "42b6f8f1-630c-400c-97bd-a3454a07f700";
    let config = ConfigBuilder::new(api_key.to_string())
        .base_url("http://localhost:8090".to_string())
        .timeout(std::time::Duration::from_millis(5000))
        .retries(3)
        .cache_enabled(true)
        .cache_size(1000)
        .cache_ttl(std::time::Duration::from_secs(30))
        .build();

    let client = TogglrClient::new(config).await?;

    println!("=== Health Check ===");
    match client.health_check().await {
        Ok(health) => println!("Health check passed: {:?}", health),
        Err(e) => println!("Health check failed: {}", e),
    }

    println!("\n=== Feature Evaluation ===");
    let context = RequestContext::new()
        .with_user_id("user123")
        .with_country_code("US")
        .with_platform("web")
        .set("custom_attr", Value::String("custom_value".to_string()));

    match client.evaluate_feature("new_ui", context).await {
        Ok(result) => {
            println!("Feature 'new_ui': enabled={}, value={}", result.enabled, result.value);
        }
        Err(e) => println!("Feature evaluation failed: {}", e),
    }

    println!("\n=== Track Events ===");
    
    let impression_event = create_track_event("A".to_string(), EventType::Success)
        .with_reward(0.0)
        .with_request_context(
            RequestContext::new()
                .with_user_id("user123")
                .with_country_code("US")
                .with_platform("web")
        )
        .with_created_at(Utc::now())
        .with_dedup_key("impression_123".to_string())
        .build();

    match client.track_event("new_ui", impression_event).await {
        Ok(()) => println!("Impression event tracked successfully"),
        Err(e) => println!("Error tracking impression event: {}", e),
    }

    let conversion_event = create_track_event("A".to_string(), EventType::Success)
        .with_reward(1.0)
        .with_request_context(
            RequestContext::new()
                .with_user_id("user123")
                .with_country_code("US")
                .with_platform("web")
        )
        .with_created_at(Utc::now())
        .with_dedup_key("conversion_123".to_string())
        .build();

    match client.track_event("new_ui", conversion_event).await {
        Ok(()) => println!("Conversion event tracked successfully"),
        Err(e) => println!("Error tracking conversion event: {}", e),
    }

    let error_event = create_track_event("B".to_string(), EventType::Error)
        .with_reward(0.0)
        .with_request_context(
            RequestContext::new()
                .with_user_id("user123")
                .with_country_code("US")
                .with_platform("web")
        )
        .with_created_at(Utc::now())
        .with_dedup_key("error_123".to_string())
        .build();

    match client.track_event("new_ui", error_event).await {
        Ok(()) => println!("Error event tracked successfully"),
        Err(e) => println!("Error tracking error event: {}", e),
    }

    println!("\n=== Feature Health ===");
    match client.get_feature_health("new_ui").await {
        Ok(health) => {
            println!("Feature health: enabled={}, auto_disabled={}", 
                health.enabled, health.auto_disabled);
            if let Some(error_rate) = health.error_rate {
                println!("Error rate: {}, threshold: {:?}", 
                    error_rate, health.threshold);
            }
        }
        Err(e) => println!("Failed to get feature health: {}", e),
    }

    println!("\n=== Error Reporting ===");
    let error_report = FeatureErrorReport {
        error_type: "timeout".to_string(),
        error_message: "Service X did not respond in 5s".to_string(),
        context: Some({
            let mut ctx = HashMap::new();
            ctx.insert("service".to_string(), Value::String("service_x".to_string()));
            ctx.insert("timeout_ms".to_string(), Value::Number(5000.into()));
            ctx
        }),
    };

    match client.report_feature_error("new_ui", error_report).await {
        Ok(()) => println!("Error reported successfully"),
        Err(e) => println!("Error reporting failed: {}", e),
    }

    Ok(())
}