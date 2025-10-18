use std::collections::HashMap;
use std::time::Duration;
use chrono::Utc;
use serde_json::Value;
use togglr_sdk::{
    TogglrClient, ConfigBuilder, RequestContext, EventType,
    create_track_event, FeatureErrorReport, BackoffConfig
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api_key = "42b6f8f1-630c-400c-97bd-a3454a07f700";
    
    let backoff = BackoffConfig {
        base_delay: Duration::from_millis(50),
        max_delay: Duration::from_secs(1),
        factor: 1.5,
    };

    let config = ConfigBuilder::new(api_key.to_string())
        .base_url("http://localhost:8090".to_string())
        .timeout(Duration::from_millis(2000))
        .retries(5)
        .backoff(backoff)
        .cache_enabled(true)
        .cache_size(500)
        .cache_ttl(Duration::from_secs(60))
        .max_connections(50)
        .insecure(false)
        .build();

    let client = TogglrClient::new(config).await?;

    println!("=== Advanced Feature Evaluation ===");
    
    let mut contexts = vec![
        RequestContext::new()
            .with_user_id("user1")
            .with_country_code("US")
            .with_platform("mobile")
            .with_device_type("phone")
            .with_os("iOS")
            .with_os_version("17.0")
            .with_app_version("1.2.3"),
        
        RequestContext::new()
            .with_user_id("user2")
            .with_country_code("DE")
            .with_platform("web")
            .with_browser("Firefox")
            .with_browser_version("120.0")
            .with_language("de-DE"),
        
        RequestContext::new()
            .with_user_id("user3")
            .with_country_code("JP")
            .with_platform("web")
            .with_browser("Chrome")
            .with_browser_version("121.0")
            .with_language("ja-JP")
            .with_age(30)
            .with_gender("female"),
    ];

    for (i, context) in contexts.iter().enumerate() {
        match client.evaluate_feature("advanced_feature", context.clone()).await {
            Ok(result) => {
                println!("User {}: enabled={}, value={}", i + 1, result.enabled, result.value);
            }
            Err(e) => {
                println!("User {}: evaluation failed - {}", i + 1, e);
            }
        }
    }

    println!("\n=== Batch Event Tracking ===");
    
    let events = vec![
        create_track_event("A".to_string(), EventType::Success)
            .with_reward(1.0)
            .with_request_context(contexts[0].clone())
            .with_created_at(Utc::now())
            .with_dedup_key("batch_event_1".to_string())
            .build(),
        
        create_track_event("B".to_string(), EventType::Failure)
            .with_reward(0.0)
            .with_request_context(contexts[1].clone())
            .with_created_at(Utc::now())
            .with_dedup_key("batch_event_2".to_string())
            .build(),
        
        create_track_event("A".to_string(), EventType::Error)
            .with_reward(0.0)
            .with_request_context(contexts[2].clone())
            .with_created_at(Utc::now())
            .with_dedup_key("batch_event_3".to_string())
            .build(),
    ];

    for (i, event) in events.iter().enumerate() {
        match client.track_event("advanced_feature", event.clone()).await {
            Ok(()) => println!("Event {} tracked successfully", i + 1),
            Err(e) => println!("Event {} tracking failed: {}", i + 1, e),
        }
    }

    println!("\n=== Error Reporting Scenarios ===");
    
    let error_scenarios = vec![
        FeatureErrorReport {
            error_type: "timeout".to_string(),
            error_message: "Database query timed out after 5s".to_string(),
            context: Some({
                let mut ctx = HashMap::new();
                ctx.insert("service".to_string(), Value::String("database".to_string()));
                ctx.insert("query_timeout_ms".to_string(), Value::Number(5000.into()));
                ctx.insert("query_type".to_string(), Value::String("SELECT".to_string()));
                ctx
            }),
        },
        FeatureErrorReport {
            error_type: "network_error".to_string(),
            error_message: "Failed to connect to external API".to_string(),
            context: Some({
                let mut ctx = HashMap::new();
                ctx.insert("service".to_string(), Value::String("external_api".to_string()));
                ctx.insert("endpoint".to_string(), Value::String("https://api.example.com/data".to_string()));
                ctx.insert("retry_count".to_string(), Value::Number(3.into()));
                ctx
            }),
        },
        FeatureErrorReport {
            error_type: "validation_error".to_string(),
            error_message: "Invalid user input provided".to_string(),
            context: Some({
                let mut ctx = HashMap::new();
                ctx.insert("field".to_string(), Value::String("email".to_string()));
                ctx.insert("value".to_string(), Value::String("invalid-email".to_string()));
                ctx.insert("validation_rule".to_string(), Value::String("email_format".to_string()));
                ctx
            }),
        },
    ];

    for (i, error_report) in error_scenarios.iter().enumerate() {
        match client.report_feature_error("advanced_feature", error_report.clone()).await {
            Ok(()) => println!("Error scenario {} reported successfully", i + 1),
            Err(e) => println!("Error scenario {} reporting failed: {}", i + 1, e),
        }
    }

    println!("\n=== Feature Health Monitoring ===");
    
    let features = vec!["advanced_feature", "basic_feature", "experimental_feature"];
    
    for feature in features {
        match client.get_feature_health(feature).await {
            Ok(health) => {
                println!("Feature '{}':", feature);
                println!("  Enabled: {}", health.enabled);
                println!("  Auto-disabled: {}", health.auto_disabled);
                if let Some(error_rate) = health.error_rate {
                    println!("  Error rate: {:.2}%", error_rate * 100.0);
                }
                if let Some(threshold) = health.threshold {
                    println!("  Threshold: {:.2}%", threshold * 100.0);
                }
                if let Some(last_error) = health.last_error_at {
                    println!("  Last error: {}", last_error);
                }
            }
            Err(e) => {
                println!("Feature '{}': health check failed - {}", feature, e);
            }
        }
    }

    println!("\n=== Performance Test ===");
    
    let start = std::time::Instant::now();
    let mut success_count = 0;
    let mut error_count = 0;
    
    for i in 0..100 {
        let context = RequestContext::new()
            .with_user_id(&format!("perf_user_{}", i))
            .with_country_code("US")
            .with_platform("web");
        
        match client.evaluate_feature("performance_test", context).await {
            Ok(_) => success_count += 1,
            Err(_) => error_count += 1,
        }
    }
    
    let duration = start.elapsed();
    println!("Performance test completed in {:?}", duration);
    println!("Success: {}, Errors: {}", success_count, error_count);
    println!("Average time per request: {:?}", duration / 100);

    Ok(())
}
