# Togglr Rust SDK

Rust SDK for Togglr feature flag management system.

## Features

- **Feature Evaluation**: Evaluate feature flags with custom context
- **Event Tracking**: Track impression, conversion, and error events
- **Health Monitoring**: Check feature health and auto-disable status
- **Error Reporting**: Report feature execution errors
- **TLS Support**: Secure connections with custom certificates
- **Retry Logic**: Automatic retry with exponential backoff
- **Caching**: Optional LRU cache for feature evaluations
- **Fluent API**: Builder pattern for easy configuration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
togglr-sdk = "1.0.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use togglr_sdk::{TogglrClient, Config, RequestContext, TrackEvent, EventType, create_track_event};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let config = Config::new("your-api-key".to_string());
    let client = TogglrClient::new(config).await?;

    // Evaluate feature
    let context = RequestContext::new()
        .with_user_id("user123")
        .with_country_code("US")
        .with_platform("web");

    let result = client.evaluate_feature("my_feature", context).await?;
    println!("Feature enabled: {}, value: {}", result.enabled, result.value);

    // Track events
    let event = create_track_event("A".to_string(), EventType::Success)
        .with_reward(1.0)
        .with_request_context(context)
        .build();

    client.track_event("my_feature", event).await?;

    Ok(())
}
```

## Configuration

### Basic Configuration

```rust
use togglr_sdk::{Config, ConfigBuilder};

let config = ConfigBuilder::new("api-key".to_string())
    .base_url("https://api.togglr.com".to_string())
    .timeout(Duration::from_secs(5))
    .retries(3)
    .build();
```

### Advanced Configuration

```rust
let config = ConfigBuilder::new("api-key".to_string())
    .base_url("https://api.togglr.com".to_string())
    .timeout(Duration::from_secs(5))
    .retries(3)
    .backoff(BackoffConfig {
        base_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(2),
        factor: 2.0,
    })
    .cache_enabled(true)
    .cache_size(1000)
    .cache_ttl(Duration::from_secs(30))
    .max_connections(100)
    .insecure(false)
    .build();
```

### TLS Configuration

```rust
let config = ConfigBuilder::new("api-key".to_string())
    .client_cert("path/to/client.crt".into())
    .client_key("path/to/client.key".into())
    .ca_cert("path/to/ca.crt".into())
    .insecure(false)
    .build();
```

## Request Context

The `RequestContext` provides a fluent API for setting user attributes:

```rust
use togglr_sdk::RequestContext;
use serde_json::Value;

let context = RequestContext::new()
    .with_user_id("user123")
    .with_user_email("user@example.com")
    .with_country_code("US")
    .with_region("California")
    .with_city("San Francisco")
    .with_platform("web")
    .with_browser("Chrome")
    .with_browser_version("120.0")
    .with_os("macOS")
    .with_os_version("14.0")
    .with_device_type("desktop")
    .with_language("en-US")
    .with_age(25)
    .with_gender("male")
    .set("custom_attr", Value::String("custom_value".to_string()));
```

## Event Tracking

### Basic Event Tracking

```rust
use togglr_sdk::{TrackEvent, EventType, create_track_event};

// Create event
let event = create_track_event("variant_A".to_string(), EventType::Success)
    .with_reward(1.0)
    .with_request_context(context)
    .with_created_at(Utc::now())
    .with_dedup_key("unique_event_id".to_string())
    .build();

// Track event
client.track_event("feature_key", event).await?;
```

### Event Types

- `EventType::Success` - Successful conversion
- `EventType::Failure` - Failed conversion
- `EventType::Error` - Error occurred

### Event Builder API

```rust
let event = create_track_event("variant_A".to_string(), EventType::Success)
    .with_reward(1.0)                    // Optional reward value
    .with_context("key", Value::String("value".to_string()))  // Single context
    .with_contexts(context_map)          // Multiple contexts
    .with_request_context(context)       // Full RequestContext
    .with_created_at(Utc::now())         // Event timestamp
    .with_dedup_key("id".to_string())    // Deduplication key
    .build();
```

## Health Monitoring

```rust
// Check feature health
let health = client.get_feature_health("feature_key").await?;
println!("Enabled: {}, Auto-disabled: {}", health.enabled, health.auto_disabled);
println!("Error rate: {:?}, Threshold: {:?}", health.error_rate, health.threshold);
```

## Error Reporting

```rust
use togglr_sdk::FeatureErrorReport;
use std::collections::HashMap;

let error_report = FeatureErrorReport {
    error_type: "timeout".to_string(),
    error_message: "Service did not respond in 5s".to_string(),
    context: Some({
        let mut ctx = HashMap::new();
        ctx.insert("service".to_string(), Value::String("payment".to_string()));
        ctx.insert("timeout_ms".to_string(), Value::Number(5000.into()));
        ctx
    }),
};

client.report_feature_error("feature_key", error_report).await?;
```

## Error Handling

The SDK uses `TogglrResult<T>` for error handling:

```rust
use togglr_sdk::{TogglrError, TogglrResult};

match client.evaluate_feature("feature", context).await {
    Ok(result) => println!("Feature enabled: {}", result.enabled),
    Err(TogglrError::Timeout(msg)) => println!("Request timed out: {}", msg),
    Err(TogglrError::NetworkError(msg)) => println!("Network error: {}", msg),
    Err(TogglrError::Unauthorized(msg)) => println!("Unauthorized: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Examples

See the `examples/` directory for complete examples:

- `simple_example.rs` - Basic usage example
- `advanced_example.rs` - Advanced configuration and error handling
- `tls_example.rs` - TLS configuration example

Run examples with:

```bash
cargo run --example simple_example
```

## License

MIT License - see LICENSE file for details.
