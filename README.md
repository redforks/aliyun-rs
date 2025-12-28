# aliyun-rs

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个类型安全、符合 Rust 惯用法的阿里云服务 SDK，提供完整的协议覆盖。

## 特性

- **完整的协议支持** - 实现了阿里云 Signature V3 认证协议，支持完整的请求签名、头部处理和响应解析
- **类型安全** - 利用 Rust 的类型系统确保 API 调用的正确性，编译时即可发现错误
- **符合 Rust 惯用法** - 采用构造器模式和链式方法调用，提供优雅的 API 设计
- **异步支持** - 基于 `reqwest` 和 `async/await`，高性能异步 I/O
- **灵活的参数样式** - 支持 Flat、RepeatList 等多种参数序列化方式
- **动态类型支持** - 提供 `Value` 和 `OpenObject` 类型，应对 API 变化的灵活性

## 已实现的服务

- **SMS（短信服务）** - 完整实现，包含 56 个 API 端点：
  - 短信发送：发送短信、批量发送短信、查询发送详情
  - 签名管理：创建、查询、修改、删除签名，签名列表查询
  - 模板管理：创建、查询、修改、删除模板，模板列表查询
  - 资质申请：提交短信资质、查询资质审核记录
  - 短链接：添加、查询、删除短链接
  - 卡片短信：完整的卡片短信 API 支持
  - 统计报表：获取短信发送统计

## 快速开始

### 添加依赖

```toml
[dependencies]
aliyun-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

### 基本使用

```rust
use aliyun_rs::sms::{Connection, Endpoint};
use aliyun_rs::v3::AccessKeySecret;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建访问凭证
    let access_key_id = "your_access_key_id";
    let access_key_secret = "your_access_key_secret";
    let app_key_secret = AccessKeySecret::new(access_key_id, access_key_secret);

    // 2. 创建 SMS 连接
    let conn = Connection::new(Endpoint::CnHangzhou, app_key_secret);

    // 3. 构建请求（必要参数通过 ::new() 构造）
    let req = aliyun_rs::sms::SendSms::new(
        "13800138000",           // 手机号码
        "阿里云短信签名",         // 签名名称
        "SMS_123456789"           // 模板CODE
    )
    // 可选参数使用链式方法设置
    .template_param(r#"{"code":"1234"}"#.to_string())
    .out_id("your-out-id".to_string());

    // 4. 发送请求
    let response = conn.send_sms(req).await?;

    println!("短信发送成功，BizId: {}", response.biz_id());

    Ok(())
}
```

### 更多示例

```rust
use aliyun_rs::sms::*;
use aliyun_rs::v3::AccessKeySecret;

// 创建签名
let create_sign_req = CreateSmsSign::new("我的签名", SignSource::SelfBuilt)
    .sign_source_list("["网站", "APP"]".to_string())
    .remark("备注信息".to_string());

let sign_response = conn.create_sms_sign(create_sign_req).await?;

// 查询签名详情
let get_sign_req = GetSmsSign::new("签名名称");
let sign_detail = conn.get_sms_sign(get_sign_req).await?;

// 批量发送短信
let batch_req = SendBatchSms::new(
    "签名名称",
    "SMS_123456789",
    "[\"13800138000\",\"13900139000\"]"
).template_param(r#"[{\"code\":\"1234\"},{\"code\":\"5678\"}]"#.to_string());

let batch_response = conn.send_batch_sms(batch_req).await?;
```

## API 设计理念

### Request 构造约定

所有的 API 请求类型都遵循以下设计模式：

1. **必要参数通过 `::new()` 构造函数** - 确保关键参数不会遗漏
2. **可选参数使用链式方法** - 利用 `derive_setters` 宏生成流畅的 builder 模式

```rust
// 必要参数：phone_numbers, sign_name, template_code
let req = SendSms::new("13800138000", "签名", "模板CODE")
    // 可选参数：template_param
    .template_param(r#"{"code":"1234"}"#.to_string())
    // 可选参数：out_id
    .out_id("custom-id".to_string());
```

### 为什么每个服务有独立的 Connection 和 Endpoint？

不同的阿里云产品使用独立的 `Connection` 结构体和 `Endpoint` 枚举，这是出于以下设计考虑：

1. **API 版本隔离** - 不同服务的 API 版本不同（如 SMS 使用 `2017-05-25`）
2. **端点域名差异** - 每个服务有独立的域名体系（如 `dysmsapi.aliyuncs.com`）
3. **服务特定方法** - 每个 `Connection` 可以提供符合该服务语义的高层方法（如 `send_sms()`、`create_sign()`）
4. **独立的端点枚举** - 不同服务的可用区域不同，通过独立的枚举提供类型安全的区域选择

```rust
// SMS 服务
let sms_conn = sms::Connection::new(
    sms::Endpoint::CnHangzhou,  // SMS 特定的端点
    app_key_secret
);

// ECS 服务（示例）
let ecs_conn = ecs::Connection::new(
    ecs::Endpoint::CnHangzhou,  // ECS 特定的端点
    app_key_secret
);

// 编译错误：无法将 ECS 的 Request 用于 SMS Connection
// sms_conn.call(ecs_request);  // ❌ 类型错误
```

## 进阶功能

### Flat 参数样式

对于嵌套结构的请求，SDK 支持 Flat 参数样式：

```rust
// 嵌套结构会被展开为 key.subkey=value 格式
// 例如：{"Config": {"Timeout": 30}} => Config.Timeout=30
```

### RepeatList 参数样式

支持数组类型的索引参数：

```rust
// 数组会被展开为 key.1=value1&key.2=value2 格式（1-indexed）
```

### 动态类型支持

使用 `Value` 和 `OpenObject` 处理动态 API 字段：

```rust
use aliyun_rs::{Value, OpenObject};

let mut extra = OpenObject::new();
extra.insert("custom_field".to_string(), Value::String("value".to_string()));
```

## TODO

- [ ] 添加更多阿里云服务支持
  - [X] Sms（短信服务）
  - [ ] ECS（云服务器）
  - [ ] OSS（对象存储）- 重新设计以支持最新 API
  - [ ] RDS（云数据库）
  - [ ] SLB（负载均衡）
  - [ ] CDN（内容分发网络）
  - [ ] VPC（专有网络）
- [ ] 增强错误处理
- [ ] 完善 API 文档注释

## 开发

```bash
# 克隆仓库
git clone https://github.com/your-repo/aliyun-rs.git
cd aliyun-rs

# 运行测试
cargo test
```

## 许可证

MIT License
