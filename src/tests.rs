//! Unit tests for aliyun-rs library traits and types.

#[cfg(feature = "sms")]
mod sms;

#[cfg(feature = "ecs")]
mod ecs;

#[cfg(feature = "oss")]
mod oss;

#[cfg(feature = "fc")]
mod fc;
