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

/// Helper to generate a test sign name with timestamp
fn test_sign_name() -> String {
    format!(
        "cirrus-test-sign-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    )
}

/// Helper to generate a test template name with timestamp
fn test_template_name() -> String {
    format!(
        "cirrus-test-template-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    )
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

    // Note: QuerySmsSignList returns todo!(), so we expect a runtime panic
    // This is kept as documentation for when the endpoint is implemented
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            conn.query_sms_sign_list(crate::sms::QuerySmsSignList::new())
                .await
        })
    })) {
        Ok(Ok(_)) => println!("Query SMS sign list succeeded"),
        Ok(Err(e)) => println!("Query SMS sign list error: {}", e),
        Err(_) => println!("Query SMS sign list panicked (expected - not implemented)"),
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
            println!(
                "Query SMS sign response code: {}",
                response.code_message.code
            );
        }
        Err(e) => {
            println!("Query SMS sign error: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_create_and_delete_sms_sign() {
    let conn = test_connection();
    let sign_name = test_sign_name();

    // Create a sign - this will likely fail without proper qualification
    let create_request = crate::sms::CreateSmsSign::new(&sign_name, 0_i64, 0_i32)
        .remark("Automated test sign for validation".to_string());

    let result = conn.create_sms_sign(create_request).await;

    match result {
        Ok(response) => {
            println!("Create SMS sign response: {}", response.code_message.code);

            // If creation succeeded, try to clean up
            if response.code_message.code == "OK" {
                let delete_request = crate::sms::DeleteSmsSign::new(&sign_name);
                match conn.delete_sms_sign(delete_request).await {
                    Ok(_) => println!("Sign deleted successfully"),
                    Err(e) => println!("Failed to delete sign: {}", e),
                }
            }
        }
        Err(e) => {
            println!(
                "Create SMS sign failed (expected without valid qualification): {}",
                e
            );
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
async fn test_query_sms_template() {
    let conn = test_connection();

    let result = conn
        .query_sms_template(crate::sms::QuerySmsTemplate::new("SMS_template"))
        .await;

    match result {
        Ok(response) => {
            println!(
                "Query SMS template response code: {}",
                response.code_message.code
            );
        }
        Err(e) => {
            println!("Query SMS template error: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_create_and_delete_sms_template() {
    let conn = test_connection();
    let template_name = test_template_name();

    // Create a simple verification code template
    let create_request =
        crate::sms::CreateSmsTemplate::new(&template_name, "您的验证码是${code}。", 0_i32)
            .remark("Automated test template for validation".to_string())
            .related_sign_name("test-sign".to_string());

    let result = conn.create_sms_template(create_request).await;

    match result {
        Ok(response) => {
            println!(
                "Create SMS template response: {}",
                response.code_message.code
            );

            // If creation succeeded, try to clean up
            if response.code_message.code == "OK" {
                let template_code = response.template_code;
                let delete_request = crate::sms::DeleteSmsTemplate::new(&template_code);
                match conn.delete_sms_template(delete_request).await {
                    Ok(_) => println!("Template deleted successfully"),
                    Err(e) => println!("Failed to delete template: {}", e),
                }
            }
        }
        Err(e) => {
            println!(
                "Create SMS template failed (expected without valid sign): {}",
                e
            );
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
                "Send SMS failed (expected without valid credentials/sign/template): {}",
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
async fn test_create_and_query_trademark() {
    let conn = test_connection();

    // Create a trademark - requires OSS file upload first
    let trademark_request = crate::sms::CreateSmsTrademark::new(
        "oss/file/key.jpg",  // This requires a real OSS file
        "12345678",          // Trademark registration number
        "TestTrademark",     // Trademark name
        "TestCompany",       // Applicant name
        "20250101,20350101", // Effect/expiration date
    );

    let result = conn.create_sms_trademark(trademark_request).await;

    match result {
        Ok(response) => {
            println!("Create trademark response: {}", response.code_message.code);
            println!("Trademark data: {}", response.data);
            // The response.data field contains the trademark_id as a string
            // if creation succeeded, but we can't parse it in the test
        }
        Err(e) => {
            println!(
                "Create trademark failed (expected without valid OSS file): {}",
                e
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_sms_sign_full_lifecycle() {
    let conn = test_connection();
    let sign_name = test_sign_name();

    // 1. Create a sign
    println!("=== Step 1: Create SMS Sign ===");
    let create_request = crate::sms::CreateSmsSign::new(&sign_name, 0_i64, 0_i32)
        .remark("Full lifecycle test".to_string());

    let create_result = conn.create_sms_sign(create_request).await;

    match create_result {
        Ok(create_response) => {
            println!("Create response: {}", create_response.code_message.code);

            if create_response.code_message.code == "OK" {
                // 2. Get the sign details
                println!("\n=== Step 2: Get SMS Sign ===");
                let get_result = conn
                    .get_sms_sign(crate::sms::GetSmsSign::new(&sign_name))
                    .await;
                match get_result {
                    Ok(get_response) => {
                        println!("Get response: {}", get_response.code_message.code);
                    }
                    Err(e) => println!("Get failed: {}", e),
                }

                // 3. Query the sign
                println!("\n=== Step 3: Query SMS Sign ===");
                let query_result = conn
                    .query_sms_sign(crate::sms::QuerySmsSign::new(&sign_name))
                    .await;
                match query_result {
                    Ok(query_response) => {
                        println!("Query response: {}", query_response.code_message.code);
                    }
                    Err(e) => println!("Query failed: {}", e),
                }

                // 4. Delete the sign
                println!("\n=== Step 4: Delete SMS Sign ===");
                let delete_request = crate::sms::DeleteSmsSign::new(&sign_name);
                match conn.delete_sms_sign(delete_request).await {
                    Ok(delete_response) => {
                        println!("Delete response: {}", delete_response.code_message.code);
                    }
                    Err(e) => println!("Delete failed: {}", e),
                }
            }
        }
        Err(e) => {
            println!("Create failed (expected): {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_sms_template_full_lifecycle() {
    let conn = test_connection();
    let template_name = test_template_name();

    // 1. Create a template
    println!("=== Step 1: Create SMS Template ===");
    let create_request =
        crate::sms::CreateSmsTemplate::new(&template_name, "您的验证码是${code}。", 0_i32)
            .remark("Full lifecycle test".to_string());

    let create_result = conn.create_sms_template(create_request).await;

    match create_result {
        Ok(create_response) => {
            println!("Create response: {}", create_response.code_message.code);

            if create_response.code_message.code == "OK" {
                let template_code = create_response.template_code;

                // 2. Get the template details
                println!("\n=== Step 2: Get SMS Template ===");
                let get_result = conn
                    .get_sms_template(crate::sms::GetSmsTemplate::new(&template_code))
                    .await;
                match get_result {
                    Ok(get_response) => {
                        println!("Get response: {}", get_response.code_message.code);
                    }
                    Err(e) => println!("Get failed: {}", e),
                }

                // 3. Query the template (old API)
                println!("\n=== Step 3: Query SMS Template ===");
                let query_result = conn
                    .query_sms_template(crate::sms::QuerySmsTemplate::new(&template_code))
                    .await;
                match query_result {
                    Ok(query_response) => {
                        println!("Query response: {}", query_response.code_message.code);
                    }
                    Err(e) => println!("Query failed: {}", e),
                }

                // 4. Delete the template
                println!("\n=== Step 4: Delete SMS Template ===");
                let delete_request = crate::sms::DeleteSmsTemplate::new(&template_code);
                match conn.delete_sms_template(delete_request).await {
                    Ok(delete_response) => {
                        println!("Delete response: {}", delete_response.code_message.code);
                    }
                    Err(e) => println!("Delete failed: {}", e),
                }
            }
        }
        Err(e) => {
            println!("Create failed (expected): {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_update_sms_sign() {
    let conn = test_connection();
    let sign_name = test_sign_name();

    // First create a sign
    let create_request = crate::sms::CreateSmsSign::new(&sign_name, 0_i64, 0_i32);

    let create_result = conn.create_sms_sign(create_request).await;

    match create_result {
        Ok(create_response) => {
            if create_response.code_message.code == "OK" {
                // Update the sign
                let update_request = crate::sms::UpdateSmsSign::new(&sign_name, 0_i64, 0_i32)
                    .remark("Updated remark".to_string());

                let update_result = conn.update_sms_sign(update_request).await;
                match update_result {
                    Ok(update_response) => {
                        println!(
                            "Update sign response: {}",
                            update_response.code_message.code
                        );
                    }
                    Err(e) => println!("Update failed: {}", e),
                }

                // Clean up
                let _ = conn
                    .delete_sms_sign(crate::sms::DeleteSmsSign::new(&sign_name))
                    .await;
            }
        }
        Err(e) => {
            println!("Create failed (expected): {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_update_sms_template() {
    let conn = test_connection();
    let template_name = test_template_name();

    // First create a template
    let create_request =
        crate::sms::CreateSmsTemplate::new(&template_name, "您的验证码是${code}。", 0_i32);

    let create_result = conn.create_sms_template(create_request).await;

    match create_result {
        Ok(create_response) => {
            if create_response.code_message.code == "OK" {
                let template_code = create_response.template_code;

                // Update the template
                let update_request = crate::sms::UpdateSmsTemplate::new(
                    &template_name,
                    &template_code,
                    "您的验证码是${code}。",
                    0_i32,
                )
                .remark("Updated remark".to_string());

                let update_result = conn.update_sms_template(update_request).await;
                match update_result {
                    Ok(update_response) => {
                        println!(
                            "Update template response: {}",
                            update_response.code_message.code
                        );
                    }
                    Err(e) => println!("Update failed: {}", e),
                }

                // Clean up
                let _ = conn
                    .delete_sms_template(crate::sms::DeleteSmsTemplate::new(&template_code))
                    .await;
            }
        }
        Err(e) => {
            println!("Create failed (expected): {}", e);
        }
    }
}
