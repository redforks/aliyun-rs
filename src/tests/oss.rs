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
    let secret = AccessKeySecret::new(
        access_key,
        std::env::var("TEST_ALI_SECRET")
            .expect("TEST_ALI_SECRET environment variable not set")
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
    let buckets = &result.buckets.bucket;
    assert!(!buckets.is_empty());
    println!("Found {} bucket(s)", buckets.len());
    for bucket in buckets {
        println!(
            "  - {} (region: {}, created: {})",
            bucket.name, bucket.region, bucket.creation_date
        );
    }
}

#[tokio::test]
#[test_log::test]
#[ignore]
async fn test_describe_regions() {
    let conn = test_connection();

    let result = conn
        .describe_regions(crate::oss::DescribeRegions::new().regions("oss-cn-hangzhou".to_owned()))
        .await
        .unwrap();

    println!("Describe regions response:");
    let regions = &result.region_info;
    assert!(!regions.is_empty());
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

/// Comprehensive test for bucket and object lifecycle operations
#[tokio::test]
#[test_log::test]
#[ignore]
async fn test_bucket_and_object_lifecycle() {
    use crate::oss::{DeleteBucket, DeleteObject, ListObjects, PutBucket, PutObject};

    let conn = test_connection();

    // Generate a random bucket name with "ali-acs-test-" prefix and 4 random digits
    let random_suffix: u16 = rand::random::<u16>() % 10000;
    let bucket_name = format!("ali-acs-test-{:04}", random_suffix);

    println!("Using bucket name: {}", bucket_name);

    // 1. Create a new bucket
    println!("Step 1: Creating bucket...");
    conn.put_bucket(PutBucket::new(&bucket_name))
        .await
        .expect("Failed to create bucket");
    println!("Bucket created successfully");

    // 2. List buckets and ensure the bucket is present
    println!("Step 2: Listing buckets to verify creation...");
    let list_result = conn.list_buckets(crate::oss::ListBuckets::new()).await.unwrap();
    let bucket_found = list_result
        .buckets
        .bucket
        .iter()
        .any(|b| b.name == bucket_name);
    assert!(bucket_found, "Created bucket not found in bucket list");
    println!("Bucket found in list");

    // 3. Put a file to the bucket with content "hello world"
    let object_key = "test-file.txt";
    let object_content = b"hello world";
    println!("Step 3: Putting object '{}' to bucket...", object_key);
    conn.put_object(
        PutObject::new(&bucket_name, object_key).body(object_content.to_vec()),
    )
    .await
    .expect("Failed to put object");
    println!("Object uploaded successfully");

    // 4. List files in the bucket and ensure that file is present
    println!("Step 4: Listing objects in bucket...");
    let objects_result = conn
        .list_objects(ListObjects::new(&bucket_name))
        .await
        .expect("Failed to list objects");
    let file_found = objects_result
        .contents
        .iter()
        .any(|o| o.key == object_key);
    assert!(file_found, "Uploaded file not found in object list");
    println!("File found in bucket");

    // 5. Remove the file from the bucket
    println!("Step 5: Deleting object from bucket...");
    conn.delete_object(DeleteObject::new(&bucket_name, object_key))
        .await
        .expect("Failed to delete object");
    println!("Object deleted successfully");

    // Verify file is gone
    println!("Verifying file is deleted...");
    let objects_result_after_delete = conn
        .list_objects(ListObjects::new(&bucket_name))
        .await
        .expect("Failed to list objects after deletion");
    let file_still_exists = objects_result_after_delete
        .contents
        .iter()
        .any(|o| o.key == object_key);
    assert!(!file_still_exists, "File still exists after deletion");
    println!("File verified as deleted");

    // 6. Delete the bucket
    println!("Step 6: Deleting bucket...");
    conn.delete_bucket(DeleteBucket::new(&bucket_name))
        .await
        .expect("Failed to delete bucket");
    println!("Bucket deleted successfully");

    // 7. Ensure that the bucket is deleted
    println!("Step 7: Verifying bucket is deleted...");
    let list_result_after_delete = conn
        .list_buckets(crate::oss::ListBuckets::new())
        .await
        .unwrap();
    let bucket_still_exists = list_result_after_delete
        .buckets
        .bucket
        .iter()
        .any(|b| b.name == bucket_name);
    assert!(!bucket_still_exists, "Bucket still exists after deletion");
    println!("Bucket verified as deleted");

    println!("All lifecycle tests passed!");
}
