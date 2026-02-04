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

use maplit::hashmap;

use crate::oss::{Connection, Endpoint, ListBuckets};
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
    use crate::oss::{DeleteBucket, DeleteObject, GetObject, ListObjects, PutBucket, PutObject};

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
    let list_result = conn
        .list_buckets(crate::oss::ListBuckets::new())
        .await
        .unwrap();
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
        PutObject::new(&bucket_name, object_key)
            .body(object_content.to_vec())
            .x_oss_meta(
                hashmap! {"p1".to_owned() => "v1".to_owned(), "p2".to_owned() => "v2".to_owned()},
            ),
    )
    .await
    .expect("Failed to put object");
    println!("Object uploaded successfully");

    // 4. Get the file from the bucket and verify content
    println!("Step 4: Getting object '{}' from bucket...", object_key);
    let retrieved_content = conn
        .get_object(GetObject::new(&bucket_name, object_key))
        .await
        .expect("Failed to get object");
    assert_eq!(
        &retrieved_content.body, object_content,
        "Retrieved content does not match original content"
    );
    assert_eq!(
        retrieved_content.x_oss_meta,
        hashmap! {"p1".to_owned() => "v1".to_owned(), "p2".to_owned() => "v2".to_owned()}
    );

    // 5. List files in the bucket and ensure that file is present
    println!("Step 5: Listing objects in bucket...");
    let objects_result = conn
        .list_objects(ListObjects::new(&bucket_name))
        .await
        .expect("Failed to list objects");
    let file_found = objects_result.contents.iter().any(|o| o.key == object_key);
    assert!(file_found, "Uploaded file not found in object list");
    println!("File found in bucket");

    // 6. Remove the file from the bucket
    println!("Step 6: Deleting object from bucket...");
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

    // 7. Delete the bucket
    println!("Step 7: Deleting bucket...");
    conn.delete_bucket(DeleteBucket::new(&bucket_name))
        .await
        .expect("Failed to delete bucket");
    println!("Bucket deleted successfully");

    // 8. Ensure that the bucket is deleted
    println!("Step 8: Verifying bucket is deleted...");
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

/// Cleanup test to remove all test buckets and their contents
#[tokio::test]
#[test_log::test]
#[ignore]
async fn clean_test_buckets() {
    use crate::oss::{DeleteBucket, DeleteObject, ListObjects};

    let conn = test_connection();

    // 1. List all buckets
    println!("Listing all buckets...");
    let list_result = conn
        .list_buckets(crate::oss::ListBuckets::new())
        .await
        .unwrap();

    // 2. Filter buckets that start with "ali-acs-test-"
    let test_buckets: Vec<_> = list_result
        .buckets
        .bucket
        .iter()
        .filter(|b| b.name.starts_with("ali-acs-test-"))
        .map(|b| b.name.clone())
        .collect();

    if test_buckets.is_empty() {
        println!("No test buckets found to clean up.");
        return;
    }

    println!("Found {} test bucket(s) to clean up:", test_buckets.len());
    for bucket in &test_buckets {
        println!("  - {}", bucket);
    }

    // 3. For each test bucket, delete all objects, then delete the bucket
    for bucket_name in test_buckets {
        println!("\nProcessing bucket: {}", bucket_name);

        // List all objects in the bucket
        println!("  Listing objects...");
        let objects_result = conn
            .list_objects(ListObjects::new(&bucket_name))
            .await
            .unwrap();

        if !objects_result.contents.is_empty() {
            println!(
                "  Found {} object(s) to delete:",
                objects_result.contents.len()
            );

            // Delete all objects
            for object in &objects_result.contents {
                println!("    Deleting object: {}", object.key);
                conn.delete_object(DeleteObject::new(&bucket_name, &object.key))
                    .await
                    .expect("Failed to delete object");
            }
            println!("  All objects deleted.");
        } else {
            println!("  No objects found in bucket.");
        }

        // Delete the bucket
        println!("  Deleting bucket...");
        conn.delete_bucket(DeleteBucket::new(&bucket_name))
            .await
            .expect("Failed to delete bucket");
        println!("  Bucket deleted successfully.");
    }

    println!("\nCleanup complete!");
}
