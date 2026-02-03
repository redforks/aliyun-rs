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

/// Tests querying regions with English language.
#[tokio::test]
#[ignore]
async fn test_describe_regions_english() {
    let conn = test_connection();

    let req = crate::ecs::DescribeRegions::new()
        .accept_language("en-US".to_string())
        .resource_type("instance".to_string());

    let response = conn.describe_regions(req).await.unwrap();
    println!("DescribeRegions (English) succeeded:");
    println!("  Number of regions: {}", response.regions.region.len());
    println!(
        "{:?}",
        response
            .regions
            .region
            .iter()
            .map(|r| &r.region_id)
            .collect::<Vec<_>>()
    );
}
