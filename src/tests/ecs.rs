use crate::ecs::{Connection, Endpoint};

/// Creates a test connection using environment variables.
fn test_connection() -> Connection {
    let access_key = std::env::var("TEST_ALI_ACCESS_KEY")
        .expect("TEST_ALI_ACCESS_KEY environment variable not set");
    let secret = crate::v3::AccessKeySecret(
        access_key.into(),
        std::env::var("TEST_ALI_SECRET")
            .expect("TEST_ALI_SECRET environment variable not set")
            .into(),
    );
    Connection::new(Endpoint::CnHangzhou, secret)
}

/// Tests querying available regions.
///
/// This test calls the DescribeRegions API to retrieve a list of
/// all available Alibaba Cloud regions and their endpoints.
#[tokio::test]
#[ignore]
async fn test_describe_regions() {
    let conn = test_connection();

    // Create request with no parameters (all optional)
    let req = crate::ecs::DescribeRegions::new();

    match conn.describe_regions(req).await {
        Ok(response) => {
            println!("DescribeRegions succeeded:");
            println!("  Request ID: {}", response.request_id);
            println!("  Number of regions: {}", response.regions.region.len());

            for region in &response.regions.region {
                println!(
                    "  - {} ({}): {} - Status: {}",
                    region.local_name, region.region_id, region.region_endpoint, region.status
                );
            }
        }
        Err(e) => {
            println!("DescribeRegions failed: {:#?}", e);
        }
    }
}

/// Tests querying regions with English language.
#[tokio::test]
#[ignore]
async fn test_describe_regions_english() {
    let conn = test_connection();

    let req = crate::ecs::DescribeRegions::new()
        .accept_language("en-US".to_string())
        .resource_type("instance".to_string());

    match conn.describe_regions(req).await {
        Ok(response) => {
            println!("DescribeRegions (English) succeeded:");
            println!("  Number of regions: {}", response.regions.region.len());
            println!(
                "  First few regions: {:?}",
                response
                    .regions
                    .region
                    .iter()
                    .take(3)
                    .map(|r| &r.region_id)
                    .collect::<Vec<_>>()
            );
        }
        Err(e) => {
            println!("DescribeRegions (English) failed: {:#?}", e);
        }
    }
}

/// Tests querying regions with Chinese language.
#[tokio::test]
#[ignore]
async fn test_describe_regions_chinese() {
    let conn = test_connection();

    let req = crate::ecs::DescribeRegions::new()
        .accept_language("zh-CN".to_string())
        .resource_type("instance".to_string());

    match conn.describe_regions(req).await {
        Ok(response) => {
            println!("DescribeRegions (Chinese) succeeded:");
            println!("  Number of regions: {}", response.regions.region.len());
            for region in &response.regions.region[..] {
                println!("  - {}: {}", region.region_id, region.local_name);
            }
        }
        Err(e) => {
            println!("DescribeRegions (Chinese) failed: {:#?}", e);
        }
    }
}
