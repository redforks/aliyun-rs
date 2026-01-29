//! Unit tests for aliyun-rs library traits and types.

use crate::IntoResponse;
use crate::ToCodeMessage;

#[test]
fn test_vec_u8_into_response() {
    let data = vec![1, 2, 3, 4, 5];
    let result: Vec<u8> = data.into_response();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_vec_u8_to_code_message() {
    let data = vec![1, 2, 3];
    let code_message = data.to_code_message();
    assert!(code_message.code.is_empty());
    assert!(code_message.message.is_empty());
}

#[cfg(feature = "sms")]
mod sms;

#[cfg(feature = "ecs")]
mod ecs;
