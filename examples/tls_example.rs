use std::path::PathBuf;
use togglr_sdk::{TogglrClient, ConfigBuilder, RequestContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api_key = "42b6f8f1-630c-400c-97bd-a3454a07f700";
    
    println!("=== TLS Configuration Examples ===");

    // Example 1: Insecure connection (for development/testing)
    println!("\n1. Insecure connection (development only):");
    let insecure_config = ConfigBuilder::new(api_key.to_string())
        .base_url("https://localhost:8090".to_string())
        .insecure(true)
        .build();

    match TogglrClient::new(insecure_config).await {
        Ok(client) => {
            println!("✓ Insecure client created successfully");
            match client.health_check().await {
                Ok(health) => println!("✓ Health check passed: {:?}", health),
                Err(e) => println!("✗ Health check failed: {}", e),
            }
        }
        Err(e) => println!("✗ Failed to create insecure client: {}", e),
    }

    // Example 2: Custom CA certificate
    println!("\n2. Custom CA certificate:");
    let ca_cert_config = ConfigBuilder::new(api_key.to_string())
        .base_url("https://api.togglr.com".to_string())
        .ca_cert(PathBuf::from("certs/ca.crt"))
        .insecure(false)
        .build();

    match TogglrClient::new(ca_cert_config).await {
        Ok(client) => {
            println!("✓ CA certificate client created successfully");
            match client.health_check().await {
                Ok(health) => println!("✓ Health check passed: {:?}", health),
                Err(e) => println!("✗ Health check failed: {}", e),
            }
        }
        Err(e) => println!("✗ Failed to create CA certificate client: {}", e),
    }

    // Example 3: Client certificate authentication
    println!("\n3. Client certificate authentication:");
    let client_cert_config = ConfigBuilder::new(api_key.to_string())
        .base_url("https://api.togglr.com".to_string())
        .client_cert(PathBuf::from("certs/client.crt"))
        .client_key(PathBuf::from("certs/client.key"))
        .ca_cert(PathBuf::from("certs/ca.crt"))
        .insecure(false)
        .build();

    match TogglrClient::new(client_cert_config).await {
        Ok(client) => {
            println!("✓ Client certificate client created successfully");
            match client.health_check().await {
                Ok(health) => println!("✓ Health check passed: {:?}", health),
                Err(e) => println!("✗ Health check failed: {}", e),
            }
        }
        Err(e) => println!("✗ Failed to create client certificate client: {}", e),
    }

    // Example 4: Production configuration with all security features
    println!("\n4. Production configuration:");
    let production_config = ConfigBuilder::new(api_key.to_string())
        .base_url("https://api.togglr.com".to_string())
        .timeout(std::time::Duration::from_secs(10))
        .retries(3)
        .cache_enabled(true)
        .cache_size(1000)
        .cache_ttl(std::time::Duration::from_secs(300))
        .max_connections(100)
        .ca_cert(PathBuf::from("certs/ca.crt"))
        .insecure(false)
        .build();

    match TogglrClient::new(production_config).await {
        Ok(client) => {
            println!("✓ Production client created successfully");
            
            // Test feature evaluation
            let context = RequestContext::new()
                .with_user_id("tls_test_user")
                .with_country_code("US")
                .with_platform("web")
                .with_browser("Chrome")
                .with_os("Linux");

            match client.evaluate_feature("tls_test_feature", context).await {
                Ok(result) => {
                    println!("✓ Feature evaluation successful: enabled={}, value={}", 
                        result.enabled, result.value);
                }
                Err(e) => println!("✗ Feature evaluation failed: {}", e),
            }

            // Test health monitoring
            match client.get_feature_health("tls_test_feature").await {
                Ok(health) => {
                    println!("✓ Feature health check successful: enabled={}, auto_disabled={}", 
                        health.enabled, health.auto_disabled);
                }
                Err(e) => println!("✗ Feature health check failed: {}", e),
            }
        }
        Err(e) => println!("✗ Failed to create production client: {}", e),
    }

    println!("\n=== TLS Configuration Notes ===");
    println!("• Use 'insecure: true' only for development/testing");
    println!("• Always use proper CA certificates in production");
    println!("• Client certificates provide additional authentication");
    println!("• Monitor certificate expiration dates");
    println!("• Use strong cipher suites and TLS 1.2+");

    Ok(())
}
