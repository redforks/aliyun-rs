//! Integration tests for Aliyun Function Compute API.
//!
//! These tests validate that the Aliyun server accepts our requests.
//! Set TEST_ALI_ACCESS_KEY and TEST_ALI_SECRET environment variables to run.

use ali_acs::AccessKeySecret;
use ali_acs::fc::{Connection, Endpoint};
use rand::Rng;

/// Helper to get the connection from environment variables
fn test_connection() -> Connection {
    let access_key = std::env::var("TEST_ALI_ACCESS_KEY")
        .expect("TEST_ALI_ACCESS_KEY environment variable not set");
    let secret = AccessKeySecret::new(
        access_key,
        std::env::var("TEST_ALI_SECRET").expect("TEST_ALI_SECRET environment variable not set"),
    );
    Connection::new(Endpoint::CnHangzhou, secret)
}

#[tokio::test]
#[ignore] // Run with: cargo test --ignored
async fn test_describe_regions() {
    let conn = test_connection();
    let result = conn
        .describe_regions(ali_acs::fc::DescribeRegions::new())
        .await
        .unwrap();

    println!(
        "Describe regions response: {} region(s) found",
        result.regions.region.len()
    );
    for region in &result.regions.region {
        println!("  - {}: {}", region.region_id, region.local_name);
    }
}

#[tokio::test]
#[ignore] // Run with: cargo test --ignored
async fn test_create_invoke_delete_function() {
    let conn = test_connection();

    // Generate a random 4-digit suffix for the function name
    let suffix: u16 = rand::thread_rng().gen_range(1000..10000);
    let function_name = format!("ali-acs-test-{}", suffix);
    println!("Creating function: {}", function_name);

    // Read the code.zip file and base64-encode it
    let code_bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/src/tests/code.zip"))
        .expect("Failed to read src/tests/code.zip");
    use base64::Engine;
    let zip_file_base64 = base64::engine::general_purpose::STANDARD.encode(&code_bytes);

    // Build the CreateFunctionInput
    let input = ali_acs::fc::CreateFunctionInput {
        function_name: function_name.clone(),
        runtime: "custom.debian10".to_string(),
        handler: "index.handler".to_string(),
        cpu: Some(0.05),
        memory_size: Some(128),
        disk_size: Some(512),
        code: Some(ali_acs::fc::InputCodeLocation {
            zip_file: Some(zip_file_base64),
            ..Default::default()
        }),
        custom_runtime_config: Some(ali_acs::fc::CustomRuntimeConfig {
            command: vec!["/code/hello-fc".to_string()],
            port: Some(3000),
            ..Default::default()
        }),
        timeout: Some(60),
        ..Default::default()
    };

    // Create the function
    let create_result = conn
        .create_function(ali_acs::fc::CreateFunction::new(input))
        .await
        .unwrap();
    println!(
        "Function created: {:?}, state: {:?}",
        create_result.function_name, create_result.state
    );

    // Create an HTTP trigger with anonymous auth to get a public URL
    let trigger_config = serde_json::json!({
        "authType": "anonymous",
        "methods": ["GET", "POST"]
    });
    let trigger_input = ali_acs::fc::CreateTriggerInput {
        trigger_name: "http-trigger".to_string(),
        trigger_type: "http".to_string(),
        trigger_config: serde_json::to_string(&trigger_config).unwrap(),
        ..Default::default()
    };
    let trigger_result = conn
        .create_trigger(ali_acs::fc::CreateTrigger::new(
            &function_name,
            trigger_input,
        ))
        .await
        .unwrap();

    let url = trigger_result
        .http_trigger
        .as_ref()
        .and_then(|t| t.url_internet.as_ref())
        .expect("HTTP trigger should have an internet URL");
    println!("HTTP trigger URL: {}", url);

    // Wait for the function to become active before sending a request
    let mut ready = false;
    for i in 0..30 {
        let func = conn
            .get_function(ali_acs::fc::GetFunction::new(&function_name))
            .await
            .unwrap();
        let state = func.state.as_deref().unwrap_or("Unknown");
        println!("  Attempt {}: function state = {}", i + 1, state);
        if state == "Active" {
            ready = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
    assert!(ready, "Function did not become Active in time");

    // Send a test HTTP request to the function's public URL
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to send HTTP request to function");
    let status = response.status();
    let body = response.text().await.expect("Failed to read response body");
    println!("HTTP response status: {}, body: {}", status, body);
    assert_eq!(body, "Hello, World!");

    // Clean up: delete the function (this also removes its triggers)
    conn.delete_function(ali_acs::fc::DeleteFunction::new(&function_name))
        .await
        .unwrap();
    println!("Function {} deleted successfully", function_name);
}
