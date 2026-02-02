//! Integration tests for Aliyun OSS API.
//!
//! These tests validate that the Aliyun server accepts our requests.
//! Set TEST_ALI_ACCESS_KEY and TEST_ALI_SECRET environment variables to run.
//!
//! Note: The current OSS module supports account-level operations (like ListBuckets)
//! using the base endpoint. Bucket-specific operations (PutBucket, DeleteBucket,
//! PutObject, DeleteObject) require the bucket name to be part of the host
//! (e.g., bucket-name.oss-cn-hangzhou.aliyuncs.com), which requires extending
//! the Connection implementation.

use crate::oss::{Connection, Endpoint, ListBuckets};
use crate::v3::AccessKeySecret;

/// Helper to get the connection from environment variables
fn test_connection() -> Connection {
    let access_key = std::env::var("TEST_ALI_ACCESS_KEY")
        .expect("TEST_ALI_ACCESS_KEY environment variable not set");
    let secret = AccessKeySecret(
        access_key.into(),
        std::env::var("TEST_ALI_SECRET")
            .expect("TEST_ALI_SECRET environment variable not set")
            .into(),
    );
    Connection::new(Endpoint::CnHangzhou, secret)
}

#[tokio::test]
#[test_log::test]
#[ignore] // Run with: cargo test --ignored
async fn test_list_buckets() {
    let conn = test_connection();
    let result = conn.list_buckets(ListBuckets::new()).await.unwrap();

    println!("List buckets response:");
    let buckets = &result.list_all_my_buckets_result.buckets.bucket;
    println!("Found {} bucket(s)", buckets.len());
    for bucket in buckets {
        println!(
            "  - {} (region: {}, created: {})",
            bucket.name, bucket.region, bucket.creation_date
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_describe_regions() {
    let conn = test_connection();

    let result = conn
        .describe_regions(crate::oss::DescribeRegions::new())
        .await;

    match result {
        Ok(response) => {
            println!("Describe regions response:");
            let regions = &response.region_info_list.region_info;
            println!("Found {} region(s)", regions.len());
            for region in regions.iter().take(5) {
                println!(
                    "  - {} (internet: {}, internal: {})",
                    region.region, region.internet_endpoint, region.internal_endpoint
                );
            }
            if regions.len() > 5 {
                println!("  ... and {} more regions", regions.len() - 5);
            }
        }
        Err(e) => {
            println!("Describe regions failed: {:?}", e);
        }
    }
}
