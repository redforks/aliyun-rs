//! Integration tests for Aliyun Function Compute API.
//!
//! These tests validate that the Aliyun server accepts our requests.
//! Set TEST_ALI_ACCESS_KEY and TEST_ALI_SECRET environment variables to run.

use crate::fc::{Connection, Endpoint};
use crate::v3::AccessKeySecret;

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
        .describe_regions(crate::fc::DescribeRegions::new())
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
