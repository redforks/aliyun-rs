#![allow(unused_mut)]

#[derive(Clone, Copy)]
pub enum Endpoint {
    CnWuhanLr,
    CnQingdao,
    CnBeijing,
    CnZhangjiakou,
    CnHuhehaote,
    CnWulanchabu,
    CnHangzhou,
    CnShanghai,
    CnNanjing,
    CnFuzhou,
    CnShenzhen,
    CnHeyuan,
    CnGuangzhou,
    CnChengdu,
    CnHongkong,
    ApNortheast1,
    ApNortheast2,
    ApSoutheast1,
    ApSoutheast2,
    ApSoutheast3,
    ApSoutheast5,
    ApSoutheast6,
    UsEast1,
    UsWest1,
    EuWest1,
    EuCentral1,
    ApSouth1,
    MeEast1,
    CnHangzhouFinance,
    CnShanghaiFinance1,
    CnShenzhenFinance1,
    ApSoutheast7,
    CnBeijingFinance1,
}

impl From<Endpoint> for &'static str {
    fn from(ep: Endpoint) -> Self {
        match ep {
            Endpoint::CnWuhanLr => "oss-cn-wuhan-lr.aliyuncs.com",
            Endpoint::CnQingdao => "oss-cn-qingdao.aliyuncs.com",
            Endpoint::CnBeijing => "oss-cn-beijing.aliyuncs.com",
            Endpoint::CnZhangjiakou => "oss-cn-zhangjiakou.aliyuncs.com",
            Endpoint::CnHuhehaote => "oss-cn-huhehaote.aliyuncs.com",
            Endpoint::CnWulanchabu => "oss-cn-wulanchabu.aliyuncs.com",
            Endpoint::CnHangzhou => "oss-cn-hangzhou.aliyuncs.com",
            Endpoint::CnShanghai => "oss-cn-shanghai.aliyuncs.com",
            Endpoint::CnNanjing => "oss-cn-nanjing.aliyuncs.com",
            Endpoint::CnFuzhou => "oss-cn-fuzhou.aliyuncs.com",
            Endpoint::CnShenzhen => "oss-cn-shenzhen.aliyuncs.com",
            Endpoint::CnHeyuan => "oss-cn-heyuan.aliyuncs.com",
            Endpoint::CnGuangzhou => "oss-cn-guangzhou.aliyuncs.com",
            Endpoint::CnChengdu => "oss-cn-chengdu.aliyuncs.com",
            Endpoint::CnHongkong => "oss-cn-hongkong.aliyuncs.com",
            Endpoint::ApNortheast1 => "oss-ap-northeast-1.aliyuncs.com",
            Endpoint::ApNortheast2 => "oss-ap-northeast-2.aliyuncs.com",
            Endpoint::ApSoutheast1 => "oss-ap-southeast-1.aliyuncs.com",
            Endpoint::ApSoutheast2 => "oss-ap-southeast-2.aliyuncs.com",
            Endpoint::ApSoutheast3 => "oss-ap-southeast-3.aliyuncs.com",
            Endpoint::ApSoutheast5 => "oss-ap-southeast-5.aliyuncs.com",
            Endpoint::ApSoutheast6 => "oss-ap-southeast-6.aliyuncs.com",
            Endpoint::UsEast1 => "oss-us-east-1.aliyuncs.com",
            Endpoint::UsWest1 => "oss-us-west-1.aliyuncs.com",
            Endpoint::EuWest1 => "oss-eu-west-1.aliyuncs.com",
            Endpoint::EuCentral1 => "oss-eu-central-1.aliyuncs.com",
            Endpoint::ApSouth1 => "oss-ap-south-1.aliyuncs.com",
            Endpoint::MeEast1 => "oss-me-east-1.aliyuncs.com",
            Endpoint::CnHangzhouFinance => "oss-cn-hzjbp-b-console.aliyuncs.com",
            Endpoint::CnShanghaiFinance1 => "oss-cn-shanghai-finance-1-internal.aliyuncs.com",
            Endpoint::CnShenzhenFinance1 => "oss-cn-shenzhen-finance-1-internal.aliyuncs.com",
            Endpoint::ApSoutheast7 => "oss-ap-southeast-7.aliyuncs.com",
            Endpoint::CnBeijingFinance1 => "oss-cn-beijing-finance-1-internal.aliyuncs.com",
        }
    }
}

mod sealed {
    /// prevent Request type used with Connection of other mod.
    pub trait Bound {}
}

#[derive(Clone)]
pub struct Connection(crate::common::Connection);

impl Connection {
    pub fn new(endpoint: Endpoint, app_key_secret: crate::v3::AccessKeySecret) -> Self {
        Self(crate::common::Connection::new(
            app_key_secret,
            "2019-05-17",
            endpoint.into(),
        ))
    }

    pub fn with_client(
        endpoint: Endpoint,
        app_key_secret: crate::v3::AccessKeySecret,
        client: reqwest::Client,
    ) -> Self {
        Self(crate::common::Connection::with_client(
            app_key_secret,
            "2019-05-17",
            endpoint.into(),
            client,
        ))
    }

    fn call<R: crate::Request + sealed::Bound>(
        &self,
        req: R,
    ) -> impl std::future::Future<Output = crate::Result<<R as crate::Request>::Response>> + Send
    {
        self.0.call(req)
    }
    ///
    /// # 列举存储空间
    ///
    /// 列举请求者拥有的所有存储空间（Bucket）。
    ///
    /// - 您也可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
    /// - 要列举存储空间，您必须有oss:GetService (ListBuckets)权限。具体操作，请参见[为RAM用户授权自定义的权限策略](https://help.aliyun.com/document_detail/199058.htm?spm=a2c4g.11186623.0.0.48be7590hCA8LI#section-ucu-jv0-zip)。
    /// - 调用接口时，如果所有Bucket已返回，则返回参数的XML中不包含Prefix、Marker、MaxKeys、IsTruncated和NextMarker响应元素。
    ///
    /// # Path
    /// `/`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_buckets(
        &self,
        req: ListBuckets,
    ) -> impl std::future::Future<Output = crate::Result<ListBucketsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'x-oss-resource-group-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-resource-group-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 查询地域
    ///
    /// 查询所有支持地域或者指定地域对应的Endpoint信息，包括外网Endpoint、内网Endpoint和传输加速Endpoint。
    ///
    /// 只支持在二级域名（例如oss-cn-hangzhou.aliyuncs.com）上调用DescribeRegions接口。
    ///
    /// # Path
    /// `/?regions`
    ///
    /// # Extra Info
    /// <details>
    /// <summary>查询所有支持地域的描述信息</summary>
    /// 请求示例
    ///
    /// ```ignore
    /// GET /?regions HTTP/1.1
    /// Host: oss-cn-hangzhou.aliyuncs.com
    /// Date: Fri, 20 Aug 2021 06:38:30 GMT
    /// Authorization: SignatureValue
    /// ```
    /// 返回示例
    /// ```ignore
    /// HTTP/1.1 200 OK
    /// x-oss-request-id: 3a8f-2e2d-7965-3ff9-51c875b*****
    /// Date: Fri, 20 Aug 2021 06:38:30 GMT
    /// Content-Type: application/xml
    /// Content-Length: 344606
    /// Server: AliyunOSS
    /// ```
    /// ```ignore
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <RegionInfoList>
    ///   <RegionInfo>
    ///      <Region>oss-cn-hangzhou</Region>
    ///      <InternetEndpoint>oss-cn-hangzhou.aliyuncs.com</InternetEndpoint>
    ///      <InternalEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</InternalEndpoint>
    ///      <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>  
    ///   </RegionInfo>
    ///   <RegionInfo>
    ///      <Region>oss-cn-shanghai</Region>
    ///      <InternetEndpoint>oss-cn-shanghai.aliyuncs.com</InternetEndpoint>
    ///      <InternalEndpoint>oss-cn-shanghai-internal.aliyuncs.com</InternalEndpoint>
    ///      <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>  
    ///   </RegionInfo>
    /// </RegionInfoList>
    /// ```
    /// </details>
    /// <details>
    /// <summary>查询指定地域的描述信息</summary>
    /// 请求示例
    ///
    /// ```ignore
    /// GET /?regions=oss-cn-hangzhou HTTP/1.1
    /// Host: oss-cn-hangzhou.aliyuncs.com
    /// Date: Fri, 20 Aug 2021 06:40:30 GMT
    /// Authorization: SignatureValue
    /// ```
    /// 返回示例
    /// ```ignore
    /// HTTP/1.1 200 OK
    /// x-oss-request-id: 3a8f-2e2d-7965-3ff9-51c875b*****
    /// Date: Fri, 20 Aug 2021 06:40:30 GMT
    /// Content-Type: application/xml
    /// Content-Length: 3446
    /// Server: AliyunOSS
    /// ```
    /// ```ignore
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <RegionInfoList>
    ///   <RegionInfo>
    ///     <Region>oss-cn-hangzhou</Region>
    ///     <InternetEndpoint>oss-cn-hangzhou.aliyuncs.com</InternetEndpoint>
    ///     <InternalEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</InternalEndpoint>
    ///     <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>  
    ///   </RegionInfo>
    /// </RegionInfoList>
    /// ```
    /// </details>
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn describe_regions(
        &self,
        req: DescribeRegions,
    ) -> impl std::future::Future<Output = crate::Result<DescribeRegionsResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 获取存储空间存储容量与文件数量
    ///
    /// 获取指定存储空间的存储容量以及文件数量。
    ///
    /// - 调用该接口时需要拥有oss:GetBucketStat权限。
    /// - 调用该接口获取的数据并非是实时数据，延时可能超过一个小时。
    /// - 调用该接口获取到的存储信息的时间点不保证是最新的，即后一次调用该接口返回的LastModifiedTime字段值可能比前一次调用该接口返回的LastModifiedTime字段值小。
    ///
    /// # Path
    /// `/?stat`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_stat(
        &self,
        req: GetBucketStat,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketStatResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 创建存储空间
    ///
    /// 创建一个存储空间（Bucket）。
    ///
    /// - 同一阿里云账号在同一地域（Region）内最多支持创建100个Bucket。
    /// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](~~31837~~)。
    ///
    /// # Path
    /// `/`
    ///
    /// # Extra Info
    /// 此接口所对应的各语言SDK如下：
    /// - Java
    /// - Python
    /// - PHP
    /// - Go
    /// - C
    /// - .NET
    /// - Android
    /// - iOS
    /// - Node.js
    /// - Ruby
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket(
        &self,
        req: PutBucket,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-resource-group-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-bucket-tagging': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-resource-group-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-bucket-tagging': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 删除存储空间
    ///
    /// 删除存储空间（Bucket）。
    ///
    /// - 只有Bucket的拥有者才有权限删除该Bucket。
    /// - 为了防止误删除的发生，OSS不支持删除一个非空的Bucket。
    ///
    /// # Path
    /// `/`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket(
        &self,
        req: DeleteBucket,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 列举存储空间中所有文件的信息
    ///
    /// 列举存储空间（Bucket）中所有文件（Object）的信息。
    ///
    /// - GetBucket (ListObjects)接口已修订为GetBucketV2 (ListObjectsV2)。建议您在开发应用程序时使用较新的版本GetBucketV2 (ListObjectsV2)。为保证向后兼容性，OSS继续支持GetBucket (ListObjects)。有关GetBucketV2 (ListObjectsV2)的更多信息，请参见[GetBucketV2 (ListObjectsV2)](~~187544~~)。
    ///
    /// - 执行GetBucket (ListObjects)请求时不会返回Object中自定义的元信息。
    ///
    /// # Path
    /// `/`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_objects(
        &self,
        req: ListObjects,
    ) -> impl std::future::Future<Output = crate::Result<ListObjectsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 列举存储空间中所有文件的信息V2
    ///
    /// 列举存储空间（Bucket）中所有文件（Object）的信息。
    ///
    /// 执行GetBucketV2 (ListObjectsV2)请求时不会返回Object中自定义的元信息。
    ///
    /// # Path
    /// `/?list-type=2`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_objects_v2(
        &self,
        req: ListObjectsV2,
    ) -> impl std::future::Future<Output = crate::Result<ListObjectsV2Response>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 查看存储空间的相关信息
    ///
    /// 查看存储空间（Bucket）的相关信息。只有Bucket的拥有者才能查看Bucket的信息。该请求可以从任何一个OSS的Endpoint发起。
    ///
    /// # Path
    /// `/?bucketInfo`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_info(
        &self,
        req: GetBucketInfo,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketInfoResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 查看存储空间的位置信息
    ///
    /// 查看存储空间（Bucket）的位置信息。只有Bucket的拥有者才能查看Bucket的位置信息。
    ///
    /// # Path
    /// `/?location`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_location(
        &self,
        req: GetBucketLocation,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketLocationResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取接入点信息
    ///
    /// 获取用户级别或Bucket级别的接入点信息。
    ///
    /// # Path
    /// `/?accessPoint`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_access_points(
        &self,
        req: ListAccessPoints,
    ) -> impl std::future::Future<Output = crate::Result<ListAccessPointsResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 获取接入点信息
    ///
    /// 获取接入点信息。
    ///
    /// # Path
    /// `/?accessPoint`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_access_point(
        &self,
        req: GetAccessPoint,
    ) -> impl std::future::Future<Output = crate::Result<GetAccessPointResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取接入点策略配置
    ///
    /// 获取接入点策略配置。
    ///
    /// # Path
    /// `/?accessPointPolicy`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_access_point_policy(
        &self,
        req: GetAccessPointPolicy,
    ) -> impl std::future::Future<Output = crate::Result<GetAccessPointPolicyResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除接入点策略
    ///
    /// 删除接入点策略。
    ///
    /// # Path
    /// `/?accessPointPolicy`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_access_point_policy(
        &self,
        req: DeleteAccessPointPolicy,
    ) -> impl std::future::Future<Output = crate::Result<DeleteAccessPointPolicyResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 配置接入点策略
    ///
    /// 配置接入点策略。
    ///
    /// # Path
    /// `/?accessPointPolicy`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_access_point_policy(
        &self,
        req: PutAccessPointPolicy,
    ) -> impl std::future::Future<Output = crate::Result<PutAccessPointPolicyResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 删除接入点
    ///
    /// 删除接入点。
    ///
    /// # Path
    /// `/?accessPoint`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_access_point(
        &self,
        req: DeleteAccessPoint,
    ) -> impl std::future::Future<Output = crate::Result<DeleteAccessPointResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 创建接入点
    ///
    /// 创建接入点。
    ///
    /// - 单个阿里云账号支持创建1000个接入点。
    /// - 单个Bucket支持创建100个接入点。
    ///
    /// # Path
    /// `/?accessPoint`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn create_access_point(
        &self,
        req: CreateAccessPoint,
    ) -> impl std::future::Future<Output = crate::Result<CreateAccessPointResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported."##
            );
        }
    }

    ///
    /// # 新建合规保留策略
    ///
    /// 新建一条合规保留策略。
    ///
    /// 对象存储OSS支持WORM（Write Once Read Many）特性，允许以不可删除、不可篡改的方式保存和使用数据。OSS允许针对存储空间（Bucket）设置基于时间的合规保留策略，保护周期为1天到70年。
    ///
    /// - 当基于时间的合规保留策略创建24小时后未提交锁定，则该策略自动失效。当合规保留策略锁定后，您可以在Bucket中上传和读取文件（Object），但是在Object的保留时间到期之前，不允许删除Object及合规保留策略。Object的保留时间到期后，才可以删除Object。关于合规保留策略的更多信息，请参见[合规保留策略](~~90564~~)。
    ///
    /// - 同一个Bucket中，版本控制和合规保留策略无法同时配置。如果Bucket已开启版本控制功能，则无法再配置保留策略。关于版本控制功能更多信息，请参见[版本控制介绍](~~109685~~)。
    ///
    /// # Path
    /// `/?worm`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn initiate_bucket_worm(
        &self,
        req: InitiateBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<InitiateBucketWormResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 删除未锁定的合规保留策略
    ///
    /// 删除指定存储空间（Bucket）未锁定的合规保留策略。
    ///
    /// 当基于时间的合规保留策略创建后，该策略默认处于InProgress状态，且该状态的有效期为24小时。在有效期24小时内，此策略对应的Bucket数据处于保护状态。
    ///
    /// - 启动合规保留策略24小时内：若此策略未提交锁定，则Bucket所有者以及授权用户可以删除此策略；若该保留策略已提交锁定，则不允许删除此策略，且无法缩短策略保护周期，仅可以延长保护周期。
    ///
    /// - 启动合规保留策略24小时后：若超过24小时该保留策略未提交锁定，则该策略自动失效。
    ///
    /// <br>如果Bucket内有文件处于保护周期内，那么您将无法删除合规保留策略，同时也无法删除Bucket。当Bucket为空时，Bucket的所有者可以删除该Bucket，从而间接删除该Bucket的保留策略。
    ///
    /// # Path
    /// `/?worm`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn abort_bucket_worm(
        &self,
        req: AbortBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<AbortBucketWormResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 锁定合规保留策略
    ///
    /// 调用CompleteBucketWorm接口锁定合规保留策略。
    ///
    /// 当基于时间的合规保留策略创建后，此策略默认处于InProgress状态，且该状态的有效期为24小时。在有效期24小时内，此策略对应的Bucket数据处于保护状态。
    ///
    /// - 启动合规保留策略24小时内：若此策略未提交锁定，则Bucket所有者以及授权用户可以删除此策略；若此保留策略已提交锁定，则不允许删除此策略，且无法缩短策略保护周期，仅可以延长保护周期。
    ///
    /// - 启动合规保留策略24小时后：若超过24小时此保留策略未提交锁定，则此策略自动失效。
    ///
    /// # Path
    /// `/`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn complete_bucket_worm(
        &self,
        req: CompleteBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<CompleteBucketWormResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 延长存储空间中文件的保留天数
    ///
    /// 延长已锁定的合规保留策略对应Bucket中Object的保留天数。
    ///
    /// 当基于时间的合规保留策略创建后，该策略默认处于InProgress状态，且该状态的有效期为24小时。在有效期24小时内，此策略对应的Bucket数据处于保护状态。
    ///
    /// - 启动合规保留策略24小时内：若此策略未提交锁定，则Bucket所有者以及授权用户可以删除此策略；若该保留策略已提交锁定，则不允许删除此策略，且无法缩短策略保护周期，仅可以延长保护周期。
    ///
    /// - 启动合规保留策略24小时后：若超过24小时该保留策略未提交锁定，则该策略自动失效。
    ///
    /// 若Bucket内有文件处于保护周期内，那么您将无法删除合规保留策略，同时也无法删除Bucket。当Bucket为空时，Bucket的所有者可以删除该Bucket，从而间接删除该Bucket的保留策略。
    ///
    ///
    /// > 若指定用于延长Object保留天数对应的WORM ID不存在，则返回404。
    ///
    ///
    /// # Path
    /// `/?wormExtend`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn extend_bucket_worm(
        &self,
        req: ExtendBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<ExtendBucketWormResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间的合规保留策略信息
    ///
    /// 获取指定存储空间（Bucket）的合规保留策略信息。
    ///
    /// 对象存储OSS支持WORM（Write Once Read Many）特性，允许您以不可删除、不可篡改的方式保存和使用数据。OSS允许针对Bucket设置基于时间的合规保留策略，保护周期为1天到70年。<br>
    /// 当合规保留策略锁定后，您可以在Bucket中上传和读取文件（Object），但是在Object的保留时间到期之前，不允许删除Object及合规保留策略。Object的保留时间到期后，才可以删除Object。
    ///
    ///
    /// > 若指定用来获取Bucket的合规保留策略信息对应的WORM ID不存在，则返回404。
    ///
    /// # Path
    /// `/?worm`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_worm(
        &self,
        req: GetBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketWormResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 设置或修改存储空间的访问权限
    ///
    /// 设置或修改存储空间（Bucket）的访问权限（ACL）。
    ///
    /// - 请求者需要对Bucket拥有写入ACL的权限`oss:PutBucketAcl`。
    ///
    /// - PutBucketAcl为覆盖语义，即新传入的ACL将覆盖原有的ACL。
    ///
    /// - 如果指定要设置ACL的Bucket不存在，调用该接口时将新建Bucket。
    ///
    /// # Path
    /// `/?acl`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_acl(
        &self,
        req: PutBucketAcl,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketAclResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间的访问权限
    ///
    /// 获取某个存储空间（Bucket）的访问权限（ACL）。只有Bucket的拥有者才能获取Bucket的访问权限。
    ///
    /// # Path
    /// `/?acl`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_acl(
        &self,
        req: GetBucketAcl,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketAclResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 设置存储空间的生命周期规则
    ///
    /// 设置存储空间的生命周期规则
    ///
    /// - 只有Bucket的拥有者以及被授予PutBucketLifecycle权限的RAM用户才能发起配置生命周期规则的请求。
    ///
    /// - 如果Bucket此前没有设置过生命周期规则，此操作会创建一个新的生命周期规则；如果Bucket此前设置过生命周期规则，此操作会覆写先前的规则配置。
    ///
    /// - PutBucketLifecycle是覆盖语义。当您需要追加生命周期规则时，请先调用GetBucketLifecycle接口获取当前生命周期规则配置，然后追加新的生命周期规则配置，最后调用PutBucketLifecycle接口更新生命周期规则配置。
    ///
    /// - PutBucketLifecycle操作可以对Object以及Part（以分片方式上传，但最后未提交的分片）设置过期时间。
    ///
    /// # Path
    /// `/?lifecycle`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_lifecycle(
        &self,
        req: PutBucketLifecycle,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketLifecycleResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-allow-same-action-overlap': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-allow-same-action-overlap': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 查询存储空间的生命周期规则
    ///
    /// 查看存储空间（Bucket）的生命周期规则（Lifecycle）。只有Bucket的拥有者才有权限查看Bucket的生命周期规则。
    ///
    /// # Path
    /// `/?lifecycle`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_lifecycle(
        &self,
        req: GetBucketLifecycle,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketLifecycleResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除存储空间的生命周期规则
    ///
    /// 调用DeleteBucketLifecycle接口删除指定存储空间（Bucket）的生命周期规则。
    ///
    /// - 只有Bucket的拥有者才能删除该Bucket的生命周期规则。
    ///
    /// - 调用DeleteBucketLifecycle接口删除指定Bucket所有的生命周期规则后，该Bucket中的文件（Object）不会被自动删除。
    ///
    /// # Path
    /// `/?lifecycle`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_lifecycle(
        &self,
        req: DeleteBucketLifecycle,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketLifecycleResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 为存储空间配置传输加速
    ///
    /// 调用PutBucketTransferAcceleration接口为存储空间（Bucket）配置传输加速。开启传输加速后，可提升全球各地用户对OSS的访问速度，适用于远距离数据传输、GB或TB级大文件上传和下载的场景。
    ///
    /// - 只有Bucket拥有者以及被授予oss:PutBucketTransferAcceleration权限的RAM用户才能发起配置传输加速的请求。
    ///
    /// - 开启传输加速后，Bucket会在保留默认Endpoint的基础上新增传输加速域名，但必须使用OSS的传输加速域名才会提升访问速度。
    ///
    /// - 使用传输加速域名访问Bucket时，OSS会收取传输加速费用。详情请参见[传输加速费用](~~173539~~)。
    ///
    /// 有关传输加速的更多信息，请参见开发指南的[传输加速](~~131312~~)。
    ///
    /// # Path
    /// `/?transferAcceleration`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_transfer_acceleration(
        &self,
        req: PutBucketTransferAcceleration,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketTransferAccelerationResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间的传输加速配置
    ///
    /// 调用GetBucketTransferAcceleration接口获取目标存储空间（Bucket）的传输加速配置。
    ///
    /// - 只有Bucket拥有者以及被授予oss:GetBucketTransferAcceleration权限的RAM用户才能发起获取传输加速配置的请求。
    ///
    /// - 如果Bucket未配置过传输加速，调用该接口时不返回加速配置状态。
    ///
    /// 有关传输加速的更多信息，请参见开发指南的[传输加速](~~131312~~)。
    ///
    /// # Path
    /// `/?transferAcceleration`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_transfer_acceleration(
        &self,
        req: GetBucketTransferAcceleration,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketTransferAccelerationResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 设置存储空间的版本控制状态
    ///
    /// 设置指定存储空间（Bucket）的版本控制状态。
    ///
    /// - 要配置版本控制，您必须有PutBucketVersioning权限。
    ///
    /// - Bucket包括未开启、开启（Enabled）或者暂停（Suspended）三种版本控制状态。默认情况下Bucket处于未开启版本控制状态。
    ///
    /// - 在Bucket处于开启版本控制状态下，所有新添加的文件（Object）都将拥有唯一的版本ID，OSS将累积所添加Object的多个版本。
    ///
    /// - 在Bucket处于暂停版本控制状态下，所有新添加Object的版本ID将为null，且OSS将不再为此状态下添加的Object累积更多的版本。
    ///
    /// 关于版本控制的更多信息，请参见[版本控制介绍](~~109695~~)。
    ///
    /// # Path
    /// `/?versioning`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_versioning(
        &self,
        req: PutBucketVersioning,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketVersioningResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取指定存储空间的版本控制状态
    ///
    /// 调用GetBucketVersioning接口获取指定Bucket的版本控制状态。
    ///
    /// # Path
    /// `/?versioning`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_versioning(
        &self,
        req: GetBucketVersioning,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketVersioningResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取文件的版本信息
    ///
    /// 列出Bucket中包括删除标记（Delete Marker）在内的所有Object的版本信息。
    ///
    /// - GetBucket(ListObjects)接口仅返回Object的当前版本，且当前版本不能为删除标记。
    /// - GetBucketVersions(ListObjectVersions)接口返回Bucket中所有Object的所有版本。
    ///
    /// 不同Object之间按字母排序返回，同一个Object的不同版本则按从新到旧排序，与版本ID的字母序无关。
    ///
    /// # Path
    /// `/?versions`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_object_versions(
        &self,
        req: ListObjectVersions,
    ) -> impl std::future::Future<Output = crate::Result<ListObjectVersionsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 设置存储空间的授权策略
    ///
    /// 为指定的存储空间（Bucket）设置授权策略（Policy)。
    ///
    /// - 向其他账号的RAM用户授权访问。
    ///
    /// 您可以授予其他账号的RAM用户访问您的OSS资源的权限。
    ///
    /// - 向匿名用户授予带特定IP条件限制的访问权限。
    ///
    /// 某些场景下，您需要向匿名用户授予带IP限制的访问策略。例如，企业内部的机密文档，只允许在企业内部访问，不允许在其他区域访问。由于企业内部人员较多，如果针对每个人配置RAM Policy，工作量非常大。此时，您可以基于Bucket Policy设置带IP限制的访问策略，从而高效方便地进行授权。
    ///
    /// 有关Bucket Policy的配置详情及场景案例，请参见[使用Bucket Policy授权其他用户访问OSS资源](~~85111~~)。有关Policy语法，请参见[权限策略语法和结构](~~93739~~)。
    ///
    /// # Path
    /// `/?policy`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_policy(
        &self,
        req: PutBucketPolicy,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketPolicyResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间的权限策略
    ///
    /// 获取指定存储空间（Bucket）的权限策略（Policy）。
    ///
    /// - 向其他账号的RAM用户授权访问。
    ///
    /// 您可以授予其他账号的RAM用户访问您的OSS资源的权限。
    ///
    /// - 向匿名用户授予带特定IP条件限制的访问权限。
    ///
    /// 某些场景下，您需要向匿名用户授予带IP限制的访问策略。例如，企业内部的机密文档，只允许在企业内部访问，不允许在其他区域访问。由于企业内部人员较多，如果针对每个人配置RAM Policy，工作量非常大。此时，您可以基于Bucket Policy设置带IP限制的访问策略，从而高效方便地进行授权。
    ///
    /// 有关Bucket Policy的配置详情及场景案例，请参见[使用Bucket Policy授权其他用户访问OSS资源](~~85111~~)。有关Policy语法，请参见[权限策略语法和结构](~~93739~~)。
    ///
    /// # Path
    /// `/?policy`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_policy(
        &self,
        req: GetBucketPolicy,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketPolicyResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除存储空间的权限策略
    ///
    /// 删除指定存储空间（Bucket）的权限策略（Policy）。
    ///
    /// - 向其他账号的RAM用户授权访问。
    ///
    /// 您可以授予其他账号的RAM用户访问您的OSS资源的权限。
    ///
    /// - 向匿名用户授予带特定IP条件限制的访问权限。
    ///
    /// 某些场景下，您需要向匿名用户授予带IP限制的访问策略。例如，企业内部的机密文档，只允许在企业内部访问，不允许在其他区域访问。由于企业内部人员较多，如果针对每个人配置RAM Policy，工作量非常大。此时，您可以基于Bucket Policy设置带IP限制的访问策略，从而高效方便地进行授权。
    ///
    /// 有关Bucket Policy的配置详情及场景案例，请参见[使用Bucket Policy授权其他用户访问OSS资源](~~85111~~)。有关Policy语法，请参见[权限策略语法和结构](~~93739~~)。
    ///
    /// # Path
    /// `/?policy`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_policy(
        &self,
        req: DeleteBucketPolicy,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketPolicyResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间授权策略是否是公共访问
    ///
    /// 查看当前Bucket Policy是否允许公共访问。
    ///
    /// 阿里云账号默认拥有查看当前Bucket Policy是否允许公共访问的权限。如果您希望通过RAM用户或者STS的方式进行查看，您必须拥有`oss:GetBucketPolicyStatus`权限。
    ///
    /// # Path
    /// `/?policyStatus`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_policy_status(
        &self,
        req: GetBucketPolicyStatus,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketPolicyStatusResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 设置RTC
    ///
    /// 为已有的跨区域复制规则开启或关闭数据复制时间控制（RTC）功能。
    ///
    /// # Path
    /// `/?rtc`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_rtc(
        &self,
        req: PutBucketRtc,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketRtcResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 指定存储空间的数据复制规则
    ///
    /// 为存储空间（Bucket）指定数据复制规则。OSS支持跨区域复制（Cross-Region Replication）和同区域复制（Same-Region Replication）。
    ///
    /// 数据复制是以异步（近实时）方式将源Bucket中的文件（Object）以及对Object的创建、更新和删除等操作自动复制到目标Bucket。使用数据复制时，有如下注意事项：
    ///
    /// - 数据复制采用异步复制，数据复制到目标Bucket需要一定的时间，通常几分钟到几小时不等，具体取决于数据的大小。
    ///
    /// - 源Bucket与目标Bucket的名称不能相同。
    ///
    /// - 使用跨区域复制时，源Bucket与目标Bucket必须处于不同的数据中心；使用同地域复制时，源Bucket与目标Bucket必须处于相同的数据中心。
    ///
    /// 关于数据复制的更多信息，请分别参见[跨区域复制介绍](https://help.aliyun.com/document_detail/31864.htm?spm=a2c4g.11186623.0.0.32af6265m8tpXg#concept-zjp-31z-5db)和[同区域复制介绍](https://help.aliyun.com/document_detail/254865.htm?spm=a2c4g.11186623.0.0.32af6265m8tpXg#concept-2067125)。
    ///
    /// # Path
    /// `/?replication&comp=add`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn put_bucket_replication(
        &self,
        req: PutBucketReplication,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketReplicationResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 设置存储空间的数据复制规则
    ///
    /// 获取某个存储空间（Bucket）已设置的数据复制规则。
    ///
    /// # Path
    /// `/?replication`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_replication(
        &self,
        req: GetBucketReplication,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketReplicationResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取可复制到的目标存储空间的地域
    ///
    /// 获取可复制到的目标存储空间（Bucket）所在的地域。您可以根据返回结果决定将源Bucket的数据复制到哪个地域。
    ///
    /// # Path
    /// `/?replicationLocation`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_replication_location(
        &self,
        req: GetBucketReplicationLocation,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketReplicationLocationResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取存储空间的数据复制进度
    ///
    /// 获取某个存储空间（Bucket）的数据复制进度。
    ///
    /// # Path
    /// `/?replicationProgress`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_replication_progress(
        &self,
        req: GetBucketReplicationProgress,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketReplicationProgressResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 停止存储空间的数据复制并删除其复制配置
    ///
    /// 停止某个存储空间（Bucket）的数据复制并删除Bucket的复制配置，此时源Bucket中的任何操作都不会被同步到目标Bucket。
    ///
    /// - 当请求的Bucket没有配置数据复制规则时，调用此接口将返回200 HTTP OK。
    ///
    /// - 调用此接口删除某个数据复制规则时，该复制规则不会立刻被删除。OSS需要一定的时间来执行清理操作，此时复制规则的状态为closing。当清理工作完成后，该复制规则才被删除。
    ///
    /// - 当请求的Bucket的数据复制规则处于closing状态时，调用此接口将返回204 NoContent。
    ///
    /// # Path
    /// `/?replication&comp=delete`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn delete_bucket_replication(
        &self,
        req: DeleteBucketReplication,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketReplicationResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 配置存储空间的清单规则
    ///
    /// 为指定存储空间（Bucket）配置清单（Inventory）规则。
    ///
    ///
    /// - 只有Bucket的拥有者以及被授予PutBucketInventory权限的用户才能发起配置清单规则的请求。
    ///
    /// - 配置清单规则前需生成一个RAM角色，该角色需要拥有读取源Bucket所有文件和向目标Bucket写入文件的权限。首次使用清单功能时，建议您通过OSS控制台进行配置。清单规则配置完成后，您可以获取拥有所有权限的RAM角色。有关配置清单规则中RAM角色的权限说明，请参见[存储空间清单](~~163489~~)。
    ///
    /// - 单个Bucket最多只能配置1000条清单规则。
    ///
    /// - 配置清单的源Bucket与存放清单文件的目标Bucket必须位于同一个Region。
    ///
    /// # Path
    /// `/?inventory`
    ///
    /// # Extra Info
    /// ### SDK
    /// - [Java](~~177804~~)
    /// - [Python](~~177819~~)
    /// - [Go](~~177811~~)
    /// - [C++](~~177809~~)
    /// - [.NET](~~178701~~)
    /// - [Node.js](~~186530~~)
    ///
    /// ### 错误码
    /// |错误码|HTTP状态码|描述|
    /// |--|--|--|
    /// |InvalidArgument|400|传入非法参数。|
    /// |InventoryExceedLimit|400|超出清单配置规则的数量限制。|
    /// |AccessDenied|403|<br>**·**发起PutBucketInventory请求时，没有传入用户验证信息。<br>**·**用户无操作权限。
    /// |InventoryAlreadyExist|409|请求的清单规则已存在。|
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_inventory(
        &self,
        req: PutBucketInventory,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketInventoryResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 查看存储空间的清单任务
    ///
    /// 查看某个存储空间（Bucket）中指定的清单（Inventory）任务。
    ///
    /// 调用该请求时，请确保您有足够的权限对存储空间的清单任务进行操作。存储空间所有者默认拥有该权限，若您无该项权限，请先向存储空间所有者申请该项操作的权限。
    ///
    /// # Path
    /// `/?inventory`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_inventory(
        &self,
        req: GetBucketInventory,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketInventoryResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 批量获取存储空间的所有清单任务
    ///
    /// 批量获取某个存储空间（Bucket）中的所有清单（Inventory）任务。
    ///
    /// - 单次请求最多可获取100条清单配置项内容。若需获取超过100条清单配置项，则需发送多次请求，并保留相应的token，作为下一次请求的参数。
    ///
    /// - 调用该请求时，请确保您有足够的权限对存储空间的清单任务进行操作。存储空间所有者默认拥有该权限，若您无该项权限，请先向存储空间所有者申请该项操作的权限。
    ///
    /// # Path
    /// `/?inventory`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_bucket_inventory(
        &self,
        req: ListBucketInventory,
    ) -> impl std::future::Future<Output = crate::Result<ListBucketInventoryResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除存储空间的指定清单任务
    ///
    /// 删除某个存储空间（Bucket）中指定的清单（Inventory）任务。
    ///
    /// - 调用该请求时，请确保您有足够的权限对存储空间的清单任务进行操作。存储空间所有者默认拥有该权限，若您无该项权限，请先向存储空间所有者申请该项操作的权限。
    /// - 请求成功将返回HTTP状态码204。
    ///
    /// # Path
    /// `/?inventory`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_inventory(
        &self,
        req: DeleteBucketInventory,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketInventoryResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 开启存储空间的日志转存功能
    ///
    /// 为存储空间（Bucket）开启日志转存功能，可将OSS的访问日志按照固定命名规则，以小时为单位生成日志文件写入您指定的Bucket。
    ///
    /// - 生成日志的源Bucket和存储日志的目标Bucket可以相同也可以不同，但是必须属于同一账号下的相同地域。
    ///
    /// - 日志文件以小时为单位生成，但并不表示某个时段的日志文件记录了该时段的所有请求，部分请求可能会出现在上一时段或下一时段的日志文件中。
    /// 日志文件命名规则及日志格式说明，请参见[日志转存](~~31868~~)。
    ///
    /// - 在您关闭日志转存功能前，OSS的日志文件会一直生成。请及时清理不再需要的日志文件，以减少您的存储费用。
    /// 您可以通过生命周期规则定期删除日志文件。更多信息，请参见[生命周期规则介绍](~~31863~~)。
    ///
    /// - OSS会根据需求在日志的尾部添加一些字段，请您在开发日志处理工具时考虑兼容性的问题。
    ///
    /// # Path
    /// `/?logging`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_logging(
        &self,
        req: PutBucketLogging,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketLoggingResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 查看存储空间的访问日志配置
    ///
    /// 查看存储空间（Bucket）的访问日志配置。只有Bucket的拥有者才能查看Bucket的访问日志配置。
    ///
    /// # Path
    /// `/?logging`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_logging(
        &self,
        req: GetBucketLogging,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketLoggingResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 关闭存储空间的访问日志记录功能
    ///
    /// 关闭存储空间（Bucket）的访问日志记录功能。
    ///
    /// - 只有Bucket的拥有者才有权限关闭Bucket访问日志记录功能。
    /// - 如果目标Bucket没有开启访问日志记录功能，则返回参数显示204状态码。
    ///
    /// # Path
    /// `/?logging`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_logging(
        &self,
        req: DeleteBucketLogging,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketLoggingResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 修改存储空间实时日志用户自定义字段配置
    ///
    /// 为存储空间（Bucket）实时日志中的user_defined_log_fields字段进行个性化配置。您可以将OSS请求中用户关心的请求头或查询参数信息记录到该字段中去以便后续分析请求。
    ///
    /// - 本功能将相关信息统一编码后填入固定字段user_defined_log_fields中。user_defined_log_fields字段的值是一个JSON文本Base64编码后的值，这个JSON文本默认有一个字段为“truncated”用于表示是否有截断发生，另外有两个字段“headers”和“querys”分别对应用户配置的请求头和查询参数信息。
    ///
    /// - 自定义请求头个数和查询参数的个数之和不能超过6。
    ///
    /// - 自定义请求头及查询参数的key、value长度总和不能超过1024字节，超过部分会被截断。
    ///
    /// - 请求头不支持下划线（_），可以使用短划线（-）替代。查询参数支持下划线（_）。
    ///
    /// - 请求头需要遵从HTTP协议的规定。必须是可打印的ASCII字符，即字符33到字符126，支持小数点（.），不支持冒号（：）。
    ///
    /// # Path
    /// `/?userDefinedLogFieldsConfig`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_user_defined_log_fields_config(
        &self,
        req: PutUserDefinedLogFieldsConfig,
    ) -> impl std::future::Future<Output = crate::Result<PutUserDefinedLogFieldsConfigResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间实时日志用户自定义字段配置
    ///
    /// 获取存储空间（Bucket）实时日志中user_defined_log_fields字段的个性化配置。
    ///
    /// # Path
    /// `/?userDefinedLogFieldsConfig`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_user_defined_log_fields_config(
        &self,
        req: GetUserDefinedLogFieldsConfig,
    ) -> impl std::future::Future<Output = crate::Result<GetUserDefinedLogFieldsConfigResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除存储空间实时日志用户自定义字段配置
    ///
    /// 删除存储空间（Bucket）实时日志中user_defined_log_fields字段的个性化配置。
    ///
    /// # Path
    /// `/?userDefinedLogFieldsConfig`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_user_defined_log_fields_config(
        &self,
        req: DeleteUserDefinedLogFieldsConfig,
    ) -> impl std::future::Future<Output = crate::Result<DeleteUserDefinedLogFieldsConfigResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 查看存储空间的静态网站托管状态和规则
    ///
    /// 查看存储空间（Bucket）的静态网站托管状态以及跳转规则。
    ///
    /// # Path
    /// `/?website`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_website(
        &self,
        req: GetBucketWebsite,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketWebsiteResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 设置存储空间为静态网站托管模式并设置跳转规则
    ///
    /// 将存储空间（Bucket）设置为静态网站托管模式并设置跳转规则（RoutingRule）。
    ///
    /// - 功能支持：此接口主要用于设置默认主页、默认404页和RoutingRule。RoutingRule用来指定3xx跳转规则以及镜像回源规则。其中镜像回源支持公共云和金融云。
    ///
    /// - 使用自有域名访问静态网站：如果要使用自有域名来访问基于Bucket的静态网站，您可以通过域名CNAME来实现。具体操作，请参见绑定自定义域名。
    ///
    /// - 索引页面和错误页面：将一个Bucket设置为静态网站托管模式时，如果指定了索引页面或错误页面，则指定的索引页面和错误页面为该Bucket内的某个Object。
    ///
    /// - 对静态网站根域名的匿名访问：将一个Bucket设置为静态网站托管模式后，对静态网站根域名的匿名访问，OSS将返回索引页面。对静态网站根域名的签名访问，OSS将返回GetBucket(ListObjects)的结果。
    ///
    /// # Path
    /// `/?website`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_website(
        &self,
        req: PutBucketWebsite,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketWebsiteResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 关闭存储空间的静态网站托管模式以及跳转规则
    ///
    /// 关闭存储空间（Bucket）的静态网站托管模式以及跳转规则。
    ///
    /// 只有Bucket的拥有者才能关闭Bucket的静态网站托管模式。
    ///
    /// # Path
    /// `/?website`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_website(
        &self,
        req: DeleteBucketWebsite,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketWebsiteResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 设置存储空间的防盗链
    ///
    /// 设置存储空间（Bucket）级别的防盗链（Referer）访问白名单，支持设置是否允许Referer字段为空以及是否允许截断QueryString的请求访问OSS。
    ///
    /// # Path
    /// `/?referer`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_referer(
        &self,
        req: PutBucketReferer,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketRefererResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 查看存储空间的防盗链相关配置
    ///
    /// 查看存储空间（Bucket）的防盗链（Referer）相关配置。
    ///
    /// # Path
    /// `/?referer`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_referer(
        &self,
        req: GetBucketReferer,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketRefererResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 为存储空间添加或修改标签
    ///
    /// 给某个存储空间（Bucket）添加或修改标签。
    ///
    /// - 只有Bucket的拥有者及授权RAM账户才能为Bucket设置用户标签，否则返回403 Forbidden错误，错误码为AccessDenied。
    ///
    /// - 最多可设置20对Bucket用户标签（Key-Value对）。
    ///
    /// - PutBucketTags是覆盖语义，即新添加的标签会完全覆盖已有的标签。
    ///
    /// # Path
    /// `/?tagging`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_tags(
        &self,
        req: PutBucketTags,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketTagsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间的标签信息
    ///
    /// 获取存储空间（Bucket）的标签信息。
    ///
    /// # Path
    /// `/?tagging`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_tags(
        &self,
        req: GetBucketTags,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketTagsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除存储空间的标签
    ///
    /// 删除存储空间（Bucket）标签。
    ///
    /// 如果目标Bucket没有任何标签或指定标签的Key不存在，则返回HTTP状态码204。
    ///
    /// # Path
    /// `/?tagging`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_tags(
        &self,
        req: DeleteBucketTags,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketTagsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 列举用户级别存储冗余类型转换任务
    ///
    /// 获取用户级别存储冗余类型转换的列表。
    ///
    /// # Path
    /// `/?redundancyTransition`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_user_data_redundancy_transition(
        &self,
        req: ListUserDataRedundancyTransition,
    ) -> impl std::future::Future<Output = crate::Result<ListUserDataRedundancyTransitionResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 列举存储空间冗余类型转换任务
    ///
    /// 列举某个Bucket下所有的存储冗余转换任务。
    ///
    /// - 要列举Bucket下所有的存储冗余转换任务，您必须具有`oss:ListBucketDataRedundancyTransition`权限。具体操作，请参见[为RAM用户授权自定义的权限策略](~~199058~~)。
    /// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](~~31837~~)。
    ///
    /// # Path
    /// `/?redundancyTransition`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_bucket_data_redundancy_transition(
        &self,
        req: ListBucketDataRedundancyTransition,
    ) -> impl std::future::Future<Output = crate::Result<ListBucketDataRedundancyTransitionResponse>>
    + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取存储空间存储冗余转换任务
    ///
    /// 获取存储冗余转换任务。
    ///
    /// - 要获取存储冗余转换任务，您必须具有oss:GetBucketDataRedundancyTransition权限。具体操作，请参见[为RAM用户授权自定义的权限策略](~~199058~~)。
    /// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](~~31837~~)。
    ///
    /// # Path
    /// `/?redundancyTransition`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_data_redundancy_transition(
        &self,
        req: GetBucketDataRedundancyTransition,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketDataRedundancyTransitionResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 创建存储冗余类型转换任务
    ///
    /// 为Bucket创建存储冗余转换任务。
    ///
    /// - Bucket所属地域支持转换存储冗余类型。支持转换存储冗余类型的地域：华东1（杭州）、华东2（上海）、华北2（北京）、华北3（张家口）、华北6（乌兰察布）、华南1（深圳）、中国香港、日本（东京）、新加坡、印度尼西亚（雅加达）、德国（法兰克福）
    /// - Bucket的存储冗余类型必须为本地冗余存储。OSS仅支持将本地冗余存储转换为同城冗余存储。
    /// - Bucket的存储类型必须为标准存储、低频访问存储或归档存储，但Bucket中的文件的存储类型可以为冷归档存储和深度冷归档存储。冷归档存储和深度冷归档存储的文件转换后依然为本地冗余存储。冷归档存储和深度冷归档存储的Bucket不支持转换存储冗余类型。
    /// - 要创建存储冗余转换任务，您必须具有oss:CreateBucketDataRedundancyTransition权限。具体操作，请参见为[RAM用户授权自定义的权限策略](~~199058~~)。
    /// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](~~31837~~)。
    ///
    /// # Path
    /// `/?redundancyTransition`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn create_bucket_data_redundancy_transition(
        &self,
        req: CreateBucketDataRedundancyTransition,
    ) -> impl std::future::Future<
        Output = crate::Result<CreateBucketDataRedundancyTransitionResponse>,
    > + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除存储冗余类型转换任务
    ///
    /// 删除存储空间数据冗余类型转换任务。
    ///
    /// - 要删除存储冗余转换任务，您必须具有`oss:DeleteBucketDataRedundancyTransition`权限。具体操作，请参见[为RAM用户授权自定义的权限策略](~~199058~~)。
    /// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](~~31837~~)。
    /// - 处于Processing状态的任务不支持删除。
    ///
    /// # Path
    /// `/?redundancyTransition`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_data_redundancy_transition(
        &self,
        req: DeleteBucketDataRedundancyTransition,
    ) -> impl std::future::Future<
        Output = crate::Result<DeleteBucketDataRedundancyTransitionResponse>,
    > + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 配置存储空间的加密规则
    ///
    /// 配置存储空间（Bucket）的加密规则。
    ///
    /// 只有Bucket的拥有者及授权的RAM用户才能为Bucket设置加密规则，否则返回403错误。有关Bucket加密的更多信息，请参见[服务器端加密](~~31871~~)。
    ///
    /// # Path
    /// `/?encryption`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_encryption(
        &self,
        req: PutBucketEncryption,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketEncryptionResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间的加密规则
    ///
    /// 获取存储空间（Bucket）的加密规则。
    ///
    /// 只有Bucket的拥有者及授权的RAM用户才能获取Bucket的加密规则，否则返回403错误。有关Bucket加密的更多信息，请参见**[服务器端加密](~~31871~~)**。
    ///
    /// # Path
    /// `/?encryption`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_encryption(
        &self,
        req: GetBucketEncryption,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketEncryptionResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除存储空间的加密规则
    ///
    /// 删除指定存储空间（Bucket）的加密规则。
    ///
    /// 只有Bucket的拥有者及授权的RAM用户才能删除Bucket的加密规则，否则返回403错误。有关Bucket加密的更多信息，请参见[服务器端加密](~~31871~~)。
    ///
    /// # Path
    /// `/?encryption`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_encryption(
        &self,
        req: DeleteBucketEncryption,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketEncryptionResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 设置请求者付费模式
    ///
    /// 设置某个存储空间（Bucket）的请求者付费模式。
    ///
    /// - 不允许匿名访问：如果您在Bucket上启用了请求者付费模式，则不允许匿名访问该Bucket。请求方必须提供身份验证信息，以便OSS能够识别请求方，从而对请求方而非Bucket拥有者收取请求所产生的费用。
    /// 当请求者是通过扮演阿里云RAM角色来请求数据时，该角色所属的账户将为此请求付费。
    ///
    /// - 申请方需携带x-oss-request-payer信息：如果您在Bucket上启用了请求者付费模式，请求方必须在其请求中包含x-oss-request-payer:requester（在POST、GET和HEAD请求的Header信息中），以表明请求方已知悉请求和数据下载将产生费用。否则，请求方无法通过验证。
    /// 数据拥有者访问该Bucket时，可以不携带x-oss-request-payer请求头。数据拥有者作为请求者访问该Bucket时，请求产生的费用由数据拥有者（也是请求者）来支付。
    ///
    /// # Path
    /// `/?requestPayment`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_request_payment(
        &self,
        req: PutBucketRequestPayment,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketRequestPaymentResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取请求者付费模式配置信息
    ///
    /// 获取请求者付费模式的配置信息。
    ///
    /// # Path
    /// `/?requestPayment`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_request_payment(
        &self,
        req: GetBucketRequestPayment,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketRequestPaymentResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 设置存储空间的跨域资源共享规则
    ///
    /// 设置指定存储空间（Bucket）的跨域资源共享CORS（Cross-Origin Resource Sharing）规则。
    ///
    ///
    /// - 默认不开启CORS
    /// <br>Bucket默认不开启CORS功能，所有跨域请求的Origin都不被允许。
    ///
    /// - 覆盖语义
    ///  <br>PutBucketCors为覆盖语义，即新配置的CORS规则将覆盖已有的CORS规则。
    ///
    /// - 应用程序中使用CORS
    /// <br>在应用程序中使用CORS功能时，需通过PutBucketCors接口手动上传CORS规则来开启CORS功能。
    /// <br>例如从`example.com`通过浏览器的`XMLHttpRequest`功能来访问OSS，需要通过本接口手动上传CORS规则，且CORS规则需由XML文档进行描述。
    ///
    /// - CORS规则匹配
    /// <br>当OSS收到一个跨域请求或OPTIONS请求，会先读取Bucket对应的CORS规则，然后进行相应的权限检查。OSS会依次检查每一条规则，使用第一条匹配的规则来允许请求并返回对应的Header。如果所有规则都匹配失败，则不附加任何CORS相关的Header。
    /// <br>CORS规则匹配成功必须满足以下三个条件：
    ///     - 请求的Origin必须匹配一个`AllowedOrigin`项。
    ///
    ///     - 请求的方法（例如GET、PUT等）或者OPTIONS请求的`Access-Control-Request-Method`头对应的方法必须匹配一个`AllowedMethod`项。
    ///
    ///     - OPTIONS请求的`Access-Control-Request-Headers`头包含的每个header都必须匹配一个`AllowedHeader`项。
    ///
    /// # Path
    /// `/?cors`
    ///
    /// # Extra Info
    /// ### SDK
    ///
    /// - [Java](~~32018~~)
    /// - [Python](~~32036~~)
    /// - [PHP](~~32110~~)
    /// - [Go](~~32156~~)
    /// - [C](~~89705~~)
    /// - [C++](~~103198~~)
    /// - [.NET](~~32095~~)
    /// - [Node.js](~~142900~~)
    /// - [Ruby](~~32128~~)
    ///
    /// ### 错误码
    /// |错误码|HTTP状态码|描述|
    /// |--|--|--|
    /// |InvalidDigest|400|上传了Content-MD5请求头后，OSS会计算消息体的Content-MD5并检查一致性，如果不一致则返回此错误码。|
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_cors(
        &self,
        req: PutBucketCors,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketCorsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间当前的跨域资源共享规则
    ///
    /// 获取指定存储空间（Bucket）当前的跨域资源共享CORS（Cross-Origin Resource Sharing）规则。
    ///
    ///  
    ///
    /// # Path
    /// `/?cors`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_cors(
        &self,
        req: GetBucketCors,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketCorsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 关闭存储空间跨域资源共享功能
    ///
    /// 调用DeleteBucketCors接口关闭指定存储空间（Bucket）的跨域资源共享CORS（Cross-Origin Resource Sharing）功能并清空所有规则。
    ///
    /// ### 注意事项
    /// 要关闭CORS功能，您必须有`oss:DeleteBucketCors`权限。
    ///
    /// # Path
    /// `/?cors`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_cors(
        &self,
        req: DeleteBucketCors,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketCorsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 发送预检请求
    ///
    /// 浏览器在发送跨域请求之前会发送一个preflight请求（Options）给OSS，并带上特定的来源域、HTTP方法和header等信息，以决定是否发送真正的请求。
    ///
    /// Options请求是由浏览器自动根据是否跨域来决定是否发送。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Options
    ///
    pub fn option_object(
        &self,
        req: OptionObject,
    ) -> impl std::future::Future<Output = crate::Result<OptionObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'Origin': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Access-Control-Request-Method': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Access-Control-Request-Headers': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'Origin': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Access-Control-Request-Method': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Access-Control-Request-Headers': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 修改存储空间访问追踪状态
    ///
    /// 修改存储空间（Bucket）的访问追踪状态。
    ///
    /// # Path
    /// `/?accessmonitor`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_access_monitor(
        &self,
        req: PutBucketAccessMonitor,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketAccessMonitorResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间访问追踪状态
    ///
    /// 获取存储空间（Bucket）的访问追踪功能是否开启。
    ///
    /// # Path
    /// `/?accessmonitor`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_access_monitor(
        &self,
        req: GetBucketAccessMonitor,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketAccessMonitorResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取元数据索引库信息
    ///
    /// 获取指定存储空间（Bucket）的元数据索引库信息。
    ///
    /// # Path
    /// `/?metaQuery`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_meta_query_status(
        &self,
        req: GetMetaQueryStatus,
    ) -> impl std::future::Future<Output = crate::Result<GetMetaQueryStatusResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 关闭存储空间元数据管理
    ///
    /// 关闭存储空间（Bucket）的元数据管理功能。OSS会自动删除Bucket的元数据索引库，将无法进行元数据索引。
    ///
    /// # Path
    /// `/?metaQuery&comp=delete`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn close_meta_query(
        &self,
        req: CloseMetaQuery,
    ) -> impl std::future::Future<Output = crate::Result<CloseMetaQueryResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 查询满足条件的文件
    ///
    /// 通过存储空间（Bucket）的元数据索引功能，查询满足指定条件的文件（Object），并按照字段和排序方式列出文件信息。
    ///
    /// # Path
    /// `/?metaQuery&comp=query`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn do_meta_query(
        &self,
        req: DoMetaQuery,
    ) -> impl std::future::Future<Output = crate::Result<DoMetaQueryResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported."##
            );
        }
    }

    ///
    /// # 开启元数据管理功能
    ///
    /// 开启元数据管理功能。开启元数据管理功能后，OSS会为Bucket创建元数据索引库并为Bucket中的所有文件（Object）建立元数据索引。元数据索引库创建完成后，OSS会继续对Bucket中新增文件进行准实时的增量追踪扫描并为增量文件建立元数据索引。
    ///
    /// # Path
    /// `/?metaQuery&comp=add`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn open_meta_query(
        &self,
        req: OpenMetaQuery,
    ) -> impl std::future::Future<Output = crate::Result<OpenMetaQueryResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 更改高防OSS实例状态
    ///
    /// 更改高防OSS实例状态。
    ///
    /// # Path
    /// `/?antiDDos`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn update_user_anti_d_dos_info(
        &self,
        req: UpdateUserAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<UpdateUserAntiDDosInfoResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'x-oss-defender-instance': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-status': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-instance': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-status': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 更新Bucket防护状态
    ///
    /// 更新Bucket防护状态。
    ///
    /// # Path
    /// `/?antiDDos`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn update_bucket_anti_d_dos_info(
        &self,
        req: UpdateBucketAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<UpdateBucketAntiDDosInfoResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-instance': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-status': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-instance': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-status': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取Bucket防护信息列表
    ///
    /// 获取Bucket防护信息列表。
    ///
    /// # Path
    /// `/?bucketAntiDDos`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_bucket_anti_d_dos_info(
        &self,
        req: ListBucketAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<ListBucketAntiDDosInfoResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 创建高防OSS实例
    ///
    /// 创建高防OSS实例。
    ///
    /// # Path
    /// `/?antiDDos`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn init_user_anti_d_dos_info(
        &self,
        req: InitUserAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<InitUserAntiDDosInfoResponse>> + Send {
        async {
            todo!(
                r##"Only HttpMethod::Get or HttpMethod::Post supported
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 初始化Bucket防护
    ///
    /// 初始化Bucket防护。
    ///
    /// # Path
    /// `/?antiDDos`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn init_bucket_anti_d_dos_info(
        &self,
        req: InitBucketAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<InitBucketAntiDDosInfoResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-instance': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-type': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-instance': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-defender-type': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 查询高防OSS实例信息
    ///
    /// 查询指定账号下的高防OSS实例信息。
    ///
    /// # Path
    /// `/?antiDDos`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_user_anti_d_dos_info(
        &self,
        req: GetUserAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<GetUserAntiDDosInfoResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 获取存储空间资源组ID
    ///
    /// 获取存储空间（Bucket）所属的资源组ID。
    ///
    /// # Path
    /// `/?resourceGroup`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_resource_group(
        &self,
        req: GetBucketResourceGroup,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketResourceGroupResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 修改存储空间资源组
    ///
    /// 修改存储空间（Bucket）所属的资源组ID。
    ///
    /// # Path
    /// `/?resourceGroup`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_resource_group(
        &self,
        req: PutBucketResourceGroup,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketResourceGroupResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 绑定自定义域名
    ///
    /// 为某个存储空间（Bucket）绑定自定义域名。
    ///
    /// # Path
    /// `/?cname&comp=add`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn put_cname(
        &self,
        req: PutCname,
    ) -> impl std::future::Future<Output = crate::Result<PutCnameResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 查询自定义域名列表
    ///
    /// 查询某个存储空间（Bucket）下绑定的所有的自定义域名（Cname）列表。
    ///
    /// # Path
    /// `/?cname`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_cname(
        &self,
        req: ListCname,
    ) -> impl std::future::Future<Output = crate::Result<ListCnameResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除Cname
    ///
    /// 删除某个存储空间（Bucket）已绑定的Cname。
    ///
    /// # Path
    /// `/?cname&comp=delete`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn delete_cname(
        &self,
        req: DeleteCname,
    ) -> impl std::future::Future<Output = crate::Result<DeleteCnameResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取CnameToken
    ///
    /// 获取已创建的CnameToken。
    ///
    /// # Path
    /// `/?comp=token`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_cname_token(
        &self,
        req: GetCnameToken,
    ) -> impl std::future::Future<Output = crate::Result<GetCnameTokenResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 创建CnameToken
    ///
    /// 创建域名所有权验证所需的CnameToken。
    ///
    /// # Path
    /// `/?cname&comp=token`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn create_cname_token(
        &self,
        req: CreateCnameToken,
    ) -> impl std::future::Future<Output = crate::Result<CreateCnameTokenResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported."##
            );
        }
    }

    ///
    /// # 新增图片样式
    ///
    /// 新增图片样式。一个图片样式中可以包含单个或多个图片处理参数。
    ///
    /// # Path
    /// `/?style`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_style(
        &self,
        req: PutStyle,
    ) -> impl std::future::Future<Output = crate::Result<PutStyleResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 查询图片样式
    ///
    /// 查询某个Bucket下已创建的所有图片样式。
    ///
    /// # Path
    /// `/?style`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_style(
        &self,
        req: ListStyle,
    ) -> impl std::future::Future<Output = crate::Result<ListStyleResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取图片样式
    ///
    /// 查询某个Bucket下指定的图片样式信息。
    ///
    /// # Path
    /// `/?style`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_style(
        &self,
        req: GetStyle,
    ) -> impl std::future::Future<Output = crate::Result<GetStyleResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除图片样式
    ///
    /// 删除某个Bucket下指定的图片样式。
    ///
    /// # Path
    /// `/?style`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_style(
        &self,
        req: DeleteStyle,
    ) -> impl std::future::Future<Output = crate::Result<DeleteStyleResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间HTTPS配置
    ///
    /// 调用GetBucketHttpsConfig接口查看Bucket的TLS版本设置。
    ///
    /// - 要查看Bucket的TLS版本设置，您必须有oss:GetBucketHttpsConfig权限。具体操作，请参见[为RAM用户授权自定义的权限策略](https://help.aliyun.com/zh/oss/user-guide/common-examples-of-ram-policies#section-ucu-jv0-zip)。
    ///
    /// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](https://help.aliyun.com/zh/oss/user-guide/regions-and-endpoints)。
    ///
    /// # Path
    /// `/?httpsConfig`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_https_config(
        &self,
        req: GetBucketHttpsConfig,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketHttpsConfigResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 修改存储空间HTTPS配置
    ///
    /// 调用PutBucketHttpsConfig接口为Bucket开启或关闭TLS版本设置。
    ///
    /// # Path
    /// `/?httpsConfig`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_https_config(
        &self,
        req: PutBucketHttpsConfig,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketHttpsConfigResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 创建对象FC接入点
    ///
    /// 创建对象FC接入点。
    ///
    /// - 阿里云账号默认拥有创建对象FC接入点的权限。如果您希望通过RAM用户或者STS的方式创建对象FC接入点，您必须拥有`oss:CreateAccessPointForObjectProcess`权限。
    /// - 单个阿里云账号支持创建1000个对象FC接入点。
    /// - 单个Bucket支持创建100个对象FC接入点。
    ///
    /// # Path
    /// `/?accessPointForObjectProcess`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn create_access_point_for_object_process(
        &self,
        req: CreateAccessPointForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<CreateAccessPointForObjectProcessResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported."##
            );
        }
    }

    ///
    /// # 获取对象FC接入点详情
    ///
    /// 获取对象FC接入点基础信息。
    ///
    /// 阿里云账号默认拥有获取对象FC接入点基础信息的权限。如果您希望通过RAM用户或者STS的方式获取对象FC接入点基础信息，您必须拥有`oss:GetAccessPointForObjectProcess`权限。
    ///
    /// # Path
    /// `/?accessPointForObjectProcess`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_access_point_for_object_process(
        &self,
        req: GetAccessPointForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<GetAccessPointForObjectProcessResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 列举对象FC接入点
    ///
    /// 获取用户级别的对象FC接入点信息。
    ///
    /// 阿里云账号默认拥有获取对象FC接入点信息的权限。如果您希望通过RAM用户或者STS的方式获取对象FC接入点信息，您必须拥有`oss:ListAccessPointsForObjectProcess`权限。
    ///
    /// # Path
    /// `/?accessPointForObjectProcess`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_access_points_for_object_process(
        &self,
        req: ListAccessPointsForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<ListAccessPointsForObjectProcessResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 删除对象FC接入点
    ///
    /// 删除对象FC接入点。
    ///
    /// 阿里云账号默认拥有删除对象FC接入点的权限。如果您希望通过RAM用户或者STS的方式删除对象FC接入点，您必须拥有`oss:DeleteAccessPointForObjectProcess`权限。
    ///
    /// # Path
    /// `/?accessPointForObjectProcess`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_access_point_for_object_process(
        &self,
        req: DeleteAccessPointForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<DeleteAccessPointForObjectProcessResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取对象FC接入点配置
    ///
    /// 获取对象FC接入点配置信息。
    ///
    /// 阿里云账号默认拥有获取对象FC接入点配置信息的权限。如果您希望通过RAM用户或者STS的方式获取对象FC接入点配置信息，您必须拥有`oss:GetAccessPointConfigForObjectProcess`权限。
    ///
    /// # Path
    /// `/?accessPointConfigForObjectProcess`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_access_point_config_for_object_process(
        &self,
        req: GetAccessPointConfigForObjectProcess,
    ) -> impl std::future::Future<
        Output = crate::Result<GetAccessPointConfigForObjectProcessResponse>,
    > + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 修改对象FC接入点配置
    ///
    /// 修改对象FC接入点的配置。
    ///
    /// 阿里云账号默认拥有修改对象FC接入点配置的权限。如果您希望通过RAM用户或者STS的方式修改对象FC接入点配置，您必须拥有`oss:PutAccessPointConfigForObjectProcess`权限。
    ///
    /// # Path
    /// `/?accessPointConfigForObjectProcess`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_access_point_config_for_object_process(
        &self,
        req: PutAccessPointConfigForObjectProcess,
    ) -> impl std::future::Future<
        Output = crate::Result<PutAccessPointConfigForObjectProcessResponse>,
    > + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 修改对象FC接入点授权策略
    ///
    /// 为对象FC接入点配置权限策略。
    ///
    /// 阿里云账号默认拥有为对象FC接入点配置权限策略的权限。如果您希望通过RAM用户或者STS的方式为对象FC接入点配置权限策略，您必须拥有`oss:PutAccessPointPolicyForObjectProcess`权限。
    ///
    /// # Path
    /// `/?accessPointPolicyForObjectProcess`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_access_point_policy_for_object_process(
        &self,
        req: PutAccessPointPolicyForObjectProcess,
    ) -> impl std::future::Future<
        Output = crate::Result<PutAccessPointPolicyForObjectProcessResponse>,
    > + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取对象FC接入点策略
    ///
    /// 获取对象FC接入点的权限策略配置。
    ///
    /// 阿里云账号默认拥有获取对象FC接入点权限策略配置的权限。如果您希望通过RAM用户或者STS的方式获取对象FC接入点的权限策略配置，您必须拥有`oss:GetAccessPointPolicyForObjectProcess`权限。
    ///
    /// # Path
    /// `/?accessPointPolicyForObjectProcess`
    ///
    /// # Request Content-Type
    /// - `application/octet-stream`
    ///
    /// # Response Content-Type
    /// - `application/json`
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_access_point_policy_for_object_process(
        &self,
        req: GetAccessPointPolicyForObjectProcess,
    ) -> impl std::future::Future<
        Output = crate::Result<GetAccessPointPolicyForObjectProcessResponse>,
    > + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除对象FC接入点策略
    ///
    /// 删除对象FC接入点的权限策略。
    ///
    /// 阿里云账号默认拥有删除对象FC接入点策略的权限策略。如果您希望通过RAM用户或者STS的方式删除对象FC接入点的权限策略，您必须拥有`oss:DeleteAccessPointPolicyForObjectProcess`权限。
    ///
    /// # Path
    /// `/?accessPointPolicyForObjectProcess`
    ///
    /// # Request Content-Type
    /// - `application/octet-stream`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_access_point_policy_for_object_process(
        &self,
        req: DeleteAccessPointPolicyForObjectProcess,
    ) -> impl std::future::Future<
        Output = crate::Result<DeleteAccessPointPolicyForObjectProcessResponse>,
    > + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-access-point-for-object-process-name': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取用户级别阻止公共访问配置
    ///
    /// 获取绑定在用户级别的阻止公共访问的配置。
    ///
    /// # Path
    /// `/?publicAccessBlock`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_public_access_block(
        &self,
        req: GetPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<GetPublicAccessBlockResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 修改用户级别阻止公共访问配置
    ///
    /// 修改OSS全局阻止公共访问的配置信息。
    ///
    /// # Path
    /// `/?publicAccessBlock`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_public_access_block(
        &self,
        req: PutPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<PutPublicAccessBlockResponse>> + Send {
        async {
            todo!(
                r##"Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 删除用户级别的阻止公共访问配置
    ///
    /// 删除用户级别的阻止公共访问配置。
    ///
    /// # Path
    /// `/?publicAccessBlock`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_public_access_block(
        &self,
        req: DeletePublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<DeletePublicAccessBlockResponse>> + Send
    {
        async {
            todo!(
                r##"Only HttpMethod::Get or HttpMethod::Post supported
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间阻止公共访问配置
    ///
    /// 获取存储空间绑定的阻止公共访问配置。
    ///
    /// # Path
    /// `/?publicAccessBlock`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_public_access_block(
        &self,
        req: GetBucketPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketPublicAccessBlockResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 开启/关闭存储空间阻止公共访问配置
    ///
    /// 获取绑定在存储空间上的阻止公共访问的配置信息。
    ///
    /// # Path
    /// `/?publicAccessBlock`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_public_access_block(
        &self,
        req: PutBucketPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketPublicAccessBlockResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 删除存储空间阻止公共访问配置
    ///
    /// 删除绑定在存储空间上的组织公共访问配置信息。
    ///
    /// # Path
    /// `/?publicAccessBlock`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_public_access_block(
        &self,
        req: DeleteBucketPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketPublicAccessBlockResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取接入点阻止公共访问配置
    ///
    /// 获取指定接入点的阻止公共访问配置信息。
    ///
    /// # Path
    /// `/?publicAccessBlock`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_access_point_public_access_block(
        &self,
        req: GetAccessPointPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<GetAccessPointPublicAccessBlockResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 修改接入点阻止公共访问配置
    ///
    /// 修改指定接入点的阻止公共访问的配置信息。
    ///
    /// # Path
    /// `/?publicAccessBlock`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_access_point_public_access_block(
        &self,
        req: PutAccessPointPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<PutAccessPointPublicAccessBlockResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: API must have 200 response"##
            );
        }
    }

    ///
    /// # 删除接入点阻止公共访问配置
    ///
    /// 删除指定接入点的阻止公共访问配置信息。
    ///
    /// # Path
    /// `/?publicAccessBlock`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_access_point_public_access_block(
        &self,
        req: DeleteAccessPointPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<DeleteAccessPointPublicAccessBlockResponse>>
    + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间归档直读配置
    ///
    /// 查看Bucket是否开启归档直读。
    ///
    /// - 要查看Bucket是否开启归档直读，您必须有`oss:GetBucketArchiveDirectRead`权限。具体操作，请参见[为RAM用户授权自定义的权限策略](~~199058~~)。
    /// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](~~31837~~)。
    ///
    /// # Path
    /// `/?bucketArchiveDirectRead`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_archive_direct_read(
        &self,
        req: GetBucketArchiveDirectRead,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketArchiveDirectReadResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 修改存储空间归档直读配置
    ///
    /// 为Bucket开启或关闭归档直读。
    ///
    /// - 要为Bucket开启归档直读，您必须有`oss:PutBucketArchiveDirectRead`权限。具体操作，请参见[为RAM用户授权自定义的权限策略](~~199058~~)。
    ///
    /// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](~~31837~~)。
    ///
    /// - 开启归档直读后，直接读取归档存储类型的文件，会产生归档直读数据取回容量（RetrievalDataArchiveDirect）费用。对于已经解冻的归档存储类型的文件，不会产生归档直读数据取回容量费用。详情请参见[数据处理费用](~~173537~~)。
    ///
    /// # Path
    /// `/?bucketArchiveDirectRead`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_archive_direct_read(
        &self,
        req: PutBucketArchiveDirectRead,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketArchiveDirectReadResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 设置存储空间禁止覆盖写规则
    ///
    /// 设置存储空间的禁止覆盖写规则。
    ///
    /// # Path
    /// `/?overwriteConfig`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_overwrite_config(
        &self,
        req: PutBucketOverwriteConfig,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketOverwriteConfigResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间禁止覆盖写规则
    ///
    /// 获取存储空间的禁止覆盖写规则配置。
    ///
    /// # Path
    /// `/?overwriteConfig`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_overwrite_config(
        &self,
        req: GetBucketOverwriteConfig,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketOverwriteConfigResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除存储空间的不覆盖写规则
    ///
    /// 删除存储空间的不覆盖写规则配置。
    ///
    /// # Path
    /// `/?overwriteConfig`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_overwrite_config(
        &self,
        req: DeleteBucketOverwriteConfig,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketOverwriteConfigResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 上传文件
    ///
    /// 上传文件（Object）。
    ///
    /// **注意事项**
    ///
    /// - 添加的Object大小不能超过5 GB。
    ///
    /// - 默认情况下，如果已存在同名Object且对该Object有访问权限，则新添加的Object将覆盖原有的Object，并返回200 OK。
    ///
    /// - OSS没有文件夹的概念，所有资源都是以文件来存储，但您可以通过创建一个以正斜线（/）结尾，大小为0的Object来创建模拟文件夹。
    ///
    /// **版本控制**
    ///
    /// - 在已开启版本控制的Bucket中，OSS会为新添加的Object自动生成唯一的版本ID，并在响应Header中通过x-oss-version-id形式返回。
    /// - 在暂停了版本控制的Bucket中，新添加的Object的版本ID为null。OSS会保证同一个Object仅有一个null的版本ID。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/octet-stream`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_object(
        &self,
        req: PutObject,
    ) -> impl std::future::Future<Output = crate::Result<PutObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-data-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption-key-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-tagging': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-meta-*': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-data-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption-key-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-tagging': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-meta-*': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 拷贝文件
    ///
    /// 拷贝同一地域下相同或不同存储空间（Bucket）之间的文件（Object）。
    ///
    /// **版本控制**
    ///
    /// - `x-oss-copy-source`默认拷贝Object的当前版本，您可以在x-oss-copy-source中加入versionId来拷贝指定的Object版本。拷贝Object时，如果源Object的对应版本为删除标记，则返回404表示该Object不存在。
    /// - 如果需要恢复Object的早期版本为当前版本，您只需将Object的早期版本拷贝到同一个Bucket中，OSS会将该Object对应早期版本置为当前版本。
    /// - 如果目标Bucket已开启版本控制，OSS将会为新拷贝的Object自动生成唯一的版本ID，此版本ID将会在响应Header中的x-oss-version-id返回。如果目标Bucket未开启或者暂停了版本控制，OSS将会为新拷贝的Object自动生成version ID为null的版本，且会覆盖原有versionId为null的版本。
    ///
    /// **使用限制**
    ///
    /// - 使用CopyObject接口时，Object的大小限制说明如下：
    ///
    ///     - 如果源Bucket和目标Bucket相同，则Object的大小无限制。
    ///
    ///     - 如果源Bucket和目标Bucket不同，则建议拷贝小于1 GB的Object。当您需要拷贝大于1 GB的Object时，请使用[UploadPartCopy](~~31994~~)接口。
    ///   
    ///      使用CopyObject或UploadPartCopy接口均要求对源Object有读权限。
    ///
    /// - 在非版本控制的Bucket中，当调用CopyObject接口拷贝文件时，如果源Object与目标Object为同一个Object，则OSS只修改源Object的元数据，不拷贝源Object的内容。
    /// - 在版本控制的Bucket中，不支持拷贝通过追加上传方式生成的Object。
    /// - 如果源Object为软链接，则只拷贝软链接，无法拷贝软链接指向的文件内容。
    ///
    /// **计量计费**
    ///
    /// - 调用一次CopyObject接口会对源Object和目标Object所在的Bucket各增加一次Get请求次数。
    /// - 调用CopyObject接口会对目标Object所在的Bucket增加相应的存储量。
    /// - 调用CopyObject接口更改Object存储类型会涉及数据覆盖。例如低频访问IA创建后10天内被覆盖为标准存储Standard，则会产生20天的低频访问不足规定时长容量费用。关于存储费用的更多信息，请参见**[存储费用](~~173534~~)**。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn copy_object(
        &self,
        req: CopyObject,
    ) -> impl std::future::Future<Output = crate::Result<CopyObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-none-match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-unmodified-since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-modified-since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-metadata-directive': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-data-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption-key-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-tagging': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-tagging-directive': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-meta-*': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-none-match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-unmodified-since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-modified-since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-metadata-directive': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-data-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption-key-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-tagging': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-tagging-directive': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-meta-*': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取文件
    ///
    /// 获取某个文件（Object）。
    ///
    /// **注意事项**
    ///
    /// - GetObject接口默认可通过HTTP和HTTPS两种方式访问。如果要设置仅允许通过HTTPS方式访问，请使用Bucket Policy的授权访问方式。具体操作，请参见[通过Bucket Policy授权用户访问指定资源](~~85111~~)。
    ///
    /// - 如果Object类型为归档类型，需要先完成解冻文件（RestoreObject）请求，且该请求不能超时。
    ///
    /// **版本控制**
    ///
    /// 默认情况下，调用GetObject接口仅返回Object的当前版本。
    /// 如果在查询参数中指定Object的versionId，则返回指定的Object版本。当versionId指定为null时，则返回versionId为null的Object版本。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/octet-stream`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_object(
        &self,
        req: GetObject,
    ) -> impl std::future::Future<Output = crate::Result<GetObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'Range': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Modified-Since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Unmodified-Since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-None-Match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Accept-Encoding': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'Range': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Modified-Since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Unmodified-Since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-None-Match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Accept-Encoding': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 以追加写的方式上传文件
    ///
    /// 以追加写的方式上传文件（Object）。
    ///
    /// **版本控制**
    ///
    /// 在目标Bucket处于开启或暂停版本控制状态下，对Appendable类型Object执行相关操作时，有如下注意事项：
    /// - 仅允许对当前版本为Appendable类型的Object执行追加上传（AppendObject）操作，且OSS不会为该Appendable类型的Object生成历史版本。
    /// - 对当前版本为Appendable类型的Object执行PutObject或DeleteObject操作时，OSS会将该Appendable类型的Object保留为历史版本，但该Object不允许继续追加。
    /// - 不支持对Appendable类型Object执行拷贝操作。
    /// - 不支持对非Appendable类型的Object，包括Normal Object、删除标记（Delete Marker）等执行AppendObject操作。
    ///
    /// **使用限制**
    ///
    /// - 通过AppendObject方式最后生成的Object大小不得超过5 GB。
    /// - 处于[合规保留策略](~~90564~~)保护期的Object不支持AppendObject操作。
    /// - AppendableObject不支持指定CMK ID进行服务端KMS加密。
    ///
    /// # Path
    /// `/{key}?append`
    ///
    /// # Request Content-Type
    /// - `application/octet-stream`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn append_object(
        &self,
        req: AppendObject,
    ) -> impl std::future::Future<Output = crate::Result<AppendObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-meta-*': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Cache-Control': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-Disposition': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-Encoding': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-MD5': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Expires': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-meta-*': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Cache-Control': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-Disposition': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-Encoding': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-MD5': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Expires': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 封存可追加对象
    ///
    /// 通过AppendObject操作创建的Appendable Object，SealAppendable 操作用于Appendable Object停止继续写入。执行该操作后，允许用户通过配置生命周期来将对应的Appendable Object存储类型转为冷归档或深度冷归档存储类型。
    ///
    /// # Path
    /// `/{key}?seal`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn seal_append_object(
        &self,
        req: SealAppendObject,
    ) -> impl std::future::Future<Output = crate::Result<SealAppendObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 删除文件
    ///
    /// 删除文件（object）。
    ///
    /// **注意事项**
    ///
    /// - 要删除文件，您必须有Object的写权限。
    /// - 无论要删除的Object是否存在，删除成功后均会返回204状态码。
    /// - 如果Object类型为软链接，使用DeleteObject接口只会删除该软链接。
    ///
    /// **版本控制**
    ///
    /// 版本控制状态下的删除行为说明如下：
    ///
    /// - 未指定versionId（临时删除）：
    /// 如果在未指定versionId的情况下执行删除操作时，默认不会删除Object的当前版本，而是对当前版本插入删除标记（Delete Marker）。此时，在未指定versionId的情况下执行GetObject操作，OSS会检测到当前版本为删除标记，并返回`404 Not Found`。此外，响应中还会返回header：`x-oss-delete-marker = true`以及新生成的删除标记的版本号`x-oss-version-id`。
    /// `x-oss-delete-marker`的值为true，表示与返回的`x-oss-version-id`对应的版本为删除标记。
    ///
    /// - 指定versionId（永久删除）：
    /// 如果在指定versionId的情况下执行删除操作时，OSS会根据`params`中指定的`versionId`参数永久删除该版本。如果要删除ID为“null”的版本，请在`params`参数中添加`params['versionId'] = “null”`，OSS将“null”字符串当成“null”的versionId，从而删除versionId为“null”的Object。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_object(
        &self,
        req: DeleteObject,
    ) -> impl std::future::Future<Output = crate::Result<DeleteObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取文件的元信息
    ///
    /// 获取某个文件（Object）的元信息。
    ///
    /// - 使用此接口不会返回文件内容。
    ///
    /// - HeadObject操作默认获取Object当前版本的元信息。如果Object的当前版本为删除标记，则返回404 Not Found。请求参数中指定versionId则返回指定版本Object的元信息。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Head
    ///
    pub fn head_object(
        &self,
        req: HeadObject,
    ) -> impl std::future::Future<Output = crate::Result<HeadObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Modified-Since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Unmodified-Since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-None-Match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Modified-Since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Unmodified-Since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-Match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'If-None-Match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取文件的元数据信息
    ///
    /// 获取文件（Object）的元数据信息，包括该Object的ETag、Size、LastModified信息，并且不返回该Object的内容。
    ///
    /// **注意事项**
    /// - 文件（Object）的元数据信息包括该Object的ETag、Size、LastModified信息，且不返回该Object的内容。如果Object类型为软链接，则会返回软链接自身信息。
    /// - 当Bucket未启用版本控制时，要获取文件的元数据信息，您必须有oss:GetObject权限。当Bucket已启用版本控制时，要获取文件指定版本（请求中携带了x-oss-version-id请求头）的元数据信息，您必须有oss:GetObjectVersion权限。具体操作，请参见[为RAM用户授权自定义的RAM Policy](~~199058~~)。
    ///
    /// **版本控制**
    ///
    /// GetObjectMeta操作默认获取Object当前版本的元数据信息。如果Object的当前版本为删除标记，则返回404 Not Found。请求参数中指定versionId则返回指定版本Object的元数据信息。
    ///
    ///
    /// # Path
    /// `/{key}?objectMeta`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Head
    ///
    pub fn get_object_meta(
        &self,
        req: GetObjectMeta,
    ) -> impl std::future::Future<Output = crate::Result<GetObjectMetaResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 解冻归档类型或冷归档的文件
    ///
    /// 解冻归档类型（Archive）或冷归档（Cold Archive）的文件（Object）。
    ///
    /// **版本控制**
    ///
    /// Object的各个版本可以对应不同的存储类型。调用RestoreObject接口默认解冻Object当前版本，您可以通过指定versionId的方式来解冻Object指定版本。
    ///
    /// >- RestoreObject接口只针对归档或冷归档类型的Object，不适用于标准类型和低频访问类型的Object。
    /// - 如果针对该Object第一次调用RestoreObject接口，则返回202。
    /// - 如果已经成功调用过RestoreObject接口，且Object已完成解冻，再次调用时返回200 OK。
    ///
    /// **解冻过程说明**
    ///
    /// 对于归档类型或者冷归档类型的Object，如果需要读取Object，请提前解冻。归档类型的Object解冻有分钟级延迟，冷归档类型的Object解冻有数小时延迟。
    ///
    /// 归档类型或者冷归档类型的Object在执行解冻前后的状态变换过程如下：
    ///
    /// 1、 Object初始时处于冷冻状态。
    ///
    /// 2、提交一次解冻请求后，Object处于解冻中状态。
    ///
    /// 3、服务端完成解冻任务后，Object进入解冻状态，此时您可以读取Object。
    /// - 对于归档类型的Object，解冻状态默认持续24小时，24小时内再次调用RestoreObject接口则解冻状态会自动延长24小时，一次解冻流程内可有效调用7次RestoreObject接口达到最长7天的解冻持续时间。您也可以通过传入解冻天数，一次调用RestoreObject接口指定最长7天的解冻持续时间。
    /// - 对于冷归档类型的Object，您可以指定解冻天数和解冻优先级，解冻天数最短为1天，最长为7天。不同解冻优先级的首字节取回时间如下：
    ///
    ///   - 高优先级（Expedited）：表示1小时内完成解冻。
    ///   - 标准（Standard）：表示2~5小时内完成解冻。如果不传入JobParameters节点，则默认为Standard。
    ///   - 批量（Bulk）：表示5~12小时内完成解冻。
    ///
    /// 4、解冻状态结束后，Object再次返回到冷冻状态。
    ///
    /// **计费说明**
    ///
    /// 状态变换过程中产生的相关费用如下：
    /// - 对处于冷冻状态的Object执行解冻操作，会产生数据取回费用。
    /// - 解冻状态最多延长7天。在此期间不再重复收取数据取回费用。
    /// - 解冻状态结束后，Object又回到冷冻状态，再次执行解冻操作会收取数据取回费用。
    ///
    /// # Path
    /// `/{key}?restore`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn restore_object(
        &self,
        req: RestoreObject,
    ) -> impl std::future::Future<Output = crate::Result<RestoreObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 清理解冻副本
    ///
    /// 清理从冷归档或深度冷归档对象中解冻而来的副本
    ///
    /// # Path
    /// `/{key}?cleanRestoredObject`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn clean_restored_object(
        &self,
        req: CleanRestoredObject,
    ) -> impl std::future::Future<Output = crate::Result<CleanRestoredObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 对文件执行SQL语句并返回结果
    ///
    /// 对目标文件执行SQL语句，返回执行结果。
    ///
    /// - 此操作要求您对该Object有读权限。
    /// - 正确执行时，该API返回206。如果SQL语句不正确，或者和文件不匹配，则会返回400错误。
    /// - 关于SelectObject的功能介绍请参见[使用SelectObject查询文件](~~106082~~)。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/octet-stream`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn select_object(
        &self,
        req: SelectObject,
    ) -> impl std::future::Future<Output = crate::Result<SelectObjectResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported."##
            );
        }
    }

    ///
    /// # 获取目标文件总行数、总列数以及Splits个数
    ///
    /// 获取目标文件总的行数，总的列数（对于CSV文件），以及Splits个数。如果该信息不存在，则会扫描整个文件，分析并记录下CSV文件的上述信息。重复调用该API则会保存上述信息而不必重新扫描整个文件。
    ///
    /// - `CreateSelectObjectMeta`操作要求您对该Object有写权限。
    ///
    /// - 如果该API执行正确，返回200。如果目标文件不是合法CSV或者JSON LINES文件，或者指定的CSV分隔符和目标CSV不匹配，则返回400。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn create_select_object_meta(
        &self,
        req: CreateSelectObjectMeta,
    ) -> impl std::future::Future<Output = crate::Result<CreateSelectObjectMetaResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported."##
            );
        }
    }

    ///
    /// # 通知OSS初始化分片上传事件
    ///
    /// 通知OSS初始化一个Multipart Upload事件。
    ///
    /// - 调用接口会返回一个OSS服务器创建的全局唯一的Upload ID，用于标识本次Multipart Upload事件。您可以根据这个ID来发起相关的操作，例如中止Multipart Upload、查询Multipart Upload等。
    ///
    /// - 初始化MultipartUpload请求，并不影响已存在的同名Object。
    ///
    /// - 该操作计算认证签名时，需要添加`?uploads`到`CanonicalizedResource`中。
    ///
    /// # Path
    /// `/{key}?uploads`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn initiate_multipart_upload(
        &self,
        req: InitiateMultipartUpload,
    ) -> impl std::future::Future<Output = crate::Result<InitiateMultipartUploadResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-tagging': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-data-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption-key-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Cache-Control': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-Disposition': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-Encoding': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Expires': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-tagging': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-data-encryption': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-server-side-encryption-key-id': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Cache-Control': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-Disposition': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Content-Encoding': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'Expires': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 分块上传数据
    ///
    /// 根据指定的Object名和uploadId来分块（Part）上传数据。
    ///
    /// - 调用UploadPart接口上传Part数据前，必须先调用InitiateMultipartUpload接口来获取OSS服务器生成的uploadId。
    ///
    /// - 如果使用同一个partNumber上传了新的数据，则OSS上已有的partNumber对应的Part数据将被覆盖。
    ///
    /// - OSS会将服务器端收到Part数据的MD5值放在ETag头返回给用户。
    ///
    /// - 如果调用InitiateMultipartUpload接口时，指定了x-oss-server-side-encryption请求头，则会对上传的Part进行加密编码，并在UploadPart响应头中返回x-oss-server-side-encryption头，其值表明该Part的服务器端加密算法。更多信息，请参见[InitiateMultipartUpload](~~31992~~)。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/octet-stream`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn upload_part(
        &self,
        req: UploadPart,
    ) -> impl std::future::Future<Output = crate::Result<UploadPartResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 完成整个文件的分片上传
    ///
    /// 来完成整个文件的分片上传。
    ///
    /// **注意事项**
    ///
    ///
    /// 调用CompleteMultipartUpload操作时，用户必须提供所有有效的Part列表（包括PartNumber和ETag）。OSS收到用户提交的Part列表后，会逐一验证每个Part的有效性。当所有的Part验证通过后，OSS将把这些Part组合成一个完整的Object。
    /// - 确认Part的大小：CompleteMultipartUpload时会确认除最后一个Part以外所有Part的大小是否都大于或等于100 KB，并检查用户提交的Part列表中的每一个PartNumber和ETag。所以在上传Part时，客户端除了需要记录Part号码外，还需要记录每次上传Part成功后服务器返回的ETag值。
    ///
    /// - 处理请求：由于OSS处理CompleteMultipartUpload请求时会持续一定的时间。在这段时间内，如果客户端与OSS之间连接中断，OSS仍会继续该请求。
    ///
    /// - PartNumber：服务端在调用CompleteMultipartUpload接口时会对PartNumber做校验。
    /// PartNumber取值为1~10000。PartNumber可以不连续，但必须升序排列。例如第一个Part的PartNumber是1，第二个Part的PartNumber可以是5。
    ///
    /// - UploadId：同一个Object可以同时拥有不同的UploadId，当Complete一个UploadId后，此UploadId将无效，但该Object的其他UploadId不受影响。
    ///
    /// - x-oss-server-side-encryption请求头：如果调用InitiateMultipartUpload接口时，指定了x-oss-server-side-encryption请求头，则在CompleteMultipartUpload的响应头中返回x-oss-server-side-encryption，其值表示该Object的服务器端加密算法。
    ///
    /// **版本控制**
    ///
    /// 在开启版本控制的情况下，调用CompleteMultipartUpload接口来完成整个文件的MultipartUpload，OSS会为整个文件生成唯一的版本ID，并在响应header中以x-oss-version-id的形式返回。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn complete_multipart_upload(
        &self,
        req: CompleteMultipartUpload,
    ) -> impl std::future::Future<Output = crate::Result<CompleteMultipartUploadResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-complete-all': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-complete-all': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported."##
            );
        }
    }

    ///
    /// # 拷贝数据上传一个数据块
    ///
    /// 从一个已存在的Object中拷贝数据来上传一个Part。
    ///
    /// 该接口为通过在UploadPart请求的基础上增加一个请求头x-oss-copy-source来调用UploadPartCopy接口，实现从一个已存在的Object中拷贝数据来上传一个Part。
    ///
    /// **注意事项**
    /// <br>当拷贝一个大于1 GB的文件时，必须使用UploadPartCopy的方式进行拷贝。如果想通过单个操作拷贝小于1 GB的文件，请参见[CopyObject](https://help.aliyun.com/document_detail/31979.htm?spm=a2c4g.11186623.0.0.49aa29d6oRCP8o#t4696.html)。
    /// <br>使用UploadPartCopy接口时，有如下注意事项：
    ///
    /// - 不允许拷贝以AppendObject方式上传的Object。
    ///
    /// - 执行UploadPartCopy的源Bucket地址和目标Bucket地址必须是同一个Region。
    ///
    /// - 调用该接口上传Part数据前，必须先调用InitiateMultipartUpload接口来获取一个OSS服务器颁发的Upload ID。
    ///
    /// - 若调用InitiateMultipartUpload接口时，指定了x-oss-server-side-encryption请求头，则会对上传的Part进行加密编码，并在UploadPart响应头中返回x-oss-server-side-encryption头，其值表明该Part的服务器端加密算法，详情请参见[InitiateMultipartUpload](~~31992~~)。
    ///
    /// - MultipartUpload要求除最后一个Part以外，其他的Part大小都要大于100 KB。因不确定是否为最后一个Part，UploadPart接口并不会立即校验上传Part的大小，只有当CompleteMultipartUpload的时候才会校验。
    ///
    /// **版本控制**
    ///
    /// UploadPartCopy默认从一个已存在的Object的当前版本中拷贝数据来上传一个Part。允许通过在请求头x-oss-copy-source中附带versionId的子条件，实现从Object的指定版本进行拷贝，例如x-oss-copy-source : /SourceBucketName/SourceObjectName?versionId=111111。
    /// >SourceObjectName要进行URL编码。响应中将会返回被拷贝Object的versionId：x-oss-copy-source-version-id。
    ///
    /// 如果未指定versionId且拷贝Object的当前版本为删除标记（Delete Marker），OSS将返回404 Not Found。通过指定versionId来拷贝删除标记时，OSS将返回400 Bad Request。
    ///
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn upload_part_copy(
        &self,
        req: UploadPartCopy,
    ) -> impl std::future::Future<Output = crate::Result<UploadPartCopyResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-range': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-none-match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-unmodified-since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-modified-since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-range': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-none-match': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-unmodified-since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-copy-source-if-modified-since': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 取消分片上传事件并删除数据块数据
    ///
    /// 用于取消MultipartUpload事件并删除对应的Part数据。
    ///
    /// **注意事项**
    ///
    /// - 您需要提供MultipartUpload事件相应的uploadId。
    /// - 取消一个MultipartUpload事件过程中，如果所属的某些Part仍然在上传，那么此次取消操作将无法删除这些Part。
    /// - 取消一个MultipartUpload事件后，您无法再使用此uploadId做任何操作，已经上传的Part数据也会被删除。
    /// - 建议您及时完成分片上传或者取消分片上传，因为已上传但是未完成或未取消的分片会占用存储空间进而产生存储费用。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn abort_multipart_upload(
        &self,
        req: AbortMultipartUpload,
    ) -> impl std::future::Future<Output = crate::Result<AbortMultipartUploadResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 列举执行中的分片上传事件
    ///
    /// 列举所有执行中的Multipart Upload事件。
    ///
    /// 调用ListMultipartUploads接口列举所有执行中的Multipart Upload事件，即已经初始化但还未完成（Complete）或者还未中止（Abort）的Multipart Upload事件。
    ///
    /// # Path
    /// `/?uploads`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_multipart_uploads(
        &self,
        req: ListMultipartUploads,
    ) -> impl std::future::Future<Output = crate::Result<ListMultipartUploadsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 列举所有成功上传的数据块
    ///
    /// 列举指定Upload ID所属的所有已经上传成功Part。
    ///
    /// - OSS的返回结果按照Part号码升序排列。
    ///
    /// - 由于网络传输可能出错，所以不推荐使用ListParts返回结果中的Part Number和ETag值来生成已经上传成功的Part列表。
    ///
    /// # Path
    /// `/{key}`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_parts(
        &self,
        req: ListParts,
    ) -> impl std::future::Future<Output = crate::Result<ListPartsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 修改文件的访问权限
    ///
    /// 修改文件（Object）的访问权限（ACL）。
    ///
    /// **注意事项**
    ///
    /// 此操作只有Bucket Owner有权限执行，且需对Object有读写权限。
    ///
    /// **版本控制**
    ///
    /// 调用PutObjectACL接口时，默认只能设置Object当前版本的ACL。您可以通过指定versionId参数来设置指定Object版本的ACL。如果Object的对应版本为删除标记（Delete Marker），则OSS将返回404 Not Found。
    ///
    /// **ACL说明**
    ///
    /// PutObjectACL接口通过Put请求中的`x-oss-object-acl`头来设置Object ACL。目前Object包括如下四种访问权限。
    ///
    /// - private：Object是私有资源。只有该Object的Owner拥有该Object的读写权限，其他用户没有权限操作该Object。
    /// - public-read：Object是公共读资源。Object Owner拥有该Object的读写权限。非Object Owner只有该Object的读权限。
    /// - public-read-write：Object是公共读写资源。所有用户拥有对该Object的读写权限。
    /// - default：Object遵循其所在Bucket的读写权限，即Bucket是什么权限，Object就是什么权限。
    ///
    /// >**注意**
    /// - Object ACL优先级高于Bucket ACL。例如Bucket ACL是private的，而Object ACL是public-read-write的，则所有用户都拥有该Object的访问权限，即使该Bucket是私有Bucket。如果某个Object未设置过ACL，则访问权限遵循Bucket ACL。
    /// - Object的读操作包括GetObject、HeadObject、CopyObject和UploadPartCopy中的对原Object的读；Object的写操作包括PutObject、PostObject、AppendObject、DeleteObject、DeleteMultipleObjects、CompleteMultipartUpload以及CopyObject对新Object的写。
    /// - 您还可以在进行Object的写操作时，在请求头中带上x-oss-object-acl来设置Object
    ///  ACL，效果与PutObjectACL等同。例如PutObject时在请求头中带上x-oss-object-acl可以在上传一个Object的同时设置此Object的ACL。
    ///
    /// # Path
    /// `/{key}?acl`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_object_acl(
        &self,
        req: PutObjectAcl,
    ) -> impl std::future::Future<Output = crate::Result<PutObjectAclResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取文件的访问权限
    ///
    /// 获取存储空间（Bucket）下某个文件（Object）的访问权限（ACL）。
    ///
    /// **版本控制**
    ///
    /// 调用GetObjectACL接口时，默认只能获取Object当前版本的ACL。您可以通过指定versionId参数来获取指定Object版本的ACL。如果Object的对应版本为删除标记（Delete Marker），则OSS将返回404 Not Found。
    ///
    /// >如果一个Object从未设置过ACL，则调用GetObjectACL时，返回的ObjectACL为default，表示该Object的ACL遵循Bucket ACL。即如果Bucket的访问权限是private，则该Object的访问权限也是private。
    ///
    /// # Path
    /// `/{key}?acl`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_object_acl(
        &self,
        req: GetObjectAcl,
    ) -> impl std::future::Future<Output = crate::Result<GetObjectAclResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 创建OSS目标文件的软链接
    ///
    /// 为OSS的目标文件（TargetObject）创建软链接（Symlink）。
    ///
    /// **注意事项**
    ///
    /// - 使用PutSymlink接口创建软链接时不会检查目标文件是否存在、目标文件类型是否合法以及目标文件是否有访问权限。
    /// - Symlink自身的访问权限（ACL）以及目标文件的ACL检查仅会在GetObject等需要访问目标文件的API中进行。
    /// - 使用PutSymlink接口时，携带以x-oss-meta-为前缀的参数，则被视为user meta，例如x-oss-meta-location。一个Object可以有多个类似的参数，但所有的user meta总大小不能超过8 KB。
    /// - 默认情况下，如果试图添加的文件已经存在，并且有访问权限，则新添加的文件将覆盖原来的文件，成功添加后将返回200 OK。
    ///
    /// **版本控制**
    ///
    /// 您可以通过TargetObject创建的软链接指向TargetObject的当前版本。
    /// 软链接本身也可以有多个版本，每个不同的版本可以指向不同的TargetObject，版本ID由OSS自动生成，在响应Header中返回x-oss-version-id。
    ///
    ///
    /// # Path
    /// `/{key}?symlink`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_symlink(
        &self,
        req: PutSymlink,
    ) -> impl std::future::Future<Output = crate::Result<PutSymlinkResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-symlink-target': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-symlink-target': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-object-acl': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-storage-class': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Parameter 'x-oss-forbid-overwrite': Unsupported ParameterIn variant: Header. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取软链接
    ///
    /// 获取OSS目标文件（TargetObject）的软链接。
    ///
    /// **版本控制**
    ///
    /// GetSymlink接口默认获取软链接的当前版本。允许通过指定versionId来获取指定版本。如果软链接的当前版本为删除标记，OSS会返回404 Not Found，在响应header中返回x-oss-delete-marker = true以及版本ID : x-oss-version-id。删除标记没有关联数据，因此也没有软链接指向的TargetObject。
    ///
    ///
    /// # Path
    /// `/{key}?symlink`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_symlink(
        &self,
        req: GetSymlink,
    ) -> impl std::future::Future<Output = crate::Result<GetSymlinkResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 设置或更新文件的标签信息
    ///
    /// 设置或更新对象（Object）的标签（Tagging）信息。
    ///
    /// **注意事项**
    ///
    /// - 对象标签使用一组键值对（Key-Value）标记对象。
    /// - 单个Object最多能设置10个标签，Key不能重复。
    /// - 每个Key长度不能超过128字符，每个Value长度不能超过256字符。
    /// - Key和Value区分大小写。
    /// - 标签的合法字符集包括大小写字母、数字、空格和以下符号：
    /// +‑=._:/
    /// 通过HTTP Header的方式设置标签且标签中包含任意字符时，您需要对标签的Key和Value进行URL编码。
    /// - 更改标签信息不会更新Object的Last‑Modified时间。
    ///
    /// 关于对象标签的更多信息，请参见[对象标签](~~106678~~)。
    ///
    /// **版本控制**
    ///
    /// 调用PutObjectTagging接口时，默认设置Object当前版本的标签信息。您可以通过指定versionId参数来设置指定Object版本的标签信息。如果Object的对应版本为删除标记（Delete Marker），则OSS将返回404 Not Found。
    ///
    /// # Path
    /// `/{key}?tagging`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_object_tagging(
        &self,
        req: PutObjectTagging,
    ) -> impl std::future::Future<Output = crate::Result<PutObjectTaggingResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取文件的标签信息
    ///
    /// 获取对象（Object）的标签（Tagging）信息。
    ///
    /// **版本控制**
    ///
    /// 调用GetObjectTagging接口时，默认只能获取Object当前版本的标签信息。您可以通过指定versionId参数来获取指定Object版本的标签信息。如果Object的对应版本为删除标记（Delete Marker），则OSS将返回404 Not Found。
    ///
    ///
    /// # Path
    /// `/{key}?tagging`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_object_tagging(
        &self,
        req: GetObjectTagging,
    ) -> impl std::future::Future<Output = crate::Result<GetObjectTaggingResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除对象的标签信息
    ///
    /// 删除指定对象（Object）的标签（Tagging）信息。
    ///
    /// **版本控制**
    ///
    /// 调用DeleteObjectTagging接口时，默认只能删除Object当前版本的标签信息。您可以通过指定versionId参数来删除指定Object版本的标签信息。如果Object的对应版本为删除标记（Delete Marker），则OSS将返回404 Not Found。
    ///
    ///
    /// # Path
    /// `/{key}?tagging`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_object_tagging(
        &self,
        req: DeleteObjectTagging,
    ) -> impl std::future::Future<Output = crate::Result<DeleteObjectTaggingResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'key': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 创建直播推流
    ///
    /// 通过RTMP协议上传音视频数据前，必须先创建一个LiveChannel。
    ///
    /// 通过RTMP协议上传音视频数据前，必须先调用该接口创建一个LiveChannel。调用该接口会返回RTMP推流地址，以及对应的播放地址。
    /// <br>您可以使用返回的地址进行推流、播放，您还可以根据该LiveChannel的名称来发起相关的操作，如查询推流状态、查询推流记录、禁止推流等。
    ///
    /// # Path
    /// `/{channel}?live`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_live_channel(
        &self,
        req: PutLiveChannel,
    ) -> impl std::future::Future<Output = crate::Result<PutLiveChannelResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported."##
            );
        }
    }

    ///
    /// # 列举指定的直播推流
    ///
    /// 列举指定的LiveChannel。
    ///
    /// # Path
    /// `/?live`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_live_channel(
        &self,
        req: ListLiveChannel,
    ) -> impl std::future::Future<Output = crate::Result<ListLiveChannelResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 删除指定的直播推流
    ///
    /// 删除指定的LiveChannel。
    ///
    /// >- 当有客户端正在向LiveChannel推流时，删除请求会失败。
    /// - 本接口只会删除LiveChannel本身，不会删除推流生成的文件。
    ///
    /// # Path
    /// `/{channel}?live`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_live_channel(
        &self,
        req: DeleteLiveChannel,
    ) -> impl std::future::Future<Output = crate::Result<DeleteLiveChannelResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 切换直播推流状态
    ///
    /// 切换LiveChannel启用（enabled）和禁用（disabled）两种状态。
    ///
    /// - LiveChannel处于disabled状态时，OSS会禁止您向该LiveChannel进行推流操作。如果您正在向该LiveChannel推流，那么推流的客户端会被强制断开（会有10s左右的延迟）。
    /// - 当没有客户端向该LiveChannel推流时，调用PutLiveChannelStatus重新创建LiveChannel也可以达到修改Status的目的。
    /// - 当有客户端向该LiveChannel推流时，只能将LiveChannel的状态修改为disabled，无法调用PutLiveChannelStatus重新创建LiveChannel。
    ///
    /// # Path
    /// `/{channel}?live`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_live_channel_status(
        &self,
        req: PutLiveChannelStatus,
    ) -> impl std::future::Future<Output = crate::Result<PutLiveChannelStatusResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取直播推流的配置信息
    ///
    /// 获取指定LiveChannel的配置信息。
    ///
    /// # Path
    /// `/{channel}?live`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_live_channel_info(
        &self,
        req: GetLiveChannelInfo,
    ) -> impl std::future::Future<Output = crate::Result<GetLiveChannelInfoResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取直播推流的推流记录
    ///
    /// 获取指定LiveChannel的推流记录。
    ///
    /// 使用GetLiveChannelHistory接口最多会返回指定LiveChannel最近的10次推流记录。
    ///
    /// # Path
    /// `/{channel}?live&comp=history`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_live_channel_history(
        &self,
        req: GetLiveChannelHistory,
    ) -> impl std::future::Future<Output = crate::Result<GetLiveChannelHistoryResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 获取直播推流的推流状态信息
    ///
    /// 获取指定LiveChannel的推流状态信息。
    ///
    /// # Path
    /// `/{channel}?live&comp=stat`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_live_channel_stat(
        &self,
        req: GetLiveChannelStat,
    ) -> impl std::future::Future<Output = crate::Result<GetLiveChannelStatResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 查看直播推流的播放列表
    ///
    /// 查看指定LiveChannel在指定时间段内推流生成的播放列表。
    ///
    /// # Path
    /// `/{channel}?vod`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/octet-stream`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_vod_playlist(
        &self,
        req: GetVodPlaylist,
    ) -> impl std::future::Future<Output = crate::Result<GetVodPlaylistResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// # 为直播推流生成点播的播放列表
    ///
    /// 为指定的LiveChannel生成一个点播用的播放列表。
    ///
    /// OSS会查询指定时间范围内由该LiveChannel推流生成的ts文件，并将其拼装为一个m3u8播放列表。
    ///
    /// # Path
    /// `/{channel}/{playlist}?vod`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Post
    ///
    pub fn post_vod_playlist(
        &self,
        req: PostVodPlaylist,
    ) -> impl std::future::Future<Output = crate::Result<PostVodPlaylistResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'playlist': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'channel': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Parameter 'playlist': Unsupported ParameterIn variant: Path. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 创建图片处理通道
    ///
    /// 创建图片处理通道
    ///
    /// # Path
    /// `/?img`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_channel(
        &self,
        req: PutChannel,
    ) -> impl std::future::Future<Output = crate::Result<PutChannelResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 修改存储空间哈希算法配置
    ///
    /// 修改存储空间哈希算法配置
    ///
    /// # Path
    /// `/?objectHash`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_hash(
        &self,
        req: PutBucketHash,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketHashResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 设置存储空间的用户自定义响应头配置
    ///
    /// 设置存储空间的用户自定义响应头配置
    ///
    /// # Path
    /// `/?x-oss-common-header`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_bucket_common_header(
        &self,
        req: PutBucketCommonHeader,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketCommonHeaderResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 删除存储空间的用户自定义响应头配置
    ///
    /// 删除存储空间的用户自定义响应头配置
    ///
    /// # Path
    /// `/?x-oss-common-header`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Delete
    ///
    pub fn delete_bucket_common_header(
        &self,
        req: DeleteBucketCommonHeader,
    ) -> impl std::future::Future<Output = crate::Result<DeleteBucketCommonHeaderResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 修改存储空间媒体处理配置
    ///
    /// 修改存储空间媒体处理配置
    ///
    /// # Path
    /// `/?processConfiguration`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_process_configuration(
        &self,
        req: PutProcessConfiguration,
    ) -> impl std::future::Future<Output = crate::Result<PutProcessConfigurationResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// # 获取存储空间事件通知配置
    ///
    /// 获取存储空间事件通知配置
    ///
    /// # Path
    /// `/?eventNotification`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn get_bucket_event_notification(
        &self,
        req: GetBucketEventNotification,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketEventNotificationResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }

    ///
    /// 配置OSS加速器异步预热规则
    ///
    /// # Path
    /// `/?x-oss-datalake-cache-prefetch-job`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn put_data_lake_cache_prefetch_job(
        &self,
        req: PutDataLakeCachePrefetchJob,
    ) -> impl std::future::Future<Output = crate::Result<PutDataLakeCachePrefetchJobResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'body': Unsupported ParameterStyle variant: Xml. Only Json, Flat, RepeatList and Simple styles are supported."##
            );
        }
    }

    ///
    /// 启动OSS加速器异步预热任务
    ///
    /// # Path
    /// `/?x-oss-datalake-cache-prefetch-job&x-oss-datalake-job-start`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Put
    ///
    pub fn start_data_lake_cache_prefetch_job(
        &self,
        req: StartDataLakeCachePrefetchJob,
    ) -> impl std::future::Future<Output = crate::Result<StartDataLakeCachePrefetchJobResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Only HttpMethod::Get or HttpMethod::Post supported
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Response struct error: 200 response must have schema"##
            );
        }
    }

    ///
    /// 列举数据湖元数据转换任务
    ///
    /// # Path
    /// `/?x-oss-datalake-storage-transfer-job`
    ///
    /// # Request Content-Type
    /// - `application/xml`
    ///
    /// # Response Content-Type
    /// - `application/xml`
    ///
    /// # Methods
    /// - Get
    ///
    pub fn list_data_lake_storage_transfer_job(
        &self,
        req: ListDataLakeStorageTransferJob,
    ) -> impl std::future::Future<Output = crate::Result<ListDataLakeStorageTransferJobResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported.
Parameter 'bucket': Unsupported ParameterIn variant: Host. Only Query, FormData, and Body parameters are supported."##
            );
        }
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListBuckets {
    /// 限定返回的Bucket名称必须以prefix作为前缀。如果不设定，则不过滤前缀信息。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 设定结果从marker之后按字母排序的第一个开始返回。如果不设定，则从头开始返回数据。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    marker: Option<String>,
    /// 限定此次返回Bucket的最大个数。
    /// 取值范围：1~1000
    /// 默认值：100
    #[setters(generate = true, strip_option)]
    max_keys: Option<i64>,
    /// 指定Bucket标签键。列举结果中仅会包含那些打上了对应标签的Bucket。
    #[setters(generate = true, strip_option)]
    tag_key: Option<String>,
    /// 指定Bucket标签值。如果请求指定了该参数，那么必须同时指定tag-value，列举结果中仅会包含那些打上了对应标签键值对的Bucket。
    #[setters(generate = true, strip_option)]
    tag_value: Option<String>,
    /// 指定标签列表。如："k1":"v1","k2":"v2"，只有Bucket匹配列表中所有的标签键值对，才会出现在列举结果中。
    /// tagging参数与tag-key和tag-value两个参数不能同时存在。
    #[setters(generate = true, strip_option)]
    tagging: Option<String>,
}

impl sealed::Bound for ListBuckets {}

impl ListBuckets {
    pub fn new() -> Self {
        Self {
            prefix: None,
            marker: None,
            max_keys: None,
            tag_key: None,
            tag_value: None,
            tagging: None,
        }
    }
}

impl crate::Request for ListBuckets {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListBuckets";

    type Body = ();

    type Response = ListBucketsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.prefix {
            params.insert("prefix".into(), f.into());
        }

        if let Some(f) = &self.marker {
            params.insert("marker".into(), f.into());
        }

        if let Some(f) = &self.max_keys {
            params.insert("max-keys".into(), f.into());
        }

        if let Some(f) = &self.tag_key {
            params.insert("tag-key".into(), f.into());
        }

        if let Some(f) = &self.tag_value {
            params.insert("tag-value".into(), f.into());
        }

        if let Some(f) = &self.tagging {
            params.insert("tagging".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DescribeRegions {
    /// 请求的地域ID。
    #[setters(generate = true, strip_option)]
    regions: Option<String>,
}

impl sealed::Bound for DescribeRegions {}

impl DescribeRegions {
    pub fn new() -> Self {
        Self { regions: None }
    }
}

impl crate::Request for DescribeRegions {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DescribeRegions";

    type Body = ();

    type Response = DescribeRegionsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.regions {
            params.insert("regions".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketStat {}

impl sealed::Bound for GetBucketStat {}

impl GetBucketStat {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketStat {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketStat";

    type Body = ();

    type Response = GetBucketStatResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucket {}

impl sealed::Bound for PutBucket {}

impl PutBucket {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucket {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucket";

    type Body = ();

    type Response = PutBucketResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucket {}

impl sealed::Bound for DeleteBucket {}

impl DeleteBucket {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucket {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucket";

    type Body = ();

    type Response = DeleteBucketResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListObjects {
    /// 对Object名字进行分组的字符。所有Object名字包含指定的前缀，第一次出现delimiter字符之间的Object作为一组元素（即CommonPrefixes）。
    #[setters(generate = true, strip_option)]
    delimiter: Option<String>,
    /// 设定从marker之后按字母排序开始返回Object。
    /// <br>marker用来实现分页显示效果，参数的长度必须小于1024字节。
    /// <br>做条件查询时，即使marker在列表中不存在，也会从符合marker字母排序的下一个开始打印。
    #[setters(generate = true, strip_option)]
    marker: Option<String>,
    /// 指定返回Object的最大数。 如果因为max-keys的设定无法一次完成列举，返回结果会附加NextMarker元素作为下一次列举的marker。<br>取值：大于0小于1000 <br>默认值：100
    #[setters(generate = true, strip_option)]
    max_keys: Option<i64>,
    /// 限定返回文件的Key必须以prefix作为前缀。
    ///
    /// - prefix参数的长度必须小于1024字节。
    ///
    /// - 使用prefix查询时，返回的Key中仍会包含prefix。
    ///
    /// 如果把prefix设为某个文件夹名，则列举以此Prefix开头的文件，即该文件夹下递归的所有文件和子文件夹。<br>
    /// 在设置prefix的基础上，将delimiter设置为正斜线（/）时，返回值中只列举该文件夹下的文件，文件夹下的子文件夹名返回在CommonPrefixes中，子文件夹下递归的所有文件和文件夹不显示。<br>
    /// 例如，一个Bucket中有三个Object ，分别为fun/test.jpg、 fun/movie/001.avi和fun/movie/007.avi。如果设定prefix为fun/，则返回三个Object；如果在prefix设置为fun/的基础上，将delimiter设置为正斜线（/），则返回fun/test.jpg和fun/movie/。
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 对返回的内容进行编码并指定编码的类型。
    ///
    /// > delimiter、marker、prefix、NextMarker以及Key使用UTF-8字符。如果delimiter、marker、prefix、NextMarker以及Key中包含XML 1.0标准不支持的控制字符，您可以通过指定encoding-type对返回结果中的Delimiter、Marker、Prefix、NextMarker以及Key进行编码。
    #[setters(generate = true, strip_option)]
    encoding_type: Option<String>,
}

impl sealed::Bound for ListObjects {}

impl ListObjects {
    pub fn new() -> Self {
        Self {
            delimiter: None,
            marker: None,
            max_keys: None,
            prefix: None,
            encoding_type: None,
        }
    }
}

impl crate::Request for ListObjects {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListObjects";

    type Body = ();

    type Response = ListObjectsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.delimiter {
            params.insert("delimiter".into(), f.into());
        }

        if let Some(f) = &self.marker {
            params.insert("marker".into(), f.into());
        }

        if let Some(f) = &self.max_keys {
            params.insert("max-keys".into(), f.into());
        }

        if let Some(f) = &self.prefix {
            params.insert("prefix".into(), f.into());
        }

        if let Some(f) = &self.encoding_type {
            params.insert("encoding-type".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListObjectsV2 {
    /// 对Object名字进行分组的字符。所有Object名字包含指定的前缀，第一次出现Delimiter字符之间的Object作为一组元素（即CommonPrefixes）。
    #[setters(generate = true, strip_option)]
    delimiter: Option<String>,
    /// 指定返回Object的最大数。<br>取值：大于0小于1000 <br>默认值：100
    ///
    /// > 如果因为Max-keys的设定无法一次完成列举，返回结果会附加一个<NextContinuationToken>作为下一次列举的Continuation-token。
    #[setters(generate = true, strip_option)]
    max_keys: Option<i64>,
    /// 限定返回文件的Key必须以Prefix作为前缀。<br>
    ///
    /// - 参数的长度必须小于1024字节。
    ///
    /// - 使用Prefix查询时，返回的Key中仍会包含Prefix。
    ///
    /// 如果把Prefix设为某个文件夹名，则列举以此Prefix开头的文件，即该文件夹下递归的所有文件和子文件夹。<br>
    /// 在设置Prefix的基础上，将Delimiter设置为正斜线（/）时，返回值就只列举该文件夹下的文件，文件夹下的子文件夹名返回在CommonPrefixes中，子文件夹下递归的所有文件和文件夹不显示。<br>
    /// 例如，一个Bucket中有三个Object ，分别为fun/test.jpg、 fun/movie/001.avi和fun/movie/007.avi。若设定Prefix为fun/，则返回三个Object；如果在Prefix设置为fun/的基础上，将Delimiter设置为正斜线（/），则返回fun/test.jpg和fun/movie/。
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 对返回的内容进行编码并指定编码的类型。
    ///
    /// >  Delimiter、Start-after、Prefix、NextContinuationToken以及Key使用UTF-8字符。如果Delimiter、Start-after、Prefix、NextContinuationToken以及Key中包含XML 1.0标准不支持的控制字符，您可以通过指定Encoding-type对返回结果中的Delimiter、Start-after、Prefix、NextContinuationToken以及Key进行编码。
    #[setters(generate = true, strip_option)]
    encoding_type: Option<String>,
    /// 指定是否在返回结果中包含owner信息。可选值如下：
    ///
    /// - true：表示返回结果中包含owner信息。
    ///
    /// - false：表示返回结果中不包含owner信息。
    #[setters(generate = true, strip_option)]
    fetch_owner: Option<bool>,
    /// 设定从Start-after之后按字母排序开始返回Object。<br>
    /// Start-after用来实现分页显示效果，参数的长度必须小于1024字节。
    /// <br>做条件查询时，即使Start-after在列表中不存在，也会从符合Start-after字母排序的下一个开始打印。
    #[setters(generate = true, strip_option)]
    start_after: Option<String>,
    /// 指定list操作需要从此token开始。您可从ListObjectsV2结果中的NextContinuationToken获取此token。
    #[setters(generate = true, strip_option)]
    continuation_token: Option<String>,
}

impl sealed::Bound for ListObjectsV2 {}

impl ListObjectsV2 {
    pub fn new() -> Self {
        Self {
            delimiter: None,
            max_keys: None,
            prefix: None,
            encoding_type: None,
            fetch_owner: None,
            start_after: None,
            continuation_token: None,
        }
    }
}

impl crate::Request for ListObjectsV2 {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListObjectsV2";

    type Body = ();

    type Response = ListObjectsV2Response;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.delimiter {
            params.insert("delimiter".into(), f.into());
        }

        if let Some(f) = &self.max_keys {
            params.insert("max-keys".into(), f.into());
        }

        if let Some(f) = &self.prefix {
            params.insert("prefix".into(), f.into());
        }

        if let Some(f) = &self.encoding_type {
            params.insert("encoding-type".into(), f.into());
        }

        if let Some(f) = &self.fetch_owner {
            params.insert("fetch-owner".into(), f.into());
        }

        if let Some(f) = &self.start_after {
            params.insert("start-after".into(), f.into());
        }

        if let Some(f) = &self.continuation_token {
            params.insert("continuation-token".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketInfo {}

impl sealed::Bound for GetBucketInfo {}

impl GetBucketInfo {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketInfo";

    type Body = ();

    type Response = GetBucketInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketLocation {}

impl sealed::Bound for GetBucketLocation {}

impl GetBucketLocation {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketLocation {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketLocation";

    type Body = ();

    type Response = GetBucketLocationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListAccessPoints {
    /// 指定返回接入点的最大数量。取值如下：
    ///
    /// - 获取用户级别接入点时，取值要求大于0小于等于1000。
    ///
    /// - 获取Bucket级别接入点时，取值要求大于0小于等于100。
    #[setters(generate = true, strip_option)]
    max_keys: Option<i64>,
    /// 指定List操作需要从此token开始。您可从返回结果中的NextContinuationToken获取此token。
    #[setters(generate = true, strip_option)]
    continuation_token: Option<String>,
}

impl sealed::Bound for ListAccessPoints {}

impl ListAccessPoints {
    pub fn new() -> Self {
        Self {
            max_keys: None,
            continuation_token: None,
        }
    }
}

impl crate::Request for ListAccessPoints {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListAccessPoints";

    type Body = ();

    type Response = ListAccessPointsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.max_keys {
            params.insert("max-keys".into(), f.into());
        }

        if let Some(f) = &self.continuation_token {
            params.insert("continuation-token".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPoint {}

impl sealed::Bound for GetAccessPoint {}

impl GetAccessPoint {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetAccessPoint {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPoint";

    type Body = ();

    type Response = GetAccessPointResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointPolicy {}

impl sealed::Bound for GetAccessPointPolicy {}

impl GetAccessPointPolicy {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetAccessPointPolicy {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointPolicy";

    type Body = ();

    type Response = GetAccessPointPolicyResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointPolicy {}

impl sealed::Bound for DeleteAccessPointPolicy {}

impl DeleteAccessPointPolicy {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteAccessPointPolicy {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteAccessPointPolicy";

    type Body = ();

    type Response = DeleteAccessPointPolicyResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointPolicy {
    /// 接入点策略配置内容。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for PutAccessPointPolicy {}

impl PutAccessPointPolicy {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutAccessPointPolicy {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutAccessPointPolicy";

    type Body = crate::OctetStream;

    type Response = PutAccessPointPolicyResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPoint {}

impl sealed::Bound for DeleteAccessPoint {}

impl DeleteAccessPoint {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteAccessPoint {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteAccessPoint";

    type Body = ();

    type Response = DeleteAccessPointResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateAccessPoint {}

impl sealed::Bound for CreateAccessPoint {}

impl CreateAccessPoint {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for CreateAccessPoint {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CreateAccessPoint";

    type Body = ();

    type Response = CreateAccessPointResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct InitiateBucketWorm {}

impl sealed::Bound for InitiateBucketWorm {}

impl InitiateBucketWorm {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for InitiateBucketWorm {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for InitiateBucketWorm {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "InitiateBucketWorm";

    type Body = crate::Form<Self>;

    type Response = InitiateBucketWormResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AbortBucketWorm {}

impl sealed::Bound for AbortBucketWorm {}

impl AbortBucketWorm {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for AbortBucketWorm {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "AbortBucketWorm";

    type Body = ();

    type Response = AbortBucketWormResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CompleteBucketWorm {
    /// 合规保留策略的ID。
    worm_id: String,
}

impl sealed::Bound for CompleteBucketWorm {}

impl CompleteBucketWorm {
    pub fn new(worm_id: impl Into<String>) -> Self {
        Self {
            worm_id: worm_id.into(),
        }
    }
}
impl crate::ToFormData for CompleteBucketWorm {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for CompleteBucketWorm {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CompleteBucketWorm";

    type Body = crate::Form<Self>;

    type Response = CompleteBucketWormResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("wormId".into(), (&self.worm_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ExtendBucketWorm {
    /// 合规保留策略的ID。
    ///
    /// > 如果指定用于延长Object保留天数对应的合规保留策略ID不存在，则返回404。
    worm_id: String,
}

impl sealed::Bound for ExtendBucketWorm {}

impl ExtendBucketWorm {
    pub fn new(worm_id: impl Into<String>) -> Self {
        Self {
            worm_id: worm_id.into(),
        }
    }
}
impl crate::ToFormData for ExtendBucketWorm {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for ExtendBucketWorm {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "ExtendBucketWorm";

    type Body = crate::Form<Self>;

    type Response = ExtendBucketWormResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("wormId".into(), (&self.worm_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketWorm {}

impl sealed::Bound for GetBucketWorm {}

impl GetBucketWorm {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketWorm {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketWorm";

    type Body = ();

    type Response = GetBucketWormResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketAcl {}

impl sealed::Bound for PutBucketAcl {}

impl PutBucketAcl {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketAcl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketAcl";

    type Body = ();

    type Response = PutBucketAclResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketAcl {}

impl sealed::Bound for GetBucketAcl {}

impl GetBucketAcl {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketAcl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketAcl";

    type Body = ();

    type Response = GetBucketAclResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketLifecycle {}

impl sealed::Bound for PutBucketLifecycle {}

impl PutBucketLifecycle {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketLifecycle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketLifecycle";

    type Body = ();

    type Response = PutBucketLifecycleResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketLifecycle {}

impl sealed::Bound for GetBucketLifecycle {}

impl GetBucketLifecycle {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketLifecycle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketLifecycle";

    type Body = ();

    type Response = GetBucketLifecycleResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketLifecycle {}

impl sealed::Bound for DeleteBucketLifecycle {}

impl DeleteBucketLifecycle {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketLifecycle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketLifecycle";

    type Body = ();

    type Response = DeleteBucketLifecycleResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketTransferAcceleration {}

impl sealed::Bound for PutBucketTransferAcceleration {}

impl PutBucketTransferAcceleration {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketTransferAcceleration {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketTransferAcceleration";

    type Body = ();

    type Response = PutBucketTransferAccelerationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketTransferAcceleration {}

impl sealed::Bound for GetBucketTransferAcceleration {}

impl GetBucketTransferAcceleration {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketTransferAcceleration {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketTransferAcceleration";

    type Body = ();

    type Response = GetBucketTransferAccelerationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketVersioning {}

impl sealed::Bound for PutBucketVersioning {}

impl PutBucketVersioning {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketVersioning {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketVersioning";

    type Body = ();

    type Response = PutBucketVersioningResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketVersioning {}

impl sealed::Bound for GetBucketVersioning {}

impl GetBucketVersioning {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketVersioning {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketVersioning";

    type Body = ();

    type Response = GetBucketVersioningResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListObjectVersions {
    /// 对Object名字进行分组的字符。所有Object名字包含指定的前缀（prefix），第一次出现delimiter字符之间的Object作为一组元素（即CommonPrefixes）。
    /// 如果将prefix设为文件夹名称后，再把delimiter设置为正斜线（/），则只返回该文件夹下的文件，该文件夹下的子文件名在CommonPrefixes中返回，子文件夹下递归的文件和文件夹不显示。
    ///
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    delimiter: Option<String>,
    /// 设定结果从key-marker之后按字母序开始返回，与version-id-marker组合使用。
    /// 参数的长度必须小于1024字节。
    ///
    /// 默认值：无
    ///
    /// >如果version-id-marker不为空，则key-marker不能为空
    #[setters(generate = true, strip_option)]
    key_marker: Option<String>,
    /// 设定结果从key-marker对象的version-id-marker之后按新旧版本排序开始返回。如果version-id-marker未设定，则默认从key-marker按字母序排序的下一个Key的第一个版本开始返回。
    ///
    /// 默认值：无
    ///
    /// 有效值：版本ID
    #[setters(generate = true, strip_option)]
    version_id_marker: Option<String>,
    /// 限定此次返回Object的最大个数。
    /// 如果因为max-keys的设定无法一次完成列举，返回结果会附加`NextKeyMarker`和`NextVersionIdMarker`作为下一次列举的marker。列举结果中包含`NextKeyMarker`和`NextVersionIdMarker`的值。
    /// 取值：大于0小于1000
    /// 默认值：100
    #[setters(generate = true, strip_option)]
    max_keys: Option<i64>,
    /// 限定返回的Object Key必须以prefix作为前缀。
    ///
    /// - prefix的长度必须小于1024字节。
    /// - 使用prefix查询时，返回结果的Key仍会包含prefix。
    ///
    /// 如果将prefix设为某个文件夹名，则列举以此prefix开头的文件，即该文件夹下递归的所有的文件和子文件夹。
    ///
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 对返回的内容进行编码并指定编码类型。
    /// 默认值：无
    /// 可选值：URL
    /// >delimiter、marker、prefix、NextMarker以及Key使用UTF-8字符。如果delimiter、marker、prefix、NextMarker以及Key中包含XML
    ///                                     1.0标准不支持的控制字符，您可以通过指定encoding-type对返回结果中的Delimiter、Marker、Prefix、NextMarker以及Key进行编码。
    #[setters(generate = true, strip_option)]
    encoding_type: Option<String>,
}

impl sealed::Bound for ListObjectVersions {}

impl ListObjectVersions {
    pub fn new() -> Self {
        Self {
            delimiter: None,
            key_marker: None,
            version_id_marker: None,
            max_keys: None,
            prefix: None,
            encoding_type: None,
        }
    }
}

impl crate::Request for ListObjectVersions {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListObjectVersions";

    type Body = ();

    type Response = ListObjectVersionsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.delimiter {
            params.insert("delimiter".into(), f.into());
        }

        if let Some(f) = &self.key_marker {
            params.insert("key-marker".into(), f.into());
        }

        if let Some(f) = &self.version_id_marker {
            params.insert("version-id-marker".into(), f.into());
        }

        if let Some(f) = &self.max_keys {
            params.insert("max-keys".into(), f.into());
        }

        if let Some(f) = &self.prefix {
            params.insert("prefix".into(), f.into());
        }

        if let Some(f) = &self.encoding_type {
            params.insert("encoding-type".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketPolicy {
    /// 请求体。
    body: Vec<u8>,
}

impl sealed::Bound for PutBucketPolicy {}

impl PutBucketPolicy {
    pub fn new(body: impl Into<Vec<u8>>) -> Self {
        Self { body: body.into() }
    }
}

impl crate::Request for PutBucketPolicy {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutBucketPolicy";

    type Body = crate::OctetStream;

    type Response = PutBucketPolicyResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketPolicy {}

impl sealed::Bound for GetBucketPolicy {}

impl GetBucketPolicy {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketPolicy {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketPolicy";

    type Body = ();

    type Response = GetBucketPolicyResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketPolicy {}

impl sealed::Bound for DeleteBucketPolicy {}

impl DeleteBucketPolicy {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketPolicy {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketPolicy";

    type Body = ();

    type Response = DeleteBucketPolicyResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketPolicyStatus {}

impl sealed::Bound for GetBucketPolicyStatus {}

impl GetBucketPolicyStatus {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketPolicyStatus {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketPolicyStatus";

    type Body = ();

    type Response = GetBucketPolicyStatusResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketRtc {}

impl sealed::Bound for PutBucketRtc {}

impl PutBucketRtc {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketRtc {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketRtc";

    type Body = ();

    type Response = PutBucketRtcResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketReplication {}

impl sealed::Bound for PutBucketReplication {}

impl PutBucketReplication {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for PutBucketReplication {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for PutBucketReplication {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutBucketReplication";

    type Body = crate::Form<Self>;

    type Response = PutBucketReplicationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketReplication {}

impl sealed::Bound for GetBucketReplication {}

impl GetBucketReplication {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketReplication {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketReplication";

    type Body = ();

    type Response = GetBucketReplicationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketReplicationLocation {}

impl sealed::Bound for GetBucketReplicationLocation {}

impl GetBucketReplicationLocation {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketReplicationLocation {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketReplicationLocation";

    type Body = ();

    type Response = GetBucketReplicationLocationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketReplicationProgress {
    /// 复制规则对应的ID。此ID可从GetBucketReplication中获取。
    rule_id: String,
}

impl sealed::Bound for GetBucketReplicationProgress {}

impl GetBucketReplicationProgress {
    pub fn new(rule_id: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
        }
    }
}

impl crate::Request for GetBucketReplicationProgress {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketReplicationProgress";

    type Body = ();

    type Response = GetBucketReplicationProgressResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("rule-id".into(), (&self.rule_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketReplication {}

impl sealed::Bound for DeleteBucketReplication {}

impl DeleteBucketReplication {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for DeleteBucketReplication {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for DeleteBucketReplication {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DeleteBucketReplication";

    type Body = crate::Form<Self>;

    type Response = DeleteBucketReplicationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketInventory {
    /// 配置的清单规则Id。
    inventory_id: String,
}

impl sealed::Bound for PutBucketInventory {}

impl PutBucketInventory {
    pub fn new(inventory_id: impl Into<String>) -> Self {
        Self {
            inventory_id: inventory_id.into(),
        }
    }
}

impl crate::Request for PutBucketInventory {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketInventory";

    type Body = ();

    type Response = PutBucketInventoryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("inventoryId".into(), (&self.inventory_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketInventory {
    /// 查询的清单规则Id。
    inventory_id: String,
}

impl sealed::Bound for GetBucketInventory {}

impl GetBucketInventory {
    pub fn new(inventory_id: impl Into<String>) -> Self {
        Self {
            inventory_id: inventory_id.into(),
        }
    }
}

impl crate::Request for GetBucketInventory {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketInventory";

    type Body = ();

    type Response = GetBucketInventoryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("inventoryId".into(), (&self.inventory_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListBucketInventory {
    /// 指定List操作需要从此token开始。您可从ListBucketInventory结果中的NextContinuationToken获取此token。
    #[setters(generate = true, strip_option)]
    continuation_token: Option<String>,
}

impl sealed::Bound for ListBucketInventory {}

impl ListBucketInventory {
    pub fn new() -> Self {
        Self {
            continuation_token: None,
        }
    }
}

impl crate::Request for ListBucketInventory {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListBucketInventory";

    type Body = ();

    type Response = ListBucketInventoryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.continuation_token {
            params.insert("continuation-token".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketInventory {
    /// 删除的清单任务Id。
    inventory_id: String,
}

impl sealed::Bound for DeleteBucketInventory {}

impl DeleteBucketInventory {
    pub fn new(inventory_id: impl Into<String>) -> Self {
        Self {
            inventory_id: inventory_id.into(),
        }
    }
}

impl crate::Request for DeleteBucketInventory {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketInventory";

    type Body = ();

    type Response = DeleteBucketInventoryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("inventoryId".into(), (&self.inventory_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketLogging {}

impl sealed::Bound for PutBucketLogging {}

impl PutBucketLogging {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketLogging {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketLogging";

    type Body = ();

    type Response = PutBucketLoggingResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketLogging {}

impl sealed::Bound for GetBucketLogging {}

impl GetBucketLogging {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketLogging {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketLogging";

    type Body = ();

    type Response = GetBucketLoggingResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketLogging {}

impl sealed::Bound for DeleteBucketLogging {}

impl DeleteBucketLogging {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketLogging {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketLogging";

    type Body = ();

    type Response = DeleteBucketLoggingResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutUserDefinedLogFieldsConfig {}

impl sealed::Bound for PutUserDefinedLogFieldsConfig {}

impl PutUserDefinedLogFieldsConfig {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutUserDefinedLogFieldsConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutUserDefinedLogFieldsConfig";

    type Body = ();

    type Response = PutUserDefinedLogFieldsConfigResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetUserDefinedLogFieldsConfig {}

impl sealed::Bound for GetUserDefinedLogFieldsConfig {}

impl GetUserDefinedLogFieldsConfig {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetUserDefinedLogFieldsConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetUserDefinedLogFieldsConfig";

    type Body = ();

    type Response = GetUserDefinedLogFieldsConfigResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteUserDefinedLogFieldsConfig {}

impl sealed::Bound for DeleteUserDefinedLogFieldsConfig {}

impl DeleteUserDefinedLogFieldsConfig {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteUserDefinedLogFieldsConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteUserDefinedLogFieldsConfig";

    type Body = ();

    type Response = DeleteUserDefinedLogFieldsConfigResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketWebsite {}

impl sealed::Bound for GetBucketWebsite {}

impl GetBucketWebsite {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketWebsite {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketWebsite";

    type Body = ();

    type Response = GetBucketWebsiteResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketWebsite {}

impl sealed::Bound for PutBucketWebsite {}

impl PutBucketWebsite {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketWebsite {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketWebsite";

    type Body = ();

    type Response = PutBucketWebsiteResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketWebsite {}

impl sealed::Bound for DeleteBucketWebsite {}

impl DeleteBucketWebsite {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketWebsite {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketWebsite";

    type Body = ();

    type Response = DeleteBucketWebsiteResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketReferer {}

impl sealed::Bound for PutBucketReferer {}

impl PutBucketReferer {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketReferer {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketReferer";

    type Body = ();

    type Response = PutBucketRefererResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketReferer {}

impl sealed::Bound for GetBucketReferer {}

impl GetBucketReferer {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketReferer {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketReferer";

    type Body = ();

    type Response = GetBucketRefererResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketTags {}

impl sealed::Bound for PutBucketTags {}

impl PutBucketTags {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketTags {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketTags";

    type Body = ();

    type Response = PutBucketTagsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketTags {}

impl sealed::Bound for GetBucketTags {}

impl GetBucketTags {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketTags {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketTags";

    type Body = ();

    type Response = GetBucketTagsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketTags {}

impl sealed::Bound for DeleteBucketTags {}

impl DeleteBucketTags {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketTags {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketTags";

    type Body = ();

    type Response = DeleteBucketTagsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListUserDataRedundancyTransition {
    /// 指定List操作需要从此token开始。
    #[setters(generate = true, strip_option)]
    continuation_token: Option<String>,
    /// 限定此次返回任务的最大个数。取值范围：1-100。
    #[setters(generate = true, strip_option)]
    max_keys: Option<i32>,
}

impl sealed::Bound for ListUserDataRedundancyTransition {}

impl ListUserDataRedundancyTransition {
    pub fn new() -> Self {
        Self {
            continuation_token: None,
            max_keys: None,
        }
    }
}

impl crate::Request for ListUserDataRedundancyTransition {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListUserDataRedundancyTransition";

    type Body = ();

    type Response = ListUserDataRedundancyTransitionResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.continuation_token {
            params.insert("continuation-token".into(), f.into());
        }

        if let Some(f) = &self.max_keys {
            params.insert("max-keys".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListBucketDataRedundancyTransition {}

impl sealed::Bound for ListBucketDataRedundancyTransition {}

impl ListBucketDataRedundancyTransition {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for ListBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListBucketDataRedundancyTransition";

    type Body = ();

    type Response = ListBucketDataRedundancyTransitionResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketDataRedundancyTransition {
    /// 存储转换任务的ID。
    x_oss_redundancy_transition_taskid: String,
}

impl sealed::Bound for GetBucketDataRedundancyTransition {}

impl GetBucketDataRedundancyTransition {
    pub fn new(x_oss_redundancy_transition_taskid: impl Into<String>) -> Self {
        Self {
            x_oss_redundancy_transition_taskid: x_oss_redundancy_transition_taskid.into(),
        }
    }
}

impl crate::Request for GetBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketDataRedundancyTransition";

    type Body = ();

    type Response = GetBucketDataRedundancyTransitionResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "x-oss-redundancy-transition-taskid".into(),
            (&self.x_oss_redundancy_transition_taskid).into(),
        );

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateBucketDataRedundancyTransition {
    /// 目标存储冗余类型。OSS仅支持将LRS（本地冗余存储）转换为ZRS（同城冗余存储）。
    x_oss_target_redundancy_type: String,
}

impl sealed::Bound for CreateBucketDataRedundancyTransition {}

impl CreateBucketDataRedundancyTransition {
    pub fn new(x_oss_target_redundancy_type: impl Into<String>) -> Self {
        Self {
            x_oss_target_redundancy_type: x_oss_target_redundancy_type.into(),
        }
    }
}
impl crate::ToFormData for CreateBucketDataRedundancyTransition {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for CreateBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateBucketDataRedundancyTransition";

    type Body = crate::Form<Self>;

    type Response = CreateBucketDataRedundancyTransitionResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "x-oss-target-redundancy-type".into(),
            (&self.x_oss_target_redundancy_type).into(),
        );

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketDataRedundancyTransition {
    /// 存储冗余转换任务的ID。
    x_oss_redundancy_transition_taskid: String,
}

impl sealed::Bound for DeleteBucketDataRedundancyTransition {}

impl DeleteBucketDataRedundancyTransition {
    pub fn new(x_oss_redundancy_transition_taskid: impl Into<String>) -> Self {
        Self {
            x_oss_redundancy_transition_taskid: x_oss_redundancy_transition_taskid.into(),
        }
    }
}

impl crate::Request for DeleteBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketDataRedundancyTransition";

    type Body = ();

    type Response = DeleteBucketDataRedundancyTransitionResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "x-oss-redundancy-transition-taskid".into(),
            (&self.x_oss_redundancy_transition_taskid).into(),
        );

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketEncryption {}

impl sealed::Bound for PutBucketEncryption {}

impl PutBucketEncryption {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketEncryption {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketEncryption";

    type Body = ();

    type Response = PutBucketEncryptionResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketEncryption {}

impl sealed::Bound for GetBucketEncryption {}

impl GetBucketEncryption {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketEncryption {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketEncryption";

    type Body = ();

    type Response = GetBucketEncryptionResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketEncryption {}

impl sealed::Bound for DeleteBucketEncryption {}

impl DeleteBucketEncryption {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketEncryption {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketEncryption";

    type Body = ();

    type Response = DeleteBucketEncryptionResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketRequestPayment {}

impl sealed::Bound for PutBucketRequestPayment {}

impl PutBucketRequestPayment {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketRequestPayment {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketRequestPayment";

    type Body = ();

    type Response = PutBucketRequestPaymentResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketRequestPayment {}

impl sealed::Bound for GetBucketRequestPayment {}

impl GetBucketRequestPayment {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketRequestPayment {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketRequestPayment";

    type Body = ();

    type Response = GetBucketRequestPaymentResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketCors {}

impl sealed::Bound for PutBucketCors {}

impl PutBucketCors {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketCors {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketCors";

    type Body = ();

    type Response = PutBucketCorsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketCors {}

impl sealed::Bound for GetBucketCors {}

impl GetBucketCors {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketCors {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketCors";

    type Body = ();

    type Response = GetBucketCorsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketCors {}

impl sealed::Bound for DeleteBucketCors {}

impl DeleteBucketCors {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketCors {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketCors";

    type Body = ();

    type Response = DeleteBucketCorsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct OptionObject {}

impl sealed::Bound for OptionObject {}

impl OptionObject {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for OptionObject {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "OptionObject";

    type Body = ();

    type Response = OptionObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketAccessMonitor {}

impl sealed::Bound for PutBucketAccessMonitor {}

impl PutBucketAccessMonitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketAccessMonitor {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketAccessMonitor";

    type Body = ();

    type Response = PutBucketAccessMonitorResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketAccessMonitor {}

impl sealed::Bound for GetBucketAccessMonitor {}

impl GetBucketAccessMonitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketAccessMonitor {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketAccessMonitor";

    type Body = ();

    type Response = GetBucketAccessMonitorResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetMetaQueryStatus {}

impl sealed::Bound for GetMetaQueryStatus {}

impl GetMetaQueryStatus {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetMetaQueryStatus {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetMetaQueryStatus";

    type Body = ();

    type Response = GetMetaQueryStatusResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CloseMetaQuery {}

impl sealed::Bound for CloseMetaQuery {}

impl CloseMetaQuery {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for CloseMetaQuery {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for CloseMetaQuery {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CloseMetaQuery";

    type Body = crate::Form<Self>;

    type Response = CloseMetaQueryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DoMetaQuery {
    /// 指定检索模式。
    /// - basic：标量检索（默认）
    /// - semantic：向量检索
    #[setters(generate = true, strip_option)]
    mode: Option<String>,
}

impl sealed::Bound for DoMetaQuery {}

impl DoMetaQuery {
    pub fn new() -> Self {
        Self { mode: None }
    }
}
impl crate::ToFormData for DoMetaQuery {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for DoMetaQuery {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DoMetaQuery";

    type Body = crate::Form<Self>;

    type Response = DoMetaQueryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.mode {
            params.insert("mode".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct OpenMetaQuery {
    /// 检索模式。取值如下：
    ///
    /// basic（默认值）：标量检索
    ///
    /// semantic：向量检索
    #[setters(generate = true, strip_option)]
    mode: Option<String>,
    /// 指定用于访问 OSS 服务的 RAM 角色名称，支持在控制台为角色授予权限，实现安全访问。
    #[setters(generate = true, strip_option)]
    role: Option<String>,
}

impl sealed::Bound for OpenMetaQuery {}

impl OpenMetaQuery {
    pub fn new() -> Self {
        Self {
            mode: None,
            role: None,
        }
    }
}
impl crate::ToFormData for OpenMetaQuery {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for OpenMetaQuery {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "OpenMetaQuery";

    type Body = crate::Form<Self>;

    type Response = OpenMetaQueryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.mode {
            params.insert("mode".into(), f.into());
        }

        if let Some(f) = &self.role {
            params.insert("role".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateUserAntiDDosInfo {}

impl sealed::Bound for UpdateUserAntiDDosInfo {}

impl UpdateUserAntiDDosInfo {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for UpdateUserAntiDDosInfo {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for UpdateUserAntiDDosInfo {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UpdateUserAntiDDosInfo";

    type Body = crate::Form<Self>;

    type Response = UpdateUserAntiDDosInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateBucketAntiDDosInfo {}

impl sealed::Bound for UpdateBucketAntiDDosInfo {}

impl UpdateBucketAntiDDosInfo {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for UpdateBucketAntiDDosInfo {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for UpdateBucketAntiDDosInfo {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UpdateBucketAntiDDosInfo";

    type Body = crate::Form<Self>;

    type Response = UpdateBucketAntiDDosInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListBucketAntiDDosInfo {
    /// 设定从marker之后按字母排序开始返回高防实例。
    ///
    /// > 第一次请求时可以将marker设置为空字符串。如果返回结果中IsTruncated存在且为true，则需要将返回结果中的marker字段作为参数进行下一次请求。
    #[setters(generate = true, strip_option)]
    marker: Option<String>,
    /// 指定返回的高防实例最大数量。
    ///
    /// 取值：[1,100]
    ///
    /// 默认值：100
    #[setters(generate = true, strip_option)]
    max_keys: Option<String>,
}

impl sealed::Bound for ListBucketAntiDDosInfo {}

impl ListBucketAntiDDosInfo {
    pub fn new() -> Self {
        Self {
            marker: None,
            max_keys: None,
        }
    }
}

impl crate::Request for ListBucketAntiDDosInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListBucketAntiDDosInfo";

    type Body = ();

    type Response = ListBucketAntiDDosInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.marker {
            params.insert("marker".into(), f.into());
        }

        if let Some(f) = &self.max_keys {
            params.insert("max-keys".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct InitUserAntiDDosInfo {}

impl sealed::Bound for InitUserAntiDDosInfo {}

impl InitUserAntiDDosInfo {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for InitUserAntiDDosInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "InitUserAntiDDosInfo";

    type Body = ();

    type Response = InitUserAntiDDosInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct InitBucketAntiDDosInfo {}

impl sealed::Bound for InitBucketAntiDDosInfo {}

impl InitBucketAntiDDosInfo {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for InitBucketAntiDDosInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "InitBucketAntiDDosInfo";

    type Body = ();

    type Response = InitBucketAntiDDosInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetUserAntiDDosInfo {}

impl sealed::Bound for GetUserAntiDDosInfo {}

impl GetUserAntiDDosInfo {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetUserAntiDDosInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetUserAntiDDosInfo";

    type Body = ();

    type Response = GetUserAntiDDosInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketResourceGroup {}

impl sealed::Bound for GetBucketResourceGroup {}

impl GetBucketResourceGroup {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketResourceGroup {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketResourceGroup";

    type Body = ();

    type Response = GetBucketResourceGroupResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketResourceGroup {}

impl sealed::Bound for PutBucketResourceGroup {}

impl PutBucketResourceGroup {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketResourceGroup {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketResourceGroup";

    type Body = ();

    type Response = PutBucketResourceGroupResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutCname {}

impl sealed::Bound for PutCname {}

impl PutCname {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for PutCname {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for PutCname {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutCname";

    type Body = crate::Form<Self>;

    type Response = PutCnameResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListCname {}

impl sealed::Bound for ListCname {}

impl ListCname {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for ListCname {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListCname";

    type Body = ();

    type Response = ListCnameResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteCname {}

impl sealed::Bound for DeleteCname {}

impl DeleteCname {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for DeleteCname {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for DeleteCname {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DeleteCname";

    type Body = crate::Form<Self>;

    type Response = DeleteCnameResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetCnameToken {
    /// 绑定的Cname名称。
    cname: String,
}

impl sealed::Bound for GetCnameToken {}

impl GetCnameToken {
    pub fn new(cname: impl Into<String>) -> Self {
        Self {
            cname: cname.into(),
        }
    }
}

impl crate::Request for GetCnameToken {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetCnameToken";

    type Body = ();

    type Response = GetCnameTokenResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("cname".into(), (&self.cname).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateCnameToken {}

impl sealed::Bound for CreateCnameToken {}

impl CreateCnameToken {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for CreateCnameToken {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for CreateCnameToken {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateCnameToken";

    type Body = crate::Form<Self>;

    type Response = CreateCnameTokenResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutStyle {
    /// 图片样式名称。
    style_name: String,
    /// 样式分类。
    ///
    /// 取值：image、document、video。
    #[setters(generate = true, strip_option)]
    category: Option<String>,
}

impl sealed::Bound for PutStyle {}

impl PutStyle {
    pub fn new(style_name: impl Into<String>) -> Self {
        Self {
            style_name: style_name.into(),
            category: None,
        }
    }
}

impl crate::Request for PutStyle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutStyle";

    type Body = ();

    type Response = PutStyleResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("styleName".into(), (&self.style_name).into());

        if let Some(f) = &self.category {
            params.insert("category".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListStyle {}

impl sealed::Bound for ListStyle {}

impl ListStyle {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for ListStyle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListStyle";

    type Body = ();

    type Response = ListStyleResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetStyle {
    /// 图片样式名称。
    style_name: String,
}

impl sealed::Bound for GetStyle {}

impl GetStyle {
    pub fn new(style_name: impl Into<String>) -> Self {
        Self {
            style_name: style_name.into(),
        }
    }
}

impl crate::Request for GetStyle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetStyle";

    type Body = ();

    type Response = GetStyleResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("styleName".into(), (&self.style_name).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteStyle {
    /// 图片样式名称。
    style_name: String,
}

impl sealed::Bound for DeleteStyle {}

impl DeleteStyle {
    pub fn new(style_name: impl Into<String>) -> Self {
        Self {
            style_name: style_name.into(),
        }
    }
}

impl crate::Request for DeleteStyle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteStyle";

    type Body = ();

    type Response = DeleteStyleResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("styleName".into(), (&self.style_name).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketHttpsConfig {}

impl sealed::Bound for GetBucketHttpsConfig {}

impl GetBucketHttpsConfig {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketHttpsConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketHttpsConfig";

    type Body = ();

    type Response = GetBucketHttpsConfigResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketHttpsConfig {}

impl sealed::Bound for PutBucketHttpsConfig {}

impl PutBucketHttpsConfig {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketHttpsConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketHttpsConfig";

    type Body = ();

    type Response = PutBucketHttpsConfigResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateAccessPointForObjectProcess {}

impl sealed::Bound for CreateAccessPointForObjectProcess {}

impl CreateAccessPointForObjectProcess {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for CreateAccessPointForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CreateAccessPointForObjectProcess";

    type Body = ();

    type Response = CreateAccessPointForObjectProcessResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointForObjectProcess {}

impl sealed::Bound for GetAccessPointForObjectProcess {}

impl GetAccessPointForObjectProcess {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetAccessPointForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointForObjectProcess";

    type Body = ();

    type Response = GetAccessPointForObjectProcessResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListAccessPointsForObjectProcess {
    /// 指定返回对象FC接入点的最大数量。
    ///
    /// 取值：大于0小于等于1000。
    ///
    /// > 如果因为max-keys的设置无法一次完成列举，返回结果会附加一个`<NextContinuationToken>`作为下一次列举的continuation-token。
    #[setters(generate = true, strip_option)]
    max_keys: Option<i64>,
    /// 指定List操作需要从此token开始。您可以从返回结果中的NextContinuationToken获取此token。
    #[setters(generate = true, strip_option)]
    continuation_token: Option<String>,
}

impl sealed::Bound for ListAccessPointsForObjectProcess {}

impl ListAccessPointsForObjectProcess {
    pub fn new() -> Self {
        Self {
            max_keys: None,
            continuation_token: None,
        }
    }
}

impl crate::Request for ListAccessPointsForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListAccessPointsForObjectProcess";

    type Body = ();

    type Response = ListAccessPointsForObjectProcessResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.max_keys {
            params.insert("max-keys".into(), f.into());
        }

        if let Some(f) = &self.continuation_token {
            params.insert("continuation-token".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointForObjectProcess {}

impl sealed::Bound for DeleteAccessPointForObjectProcess {}

impl DeleteAccessPointForObjectProcess {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteAccessPointForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteAccessPointForObjectProcess";

    type Body = ();

    type Response = DeleteAccessPointForObjectProcessResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointConfigForObjectProcess {}

impl sealed::Bound for GetAccessPointConfigForObjectProcess {}

impl GetAccessPointConfigForObjectProcess {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetAccessPointConfigForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointConfigForObjectProcess";

    type Body = ();

    type Response = GetAccessPointConfigForObjectProcessResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointConfigForObjectProcess {}

impl sealed::Bound for PutAccessPointConfigForObjectProcess {}

impl PutAccessPointConfigForObjectProcess {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutAccessPointConfigForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutAccessPointConfigForObjectProcess";

    type Body = ();

    type Response = PutAccessPointConfigForObjectProcessResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointPolicyForObjectProcess {
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for PutAccessPointPolicyForObjectProcess {}

impl PutAccessPointPolicyForObjectProcess {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutAccessPointPolicyForObjectProcess {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutAccessPointPolicyForObjectProcess";

    type Body = crate::OctetStream;

    type Response = PutAccessPointPolicyForObjectProcessResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointPolicyForObjectProcess {}

impl sealed::Bound for GetAccessPointPolicyForObjectProcess {}

impl GetAccessPointPolicyForObjectProcess {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetAccessPointPolicyForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointPolicyForObjectProcess";

    type Body = ();

    type Response = GetAccessPointPolicyForObjectProcessResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointPolicyForObjectProcess {}

impl sealed::Bound for DeleteAccessPointPolicyForObjectProcess {}

impl DeleteAccessPointPolicyForObjectProcess {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteAccessPointPolicyForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteAccessPointPolicyForObjectProcess";

    type Body = ();

    type Response = DeleteAccessPointPolicyForObjectProcessResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetPublicAccessBlock {}

impl sealed::Bound for GetPublicAccessBlock {}

impl GetPublicAccessBlock {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetPublicAccessBlock";

    type Body = ();

    type Response = GetPublicAccessBlockResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutPublicAccessBlock {}

impl sealed::Bound for PutPublicAccessBlock {}

impl PutPublicAccessBlock {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutPublicAccessBlock";

    type Body = ();

    type Response = PutPublicAccessBlockResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeletePublicAccessBlock {}

impl sealed::Bound for DeletePublicAccessBlock {}

impl DeletePublicAccessBlock {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeletePublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeletePublicAccessBlock";

    type Body = ();

    type Response = DeletePublicAccessBlockResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketPublicAccessBlock {}

impl sealed::Bound for GetBucketPublicAccessBlock {}

impl GetBucketPublicAccessBlock {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketPublicAccessBlock";

    type Body = ();

    type Response = GetBucketPublicAccessBlockResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketPublicAccessBlock {}

impl sealed::Bound for PutBucketPublicAccessBlock {}

impl PutBucketPublicAccessBlock {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketPublicAccessBlock";

    type Body = ();

    type Response = PutBucketPublicAccessBlockResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketPublicAccessBlock {}

impl sealed::Bound for DeleteBucketPublicAccessBlock {}

impl DeleteBucketPublicAccessBlock {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketPublicAccessBlock";

    type Body = ();

    type Response = DeleteBucketPublicAccessBlockResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointPublicAccessBlock {
    /// 接入点名称。
    #[setters(generate = true, strip_option)]
    x_oss_access_point_name: Option<String>,
}

impl sealed::Bound for GetAccessPointPublicAccessBlock {}

impl GetAccessPointPublicAccessBlock {
    pub fn new() -> Self {
        Self {
            x_oss_access_point_name: None,
        }
    }
}

impl crate::Request for GetAccessPointPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointPublicAccessBlock";

    type Body = ();

    type Response = GetAccessPointPublicAccessBlockResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.x_oss_access_point_name {
            params.insert("x-oss-access-point-name".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointPublicAccessBlock {
    /// 接入点名称。
    x_oss_access_point_name: String,
}

impl sealed::Bound for PutAccessPointPublicAccessBlock {}

impl PutAccessPointPublicAccessBlock {
    pub fn new(x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_name: x_oss_access_point_name.into(),
        }
    }
}

impl crate::Request for PutAccessPointPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutAccessPointPublicAccessBlock";

    type Body = ();

    type Response = PutAccessPointPublicAccessBlockResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "x-oss-access-point-name".into(),
            (&self.x_oss_access_point_name).into(),
        );

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointPublicAccessBlock {
    /// 接入点名称。
    #[setters(generate = true, strip_option)]
    x_oss_access_point_name: Option<String>,
}

impl sealed::Bound for DeleteAccessPointPublicAccessBlock {}

impl DeleteAccessPointPublicAccessBlock {
    pub fn new() -> Self {
        Self {
            x_oss_access_point_name: None,
        }
    }
}

impl crate::Request for DeleteAccessPointPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteAccessPointPublicAccessBlock";

    type Body = ();

    type Response = DeleteAccessPointPublicAccessBlockResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.x_oss_access_point_name {
            params.insert("x-oss-access-point-name".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketArchiveDirectRead {}

impl sealed::Bound for GetBucketArchiveDirectRead {}

impl GetBucketArchiveDirectRead {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketArchiveDirectRead {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketArchiveDirectRead";

    type Body = ();

    type Response = GetBucketArchiveDirectReadResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketArchiveDirectRead {}

impl sealed::Bound for PutBucketArchiveDirectRead {}

impl PutBucketArchiveDirectRead {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketArchiveDirectRead {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketArchiveDirectRead";

    type Body = ();

    type Response = PutBucketArchiveDirectReadResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketOverwriteConfig {}

impl sealed::Bound for PutBucketOverwriteConfig {}

impl PutBucketOverwriteConfig {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketOverwriteConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketOverwriteConfig";

    type Body = ();

    type Response = PutBucketOverwriteConfigResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketOverwriteConfig {}

impl sealed::Bound for GetBucketOverwriteConfig {}

impl GetBucketOverwriteConfig {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketOverwriteConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketOverwriteConfig";

    type Body = ();

    type Response = GetBucketOverwriteConfigResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketOverwriteConfig {}

impl sealed::Bound for DeleteBucketOverwriteConfig {}

impl DeleteBucketOverwriteConfig {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketOverwriteConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketOverwriteConfig";

    type Body = ();

    type Response = DeleteBucketOverwriteConfigResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutObject {
    /// 请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for PutObject {}

impl PutObject {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutObject";

    type Body = crate::OctetStream;

    type Response = PutObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CopyObject {}

impl sealed::Bound for CopyObject {}

impl CopyObject {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for CopyObject {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CopyObject";

    type Body = ();

    type Response = CopyObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetObject {
    /// 指定OSS返回请求的content-type头。
    #[setters(generate = true, strip_option)]
    response_content_type: Option<String>,
    /// 指定OSS返回请求的content-language头。
    #[setters(generate = true, strip_option)]
    response_content_language: Option<String>,
    /// 指定OSS返回请求的expires头。
    #[setters(generate = true, strip_option)]
    response_expires: Option<String>,
    /// 指定OSS返回请求的cache-control头。
    #[setters(generate = true, strip_option)]
    response_cache_control: Option<String>,
    /// 指定OSS返回请求的content-disposition头。
    #[setters(generate = true, strip_option)]
    response_content_disposition: Option<String>,
    /// 指定OSS返回请求的content-encoding头。
    #[setters(generate = true, strip_option)]
    response_content_encoding: Option<String>,
    /// 目标文件的版本ID。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObject {}

impl GetObject {
    pub fn new() -> Self {
        Self {
            response_content_type: None,
            response_content_language: None,
            response_expires: None,
            response_cache_control: None,
            response_content_disposition: None,
            response_content_encoding: None,
            version_id: None,
        }
    }
}

impl crate::Request for GetObject {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObject";

    type Body = ();

    type Response = GetObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.response_content_type {
            params.insert("response-content-type".into(), f.into());
        }

        if let Some(f) = &self.response_content_language {
            params.insert("response-content-language".into(), f.into());
        }

        if let Some(f) = &self.response_expires {
            params.insert("response-expires".into(), f.into());
        }

        if let Some(f) = &self.response_cache_control {
            params.insert("response-cache-control".into(), f.into());
        }

        if let Some(f) = &self.response_content_disposition {
            params.insert("response-content-disposition".into(), f.into());
        }

        if let Some(f) = &self.response_content_encoding {
            params.insert("response-content-encoding".into(), f.into());
        }

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AppendObject {
    /// 用于指定从何处进行追加。 每次操作成功后，响应消息头x-oss-next-append-position会标明下一次追加的position。首次追加操作的position必须为0，后续追加操作的position是Object的当前大小。例如，第一次AppendObject请求指定position值为0，content-length是65536，则第二次AppendObject需要指定position为65536。
    ///
    /// - 当position值为0，且不存在同名Object时，则AppendObject与PutObject请求类似，即允许设置x-oss-server-side-encryption等请求头。如果加入了正确的x-oss-server-side-encryption头，那么后续的AppendObject响应头部也会包含x-oss-server-side-encryption头。后续如需更改元数据，可以使用CopyObject接口。
    /// - 在position值正确的情况下，对已存在的Appendable Object追加一个大小为0的内容，该操作不会改变Object的状态。
    position: i64,
    /// 请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for AppendObject {}

impl AppendObject {
    pub fn new(position: impl Into<i64>) -> Self {
        Self {
            position: position.into(),
            body: None,
        }
    }
}

impl crate::Request for AppendObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "AppendObject";

    type Body = crate::OctetStream;

    type Response = AppendObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("position".into(), (&self.position).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SealAppendObject {
    /// 用于指定从用户期望Seal时该文件的长度。
    position: i64,
}

impl sealed::Bound for SealAppendObject {}

impl SealAppendObject {
    pub fn new(position: impl Into<i64>) -> Self {
        Self {
            position: position.into(),
        }
    }
}
impl crate::ToFormData for SealAppendObject {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for SealAppendObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SealAppendObject";

    type Body = crate::Form<Self>;

    type Response = SealAppendObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("position".into(), (&self.position).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteObject {
    /// Object对应的版本ID。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for DeleteObject {}

impl DeleteObject {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}

impl crate::Request for DeleteObject {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteObject";

    type Body = ();

    type Response = DeleteObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct HeadObject {
    /// 请求Object的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for HeadObject {}

impl HeadObject {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}

impl crate::Request for HeadObject {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "HeadObject";

    type Body = ();

    type Response = HeadObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetObjectMeta {
    /// Object的版本ID。只有查看Object指定版本的元数据信息时才显示该字段。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObjectMeta {}

impl GetObjectMeta {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}

impl crate::Request for GetObjectMeta {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObjectMeta";

    type Body = ();

    type Response = GetObjectMetaResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RestoreObject {
    /// 请求解冻的Obejct的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for RestoreObject {}

impl RestoreObject {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}
impl crate::ToFormData for RestoreObject {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for RestoreObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RestoreObject";

    type Body = crate::Form<Self>;

    type Response = RestoreObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CleanRestoredObject {}

impl sealed::Bound for CleanRestoredObject {}

impl CleanRestoredObject {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for CleanRestoredObject {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for CleanRestoredObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CleanRestoredObject";

    type Body = crate::Form<Self>;

    type Response = CleanRestoredObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SelectObject {
    /// 如果是csv文件，该值需要设置为csv/select；如果是json文件，则需要设置为json/select。
    x_oss_process: String,
}

impl sealed::Bound for SelectObject {}

impl SelectObject {
    pub fn new(x_oss_process: impl Into<String>) -> Self {
        Self {
            x_oss_process: x_oss_process.into(),
        }
    }
}
impl crate::ToFormData for SelectObject {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for SelectObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SelectObject";

    type Body = crate::Form<Self>;

    type Response = SelectObjectResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("x-oss-process".into(), (&self.x_oss_process).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateSelectObjectMeta {
    /// 如果是csv文件，该值需要设置为csv/meta；如果是json文件，则需要设置为json/meta。
    x_oss_process: String,
}

impl sealed::Bound for CreateSelectObjectMeta {}

impl CreateSelectObjectMeta {
    pub fn new(x_oss_process: impl Into<String>) -> Self {
        Self {
            x_oss_process: x_oss_process.into(),
        }
    }
}
impl crate::ToFormData for CreateSelectObjectMeta {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for CreateSelectObjectMeta {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateSelectObjectMeta";

    type Body = crate::Form<Self>;

    type Response = CreateSelectObjectMetaResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("x-oss-process".into(), (&self.x_oss_process).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct InitiateMultipartUpload {
    /// 指定对返回的Key进行编码，目前支持URL编码。Key使用UTF-8字符，但XML 1.0标准不支持解析一些控制字符，例如ASCII值从0到10的字符。对于Key中包含XML
    ///                               1.0标准不支持的控制字符，可以通过指定encoding-type对返回的Key进行编码。
    /// 默认值：无
    /// 可选值：**url**
    #[setters(generate = true, strip_option)]
    encoding_type: Option<String>,
}

impl sealed::Bound for InitiateMultipartUpload {}

impl InitiateMultipartUpload {
    pub fn new() -> Self {
        Self {
            encoding_type: None,
        }
    }
}
impl crate::ToFormData for InitiateMultipartUpload {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for InitiateMultipartUpload {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "InitiateMultipartUpload";

    type Body = crate::Form<Self>;

    type Response = InitiateMultipartUploadResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.encoding_type {
            params.insert("encoding-type".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UploadPart {
    /// 每一个上传的Part都有一个标识它的号码（partNumber）。
    ///
    /// 取值：1~10000
    ///
    /// 单个Part的大小限制为100 KB~5 GB。
    /// >MultipartUpload事件中除最后一个Part以外，其他Part的大小都要大于或等于100 KB。因不确定是否为最后一个Part，UploadPart接口并不会立即校验上传Part的大小，只有当CompleteMultipartUpload时才会校验。
    part_number: i64,
    /// uploadId用于唯一标识上传的Part属于哪个Object。
    upload_id: String,
    /// 请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for UploadPart {}

impl UploadPart {
    pub fn new(part_number: impl Into<i64>, upload_id: impl Into<String>) -> Self {
        Self {
            part_number: part_number.into(),
            upload_id: upload_id.into(),
            body: None,
        }
    }
}

impl crate::Request for UploadPart {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UploadPart";

    type Body = crate::OctetStream;

    type Response = UploadPartResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("partNumber".into(), (&self.part_number).into());
        params.insert("uploadId".into(), (&self.upload_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CompleteMultipartUpload {
    /// 此次MultipartUpload事件的唯一标识。
    upload_id: String,
    /// 指定对返回的Key进行编码，目前只支持URL编码。
    ///
    ///
    /// Key使用UTF-8字符，但XML 1.0标准不支持解析一些控制字符，例如ASCII码值从0到10的字符。当Key中包含XML 1.0标准不支持的控制字符时，您可以通过指定Encoding-type对返回的Key进行编码。
    ///
    /// 默认值：无
    ///
    /// 有效值：url
    #[setters(generate = true, strip_option)]
    encoding_type: Option<String>,
}

impl sealed::Bound for CompleteMultipartUpload {}

impl CompleteMultipartUpload {
    pub fn new(upload_id: impl Into<String>) -> Self {
        Self {
            upload_id: upload_id.into(),
            encoding_type: None,
        }
    }
}
impl crate::ToFormData for CompleteMultipartUpload {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for CompleteMultipartUpload {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CompleteMultipartUpload";

    type Body = crate::Form<Self>;

    type Response = CompleteMultipartUploadResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("uploadId".into(), (&self.upload_id).into());

        if let Some(f) = &self.encoding_type {
            params.insert("encoding-type".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UploadPartCopy {
    /// 每一个上传的Part都有一个标识它的号码（partNumber）。
    ///
    /// 取值：1~10000
    ///
    /// 单个Part的大小限制为100 KB~5 GB。
    /// > MultipartUpload事件中除最后一个Part以外，其他Part的大小都要大于或等于100 KB。因不确定是否为最后一个Part，UploadPart接口并不会立即校验上传Part的大小，只有当CompleteMultipartUpload时才会校验。
    part_number: i64,
    /// uploadId用于唯一标识上传的Part属于哪个Object。
    upload_id: String,
}

impl sealed::Bound for UploadPartCopy {}

impl UploadPartCopy {
    pub fn new(part_number: impl Into<i64>, upload_id: impl Into<String>) -> Self {
        Self {
            part_number: part_number.into(),
            upload_id: upload_id.into(),
        }
    }
}

impl crate::Request for UploadPartCopy {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "UploadPartCopy";

    type Body = ();

    type Response = UploadPartCopyResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("partNumber".into(), (&self.part_number).into());
        params.insert("uploadId".into(), (&self.upload_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AbortMultipartUpload {
    /// 此次MultipartUpload事件的唯一标识。
    upload_id: String,
}

impl sealed::Bound for AbortMultipartUpload {}

impl AbortMultipartUpload {
    pub fn new(upload_id: impl Into<String>) -> Self {
        Self {
            upload_id: upload_id.into(),
        }
    }
}

impl crate::Request for AbortMultipartUpload {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "AbortMultipartUpload";

    type Body = ();

    type Response = AbortMultipartUploadResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("uploadId".into(), (&self.upload_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListMultipartUploads {
    /// 用于对Object名称进行分组的字符。所有名称包含指定的前缀且首次出现delimiter字符之间的Object作为一组元素CommonPrefixes。
    #[setters(generate = true, strip_option)]
    delimiter: Option<String>,
    /// 限定此次返回Multipart Upload事件的最大个数，默认值为1000。最大值为1000。
    #[setters(generate = true, strip_option)]
    max_uploads: Option<i64>,
    /// 与upload-id-marker参数配合使用，用于指定返回结果的起始位置。
    ///   - 如果未设置upload-id-marker参数，查询结果中包含所有Object名称的字典序大于key-marker参数值的Multipart Upload事件。
    ///   - 如果设置了upload-id-marker参数，查询结果中包含：
    ///     - 所有Object名称的字典序大于key-marker参数值的Multipart Upload事件。
    ///     - Object名称等于key-marker参数值，但是UploadId比upload-id-marker参数值大的Multipart Upload事件。
    #[setters(generate = true, strip_option)]
    key_marker: Option<String>,
    /// 限定返回的Object Key必须以prefix作为前缀。注意使用prefix查询时，返回的Key中仍会包含prefix。
    /// > 您可以灵活地使用prefix参数对Bucket内的Object进行分组管理（类似文件夹功能）。
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 与key-marker参数配合使用，用于指定返回结果的起始位置。
    ///   - 如果未设置key-marker参数，则OSS会忽略upload-id-marker参数。
    ///   - 如果设置了key-marker参数，查询结果中包含：
    ///     - 所有Object名称的字典序大于key-marker参数值的Multipart Upload事件。
    ///     - Object名称等于key-marker参数值，但是UploadId比upload-id-marker参数值大的Multipart Upload事件。
    #[setters(generate = true, strip_option)]
    upload_id_marker: Option<String>,
    /// 指定对返回的内容进行编码，指定编码的类型。Delimiter、KeyMarker、Prefix、NextKeyMarker和Key使用UTF-8字符，但xml 1.0标准不支持解析一些控制字符，例如ASCII值从0到10的字符。对于包含xml
    ///                               1.0标准不支持的控制字符，可以通过指定encoding-type对返回的Delimiter、KeyMarker、Prefix、NextKeyMarker和Key进行编码。
    ///
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    encoding_type: Option<String>,
}

impl sealed::Bound for ListMultipartUploads {}

impl ListMultipartUploads {
    pub fn new() -> Self {
        Self {
            delimiter: None,
            max_uploads: None,
            key_marker: None,
            prefix: None,
            upload_id_marker: None,
            encoding_type: None,
        }
    }
}

impl crate::Request for ListMultipartUploads {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListMultipartUploads";

    type Body = ();

    type Response = ListMultipartUploadsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.delimiter {
            params.insert("delimiter".into(), f.into());
        }

        if let Some(f) = &self.max_uploads {
            params.insert("max-uploads".into(), f.into());
        }

        if let Some(f) = &self.key_marker {
            params.insert("key-marker".into(), f.into());
        }

        if let Some(f) = &self.prefix {
            params.insert("prefix".into(), f.into());
        }

        if let Some(f) = &self.upload_id_marker {
            params.insert("upload-id-marker".into(), f.into());
        }

        if let Some(f) = &self.encoding_type {
            params.insert("encoding-type".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListParts {
    /// MultipartUpload事件的ID。
    upload_id: String,
    /// 规定在OSS响应中的最大Part数目。
    ///
    /// 默认值：**1,000**
    ///
    /// 最大值：**1,000**
    #[setters(generate = true, strip_option)]
    max_parts: Option<i64>,
    /// 指定List的起始位置，只有Part Number数目大于该参数的Part会被列出。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    part_number_marker: Option<i64>,
    /// 指定对返回的内容进行编码，指定编码的类型。Key使用UTF-8字符，但xml 1.0标准不支持解析一些控制字符，比如ascii值从0到10的字符。对于Key中包含xml 1.0标准不支持的控制字符，可以通过指定Encoding-type对返回的Key进行编码。
    ///
    /// 默认值：无
    ///
    /// 可选值：**url**
    #[setters(generate = true, strip_option)]
    encoding_type: Option<String>,
}

impl sealed::Bound for ListParts {}

impl ListParts {
    pub fn new(upload_id: impl Into<String>) -> Self {
        Self {
            upload_id: upload_id.into(),
            max_parts: None,
            part_number_marker: None,
            encoding_type: None,
        }
    }
}

impl crate::Request for ListParts {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListParts";

    type Body = ();

    type Response = ListPartsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("uploadId".into(), (&self.upload_id).into());

        if let Some(f) = &self.max_parts {
            params.insert("max-parts".into(), f.into());
        }

        if let Some(f) = &self.part_number_marker {
            params.insert("part-number-marker".into(), f.into());
        }

        if let Some(f) = &self.encoding_type {
            params.insert("encoding-type".into(), serde_json::to_string(f)?.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutObjectAcl {
    /// Object对应的版本
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for PutObjectAcl {}

impl PutObjectAcl {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}

impl crate::Request for PutObjectAcl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutObjectAcl";

    type Body = ();

    type Response = PutObjectAclResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetObjectAcl {
    /// Object对应的版本。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObjectAcl {}

impl GetObjectAcl {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}

impl crate::Request for GetObjectAcl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObjectAcl";

    type Body = ();

    type Response = GetObjectAclResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutSymlink {}

impl sealed::Bound for PutSymlink {}

impl PutSymlink {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutSymlink {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutSymlink";

    type Body = ();

    type Response = PutSymlinkResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetSymlink {
    /// 获取软链接的当前Object版本。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetSymlink {}

impl GetSymlink {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}

impl crate::Request for GetSymlink {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetSymlink";

    type Body = ();

    type Response = GetSymlinkResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutObjectTagging {
    /// 版本对应的ID。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for PutObjectTagging {}

impl PutObjectTagging {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}

impl crate::Request for PutObjectTagging {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutObjectTagging";

    type Body = ();

    type Response = PutObjectTaggingResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetObjectTagging {
    /// 获取的目标Object的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObjectTagging {}

impl GetObjectTagging {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}

impl crate::Request for GetObjectTagging {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObjectTagging";

    type Body = ();

    type Response = GetObjectTaggingResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteObjectTagging {
    /// 删除的Object的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for DeleteObjectTagging {}

impl DeleteObjectTagging {
    pub fn new() -> Self {
        Self { version_id: None }
    }
}

impl crate::Request for DeleteObjectTagging {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteObjectTagging";

    type Body = ();

    type Response = DeleteObjectTaggingResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.version_id {
            params.insert("versionId".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutLiveChannel {}

impl sealed::Bound for PutLiveChannel {}

impl PutLiveChannel {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutLiveChannel {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutLiveChannel";

    type Body = ();

    type Response = PutLiveChannelResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListLiveChannel {
    /// 设定结果从marker之后按字母排序的第一个开始返回。
    #[setters(generate = true, strip_option)]
    marker: Option<String>,
    /// 限定此次返回LiveChannel的最大数。max-keys取值不能大于1000。
    /// 默认值：100
    #[setters(generate = true, strip_option)]
    max_keys: Option<i64>,
    /// 限定返回的LiveChannel必须以prefix作为前缀。注意使用prefix查询时，返回的key中仍会包含prefix。
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
}

impl sealed::Bound for ListLiveChannel {}

impl ListLiveChannel {
    pub fn new() -> Self {
        Self {
            marker: None,
            max_keys: None,
            prefix: None,
        }
    }
}

impl crate::Request for ListLiveChannel {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListLiveChannel";

    type Body = ();

    type Response = ListLiveChannelResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.marker {
            params.insert("marker".into(), f.into());
        }

        if let Some(f) = &self.max_keys {
            params.insert("max-keys".into(), f.into());
        }

        if let Some(f) = &self.prefix {
            params.insert("prefix".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteLiveChannel {}

impl sealed::Bound for DeleteLiveChannel {}

impl DeleteLiveChannel {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteLiveChannel {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteLiveChannel";

    type Body = ();

    type Response = DeleteLiveChannelResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutLiveChannelStatus {
    /// 设置LiveChannel的Status。
    /// 有效值：
    /// - enabled：启用LiveChannel
    /// - disabled：禁用LiveChannel
    status: String,
}

impl sealed::Bound for PutLiveChannelStatus {}

impl PutLiveChannelStatus {
    pub fn new(status: impl Into<String>) -> Self {
        Self {
            status: status.into(),
        }
    }
}

impl crate::Request for PutLiveChannelStatus {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutLiveChannelStatus";

    type Body = ();

    type Response = PutLiveChannelStatusResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("status".into(), (&self.status).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLiveChannelInfo {}

impl sealed::Bound for GetLiveChannelInfo {}

impl GetLiveChannelInfo {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetLiveChannelInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLiveChannelInfo";

    type Body = ();

    type Response = GetLiveChannelInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLiveChannelHistory {}

impl sealed::Bound for GetLiveChannelHistory {}

impl GetLiveChannelHistory {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetLiveChannelHistory {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLiveChannelHistory";

    type Body = ();

    type Response = GetLiveChannelHistoryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLiveChannelStat {}

impl sealed::Bound for GetLiveChannelStat {}

impl GetLiveChannelStat {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetLiveChannelStat {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLiveChannelStat";

    type Body = ();

    type Response = GetLiveChannelStatResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetVodPlaylist {
    /// 指定查询ts文件的终止时间，格式为Unix timestamp。
    /// > EndTime必须大于 StartTime，且时间跨度不能大于1天。
    end_time: String,
    /// 指定查询ts文件的起始时间，格式为Unix timestamp。
    start_time: String,
}

impl sealed::Bound for GetVodPlaylist {}

impl GetVodPlaylist {
    pub fn new(end_time: impl Into<String>, start_time: impl Into<String>) -> Self {
        Self {
            end_time: end_time.into(),
            start_time: start_time.into(),
        }
    }
}

impl crate::Request for GetVodPlaylist {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetVodPlaylist";

    type Body = ();

    type Response = GetVodPlaylistResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("endTime".into(), (&self.end_time).into());
        params.insert("startTime".into(), (&self.start_time).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PostVodPlaylist {
    /// 指定查询ts文件的终止时间。
    /// 格式：Unix timestamp
    /// >EndTime必须大于StartTime，且时间跨度不能大于1天。
    end_time: String,
    /// 指定查询ts文件的起始时间，格式为Unix timestamp。
    start_time: String,
}

impl sealed::Bound for PostVodPlaylist {}

impl PostVodPlaylist {
    pub fn new(end_time: impl Into<String>, start_time: impl Into<String>) -> Self {
        Self {
            end_time: end_time.into(),
            start_time: start_time.into(),
        }
    }
}
impl crate::ToFormData for PostVodPlaylist {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for PostVodPlaylist {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PostVodPlaylist";

    type Body = crate::Form<Self>;

    type Response = PostVodPlaylistResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("endTime".into(), (&self.end_time).into());
        params.insert("startTime".into(), (&self.start_time).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutChannel {}

impl sealed::Bound for PutChannel {}

impl PutChannel {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutChannel {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutChannel";

    type Body = ();

    type Response = PutChannelResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketHash {}

impl sealed::Bound for PutBucketHash {}

impl PutBucketHash {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketHash {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketHash";

    type Body = ();

    type Response = PutBucketHashResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketCommonHeader {}

impl sealed::Bound for PutBucketCommonHeader {}

impl PutBucketCommonHeader {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutBucketCommonHeader {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutBucketCommonHeader";

    type Body = ();

    type Response = PutBucketCommonHeaderResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketCommonHeader {}

impl sealed::Bound for DeleteBucketCommonHeader {}

impl DeleteBucketCommonHeader {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteBucketCommonHeader {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteBucketCommonHeader";

    type Body = ();

    type Response = DeleteBucketCommonHeaderResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutProcessConfiguration {}

impl sealed::Bound for PutProcessConfiguration {}

impl PutProcessConfiguration {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for PutProcessConfiguration {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutProcessConfiguration";

    type Body = ();

    type Response = PutProcessConfigurationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketEventNotification {}

impl sealed::Bound for GetBucketEventNotification {}

impl GetBucketEventNotification {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetBucketEventNotification {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketEventNotification";

    type Body = ();

    type Response = GetBucketEventNotificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutDataLakeCachePrefetchJob {
    #[setters(generate = true, strip_option)]
    x_oss_datalake_job_id: Option<String>,
}

impl sealed::Bound for PutDataLakeCachePrefetchJob {}

impl PutDataLakeCachePrefetchJob {
    pub fn new() -> Self {
        Self {
            x_oss_datalake_job_id: None,
        }
    }
}

impl crate::Request for PutDataLakeCachePrefetchJob {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "PutDataLakeCachePrefetchJob";

    type Body = ();

    type Response = PutDataLakeCachePrefetchJobResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.x_oss_datalake_job_id {
            params.insert("x-oss-datalake-job-id".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct StartDataLakeCachePrefetchJob {
    x_oss_datalake_job_id: String,
}

impl sealed::Bound for StartDataLakeCachePrefetchJob {}

impl StartDataLakeCachePrefetchJob {
    pub fn new(x_oss_datalake_job_id: impl Into<String>) -> Self {
        Self {
            x_oss_datalake_job_id: x_oss_datalake_job_id.into(),
        }
    }
}

impl crate::Request for StartDataLakeCachePrefetchJob {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "StartDataLakeCachePrefetchJob";

    type Body = ();

    type Response = StartDataLakeCachePrefetchJobResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "x-oss-datalake-job-id".into(),
            (&self.x_oss_datalake_job_id).into(),
        );

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListDataLakeStorageTransferJob {}

impl sealed::Bound for ListDataLakeStorageTransferJob {}

impl ListDataLakeStorageTransferJob {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for ListDataLakeStorageTransferJob {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListDataLakeStorageTransferJob";

    type Body = ();

    type Response = ListDataLakeStorageTransferJobResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompleteMultipartUploadResponseCompleteMultipartUploadResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Location")]
    pub location: String,
}

impl crate::FlatSerialize for CompleteMultipartUploadResponseCompleteMultipartUploadResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CopyObjectResponseCopyObjectResult {
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
}

impl crate::FlatSerialize for CopyObjectResponseCopyObjectResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointForObjectProcessResponseCreateAccessPointForObjectProcessResult {
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    pub access_point_for_object_process_alias: String,
    #[serde(rename = "AccessPointForObjectProcessArn")]
    pub access_point_for_object_process_arn: String,
}

impl crate::FlatSerialize
    for CreateAccessPointForObjectProcessResponseCreateAccessPointForObjectProcessResult
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point_for_object_process_alias,
            &format!("{}.AccessPointForObjectProcessAlias", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_point_for_object_process_arn,
            &format!("{}.AccessPointForObjectProcessArn", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateBucketDataRedundancyTransitionResponseBucketDataRedundancyTransition {
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

impl crate::FlatSerialize
    for CreateBucketDataRedundancyTransitionResponseBucketDataRedundancyTransition
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.task_id, &format!("{}.TaskId", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsResponseRegionInfoList {
    #[serde(rename = "RegionInfo")]
    pub region_info: Vec<String>,
}

impl crate::FlatSerialize for DescribeRegionsResponseRegionInfoList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.region_info,
            &format!("{}.RegionInfo", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointConfigForObjectProcessResponseGetAccessPointConfigForObjectProcessResult {
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: String,
    #[serde(rename = "ObjectProcessConfiguration")]
    pub object_process_configuration: String,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: String,
}

impl crate::FlatSerialize
    for GetAccessPointConfigForObjectProcessResponseGetAccessPointConfigForObjectProcessResult
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.allow_anonymous_access_for_object_process,
            &format!("{}.AllowAnonymousAccessForObjectProcess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.object_process_configuration,
            &format!("{}.ObjectProcessConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.public_access_block_configuration,
            &format!("{}.PublicAccessBlockConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResult {
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    pub access_point_for_object_process_alias: String,
    #[serde(rename = "AccessPointForObjectProcessArn")]
    pub access_point_for_object_process_arn: String,
    #[serde(rename = "AccessPointName")]
    pub access_point_name: String,
    #[serde(rename = "AccessPointNameForObjectProcess")]
    pub access_point_name_for_object_process: String,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: String,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    #[serde(rename = "Endpoints")]
    pub endpoints:
        GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResultEndpoints,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: String,
    #[serde(rename = "Status")]
    pub status: String,
}

impl crate::FlatSerialize
    for GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResult
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point_for_object_process_alias,
            &format!("{}.AccessPointForObjectProcessAlias", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_point_for_object_process_arn,
            &format!("{}.AccessPointForObjectProcessArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_point_name,
            &format!("{}.AccessPointName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_point_name_for_object_process,
            &format!("{}.AccessPointNameForObjectProcess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.account_id,
            &format!("{}.AccountId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.allow_anonymous_access_for_object_process,
            &format!("{}.AllowAnonymousAccessForObjectProcess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.endpoints,
            &format!("{}.Endpoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.public_access_block_configuration,
            &format!("{}.PublicAccessBlockConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResultEndpoints {
    #[serde(rename = "InternalEndpoint")]
    pub internal_endpoint: String,
    #[serde(rename = "PublicEndpoint")]
    pub public_endpoint: String,
}

impl crate::FlatSerialize
    for GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResultEndpoints
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.internal_endpoint,
            &format!("{}.InternalEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.public_endpoint,
            &format!("{}.PublicEndpoint", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketAclResponseAccessControlPolicy {
    #[serde(rename = "AccessControlList")]
    pub access_control_list: GetBucketAclResponseAccessControlPolicyAccessControlList,
    #[serde(rename = "Owner")]
    pub owner: String,
}

impl crate::FlatSerialize for GetBucketAclResponseAccessControlPolicy {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_control_list,
            &format!("{}.AccessControlList", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketAclResponseAccessControlPolicyAccessControlList {
    #[serde(rename = "Grant")]
    pub grant: String,
}

impl crate::FlatSerialize for GetBucketAclResponseAccessControlPolicyAccessControlList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.grant, &format!("{}.Grant", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketCorsResponseCorsConfiguration {
    #[serde(rename = "CORSRule")]
    pub cors_rule: Vec<String>,
    #[serde(rename = "ResponseVary")]
    pub response_vary: bool,
}

impl crate::FlatSerialize for GetBucketCorsResponseCorsConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.cors_rule,
            &format!("{}.CORSRule", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.response_vary,
            &format!("{}.ResponseVary", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketEncryptionResponseServerSideEncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    pub apply_server_side_encryption_by_default: String,
}

impl crate::FlatSerialize for GetBucketEncryptionResponseServerSideEncryptionRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.apply_server_side_encryption_by_default,
            &format!("{}.ApplyServerSideEncryptionByDefault", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketPolicyStatusResponsePolicyStatus {
    #[serde(rename = "IsPublic")]
    pub is_public: bool,
}

impl crate::FlatSerialize for GetBucketPolicyStatusResponsePolicyStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.is_public,
            &format!("{}.IsPublic", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationLocationResponseReplicationLocation {
    #[serde(rename = "Location")]
    pub location: Vec<String>,
    #[serde(rename = "LocationRTCConstraint")]
    pub location_rtc_constraint:
        GetBucketReplicationLocationResponseReplicationLocationLocationRtcConstraint,
    #[serde(rename = "LocationTransferTypeConstraint")]
    pub location_transfer_type_constraint:
        GetBucketReplicationLocationResponseReplicationLocationLocationTransferTypeConstraint,
}

impl crate::FlatSerialize for GetBucketReplicationLocationResponseReplicationLocation {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.location_rtc_constraint,
            &format!("{}.LocationRTCConstraint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.location_transfer_type_constraint,
            &format!("{}.LocationTransferTypeConstraint", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationLocationResponseReplicationLocationLocationRtcConstraint {
    #[serde(rename = "Location")]
    pub location: Vec<String>,
}

impl crate::FlatSerialize
    for GetBucketReplicationLocationResponseReplicationLocationLocationRtcConstraint
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationLocationResponseReplicationLocationLocationTransferTypeConstraint {
    #[serde(rename = "LocationTransferType")]
    pub location_transfer_type: Vec<String>,
}

impl crate::FlatSerialize
    for GetBucketReplicationLocationResponseReplicationLocationLocationTransferTypeConstraint
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.location_transfer_type,
            &format!("{}.LocationTransferType", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationProgressResponseReplicationProgress {
    #[serde(rename = "Rule")]
    pub rule: Vec<String>,
}

impl crate::FlatSerialize for GetBucketReplicationProgressResponseReplicationProgress {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationResponseReplicationConfiguration {
    #[serde(rename = "Rule")]
    pub rule: Vec<String>,
}

impl crate::FlatSerialize for GetBucketReplicationResponseReplicationConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketRequestPaymentResponseRequestPaymentConfiguration {
    #[serde(rename = "Payer")]
    pub payer: String,
}

impl crate::FlatSerialize for GetBucketRequestPaymentResponseRequestPaymentConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.payer, &format!("{}.Payer", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketResourceGroupResponseBucketResourceGroupConfiguration {
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: String,
}

impl crate::FlatSerialize for GetBucketResourceGroupResponseBucketResourceGroupConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resource_group_id,
            &format!("{}.ResourceGroupId", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketTagsResponseTagging {
    #[serde(rename = "TagSet")]
    pub tag_set: String,
}

impl crate::FlatSerialize for GetBucketTagsResponseTagging {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tag_set, &format!("{}.TagSet", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketTransferAccelerationResponseTransferAccelerationConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl crate::FlatSerialize
    for GetBucketTransferAccelerationResponseTransferAccelerationConfiguration
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.enabled, &format!("{}.Enabled", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketVersioningResponseVersioningConfiguration {
    #[serde(rename = "Status")]
    pub status: String,
}

impl crate::FlatSerialize for GetBucketVersioningResponseVersioningConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketWormResponseWormConfiguration {
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    #[serde(rename = "ExpirationDate")]
    pub expiration_date: String,
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: i32,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "WormId")]
    pub worm_id: String,
}

impl crate::FlatSerialize for GetBucketWormResponseWormConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.expiration_date,
            &format!("{}.ExpirationDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.retention_period_in_days,
            &format!("{}.RetentionPeriodInDays", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.state, &format!("{}.State", name), params);
        crate::FlatSerialize::flat_serialize(&self.worm_id, &format!("{}.WormId", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelHistoryResponseLiveChannelHistory {
    #[serde(rename = "LiveRecord")]
    pub live_record: Vec<String>,
}

impl crate::FlatSerialize for GetLiveChannelHistoryResponseLiveChannelHistory {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.live_record,
            &format!("{}.LiveRecord", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelInfoResponseLiveChannelConfiguration {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Target")]
    pub target: String,
}

impl crate::FlatSerialize for GetLiveChannelInfoResponseLiveChannelConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.Description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(&self.target, &format!("{}.Target", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelStatResponseLiveChannelStat {
    #[serde(rename = "Audio")]
    pub audio: String,
    #[serde(rename = "ConnectedTime")]
    pub connected_time: String,
    #[serde(rename = "RemoteAddr")]
    pub remote_addr: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Video")]
    pub video: String,
}

impl crate::FlatSerialize for GetLiveChannelStatResponseLiveChannelStat {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.audio, &format!("{}.Audio", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.connected_time,
            &format!("{}.ConnectedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.remote_addr,
            &format!("{}.RemoteAddr", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(&self.video, &format!("{}.Video", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetMetaQueryStatusResponseMetaQueryStatus {
    #[serde(rename = "CreateTime")]
    pub create_time: String,
    #[serde(rename = "Phase")]
    pub phase: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "UpdateTime")]
    pub update_time: String,
}

impl crate::FlatSerialize for GetMetaQueryStatusResponseMetaQueryStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.phase, &format!("{}.Phase", name), params);
        crate::FlatSerialize::flat_serialize(&self.state, &format!("{}.State", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.update_time,
            &format!("{}.UpdateTime", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectAclResponseAccessControlPolicy {
    #[serde(rename = "AccessControlList")]
    pub access_control_list: GetObjectAclResponseAccessControlPolicyAccessControlList,
    #[serde(rename = "Owner")]
    pub owner: String,
}

impl crate::FlatSerialize for GetObjectAclResponseAccessControlPolicy {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_control_list,
            &format!("{}.AccessControlList", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectAclResponseAccessControlPolicyAccessControlList {
    #[serde(rename = "Grant")]
    pub grant: String,
}

impl crate::FlatSerialize for GetObjectAclResponseAccessControlPolicyAccessControlList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.grant, &format!("{}.Grant", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectTaggingResponseTagging {
    #[serde(rename = "TagSet")]
    pub tag_set: String,
}

impl crate::FlatSerialize for GetObjectTaggingResponseTagging {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tag_set, &format!("{}.TagSet", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetUserAntiDDosInfoResponseAntiDdosListConfiguration {
    #[serde(rename = "AntiDDOSConfiguration")]
    pub anti_ddos_configuration: Vec<String>,
}

impl crate::FlatSerialize for GetUserAntiDDosInfoResponseAntiDdosListConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.anti_ddos_configuration,
            &format!("{}.AntiDDOSConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InitiateMultipartUploadResponseInitiateMultipartUploadResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "UploadId")]
    pub upload_id: String,
}

impl crate::FlatSerialize for InitiateMultipartUploadResponseInitiateMultipartUploadResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.upload_id,
            &format!("{}.UploadId", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResult {
    #[serde(rename = "AccessPointsForObjectProcess")]
    pub access_points_for_object_process: ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcess,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: String,
}

impl crate::FlatSerialize
    for ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResult
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_points_for_object_process,
            &format!("{}.AccessPointsForObjectProcess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.account_id,
            &format!("{}.AccountId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_continuation_token,
            &format!("{}.NextContinuationToken", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcess {
    #[serde(rename = "AccessPointForObjectProcess")]
    pub access_point_for_object_process: Vec<ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcessAccessPointForObjectProcessItem>,
}

impl crate::FlatSerialize for ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcess {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'a>>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.access_point_for_object_process, &format!("{}.AccessPointForObjectProcess", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcessAccessPointForObjectProcessItem
{
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    pub access_point_for_object_process_alias: String,
    #[serde(rename = "AccessPointName")]
    pub access_point_name: String,
    #[serde(rename = "AccessPointNameForObjectProcess")]
    pub access_point_name_for_object_process: String,
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: String,
    #[serde(rename = "Status")]
    pub status: String,
}

impl crate::FlatSerialize for ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcessAccessPointForObjectProcessItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'a>>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.access_point_for_object_process_alias, &format!("{}.AccessPointForObjectProcessAlias", name), params);
        crate::FlatSerialize::flat_serialize(&self.access_point_name, &format!("{}.AccessPointName", name), params);
        crate::FlatSerialize::flat_serialize(&self.access_point_name_for_object_process, &format!("{}.AccessPointNameForObjectProcess", name), params);
        crate::FlatSerialize::flat_serialize(&self.allow_anonymous_access_for_object_process, &format!("{}.AllowAnonymousAccessForObjectProcess", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketAntiDDosInfoResponseAntiDdosListConfiguration {
    #[serde(rename = "AntiDDOSConfiguration")]
    pub anti_ddos_configuration: Vec<String>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "Marker")]
    pub marker: String,
}

impl crate::FlatSerialize for ListBucketAntiDDosInfoResponseAntiDdosListConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.anti_ddos_configuration,
            &format!("{}.AntiDDOSConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.marker, &format!("{}.Marker", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketDataRedundancyTransitionResponseListBucketDataRedundancyTransition {
    #[serde(rename = "BucketDataRedundancyTransition")]
    pub bucket_data_redundancy_transition: String,
}

impl crate::FlatSerialize
    for ListBucketDataRedundancyTransitionResponseListBucketDataRedundancyTransition
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bucket_data_redundancy_transition,
            &format!("{}.BucketDataRedundancyTransition", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketInventoryResponseListInventoryConfigurationsResult {
    #[serde(rename = "InventoryConfiguration")]
    pub inventory_configuration: Vec<String>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: String,
}

impl crate::FlatSerialize for ListBucketInventoryResponseListInventoryConfigurationsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.inventory_configuration,
            &format!("{}.InventoryConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_continuation_token,
            &format!("{}.NextContinuationToken", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketsResponseListAllMyBucketsResult {
    #[serde(rename = "Buckets")]
    pub buckets: ListBucketsResponseListAllMyBucketsResultBuckets,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "Marker")]
    pub marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i64,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
}

impl crate::FlatSerialize for ListBucketsResponseListAllMyBucketsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.buckets, &format!("{}.Buckets", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.marker, &format!("{}.Marker", name), params);
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_marker,
            &format!("{}.NextMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketsResponseListAllMyBucketsResultBuckets {
    #[serde(rename = "Bucket")]
    pub bucket: Vec<String>,
}

impl crate::FlatSerialize for ListBucketsResponseListAllMyBucketsResultBuckets {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListCnameResponseListCnameResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Cname")]
    pub cname: Vec<String>,
    #[serde(rename = "Owner")]
    pub owner: String,
}

impl crate::FlatSerialize for ListCnameResponseListCnameResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.cname, &format!("{}.Cname", name), params);
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListLiveChannelResponseListLiveChannelResult {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "LiveChannel")]
    pub live_channel: Vec<String>,
    #[serde(rename = "Marker")]
    pub marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i64,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
}

impl crate::FlatSerialize for ListLiveChannelResponseListLiveChannelResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.live_channel,
            &format!("{}.LiveChannel", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.marker, &format!("{}.Marker", name), params);
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_marker,
            &format!("{}.NextMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListMultipartUploadsResponseListMultipartUploadsResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Vec<String>,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "KeyMarker")]
    pub key_marker: String,
    #[serde(rename = "MaxUploads")]
    pub max_uploads: i64,
    #[serde(rename = "NextKeyMarker")]
    pub next_key_marker: String,
    #[serde(rename = "NextUploadIdMarker")]
    pub next_upload_id_marker: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Upload")]
    pub upload: Vec<String>,
    #[serde(rename = "UploadIdMarker")]
    pub upload_id_marker: String,
}

impl crate::FlatSerialize for ListMultipartUploadsResponseListMultipartUploadsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.common_prefixes,
            &format!("{}.CommonPrefixes", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.delimiter,
            &format!("{}.Delimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.key_marker,
            &format!("{}.KeyMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_uploads,
            &format!("{}.MaxUploads", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_key_marker,
            &format!("{}.NextKeyMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_upload_id_marker,
            &format!("{}.NextUploadIdMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.upload, &format!("{}.Upload", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.upload_id_marker,
            &format!("{}.UploadIdMarker", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectVersionsResponseListVersionsResult {
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Vec<String>,
    #[serde(rename = "DeleteMarker")]
    pub delete_marker: Vec<String>,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "KeyMarker")]
    pub key_marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NextKeyMarker")]
    pub next_key_marker: String,
    #[serde(rename = "NextVersionIdMarker")]
    pub next_version_id_marker: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Version")]
    pub version: Vec<String>,
    #[serde(rename = "VersionIdMarker")]
    pub version_id_marker: String,
}

impl crate::FlatSerialize for ListObjectVersionsResponseListVersionsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.common_prefixes,
            &format!("{}.CommonPrefixes", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.delete_marker,
            &format!("{}.DeleteMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.delimiter,
            &format!("{}.Delimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.key_marker,
            &format!("{}.KeyMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_key_marker,
            &format!("{}.NextKeyMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_version_id_marker,
            &format!("{}.NextVersionIdMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.version, &format!("{}.Version", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.version_id_marker,
            &format!("{}.VersionIdMarker", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectsResponseListBucketResult {
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Vec<String>,
    #[serde(rename = "Contents")]
    pub contents: Vec<String>,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "Marker")]
    pub marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
}

impl crate::FlatSerialize for ListObjectsResponseListBucketResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.common_prefixes,
            &format!("{}.CommonPrefixes", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.contents, &format!("{}.Contents", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.delimiter,
            &format!("{}.Delimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.marker, &format!("{}.Marker", name), params);
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_marker,
            &format!("{}.NextMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectsV2ResponseListBucketResult {
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Vec<String>,
    #[serde(rename = "Contents")]
    pub contents: Vec<String>,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: String,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "KeyCount")]
    pub key_count: i32,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "StartAfter")]
    pub start_after: String,
}

impl crate::FlatSerialize for ListObjectsV2ResponseListBucketResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.common_prefixes,
            &format!("{}.CommonPrefixes", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.contents, &format!("{}.Contents", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.continuation_token,
            &format!("{}.ContinuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.delimiter,
            &format!("{}.Delimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.key_count,
            &format!("{}.KeyCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_continuation_token,
            &format!("{}.NextContinuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.start_after,
            &format!("{}.StartAfter", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListPartsResponseListPartResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "MaxParts")]
    pub max_parts: i64,
    #[serde(rename = "NextPartNumberMarker")]
    pub next_part_number_marker: i64,
    #[serde(rename = "Part")]
    pub part: Vec<String>,
    #[serde(rename = "PartNumberMarker")]
    pub part_number_marker: i64,
    #[serde(rename = "UploadId")]
    pub upload_id: String,
}

impl crate::FlatSerialize for ListPartsResponseListPartResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.max_parts,
            &format!("{}.MaxParts", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_part_number_marker,
            &format!("{}.NextPartNumberMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.part, &format!("{}.Part", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.part_number_marker,
            &format!("{}.PartNumberMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.upload_id,
            &format!("{}.UploadId", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListStyleResponseStyleList {
    #[serde(rename = "Style")]
    pub style: Vec<String>,
}

impl crate::FlatSerialize for ListStyleResponseStyleList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.style, &format!("{}.Style", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListUserDataRedundancyTransitionResponseListBucketDataRedundancyTransition {
    #[serde(rename = "BucketDataRedundancyTransition")]
    pub bucket_data_redundancy_transition: Vec<String>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: String,
}

impl crate::FlatSerialize
    for ListUserDataRedundancyTransitionResponseListBucketDataRedundancyTransition
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bucket_data_redundancy_transition,
            &format!("{}.BucketDataRedundancyTransition", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_continuation_token,
            &format!("{}.NextContinuationToken", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutDataLakeCachePrefetchJobResponseDataLakeCachePrefetchJobId {
    #[serde(rename = "ID")]
    pub id: String,
}

impl crate::FlatSerialize for PutDataLakeCachePrefetchJobResponseDataLakeCachePrefetchJobId {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutLiveChannelResponseCreateLiveChannelResult {
    #[serde(rename = "PlayUrls")]
    pub play_urls: String,
    #[serde(rename = "PublishUrls")]
    pub publish_urls: String,
}

impl crate::FlatSerialize for PutLiveChannelResponseCreateLiveChannelResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.play_urls,
            &format!("{}.PlayUrls", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.publish_urls,
            &format!("{}.PublishUrls", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UploadPartCopyResponseCopyPartResult {
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
}

impl crate::FlatSerialize for UploadPartCopyResponseCopyPartResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListAllMyBucketsResult")]
    pub list_all_my_buckets_result: ListBucketsResponseListAllMyBucketsResult,
}

impl AsRef<crate::CodeMessage> for ListBucketsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "RegionInfoList")]
    pub region_info_list: DescribeRegionsResponseRegionInfoList,
}

impl AsRef<crate::CodeMessage> for DescribeRegionsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketStatResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketStat")]
    pub bucket_stat: String,
}

impl AsRef<crate::CodeMessage> for GetBucketStatResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListBucketResult")]
    pub list_bucket_result: ListObjectsResponseListBucketResult,
}

impl AsRef<crate::CodeMessage> for ListObjectsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectsV2Response {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListBucketResult")]
    pub list_bucket_result: ListObjectsV2ResponseListBucketResult,
}

impl AsRef<crate::CodeMessage> for ListObjectsV2Response {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketInfo")]
    pub bucket_info: String,
}

impl AsRef<crate::CodeMessage> for GetBucketInfoResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketLocationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "LocationConstraint")]
    pub location_constraint: String,
}

impl AsRef<crate::CodeMessage> for GetBucketLocationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListAccessPointsResult")]
    pub list_access_points_result: String,
}

impl AsRef<crate::CodeMessage> for ListAccessPointsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "GetAccessPointResult")]
    pub get_access_point_result: String,
}

impl AsRef<crate::CodeMessage> for GetAccessPointResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointPolicyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for GetAccessPointPolicyResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteAccessPointPolicyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteAccessPointPolicyResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutAccessPointPolicyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutAccessPointPolicyResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteAccessPointResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteAccessPointResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CreateAccessPointResult")]
    pub create_access_point_result: String,
}

impl AsRef<crate::CodeMessage> for CreateAccessPointResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct InitiateBucketWormResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for InitiateBucketWormResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct AbortBucketWormResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for AbortBucketWormResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CompleteBucketWormResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for CompleteBucketWormResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ExtendBucketWormResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for ExtendBucketWormResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketWormResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "WormConfiguration")]
    pub worm_configuration: GetBucketWormResponseWormConfiguration,
}

impl AsRef<crate::CodeMessage> for GetBucketWormResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketAclResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketAclResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketAclResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "AccessControlPolicy")]
    pub access_control_policy: GetBucketAclResponseAccessControlPolicy,
}

impl AsRef<crate::CodeMessage> for GetBucketAclResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketLifecycleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketLifecycleResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketLifecycleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "LifecycleConfiguration")]
    pub lifecycle_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketLifecycleResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketLifecycleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketLifecycleResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketTransferAccelerationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketTransferAccelerationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketTransferAccelerationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "TransferAccelerationConfiguration")]
    pub transfer_acceleration_configuration:
        GetBucketTransferAccelerationResponseTransferAccelerationConfiguration,
}

impl AsRef<crate::CodeMessage> for GetBucketTransferAccelerationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketVersioningResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketVersioningResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketVersioningResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "VersioningConfiguration")]
    pub versioning_configuration: GetBucketVersioningResponseVersioningConfiguration,
}

impl AsRef<crate::CodeMessage> for GetBucketVersioningResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectVersionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListVersionsResult")]
    pub list_versions_result: ListObjectVersionsResponseListVersionsResult,
}

impl AsRef<crate::CodeMessage> for ListObjectVersionsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketPolicyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketPolicyResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketPolicyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for GetBucketPolicyResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketPolicyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketPolicyResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketPolicyStatusResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "PolicyStatus")]
    pub policy_status: GetBucketPolicyStatusResponsePolicyStatus,
}

impl AsRef<crate::CodeMessage> for GetBucketPolicyStatusResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketRtcResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketRtcResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketReplicationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketReplicationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: GetBucketReplicationResponseReplicationConfiguration,
}

impl AsRef<crate::CodeMessage> for GetBucketReplicationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationLocationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ReplicationLocation")]
    pub replication_location: GetBucketReplicationLocationResponseReplicationLocation,
}

impl AsRef<crate::CodeMessage> for GetBucketReplicationLocationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationProgressResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ReplicationProgress")]
    pub replication_progress: GetBucketReplicationProgressResponseReplicationProgress,
}

impl AsRef<crate::CodeMessage> for GetBucketReplicationProgressResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketReplicationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketReplicationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketInventoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketInventoryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketInventoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "InventoryConfiguration")]
    pub inventory_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketInventoryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketInventoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListInventoryConfigurationsResult")]
    pub list_inventory_configurations_result:
        ListBucketInventoryResponseListInventoryConfigurationsResult,
}

impl AsRef<crate::CodeMessage> for ListBucketInventoryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketInventoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketInventoryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketLoggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketLoggingResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketLoggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketLoggingStatus")]
    pub bucket_logging_status: String,
}

impl AsRef<crate::CodeMessage> for GetBucketLoggingResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketLoggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketLoggingResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutUserDefinedLogFieldsConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutUserDefinedLogFieldsConfigResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetUserDefinedLogFieldsConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "UserDefinedLogFieldsConfiguration")]
    pub user_defined_log_fields_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetUserDefinedLogFieldsConfigResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteUserDefinedLogFieldsConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteUserDefinedLogFieldsConfigResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketWebsiteResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "WebsiteConfiguration")]
    pub website_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketWebsiteResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketWebsiteResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketWebsiteResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketWebsiteResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketWebsiteResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketRefererResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketRefererResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketRefererResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "RefererConfiguration")]
    pub referer_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketRefererResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketTagsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketTagsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketTagsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Tagging")]
    pub tagging: GetBucketTagsResponseTagging,
}

impl AsRef<crate::CodeMessage> for GetBucketTagsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketTagsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketTagsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListUserDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListBucketDataRedundancyTransition")]
    pub list_bucket_data_redundancy_transition:
        ListUserDataRedundancyTransitionResponseListBucketDataRedundancyTransition,
}

impl AsRef<crate::CodeMessage> for ListUserDataRedundancyTransitionResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListBucketDataRedundancyTransition")]
    pub list_bucket_data_redundancy_transition:
        ListBucketDataRedundancyTransitionResponseListBucketDataRedundancyTransition,
}

impl AsRef<crate::CodeMessage> for ListBucketDataRedundancyTransitionResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketDataRedundancyTransition")]
    pub bucket_data_redundancy_transition: String,
}

impl AsRef<crate::CodeMessage> for GetBucketDataRedundancyTransitionResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateBucketDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketDataRedundancyTransition")]
    pub bucket_data_redundancy_transition:
        CreateBucketDataRedundancyTransitionResponseBucketDataRedundancyTransition,
}

impl AsRef<crate::CodeMessage> for CreateBucketDataRedundancyTransitionResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketDataRedundancyTransitionResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketEncryptionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketEncryptionResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketEncryptionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ServerSideEncryptionRule")]
    pub server_side_encryption_rule: GetBucketEncryptionResponseServerSideEncryptionRule,
}

impl AsRef<crate::CodeMessage> for GetBucketEncryptionResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketEncryptionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketEncryptionResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketRequestPaymentResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketRequestPaymentResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketRequestPaymentResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "RequestPaymentConfiguration")]
    pub request_payment_configuration: GetBucketRequestPaymentResponseRequestPaymentConfiguration,
}

impl AsRef<crate::CodeMessage> for GetBucketRequestPaymentResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketCorsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketCorsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketCorsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CORSConfiguration")]
    pub cors_configuration: GetBucketCorsResponseCorsConfiguration,
}

impl AsRef<crate::CodeMessage> for GetBucketCorsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketCorsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketCorsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct OptionObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for OptionObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketAccessMonitorResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketAccessMonitorResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketAccessMonitorResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "AccessMonitorConfiguration")]
    pub access_monitor_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketAccessMonitorResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetMetaQueryStatusResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "MetaQueryStatus")]
    pub meta_query_status: GetMetaQueryStatusResponseMetaQueryStatus,
}

impl AsRef<crate::CodeMessage> for GetMetaQueryStatusResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CloseMetaQueryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for CloseMetaQueryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DoMetaQueryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "MetaQuery")]
    pub meta_query: String,
}

impl AsRef<crate::CodeMessage> for DoMetaQueryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct OpenMetaQueryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for OpenMetaQueryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UpdateUserAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for UpdateUserAntiDDosInfoResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UpdateBucketAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for UpdateBucketAntiDDosInfoResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "AntiDDOSListConfiguration")]
    pub anti_ddos_list_configuration: ListBucketAntiDDosInfoResponseAntiDdosListConfiguration,
}

impl AsRef<crate::CodeMessage> for ListBucketAntiDDosInfoResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct InitUserAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for InitUserAntiDDosInfoResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct InitBucketAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for InitBucketAntiDDosInfoResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetUserAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "AntiDDOSListConfiguration")]
    pub anti_ddos_list_configuration: GetUserAntiDDosInfoResponseAntiDdosListConfiguration,
}

impl AsRef<crate::CodeMessage> for GetUserAntiDDosInfoResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketResourceGroupResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketResourceGroupConfiguration")]
    pub bucket_resource_group_configuration:
        GetBucketResourceGroupResponseBucketResourceGroupConfiguration,
}

impl AsRef<crate::CodeMessage> for GetBucketResourceGroupResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketResourceGroupResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketResourceGroupResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutCnameResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutCnameResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListCnameResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListCnameResult")]
    pub list_cname_result: ListCnameResponseListCnameResult,
}

impl AsRef<crate::CodeMessage> for ListCnameResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteCnameResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteCnameResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetCnameTokenResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CnameToken")]
    pub cname_token: String,
}

impl AsRef<crate::CodeMessage> for GetCnameTokenResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateCnameTokenResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CnameToken")]
    pub cname_token: String,
}

impl AsRef<crate::CodeMessage> for CreateCnameTokenResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutStyleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutStyleResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListStyleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "StyleList")]
    pub style_list: ListStyleResponseStyleList,
}

impl AsRef<crate::CodeMessage> for ListStyleResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetStyleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Style")]
    pub style: String,
}

impl AsRef<crate::CodeMessage> for GetStyleResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteStyleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteStyleResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketHttpsConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "HttpsConfiguration")]
    pub https_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketHttpsConfigResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketHttpsConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketHttpsConfigResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CreateAccessPointForObjectProcessResult")]
    pub create_access_point_for_object_process_result:
        CreateAccessPointForObjectProcessResponseCreateAccessPointForObjectProcessResult,
}

impl AsRef<crate::CodeMessage> for CreateAccessPointForObjectProcessResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "GetAccessPointForObjectProcessResult")]
    pub get_access_point_for_object_process_result:
        GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResult,
}

impl AsRef<crate::CodeMessage> for GetAccessPointForObjectProcessResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListAccessPointsForObjectProcessResult")]
    pub list_access_points_for_object_process_result:
        ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResult,
}

impl AsRef<crate::CodeMessage> for ListAccessPointsForObjectProcessResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteAccessPointForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteAccessPointForObjectProcessResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointConfigForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "GetAccessPointConfigForObjectProcessResult")]
    pub get_access_point_config_for_object_process_result:
        GetAccessPointConfigForObjectProcessResponseGetAccessPointConfigForObjectProcessResult,
}

impl AsRef<crate::CodeMessage> for GetAccessPointConfigForObjectProcessResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutAccessPointConfigForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutAccessPointConfigForObjectProcessResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutAccessPointPolicyForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutAccessPointPolicyForObjectProcessResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointPolicyForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for GetAccessPointPolicyForObjectProcessResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteAccessPointPolicyForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteAccessPointPolicyForObjectProcessResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetPublicAccessBlockResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutPublicAccessBlockResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeletePublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeletePublicAccessBlockResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketPublicAccessBlockResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketPublicAccessBlockResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketPublicAccessBlockResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetAccessPointPublicAccessBlockResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutAccessPointPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutAccessPointPublicAccessBlockResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteAccessPointPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteAccessPointPublicAccessBlockResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketArchiveDirectReadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ArchiveDirectReadConfiguration")]
    pub archive_direct_read_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketArchiveDirectReadResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketArchiveDirectReadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketArchiveDirectReadResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketOverwriteConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketOverwriteConfigResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketOverwriteConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "OverwriteConfiguration")]
    pub overwrite_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketOverwriteConfigResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketOverwriteConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketOverwriteConfigResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CopyObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CopyObjectResult")]
    pub copy_object_result: CopyObjectResponseCopyObjectResult,
}

impl AsRef<crate::CodeMessage> for CopyObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for GetObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct AppendObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for AppendObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct SealAppendObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for SealAppendObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct HeadObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for HeadObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectMetaResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for GetObjectMetaResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RestoreObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for RestoreObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CleanRestoredObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for CleanRestoredObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct SelectObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for SelectObjectResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateSelectObjectMetaResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for CreateSelectObjectMetaResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct InitiateMultipartUploadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "InitiateMultipartUploadResult")]
    pub initiate_multipart_upload_result:
        InitiateMultipartUploadResponseInitiateMultipartUploadResult,
}

impl AsRef<crate::CodeMessage> for InitiateMultipartUploadResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UploadPartResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for UploadPartResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CompleteMultipartUploadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CompleteMultipartUploadResult")]
    pub complete_multipart_upload_result:
        CompleteMultipartUploadResponseCompleteMultipartUploadResult,
}

impl AsRef<crate::CodeMessage> for CompleteMultipartUploadResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UploadPartCopyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CopyPartResult")]
    pub copy_part_result: UploadPartCopyResponseCopyPartResult,
}

impl AsRef<crate::CodeMessage> for UploadPartCopyResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct AbortMultipartUploadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for AbortMultipartUploadResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListMultipartUploadsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListMultipartUploadsResult")]
    pub list_multipart_uploads_result: ListMultipartUploadsResponseListMultipartUploadsResult,
}

impl AsRef<crate::CodeMessage> for ListMultipartUploadsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListPartsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListPartResult")]
    pub list_part_result: ListPartsResponseListPartResult,
}

impl AsRef<crate::CodeMessage> for ListPartsResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutObjectAclResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutObjectAclResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectAclResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "AccessControlPolicy")]
    pub access_control_policy: GetObjectAclResponseAccessControlPolicy,
}

impl AsRef<crate::CodeMessage> for GetObjectAclResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutSymlinkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutSymlinkResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetSymlinkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for GetSymlinkResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutObjectTaggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutObjectTaggingResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectTaggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Tagging")]
    pub tagging: GetObjectTaggingResponseTagging,
}

impl AsRef<crate::CodeMessage> for GetObjectTaggingResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteObjectTaggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteObjectTaggingResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutLiveChannelResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CreateLiveChannelResult")]
    pub create_live_channel_result: PutLiveChannelResponseCreateLiveChannelResult,
}

impl AsRef<crate::CodeMessage> for PutLiveChannelResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListLiveChannelResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListLiveChannelResult")]
    pub list_live_channel_result: ListLiveChannelResponseListLiveChannelResult,
}

impl AsRef<crate::CodeMessage> for ListLiveChannelResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteLiveChannelResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteLiveChannelResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutLiveChannelStatusResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutLiveChannelStatusResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "LiveChannelConfiguration")]
    pub live_channel_configuration: GetLiveChannelInfoResponseLiveChannelConfiguration,
}

impl AsRef<crate::CodeMessage> for GetLiveChannelInfoResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelHistoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "LiveChannelHistory")]
    pub live_channel_history: GetLiveChannelHistoryResponseLiveChannelHistory,
}

impl AsRef<crate::CodeMessage> for GetLiveChannelHistoryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelStatResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "LiveChannelStat")]
    pub live_channel_stat: GetLiveChannelStatResponseLiveChannelStat,
}

impl AsRef<crate::CodeMessage> for GetLiveChannelStatResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetVodPlaylistResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for GetVodPlaylistResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PostVodPlaylistResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PostVodPlaylistResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutChannelResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutChannelResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketHashResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketHashResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketCommonHeaderResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutBucketCommonHeaderResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketCommonHeaderResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for DeleteBucketCommonHeaderResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutProcessConfigurationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for PutProcessConfigurationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketEventNotificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: String,
}

impl AsRef<crate::CodeMessage> for GetBucketEventNotificationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutDataLakeCachePrefetchJobResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "DataLakeCachePrefetchJobID")]
    pub data_lake_cache_prefetch_job_id:
        PutDataLakeCachePrefetchJobResponseDataLakeCachePrefetchJobId,
}

impl AsRef<crate::CodeMessage> for PutDataLakeCachePrefetchJobResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct StartDataLakeCachePrefetchJobResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl AsRef<crate::CodeMessage> for StartDataLakeCachePrefetchJobResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListDataLakeStorageTransferJobResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "DataLakeStorageTransferJobs")]
    pub data_lake_storage_transfer_jobs: String,
}

impl AsRef<crate::CodeMessage> for ListDataLakeStorageTransferJobResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}
