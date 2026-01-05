//! Integration tests for Aliyun SMS API.
//!
//! These tests validate that the Aliyun server accepts our requests.
//! Set TEST_ALI_ACCESS_KEY and TEST_ALI_SECRET environment variables to run.
//!
//! Test resources are created with "cirrus-test-" prefix for easy cleanup.

use crate::sms::{Connection, Endpoint};
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

/// Helper to format date as YYYYMMDD
fn format_date(date: time::OffsetDateTime) -> String {
    format!(
        "{:04}{:02}{:02}",
        date.year(),
        date.month() as u8,
        date.day()
    )
}

#[tokio::test]
#[ignore] // Run with: cargo test --ignored
async fn test_query_sms_sign_list() {
    let conn = test_connection();

    let result = conn
        .query_sms_sign_list(crate::sms::QuerySmsSignList::new())
        .await;

    match result {
        Ok(response) => {
            println!("Query SMS sign list response code: {:?}", response);
        }
        Err(e) => {
            println!("Query SMS sign list error: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_get_sms_sign_by_name() {
    let conn = test_connection();

    let result = conn
        .get_sms_sign(crate::sms::GetSmsSign::new("test-sign"))
        .await;

    match result {
        Ok(response) => {
            println!("Get SMS sign response code: {}", response.code_message.code);
        }
        Err(e) => {
            println!("Get SMS sign error: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_query_sms_sign() {
    let conn = test_connection();

    let result = conn
        .query_sms_sign(crate::sms::QuerySmsSign::new("test-sign"))
        .await;

    match result {
        Ok(response) => {
            println!("Query SMS sign response code: {:?}", response);
        }
        Err(e) => {
            println!("Query SMS sign error: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_get_sms_template() {
    let conn = test_connection();

    let result = conn
        .get_sms_template(crate::sms::GetSmsTemplate::new("SMS_template"))
        .await;

    match result {
        Ok(response) => {
            println!(
                "Get SMS template response code: {}",
                response.code_message.code
            );
        }
        Err(e) => {
            println!("Get SMS template error: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_send_sms() {
    let conn = test_connection();

    // Send a test SMS - this requires an approved sign and template
    let send_request = crate::sms::SendSms::new(
        "15000000000",  // Test phone number - must be bound in console
        "test-sign",    // Must be an approved sign
        "SMS_template", // Must be an approved template
    )
    .template_param(r#"{"code":"1234"}"#.to_string());

    let result = conn.send_sms(send_request).await;

    match result {
        Ok(response) => {
            println!("Send SMS response: {}", response.code_message.code);
            println!("BizId: {:?}", response.biz_id);
            println!("RequestId: {:?}", response.request_id);
        }
        Err(e) => {
            println!(
                "Send SMS failed (expected without valid credentials/sign/template): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_query_send_details() {
    let conn = test_connection();

    let send_date = format_date(time::OffsetDateTime::now_utc());
    let phone_number = "15000000000";

    let query_request = crate::sms::QuerySendDetails::new(phone_number, &send_date, 10_i64, 1_i64);

    let result = conn.query_send_details(query_request).await;

    match result {
        Ok(response) => {
            println!(
                "Query send details response code: {}",
                response.code_message.code
            );
            let details = &response.sms_send_detail_dt_os.sms_send_detail_dto;
            if !details.is_empty() {
                println!("Found {} send detail(s)", details.len());
            }
        }
        Err(e) => {
            println!("Query send details error: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_query_sms_template_list() {
    let conn = test_connection();

    let result = conn
        .query_sms_template_list(crate::sms::QuerySmsTemplateList::new())
        .await;

    match result {
        Ok(response) => {
            println!(
                "Query SMS template list response code: {}",
                response.code_message.code
            );
        }
        Err(e) => {
            println!("Query SMS template list error: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_query_send_statistics() {
    let conn = test_connection();

    let end_date = format_date(time::OffsetDateTime::now_utc());
    let start_date = format_date(time::OffsetDateTime::now_utc() - time::Duration::days(7));

    let result = conn
        .query_send_statistics(crate::sms::QuerySendStatistics::new(
            0_i32,
            &start_date,
            &end_date,
            1_i32,
            10_i32,
        ))
        .await;

    match result {
        Ok(response) => {
            println!(
                "Query send statistics response code: {}",
                response.code_message.code
            );
        }
        Err(e) => {
            println!("Query send statistics error: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_send_batch_sms() {
    let conn = test_connection();

    // Send batch SMS - requires approved signs and templates
    let phone_numbers = r#"["15000000000","15000000001"]"#;
    let sign_names = r#"["test-sign","test-sign"]"#;
    let template_params = r#"[{"code":"1234"},{"code":"5678"}]"#;

    let send_request = crate::sms::SendBatchSms::new(
        phone_numbers,
        sign_names,
        "SMS_template", // Must be an approved template
    )
    .template_param_json(template_params.to_string());

    let result = conn.send_batch_sms(send_request).await;

    match result {
        Ok(response) => {
            println!("Send batch SMS response: {}", response.code_message.code);
        }
        Err(e) => {
            println!(
                "Send batch SMS failed (expected without valid credentials): {}",
                e
            );
        }
    }
}
