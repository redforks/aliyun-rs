#[derive(Clone, Copy, Debug, strum::EnumString)]
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

impl Endpoint {
    pub fn name(self) -> &'static str {
        match self {
            Endpoint::CnWuhanLr => "cn-wuhan-lr",
            Endpoint::CnQingdao => "cn-qingdao",
            Endpoint::CnBeijing => "cn-beijing",
            Endpoint::CnZhangjiakou => "cn-zhangjiakou",
            Endpoint::CnHuhehaote => "cn-huhehaote",
            Endpoint::CnWulanchabu => "cn-wulanchabu",
            Endpoint::CnHangzhou => "cn-hangzhou",
            Endpoint::CnShanghai => "cn-shanghai",
            Endpoint::CnNanjing => "cn-nanjing",
            Endpoint::CnFuzhou => "cn-fuzhou",
            Endpoint::CnShenzhen => "cn-shenzhen",
            Endpoint::CnHeyuan => "cn-heyuan",
            Endpoint::CnGuangzhou => "cn-guangzhou",
            Endpoint::CnChengdu => "cn-chengdu",
            Endpoint::CnHongkong => "cn-hongkong",
            Endpoint::ApNortheast1 => "ap-northeast-1",
            Endpoint::ApNortheast2 => "ap-northeast-2",
            Endpoint::ApSoutheast1 => "ap-southeast-1",
            Endpoint::ApSoutheast2 => "ap-southeast-2",
            Endpoint::ApSoutheast3 => "ap-southeast-3",
            Endpoint::ApSoutheast5 => "ap-southeast-5",
            Endpoint::ApSoutheast6 => "ap-southeast-6",
            Endpoint::UsEast1 => "us-east-1",
            Endpoint::UsWest1 => "us-west-1",
            Endpoint::EuWest1 => "eu-west-1",
            Endpoint::EuCentral1 => "eu-central-1",
            Endpoint::ApSouth1 => "ap-south-1",
            Endpoint::MeEast1 => "me-east-1",
            Endpoint::CnHangzhouFinance => "cn-hangzhou-finance",
            Endpoint::CnShanghaiFinance1 => "cn-shanghai-finance-1",
            Endpoint::CnShenzhenFinance1 => "cn-shenzhen-finance-1",
            Endpoint::ApSoutheast7 => "ap-southeast-7",
            Endpoint::CnBeijingFinance1 => "cn-beijing-finance-1",
        }
    }
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
pub struct Connection(crate::common::Connection<crate::auth::Oss4HmacSha256>);

impl Connection {
    pub fn new(endpoint: Endpoint, app_key_secret: crate::v3::AccessKeySecret) -> Self {
        Self(crate::common::Connection::new(
            crate::auth::Oss4HmacSha256::new(app_key_secret, endpoint.name()),
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
            crate::auth::Oss4HmacSha256::new(app_key_secret, endpoint.name()),
            "2019-05-17",
            endpoint.into(),
            client,
        ))
    }

    fn call<R: crate::Request + sealed::Bound>(
        &self,
        req: R,
    ) -> impl std::future::Future<
        Output = crate::Result<<R::ResponseWrap as crate::IntoResponse>::Response>,
    > + Send {
        self.0.call(req)
    }
}

impl Connection {
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
    /// - GET
    ///
    pub fn list_buckets(
        &self,
        req: ListBuckets,
    ) -> impl std::future::Future<Output = crate::Result<ListBucketsResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn describe_regions(
        &self,
        req: DescribeRegions,
    ) -> impl std::future::Future<Output = crate::Result<DescribeRegionsResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_stat(
        &self,
        req: GetBucketStat,
    ) -> impl std::future::Future<Output = crate::Result<BucketStat>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket(
        &self,
        req: PutBucket,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket(
        &self,
        req: DeleteBucket,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_objects(
        &self,
        req: ListObjects,
    ) -> impl std::future::Future<Output = crate::Result<ListObjectsResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_objects_v2(
        &self,
        req: ListObjectsV2,
    ) -> impl std::future::Future<Output = crate::Result<ListObjectsV2Response>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_info(
        &self,
        req: GetBucketInfo,
    ) -> impl std::future::Future<Output = crate::Result<BucketInfo>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_location(
        &self,
        req: GetBucketLocation,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketLocationResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_access_points(
        &self,
        req: ListAccessPoints,
    ) -> impl std::future::Future<Output = crate::Result<ListAccessPointsResult>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_access_point(
        &self,
        req: GetAccessPoint,
    ) -> impl std::future::Future<Output = crate::Result<GetAccessPointResult>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_access_point_policy(
        &self,
        req: GetAccessPointPolicy,
    ) -> impl std::future::Future<Output = crate::Result<GetAccessPointPolicyResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_access_point_policy(
        &self,
        req: DeleteAccessPointPolicy,
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_access_point_policy(
        &self,
        req: PutAccessPointPolicy,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_access_point(
        &self,
        req: DeleteAccessPoint,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn create_access_point(
        &self,
        req: CreateAccessPoint,
    ) -> impl std::future::Future<Output = crate::Result<CreateAccessPointResult>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn initiate_bucket_worm(
        &self,
        req: InitiateBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<InitiateBucketWormResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn abort_bucket_worm(
        &self,
        req: AbortBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn complete_bucket_worm(
        &self,
        req: CompleteBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn extend_bucket_worm(
        &self,
        req: ExtendBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_worm(
        &self,
        req: GetBucketWorm,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketWormResponse>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_acl(
        &self,
        req: PutBucketAcl,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_acl(
        &self,
        req: GetBucketAcl,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketAclResponse>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_lifecycle(
        &self,
        req: PutBucketLifecycle,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_lifecycle(
        &self,
        req: GetBucketLifecycle,
    ) -> impl std::future::Future<Output = crate::Result<LifecycleConfiguration>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_lifecycle(
        &self,
        req: DeleteBucketLifecycle,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_transfer_acceleration(
        &self,
        req: PutBucketTransferAcceleration,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_transfer_acceleration(
        &self,
        req: GetBucketTransferAcceleration,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketTransferAccelerationResponse>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_versioning(
        &self,
        req: PutBucketVersioning,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_versioning(
        &self,
        req: GetBucketVersioning,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketVersioningResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_object_versions(
        &self,
        req: ListObjectVersions,
    ) -> impl std::future::Future<Output = crate::Result<ListObjectVersionsResponse>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_policy(
        &self,
        req: PutBucketPolicy,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_policy(
        &self,
        req: GetBucketPolicy,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketPolicyResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_policy(
        &self,
        req: DeleteBucketPolicy,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_policy_status(
        &self,
        req: GetBucketPolicyStatus,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketPolicyStatusResponse>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_rtc(
        &self,
        req: PutBucketRtc,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn put_bucket_replication(
        &self,
        req: PutBucketReplication,
    ) -> impl std::future::Future<Output = crate::Result<PutBucketReplicationResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_replication(
        &self,
        req: GetBucketReplication,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketReplicationResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_replication_location(
        &self,
        req: GetBucketReplicationLocation,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketReplicationLocationResponse>> + Send
    {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_replication_progress(
        &self,
        req: GetBucketReplicationProgress,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketReplicationProgressResponse>> + Send
    {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn delete_bucket_replication(
        &self,
        req: DeleteBucketReplication,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_inventory(
        &self,
        req: PutBucketInventory,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_inventory(
        &self,
        req: GetBucketInventory,
    ) -> impl std::future::Future<Output = crate::Result<InventoryConfiguration>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_bucket_inventory(
        &self,
        req: ListBucketInventory,
    ) -> impl std::future::Future<Output = crate::Result<ListBucketInventoryResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_inventory(
        &self,
        req: DeleteBucketInventory,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_logging(
        &self,
        req: PutBucketLogging,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_logging(
        &self,
        req: GetBucketLogging,
    ) -> impl std::future::Future<Output = crate::Result<BucketLoggingStatus>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_logging(
        &self,
        req: DeleteBucketLogging,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_user_defined_log_fields_config(
        &self,
        req: PutUserDefinedLogFieldsConfig,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_user_defined_log_fields_config(
        &self,
        req: GetUserDefinedLogFieldsConfig,
    ) -> impl std::future::Future<Output = crate::Result<UserDefinedLogFieldsConfiguration>> + Send
    {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_user_defined_log_fields_config(
        &self,
        req: DeleteUserDefinedLogFieldsConfig,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_website(
        &self,
        req: GetBucketWebsite,
    ) -> impl std::future::Future<Output = crate::Result<WebsiteConfiguration>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_website(
        &self,
        req: PutBucketWebsite,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_website(
        &self,
        req: DeleteBucketWebsite,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_referer(
        &self,
        req: PutBucketReferer,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_referer(
        &self,
        req: GetBucketReferer,
    ) -> impl std::future::Future<Output = crate::Result<RefererConfiguration>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_tags(
        &self,
        req: PutBucketTags,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_tags(
        &self,
        req: GetBucketTags,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketTagsResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_tags(
        &self,
        req: DeleteBucketTags,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_user_data_redundancy_transition(
        &self,
        req: ListUserDataRedundancyTransition,
    ) -> impl std::future::Future<Output = crate::Result<ListUserDataRedundancyTransitionResponse>> + Send
    {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_bucket_data_redundancy_transition(
        &self,
        req: ListBucketDataRedundancyTransition,
    ) -> impl std::future::Future<Output = crate::Result<ListBucketDataRedundancyTransitionResponse>>
    + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_data_redundancy_transition(
        &self,
        req: GetBucketDataRedundancyTransition,
    ) -> impl std::future::Future<Output = crate::Result<BucketDataRedundancyTransition>> + Send
    {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn create_bucket_data_redundancy_transition(
        &self,
        req: CreateBucketDataRedundancyTransition,
    ) -> impl std::future::Future<
        Output = crate::Result<CreateBucketDataRedundancyTransitionResponse>,
    > + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_data_redundancy_transition(
        &self,
        req: DeleteBucketDataRedundancyTransition,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_encryption(
        &self,
        req: PutBucketEncryption,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_encryption(
        &self,
        req: GetBucketEncryption,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketEncryptionResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_encryption(
        &self,
        req: DeleteBucketEncryption,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_request_payment(
        &self,
        req: PutBucketRequestPayment,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_request_payment(
        &self,
        req: GetBucketRequestPayment,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketRequestPaymentResponse>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_cors(
        &self,
        req: PutBucketCors,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_cors(
        &self,
        req: GetBucketCors,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketCorsResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_cors(
        &self,
        req: DeleteBucketCors,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - OPTIONS
    ///
    pub fn option_object(
        &self,
        req: OptionObject,
    ) -> impl std::future::Future<Output = crate::Result<OptionObjectResponse>> + Send {
        async {
            todo!(
                r##"Only HttpMethod::Get, HttpMethod::Post, HttpMethod::Put, HttpMethod::Delete, or HttpMethod::Head supported"##
            );
        }
    }

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
    /// - PUT
    ///
    pub fn put_bucket_access_monitor(
        &self,
        req: PutBucketAccessMonitor,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_access_monitor(
        &self,
        req: GetBucketAccessMonitor,
    ) -> impl std::future::Future<Output = crate::Result<AccessMonitorConfiguration>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_meta_query_status(
        &self,
        req: GetMetaQueryStatus,
    ) -> impl std::future::Future<Output = crate::Result<GetMetaQueryStatusResponse>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn close_meta_query(
        &self,
        req: CloseMetaQuery,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn do_meta_query(
        &self,
        req: DoMetaQuery,
    ) -> impl std::future::Future<Output = crate::Result<MetaQueryResp>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn open_meta_query(
        &self,
        req: OpenMetaQuery,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn update_user_anti_d_dos_info(
        &self,
        req: UpdateUserAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn update_bucket_anti_d_dos_info(
        &self,
        req: UpdateBucketAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_bucket_anti_d_dos_info(
        &self,
        req: ListBucketAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<ListBucketAntiDDosInfoResponse>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn init_user_anti_d_dos_info(
        &self,
        req: InitUserAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<InitUserAntiDDosInfoResponse>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn init_bucket_anti_d_dos_info(
        &self,
        req: InitBucketAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_user_anti_d_dos_info(
        &self,
        req: GetUserAntiDDosInfo,
    ) -> impl std::future::Future<Output = crate::Result<GetUserAntiDDosInfoResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_resource_group(
        &self,
        req: GetBucketResourceGroup,
    ) -> impl std::future::Future<Output = crate::Result<GetBucketResourceGroupResponse>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_resource_group(
        &self,
        req: PutBucketResourceGroup,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn put_cname(
        &self,
        req: PutCname,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_cname(
        &self,
        req: ListCname,
    ) -> impl std::future::Future<Output = crate::Result<ListCnameResponse>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn delete_cname(
        &self,
        req: DeleteCname,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_cname_token(
        &self,
        req: GetCnameToken,
    ) -> impl std::future::Future<Output = crate::Result<CnameToken>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn create_cname_token(
        &self,
        req: CreateCnameToken,
    ) -> impl std::future::Future<Output = crate::Result<CnameToken>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_style(
        &self,
        req: PutStyle,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_style(
        &self,
        req: ListStyle,
    ) -> impl std::future::Future<Output = crate::Result<ListStyleResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_style(
        &self,
        req: GetStyle,
    ) -> impl std::future::Future<Output = crate::Result<StyleInfo>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_style(
        &self,
        req: DeleteStyle,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_https_config(
        &self,
        req: GetBucketHttpsConfig,
    ) -> impl std::future::Future<Output = crate::Result<HttpsConfiguration>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_https_config(
        &self,
        req: PutBucketHttpsConfig,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn create_access_point_for_object_process(
        &self,
        req: CreateAccessPointForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<CreateAccessPointForObjectProcessResponse>> + Send
    {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_access_point_for_object_process(
        &self,
        req: GetAccessPointForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<GetAccessPointForObjectProcessResponse>> + Send
    {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_access_points_for_object_process(
        &self,
        req: ListAccessPointsForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<ListAccessPointsForObjectProcessResponse>> + Send
    {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_access_point_for_object_process(
        &self,
        req: DeleteAccessPointForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_access_point_config_for_object_process(
        &self,
        req: GetAccessPointConfigForObjectProcess,
    ) -> impl std::future::Future<
        Output = crate::Result<GetAccessPointConfigForObjectProcessResponse>,
    > + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_access_point_config_for_object_process(
        &self,
        req: PutAccessPointConfigForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_access_point_policy_for_object_process(
        &self,
        req: PutAccessPointPolicyForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_access_point_policy_for_object_process(
        &self,
        req: GetAccessPointPolicyForObjectProcess,
    ) -> impl std::future::Future<
        Output = crate::Result<GetAccessPointPolicyForObjectProcessResponse>,
    > + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_access_point_policy_for_object_process(
        &self,
        req: DeleteAccessPointPolicyForObjectProcess,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_public_access_block(
        &self,
        req: GetPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<PublicAccessBlockConfiguration>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_public_access_block(
        &self,
        req: PutPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_public_access_block(
        &self,
        req: DeletePublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_public_access_block(
        &self,
        req: GetBucketPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<PublicAccessBlockConfiguration>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_public_access_block(
        &self,
        req: PutBucketPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_public_access_block(
        &self,
        req: DeleteBucketPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_access_point_public_access_block(
        &self,
        req: GetAccessPointPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<PublicAccessBlockConfiguration>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_access_point_public_access_block(
        &self,
        req: PutAccessPointPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<PutAccessPointPublicAccessBlockResponse>> + Send
    {
        async {
            todo!(r##"API must have 200 or 204 response"##);
        }
    }

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
    /// - DELETE
    ///
    pub fn delete_access_point_public_access_block(
        &self,
        req: DeleteAccessPointPublicAccessBlock,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_archive_direct_read(
        &self,
        req: GetBucketArchiveDirectRead,
    ) -> impl std::future::Future<Output = crate::Result<ArchiveDirectReadConfiguration>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_archive_direct_read(
        &self,
        req: PutBucketArchiveDirectRead,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_overwrite_config(
        &self,
        req: PutBucketOverwriteConfig,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_overwrite_config(
        &self,
        req: GetBucketOverwriteConfig,
    ) -> impl std::future::Future<Output = crate::Result<OverwriteConfiguration>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_overwrite_config(
        &self,
        req: DeleteBucketOverwriteConfig,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_object(
        &self,
        req: PutObject,
    ) -> impl std::future::Future<Output = crate::Result<PutObjectResponse>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn copy_object(
        &self,
        req: CopyObject,
    ) -> impl std::future::Future<Output = crate::Result<CopyObjectResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_object(
        &self,
        req: GetObject,
    ) -> impl std::future::Future<Output = crate::Result<GetObjectResponse>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn append_object(
        &self,
        req: AppendObject,
    ) -> impl std::future::Future<Output = crate::Result<AppendObjectResponse>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn seal_append_object(
        &self,
        req: SealAppendObject,
    ) -> impl std::future::Future<Output = crate::Result<SealAppendObjectResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_object(
        &self,
        req: DeleteObject,
    ) -> impl std::future::Future<Output = crate::Result<DeleteObjectResponse>> + Send {
        self.call(req)
    }

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
    /// - HEAD
    ///
    pub fn head_object(
        &self,
        req: HeadObject,
    ) -> impl std::future::Future<Output = crate::Result<HeadObjectResponse>> + Send {
        self.call(req)
    }

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
    /// - HEAD
    ///
    pub fn get_object_meta(
        &self,
        req: GetObjectMeta,
    ) -> impl std::future::Future<Output = crate::Result<GetObjectMetaResponse>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn restore_object(
        &self,
        req: RestoreObject,
    ) -> impl std::future::Future<Output = crate::Result<RestoreObjectResponse>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn clean_restored_object(
        &self,
        req: CleanRestoredObject,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn select_object(
        &self,
        req: SelectObject,
    ) -> impl std::future::Future<Output = crate::Result<Vec<u8>>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn create_select_object_meta(
        &self,
        req: CreateSelectObjectMeta,
    ) -> impl std::future::Future<Output = crate::Result<SelectMetaStatus>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn initiate_multipart_upload(
        &self,
        req: InitiateMultipartUpload,
    ) -> impl std::future::Future<Output = crate::Result<InitiateMultipartUploadResponse>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn upload_part(
        &self,
        req: UploadPart,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn complete_multipart_upload(
        &self,
        req: CompleteMultipartUpload,
    ) -> impl std::future::Future<Output = crate::Result<CompleteMultipartUploadResponse>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn upload_part_copy(
        &self,
        req: UploadPartCopy,
    ) -> impl std::future::Future<Output = crate::Result<UploadPartCopyResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn abort_multipart_upload(
        &self,
        req: AbortMultipartUpload,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_multipart_uploads(
        &self,
        req: ListMultipartUploads,
    ) -> impl std::future::Future<Output = crate::Result<ListMultipartUploadsResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_parts(
        &self,
        req: ListParts,
    ) -> impl std::future::Future<Output = crate::Result<ListPartsResponse>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_object_acl(
        &self,
        req: PutObjectAcl,
    ) -> impl std::future::Future<Output = crate::Result<PutObjectAclResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_object_acl(
        &self,
        req: GetObjectAcl,
    ) -> impl std::future::Future<Output = crate::Result<GetObjectAclResponse>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_symlink(
        &self,
        req: PutSymlink,
    ) -> impl std::future::Future<Output = crate::Result<PutSymlinkResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_symlink(
        &self,
        req: GetSymlink,
    ) -> impl std::future::Future<Output = crate::Result<GetSymlinkResponse>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_object_tagging(
        &self,
        req: PutObjectTagging,
    ) -> impl std::future::Future<Output = crate::Result<PutObjectTaggingResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_object_tagging(
        &self,
        req: GetObjectTagging,
    ) -> impl std::future::Future<Output = crate::Result<GetObjectTaggingResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_object_tagging(
        &self,
        req: DeleteObjectTagging,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_live_channel(
        &self,
        req: PutLiveChannel,
    ) -> impl std::future::Future<Output = crate::Result<PutLiveChannelResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_live_channel(
        &self,
        req: ListLiveChannel,
    ) -> impl std::future::Future<Output = crate::Result<ListLiveChannelResponse>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_live_channel(
        &self,
        req: DeleteLiveChannel,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_live_channel_status(
        &self,
        req: PutLiveChannelStatus,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_live_channel_info(
        &self,
        req: GetLiveChannelInfo,
    ) -> impl std::future::Future<Output = crate::Result<GetLiveChannelInfoResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_live_channel_history(
        &self,
        req: GetLiveChannelHistory,
    ) -> impl std::future::Future<Output = crate::Result<GetLiveChannelHistoryResponse>> + Send
    {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_live_channel_stat(
        &self,
        req: GetLiveChannelStat,
    ) -> impl std::future::Future<Output = crate::Result<GetLiveChannelStatResponse>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_vod_playlist(
        &self,
        req: GetVodPlaylist,
    ) -> impl std::future::Future<Output = crate::Result<Vec<u8>>> + Send {
        self.call(req)
    }

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
    /// - POST
    ///
    pub fn post_vod_playlist(
        &self,
        req: PostVodPlaylist,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_channel(
        &self,
        req: PutChannel,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_hash(
        &self,
        req: PutBucketHash,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_bucket_common_header(
        &self,
        req: PutBucketCommonHeader,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - DELETE
    ///
    pub fn delete_bucket_common_header(
        &self,
        req: DeleteBucketCommonHeader,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_process_configuration(
        &self,
        req: PutProcessConfiguration,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn get_bucket_event_notification(
        &self,
        req: GetBucketEventNotification,
    ) -> impl std::future::Future<Output = crate::Result<EventNotificationConfiguration>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn put_data_lake_cache_prefetch_job(
        &self,
        req: PutDataLakeCachePrefetchJob,
    ) -> impl std::future::Future<Output = crate::Result<PutDataLakeCachePrefetchJobResponse>> + Send
    {
        self.call(req)
    }

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
    /// - PUT
    ///
    pub fn start_data_lake_cache_prefetch_job(
        &self,
        req: StartDataLakeCachePrefetchJob,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

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
    /// - GET
    ///
    pub fn list_data_lake_storage_transfer_job(
        &self,
        req: ListDataLakeStorageTransferJob,
    ) -> impl std::future::Future<Output = crate::Result<DataLakeStorageTransferJobs>> + Send {
        self.call(req)
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
    /// Bucket所属资源组Id
    #[setters(generate = true, strip_option)]
    x_oss_resource_group_id: Option<String>,
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
            x_oss_resource_group_id: None,
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

    type ResponseWrap = crate::XmlResponseWrap<ListBucketsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(6);

        if let Some(f) = &self.marker {
            params.push(("marker".into(), (f).into()));
        }

        if let Some(f) = &self.max_keys {
            params.push(("max-keys".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        if let Some(f) = &self.tag_key {
            params.push(("tag-key".into(), (f).into()));
        }

        if let Some(f) = &self.tag_value {
            params.push(("tag-value".into(), (f).into()));
        }

        if let Some(f) = &self.tagging {
            params.push(("tagging".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);

        if let Some(f) = &self.x_oss_resource_group_id {
            headers.push(("x-oss-resource-group-id".into(), f.to_string()));
        }

        headers
    }

    fn to_body(self) -> Self::Body {}
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

    type ResponseWrap = crate::XmlResponseWrap<DescribeRegionsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("regions".into(), "".into()));

        if let Some(f) = &self.regions {
            params.push(("regions".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketStat {
    /// 要查询的Bucket
    bucket: String,
}

impl sealed::Bound for GetBucketStat {}

impl GetBucketStat {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketStat {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketStat";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<BucketStat>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("stat".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucket {
    /// Bucket名称。存储空间的命名规范如下：
    ///
    /// - 只能包括小写字母、数字和短划线（-）。
    /// - 必须以小写字母或者数字开头和结尾。
    /// - 长度必须在3~63字符之间。
    bucket: String,
    /// 指定Bucket的访问权限ACL。包含如下三种访问权限：
    ///
    /// - public-read-write：公共读写
    ///
    /// - public-read：公共读
    ///
    /// - private：私有（默认值）
    ///
    /// 关于Bucket访问权限ACL的更多信息，请参见[设置存储空间访问权限ACL](~~31843~~)。
    #[setters(generate = true, strip_option)]
    x_oss_acl: Option<BucketACL>,
    /// 指定资源组ID。
    /// - 如果在请求中携带该请求头并指定资源组ID，则创建的存储空间属于该资源组。当指定的资源组ID为rg-default-id时，创建的存储空间属于默认资源组。
    /// - 如果在请求中未携带该请求头，则创建的存储空间属于默认资源组。
    ///
    /// 您可以通过资源管理的控制台或ListResourceGroups接口获取资源组ID。具体操作，请分别参见[查看资源组基本信息](~~151181~~)和[ListResourceGroups](~~158855~~)。
    /// > 创建无地域属性Bucket时不支持配置资源组。
    #[setters(generate = true, strip_option)]
    x_oss_resource_group_id: Option<String>,
    /// 指定Bucket标签，如 k1=v1&k2=v2。
    #[setters(generate = true, strip_option)]
    x_oss_bucket_tagging: Option<String>,
    /// 存储创建Bucket信息的容器。
    #[setters(generate = true, strip_option)]
    body: Option<CreateBucketConfiguration>,
}

impl sealed::Bound for PutBucket {}

impl PutBucket {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_acl: None,
            x_oss_resource_group_id: None,
            x_oss_bucket_tagging: None,
            body: None,
        }
    }
}

impl crate::Request for PutBucket {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucket";

    type Body = crate::XmlBody<CreateBucketConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(3);

        if let Some(f) = &self.x_oss_acl {
            headers.push(("x-oss-acl".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_bucket_tagging {
            headers.push(("x-oss-bucket-tagging".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_resource_group_id {
            headers.push(("x-oss-resource-group-id".into(), f.to_string()));
        }

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucket {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for DeleteBucket {}

impl DeleteBucket {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucket {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucket {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucket";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListObjects {
    /// Bucket名称。
    bucket: String,
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
    encoding_type: Option<EncodeType>,
}

impl sealed::Bound for ListObjects {}

impl ListObjects {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
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

    type ResponseWrap = crate::XmlResponseWrap<ListObjectsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);

        if let Some(f) = &self.delimiter {
            params.push(("delimiter".into(), (f).into()));
        }

        if let Some(f) = &self.encoding_type {
            params.push(("encoding-type".into(), (f).into()));
        }

        if let Some(f) = &self.marker {
            params.push(("marker".into(), (f).into()));
        }

        if let Some(f) = &self.max_keys {
            params.push(("max-keys".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListObjectsV2 {
    /// Bucket名称。
    bucket: String,
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
    encoding_type: Option<EncodeType>,
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
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
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

    type ResponseWrap = crate::XmlResponseWrap<ListObjectsV2Response>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(8);
        params.push(("list-type".into(), "2".into()));

        if let Some(f) = &self.continuation_token {
            params.push(("continuation-token".into(), (f).into()));
        }

        if let Some(f) = &self.delimiter {
            params.push(("delimiter".into(), (f).into()));
        }

        if let Some(f) = &self.encoding_type {
            params.push(("encoding-type".into(), (f).into()));
        }

        if let Some(f) = &self.fetch_owner {
            params.push(("fetch-owner".into(), (f).into()));
        }

        if let Some(f) = &self.max_keys {
            params.push(("max-keys".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        if let Some(f) = &self.start_after {
            params.push(("start-after".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketInfo {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketInfo {}

impl GetBucketInfo {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketInfo";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<BucketInfo>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("bucketInfo".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketLocation {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketLocation {}

impl GetBucketLocation {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketLocation {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketLocation";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketLocationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("location".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
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

    type ResponseWrap = crate::XmlResponseWrap<ListAccessPointsResult>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("accessPoint".into(), "".into()));

        if let Some(f) = &self.continuation_token {
            params.push(("continuation-token".into(), (f).into()));
        }

        if let Some(f) = &self.max_keys {
            params.push(("max-keys".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPoint {
    /// Bucket名称。
    bucket: String,
    /// 接入点名称。
    x_oss_access_point_name: String,
}

impl sealed::Bound for GetAccessPoint {}

impl GetAccessPoint {
    pub fn new(bucket: impl Into<String>, x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_name: x_oss_access_point_name.into(),
        }
    }
}

impl crate::Request for GetAccessPoint {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPoint";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetAccessPointResult>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPoint".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-name".into(),
            self.x_oss_access_point_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointPolicy {
    /// Bucket名称。
    bucket: String,
    /// 接入点名称。
    x_oss_access_point_name: String,
}

impl sealed::Bound for GetAccessPointPolicy {}

impl GetAccessPointPolicy {
    pub fn new(bucket: impl Into<String>, x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_name: x_oss_access_point_name.into(),
        }
    }
}

impl crate::Request for GetAccessPointPolicy {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointPolicy";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<GetAccessPointPolicyResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointPolicy".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-name".into(),
            self.x_oss_access_point_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointPolicy {
    /// Bucket名称。
    bucket: String,
    /// 接入点名称。
    x_oss_access_point_name: String,
}

impl sealed::Bound for DeleteAccessPointPolicy {}

impl DeleteAccessPointPolicy {
    pub fn new(bucket: impl Into<String>, x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_name: x_oss_access_point_name.into(),
        }
    }
}
impl crate::ToFormData for DeleteAccessPointPolicy {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteAccessPointPolicy {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteAccessPointPolicy";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointPolicy".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-name".into(),
            self.x_oss_access_point_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointPolicy {
    /// Bucket名称。
    bucket: String,
    /// 接入点名称。
    #[setters(generate = true, strip_option)]
    x_oss_access_point_name: Option<String>,
    /// 接入点策略配置内容。
    #[setters(generate = true, strip_option)]
    body: Option<String>,
}

impl sealed::Bound for PutAccessPointPolicy {}

impl PutAccessPointPolicy {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_name: None,
            body: None,
        }
    }
}

impl crate::Request for PutAccessPointPolicy {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAccessPointPolicy";

    type Body = crate::JsonBody<String>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointPolicy".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);

        if let Some(f) = &self.x_oss_access_point_name {
            headers.push(("x-oss-access-point-name".into(), f.to_string()));
        }

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPoint {
    /// Bucket名称。
    bucket: String,
    /// 接入点名称。
    x_oss_access_point_name: String,
}

impl sealed::Bound for DeleteAccessPoint {}

impl DeleteAccessPoint {
    pub fn new(bucket: impl Into<String>, x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_name: x_oss_access_point_name.into(),
        }
    }
}
impl crate::ToFormData for DeleteAccessPoint {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteAccessPoint {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteAccessPoint";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPoint".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-name".into(),
            self.x_oss_access_point_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateAccessPoint {
    /// Bucket名称。
    bucket: String,
    /// 保存接入点信息的容器。
    #[setters(generate = true, strip_option)]
    body: Option<CreateAccessPointConfiguration>,
}

impl sealed::Bound for CreateAccessPoint {}

impl CreateAccessPoint {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for CreateAccessPoint {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "CreateAccessPoint";

    type Body = crate::XmlBody<CreateAccessPointConfiguration>;

    type ResponseWrap = crate::XmlResponseWrap<CreateAccessPointResult>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPoint".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct InitiateBucketWorm {
    /// Bucket名称。
    bucket: String,
    /// 请求体。
    #[setters(generate = true, strip_option)]
    body: Option<InitiateWormConfiguration>,
}

impl sealed::Bound for InitiateBucketWorm {}

impl InitiateBucketWorm {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for InitiateBucketWorm {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "InitiateBucketWorm";

    type Body = crate::XmlBody<InitiateWormConfiguration>;

    type ResponseWrap = crate::XmlResponseWrap<InitiateBucketWormResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("worm".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-worm-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_worm_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AbortBucketWorm {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for AbortBucketWorm {}

impl AbortBucketWorm {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for AbortBucketWorm {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for AbortBucketWorm {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "AbortBucketWorm";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("worm".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CompleteBucketWorm {
    /// Bucket名称。
    bucket: String,
    /// 合规保留策略的ID。
    worm_id: String,
}

impl sealed::Bound for CompleteBucketWorm {}

impl CompleteBucketWorm {
    pub fn new(bucket: impl Into<String>, worm_id: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            worm_id: worm_id.into(),
        }
    }
}
impl crate::ToFormData for CompleteBucketWorm {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CompleteBucketWorm {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CompleteBucketWorm";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("wormId".into(), (&self.worm_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ExtendBucketWorm {
    /// Bucket名称。
    bucket: String,
    /// 合规保留策略的ID。
    ///
    /// > 如果指定用于延长Object保留天数对应的合规保留策略ID不存在，则返回404。
    worm_id: String,
    /// 保存合规保留策略的容器。
    #[setters(generate = true, strip_option)]
    body: Option<ExtendWormConfiguration>,
}

impl sealed::Bound for ExtendBucketWorm {}

impl ExtendBucketWorm {
    pub fn new(bucket: impl Into<String>, worm_id: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            worm_id: worm_id.into(),
            body: None,
        }
    }
}

impl crate::Request for ExtendBucketWorm {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "ExtendBucketWorm";

    type Body = crate::XmlBody<ExtendWormConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("wormExtend".into(), "".into()));
        params.push(("wormId".into(), (&self.worm_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketWorm {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketWorm {}

impl GetBucketWorm {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketWorm {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketWorm";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketWormResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("worm".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketAcl {
    /// Bucket名称。
    bucket: String,
    /// 设置Bucket的访问权限ACL。PutBucketAcl接口通过Put请求中的x-oss-acl请求头来设置访问权限，如果没有该请求头，则访问权限设置不生效。
    /// <br>取值如下：
    ///
    /// - public-read-write（公共读写）：所有用户都有该Bucket内的文件的读写权限。请谨慎使用该访问权限。
    /// - public-read（公共读）：Bucket的拥有者和授权用户有该Bucket内的文件的读写权限，其他用户只有该Bucket内的文件的读权限。请谨慎使用该访问权限。
    /// - private：Bucket的拥有者和授权用户有该Bucket内的文件的读写权限，其他用户没有权限操作该Bucket内的文件。
    x_oss_acl: BucketACL,
}

impl sealed::Bound for PutBucketAcl {}

impl PutBucketAcl {
    pub fn new(bucket: impl Into<String>, x_oss_acl: impl Into<BucketACL>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_acl: x_oss_acl.into(),
        }
    }
}
impl crate::ToFormData for PutBucketAcl {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for PutBucketAcl {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketAcl";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("acl".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push(("x-oss-acl".into(), self.x_oss_acl.to_string()));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketAcl {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketAcl {}

impl GetBucketAcl {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketAcl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketAcl";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketAclResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("acl".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketLifecycle {
    /// Bucket名称。
    bucket: String,
    /// 指定生命周期规则是否允许前缀重叠。取值如下：
    ///
    /// true：允许前缀重叠。
    ///
    /// false：不允许前缀重叠。
    #[setters(generate = true, strip_option)]
    x_oss_allow_same_action_overlap: Option<String>,
    /// 保存Lifecycle配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<LifecycleConfiguration>,
}

impl sealed::Bound for PutBucketLifecycle {}

impl PutBucketLifecycle {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_allow_same_action_overlap: None,
            body: None,
        }
    }
}

impl crate::Request for PutBucketLifecycle {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketLifecycle";

    type Body = crate::XmlBody<LifecycleConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("lifecycle".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);

        if let Some(f) = &self.x_oss_allow_same_action_overlap {
            headers.push(("x-oss-allow-same-action-overlap".into(), f.to_string()));
        }

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketLifecycle {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketLifecycle {}

impl GetBucketLifecycle {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketLifecycle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketLifecycle";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<LifecycleConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("lifecycle".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketLifecycle {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for DeleteBucketLifecycle {}

impl DeleteBucketLifecycle {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketLifecycle {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketLifecycle {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketLifecycle";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("lifecycle".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketTransferAcceleration {
    /// Bucket名称。
    bucket: String,
    /// 传输加速配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<TransferAccelerationConfiguration>,
}

impl sealed::Bound for PutBucketTransferAcceleration {}

impl PutBucketTransferAcceleration {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketTransferAcceleration {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketTransferAcceleration";

    type Body = crate::XmlBody<TransferAccelerationConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("transferAcceleration".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketTransferAcceleration {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketTransferAcceleration {}

impl GetBucketTransferAcceleration {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketTransferAcceleration {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketTransferAcceleration";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketTransferAccelerationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("transferAcceleration".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketVersioning {
    /// Bucket名称。
    bucket: String,
    /// 请求体。
    #[setters(generate = true, strip_option)]
    body: Option<VersioningConfiguration>,
}

impl sealed::Bound for PutBucketVersioning {}

impl PutBucketVersioning {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketVersioning {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketVersioning";

    type Body = crate::XmlBody<VersioningConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("versioning".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketVersioning {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketVersioning {}

impl GetBucketVersioning {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketVersioning {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketVersioning";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketVersioningResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("versioning".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListObjectVersions {
    /// Bucket名称。
    bucket: String,
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
    encoding_type: Option<EncodeType>,
}

impl sealed::Bound for ListObjectVersions {}

impl ListObjectVersions {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
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

    type ResponseWrap = crate::XmlResponseWrap<ListObjectVersionsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(7);
        params.push(("versions".into(), "".into()));

        if let Some(f) = &self.delimiter {
            params.push(("delimiter".into(), (f).into()));
        }

        if let Some(f) = &self.encoding_type {
            params.push(("encoding-type".into(), (f).into()));
        }

        if let Some(f) = &self.key_marker {
            params.push(("key-marker".into(), (f).into()));
        }

        if let Some(f) = &self.max_keys {
            params.push(("max-keys".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        if let Some(f) = &self.version_id_marker {
            params.push(("version-id-marker".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketPolicy {
    /// Bucket名称。
    bucket: String,
    /// 请求体。
    body: Vec<u8>,
}

impl sealed::Bound for PutBucketPolicy {}

impl PutBucketPolicy {
    pub fn new(bucket: impl Into<String>, body: impl Into<Vec<u8>>) -> Self {
        Self {
            bucket: bucket.into(),
            body: body.into(),
        }
    }
}

impl crate::Request for PutBucketPolicy {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketPolicy";

    type Body = crate::OctetStream;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("policy".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketPolicy {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketPolicy {}

impl GetBucketPolicy {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketPolicy {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketPolicy";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<GetBucketPolicyResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("policy".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketPolicy {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for DeleteBucketPolicy {}

impl DeleteBucketPolicy {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketPolicy {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketPolicy {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketPolicy";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("policy".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketPolicyStatus {
    /// 存储空间名称。
    bucket: String,
}

impl sealed::Bound for GetBucketPolicyStatus {}

impl GetBucketPolicyStatus {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketPolicyStatus {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketPolicyStatus";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketPolicyStatusResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("policyStatus".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketRtc {
    /// Bucket名称。
    bucket: String,
    /// 保存RTC配置规则的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<RtcConfiguration>,
}

impl sealed::Bound for PutBucketRtc {}

impl PutBucketRtc {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketRtc {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketRtc";

    type Body = crate::XmlBody<RtcConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("rtc".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketReplication {
    /// Bucket名称。
    bucket: String,
    /// 指定数据复制配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<PutBucketReplicationbody>,
}

impl sealed::Bound for PutBucketReplication {}

impl PutBucketReplication {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketReplication {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutBucketReplication";

    type Body = crate::XmlBody<PutBucketReplicationbody>;

    type ResponseWrap = crate::XmlResponseWrap<PutBucketReplicationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("replication".into(), "".into()));
        params.push(("comp".into(), "add".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-replication-rule-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_replication_rule_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketReplication {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketReplication {}

impl GetBucketReplication {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketReplication {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketReplication";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketReplicationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("replication".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketReplicationLocation {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketReplicationLocation {}

impl GetBucketReplicationLocation {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketReplicationLocation {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketReplicationLocation";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketReplicationLocationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("replicationLocation".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketReplicationProgress {
    /// Bucket名称。
    bucket: String,
    /// 复制规则对应的ID。此ID可从GetBucketReplication中获取。
    rule_id: String,
}

impl sealed::Bound for GetBucketReplicationProgress {}

impl GetBucketReplicationProgress {
    pub fn new(bucket: impl Into<String>, rule_id: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            rule_id: rule_id.into(),
        }
    }
}

impl crate::Request for GetBucketReplicationProgress {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketReplicationProgress";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketReplicationProgressResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("replicationProgress".into(), "".into()));
        params.push(("rule-id".into(), (&self.rule_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketReplication {
    /// Bucket名称。
    bucket: String,
    /// 保存需要删除的数据复制规则的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<DeleteBucketReplicationbody>,
}

impl sealed::Bound for DeleteBucketReplication {}

impl DeleteBucketReplication {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for DeleteBucketReplication {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DeleteBucketReplication";

    type Body = crate::XmlBody<DeleteBucketReplicationbody>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("replication".into(), "".into()));
        params.push(("comp".into(), "delete".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketInventory {
    /// Bucket名称。
    bucket: String,
    /// 配置的清单规则Id。
    inventory_id: String,
    /// 存储清单配置信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<InventoryConfiguration>,
}

impl sealed::Bound for PutBucketInventory {}

impl PutBucketInventory {
    pub fn new(bucket: impl Into<String>, inventory_id: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            inventory_id: inventory_id.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketInventory {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketInventory";

    type Body = crate::XmlBody<InventoryConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("inventory".into(), "".into()));
        params.push(("inventoryId".into(), (&self.inventory_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketInventory {
    /// Bucket名称。
    bucket: String,
    /// 查询的清单规则Id。
    inventory_id: String,
}

impl sealed::Bound for GetBucketInventory {}

impl GetBucketInventory {
    pub fn new(bucket: impl Into<String>, inventory_id: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            inventory_id: inventory_id.into(),
        }
    }
}

impl crate::Request for GetBucketInventory {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketInventory";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<InventoryConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("inventory".into(), "".into()));
        params.push(("inventoryId".into(), (&self.inventory_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListBucketInventory {
    /// Bucket名称。
    bucket: String,
    /// 指定List操作需要从此token开始。您可从ListBucketInventory结果中的NextContinuationToken获取此token。
    #[setters(generate = true, strip_option)]
    continuation_token: Option<String>,
}

impl sealed::Bound for ListBucketInventory {}

impl ListBucketInventory {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            continuation_token: None,
        }
    }
}

impl crate::Request for ListBucketInventory {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListBucketInventory";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListBucketInventoryResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("inventory".into(), "".into()));

        if let Some(f) = &self.continuation_token {
            params.push(("continuation-token".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketInventory {
    /// Bucket名称。
    bucket: String,
    /// 删除的清单任务Id。
    inventory_id: String,
}

impl sealed::Bound for DeleteBucketInventory {}

impl DeleteBucketInventory {
    pub fn new(bucket: impl Into<String>, inventory_id: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            inventory_id: inventory_id.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketInventory {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketInventory {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketInventory";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("inventory".into(), "".into()));
        params.push(("inventoryId".into(), (&self.inventory_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketLogging {
    /// Bucket名称。
    bucket: String,
    /// 存储访问日志状态信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketLoggingStatus>,
}

impl sealed::Bound for PutBucketLogging {}

impl PutBucketLogging {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketLogging {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketLogging";

    type Body = crate::XmlBody<BucketLoggingStatus>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("logging".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketLogging {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketLogging {}

impl GetBucketLogging {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketLogging {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketLogging";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<BucketLoggingStatus>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("logging".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketLogging {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for DeleteBucketLogging {}

impl DeleteBucketLogging {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketLogging {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketLogging {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketLogging";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("logging".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutUserDefinedLogFieldsConfig {
    /// 存储空间名称。
    bucket: String,
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<UserDefinedLogFieldsConfiguration>,
}

impl sealed::Bound for PutUserDefinedLogFieldsConfig {}

impl PutUserDefinedLogFieldsConfig {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutUserDefinedLogFieldsConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutUserDefinedLogFieldsConfig";

    type Body = crate::XmlBody<UserDefinedLogFieldsConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("userDefinedLogFieldsConfig".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetUserDefinedLogFieldsConfig {
    /// 存储空间名称。
    bucket: String,
}

impl sealed::Bound for GetUserDefinedLogFieldsConfig {}

impl GetUserDefinedLogFieldsConfig {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetUserDefinedLogFieldsConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetUserDefinedLogFieldsConfig";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<UserDefinedLogFieldsConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("userDefinedLogFieldsConfig".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteUserDefinedLogFieldsConfig {
    /// 存储空间名称。
    bucket: String,
}

impl sealed::Bound for DeleteUserDefinedLogFieldsConfig {}

impl DeleteUserDefinedLogFieldsConfig {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteUserDefinedLogFieldsConfig {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteUserDefinedLogFieldsConfig {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteUserDefinedLogFieldsConfig";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("userDefinedLogFieldsConfig".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketWebsite {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketWebsite {}

impl GetBucketWebsite {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketWebsite {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketWebsite";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<WebsiteConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("website".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketWebsite {
    /// Bucket名称。
    bucket: String,
    /// 保存静态网站配置的容器。
    #[setters(generate = true, strip_option)]
    body: Option<WebsiteConfiguration>,
}

impl sealed::Bound for PutBucketWebsite {}

impl PutBucketWebsite {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketWebsite {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketWebsite";

    type Body = crate::XmlBody<WebsiteConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("website".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketWebsite {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for DeleteBucketWebsite {}

impl DeleteBucketWebsite {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketWebsite {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketWebsite {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketWebsite";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("website".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketReferer {
    /// Bucket名称。
    bucket: String,
    /// 保存Referer配置内容的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<RefererConfiguration>,
}

impl sealed::Bound for PutBucketReferer {}

impl PutBucketReferer {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketReferer {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketReferer";

    type Body = crate::XmlBody<RefererConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("referer".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketReferer {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketReferer {}

impl GetBucketReferer {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketReferer {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketReferer";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<RefererConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("referer".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketTags {
    /// Bucket名称。
    bucket: String,
    /// 设置Bucket TagSet的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Tagging>,
}

impl sealed::Bound for PutBucketTags {}

impl PutBucketTags {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketTags {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketTags";

    type Body = crate::XmlBody<Tagging>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("tagging".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketTags {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketTags {}

impl GetBucketTags {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketTags {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketTags";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketTagsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("tagging".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketTags {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for DeleteBucketTags {}

impl DeleteBucketTags {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketTags {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketTags {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketTags";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("tagging".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
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

    type ResponseWrap = crate::XmlResponseWrap<ListUserDataRedundancyTransitionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("redundancyTransition".into(), "".into()));

        if let Some(f) = &self.continuation_token {
            params.push(("continuation-token".into(), (f).into()));
        }

        if let Some(f) = &self.max_keys {
            params.push(("max-keys".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListBucketDataRedundancyTransition {
    /// 存储空间名称。
    bucket: String,
}

impl sealed::Bound for ListBucketDataRedundancyTransition {}

impl ListBucketDataRedundancyTransition {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for ListBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListBucketDataRedundancyTransition";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListBucketDataRedundancyTransitionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("redundancyTransition".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketDataRedundancyTransition {
    /// 存储空间名称。
    bucket: String,
    /// 存储转换任务的ID。
    x_oss_redundancy_transition_taskid: String,
}

impl sealed::Bound for GetBucketDataRedundancyTransition {}

impl GetBucketDataRedundancyTransition {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_redundancy_transition_taskid: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_redundancy_transition_taskid: x_oss_redundancy_transition_taskid.into(),
        }
    }
}

impl crate::Request for GetBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketDataRedundancyTransition";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<BucketDataRedundancyTransition>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("redundancyTransition".into(), "".into()));
        params.push((
            "x-oss-redundancy-transition-taskid".into(),
            (&self.x_oss_redundancy_transition_taskid).into(),
        ));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateBucketDataRedundancyTransition {
    /// 要创建存储冗余类型转换任务的Bucket。
    bucket: String,
    /// 目标存储冗余类型。OSS仅支持将LRS（本地冗余存储）转换为ZRS（同城冗余存储）。
    x_oss_target_redundancy_type: String,
}

impl sealed::Bound for CreateBucketDataRedundancyTransition {}

impl CreateBucketDataRedundancyTransition {
    pub fn new(bucket: impl Into<String>, x_oss_target_redundancy_type: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_target_redundancy_type: x_oss_target_redundancy_type.into(),
        }
    }
}
impl crate::ToFormData for CreateBucketDataRedundancyTransition {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CreateBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateBucketDataRedundancyTransition";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<CreateBucketDataRedundancyTransitionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("redundancyTransition".into(), "".into()));
        params.push((
            "x-oss-target-redundancy-type".into(),
            (&self.x_oss_target_redundancy_type).into(),
        ));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketDataRedundancyTransition {
    /// 要删除存储冗余类型转换任务的Bucket。
    bucket: String,
    /// 存储冗余转换任务的ID。
    x_oss_redundancy_transition_taskid: String,
}

impl sealed::Bound for DeleteBucketDataRedundancyTransition {}

impl DeleteBucketDataRedundancyTransition {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_redundancy_transition_taskid: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_redundancy_transition_taskid: x_oss_redundancy_transition_taskid.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketDataRedundancyTransition {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketDataRedundancyTransition";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("redundancyTransition".into(), "".into()));
        params.push((
            "x-oss-redundancy-transition-taskid".into(),
            (&self.x_oss_redundancy_transition_taskid).into(),
        ));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketEncryption {
    /// Bucket名称。
    bucket: String,
    /// 配置服务器端加密规则的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<ServerSideEncryptionRule>,
}

impl sealed::Bound for PutBucketEncryption {}

impl PutBucketEncryption {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketEncryption {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketEncryption";

    type Body = crate::XmlBody<ServerSideEncryptionRule>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("encryption".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketEncryption {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketEncryption {}

impl GetBucketEncryption {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketEncryption {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketEncryption";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketEncryptionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("encryption".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketEncryption {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for DeleteBucketEncryption {}

impl DeleteBucketEncryption {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketEncryption {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketEncryption {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketEncryption";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("encryption".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketRequestPayment {
    /// Bucket名称。
    bucket: String,
    /// 配置请求者付费的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<RequestPaymentConfiguration>,
}

impl sealed::Bound for PutBucketRequestPayment {}

impl PutBucketRequestPayment {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketRequestPayment {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketRequestPayment";

    type Body = crate::XmlBody<RequestPaymentConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("requestPayment".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketRequestPayment {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketRequestPayment {}

impl GetBucketRequestPayment {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketRequestPayment {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketRequestPayment";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketRequestPaymentResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("requestPayment".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketCors {
    /// Bucket名称。
    bucket: String,
    /// 设置跨域资源共享规则的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<CORSConfiguration>,
}

impl sealed::Bound for PutBucketCors {}

impl PutBucketCors {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketCors {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketCors";

    type Body = crate::XmlBody<CORSConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("cors".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketCors {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketCors {}

impl GetBucketCors {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketCors {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketCors";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketCorsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("cors".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketCors {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for DeleteBucketCors {}

impl DeleteBucketCors {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketCors {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketCors {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketCors";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("cors".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct OptionObject {
    /// Bucket名称。
    bucket: String,
    /// Object完整路径。
    key: String,
    /// 请求来源域，用于标识跨域请求。
    /// 在实际请求中只能设置一个该请求头。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    origin: Option<String>,
    /// 在实际请求中会用到的方法。
    /// 在实际请求中只能设置一个该请求头。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    access_control_request_method: Option<String>,
    /// 在实际请求中会用到的除了简单头部之外的header。
    /// 在实际请求中可以为该请求头设置多个header，多个header之间使用英文逗号(,)隔开。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    access_control_request_headers: Option<String>,
}

impl sealed::Bound for OptionObject {}

impl OptionObject {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            origin: None,
            access_control_request_method: None,
            access_control_request_headers: None,
        }
    }
}

impl crate::Request for OptionObject {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "OptionObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<OptionObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(3);

        if let Some(f) = &self.access_control_request_headers {
            headers.push(("Access-Control-Request-Headers".into(), f.to_string()));
        }

        if let Some(f) = &self.access_control_request_method {
            headers.push(("Access-Control-Request-Method".into(), f.to_string()));
        }

        if let Some(f) = &self.origin {
            headers.push(("Origin".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {}

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("Access-Control-Allow-Origin") {
            if let Ok(s) = value.to_str() {
                inner.access_control_allow_origin = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Access-Control-Allow-Methods") {
            if let Ok(s) = value.to_str() {
                inner.access_control_allow_methods = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Access-Control-Allow-Headers") {
            if let Ok(s) = value.to_str() {
                inner.access_control_allow_headers = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Access-Control-Expose-Headers") {
            if let Ok(s) = value.to_str() {
                inner.access_control_expose_headers = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Access-Control-Max-Age") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.access_control_max_age = Some(parsed);
                }
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketAccessMonitor {
    /// Bucket名称。
    bucket: String,
    /// 修改访问跟踪状态配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<AccessMonitorConfiguration>,
}

impl sealed::Bound for PutBucketAccessMonitor {}

impl PutBucketAccessMonitor {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketAccessMonitor {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketAccessMonitor";

    type Body = crate::XmlBody<AccessMonitorConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessmonitor".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketAccessMonitor {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketAccessMonitor {}

impl GetBucketAccessMonitor {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketAccessMonitor {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketAccessMonitor";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<AccessMonitorConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessmonitor".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetMetaQueryStatus {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetMetaQueryStatus {}

impl GetMetaQueryStatus {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetMetaQueryStatus {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetMetaQueryStatus";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetMetaQueryStatusResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("metaQuery".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CloseMetaQuery {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for CloseMetaQuery {}

impl CloseMetaQuery {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for CloseMetaQuery {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CloseMetaQuery {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CloseMetaQuery";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("metaQuery".into(), "".into()));
        params.push(("comp".into(), "delete".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DoMetaQuery {
    /// Bucket名称。
    bucket: String,
    /// 指定检索模式。
    /// - basic：标量检索（默认）
    /// - semantic：向量检索
    #[setters(generate = true, strip_option)]
    mode: Option<String>,
    /// 保存查询条件的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<MetaQuery>,
}

impl sealed::Bound for DoMetaQuery {}

impl DoMetaQuery {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            mode: None,
            body: None,
        }
    }
}

impl crate::Request for DoMetaQuery {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DoMetaQuery";

    type Body = crate::XmlBody<MetaQuery>;

    type ResponseWrap = crate::XmlResponseWrap<MetaQueryResp>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("metaQuery".into(), "".into()));
        params.push(("comp".into(), "query".into()));

        if let Some(f) = &self.mode {
            params.push(("mode".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct OpenMetaQuery {
    /// Bucket名称。
    bucket: String,
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
    /// 接口请求体参数
    #[setters(generate = true, strip_option)]
    body: Option<MetaQueryOpenRequest>,
}

impl sealed::Bound for OpenMetaQuery {}

impl OpenMetaQuery {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            mode: None,
            role: None,
            body: None,
        }
    }
}

impl crate::Request for OpenMetaQuery {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "OpenMetaQuery";

    type Body = crate::XmlBody<MetaQueryOpenRequest>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(4);
        params.push(("metaQuery".into(), "".into()));
        params.push(("comp".into(), "add".into()));

        if let Some(f) = &self.mode {
            params.push(("mode".into(), (f).into()));
        }

        if let Some(f) = &self.role {
            params.push(("role".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateUserAntiDDosInfo {
    /// 高防实例ID。
    x_oss_defender_instance: String,
    /// 更改高防OSS实例状态。取值为：HaltDefending，表示解除防护。
    x_oss_defender_status: String,
}

impl sealed::Bound for UpdateUserAntiDDosInfo {}

impl UpdateUserAntiDDosInfo {
    pub fn new(
        x_oss_defender_instance: impl Into<String>,
        x_oss_defender_status: impl Into<String>,
    ) -> Self {
        Self {
            x_oss_defender_instance: x_oss_defender_instance.into(),
            x_oss_defender_status: x_oss_defender_status.into(),
        }
    }
}
impl crate::ToFormData for UpdateUserAntiDDosInfo {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for UpdateUserAntiDDosInfo {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UpdateUserAntiDDosInfo";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("antiDDos".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(2);
        headers.push((
            "x-oss-defender-instance".into(),
            self.x_oss_defender_instance.to_string(),
        ));
        headers.push((
            "x-oss-defender-status".into(),
            self.x_oss_defender_status.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateBucketAntiDDosInfo {
    /// Bucket名称。
    bucket: String,
    /// 高防实例ID。
    x_oss_defender_instance: String,
    /// 更新高防实例状态。取值如下：
    ///
    /// - Init：初始化防护状态。该状态下需要传入待防护的自定义域名。
    ///
    /// - Defending：防护中状态。该状态下可以选择是否传入待防护的自定义域名。
    ///
    /// - HaltDefending：解除防护状态。该状态下不需要传入待防护的自定义域名。
    x_oss_defender_status: String,
    /// 保存高防实例配置信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketAntiDDOSConfiguration>,
}

impl sealed::Bound for UpdateBucketAntiDDosInfo {}

impl UpdateBucketAntiDDosInfo {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_defender_instance: impl Into<String>,
        x_oss_defender_status: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_defender_instance: x_oss_defender_instance.into(),
            x_oss_defender_status: x_oss_defender_status.into(),
            body: None,
        }
    }
}

impl crate::Request for UpdateBucketAntiDDosInfo {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UpdateBucketAntiDDosInfo";

    type Body = crate::XmlBody<BucketAntiDDOSConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("antiDDos".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(2);
        headers.push((
            "x-oss-defender-instance".into(),
            self.x_oss_defender_instance.to_string(),
        ));
        headers.push((
            "x-oss-defender-status".into(),
            self.x_oss_defender_status.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
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

    type ResponseWrap = crate::XmlResponseWrap<ListBucketAntiDDosInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("bucketAntiDDos".into(), "".into()));

        if let Some(f) = &self.marker {
            params.push(("marker".into(), (f).into()));
        }

        if let Some(f) = &self.max_keys {
            params.push(("max-keys".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for InitUserAntiDDosInfo {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for InitUserAntiDDosInfo {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "InitUserAntiDDosInfo";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<InitUserAntiDDosInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("antiDDos".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-defender-instance") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_defender_instance = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct InitBucketAntiDDosInfo {
    /// Bucket名称。
    bucket: String,
    /// 高防实例ID。
    x_oss_defender_instance: String,
    /// 高防实例类型。取值固定为AntiDDosPremimum。
    x_oss_defender_type: String,
    /// 保存高防实例配置信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketAntiDDOSConfiguration>,
}

impl sealed::Bound for InitBucketAntiDDosInfo {}

impl InitBucketAntiDDosInfo {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_defender_instance: impl Into<String>,
        x_oss_defender_type: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_defender_instance: x_oss_defender_instance.into(),
            x_oss_defender_type: x_oss_defender_type.into(),
            body: None,
        }
    }
}

impl crate::Request for InitBucketAntiDDosInfo {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "InitBucketAntiDDosInfo";

    type Body = crate::XmlBody<BucketAntiDDOSConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("antiDDos".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(2);
        headers.push((
            "x-oss-defender-instance".into(),
            self.x_oss_defender_instance.to_string(),
        ));
        headers.push((
            "x-oss-defender-type".into(),
            self.x_oss_defender_type.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
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

    type ResponseWrap = crate::XmlResponseWrap<GetUserAntiDDosInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("antiDDos".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketResourceGroup {
    /// 要查询的目标Bucket
    bucket: String,
}

impl sealed::Bound for GetBucketResourceGroup {}

impl GetBucketResourceGroup {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketResourceGroup {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketResourceGroup";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketResourceGroupResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("resourceGroup".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketResourceGroup {
    /// 要修改的目标Bucket
    bucket: String,
    /// 配置资源组ID的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketResourceGroupConfiguration>,
}

impl sealed::Bound for PutBucketResourceGroup {}

impl PutBucketResourceGroup {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketResourceGroup {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketResourceGroup";

    type Body = crate::XmlBody<BucketResourceGroupConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("resourceGroup".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutCname {
    /// Bucket名称。
    bucket: String,
    /// 保存Cname配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketCnameConfiguration>,
}

impl sealed::Bound for PutCname {}

impl PutCname {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutCname {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutCname";

    type Body = crate::XmlBody<BucketCnameConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("cname".into(), "".into()));
        params.push(("comp".into(), "add".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListCname {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for ListCname {}

impl ListCname {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for ListCname {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListCname";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListCnameResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("cname".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteCname {
    /// Bucket名称。
    bucket: String,
    /// 删除Cname配置信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<DeleteCnamebody>,
}

impl sealed::Bound for DeleteCname {}

impl DeleteCname {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for DeleteCname {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DeleteCname";

    type Body = crate::XmlBody<DeleteCnamebody>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("cname".into(), "".into()));
        params.push(("comp".into(), "delete".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetCnameToken {
    /// Bucket名称。
    bucket: String,
    /// 绑定的Cname名称。
    cname: String,
}

impl sealed::Bound for GetCnameToken {}

impl GetCnameToken {
    pub fn new(bucket: impl Into<String>, cname: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            cname: cname.into(),
        }
    }
}

impl crate::Request for GetCnameToken {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetCnameToken";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<CnameToken>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("comp".into(), "token".into()));
        params.push(("cname".into(), (&self.cname).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateCnameToken {
    /// Bucket名称。
    bucket: String,
    /// 创建CnameToken的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<CnameTokenbody>,
}

impl sealed::Bound for CreateCnameToken {}

impl CreateCnameToken {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for CreateCnameToken {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateCnameToken";

    type Body = crate::XmlBody<CnameTokenbody>;

    type ResponseWrap = crate::XmlResponseWrap<CnameToken>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("cname".into(), "".into()));
        params.push(("comp".into(), "token".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutStyle {
    /// Bucket名称。
    bucket: String,
    /// 图片样式名称。
    style_name: String,
    /// 样式分类。
    ///
    /// 取值：image、document、video。
    #[setters(generate = true, strip_option)]
    category: Option<String>,
    /// 保存图片样式信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Style>,
}

impl sealed::Bound for PutStyle {}

impl PutStyle {
    pub fn new(bucket: impl Into<String>, style_name: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            style_name: style_name.into(),
            category: None,
            body: None,
        }
    }
}

impl crate::Request for PutStyle {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutStyle";

    type Body = crate::XmlBody<Style>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("style".into(), "".into()));

        if let Some(f) = &self.category {
            params.push(("category".into(), (f).into()));
        }
        params.push(("styleName".into(), (&self.style_name).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListStyle {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for ListStyle {}

impl ListStyle {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for ListStyle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListStyle";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListStyleResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("style".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetStyle {
    /// Bucket名称。
    bucket: String,
    /// 图片样式名称。
    style_name: String,
}

impl sealed::Bound for GetStyle {}

impl GetStyle {
    pub fn new(bucket: impl Into<String>, style_name: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            style_name: style_name.into(),
        }
    }
}

impl crate::Request for GetStyle {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetStyle";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<StyleInfo>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("style".into(), "".into()));
        params.push(("styleName".into(), (&self.style_name).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteStyle {
    /// Bucket名称。
    bucket: String,
    /// 图片样式名称。
    style_name: String,
}

impl sealed::Bound for DeleteStyle {}

impl DeleteStyle {
    pub fn new(bucket: impl Into<String>, style_name: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            style_name: style_name.into(),
        }
    }
}
impl crate::ToFormData for DeleteStyle {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteStyle {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteStyle";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("style".into(), "".into()));
        params.push(("styleName".into(), (&self.style_name).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketHttpsConfig {
    /// Bucket名称
    bucket: String,
}

impl sealed::Bound for GetBucketHttpsConfig {}

impl GetBucketHttpsConfig {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketHttpsConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketHttpsConfig";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<HttpsConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("httpsConfig".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketHttpsConfig {
    /// 存储空间名称
    bucket: String,
    /// 保存HTTPS配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<HttpsConfiguration>,
}

impl sealed::Bound for PutBucketHttpsConfig {}

impl PutBucketHttpsConfig {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketHttpsConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketHttpsConfig";

    type Body = crate::XmlBody<HttpsConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("httpsConfig".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateAccessPointForObjectProcess {
    /// 要创建对象FC接入点的Bucket。
    bucket: String,
    /// 填写对象FC接入点名称。接入点命名规范说明如下：
    /// - 命名长度不能超过63个字符
    /// - 只允许包含小写字母、数字、短划线（-），且不能以短划线开头或结尾。
    /// - 名称在当前地域必须唯一。
    x_oss_access_point_for_object_process_name: String,
    /// 请求体参数。
    #[setters(generate = true, strip_option)]
    body: Option<PointForObjectProcessbody>,
}

impl sealed::Bound for CreateAccessPointForObjectProcess {}

impl CreateAccessPointForObjectProcess {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_access_point_for_object_process_name: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: None,
        }
    }
}

impl crate::Request for CreateAccessPointForObjectProcess {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "CreateAccessPointForObjectProcess";

    type Body = crate::XmlBody<PointForObjectProcessbody>;

    type ResponseWrap = crate::XmlResponseWrap<CreateAccessPointForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointForObjectProcess".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointForObjectProcess {
    /// 存储空间名称。
    bucket: String,
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
}

impl sealed::Bound for GetAccessPointForObjectProcess {}

impl GetAccessPointForObjectProcess {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_access_point_for_object_process_name: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
        }
    }
}

impl crate::Request for GetAccessPointForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointForObjectProcess";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetAccessPointForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointForObjectProcess".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
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

    type ResponseWrap = crate::XmlResponseWrap<ListAccessPointsForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("accessPointForObjectProcess".into(), "".into()));

        if let Some(f) = &self.continuation_token {
            params.push(("continuation-token".into(), (f).into()));
        }

        if let Some(f) = &self.max_keys {
            params.push(("max-keys".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointForObjectProcess {
    /// 要删除对象FC接入点的Bucket。
    bucket: String,
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
}

impl sealed::Bound for DeleteAccessPointForObjectProcess {}

impl DeleteAccessPointForObjectProcess {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_access_point_for_object_process_name: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
        }
    }
}
impl crate::ToFormData for DeleteAccessPointForObjectProcess {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteAccessPointForObjectProcess {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteAccessPointForObjectProcess";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointForObjectProcess".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointConfigForObjectProcess {
    /// 存储空间名称。
    bucket: String,
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
}

impl sealed::Bound for GetAccessPointConfigForObjectProcess {}

impl GetAccessPointConfigForObjectProcess {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_access_point_for_object_process_name: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
        }
    }
}

impl crate::Request for GetAccessPointConfigForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointConfigForObjectProcess";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetAccessPointConfigForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointConfigForObjectProcess".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointConfigForObjectProcess {
    /// 存储空间名称。
    bucket: String,
    /// 填写对象FC接入点名称。接入点命名规范说明如下：
    ///
    /// - 命名长度不能超过63个字符。
    /// - 只允许包含小写字母、数字、短划线（-），且不能以短划线开头或结尾。
    /// - 名称在当前地域必须唯一。
    x_oss_access_point_for_object_process_name: String,
    /// 请求体参数。
    #[setters(generate = true, strip_option)]
    body: Option<ConfigForObjectProcessbody>,
}

impl sealed::Bound for PutAccessPointConfigForObjectProcess {}

impl PutAccessPointConfigForObjectProcess {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_access_point_for_object_process_name: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: None,
        }
    }
}

impl crate::Request for PutAccessPointConfigForObjectProcess {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAccessPointConfigForObjectProcess";

    type Body = crate::XmlBody<ConfigForObjectProcessbody>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointConfigForObjectProcess".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointPolicyForObjectProcess {
    /// 存储空间名称。
    bucket: String,
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<String>,
}

impl sealed::Bound for PutAccessPointPolicyForObjectProcess {}

impl PutAccessPointPolicyForObjectProcess {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_access_point_for_object_process_name: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: None,
        }
    }
}

impl crate::Request for PutAccessPointPolicyForObjectProcess {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAccessPointPolicyForObjectProcess";

    type Body = crate::JsonBody<String>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointPolicyForObjectProcess".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointPolicyForObjectProcess {
    /// 存储空间名称。
    bucket: String,
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
    body: Vec<u8>,
}

impl sealed::Bound for GetAccessPointPolicyForObjectProcess {}

impl GetAccessPointPolicyForObjectProcess {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_access_point_for_object_process_name: impl Into<String>,
        body: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: body.into(),
        }
    }
}

impl crate::Request for GetAccessPointPolicyForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointPolicyForObjectProcess";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::XmlResponseWrap<GetAccessPointPolicyForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointPolicyForObjectProcess".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointPolicyForObjectProcess {
    /// 要删除对象FC接入点策略的Bucket。
    bucket: String,
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
    body: Vec<u8>,
}

impl sealed::Bound for DeleteAccessPointPolicyForObjectProcess {}

impl DeleteAccessPointPolicyForObjectProcess {
    pub fn new(
        bucket: impl Into<String>,
        x_oss_access_point_for_object_process_name: impl Into<String>,
        body: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: body.into(),
        }
    }
}

impl crate::Request for DeleteAccessPointPolicyForObjectProcess {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteAccessPointPolicyForObjectProcess";

    type Body = crate::OctetStream;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("accessPointPolicyForObjectProcess".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body)
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

    type ResponseWrap = crate::XmlResponseWrap<PublicAccessBlockConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("publicAccessBlock".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutPublicAccessBlock {
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<PublicAccessBlockConfiguration>,
}

impl sealed::Bound for PutPublicAccessBlock {}

impl PutPublicAccessBlock {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutPublicAccessBlock {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutPublicAccessBlock";

    type Body = crate::XmlBody<PublicAccessBlockConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("publicAccessBlock".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
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
impl crate::ToFormData for DeletePublicAccessBlock {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeletePublicAccessBlock {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeletePublicAccessBlock";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("publicAccessBlock".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketPublicAccessBlock {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for GetBucketPublicAccessBlock {}

impl GetBucketPublicAccessBlock {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketPublicAccessBlock";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<PublicAccessBlockConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("publicAccessBlock".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketPublicAccessBlock {
    /// Bucket名称。
    bucket: String,
    /// 接口请求体参数。
    #[setters(generate = true, strip_option)]
    body: Option<PublicAccessBlockConfiguration>,
}

impl sealed::Bound for PutBucketPublicAccessBlock {}

impl PutBucketPublicAccessBlock {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketPublicAccessBlock {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketPublicAccessBlock";

    type Body = crate::XmlBody<PublicAccessBlockConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("publicAccessBlock".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketPublicAccessBlock {
    /// Bucket名称。
    bucket: String,
}

impl sealed::Bound for DeleteBucketPublicAccessBlock {}

impl DeleteBucketPublicAccessBlock {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketPublicAccessBlock {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketPublicAccessBlock {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketPublicAccessBlock";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("publicAccessBlock".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointPublicAccessBlock {
    /// Bucket名称。
    bucket: String,
    /// 接入点名称。
    #[setters(generate = true, strip_option)]
    x_oss_access_point_name: Option<String>,
}

impl sealed::Bound for GetAccessPointPublicAccessBlock {}

impl GetAccessPointPublicAccessBlock {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_name: None,
        }
    }
}

impl crate::Request for GetAccessPointPublicAccessBlock {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointPublicAccessBlock";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<PublicAccessBlockConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("publicAccessBlock".into(), "".into()));

        if let Some(f) = &self.x_oss_access_point_name {
            params.push(("x-oss-access-point-name".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointPublicAccessBlock {
    /// Bucket名称。
    bucket: String,
    /// 接入点名称。
    x_oss_access_point_name: String,
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<PublicAccessBlockConfiguration>,
}

impl sealed::Bound for PutAccessPointPublicAccessBlock {}

impl PutAccessPointPublicAccessBlock {
    pub fn new(bucket: impl Into<String>, x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_name: x_oss_access_point_name.into(),
            body: None,
        }
    }
}

impl crate::Request for PutAccessPointPublicAccessBlock {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAccessPointPublicAccessBlock";

    type Body = crate::XmlBody<PublicAccessBlockConfiguration>;

    type ResponseWrap = crate::XmlResponseWrap<PutAccessPointPublicAccessBlockResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("publicAccessBlock".into(), "".into()));
        params.push((
            "x-oss-access-point-name".into(),
            (&self.x_oss_access_point_name).into(),
        ));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointPublicAccessBlock {
    /// Bucket名称。
    bucket: String,
    /// 接入点名称。
    #[setters(generate = true, strip_option)]
    x_oss_access_point_name: Option<String>,
}

impl sealed::Bound for DeleteAccessPointPublicAccessBlock {}

impl DeleteAccessPointPublicAccessBlock {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_access_point_name: None,
        }
    }
}
impl crate::ToFormData for DeleteAccessPointPublicAccessBlock {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteAccessPointPublicAccessBlock {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteAccessPointPublicAccessBlock";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("publicAccessBlock".into(), "".into()));

        if let Some(f) = &self.x_oss_access_point_name {
            params.push(("x-oss-access-point-name".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketArchiveDirectRead {
    /// 存储空间名称。
    bucket: String,
}

impl sealed::Bound for GetBucketArchiveDirectRead {}

impl GetBucketArchiveDirectRead {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketArchiveDirectRead {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketArchiveDirectRead";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ArchiveDirectReadConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("bucketArchiveDirectRead".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketArchiveDirectRead {
    /// 存储空间名称。
    bucket: String,
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<ArchiveDirectReadConfiguration>,
}

impl sealed::Bound for PutBucketArchiveDirectRead {}

impl PutBucketArchiveDirectRead {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketArchiveDirectRead {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketArchiveDirectRead";

    type Body = crate::XmlBody<ArchiveDirectReadConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("bucketArchiveDirectRead".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketOverwriteConfig {
    /// 存储空间名称
    bucket: String,
    /// 接口请求体结构
    #[setters(generate = true, strip_option)]
    body: Option<OverwriteConfiguration>,
}

impl sealed::Bound for PutBucketOverwriteConfig {}

impl PutBucketOverwriteConfig {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketOverwriteConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketOverwriteConfig";

    type Body = crate::XmlBody<OverwriteConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("overwriteConfig".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketOverwriteConfig {
    /// 存储空间名称
    bucket: String,
}

impl sealed::Bound for GetBucketOverwriteConfig {}

impl GetBucketOverwriteConfig {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketOverwriteConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketOverwriteConfig";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<OverwriteConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("overwriteConfig".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketOverwriteConfig {
    /// 存储空间名称
    bucket: String,
}

impl sealed::Bound for DeleteBucketOverwriteConfig {}

impl DeleteBucketOverwriteConfig {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketOverwriteConfig {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketOverwriteConfig {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketOverwriteConfig";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("overwriteConfig".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutObject {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// 指定PutObject操作时是否覆盖同名Object。 当目标Bucket处于已开启或已暂停的版本控制状态时，**x-oss-forbid-overwrite**请求Header设置无效，即允许覆盖同名Object。
    ///   - 不指定**x-oss-forbid-overwrite**或者指定**x-oss-forbid-overwrite**为**false**时，表示允许覆盖同名Object。
    ///   - 指定**x-oss-forbid-overwrite**为**true**时，表示禁止覆盖同名Object。
    ///
    /// 设置**x-oss-forbid-overwrite**请求Header会导致QPS处理性能下降，如果您有大量的操作需要使用**x-oss-forbid-overwrite**请求Header（QPS>1000），请联系技术支持，避免影响您的业务。
    /// 默认值：**false**
    #[setters(generate = true, strip_option)]
    x_oss_forbid_overwrite: Option<bool>,
    /// 创建Object时，指定服务器端加密方式。
    ///
    /// 取值：**AES256**、**KMS****或**SM4****
    ///
    /// 指定此选项后，在响应头中会返回此选项，OSS会对上传的Object进行加密编码存储。当下载该Object时，响应头中会包含**x-oss-server-side-encryption**，且该值会被设置成此Object的加密算法。
    #[setters(generate = true, strip_option)]
    x_oss_server_side_encryption: Option<String>,
    /// 创建Object时，指定服务器端加密方式。
    ///
    /// 取值：**AES256**、**KMS**或**SM4**
    ///
    /// 指定此选项后，在响应头中会返回此选项，OSS会对上传的Object进行加密编码存储。当下载该Object时，响应头中会包含**x-oss-server-side-encryption**，且该值会被设置成此Object的加密算法。
    #[setters(generate = true, strip_option)]
    x_oss_server_side_data_encryption: Option<String>,
    /// KMS托管的用户主密钥。
    /// 此选项仅在**x-oss-server-side-encryption**为KMS时有效。
    #[setters(generate = true, strip_option)]
    x_oss_server_side_encryption_key_id: Option<String>,
    /// 指定OSS创建Object时的访问权限。
    ///
    /// 取值：
    ///
    /// - default（默认）：Object遵循所在存储空间的访问权限。
    /// - private：Object是私有资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户没有权限操作该Object。
    /// - public-read：Object是公共读资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户只有该Object的读权限。请谨慎使用该权限。
    /// - public-read-write：Object是公共读写资源。所有用户都有该Object的读写权限。请谨慎使用该权限。
    ///
    /// 关于访问权限的更多信息，请参见**[读写权限ACL](~~100676~~)**。
    #[setters(generate = true, strip_option)]
    x_oss_object_acl: Option<ObjectACL>,
    /// 指定Object的存储类型。                               对于任意存储类型的Bucket，如果上传Object时指定此参数，则此次上传的Object将存储为指定的类型。例如在IA类型的Bucket中上传Object时，如果指定x-oss-storage-class为Standard，则该Object直接存储为Standard。                              取值：                                 Standard：标准存储                                    IA：低频访问                                    Archive：归档存储                                    ColdArchive：冷归档存储                                    关于存储类型的更多信息，请参见存储类型介绍。
    #[setters(generate = true, strip_option)]
    x_oss_storage_class: Option<StorageClass>,
    /// 指定Object的标签，可同时设置多个标签，例如TagA=A&TagB=B。
    /// > Key和Value需要先进行URL编码，如果某项没有”=“，则看作Value为空字符串。
    #[setters(generate = true, strip_option)]
    x_oss_tagging: Option<String>,
    /// 使用PutObject接口时，如果配置以**x-oss-meta-***为前缀的参数，则该参数视为元数据，例如`x-oss-meta-location`。一个Object可以有多个类似的参数，但所有的元数据总大小不能超过8 KB。
    ///
    /// 元数据支持短划线（-）、数字、英文字母（a~z）。英文字符的大写字母会被转成小写字母，不支持下划线（_）在内的其他字符。
    #[setters(generate = true, strip_option)]
    x_oss_meta: Option<std::collections::HashMap<String, String>>,
    /// 请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 指定Object下载时的缓存行为。取值如下： - no-cache：不可直接使用缓存，而是先到服务端验证Object是否已更新。如果Object已更新，表明缓存已过期，需从服务端重新下载Object；如果Object未更新，表明缓存未过期，此时将使用本地缓存。 - no-store：所有内容都不会被缓存。 - public：所有内容都将被缓存。 - private：所有内容只在客户端缓存。 - max-age=<seconds>：缓存内容的相对过期时间，单位为秒。此选项仅在HTTP 1.1中可用。 默认值：无
    #[setters(generate = true, strip_option)]
    cache_control: Option<String>,
    /// 指定Object的展示形式。取值如下： - Content-Disposition:inline：直接预览文件内容。 - Content-Disposition:attachment：以原文件名的形式下载到浏览器指定路径。 - Content-Disposition:attachment; filename="yourFileName"：以自定义文件名的形式下载到浏览器指定路径。 yourFileName用于自定义下载后的文件名称，例如example.jpg。 将Object下载到浏览器指定路径时： **说明** - 如果Object名称包含星号（*）、正斜线（/）等特殊字符时，可能会出现特殊字符转义的情况。例如，下载`example*.jpg`到本地时，`example*.jpg`可能会转义为`example_.jpg`。 - 如需确保下载名称中包含中文字符的Object到本地指定路径后，文件名称不出现乱码的现象，您需要将名称中包含的中文字符进行URL编码。例如，将`测试.txt`从OSS下载到本地后，需要保留文件名为`测试.txt`，需按照`"attachment;filename="+URLEncoder.encode("测试","UTF-8")+".txt;filename*=UTF-8''"+URLEncoder.encode("测试","UTF-8")+".txt"`的格式设置Content-Disposition，即attachment;filename=%E6%B5%8B%E8%AF%95.txt;filename*=UTF-8''%E6%B5%8B%E8%AF%95.txt。 通过文件URL访问文件时是预览还是以附件形式下载，与文件所在Bucket的创建时间、OSS开通时间以及使用的域名类型有关。更多信息，请参见[通过文件URL访问文件无法预览而是以附件形式下载？](https://help.aliyun.com/zh/oss/images-downloaded-as-an-attachment-instead-of-being-previewed-by-using-a-url#concept-2331929)。 默认值：无
    #[setters(generate = true, strip_option)]
    content_disposition: Option<String>,
    #[setters(generate = true, strip_option)]
    content_type: Option<String>,
    /// 声明Object的编码方式。必须按照Object的实际编码类型填写，否则可能造成客户端解析失败或下载失败。若Object未编码，请置空此项。取值如下： - identity（默认值）：表示Object未经过压缩或编码。 - gzip：表示Object采用Lempel-Ziv（LZ77）压缩算法以及32位CRC校验的编码方式。 - compress：表示Object采用Lempel-Ziv-Welch（LZW）压缩算法的编码方式。 - deflate：表示Object采用zlib结构和deflate压缩算法的编码方式。 - br：表示Object采用Brotli算法的编码方式。 默认值：无
    #[setters(generate = true, strip_option)]
    content_encoding: Option<String>,
    /// 用于检查消息内容完整性。Content-MD5是由MD5算法生成的值。设置该请求头后，OSS会计算消息体的Content-MD5并检查一致性。详细信息请参见[Content-MD5的计算方法](https://help.aliyun.com/zh/oss/developer-reference/include-signatures-in-the-authorization-header#section-i74-k35-5w4)。 为确保数据完整性，OSS提供多种数据MD5值校验方式。如需通过Content-MD5进行MD5验证，可将Content-MD5加入到请求头中。 默认值：无
    #[setters(generate = true, strip_option)]
    content_md5: Option<String>,
    /// 描述HTTP消息体的传输大小，单位为字节。 如果请求头中的Content-Length值小于实际请求体传输的数据大小，OSS仍将成功创建Object，但Object大小等于Content-Length中定义的大小，超出部分数据将被丢弃。
    #[setters(generate = true, strip_option)]
    content_length: Option<String>,
    /// 指定Object的过期时间。详细信息请参见[RFC2616](https://www.ietf.org/rfc/rfc2616.txt)。 默认值：无
    #[setters(generate = true, strip_option)]
    expires: Option<String>,
}

impl sealed::Bound for PutObject {}

impl PutObject {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            x_oss_forbid_overwrite: None,
            x_oss_server_side_encryption: None,
            x_oss_server_side_data_encryption: None,
            x_oss_server_side_encryption_key_id: None,
            x_oss_object_acl: None,
            x_oss_storage_class: None,
            x_oss_tagging: None,
            x_oss_meta: None,
            body: None,
            cache_control: None,
            content_disposition: None,
            content_type: None,
            content_encoding: None,
            content_md5: None,
            content_length: None,
            expires: None,
        }
    }
}

impl crate::Request for PutObject {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::SelfResponseWrap<PutObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(15);

        if let Some(f) = &self.cache_control {
            headers.push(("Cache-Control".into(), f.to_string()));
        }

        if let Some(f) = &self.content_disposition {
            headers.push(("Content-Disposition".into(), f.to_string()));
        }

        if let Some(f) = &self.content_encoding {
            headers.push(("Content-Encoding".into(), f.to_string()));
        }

        if let Some(f) = &self.content_length {
            headers.push(("Content-Length".into(), f.to_string()));
        }

        if let Some(f) = &self.content_md5 {
            headers.push(("Content-MD5".into(), f.to_string()));
        }

        if let Some(f) = &self.content_type {
            headers.push(("Content-Type".into(), f.to_string()));
        }

        if let Some(f) = &self.expires {
            headers.push(("Expires".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_forbid_overwrite {
            headers.push(("x-oss-forbid-overwrite".into(), f.to_string()));
        }

        if let Some(map) = &self.x_oss_meta {
            for (key, value) in map {
                headers.push((format!("{}{}", "x-oss-meta-", key).into(), value.clone()));
            }
        }

        if let Some(f) = &self.x_oss_object_acl {
            headers.push(("x-oss-object-acl".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_data_encryption {
            headers.push(("x-oss-server-side-data-encryption".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_encryption {
            headers.push(("x-oss-server-side-encryption".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_encryption_key_id {
            headers.push(("x-oss-server-side-encryption-key-id".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_storage_class {
            headers.push(("x-oss-storage-class".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_tagging {
            headers.push(("x-oss-tagging".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body.unwrap_or_default())
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-hash-crc64ecma") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.x_oss_hash_crc64ecma = Some(parsed);
                }
            }
        }
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CopyObject {
    /// Bucket名称。
    bucket: String,
    /// Object完整路径。
    key: String,
    /// 指定拷贝的源地址。
    /// 默认值：无
    x_oss_copy_source: String,
    /// 指定CopyObject操作时是否覆盖同名目标Object。当目标Bucket处于已开启或已暂停版本控制状态时，**x-oss-forbid-overwrite**请求Header设置无效，即允许覆盖同名Object。
    ///   - 未指定**x-oss-forbid-overwrite**或者指定**x-oss-forbid-overwrite**为**false**时，表示允许覆盖同名目标Object。
    ///   - 指定**x-oss-forbid-overwrite**为**true**时，表示禁止覆盖同名Object。
    ///
    /// 设置**x-oss-forbid-overwrite**请求Header会导致QPS处理性能下降，如果您有大量的操作需要使用x-**x-oss-forbid-overwrite**请求Header（QPS>1000），请联系技术支持，避免影响您的业务。
    /// 默认值：false
    #[setters(generate = true, strip_option)]
    x_oss_forbid_overwrite: Option<String>,
    /// 如果源Object的ETag值和您提供的ETag相等，则执行拷贝操作，并返回200 OK。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    x_oss_copy_source_if_match: Option<String>,
    /// 如果源Object的ETag值和您提供的ETag不相等，则执行拷贝操作，并返回200 OK。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    x_oss_copy_source_if_none_match: Option<String>,
    /// 如果指定的时间等于或者晚于文件实际修改时间，则正常拷贝文件，并返回200 OK。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    x_oss_copy_source_if_unmodified_since: Option<String>,
    /// 如果源Object在用户指定的时间以后被修改过，则执行拷贝操作。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    x_oss_copy_source_if_modified_since: Option<String>,
    /// 指定如何设置目标Object的元信息。
    ///   - **COPY**（默认值）：复制源Object的元数据到目标Object。
    /// OSS不会复制源Object的**x-oss-server-side-encryption**属性配置到目标Object。目标Object的服务器端加密编码方式取决于当前拷贝操作是否指定了**x-oss-server-side-encryption**。
    ///   - **REPLACE**：忽略源Object的元数据，直接采用请求中指定的元数据。
    ///
    /// > 如果拷贝操作的源Object地址和目标Object地址相同，且未开启版本控制时，则无论x-oss-metadata-directive为何值，都会忽略源Object的元数据，目标Object将直接采用请求中指定的元数据。
    #[setters(generate = true, strip_option)]
    x_oss_metadata_directive: Option<String>,
    /// 指定OSS创建目标Object时，服务器端熵编码加密算法 。
    /// 取值：**AES256**、**KMS**
    /// 您只有购买了KMS套件，才能使用KMS加密算法，否则OSS会返回KmsServiceNotEnabled错误。
    ///   - 如果拷贝操作中未指定**x-oss-server-side-encryption**，则无论源Object是否进行过服务器端加密编码，拷贝后的目标Object均不进行服务器端加密编码。
    ///   - 如果拷贝操作中指定了**x-oss-server-side-encryption**，则无论源Object是否进行过服务器端加密编码，拷贝后的目标Object均会进行服务器端加密编码。并且拷贝操作的响应头中会包含**x-oss-server-side-encryption**，值为目标Object的加密算法。
    /// 在目标Object被下载时，响应头中也会包含**x-oss-server-side-encryption**，值为该Object的加密算法。
    #[setters(generate = true, strip_option)]
    x_oss_server_side_encryption: Option<String>,
    /// 指定Object的加密算法。如果未指定此选项，表明Object使用AES256加密算法。此选项仅当x-oss-server-side-encryption为KMS时有效。
    #[setters(generate = true, strip_option)]
    x_oss_server_side_data_encryption: Option<String>,
    /// 表示KMS托管的用户主密钥。
    /// 该参数仅在**x-oss-server-side-encryption**为KMS时有效。
    #[setters(generate = true, strip_option)]
    x_oss_server_side_encryption_key_id: Option<String>,
    /// 指定OSS创建目标Object时的访问权限。
    ///
    /// 取值：
    ///
    /// - default（默认）：Object遵循所在存储空间的访问权限。
    /// - private：Object是私有资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户没有权限操作该Object。
    /// - public-read：Object是公共读资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户只有该Object的读权限。请谨慎使用该权限。
    /// - public-read-write：Object是公共读写资源。所有用户都有该Object的读写权限。请谨慎使用该权限。
    ///
    /// 关于访问权限的更多信息，请参见**[读写权限ACL](~~100676~~)**。
    #[setters(generate = true, strip_option)]
    x_oss_object_acl: Option<ObjectACL>,
    /// 指定Object的存储类型。
    /// 对于任意存储类型Bucket，如果上传Object时指定该值，则此次上传的Object将存储为指定的类型。例如在IA类型的Bucket中上传Object时，如果指定**x-oss-storage-class**为Standard，则该Object直接存储为Standard类型。
    ///
    /// 取值：
    /// - Standard：标准存储
    /// - IA：低频访问
    /// - Archive：归档存储
    /// - ColdArchive：冷归档存储
    ///
    /// 关于存储类型的更多信息，请参见**[存储类型介绍](~~51374~~)**。
    #[setters(generate = true, strip_option)]
    x_oss_storage_class: Option<StorageClass>,
    /// 指定Object的对象标签，可同时设置多个标签，例如TagA=A&TagB=B。
    /// > Key和Value需要先进行URL编码，如果某项没有“=”，则看作Value为空字符串。
    #[setters(generate = true, strip_option)]
    x_oss_tagging: Option<String>,
    /// 指定如何设置目标Object的对象标签。取值如下：
    ///   - **Copy**（默认值）：复制源Object的对象标签到目标 Object。
    ///   - **Replace**：忽略源Object的对象标签，直接采用请求中指定的对象标签。
    #[setters(generate = true, strip_option)]
    x_oss_tagging_directive: Option<String>,
    /// 如果配置以x-oss-meta-*为前缀的参数，则该参数视为元数据。
    /// 元数据大小限制：一个Object可以包含多个元数据，但所有的元数据总大小不能超过8 KB。
    /// 元数据命名规则：支持短划线（-）、数字、英文字母（a~z）。英文字符的大写字母会被转成小写字母，不支持下划线（_）在内的其他字符。
    #[setters(generate = true, strip_option)]
    x_oss_meta: Option<std::collections::HashMap<String, String>>,
}

impl sealed::Bound for CopyObject {}

impl CopyObject {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        x_oss_copy_source: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            x_oss_copy_source: x_oss_copy_source.into(),
            x_oss_forbid_overwrite: None,
            x_oss_copy_source_if_match: None,
            x_oss_copy_source_if_none_match: None,
            x_oss_copy_source_if_unmodified_since: None,
            x_oss_copy_source_if_modified_since: None,
            x_oss_metadata_directive: None,
            x_oss_server_side_encryption: None,
            x_oss_server_side_data_encryption: None,
            x_oss_server_side_encryption_key_id: None,
            x_oss_object_acl: None,
            x_oss_storage_class: None,
            x_oss_tagging: None,
            x_oss_tagging_directive: None,
            x_oss_meta: None,
        }
    }
}
impl crate::ToFormData for CopyObject {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CopyObject {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "CopyObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<CopyObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(15);
        headers.push((
            "x-oss-copy-source".into(),
            self.x_oss_copy_source.to_string(),
        ));

        if let Some(f) = &self.x_oss_copy_source_if_match {
            headers.push(("x-oss-copy-source-if-match".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_copy_source_if_modified_since {
            headers.push(("x-oss-copy-source-if-modified-since".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_copy_source_if_none_match {
            headers.push(("x-oss-copy-source-if-none-match".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_copy_source_if_unmodified_since {
            headers.push((
                "x-oss-copy-source-if-unmodified-since".into(),
                f.to_string(),
            ));
        }

        if let Some(f) = &self.x_oss_forbid_overwrite {
            headers.push(("x-oss-forbid-overwrite".into(), f.to_string()));
        }

        if let Some(map) = &self.x_oss_meta {
            for (key, value) in map {
                headers.push((format!("{}{}", "x-oss-meta-", key).into(), value.clone()));
            }
        }

        if let Some(f) = &self.x_oss_metadata_directive {
            headers.push(("x-oss-metadata-directive".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_object_acl {
            headers.push(("x-oss-object-acl".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_data_encryption {
            headers.push(("x-oss-server-side-data-encryption".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_encryption {
            headers.push(("x-oss-server-side-encryption".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_encryption_key_id {
            headers.push(("x-oss-server-side-encryption-key-id".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_storage_class {
            headers.push(("x-oss-storage-class".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_tagging {
            headers.push(("x-oss-tagging".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_tagging_directive {
            headers.push(("x-oss-tagging-directive".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-copy-source-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_copy_source_version_id = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetObject {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
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
    /// 指定文件传输的范围。
    ///   - 如果指定的范围符合规范，返回消息中会包含整个Object的大小和此次返回Object的范围。例如：Content-Range: bytes 0~9/44，表示整个Object大小为44，此次返回的范围为0~9。
    ///   - 如果指定的范围不符合规范，则传送整个Object，并且结果中不包含Content-Range。
    ///
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    range: Option<String>,
    /// 如果指定的时间早于实际修改时间或指定的时间不符合规范，则直接返回Object，并返回200 OK；如果指定的时间等于或者晚于实际修改时间，则返回304 Not Modified。
    /// 时间格式：GMT，例如`Fri, 13 Nov 2015 14:47:53 GMT`
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    if_modified_since: Option<String>,
    /// 如果指定的时间等于或者晚于Object实际修改时间，则正常传输Object，并返回200 OK；如果指定的时间早于实际修改时间，则返回412 Precondition
    ///                               Failed。
    /// 时间格式：GMT，例如`Fri, 13 Nov 2015 14:47:53 GMT`
    /// **If-Modified-Since**和**If-Unmodified-Since**可以同时使用。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    if_unmodified_since: Option<String>,
    /// 如果传入的ETag和Object的ETag匹配，则正常传输Object，并返回200 OK；如果传入的ETag和Object的ETag不匹配，则返回412 Precondition Failed。
    /// Object的ETag值用于验证数据是否发生了更改，您可以基于ETag值验证数据完整性。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    if_match: Option<String>,
    /// 如果传入的ETag值和Object的ETag不匹配，则正常传输Object，并返回200 OK；如果传入的ETag和Object的ETag匹配，则返回304 Not Modified。
    /// **If-Match**和**If-None-Match**可以同时使用。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    if_none_match: Option<String>,
    /// 指定客户端的编码类型。
    /// 如果要对返回内容进行Gzip压缩传输，您需要在请求头中以显示方式加入Accept-Encoding:gzip。OSS会根据Object的Content-Type和Object大小（不小于1
    ///                                  KB）判断是否返回经过Gzip压缩的数据。
    /// >   - 如果采用了Gzip压缩，则不会附带ETag信息。
    /// >   - 目前OSS支持Gzip压缩的Content-Type为text/cache-manifest、 text/xml、text/plain、text/css、application/javascript、application/x-javascript、application/rss+xml、application/json和text/json。
    ///
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    accept_encoding: Option<String>,
    /// 目标文件的版本ID。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObject {}

impl GetObject {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            response_content_type: None,
            response_content_language: None,
            response_expires: None,
            response_cache_control: None,
            response_content_disposition: None,
            response_content_encoding: None,
            range: None,
            if_modified_since: None,
            if_unmodified_since: None,
            if_match: None,
            if_none_match: None,
            accept_encoding: None,
            version_id: None,
        }
    }
}

impl crate::Request for GetObject {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = ();

    type ResponseWrap = crate::BinaryResponseWithMetaWrap<GetObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(7);

        if let Some(f) = &self.response_cache_control {
            params.push(("response-cache-control".into(), (f).into()));
        }

        if let Some(f) = &self.response_content_disposition {
            params.push(("response-content-disposition".into(), (f).into()));
        }

        if let Some(f) = &self.response_content_encoding {
            params.push(("response-content-encoding".into(), (f).into()));
        }

        if let Some(f) = &self.response_content_language {
            params.push(("response-content-language".into(), (f).into()));
        }

        if let Some(f) = &self.response_content_type {
            params.push(("response-content-type".into(), (f).into()));
        }

        if let Some(f) = &self.response_expires {
            params.push(("response-expires".into(), (f).into()));
        }

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(6);

        if let Some(f) = &self.accept_encoding {
            headers.push(("Accept-Encoding".into(), f.to_string()));
        }

        if let Some(f) = &self.if_match {
            headers.push(("If-Match".into(), f.to_string()));
        }

        if let Some(f) = &self.if_modified_since {
            headers.push(("If-Modified-Since".into(), f.to_string()));
        }

        if let Some(f) = &self.if_none_match {
            headers.push(("If-None-Match".into(), f.to_string()));
        }

        if let Some(f) = &self.if_unmodified_since {
            headers.push(("If-Unmodified-Since".into(), f.to_string()));
        }

        if let Some(f) = &self.range {
            headers.push(("Range".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {}

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-server-side-encryption") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_server_side_encryption = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-server-side-encryption-key-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_server_side_encryption_key_id = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-storage-class") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_storage_class = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-object-type") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_object_type = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-next-append-position") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.x_oss_next_append_position = Some(parsed);
                }
            }
        }
        if let Some(value) = headers.get("x-oss-hash-crc64ecma") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_hash_crc64ecma = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-expiration") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_expiration = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-restore") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_restore = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-process-status") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_process_status = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-request-charged") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_request_charged = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Content-Md5") {
            if let Ok(s) = value.to_str() {
                inner.content_md5 = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Content-Length") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.content_length = Some(parsed);
                }
            }
        }
        if let Some(value) = headers.get("Last-Modified") {
            if let Ok(s) = value.to_str() {
                inner.last_modified = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Content-Type") {
            if let Ok(s) = value.to_str() {
                inner.content_type = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("ETag") {
            if let Ok(s) = value.to_str() {
                inner.e_tag = Some(s.to_string());
            }
        }
        // Handle wildcard headers matching prefix "x-oss-meta-"
        for (k, v) in headers
            .iter()
            .filter(|(k, _)| k.as_str().starts_with("x-oss-meta-"))
        {
            let key = k.as_str().trim_start_matches("x-oss-meta-");
            if let Ok(value) = v.to_str() {
                inner.x_oss_meta.insert(key.to_owned(), value.to_owned());
            }
        }
        if let Some(value) = headers.get("x-oss-tagging-count") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.x_oss_tagging_count = Some(parsed);
                }
            }
        }
        if let Some(value) = headers.get("Content-Disposition") {
            if let Ok(s) = value.to_str() {
                inner.content_disposition = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AppendObject {
    /// Bucket名称。
    bucket: String,
    /// Object完整路径。
    key: String,
    /// 用于指定从何处进行追加。 每次操作成功后，响应消息头x-oss-next-append-position会标明下一次追加的position。首次追加操作的position必须为0，后续追加操作的position是Object的当前大小。例如，第一次AppendObject请求指定position值为0，content-length是65536，则第二次AppendObject需要指定position为65536。
    ///
    /// - 当position值为0，且不存在同名Object时，则AppendObject与PutObject请求类似，即允许设置x-oss-server-side-encryption等请求头。如果加入了正确的x-oss-server-side-encryption头，那么后续的AppendObject响应头部也会包含x-oss-server-side-encryption头。后续如需更改元数据，可以使用CopyObject接口。
    /// - 在position值正确的情况下，对已存在的Appendable Object追加一个大小为0的内容，该操作不会改变Object的状态。
    position: i64,
    /// 指定服务器端加密方式。
    /// 合法值：
    ///
    /// - AES256：使用OSS完全托管密钥进行加解密（SSE-OSS）。
    /// - KMS：使用KMS托管密钥进行加解密。
    /// - SM4：国密SM4算法。
    #[setters(generate = true, strip_option)]
    x_oss_server_side_encryption: Option<String>,
    /// 指定Object的访问权限。 取值范围如下：
    ///
    /// - default（默认）：Object遵循所在存储空间的访问权限。
    /// - private：Object是私有资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户没有权限操作该Object。
    /// - public-read：Object是公共读资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户只有该Object的读权限。请谨慎使用该权限。
    /// - public-read-write：Object是公共读写资源。所有用户都有该Object的读写权限。请谨慎使用该权限。
    ///
    /// 关于访问权限的更多信息，请参见[读写权限ACL](~~100676~~)。
    #[setters(generate = true, strip_option)]
    x_oss_object_acl: Option<ObjectACL>,
    /// 指定Object的存储类型。取值范围如下：
    ///
    /// - Standard：标准存储
    /// -  IA：低频访问
    /// - Archive：归档存储
    /// 对于任意存储类型的Bucket，如果上传Object时指定此参数，则此次上传的Object将存储为指定的类型。例如在IA类型的Bucket中上传Object时，如果指定x-oss-storage-class为Standard，则该Object直接存储为Standard。
    /// 关于存储类型的更多信息，请参见存储类型介绍。
    ///
    /// ><notice> 该值仅在首次执行AppendObject操作时有效，后续追加时不生效。
    #[setters(generate = true, strip_option)]
    x_oss_storage_class: Option<StorageClass>,
    /// 创建AppendObject时可以添加x-oss-meta-*，继续追加时不可以携带此参数。如果配置以x-oss-meta-*为前缀的参数，则该参数视为元数据。
    /// 元数据大小限制：一个Object可以包含多个元数据，但所有的元数据总大小不能超过8 KB。
    /// 元数据命名规则：支持短划线（-）、数字、英文字母（a~z）。英文字符的大写字母会被转成小写字母，不支持下划线（_）在内的其他字符。
    #[setters(generate = true, strip_option)]
    x_oss_meta: Option<std::collections::HashMap<String, String>>,
    /// 指定该Object的网页缓存行为。更多信息，请参见**[RFC2616](https://www.ietf.org/rfc/rfc2616.txt)**。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    cache_control: Option<String>,
    /// 指定该Object被下载时的名称。更多信息，请参见**[RFC2616](https://www.ietf.org/rfc/rfc2616.txt)**。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    content_disposition: Option<String>,
    /// 指定该Object的内容编码格式。更多信息，请参见**[RFC2616](https://www.ietf.org/rfc/rfc2616.txt)**。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    content_encoding: Option<String>,
    /// 是一串由MD5算法生成的值，该请求头用于检查消息内容是否与发送时一致。
    /// 获取Content-MD5值：对消息内容（不包括头部）执行MD5算法，获得128比特位数字，然后对该数字进行base64编码。
    /// 默认值：无
    /// 限制：无
    #[setters(generate = true, strip_option)]
    content_md5: Option<String>,
    /// 过期时间。更多信息，请参见**[RFC2616](https://www.ietf.org/rfc/rfc2616.txt)**。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    expires: Option<String>,
    /// 请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for AppendObject {}

impl AppendObject {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        position: impl Into<i64>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            position: position.into(),
            x_oss_server_side_encryption: None,
            x_oss_object_acl: None,
            x_oss_storage_class: None,
            x_oss_meta: None,
            cache_control: None,
            content_disposition: None,
            content_encoding: None,
            content_md5: None,
            expires: None,
            body: None,
        }
    }
}

impl crate::Request for AppendObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "AppendObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::XmlResponseWrap<AppendObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("append".into(), "".into()));
        params.push(("position".into(), (&self.position).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(9);

        if let Some(f) = &self.cache_control {
            headers.push(("Cache-Control".into(), f.to_string()));
        }

        if let Some(f) = &self.content_disposition {
            headers.push(("Content-Disposition".into(), f.to_string()));
        }

        if let Some(f) = &self.content_encoding {
            headers.push(("Content-Encoding".into(), f.to_string()));
        }

        if let Some(f) = &self.content_md5 {
            headers.push(("Content-MD5".into(), f.to_string()));
        }

        if let Some(f) = &self.expires {
            headers.push(("Expires".into(), f.to_string()));
        }

        if let Some(map) = &self.x_oss_meta {
            for (key, value) in map {
                headers.push((format!("{}{}", "x-oss-meta-", key).into(), value.clone()));
            }
        }

        if let Some(f) = &self.x_oss_object_acl {
            headers.push(("x-oss-object-acl".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_encryption {
            headers.push(("x-oss-server-side-encryption".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_storage_class {
            headers.push(("x-oss-storage-class".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body.unwrap_or_default())
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-next-append-position") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.x_oss_next_append_position = Some(parsed);
                }
            }
        }
        if let Some(value) = headers.get("x-oss-hash-crc64ecma") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_hash_crc64ecma = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SealAppendObject {
    /// 存储空间名称
    bucket: String,
    /// Appendable Object的名称
    key: String,
    /// 用于指定从用户期望Seal时该文件的长度。
    position: i64,
}

impl sealed::Bound for SealAppendObject {}

impl SealAppendObject {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        position: impl Into<i64>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            position: position.into(),
        }
    }
}
impl crate::ToFormData for SealAppendObject {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for SealAppendObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SealAppendObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<SealAppendObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("seal".into(), "".into()));
        params.push(("position".into(), (&self.position).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-sealed-time") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_sealed_time = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteObject {
    /// Bucket信息。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// Object对应的版本ID。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for DeleteObject {}

impl DeleteObject {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
        }
    }
}
impl crate::ToFormData for DeleteObject {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteObject {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::SelfResponseWrap<DeleteObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-delete-marker") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<bool>() {
                    inner.x_oss_delete_marker = Some(parsed);
                }
            }
        }
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct HeadObject {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// 请求Object的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
    /// 如果传入参数中的时间早于实际修改时间，则返回200 OK和Object Meta；否则返回304 not modified。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    if_modified_since: Option<String>,
    /// 如果传入参数中的时间等于或者晚于文件实际修改时间，则返回200 OK和Object Meta；否则返回412 precondition failed。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    if_unmodified_since: Option<String>,
    /// 如果传入期望的ETag和Object的 ETag匹配，则返回200 OK和Object Meta；否则返回412 precondition failed。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    if_match: Option<String>,
    /// 如果传入期望的ETag值和Object的ETag不匹配，则返回200 OK和Object Meta；否则返回304 Not Modified。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    if_none_match: Option<String>,
}

impl sealed::Bound for HeadObject {}

impl HeadObject {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
            if_modified_since: None,
            if_unmodified_since: None,
            if_match: None,
            if_none_match: None,
        }
    }
}

impl crate::Request for HeadObject {
    const METHOD: http::Method = http::Method::HEAD;

    const ACTION: &'static str = "HeadObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<HeadObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(4);

        if let Some(f) = &self.if_match {
            headers.push(("If-Match".into(), f.to_string()));
        }

        if let Some(f) = &self.if_modified_since {
            headers.push(("If-Modified-Since".into(), f.to_string()));
        }

        if let Some(f) = &self.if_none_match {
            headers.push(("If-None-Match".into(), f.to_string()));
        }

        if let Some(f) = &self.if_unmodified_since {
            headers.push(("If-Unmodified-Since".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {}

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-server-side-encryption") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_server_side_encryption = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-server-side-encryption-key-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_server_side_encryption_key_id = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-storage-class") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_storage_class = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-object-type") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_object_type = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-next-append-position") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.x_oss_next_append_position = Some(parsed);
                }
            }
        }
        if let Some(value) = headers.get("x-oss-hash-crc64ecma") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_hash_crc64ecma = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-expiration") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_expiration = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-restore") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_restore = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-process-status") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_process_status = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-request-charged") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_request_charged = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Content-Md5") {
            if let Ok(s) = value.to_str() {
                inner.content_md5 = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Content-Length") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.content_length = Some(parsed);
                }
            }
        }
        if let Some(value) = headers.get("Last-Modified") {
            if let Ok(s) = value.to_str() {
                inner.last_modified = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Content-Type") {
            if let Ok(s) = value.to_str() {
                inner.content_type = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("ETag") {
            if let Ok(s) = value.to_str() {
                inner.e_tag = Some(s.to_string());
            }
        }
        // Handle wildcard headers matching prefix "x-oss-meta-"
        for (k, v) in headers
            .iter()
            .filter(|(k, _)| k.as_str().starts_with("x-oss-meta-"))
        {
            let key = k.as_str().trim_start_matches("x-oss-meta-");
            if let Ok(value) = v.to_str() {
                inner.x_oss_meta.insert(key.to_owned(), value.to_owned());
            }
        }
        if let Some(value) = headers.get("x-oss-transition-time") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_transition_time = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-tagging-count") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.x_oss_tagging_count = Some(parsed);
                }
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetObjectMeta {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// Object的版本ID。只有查看Object指定版本的元数据信息时才显示该字段。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObjectMeta {}

impl GetObjectMeta {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
        }
    }
}

impl crate::Request for GetObjectMeta {
    const METHOD: http::Method = http::Method::HEAD;

    const ACTION: &'static str = "GetObjectMeta";
    const URL_PATH: &'static str = "/{key}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetObjectMetaResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("objectMeta".into(), "".into()));

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {}

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("ETag") {
            if let Ok(s) = value.to_str() {
                inner.e_tag = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Content-Length") {
            if let Ok(s) = value.to_str() {
                if let Ok(parsed) = s.parse::<i64>() {
                    inner.content_length = Some(parsed);
                }
            }
        }
        if let Some(value) = headers.get("x-oss-last-access-time") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_last_access_time = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("Last-Modifed") {
            if let Ok(s) = value.to_str() {
                inner.last_modifed = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-transition-time") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_transition_time = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RestoreObject {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// 请求解冻的Obejct的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
    /// 保存解冻请求信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<RestoreRequest>,
}

impl sealed::Bound for RestoreObject {}

impl RestoreObject {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
            body: None,
        }
    }
}

impl crate::Request for RestoreObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RestoreObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::XmlBody<RestoreRequest>;

    type ResponseWrap = crate::XmlResponseWrap<RestoreObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("restore".into(), "".into()));

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-object-restore-priority") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_object_restore_priority = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CleanRestoredObject {
    /// 存储空间名称
    bucket: String,
    /// 要清理的解冻副本文件名称
    key: String,
}

impl sealed::Bound for CleanRestoredObject {}

impl CleanRestoredObject {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
        }
    }
}
impl crate::ToFormData for CleanRestoredObject {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CleanRestoredObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CleanRestoredObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("cleanRestoredObject".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SelectObject {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// 如果是csv文件，该值需要设置为csv/select；如果是json文件，则需要设置为json/select。
    x_oss_process: String,
    /// 保存SelectObject请求的容器。
    #[setters(generate = true, strip_option)]
    body: Option<SelectRequest>,
}

impl sealed::Bound for SelectObject {}

impl SelectObject {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        x_oss_process: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            x_oss_process: x_oss_process.into(),
            body: None,
        }
    }
}

impl crate::Request for SelectObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SelectObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::XmlBody<SelectRequest>;

    type ResponseWrap = crate::BinaryResponseWrap;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("x-oss-process".into(), (&self.x_oss_process).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateSelectObjectMeta {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// 如果是csv文件，该值需要设置为csv/meta；如果是json文件，则需要设置为json/meta。
    x_oss_process: String,
    /// 保存CreateSelectObjectMeta请求的容器。
    #[setters(generate = true, strip_option)]
    body: Option<SelectMetaRequest>,
}

impl sealed::Bound for CreateSelectObjectMeta {}

impl CreateSelectObjectMeta {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        x_oss_process: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            x_oss_process: x_oss_process.into(),
            body: None,
        }
    }
}

impl crate::Request for CreateSelectObjectMeta {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateSelectObjectMeta";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::XmlBody<SelectMetaRequest>;

    type ResponseWrap = crate::XmlResponseWrap<SelectMetaStatus>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("x-oss-process".into(), (&self.x_oss_process).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct InitiateMultipartUpload {
    /// 初始化一个Multipart Upload事件的Bucket名称。
    ///
    bucket: String,
    /// 初始化一个Multipart Upload事件的Object名称。
    ///
    key: String,
    /// 指定对返回的Key进行编码，目前支持URL编码。Key使用UTF-8字符，但XML 1.0标准不支持解析一些控制字符，例如ASCII值从0到10的字符。对于Key中包含XML
    ///                               1.0标准不支持的控制字符，可以通过指定encoding-type对返回的Key进行编码。
    /// 默认值：无
    /// 可选值：**url**
    #[setters(generate = true, strip_option)]
    encoding_type: Option<EncodeType>,
    /// 指定InitiateMultipartUpload操作时是否覆盖同名Object。当目标Bucket处于已开启或已暂停的版本控制状态时，**x-oss-forbid-overwrite**请求Header设置无效，即允许覆盖同名Object。
    ///   - 不指定**x-oss-forbid-overwrite**或者指定**x-oss-forbid-overwrite**为**false**时，表示允许覆盖同名Object。
    ///   - 指定**x-oss-forbid-overwrite**为**true**时，表示禁止覆盖同名Object。
    ///
    /// 设置**x-oss-forbid-overwrite**请求Header会导致QPS处理性能下降，如果您有大量的操作需要使用**x-oss-forbid-overwrite**请求Header（QPS>1000），请联系技术支持，避免影响您的业务。
    #[setters(generate = true, strip_option)]
    x_oss_forbid_overwrite: Option<String>,
    /// 指定Object的存储类型。
    /// 对于任意存储类型Bucket，如果上传Object时指定该值，则此次上传的Object将存储为指定的类型。例如在IA类型的Bucket中上传Object时，如果指定**x-oss-storage-class**为Standard，则该Object直接存储为Standard类型。
    /// 取值：
    ///
    /// - Standard：标准存储
    /// - IA：低频访问
    /// - Archive：归档存储
    /// - ColdArchive：冷归档存储
    ///
    /// 关于存储类型的更多信息，请参见**[存储类型介绍](~~51374~~)**。
    #[setters(generate = true, strip_option)]
    x_oss_storage_class: Option<StorageClass>,
    /// 指定Object的标签，可同时设置多个标签，例如： TagA=A&TagB=B
    /// > Key和Value需要先进行URL编码，如果某项没有`=`，则看作Value为空字符串。
    #[setters(generate = true, strip_option)]
    x_oss_tagging: Option<String>,
    /// 指定上传该Object的每个part时使用的服务器端加密方式。
    /// 取值：**AES256**、**KMS**或**SM4**
    /// > 使用KMS加密算法前，您需要先开通密钥管理服务KMS。
    ///
    ///
    /// 指定此参数后，在响应头中会返回此参数，OSS会对上传的每个part进行加密编码存储。当下载该Object时，响应头中会包含x-oss-server-side-encryption，且该值会被设置成此Object的加密算法。
    #[setters(generate = true, strip_option)]
    x_oss_server_side_encryption: Option<String>,
    /// 指定Object的加密算法。若未指定此选项，表明Object使用AES256加密算法。此选项仅当**x-oss-server-side-encryption**为KMS时有效。
    /// 取值：SM4
    #[setters(generate = true, strip_option)]
    x_oss_server_side_data_encryption: Option<String>,
    /// 表示KMS托管的用户主密钥。
    /// 该参数在**x-oss-server-side-encryption**值为KMS时有效。
    #[setters(generate = true, strip_option)]
    x_oss_server_side_encryption_key_id: Option<String>,
    /// 指定该Object被下载时的网页的缓存行为。更多信息，请参见**[RFC 2616](https://www.ietf.org/rfc/rfc2616.txt)**。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    cache_control: Option<String>,
    /// 指定该Object被下载时的名称。更多信息，请参见**[RFC 2616](https://www.ietf.org/rfc/rfc2616.txt)**。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    content_disposition: Option<String>,
    /// 指定该Object被下载时的内容编码格式。更多信息，请参见**[RFC 2616](https://www.ietf.org/rfc/rfc2616.txt)**。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    content_encoding: Option<String>,
    /// 过期时间，单位为毫秒。更多信息，请参见**[RFC 2616](https://www.ietf.org/rfc/rfc2616.txt)**。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    expires: Option<String>,
}

impl sealed::Bound for InitiateMultipartUpload {}

impl InitiateMultipartUpload {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            encoding_type: None,
            x_oss_forbid_overwrite: None,
            x_oss_storage_class: None,
            x_oss_tagging: None,
            x_oss_server_side_encryption: None,
            x_oss_server_side_data_encryption: None,
            x_oss_server_side_encryption_key_id: None,
            cache_control: None,
            content_disposition: None,
            content_encoding: None,
            expires: None,
        }
    }
}
impl crate::ToFormData for InitiateMultipartUpload {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for InitiateMultipartUpload {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "InitiateMultipartUpload";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<InitiateMultipartUploadResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("uploads".into(), "".into()));

        if let Some(f) = &self.encoding_type {
            params.push(("encoding-type".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(10);

        if let Some(f) = &self.cache_control {
            headers.push(("Cache-Control".into(), f.to_string()));
        }

        if let Some(f) = &self.content_disposition {
            headers.push(("Content-Disposition".into(), f.to_string()));
        }

        if let Some(f) = &self.content_encoding {
            headers.push(("Content-Encoding".into(), f.to_string()));
        }

        if let Some(f) = &self.expires {
            headers.push(("Expires".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_forbid_overwrite {
            headers.push(("x-oss-forbid-overwrite".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_data_encryption {
            headers.push(("x-oss-server-side-data-encryption".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_encryption {
            headers.push(("x-oss-server-side-encryption".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_server_side_encryption_key_id {
            headers.push(("x-oss-server-side-encryption-key-id".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_storage_class {
            headers.push(("x-oss-storage-class".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_tagging {
            headers.push(("x-oss-tagging".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UploadPart {
    /// Bucket名称。
    bucket: String,
    /// Object完整路径。
    key: String,
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
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        part_number: impl Into<i64>,
        upload_id: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            part_number: part_number.into(),
            upload_id: upload_id.into(),
            body: None,
        }
    }
}

impl crate::Request for UploadPart {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UploadPart";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::OctetStream;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("partNumber".into(), (&self.part_number).into()));
        params.push(("uploadId".into(), (&self.upload_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CompleteMultipartUpload {
    /// Bucket名称。
    bucket: String,
    /// Object完整路径。
    key: String,
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
    encoding_type: Option<EncodeType>,
    /// 指定CompleteMultipartUpload操作时是否覆盖同名Object。
    ///
    /// - 不指定x-oss-forbid-overwrite或者指定x-oss-forbid-overwrite为false时，表示允许覆盖同名Object。
    /// - 指定x-oss-forbid-overwrite为true时，表示禁止覆盖同名Object。
    ///
    /// >
    /// - 当目标Bucket的版本控制状态为“开启”或“暂停”时，x-oss-forbid-overwrite请求Header设置无效，即允许覆盖同名Object。
    /// - 设置x-oss-forbid-overwrite请求Header会导致QPS处理性能下降，如果您有大量的操作需要使用x-oss-forbid-overwrite请求Header（QPS > 1000），请工单联系我们进行确认，避免影响您的业务。
    #[setters(generate = true, strip_option)]
    x_oss_forbid_overwrite: Option<String>,
    /// 指定是否列举当前UploadId已上传的所有Part。
    ///
    /// - 如果指定了x-oss-complete-all:yes，则OSS会列举当前UploadId已上传的所有Part，然后按照PartNumber的序号排序并执行CompleteMultipartUpload操作。执行CompleteMultipartUpload过程中无法检测正在上传或者漏传的Part，因此用户需要自己确保Part的完整性。
    /// - 如果指定了x-oss-complete-all:yes，则不允许继续指定body，否则报错。
    /// - 如果指定了x-oss-complete-all:yes，response的格式保持不变。
    #[setters(generate = true, strip_option)]
    x_oss_complete_all: Option<String>,
    /// 保存CompleteMultipartUpload请求内容的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<CompleteMultipartUploadSchema>,
}

impl sealed::Bound for CompleteMultipartUpload {}

impl CompleteMultipartUpload {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        upload_id: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            upload_id: upload_id.into(),
            encoding_type: None,
            x_oss_forbid_overwrite: None,
            x_oss_complete_all: None,
            body: None,
        }
    }
}

impl crate::Request for CompleteMultipartUpload {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CompleteMultipartUpload";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::XmlBody<CompleteMultipartUploadSchema>;

    type ResponseWrap = crate::XmlResponseWrap<CompleteMultipartUploadResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

        if let Some(f) = &self.encoding_type {
            params.push(("encoding-type".into(), (f).into()));
        }
        params.push(("uploadId".into(), (&self.upload_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(2);

        if let Some(f) = &self.x_oss_complete_all {
            headers.push(("x-oss-complete-all".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_forbid_overwrite {
            headers.push(("x-oss-forbid-overwrite".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UploadPartCopy {
    /// Bucket名称。
    bucket: String,
    /// Object完整路径。
    key: String,
    /// 拷贝源地址（必须有可读权限）。
    x_oss_copy_source: String,
    /// 每一个上传的Part都有一个标识它的号码（partNumber）。
    ///
    /// 取值：1~10000
    ///
    /// 单个Part的大小限制为100 KB~5 GB。
    /// > MultipartUpload事件中除最后一个Part以外，其他Part的大小都要大于或等于100 KB。因不确定是否为最后一个Part，UploadPart接口并不会立即校验上传Part的大小，只有当CompleteMultipartUpload时才会校验。
    part_number: i64,
    /// uploadId用于唯一标识上传的Part属于哪个Object。
    upload_id: String,
    /// 源Object的拷贝范围。例如设置bytes=0~9，表示拷贝0到9这10个字符。
    /// 默认值：无
    ///   - 不指定该请求头时，表示拷贝整个源Object。
    ///   - 当指定该请求头时，则返回消息中会包含整个文件的长度和此次拷贝的范围，例如：Content-Range: bytes 0~9/44，表示整个文件长度为44，此次拷贝的范围为0~9。
    ///   - 当指定的范围不符合规范时，则拷贝整个源Object，并且不在结果中提及Content-Range。
    #[setters(generate = true, strip_option)]
    x_oss_copy_source_range: Option<String>,
    /// 如果源Object的ETAG值和用户提供的ETAG相等，则执行拷贝操作；否则返回412 HTTP错误码（预处理失败）。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    x_oss_copy_source_if_match: Option<String>,
    /// 如果传入的ETag值和Object的ETag不匹配，则正常传输文件，并返回200 OK；否则返回304 Not Modified。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    x_oss_copy_source_if_none_match: Option<String>,
    /// 如果传入参数中的时间等于或者晚于文件实际修改时间，则正常传输文件，并返回200 OK；否则返回412 precondition failed错误。
    /// 默认值：无
    #[setters(generate = true, strip_option)]
    x_oss_copy_source_if_unmodified_since: Option<String>,
    /// 如果指定的时间早于实际修改时间，则正常传送文件，并返回200 OK；否则返回304 not modified。
    /// 默认值：无
    /// 时间格式：GMT时间，例如Fri, 13 Nov 2015 14:47:53 GMT
    #[setters(generate = true, strip_option)]
    x_oss_copy_source_if_modified_since: Option<String>,
}

impl sealed::Bound for UploadPartCopy {}

impl UploadPartCopy {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        x_oss_copy_source: impl Into<String>,
        part_number: impl Into<i64>,
        upload_id: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            x_oss_copy_source: x_oss_copy_source.into(),
            part_number: part_number.into(),
            upload_id: upload_id.into(),
            x_oss_copy_source_range: None,
            x_oss_copy_source_if_match: None,
            x_oss_copy_source_if_none_match: None,
            x_oss_copy_source_if_unmodified_since: None,
            x_oss_copy_source_if_modified_since: None,
        }
    }
}
impl crate::ToFormData for UploadPartCopy {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for UploadPartCopy {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UploadPartCopy";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<UploadPartCopyResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("partNumber".into(), (&self.part_number).into()));
        params.push(("uploadId".into(), (&self.upload_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(6);
        headers.push((
            "x-oss-copy-source".into(),
            self.x_oss_copy_source.to_string(),
        ));

        if let Some(f) = &self.x_oss_copy_source_if_match {
            headers.push(("x-oss-copy-source-if-match".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_copy_source_if_modified_since {
            headers.push(("x-oss-copy-source-if-modified-since".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_copy_source_if_none_match {
            headers.push(("x-oss-copy-source-if-none-match".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_copy_source_if_unmodified_since {
            headers.push((
                "x-oss-copy-source-if-unmodified-since".into(),
                f.to_string(),
            ));
        }

        if let Some(f) = &self.x_oss_copy_source_range {
            headers.push(("x-oss-copy-source-range".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-copy-source-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_copy_source_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AbortMultipartUpload {
    /// Bucket名称。
    bucket: String,
    /// Object完整路径。
    key: String,
    /// 此次MultipartUpload事件的唯一标识。
    upload_id: String,
}

impl sealed::Bound for AbortMultipartUpload {}

impl AbortMultipartUpload {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        upload_id: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            upload_id: upload_id.into(),
        }
    }
}
impl crate::ToFormData for AbortMultipartUpload {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for AbortMultipartUpload {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "AbortMultipartUpload";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("uploadId".into(), (&self.upload_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListMultipartUploads {
    /// Bucket名称。
    bucket: String,
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
    encoding_type: Option<EncodeType>,
}

impl sealed::Bound for ListMultipartUploads {}

impl ListMultipartUploads {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
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

    type ResponseWrap = crate::XmlResponseWrap<ListMultipartUploadsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(7);
        params.push(("uploads".into(), "".into()));

        if let Some(f) = &self.delimiter {
            params.push(("delimiter".into(), (f).into()));
        }

        if let Some(f) = &self.encoding_type {
            params.push(("encoding-type".into(), (f).into()));
        }

        if let Some(f) = &self.key_marker {
            params.push(("key-marker".into(), (f).into()));
        }

        if let Some(f) = &self.max_uploads {
            params.push(("max-uploads".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        if let Some(f) = &self.upload_id_marker {
            params.push(("upload-id-marker".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListParts {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
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
    encoding_type: Option<EncodeType>,
}

impl sealed::Bound for ListParts {}

impl ListParts {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        upload_id: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
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
    const URL_PATH: &'static str = "/{key}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListPartsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(4);

        if let Some(f) = &self.encoding_type {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("encoding-type".into(), json.into()));
            }
        }

        if let Some(f) = &self.max_parts {
            params.push(("max-parts".into(), (f).into()));
        }

        if let Some(f) = &self.part_number_marker {
            params.push(("part-number-marker".into(), (f).into()));
        }
        params.push(("uploadId".into(), (&self.upload_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutObjectAcl {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// 指定OSS创建Object时的访问权限。
    /// 取值：
    ///
    /// - default（默认）：Object遵循所在存储空间的访问权限。
    /// - private：Object是私有资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户没有权限操作该Object。
    /// - public-read：Object是公共读资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户只有该Object的读权限。请谨慎使用该权限。
    /// - public-read-write：Object是公共读写资源。所有用户都有该Object的读写权限。请谨慎使用该权限。
    ///
    /// 关于访问权限的更多信息，请参见**[读写权限ACL](~~100676~~)**。
    x_oss_object_acl: ObjectACL,
    /// Object对应的版本
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for PutObjectAcl {}

impl PutObjectAcl {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        x_oss_object_acl: impl Into<ObjectACL>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            x_oss_object_acl: x_oss_object_acl.into(),
            version_id: None,
        }
    }
}
impl crate::ToFormData for PutObjectAcl {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for PutObjectAcl {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutObjectAcl";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<PutObjectAclResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("acl".into(), "".into()));

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push(("x-oss-object-acl".into(), self.x_oss_object_acl.to_string()));

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetObjectAcl {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// Object对应的版本。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObjectAcl {}

impl GetObjectAcl {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
        }
    }
}

impl crate::Request for GetObjectAcl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObjectAcl";
    const URL_PATH: &'static str = "/{key}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetObjectAclResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("acl".into(), "".into()));

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutSymlink {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// 软链接指向的目标文件。
    /// 合法值：命名规范同Object
    ///   - TargetObjectName同ObjectName一样，需要对其进行URL编码。
    ///   - 软链接的目标文件类型不能为软链接。
    x_oss_symlink_target: String,
    /// 指定OSS创建Object时的访问权限。
    ///
    /// 取值：
    ///
    /// - default（默认）：Object遵循所在存储空间的访问权限。
    /// - private：Object是私有资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户没有权限操作该Object。
    /// - public-read：Object是公共读资源。只有Object的拥有者和授权用户有该Object的读写权限，其他用户只有该Object的读权限。请谨慎使用该权限。
    /// - public-read-write：Object是公共读写资源。所有用户都有该Object的读写权限。请谨慎使用该权限。
    ///
    /// 关于访问权限的更多信息，请参见**[读写权限ACL](~~100676~~)**。
    #[setters(generate = true, strip_option)]
    x_oss_object_acl: Option<ObjectACL>,
    /// 指定Object的存储类型。
    /// 对于任意存储类型的Bucket，如果上传Object时指定此参数，则此次上传的Object将存储为指定的类型。例如在IA类型的Bucket中上传Object时，如果指定**x-oss-storage-class**为Standard，则该Object直接存储为Standard。
    ///
    /// 取值：
    /// - Standard：标准存储
    /// - IA：低频访问
    /// - Archive：归档存储
    ///
    /// IA与Archive类型的单个Object大小如果不足64 KB，则会按64 KB计量计费。建议在使用PutSymlink接口时不要将Object的存储类型指定为IA或Archive。
    /// 关于存储类型的更多信息，请参见**[存储类型介绍](~~51374~~)**。
    #[setters(generate = true, strip_option)]
    x_oss_storage_class: Option<StorageClass>,
    /// 指定PutSymlink操作时是否覆盖同名Object。
    ///   - 不指定**x-oss-forbid-overwrite**或者指定**x-oss-forbid-overwrite**为**false**时，表示允许覆盖同名Object。
    ///   - 指定**x-oss-forbid-overwrite**为**true**时，表示禁止覆盖同名Object。
    ///
    /// 设置**x-oss-forbid-overwrite**请求Header会导致QPS处理性能下降，如果您有大量的操作需要使用**x-oss-forbid-overwrite**请求Header（QPS>1000），请联系技术支持，避免影响您的业务。
    /// > 当目标Bucket处于已开启或已暂停版本控制状态时，**x-oss-forbid-overwrite**请求Header设置无效，即允许覆盖同名Object。
    #[setters(generate = true, strip_option)]
    x_oss_forbid_overwrite: Option<String>,
}

impl sealed::Bound for PutSymlink {}

impl PutSymlink {
    pub fn new(
        bucket: impl Into<String>,
        key: impl Into<String>,
        x_oss_symlink_target: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            x_oss_symlink_target: x_oss_symlink_target.into(),
            x_oss_object_acl: None,
            x_oss_storage_class: None,
            x_oss_forbid_overwrite: None,
        }
    }
}
impl crate::ToFormData for PutSymlink {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for PutSymlink {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutSymlink";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<PutSymlinkResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("symlink".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(4);

        if let Some(f) = &self.x_oss_forbid_overwrite {
            headers.push(("x-oss-forbid-overwrite".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_object_acl {
            headers.push(("x-oss-object-acl".into(), f.to_string()));
        }

        if let Some(f) = &self.x_oss_storage_class {
            headers.push(("x-oss-storage-class".into(), f.to_string()));
        }
        headers.push((
            "x-oss-symlink-target".into(),
            self.x_oss_symlink_target.to_string(),
        ));

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetSymlink {
    /// Bucket名称。
    bucket: String,
    /// Object完整路径。
    key: String,
    /// 获取软链接的当前Object版本。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetSymlink {}

impl GetSymlink {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
        }
    }
}

impl crate::Request for GetSymlink {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetSymlink";
    const URL_PATH: &'static str = "/{key}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetSymlinkResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("symlink".into(), "".into()));

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {}

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-symlink-target") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_symlink_target = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutObjectTagging {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// 版本对应的ID。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
    /// 保存标签集合的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Tagging>,
}

impl sealed::Bound for PutObjectTagging {}

impl PutObjectTagging {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
            body: None,
        }
    }
}

impl crate::Request for PutObjectTagging {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutObjectTagging";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::XmlBody<Tagging>;

    type ResponseWrap = crate::XmlResponseWrap<PutObjectTaggingResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("tagging".into(), "".into()));

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Unwrap the response wrapper to access inner response struct
        let inner = &mut resp.inner;
        if let Some(value) = headers.get("x-oss-version-id") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_version_id = Some(s.to_string());
            }
        }
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetObjectTagging {
    /// Bucket名称。
    bucket: String,
    /// Object的完整路径。
    key: String,
    /// 获取的目标Object的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObjectTagging {}

impl GetObjectTagging {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
        }
    }
}

impl crate::Request for GetObjectTagging {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObjectTagging";
    const URL_PATH: &'static str = "/{key}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetObjectTaggingResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("tagging".into(), "".into()));

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteObjectTagging {
    /// bucket名称。
    bucket: String,
    /// Object完整路径。
    key: String,
    /// 删除的Object的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for DeleteObjectTagging {}

impl DeleteObjectTagging {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
        }
    }
}
impl crate::ToFormData for DeleteObjectTagging {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteObjectTagging {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteObjectTagging";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("tagging".into(), "".into()));

        if let Some(f) = &self.version_id {
            params.push(("versionId".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{key}", self.key.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.key).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutLiveChannel {
    /// Bucket名称。
    bucket: String,
    /// 频道名称。不能包含斜杠（/）。
    channel: String,
    /// 保存LiveChannel配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<LiveChannelConfiguration>,
}

impl sealed::Bound for PutLiveChannel {}

impl PutLiveChannel {
    pub fn new(bucket: impl Into<String>, channel: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            channel: channel.into(),
            body: None,
        }
    }
}

impl crate::Request for PutLiveChannel {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutLiveChannel";
    const URL_PATH: &'static str = "/{channel}";

    type Body = crate::XmlBody<LiveChannelConfiguration>;

    type ResponseWrap = crate::XmlResponseWrap<PutLiveChannelResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("live".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{channel}", self.channel.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.channel).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListLiveChannel {
    /// Bucket名称。
    bucket: String,
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
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
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

    type ResponseWrap = crate::XmlResponseWrap<ListLiveChannelResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(4);
        params.push(("live".into(), "".into()));

        if let Some(f) = &self.marker {
            params.push(("marker".into(), (f).into()));
        }

        if let Some(f) = &self.max_keys {
            params.push(("max-keys".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteLiveChannel {
    /// Bucket名称。
    bucket: String,
    /// 指定LiveChannel名称，该LiveChannel必须存在。
    channel: String,
}

impl sealed::Bound for DeleteLiveChannel {}

impl DeleteLiveChannel {
    pub fn new(bucket: impl Into<String>, channel: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            channel: channel.into(),
        }
    }
}
impl crate::ToFormData for DeleteLiveChannel {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteLiveChannel {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteLiveChannel";
    const URL_PATH: &'static str = "/{channel}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("live".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{channel}", self.channel.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.channel).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutLiveChannelStatus {
    /// Bucket名称。
    bucket: String,
    /// 指定LiveChannel名称，该LiveChannel必须存在。
    channel: String,
    /// 设置LiveChannel的Status。
    /// 有效值：
    /// - enabled：启用LiveChannel
    /// - disabled：禁用LiveChannel
    status: String,
}

impl sealed::Bound for PutLiveChannelStatus {}

impl PutLiveChannelStatus {
    pub fn new(
        bucket: impl Into<String>,
        channel: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            channel: channel.into(),
            status: status.into(),
        }
    }
}
impl crate::ToFormData for PutLiveChannelStatus {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for PutLiveChannelStatus {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutLiveChannelStatus";
    const URL_PATH: &'static str = "/{channel}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("live".into(), "".into()));
        params.push(("status".into(), (&self.status).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{channel}", self.channel.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.channel).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLiveChannelInfo {
    /// Bucket名称。
    bucket: String,
    /// 频道名称。不能包含斜杠（/）。
    channel: String,
}

impl sealed::Bound for GetLiveChannelInfo {}

impl GetLiveChannelInfo {
    pub fn new(bucket: impl Into<String>, channel: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            channel: channel.into(),
        }
    }
}

impl crate::Request for GetLiveChannelInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLiveChannelInfo";
    const URL_PATH: &'static str = "/{channel}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetLiveChannelInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("live".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{channel}", self.channel.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.channel).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLiveChannelHistory {
    /// Bucket名称。
    bucket: String,
    /// LiveChannel名称。
    channel: String,
}

impl sealed::Bound for GetLiveChannelHistory {}

impl GetLiveChannelHistory {
    pub fn new(bucket: impl Into<String>, channel: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            channel: channel.into(),
        }
    }
}

impl crate::Request for GetLiveChannelHistory {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLiveChannelHistory";
    const URL_PATH: &'static str = "/{channel}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetLiveChannelHistoryResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("live".into(), "".into()));
        params.push(("comp".into(), "history".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{channel}", self.channel.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.channel).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLiveChannelStat {
    /// Bucket名称。
    bucket: String,
    /// Livechannel名称。
    channel: String,
}

impl sealed::Bound for GetLiveChannelStat {}

impl GetLiveChannelStat {
    pub fn new(bucket: impl Into<String>, channel: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            channel: channel.into(),
        }
    }
}

impl crate::Request for GetLiveChannelStat {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLiveChannelStat";
    const URL_PATH: &'static str = "/{channel}";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetLiveChannelStatResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("live".into(), "".into()));
        params.push(("comp".into(), "stat".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{channel}", self.channel.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.channel).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetVodPlaylist {
    /// Bucket名称。
    bucket: String,
    /// 指定LiveChannel名称。该LiveChannel必须存在。
    channel: String,
    /// 指定查询ts文件的终止时间，格式为Unix timestamp。
    /// > EndTime必须大于 StartTime，且时间跨度不能大于1天。
    end_time: String,
    /// 指定查询ts文件的起始时间，格式为Unix timestamp。
    start_time: String,
}

impl sealed::Bound for GetVodPlaylist {}

impl GetVodPlaylist {
    pub fn new(
        bucket: impl Into<String>,
        channel: impl Into<String>,
        end_time: impl Into<String>,
        start_time: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            channel: channel.into(),
            end_time: end_time.into(),
            start_time: start_time.into(),
        }
    }
}

impl crate::Request for GetVodPlaylist {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetVodPlaylist";
    const URL_PATH: &'static str = "/{channel}";

    type Body = ();

    type ResponseWrap = crate::BinaryResponseWrap;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("vod".into(), "".into()));
        params.push(("endTime".into(), (&self.end_time).into()));
        params.push(("startTime".into(), (&self.start_time).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{channel}", self.channel.to_string())])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}", self.bucket, self.channel).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PostVodPlaylist {
    /// Bucket名称。
    bucket: String,
    /// 指定LiveChannel名称，该LiveChannel必须存在。
    channel: String,
    /// 指定生成的点播播放列表的名称，必须以“.m3u8”结尾。
    playlist: String,
    /// 指定查询ts文件的终止时间。
    /// 格式：Unix timestamp
    /// >EndTime必须大于StartTime，且时间跨度不能大于1天。
    end_time: String,
    /// 指定查询ts文件的起始时间，格式为Unix timestamp。
    start_time: String,
}

impl sealed::Bound for PostVodPlaylist {}

impl PostVodPlaylist {
    pub fn new(
        bucket: impl Into<String>,
        channel: impl Into<String>,
        playlist: impl Into<String>,
        end_time: impl Into<String>,
        start_time: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            channel: channel.into(),
            playlist: playlist.into(),
            end_time: end_time.into(),
            start_time: start_time.into(),
        }
    }
}
impl crate::ToFormData for PostVodPlaylist {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for PostVodPlaylist {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PostVodPlaylist";
    const URL_PATH: &'static str = "/{channel}/{playlist}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("vod".into(), "".into()));
        params.push(("endTime".into(), (&self.end_time).into()));
        params.push(("startTime".into(), (&self.start_time).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{channel}", self.channel.to_string()),
            ("{playlist}", self.playlist.to_string()),
        ])
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/{}/{}", self.bucket, self.channel, self.playlist).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutChannel {
    /// 存储空间名称
    bucket: String,
    /// 接口请求体
    #[setters(generate = true, strip_option)]
    body: Option<Channel>,
}

impl sealed::Bound for PutChannel {}

impl PutChannel {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutChannel {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutChannel";

    type Body = crate::XmlBody<Channel>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("img".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketHash {
    /// 存储空间名称
    bucket: String,
    /// 接口请求体
    #[setters(generate = true, strip_option)]
    body: Option<ObjectHashConfiguration>,
}

impl sealed::Bound for PutBucketHash {}

impl PutBucketHash {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketHash {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketHash";

    type Body = crate::XmlBody<ObjectHashConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("objectHash".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketCommonHeader {
    /// 存储空间名称
    bucket: String,
    /// 接口请求体
    #[setters(generate = true, strip_option)]
    body: Option<CommonHeaders>,
}

impl sealed::Bound for PutBucketCommonHeader {}

impl PutBucketCommonHeader {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketCommonHeader {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketCommonHeader";

    type Body = crate::XmlBody<CommonHeaders>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("x-oss-common-header".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketCommonHeader {
    /// 存储空间名称
    bucket: String,
}

impl sealed::Bound for DeleteBucketCommonHeader {}

impl DeleteBucketCommonHeader {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
impl crate::ToFormData for DeleteBucketCommonHeader {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketCommonHeader {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketCommonHeader";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("x-oss-common-header".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutProcessConfiguration {
    /// 存储空间名称
    bucket: String,
    /// 请求结构体
    #[setters(generate = true, strip_option)]
    body: Option<BucketProcessConfiguration>,
}

impl sealed::Bound for PutProcessConfiguration {}

impl PutProcessConfiguration {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            body: None,
        }
    }
}

impl crate::Request for PutProcessConfiguration {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutProcessConfiguration";

    type Body = crate::XmlBody<BucketProcessConfiguration>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("processConfiguration".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetBucketEventNotification {
    /// 存储空间名称
    bucket: String,
}

impl sealed::Bound for GetBucketEventNotification {}

impl GetBucketEventNotification {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for GetBucketEventNotification {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetBucketEventNotification";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<EventNotificationConfiguration>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("eventNotification".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutDataLakeCachePrefetchJob {
    bucket: String,
    #[setters(generate = true, strip_option)]
    x_oss_datalake_job_id: Option<String>,
    #[setters(generate = true, strip_option)]
    body: Option<CreateDataLakeCachePrefetchJob>,
}

impl sealed::Bound for PutDataLakeCachePrefetchJob {}

impl PutDataLakeCachePrefetchJob {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_datalake_job_id: None,
            body: None,
        }
    }
}

impl crate::Request for PutDataLakeCachePrefetchJob {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutDataLakeCachePrefetchJob";

    type Body = crate::XmlBody<CreateDataLakeCachePrefetchJob>;

    type ResponseWrap = crate::XmlResponseWrap<PutDataLakeCachePrefetchJobResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("x-oss-datalake-cache-prefetch-job".into(), "".into()));

        if let Some(f) = &self.x_oss_datalake_job_id {
            params.push(("x-oss-datalake-job-id".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct StartDataLakeCachePrefetchJob {
    bucket: String,
    x_oss_datalake_job_id: String,
}

impl sealed::Bound for StartDataLakeCachePrefetchJob {}

impl StartDataLakeCachePrefetchJob {
    pub fn new(bucket: impl Into<String>, x_oss_datalake_job_id: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            x_oss_datalake_job_id: x_oss_datalake_job_id.into(),
        }
    }
}
impl crate::ToFormData for StartDataLakeCachePrefetchJob {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for StartDataLakeCachePrefetchJob {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "StartDataLakeCachePrefetchJob";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("x-oss-datalake-cache-prefetch-job".into(), "".into()));
        params.push(("x-oss-datalake-job-start".into(), "".into()));
        params.push((
            "x-oss-datalake-job-id".into(),
            (&self.x_oss_datalake_job_id).into(),
        ));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListDataLakeStorageTransferJob {
    bucket: String,
}

impl sealed::Bound for ListDataLakeStorageTransferJob {}

impl ListDataLakeStorageTransferJob {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl crate::Request for ListDataLakeStorageTransferJob {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListDataLakeStorageTransferJob";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<DataLakeStorageTransferJobs>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("x-oss-datalake-storage-transfer-job".into(), "".into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn process_endpoint(&self, endpoint: &'static str) -> std::borrow::Cow<'static, str> {
        format!("{}.{}", self.bucket, endpoint).into()
    }

    fn resource_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/{}/", self.bucket).into()
    }

    fn to_body(self) -> Self::Body {}
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AcceleratePathsPathItem {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "CachePolicy")]
    pub cache_policy: Option<String>,
}

impl crate::FlatSerialize for AcceleratePathsPathItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.cache_policy,
            &format!("{}.CachePolicy", name),
            params,
        );
    }
}

/// 加速器加速路径集合
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AcceleratePaths {
    #[serde(rename = "DefaultCachePolicy")]
    pub default_cache_policy: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: Vec<AcceleratePathsPathItem>,
}

impl crate::FlatSerialize for AcceleratePaths {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.default_cache_policy,
            &format!("{}.DefaultCachePolicy", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.path, &format!("{}.Path", name), params);
    }
}

/// 存储ACL信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessControlList {
    /// Bucket的读写权限ACL。
    #[serde(rename = "Grant")]
    pub grant: Option<ObjectACL>,
}

impl crate::FlatSerialize for AccessControlList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.grant, &format!("{}.Grant", name), params);
    }
}

/// 保存Bucket拥有者信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Owner {
    /// Bucket拥有者的用户ID。
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// Bucket拥有者的名称 （目前和ID一致）。
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
}

impl crate::FlatSerialize for Owner {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.display_name,
            &format!("{}.DisplayName", name),
            params,
        );
    }
}

/// 保存Get Object ACL结果的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessControlPolicy {
    /// 保存Bucket拥有者信息的容器。
    ///
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    /// 存储ACL信息的容器。
    #[serde(rename = "AccessControlList")]
    pub access_control_list: Option<AccessControlList>,
}

impl crate::FlatSerialize for AccessControlPolicy {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.access_control_list,
            &format!("{}.AccessControlList", name),
            params,
        );
    }
}

/// Bucket的访问跟踪状态配置信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessMonitorConfiguration {
    /// Bucket的访问跟踪状态。
    #[serde(rename = "Status")]
    pub status: Option<AccessMonitorStatus>,
}

impl crate::FlatSerialize for AccessMonitorConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

impl crate::ToCodeMessage for AccessMonitorConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 保存VPC网络来源信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessPointVpcConfiguration {
    /// 仅当NetworkOrigin取值为vpc时，需要指定VPC ID。
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
}

impl crate::FlatSerialize for AccessPointVpcConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.vpc_id, &format!("{}.VpcId", name), params);
    }
}

/// 保存单个接入点信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessPoint {
    /// 配置接入点的Bucket名称。
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    pub access_point_name: Option<String>,
    /// 接入点别名。
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
    /// 接入点网络来源。
    #[serde(rename = "NetworkOrigin")]
    pub network_origin: Option<String>,
    /// 保存VPC网络来源信息的容器。
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<AccessPointVpcConfiguration>,
    /// 接入点所处状态。
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

impl crate::FlatSerialize for AccessPoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.access_point_name,
            &format!("{}.AccessPointName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.alias, &format!("{}.Alias", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.network_origin,
            &format!("{}.NetworkOrigin", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.vpc_configuration,
            &format!("{}.VpcConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

/// 服务器端默认加密方式的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ApplyServerSideEncryptionByDefault {
    /// 设置服务器端默认加密方式。
    /// 取值：KMS、AES256、SM4
    /// 使用KMS密钥功能时会产生少量的KMS密钥API调用费用，费用详情请参见**[KMS计费标准](~~52608~~)**。
    /// 进行跨区域复制时，若目标Bucket启用了默认服务器端加密方式，且复制规则配置了ReplicaCMKID，有以下两种情况：
    ///   - 若源Bucket中的对象未加密，则使用目标Bucket的默认加密方式对跨区域复制过来的明文对象进行加密。
    ///   - 若源Bucket中的对象使用了SSE-KMS或SSE-OSS的加密方式，则目标Bucket针对这些对象仍然使用原加密方式进行加密。
    ///
    /// 更多信息，请参见**[跨区域复制结合服务器端加密](~~177216~~)**。
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: Option<String>,
    /// 当SSEAlgorithm值为KMS，且使用指定的密钥加密时，需输入KMSMasterKeyID。其他情况下，必须为空。
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_master_key_id: Option<String>,
    /// 指定Object的加密算法。若未指定此选项，表明Object使用AES256加密算法。此选项仅当SSEAlgorithm取值为KMS有效。
    /// 取值：SM4
    #[serde(rename = "KMSDataEncryption")]
    pub kms_data_encryption: Option<String>,
}

impl crate::FlatSerialize for ApplyServerSideEncryptionByDefault {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.sse_algorithm,
            &format!("{}.SSEAlgorithm", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.kms_master_key_id,
            &format!("{}.KMSMasterKeyID", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.kms_data_encryption,
            &format!("{}.KMSDataEncryption", name),
            params,
        );
    }
}

/// 保存归档直读状态的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ArchiveDirectReadConfiguration {
    /// Bucket是否开启归档直读。取值如下：
    /// - true：开启归档直读。
    /// - false：关闭归档直读。
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
}

impl crate::FlatSerialize for ArchiveDirectReadConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.enabled, &format!("{}.Enabled", name), params);
    }
}

impl crate::ToCodeMessage for ArchiveDirectReadConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 异步 Fetch 任务配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AsyncFetchTaskConfiguration {
    #[serde(rename = "Url")]
    pub url: Option<String>,
    #[serde(rename = "Object")]
    pub object: Option<String>,
    #[serde(rename = "Host")]
    pub host: Option<String>,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<String>,
    #[serde(rename = "Callback")]
    pub callback: Option<String>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<String>,
    #[serde(rename = "IgnoreSameKey")]
    pub ignore_same_key: Option<bool>,
}

impl crate::FlatSerialize for AsyncFetchTaskConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.url, &format!("{}.Url", name), params);
        crate::FlatSerialize::flat_serialize(&self.object, &format!("{}.Object", name), params);
        crate::FlatSerialize::flat_serialize(&self.host, &format!("{}.Host", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.content_md5,
            &format!("{}.ContentMD5", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.callback, &format!("{}.Callback", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.ignore_same_key,
            &format!("{}.IgnoreSameKey", name),
            params,
        );
    }
}

/// 异步 fetch 任务信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AsyncFetchTaskInfo {
    #[serde(rename = "TaskId")]
    pub task_id: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<String>,
    #[serde(rename = "TaskInfo")]
    pub task_info: Option<AsyncFetchTaskConfiguration>,
}

impl crate::FlatSerialize for AsyncFetchTaskInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.task_id, &format!("{}.TaskId", name), params);
        crate::FlatSerialize::flat_serialize(&self.state, &format!("{}.State", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.error_msg,
            &format!("{}.ErrorMsg", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.task_info,
            &format!("{}.TaskInfo", name),
            params,
        );
    }
}

/// 异步 Fetch 任务返回结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AsyncFetchTaskResult {
    #[serde(rename = "TaskId")]
    pub task_id: Option<String>,
}

impl crate::FlatSerialize for AsyncFetchTaskResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.task_id, &format!("{}.TaskId", name), params);
    }
}

/// 保存Bucket信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Bucket {
    /// Bucket创建时间。格式为`yyyy-mm-ddThh:mm:ss.timezone`。
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,
    /// Bucket访问的外网域名。
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: Option<String>,
    /// 同地域ECS访问Bucket的内网域名。
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: Option<String>,
    /// Bucket所在的数据中心。
    #[serde(rename = "Location")]
    pub location: Option<String>,
    /// Bucket名称。
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Bucket存储类型，支持Standard、IA、Archive和ColdArchive四种存储类型。
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    /// Bucket所在地域。
    #[serde(rename = "Region")]
    pub region: Option<String>,
}

impl crate::FlatSerialize for Bucket {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_endpoint,
            &format!("{}.ExtranetEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_endpoint,
            &format!("{}.IntranetEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
    }
}

/// 保存域名信息列表的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAntiDDOSConfigurationCnames {
    /// 待防护自定义域名。
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: Vec<String>,
}

impl crate::FlatSerialize for BucketAntiDDOSConfigurationCnames {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.domain, &format!("{}.Domain", name), params);
    }
}

/// 保存高防实例配置信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAntiDDOSConfiguration {
    /// 保存域名信息列表的容器。
    #[serde(rename = "Cnames")]
    pub cnames: Option<BucketAntiDDOSConfigurationCnames>,
}

impl crate::FlatSerialize for BucketAntiDDOSConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.cnames, &format!("{}.Cnames", name), params);
    }
}

/// 保存自定义域名的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAntiDDOSInfoCnames {
    /// 自定义域名。
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: Vec<String>,
}

impl crate::FlatSerialize for BucketAntiDDOSInfoCnames {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.domain, &format!("{}.Domain", name), params);
    }
}

/// 保存高防实例信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAntiDDOSInfo {
    /// 高防实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,
    /// Bucket拥有者的UID。
    #[serde(rename = "Owner")]
    pub owner: Option<String>,
    /// 防护的Bucket名称。
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    /// 高防实例创建时间，格式为时间戳。
    #[serde(rename = "Ctime")]
    pub ctime: Option<i64>,
    /// 高防实例更新时间，格式为时间戳。
    #[serde(rename = "Mtime")]
    pub mtime: Option<i64>,
    /// 高防实例激活时间，格式为时间戳。
    #[serde(rename = "ActiveTime")]
    pub active_time: Option<i64>,
    /// 高防实例所处状态。
    ///
    /// - Init：初始化防护状态。
    ///
    /// - Defending：防护中状态。
    ///
    /// - HaltDefending：解除防护状态。
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 高防实例类型。固定值为AntiDDosPremimum。
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
    /// 保存自定义域名的容器。
    #[serde(rename = "Cnames")]
    pub cnames: Option<BucketAntiDDOSInfoCnames>,
}

impl crate::FlatSerialize for BucketAntiDDOSInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.instance_id,
            &format!("{}.InstanceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.ctime, &format!("{}.Ctime", name), params);
        crate::FlatSerialize::flat_serialize(&self.mtime, &format!("{}.Mtime", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.active_time,
            &format!("{}.ActiveTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
        crate::FlatSerialize::flat_serialize(&self.cnames, &format!("{}.Cnames", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketChannelConfigRuleListRuleItem {
    /// 规则名称
    #[serde(rename = "RuleName")]
    pub rule_name: Option<String>,
    /// 规则匹配正则
    #[serde(rename = "RuleRegex")]
    pub rule_regex: Option<String>,
    /// 规则内容
    #[serde(rename = "FrontContent")]
    pub front_content: Option<String>,
}

impl crate::FlatSerialize for BucketChannelConfigRuleListRuleItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.rule_name,
            &format!("{}.RuleName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.rule_regex,
            &format!("{}.RuleRegex", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.front_content,
            &format!("{}.FrontContent", name),
            params,
        );
    }
}

/// 规则列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketChannelConfigRuleList {
    /// 保存规则的容器
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: Vec<BucketChannelConfigRuleListRuleItem>,
}

impl crate::FlatSerialize for BucketChannelConfigRuleList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

/// Bucket图片处理通道配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketChannelConfig {
    /// 版本
    #[serde(rename = "version")]
    pub version: Option<i32>,
    /// 调试信息
    #[serde(rename = "DebugInfo")]
    pub debug_info: Option<String>,
    /// 规则列表
    #[serde(rename = "RuleList")]
    pub rule_list: Option<BucketChannelConfigRuleList>,
}

impl crate::FlatSerialize for BucketChannelConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.version, &format!("{}.version", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.debug_info,
            &format!("{}.DebugInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.rule_list,
            &format!("{}.RuleList", name),
            params,
        );
    }
}

/// 保存Cname的证书配置信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CertificateConfiguration {
    /// 证书ID。
    #[serde(rename = "CertId")]
    pub cert_id: Option<String>,
    /// 证书公钥。
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,
    /// 证书私钥。
    #[serde(rename = "PrivateKey")]
    pub private_key: Option<String>,
    /// 当前证书ID。如果Force值不为true，OSS Server会检查该值与当前证书ID是否匹配，不匹配则报错。
    #[serde(rename = "PreviousCertId")]
    pub previous_cert_id: Option<String>,
    /// 是否强制覆盖证书。
    #[serde(rename = "Force")]
    pub force: Option<bool>,
    /// 是否删除证书。
    #[serde(rename = "DeleteCertificate")]
    pub delete_certificate: Option<bool>,
}

impl crate::FlatSerialize for CertificateConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.cert_id, &format!("{}.CertId", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.certificate,
            &format!("{}.Certificate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.private_key,
            &format!("{}.PrivateKey", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.previous_cert_id,
            &format!("{}.PreviousCertId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.force, &format!("{}.Force", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.delete_certificate,
            &format!("{}.DeleteCertificate", name),
            params,
        );
    }
}

/// Cname信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketCnameConfigurationCname {
    /// 自定义域名。
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// 保存证书配置信息的容器
    #[serde(rename = "CertificateConfiguration")]
    pub certificate_configuration: Option<CertificateConfiguration>,
}

impl crate::FlatSerialize for BucketCnameConfigurationCname {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.domain, &format!("{}.Domain", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.certificate_configuration,
            &format!("{}.CertificateConfiguration", name),
            params,
        );
    }
}

/// Cname配置的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketCnameConfiguration {
    /// Cname信息的容器。
    #[serde(rename = "Cname")]
    pub cname: Option<BucketCnameConfigurationCname>,
}

impl crate::FlatSerialize for BucketCnameConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.cname, &format!("{}.Cname", name), params);
    }
}

/// Bucket存储冗余转换任务详情
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketDataRedundancyTransition {
    /// Bucket名称
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    /// 存储冗余转换任务的ID
    #[serde(rename = "TaskId")]
    pub task_id: Option<String>,
    /// 存储冗余转换任务的状态，取值如下：
    ///
    /// - Queueing：队列中。
    /// - Processing：进行中。
    /// - Finished：完成。
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 存储冗余转换任务的创建时间
    #[serde(rename = "CreateTime")]
    pub create_time: Option<String>,
    /// 存储冗余转换任务的开始时间
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,
    /// 存储冗余转换任务的完成时间
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,
    /// 存储冗余转换任务的进度百分比。取值范围：0-100
    #[serde(rename = "ProcessPercentage")]
    pub process_percentage: Option<i32>,
    /// 存储冗余转换任务的预计剩余耗时。单位为小时。
    #[serde(rename = "EstimatedRemainingTime")]
    pub estimated_remaining_time: Option<String>,
}

impl crate::FlatSerialize for BucketDataRedundancyTransition {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.task_id, &format!("{}.TaskId", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.StartTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.end_time, &format!("{}.EndTime", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.process_percentage,
            &format!("{}.ProcessPercentage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.estimated_remaining_time,
            &format!("{}.EstimatedRemainingTime", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for BucketDataRedundancyTransition {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// Bucket服务端加密配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketInfoBucketServerSideEncryptionRule {
    /// 设置服务器端默认加密方式。
    ///
    /// 取值：KMS、AES256、SM4。
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: Option<String>,
    /// 服务端加密KMS密钥
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_master_key_id: Option<String>,
    /// 指定Object的加密算法。如果未指定此选项，表明Object使用AES256加密算法。此选项仅当SSEAlgorithm取值为KMS有效。
    ///
    /// 取值：SM4
    #[serde(rename = "KMSDataEncryption")]
    pub kms_data_encryption: Option<String>,
}

impl crate::FlatSerialize for BucketInfoBucketServerSideEncryptionRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.sse_algorithm,
            &format!("{}.SSEAlgorithm", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.kms_master_key_id,
            &format!("{}.KMSMasterKeyID", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.kms_data_encryption,
            &format!("{}.KMSDataEncryption", name),
            params,
        );
    }
}

/// Bucket日志配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketInfoBucketBucketPolicy {
    /// 存储日志记录的Bucket名称。
    #[serde(rename = "LogBucket")]
    pub log_bucket: Option<String>,
    /// 存储日志文件的目录。
    #[serde(rename = "LogPrefix")]
    pub log_prefix: Option<String>,
}

impl crate::FlatSerialize for BucketInfoBucketBucketPolicy {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.log_bucket,
            &format!("{}.LogBucket", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.log_prefix,
            &format!("{}.LogPrefix", name),
            params,
        );
    }
}

/// 保存Bucket信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketInfoBucket {
    /// Bucket是否开启访问追踪
    #[serde(rename = "AccessMonitor")]
    pub access_monitor: Option<String>,
    /// Bucket的创建时间
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,
    /// Bucket是否开启跨区域复制
    #[serde(rename = "CrossRegionReplication")]
    pub cross_region_replication: Option<String>,
    /// Bucket的数据容灾类型
    #[serde(rename = "DataRedundancyType")]
    pub data_redundancy_type: Option<DataRedundancyType>,
    /// Bucket外网访问域名
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: Option<String>,
    /// Bucket内网访问域名
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: Option<String>,
    /// Bucket所在地域
    #[serde(rename = "Location")]
    pub location: Option<String>,
    /// Bucket名称
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Bucket所在的资源组ID
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: Option<String>,
    /// Bucket的存储类型
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    /// Bucket传输
    #[serde(rename = "TransferAcceleration")]
    pub transfer_acceleration: Option<String>,
    /// Bucket多版本状态
    #[serde(rename = "Versioning")]
    pub versioning: Option<BucketVersioningStatus>,
    /// Bucket所有者
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    /// Bucket权限
    #[serde(rename = "AccessControlList")]
    pub access_control_list: Option<AccessControlList>,
    /// Bucket服务端加密配置
    #[serde(rename = "ServerSideEncryptionRule")]
    pub server_side_encryption_rule: Option<BucketInfoBucketServerSideEncryptionRule>,
    /// Bucket日志配置
    #[serde(rename = "BucketPolicy")]
    pub bucket_policy: Option<BucketInfoBucketBucketPolicy>,
    /// Bucket描述信息。
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    /// Bucket是否开启阻止公共访问
    #[serde(rename = "BlockPublicAccess")]
    pub block_public_access: Option<bool>,
}

impl crate::FlatSerialize for BucketInfoBucket {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_monitor,
            &format!("{}.AccessMonitor", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cross_region_replication,
            &format!("{}.CrossRegionReplication", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.data_redundancy_type,
            &format!("{}.DataRedundancyType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_endpoint,
            &format!("{}.ExtranetEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_endpoint,
            &format!("{}.IntranetEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.resource_group_id,
            &format!("{}.ResourceGroupId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.transfer_acceleration,
            &format!("{}.TransferAcceleration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.versioning,
            &format!("{}.Versioning", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.access_control_list,
            &format!("{}.AccessControlList", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.server_side_encryption_rule,
            &format!("{}.ServerSideEncryptionRule", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bucket_policy,
            &format!("{}.BucketPolicy", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.comment, &format!("{}.Comment", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.block_public_access,
            &format!("{}.BlockPublicAccess", name),
            params,
        );
    }
}

/// 保存Bucket信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketInfo {
    /// 保存Bucket信息的容器
    #[serde(rename = "Bucket")]
    pub bucket: Option<BucketInfoBucket>,
}

impl crate::FlatSerialize for BucketInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
    }
}

impl crate::ToCodeMessage for BucketInfo {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 访问日志信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LoggingEnabled {
    /// 指定存储访问日志的Bucket。
    #[serde(rename = "TargetBucket")]
    pub target_bucket: String,
    /// 指定保存的日志文件前缀，可以为空。
    #[serde(rename = "TargetPrefix")]
    pub target_prefix: Option<String>,
    /// 日志转存授权角色
    #[serde(rename = "LoggingRole")]
    pub logging_role: Option<String>,
}

impl crate::FlatSerialize for LoggingEnabled {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.target_bucket,
            &format!("{}.TargetBucket", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.target_prefix,
            &format!("{}.TargetPrefix", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.logging_role,
            &format!("{}.LoggingRole", name),
            params,
        );
    }
}

/// 存储访问日志状态信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketLoggingStatus {
    /// 访问日志信息的容器。
    #[serde(rename = "LoggingEnabled")]
    pub logging_enabled: LoggingEnabled,
}

impl crate::FlatSerialize for BucketLoggingStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.logging_enabled,
            &format!("{}.LoggingEnabled", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for BucketLoggingStatus {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// Bucket图片处理配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketProcessConfiguration {
    /// 域名
    #[serde(rename = "CompliedHost")]
    pub complied_host: Option<String>,
    /// 是否支持OSS域名
    #[serde(rename = "OssDomainSupportAtProcess")]
    pub oss_domain_support_at_process: Option<String>,
    /// 原图保护
    #[serde(rename = "SourceFileProtect")]
    pub source_file_protect: Option<String>,
    /// 原图保护后缀
    #[serde(rename = "SourceFileProtectSuffix")]
    pub source_file_protect_suffix: Option<String>,
    /// 图片处理频道配置
    #[serde(rename = "BucketChannelConfig")]
    pub bucket_channel_config: Option<BucketChannelConfig>,
    /// 样式分隔符
    #[serde(rename = "StyleDelimiters")]
    pub style_delimiters: Option<String>,
}

impl crate::FlatSerialize for BucketProcessConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.complied_host,
            &format!("{}.CompliedHost", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_domain_support_at_process,
            &format!("{}.OssDomainSupportAtProcess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_file_protect,
            &format!("{}.SourceFileProtect", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_file_protect_suffix,
            &format!("{}.SourceFileProtectSuffix", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bucket_channel_config,
            &format!("{}.BucketChannelConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.style_delimiters,
            &format!("{}.StyleDelimiters", name),
            params,
        );
    }
}

/// 存储空间级别QoS配置信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketQoSConfiguration {
    /// 总上行带宽，单位Gbps
    #[serde(rename = "TotalUploadBandwidth")]
    pub total_upload_bandwidth: Option<i64>,
    /// 内网上行带宽，单位Gbps
    #[serde(rename = "IntranetUploadBandwidth")]
    pub intranet_upload_bandwidth: Option<i64>,
    /// 公网上行带宽，单位Gbps
    #[serde(rename = "ExtranetUploadBandwidth")]
    pub extranet_upload_bandwidth: Option<i64>,
    /// 总下行带宽，单位Gbps
    #[serde(rename = "TotalDownloadBandwidth")]
    pub total_download_bandwidth: Option<i64>,
    /// 内网下行带宽，单位Gbps
    #[serde(rename = "IntranetDownloadBandwidth")]
    pub intranet_download_bandwidth: Option<i64>,
    /// 公网下行带宽，单位Gbps
    #[serde(rename = "ExtranetDownloadBandwidth")]
    pub extranet_download_bandwidth: Option<i64>,
    /// 总QPS
    #[serde(rename = "TotalQps")]
    pub total_qps: Option<i64>,
    /// 内网QPS
    #[serde(rename = "IntranetQps")]
    pub intranet_qps: Option<i64>,
    /// 公网QPS
    #[serde(rename = "ExtranetQps")]
    pub extranet_qps: Option<i64>,
    /// 是否独立与用户级别QoS配置
    #[serde(rename = "Exclusive")]
    pub exclusive: Option<bool>,
}

impl crate::FlatSerialize for BucketQoSConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.total_upload_bandwidth,
            &format!("{}.TotalUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_upload_bandwidth,
            &format!("{}.IntranetUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_upload_bandwidth,
            &format!("{}.ExtranetUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_download_bandwidth,
            &format!("{}.TotalDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_download_bandwidth,
            &format!("{}.IntranetDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_download_bandwidth,
            &format!("{}.ExtranetDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_qps,
            &format!("{}.TotalQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_qps,
            &format!("{}.IntranetQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_qps,
            &format!("{}.ExtranetQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.exclusive,
            &format!("{}.Exclusive", name),
            params,
        );
    }
}

/// Bucket资源组配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketResourceGroupConfiguration {
    /// 所属资源组ID
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: Option<String>,
}

impl crate::FlatSerialize for BucketResourceGroupConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resource_group_id,
            &format!("{}.ResourceGroupId", name),
            params,
        );
    }
}

/// BucketStat结构的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketStat {
    /// Bucket的总存储量，单位字节。
    #[serde(rename = "Storage")]
    pub storage: Option<i64>,
    /// Bucket中总的Object数量。
    #[serde(rename = "ObjectCount")]
    pub object_count: Option<i64>,
    /// Bucket中已经初始化但还未完成（Complete）或者还未中止（Abort）的Multipart Upload数量。
    #[serde(rename = "MultipartUploadCount")]
    pub multipart_upload_count: Option<i64>,
    /// Bucket中Live Channel的数量。
    #[serde(rename = "LiveChannelCount")]
    pub live_channel_count: Option<i64>,
    /// Bucket中上传的Multipart分片的数量。
    #[serde(rename = "MultipartPartCount")]
    pub multipart_part_count: Option<i64>,
    /// Bucket中删除标记的数量。
    #[serde(rename = "DeleteMarkerCount")]
    pub delete_marker_count: Option<i64>,
    /// 获取到的存储信息的时间点，格式为时间戳，单位为秒。
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: Option<i64>,
    /// 标准存储类型的存储量，单位字节。
    #[serde(rename = "StandardStorage")]
    pub standard_storage: Option<i64>,
    /// 标准存储类型的Object数量。
    #[serde(rename = "StandardObjectCount")]
    pub standard_object_count: Option<i64>,
    /// 低频存储类型的计费存储量，单位字节。
    #[serde(rename = "InfrequentAccessStorage")]
    pub infrequent_access_storage: Option<i64>,
    /// 低频存储类型的实际存储量，单位字节。
    #[serde(rename = "InfrequentAccessRealStorage")]
    pub infrequent_access_real_storage: Option<i64>,
    /// 低频存储类型的Object数量。
    #[serde(rename = "InfrequentAccessObjectCount")]
    pub infrequent_access_object_count: Option<i64>,
    /// 归档存储类型的计费存储量，单位字节。
    #[serde(rename = "ArchiveStorage")]
    pub archive_storage: Option<i64>,
    /// 归档存储类型的实际存储量，单位字节。
    #[serde(rename = "ArchiveRealStorage")]
    pub archive_real_storage: Option<i64>,
    /// 归档存储类型的Object数量。
    #[serde(rename = "ArchiveObjectCount")]
    pub archive_object_count: Option<i64>,
    /// 冷归档存储类型的计费存储量，单位字节。
    #[serde(rename = "ColdArchiveStorage")]
    pub cold_archive_storage: Option<i64>,
    /// 冷归档存储类型的实际存储量，单位字节。
    #[serde(rename = "ColdArchiveRealStorage")]
    pub cold_archive_real_storage: Option<i64>,
    /// 冷归档存储类型的Object数量。
    #[serde(rename = "ColdArchiveObjectCount")]
    pub cold_archive_object_count: Option<i64>,
    /// 深度冷归档存储类型的计费存储量，单位字节。
    #[serde(rename = "DeepColdArchiveStorage")]
    pub deep_cold_archive_storage: Option<i64>,
    /// 深度冷归档存储类型的实际存储量，单位字节。
    #[serde(rename = "DeepColdArchiveRealStorage")]
    pub deep_cold_archive_real_storage: Option<i64>,
    /// 深度冷归档存储类型的Object数量。
    #[serde(rename = "DeepColdArchiveObjectCount")]
    pub deep_cold_archive_object_count: Option<i64>,
    /// Bucket中Multipart分片的存储量
    #[serde(rename = "MultipartPartStorage")]
    pub multipart_part_storage: Option<i64>,
    /// Bucket中标准类型的Multipart分片的数量
    #[serde(rename = "StandardMultipartPartCount")]
    pub standard_multipart_part_count: Option<i64>,
    /// Bucket中标准类型的Multipart分片的存储量
    #[serde(rename = "StandardMultipartPartStorage")]
    pub standard_multipart_part_storage: Option<i64>,
    /// Bucket中低频类型的Multipart分片的数量
    #[serde(rename = "InfrequentMultipartPartCount")]
    pub infrequent_multipart_part_count: Option<i64>,
    /// Bucket中低频类型的Multipart分片的存储量
    #[serde(rename = "InfrequentMultipartPartStorage")]
    pub infrequent_multipart_part_storage: Option<i64>,
    /// Bucket中归档类型的Multipart分片的数量
    #[serde(rename = "ArchiveMultipartPartCount")]
    pub archive_multipart_part_count: Option<i64>,
    /// Bucket中归档类型的Multipart分片的存储量
    #[serde(rename = "ArchiveMultipartPartStorage")]
    pub archive_multipart_part_storage: Option<i64>,
    /// Bucket中冷归档类型的Multipart分片的数量
    #[serde(rename = "ColdArchiveMultipartPartCount")]
    pub cold_archive_multipart_part_count: Option<i64>,
    /// Bucket中冷归档类型的Multipart分片的存储量
    #[serde(rename = "ColdArchiveMultipartPartStorage")]
    pub cold_archive_multipart_part_storage: Option<i64>,
    /// Bucket中深度冷归档类型的Multipart分片的数量
    #[serde(rename = "DeepColdArchiveMultipartPartCount")]
    pub deep_cold_archive_multipart_part_count: Option<i64>,
    /// Bucket中深度冷归档类型的Multipart分片的存储量
    #[serde(rename = "DeepColdArchiveMultipartPartStorage")]
    pub deep_cold_archive_multipart_part_storage: Option<i64>,
}

impl crate::FlatSerialize for BucketStat {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.storage, &format!("{}.Storage", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.object_count,
            &format!("{}.ObjectCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.multipart_upload_count,
            &format!("{}.MultipartUploadCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.live_channel_count,
            &format!("{}.LiveChannelCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.multipart_part_count,
            &format!("{}.MultipartPartCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.delete_marker_count,
            &format!("{}.DeleteMarkerCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.LastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.standard_storage,
            &format!("{}.StandardStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.standard_object_count,
            &format!("{}.StandardObjectCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.infrequent_access_storage,
            &format!("{}.InfrequentAccessStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.infrequent_access_real_storage,
            &format!("{}.InfrequentAccessRealStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.infrequent_access_object_count,
            &format!("{}.InfrequentAccessObjectCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.archive_storage,
            &format!("{}.ArchiveStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.archive_real_storage,
            &format!("{}.ArchiveRealStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.archive_object_count,
            &format!("{}.ArchiveObjectCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cold_archive_storage,
            &format!("{}.ColdArchiveStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cold_archive_real_storage,
            &format!("{}.ColdArchiveRealStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cold_archive_object_count,
            &format!("{}.ColdArchiveObjectCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.deep_cold_archive_storage,
            &format!("{}.DeepColdArchiveStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.deep_cold_archive_real_storage,
            &format!("{}.DeepColdArchiveRealStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.deep_cold_archive_object_count,
            &format!("{}.DeepColdArchiveObjectCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.multipart_part_storage,
            &format!("{}.MultipartPartStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.standard_multipart_part_count,
            &format!("{}.StandardMultipartPartCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.standard_multipart_part_storage,
            &format!("{}.StandardMultipartPartStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.infrequent_multipart_part_count,
            &format!("{}.InfrequentMultipartPartCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.infrequent_multipart_part_storage,
            &format!("{}.InfrequentMultipartPartStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.archive_multipart_part_count,
            &format!("{}.ArchiveMultipartPartCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.archive_multipart_part_storage,
            &format!("{}.ArchiveMultipartPartStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cold_archive_multipart_part_count,
            &format!("{}.ColdArchiveMultipartPartCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cold_archive_multipart_part_storage,
            &format!("{}.ColdArchiveMultipartPartStorage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.deep_cold_archive_multipart_part_count,
            &format!("{}.DeepColdArchiveMultipartPartCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.deep_cold_archive_multipart_part_storage,
            &format!("{}.DeepColdArchiveMultipartPartStorage", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for BucketStat {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// CORS规则的容器。
///
/// 每个Bucket最多允许10条CORS规则。上传的XML文档最大允许16 KB。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CORSRule {
    /// 指定允许的跨域请求来源。
    #[serde(rename = "AllowedOrigin")]
    #[serde(default)]
    pub allowed_origin: Vec<String>,
    /// 指定允许的跨域请求方法。
    #[serde(rename = "AllowedMethod")]
    #[serde(default)]
    pub allowed_method: Vec<String>,
    /// 控制OPTIONS预取指令Access-Control-Request-Headers中指定的Header是否被允许。在Access-Control-Request-Headers中指定的每个Header都必须在AllowedHeader中有对应的项。
    ///
    /// > 仅允许使用一个星号（*）通配符。
    #[serde(rename = "AllowedHeader")]
    #[serde(default)]
    pub allowed_header: Vec<String>,
    /// 指定允许用户从应用程序中访问的响应头。例如一个JavaScript的XMLHttpRequest对象。
    ///
    /// > 不允许使用星号（*）通配符。
    #[serde(rename = "ExposeHeader")]
    #[serde(default)]
    pub expose_header: Vec<String>,
    /// 指定浏览器对特定资源的预取（OPTIONS）请求返回结果的缓存时间。单位为秒。
    ///
    /// 单条CORS规则仅允许一个MaxAgeSeconds。
    #[serde(rename = "MaxAgeSeconds")]
    pub max_age_seconds: Option<i64>,
}

impl crate::FlatSerialize for CORSRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.allowed_origin,
            &format!("{}.AllowedOrigin", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.allowed_method,
            &format!("{}.AllowedMethod", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.allowed_header,
            &format!("{}.AllowedHeader", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.expose_header,
            &format!("{}.ExposeHeader", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_age_seconds,
            &format!("{}.MaxAgeSeconds", name),
            params,
        );
    }
}

/// Bucket的CORS规则容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CORSConfiguration {
    /// CORS规则的容器。
    ///
    /// 每个Bucket最多允许10条CORS规则。上传的XML文档最大允许16 KB。
    #[serde(rename = "CORSRule")]
    #[serde(default)]
    pub cors_rule: Vec<CORSRule>,
    /// 是否返回Vary: Origin头。取值范围如下：
    ///
    /// - true：不管发送的是否是跨域请求或跨域请求是否成功，均会返回Vary: Origin头。
    /// - false（默认值）：任何情况下均不返回Vary: Origin头。
    ///
    /// > 此字段不能单独配置，必须至少配置一项跨域规则才能生效。
    #[serde(rename = "ResponseVary")]
    pub response_vary: Option<bool>,
}

impl crate::FlatSerialize for CORSConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
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

/// 保存Select请求的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CSVInput {
    /// 指定CSV文件头信息。
    ///
    /// - Use：该CSV文件有头信息，可以用CSV列名作为Select中的列名。
    ///
    /// - Ignore：该CSV文件有头信息，但不可用CSV列名作为Select中的列名。
    ///
    /// - None：该文件没有头信息，为默认值。
    #[serde(rename = "FileHeaderInfo")]
    pub file_header_info: Option<FileHeaderInfo>,
    /// 指定换行符，以Base64编码。默认值为\n（可选）。未编码前的值最多为两个字符，以字符的ANSI值表示，例如在Java中使用\n表示换行。
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: Option<String>,
    /// 指定CSV列分隔符，以Base64编码。默认值为`,`（可选）。未编码前的值必须为一个字符，以字符的ANSI值表示，例如在Java中使用`,`表示逗号。
    #[serde(rename = "FieldDelimiter")]
    pub field_delimiter: Option<String>,
    /// 指定CSV的引号字符，以Base64编码。默认值为`\”`（可选）。在CSV中引号内的换行符，列分隔符将被视作普通字符。未编码前的值必须为一个字符，以字符的ANSI值表示，例如在Java中使用`\”`表示引号。
    #[serde(rename = "QuoteCharacter")]
    pub quote_character: Option<String>,
    /// 指定CSV的注释符，以Base64编码。默认值为空（即没有注释符）。
    #[serde(rename = "CommentCharacter")]
    pub comment_character: Option<String>,
    /// 指定查询文件的范围（可选）。支持两种格式：
    ///
    /// > 使用Range参数查询的文件需要有select meta。关于select meta的更多信息，请参见[CreateSelectObjectMeta](~~74054~~)。
    ///
    /// - 按行查询：line-range=start-end。例如line-range=10-20表示扫描第10行到第20行。
    ///
    /// - 按Split查询：split-range=start-end。例如split-range=10-20表示扫描第10到第20个split。
    ///
    /// <br>其中start和end均为inclusive。其格式和range get中的range参数一致。
    /// <br>仅在文档是CSV或者JSON Type为LINES时使用。
    #[serde(rename = "Range")]
    pub range: Option<String>,
    /// 指定CSV内容是否含有在引号中的换行符。
    /// <br>例如某一列值为`"abc\ndef" `（此处`\n`为换行）， 则该值需设置为true。当该值为false时，select支持header range的语义，可以更高效的进行分片查询。
    #[serde(rename = "AllowQuotedRecordDelimiter")]
    pub allow_quoted_record_delimiter: Option<bool>,
}

impl crate::FlatSerialize for CSVInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.file_header_info,
            &format!("{}.FileHeaderInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.record_delimiter,
            &format!("{}.RecordDelimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.field_delimiter,
            &format!("{}.FieldDelimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.quote_character,
            &format!("{}.QuoteCharacter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.comment_character,
            &format!("{}.CommentCharacter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.range, &format!("{}.Range", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.allow_quoted_record_delimiter,
            &format!("{}.AllowQuotedRecordDelimiter", name),
            params,
        );
    }
}

/// 保存Select请求的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CSVOutput {
    /// 指定换行符，以Base64编码。未编码前的值最多为两个字符，以字符的ANSI值表示，例如在Java中使用`\n`表示换行。
    /// <br>默认值：`\n`
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: Option<String>,
    /// 指定CSV列分隔符，以Base64编码。未编码前的值必须为一个字符，以字符的ANSI值表示，例如在Java中使用`,`表示逗号。
    /// <br>默认值：`,`
    #[serde(rename = "FieldDelimiter")]
    pub field_delimiter: Option<String>,
}

impl crate::FlatSerialize for CSVOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.record_delimiter,
            &format!("{}.RecordDelimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.field_delimiter,
            &format!("{}.FieldDelimiter", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CacheQuotaConfigurationQuotaDesc {
    #[serde(rename = "Quota")]
    pub quota: Option<i64>,
}

impl crate::FlatSerialize for CacheQuotaConfigurationQuotaDesc {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.quota, &format!("{}.Quota", name), params);
    }
}

/// 加速器容量配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CacheQuotaConfiguration {
    #[serde(rename = "QuotaDesc")]
    pub quota_desc: Option<CacheQuotaConfigurationQuotaDesc>,
}

impl crate::FlatSerialize for CacheQuotaConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.quota_desc,
            &format!("{}.QuotaDesc", name),
            params,
        );
    }
}

/// 加速器基本信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CacheBaseInfo {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "AvailableZone")]
    pub available_zone: Option<String>,
    #[serde(rename = "QuotaConfiguration")]
    pub quota_configuration: Option<CacheQuotaConfiguration>,
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,
}

impl crate::FlatSerialize for CacheBaseInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.available_zone,
            &format!("{}.AvailableZone", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.quota_configuration,
            &format!("{}.QuotaConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
    }
}

/// 加速器关联存储空间信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CacheBucketInfo {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "CachePolicy")]
    pub cache_policy: Option<String>,
    #[serde(rename = "AcceleratePaths")]
    pub accelerate_paths: Option<AcceleratePaths>,
}

impl crate::FlatSerialize for CacheBucketInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.cache_policy,
            &format!("{}.CachePolicy", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.accelerate_paths,
            &format!("{}.AcceleratePaths", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CacheConfigurationCachesCache {
    #[serde(rename = "AvailableZone")]
    pub available_zone: Option<String>,
    #[serde(rename = "CacheName")]
    pub cache_name: Option<String>,
    #[serde(rename = "CachePolicy")]
    pub cache_policy: Option<String>,
    #[serde(rename = "AcceleratePaths")]
    pub accelerate_paths: Option<AcceleratePaths>,
}

impl crate::FlatSerialize for CacheConfigurationCachesCache {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.available_zone,
            &format!("{}.AvailableZone", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cache_name,
            &format!("{}.CacheName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cache_policy,
            &format!("{}.CachePolicy", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.accelerate_paths,
            &format!("{}.AcceleratePaths", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CacheConfigurationCaches {
    #[serde(rename = "Cache")]
    pub cache: Option<CacheConfigurationCachesCache>,
}

impl crate::FlatSerialize for CacheConfigurationCaches {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.cache, &format!("{}.Cache", name), params);
    }
}

/// 存储空间加速器配置信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CacheConfiguration {
    #[serde(rename = "Caches")]
    pub caches: Option<CacheConfigurationCaches>,
}

impl crate::FlatSerialize for CacheConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.caches, &format!("{}.Caches", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CacheDetailInfoBuckets {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: Vec<CacheBucketInfo>,
}

impl crate::FlatSerialize for CacheDetailInfoBuckets {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
    }
}

/// 加速器详细信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CacheDetailInfo {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "AvailableZone")]
    pub available_zone: Option<String>,
    #[serde(rename = "QuotaConfiguration")]
    pub quota_configuration: Option<CacheQuotaConfiguration>,
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,
    #[serde(rename = "Buckets")]
    pub buckets: Option<CacheDetailInfoBuckets>,
}

impl crate::FlatSerialize for CacheDetailInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.available_zone,
            &format!("{}.AvailableZone", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.quota_configuration,
            &format!("{}.QuotaConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.buckets, &format!("{}.Buckets", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CallbackPolicyPolicyItemItem {
    /// 回调参数配置条目名称
    #[serde(rename = "PolicyName")]
    pub policy_name: Option<String>,
    /// 回调地址
    #[serde(rename = "Callback")]
    pub callback: Option<String>,
    /// 回调参数
    #[serde(rename = "CallbackVar")]
    pub callback_var: Option<String>,
}

impl crate::FlatSerialize for CallbackPolicyPolicyItemItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.policy_name,
            &format!("{}.PolicyName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.callback, &format!("{}.Callback", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.callback_var,
            &format!("{}.CallbackVar", name),
            params,
        );
    }
}

/// 回调规则配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CallbackPolicy {
    /// 回调参数配置条目
    #[serde(rename = "PolicyItem")]
    #[serde(default)]
    pub policy_item: Vec<CallbackPolicyPolicyItemItem>,
}

impl crate::FlatSerialize for CallbackPolicy {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.policy_item,
            &format!("{}.PolicyItem", name),
            params,
        );
    }
}

/// xxx
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Channel {
    /// 图片处理频道的状态
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 是否禁止原图访问
    #[serde(rename = "OrigPicForbidden")]
    pub orig_pic_forbidden: Option<bool>,
    /// 是否仅允许使用样式进行图片处理
    #[serde(rename = "UseStyleOnly")]
    pub use_style_only: Option<bool>,
    /// 是否自动设置Content-Type
    #[serde(rename = "AutoSetContentType")]
    pub auto_set_content_type: Option<bool>,
    /// 是否使用源格式
    #[serde(rename = "UseSrcFormat")]
    pub use_src_format: Option<bool>,
    /// 是否设置Content-Disposition: attachment响应头
    #[serde(rename = "SetAttachName")]
    pub set_attach_name: Option<bool>,
    /// 默认404图片
    #[serde(rename = "Default404Pic")]
    pub default404_pic: Option<String>,
    /// 样式分隔符
    #[serde(rename = "StyleDelimiters")]
    pub style_delimiters: Option<String>,
}

impl crate::FlatSerialize for Channel {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.orig_pic_forbidden,
            &format!("{}.OrigPicForbidden", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.use_style_only,
            &format!("{}.UseStyleOnly", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.auto_set_content_type,
            &format!("{}.AutoSetContentType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.use_src_format,
            &format!("{}.UseSrcFormat", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.set_attach_name,
            &format!("{}.SetAttachName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.default404_pic,
            &format!("{}.Default404Pic", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.style_delimiters,
            &format!("{}.StyleDelimiters", name),
            params,
        );
    }
}

/// 图片处理频道配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelInfo {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "OrigPicForbidden")]
    pub orig_pic_forbidden: Option<bool>,
    #[serde(rename = "UseSrcFormat")]
    pub use_src_format: Option<bool>,
    #[serde(rename = "SetAttachName")]
    pub set_attach_name: Option<bool>,
    #[serde(rename = "UseStyleOnly")]
    pub use_style_only: Option<bool>,
    #[serde(rename = "AutoSetContentType")]
    pub auto_set_content_type: Option<bool>,
    #[serde(rename = "StyleDelimiters")]
    pub style_delimiters: Option<String>,
}

impl crate::FlatSerialize for ChannelInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.orig_pic_forbidden,
            &format!("{}.OrigPicForbidden", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.use_src_format,
            &format!("{}.UseSrcFormat", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.set_attach_name,
            &format!("{}.SetAttachName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.use_style_only,
            &format!("{}.UseStyleOnly", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.auto_set_content_type,
            &format!("{}.AutoSetContentType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.style_delimiters,
            &format!("{}.StyleDelimiters", name),
            params,
        );
    }
}

/// 证书信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameCertificate {
    /// 证书来源
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
    /// 证书ID
    #[serde(rename = "CertId")]
    pub cert_id: Option<String>,
    /// 证书状态
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 证书绑定时间
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,
    /// 证书签名
    #[serde(rename = "Fingerprint")]
    pub fingerprint: Option<String>,
    /// 证书有效期起始时间
    #[serde(rename = "ValidStartDate")]
    pub valid_start_date: Option<String>,
    /// 证书有效期终止时间
    #[serde(rename = "ValidEndDate")]
    pub valid_end_date: Option<String>,
}

impl crate::FlatSerialize for CnameCertificate {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
        crate::FlatSerialize::flat_serialize(&self.cert_id, &format!("{}.CertId", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.fingerprint,
            &format!("{}.Fingerprint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.valid_start_date,
            &format!("{}.ValidStartDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.valid_end_date,
            &format!("{}.ValidEndDate", name),
            params,
        );
    }
}

/// Cname信息概况。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameInfo {
    /// 自定义域名。
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// 绑定自定义域名的时间。
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
    /// 域名所处状态。取值为：
    ///
    /// - Enabled：：启用该域名。
    ///
    /// - Disabled：禁用该域名。
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 证书信息的容器。
    #[serde(rename = "Certificate")]
    pub certificate: Option<CnameCertificate>,
}

impl crate::FlatSerialize for CnameInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.domain, &format!("{}.Domain", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.certificate,
            &format!("{}.Certificate", name),
            params,
        );
    }
}

/// Cname信息概况
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameSummary {
    /// 自定义域名
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// 绑定自定义域名的时间
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
    /// 域名所处的状态
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 证书信息的容器
    #[serde(rename = "Certificate")]
    pub certificate: Option<CnameCertificate>,
}

impl crate::FlatSerialize for CnameSummary {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.domain, &format!("{}.Domain", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.certificate,
            &format!("{}.Certificate", name),
            params,
        );
    }
}

/// CnameToken的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameToken {
    /// 绑定Cname的Bucket名称。
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    /// 绑定的Cname名称。
    #[serde(rename = "Cname")]
    pub cname: Option<String>,
    /// OSS返回的CnameToken。
    #[serde(rename = "Token")]
    pub token: Option<String>,
    /// CnameToken的过期时间。
    #[serde(rename = "ExpireTime")]
    pub expire_time: Option<String>,
}

impl crate::FlatSerialize for CnameToken {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.cname, &format!("{}.Cname", name), params);
        crate::FlatSerialize::flat_serialize(&self.token, &format!("{}.Token", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.expire_time,
            &format!("{}.ExpireTime", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for CnameToken {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 存放bucket备注内容的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommentConfiguration {
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
}

impl crate::FlatSerialize for CommentConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.comment, &format!("{}.Comment", name), params);
    }
}

/// 存放用户自定义HTTP Header的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommonHeadersHeaderItem {
    /// HTTP Header的Key
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// HTTP Header的Value
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl crate::FlatSerialize for CommonHeadersHeaderItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
    }
}

/// 存放用户自定义HTTP Header配置的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommonHeaders {
    /// 存放用户自定义HTTP Header列表的容器
    #[serde(rename = "Header")]
    #[serde(default)]
    pub header: Vec<CommonHeadersHeaderItem>,
}

impl crate::FlatSerialize for CommonHeaders {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.header, &format!("{}.Header", name), params);
    }
}

/// 如果请求中指定了delimiter参数，则OSS返回的响应中包含CommonPrefixes元素。该元素标明以delimiter结尾，并有共同前缀的Object名称的集合。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommonPrefix {
    /// 本次查询结果的前缀。
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
}

impl crate::FlatSerialize for CommonPrefix {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
    }
}

/// 保存已上传Part信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompleteMultipartUploadPartItem {
    /// Part数目。
    #[serde(rename = "PartNumber")]
    pub part_number: Option<i64>,
    /// Object生成时会创建相应的ETag ，ETag用于标识一个Object的内容。
    ///
    /// 通过CompleteMultipartUpload请求创建的Object，ETag值是基于一定计算规则生成的唯一值，但不是其内容的MD5值。
    ///
    /// > ETag值可以用于检查Object内容是否发生变化。不建议使用ETag作为Object内容的MD5来校验数据完整性。
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
}

impl crate::FlatSerialize for CompleteMultipartUploadPartItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.part_number,
            &format!("{}.PartNumber", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
    }
}

/// 保存CompleteMultipartUpload请求内容的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompleteMultipartUploadSchema {
    /// 保存已上传Part信息的容器。
    #[serde(rename = "Part")]
    #[serde(default)]
    pub part: Vec<CompleteMultipartUploadPartItem>,
}

impl crate::FlatSerialize for CompleteMultipartUploadSchema {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.part, &format!("{}.Part", name), params);
    }
}

/// CopyObject的结果。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CopyObjectResult {
    /// Object生成时会创建相应的ETag ，ETag用于标识一个Object的内容。
    ///   - 对于PutObject请求创建的Object，ETag值是其内容的MD5值。
    ///   - 对于其他方式创建的Object，ETag值是基于一定计算规则生成的唯一值，但不是其内容的MD5值。
    ///
    /// >ETag值可以用于检查Object内容是否发生变化。不建议使用ETag作为Object内容的MD5来校验数据完整性。
    /// 默认值：无
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    /// 最近一次修改的时间。
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
}

impl crate::FlatSerialize for CopyObjectResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
    }
}

/// CopyObject的结果。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CopyPartResult {
    /// Object生成时会创建相应的ETag ，ETag用于标识一个Object的内容。
    /// 通过CompleteMultipartUpload请求创建的Object，ETag值是基于一定计算规则生成的唯一值，但不是其内容的MD5值。
    ///
    /// > ETag值可以用于检查Object内容是否发生变化。不建议使用ETag作为Object内容的MD5来校验数据完整性。
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    /// Part上传的时间。
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
}

impl crate::FlatSerialize for CopyPartResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
    }
}

/// 保存接入点信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointConfiguration {
    /// 接入点名称。接入点命名规范如下：
    ///
    /// - 接入点名称在当前阿里云账号单个地域内唯一。
    ///
    /// - 不允许以-ossalias结尾。
    ///
    /// - 只能包括小写字母、数字和短划线（-），不能以短划线（-）开头或结尾。
    ///
    /// - 命名长度为3~19个字符。
    #[serde(rename = "AccessPointName")]
    pub access_point_name: Option<String>,
    /// 接入点网络来源。
    #[serde(rename = "NetworkOrigin")]
    pub network_origin: Option<String>,
    /// 保存VPC网络来源信息的容器。
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<AccessPointVpcConfiguration>,
}

impl crate::FlatSerialize for CreateAccessPointConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point_name,
            &format!("{}.AccessPointName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.network_origin,
            &format!("{}.NetworkOrigin", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.vpc_configuration,
            &format!("{}.VpcConfiguration", name),
            params,
        );
    }
}

/// 保存接入点信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointResult {
    /// 接入点ARN。
    #[serde(rename = "AccessPointArn")]
    pub access_point_arn: Option<String>,
    /// 接入点别名。
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
}

impl crate::FlatSerialize for CreateAccessPointResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point_arn,
            &format!("{}.AccessPointArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.alias, &format!("{}.Alias", name), params);
    }
}

impl crate::ToCodeMessage for CreateAccessPointResult {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// Bucket存储类型和数据容灾类型的配置信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateBucketConfiguration {
    /// Bucket的存储类型。 取值范围如下：
    ///
    /// - Standard（默认）：标准存储
    /// - IA：低频访问
    /// - Archive：归档存储
    /// - ColdArchive：冷归档存储
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    /// 指定Bucket的数据容灾类型。
    ///
    /// - LRS（默认值）
    /// <br>本地冗余LRS，将您的数据冗余存储在同一个可用区的不同存储设备上，可支持两个存储设备并发损坏时，仍维持数据不丢失，可正常访问。
    ///
    /// - ZRS
    /// <br>同城冗余ZRS采用多可用区（AZ）机制，将您的数据冗余存储在同一地域（Region）的3个可用区。可支持单个可用区（机房）整体故障时（例如断电、火灾等），仍然能够保障数据的正常访问。
    /// > 归档类型的Bucket不支持设置同城冗余。</props>
    #[serde(rename = "DataRedundancyType")]
    pub data_redundancy_type: Option<DataRedundancyType>,
}

impl crate::FlatSerialize for CreateBucketConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.data_redundancy_type,
            &format!("{}.DataRedundancyType", name),
            params,
        );
    }
}

/// 加速器创建参数
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateCacheConfiguration {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "AvailableZone")]
    pub available_zone: Option<String>,
    #[serde(rename = "QuotaConfiguration")]
    pub quota_configuration: Option<CacheQuotaConfiguration>,
}

impl crate::FlatSerialize for CreateCacheConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.available_zone,
            &format!("{}.AvailableZone", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.quota_configuration,
            &format!("{}.QuotaConfiguration", name),
            params,
        );
    }
}

/// oss cache异步预热规则
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateDataLakeCachePrefetchJob {
    #[serde(rename = "Includes")]
    #[serde(default)]
    pub includes: Vec<String>,
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
    #[serde(rename = "Excludes")]
    #[serde(default)]
    pub excludes: Vec<String>,
}

impl crate::FlatSerialize for CreateDataLakeCachePrefetchJob {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.includes, &format!("{}.Includes", name), params);
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
        crate::FlatSerialize::flat_serialize(&self.excludes, &format!("{}.Excludes", name), params);
    }
}

/// 数据湖元数据转换任务配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateDataLakeStorageTransferJob {
    #[serde(rename = "Includes")]
    #[serde(default)]
    pub includes: Vec<String>,
    #[serde(rename = "ExecutorRoleId")]
    pub executor_role_id: Option<String>,
    #[serde(rename = "LogBaseDir")]
    pub log_base_dir: Option<String>,
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
    #[serde(rename = "NeedVerify")]
    pub need_verify: Option<bool>,
}

impl crate::FlatSerialize for CreateDataLakeStorageTransferJob {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.includes, &format!("{}.Includes", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.executor_role_id,
            &format!("{}.ExecutorRoleId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.log_base_dir,
            &format!("{}.LogBaseDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.need_verify,
            &format!("{}.NeedVerify", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFileGroupPartItem {
    #[serde(rename = "PartNumber")]
    pub part_number: Option<i64>,
    #[serde(rename = "PartName")]
    pub part_name: Option<String>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
}

impl crate::FlatSerialize for CreateFileGroupPartItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.part_number,
            &format!("{}.PartNumber", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.part_name,
            &format!("{}.PartName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
    }
}

/// 创建FileGroup类型文件的请求
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFileGroup {
    #[serde(rename = "Part")]
    #[serde(default)]
    pub part: Vec<CreateFileGroupPartItem>,
}

impl crate::FlatSerialize for CreateFileGroup {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.part, &format!("{}.Part", name), params);
    }
}

/// 创建FileGroup类型文件的结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFileGroupResult {
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
}

impl crate::FlatSerialize for CreateFileGroupResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(&self.size, &format!("{}.Size", name), params);
    }
}

/// 创建预留空间的结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateLargeReservedCapacityResult {
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
}

impl crate::FlatSerialize for CreateLargeReservedCapacityResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
    }
}

/// ObjectLink类型文件创建结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateObjectLinkResult {
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
}

impl crate::FlatSerialize for CreateObjectLinkResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataAcceleratorBasicInfomation {
    #[serde(rename = "AcceleratePaths")]
    pub accelerate_paths: Option<AcceleratePaths>,
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<i64>,
    #[serde(rename = "Quota")]
    pub quota: Option<String>,
    #[serde(rename = "AvailableZone")]
    pub available_zone: Option<String>,
    #[serde(rename = "QuotaFrozenUtil")]
    pub quota_frozen_util: Option<i64>,
}

impl crate::FlatSerialize for DataAcceleratorBasicInfomation {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.accelerate_paths,
            &format!("{}.AcceleratePaths", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.quota, &format!("{}.Quota", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.available_zone,
            &format!("{}.AvailableZone", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.quota_frozen_util,
            &format!("{}.QuotaFrozenUtil", name),
            params,
        );
    }
}

/// 数据湖加速器信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataAccelerator {
    #[serde(rename = "BasicInfomation")]
    pub basic_infomation: Option<DataAcceleratorBasicInfomation>,
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl crate::FlatSerialize for DataAccelerator {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.basic_infomation,
            &format!("{}.BasicInfomation", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bucket_name,
            &format!("{}.BucketName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataAcceleratorConfiguration {
    #[serde(rename = "AvailableZone")]
    pub available_zone: Option<String>,
    #[serde(rename = "Quota")]
    pub quota: Option<String>,
    #[serde(rename = "AcceleratePaths")]
    pub accelerate_paths: Option<AcceleratePaths>,
}

impl crate::FlatSerialize for DataAcceleratorConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.available_zone,
            &format!("{}.AvailableZone", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.quota, &format!("{}.Quota", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.accelerate_paths,
            &format!("{}.AcceleratePaths", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeCachePrefetchJobRulePrefixFilterIncludes {
    #[serde(rename = "Include")]
    #[serde(default)]
    pub include: Vec<String>,
}

impl crate::FlatSerialize for DataLakeCachePrefetchJobRulePrefixFilterIncludes {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.include, &format!("{}.Include", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeCachePrefetchJobRulePrefixFilterExcludes {
    #[serde(rename = "Exclude")]
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl crate::FlatSerialize for DataLakeCachePrefetchJobRulePrefixFilterExcludes {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.exclude, &format!("{}.Exclude", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeCachePrefetchJobRulePrefixFilter {
    #[serde(rename = "Includes")]
    pub includes: Option<DataLakeCachePrefetchJobRulePrefixFilterIncludes>,
    #[serde(rename = "Excludes")]
    pub excludes: Option<DataLakeCachePrefetchJobRulePrefixFilterExcludes>,
}

impl crate::FlatSerialize for DataLakeCachePrefetchJobRulePrefixFilter {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.includes, &format!("{}.Includes", name), params);
        crate::FlatSerialize::flat_serialize(&self.excludes, &format!("{}.Excludes", name), params);
    }
}

/// oss加速器异步预热规则
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeCachePrefetchJobRule {
    #[serde(rename = "PrefixFilter")]
    pub prefix_filter: Option<DataLakeCachePrefetchJobRulePrefixFilter>,
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
}

impl crate::FlatSerialize for DataLakeCachePrefetchJobRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.prefix_filter,
            &format!("{}.PrefixFilter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
    }
}

/// oss加速器异步预热配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeCachePrefetchJob {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "HistoryId")]
    pub history_id: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    #[serde(rename = "Type")]
    pub r#type: Option<i32>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "CreateTime")]
    pub create_time: Option<i64>,
    #[serde(rename = "LastModifyTime")]
    pub last_modify_time: Option<i64>,
    #[serde(rename = "Rule")]
    pub rule: Option<DataLakeCachePrefetchJobRule>,
}

impl crate::FlatSerialize for DataLakeCachePrefetchJob {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.Id", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.history_id,
            &format!("{}.HistoryId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modify_time,
            &format!("{}.LastModifyTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

/// oss加速器异步预热任务执行记录
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeCachePrefetchJobHistory {
    #[serde(rename = "JobId")]
    pub job_id: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "StartTime")]
    pub start_time: Option<i64>,
    #[serde(rename = "EndTime")]
    pub end_time: Option<i64>,
    #[serde(rename = "TotalCount")]
    pub total_count: Option<i64>,
    #[serde(rename = "SucceedCount")]
    pub succeed_count: Option<i64>,
}

impl crate::FlatSerialize for DataLakeCachePrefetchJobHistory {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.job_id, &format!("{}.JobId", name), params);
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.Id", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.StartTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.end_time, &format!("{}.EndTime", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.total_count,
            &format!("{}.TotalCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.succeed_count,
            &format!("{}.SucceedCount", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobRulePrefixFilterIncludes {
    #[serde(rename = "Include")]
    #[serde(default)]
    pub include: Vec<String>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJobRulePrefixFilterIncludes {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.include, &format!("{}.Include", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobRulePrefixFilter {
    #[serde(rename = "Includes")]
    pub includes: Option<DataLakeStorageTransferJobRulePrefixFilterIncludes>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJobRulePrefixFilter {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.includes, &format!("{}.Includes", name), params);
    }
}

/// 数据湖元数据转换任务规则
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobRule {
    #[serde(rename = "PrefixFilter")]
    pub prefix_filter: Option<DataLakeStorageTransferJobRulePrefixFilter>,
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
    #[serde(rename = "ExecutorRoleId")]
    pub executor_role_id: Option<String>,
    #[serde(rename = "LogBaseDir")]
    pub log_base_dir: Option<String>,
    #[serde(rename = "NeedVerify")]
    pub need_verify: Option<bool>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJobRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.prefix_filter,
            &format!("{}.PrefixFilter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.executor_role_id,
            &format!("{}.ExecutorRoleId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.log_base_dir,
            &format!("{}.LogBaseDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.need_verify,
            &format!("{}.NeedVerify", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobProgressInfo {
    #[serde(rename = "Percent")]
    pub percent: Option<i64>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJobProgressInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.percent, &format!("{}.Percent", name), params);
    }
}

/// 数据湖元数据转换任务配置信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJob {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "HistoryId")]
    pub history_id: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    #[serde(rename = "Type")]
    pub r#type: Option<i32>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "CreateTime")]
    pub create_time: Option<i64>,
    #[serde(rename = "LastModifyTime")]
    pub last_modify_time: Option<i64>,
    #[serde(rename = "Rule")]
    pub rule: Option<DataLakeStorageTransferJobRule>,
    #[serde(rename = "ProgressInfo")]
    pub progress_info: Option<DataLakeStorageTransferJobProgressInfo>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJob {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.Id", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.history_id,
            &format!("{}.HistoryId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modify_time,
            &format!("{}.LastModifyTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.progress_info,
            &format!("{}.ProgressInfo", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobHistoryDetailInfo {
    #[serde(rename = "HDFSTransferJobId")]
    pub hdfs_transfer_job_id: Option<String>,
    #[serde(rename = "HDFSTransferDataDir")]
    pub hdfs_transfer_data_dir: Option<String>,
    #[serde(rename = "HDFSTransferImportMetaDir")]
    pub hdfs_transfer_import_meta_dir: Option<String>,
    #[serde(rename = "HDFSTransferErrInfoDir")]
    pub hdfs_transfer_err_info_dir: Option<String>,
    #[serde(rename = "LogBaseDir")]
    pub log_base_dir: Option<String>,
    #[serde(rename = "HDFSFailedCount")]
    pub hdfs_failed_count: Option<i64>,
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<String>,
    #[serde(rename = "VerifyTotalCount")]
    pub verify_total_count: Option<i64>,
    #[serde(rename = "VerifyStatus")]
    pub verify_status: Option<String>,
    #[serde(rename = "VerifyErrInfoDir")]
    pub verify_err_info_dir: Option<String>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJobHistoryDetailInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.hdfs_transfer_job_id,
            &format!("{}.HDFSTransferJobId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.hdfs_transfer_data_dir,
            &format!("{}.HDFSTransferDataDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.hdfs_transfer_import_meta_dir,
            &format!("{}.HDFSTransferImportMetaDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.hdfs_transfer_err_info_dir,
            &format!("{}.HDFSTransferErrInfoDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.log_base_dir,
            &format!("{}.LogBaseDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.hdfs_failed_count,
            &format!("{}.HDFSFailedCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.error_msg,
            &format!("{}.ErrorMsg", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.verify_total_count,
            &format!("{}.VerifyTotalCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.verify_status,
            &format!("{}.VerifyStatus", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.verify_err_info_dir,
            &format!("{}.VerifyErrInfoDir", name),
            params,
        );
    }
}

/// 数据湖元数据转换历史任务
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobHistory {
    #[serde(rename = "JobId")]
    pub job_id: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "StartTime")]
    pub start_time: Option<i64>,
    #[serde(rename = "EndTime")]
    pub end_time: Option<i64>,
    #[serde(rename = "TotalCount")]
    pub total_count: Option<i64>,
    #[serde(rename = "SucceedCount")]
    pub succeed_count: Option<i64>,
    #[serde(rename = "DetailInfo")]
    pub detail_info: Option<DataLakeStorageTransferJobHistoryDetailInfo>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJobHistory {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.job_id, &format!("{}.JobId", name), params);
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.Id", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.StartTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.end_time, &format!("{}.EndTime", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.total_count,
            &format!("{}.TotalCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.succeed_count,
            &format!("{}.SucceedCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.detail_info,
            &format!("{}.DetailInfo", name),
            params,
        );
    }
}

/// 数据湖元数据转换历史任务ID
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobHistoryId {
    #[serde(rename = "HistoryId")]
    pub history_id: Option<String>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJobHistoryId {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.history_id,
            &format!("{}.HistoryId", name),
            params,
        );
    }
}

/// 数据湖元数据转换任务ID
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobId {
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJobId {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.Id", name), params);
    }
}

/// 数据湖元数据转换任务列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobs {
    #[serde(rename = "Truncated")]
    pub truncated: Option<String>,
    #[serde(rename = "NextMarkerBucket")]
    pub next_marker_bucket: Option<String>,
    #[serde(rename = "NextMarkerJobId")]
    pub next_marker_job_id: Option<String>,
    #[serde(rename = "DataLakeStorageTransferJob")]
    #[serde(default)]
    pub data_lake_storage_transfer_job: Vec<DataLakeStorageTransferJob>,
}

impl crate::FlatSerialize for DataLakeStorageTransferJobs {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.truncated,
            &format!("{}.Truncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_marker_bucket,
            &format!("{}.NextMarkerBucket", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_marker_job_id,
            &format!("{}.NextMarkerJobId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.data_lake_storage_transfer_job,
            &format!("{}.DataLakeStorageTransferJob", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for DataLakeStorageTransferJobs {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 对象标识符。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectIdentifier {
    /// 对象 Key。
    #[serde(rename = "Key")]
    pub key: String,
    /// 版本ID。
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
}

impl crate::FlatSerialize for ObjectIdentifier {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.version_id,
            &format!("{}.VersionId", name),
            params,
        );
    }
}

/// 保存DeleteMultipleObjects请求的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Delete {
    /// 打开简单响应模式的开关。
    /// DeleteMultipleObjects提供以下两种消息返回模式：
    ///   - 简单模式（quiet）：OSS不返回消息体。
    ///   - 详细模式（verbose）：OSS返回的消息体中会包含所有删除Object的结果。默认采用详细模式。
    ///
    /// 有效值：**true**（开启简单模式）、**false**（开启详细模式）
    /// 默认值：**false**
    #[serde(rename = "Quiet")]
    pub quiet: Option<bool>,
    /// 保存一个Object信息的容器。
    #[serde(rename = "Object")]
    #[serde(default)]
    pub object: Vec<ObjectIdentifier>,
}

impl crate::FlatSerialize for Delete {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.quiet, &format!("{}.Quiet", name), params);
        crate::FlatSerialize::flat_serialize(&self.object, &format!("{}.Object", name), params);
    }
}

/// 保存删除标记的容器。
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeleteMarkerEntry {
    /// Object的名称。
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// Object的版本ID。
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
    /// Object是否为当前版本。
    /// 取值：
    ///
    /// - true：Object为当前版本。
    ///
    /// - false：Object为非当前版本。
    #[serde(rename = "IsLatest")]
    pub is_latest: Option<bool>,
    /// Object最后被修改的时间。
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
    /// 保存Bucket拥有者信息的容器。
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
}

impl crate::FlatSerialize for DeleteMarkerEntry {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.version_id,
            &format!("{}.VersionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_latest,
            &format!("{}.IsLatest", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
    }
}

/// 保存被成功删除的Object的容器。
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeletedObject {
    /// 被删除Object的名字。
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 表示该版本是否为删除标记。true为是，false为否。
    /// > 只有当创建删除标记和永久删除删除标记时，才会返回该元素，且值为true。
    #[serde(rename = "DeleteMarker")]
    pub delete_marker: Option<bool>,
    /// 表示删除标记对应的版本ID。
    #[serde(rename = "DeleteMarkerVersionId")]
    pub delete_marker_version_id: Option<String>,
    /// Object对应的版本ID。
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
}

impl crate::FlatSerialize for DeletedObject {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.delete_marker,
            &format!("{}.DeleteMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.delete_marker_version_id,
            &format!("{}.DeleteMarkerVersionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.version_id,
            &format!("{}.VersionId", name),
            params,
        );
    }
}

/// 错误响应。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Error {
    /// 错误码。
    #[serde(rename = "Code")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "Message")]
    pub message: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
    /// 访问OSS所用的域名。
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,
}

impl crate::FlatSerialize for Error {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.code, &format!("{}.Code", name), params);
        crate::FlatSerialize::flat_serialize(&self.message, &format!("{}.Message", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.request_id,
            &format!("{}.RequestId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.host_id, &format!("{}.HostId", name), params);
    }
}

/// 404页面的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ErrorDocument {
    /// 错误页面。
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 返回错误页面时的HTTP状态码。
    #[serde(rename = "HttpStatus")]
    pub http_status: Option<i64>,
}

impl crate::FlatSerialize for ErrorDocument {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.http_status,
            &format!("{}.HttpStatus", name),
            params,
        );
    }
}

/// Object过滤配置键
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionComputeConfigurationFilterKey {
    /// Object前缀
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    /// Object后缀
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
}

impl crate::FlatSerialize for FunctionComputeConfigurationFilterKey {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.suffix, &format!("{}.Suffix", name), params);
    }
}

/// Object过滤配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionComputeConfigurationFilter {
    /// Object过滤配置键
    #[serde(rename = "Key")]
    pub key: Option<FunctionComputeConfigurationFilterKey>,
}

impl crate::FlatSerialize for FunctionComputeConfigurationFilter {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
    }
}

/// 函数计算配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionComputeConfigurationFunction {
    /// 请求的函数计算服务资源
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    /// 向函数计算请求服务的角色
    #[serde(rename = "AssumeRole")]
    pub assume_role: Option<String>,
}

impl crate::FlatSerialize for FunctionComputeConfigurationFunction {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.arn, &format!("{}.Arn", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.assume_role,
            &format!("{}.AssumeRole", name),
            params,
        );
    }
}

/// 函数计算服务配置项
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionComputeConfiguration {
    /// 配置项ID
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// 事件列表
    #[serde(rename = "Event")]
    #[serde(default)]
    pub event: Vec<String>,
    /// Object过滤配置
    #[serde(rename = "Filter")]
    pub filter: Option<FunctionComputeConfigurationFilter>,
    /// 函数计算配置
    #[serde(rename = "Function")]
    pub function: Option<FunctionComputeConfigurationFunction>,
}

impl crate::FlatSerialize for FunctionComputeConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
        crate::FlatSerialize::flat_serialize(&self.event, &format!("{}.Event", name), params);
        crate::FlatSerialize::flat_serialize(&self.filter, &format!("{}.Filter", name), params);
        crate::FlatSerialize::flat_serialize(&self.function, &format!("{}.Function", name), params);
    }
}

/// 存储空间事件通知配置信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventNotificationConfiguration {
    /// 存储空间函数计算事件通知服务配置信息
    #[serde(rename = "FunctionComputeConfiguration")]
    #[serde(default)]
    pub function_compute_configuration: Vec<FunctionComputeConfiguration>,
}

impl crate::FlatSerialize for EventNotificationConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.function_compute_configuration,
            &format!("{}.FunctionComputeConfiguration", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for EventNotificationConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 根节点。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ExtendWormConfiguration {
    /// 指定Object的保留天数。
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: Option<i32>,
}

impl crate::FlatSerialize for ExtendWormConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.retention_period_in_days,
            &format!("{}.RetentionPeriodInDays", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FileGroupInfoFilePartPartItem {
    #[serde(rename = "PartNumber")]
    pub part_number: Option<i64>,
    #[serde(rename = "PartName")]
    pub part_name: Option<String>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    #[serde(rename = "PartSize")]
    pub part_size: Option<i64>,
}

impl crate::FlatSerialize for FileGroupInfoFilePartPartItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.part_number,
            &format!("{}.PartNumber", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.part_name,
            &format!("{}.PartName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.part_size,
            &format!("{}.PartSize", name),
            params,
        );
    }
}

/// FileGroup类型文件的信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FileGroupInfoFilePart {
    #[serde(rename = "Part")]
    #[serde(default)]
    pub part: Vec<FileGroupInfoFilePartPartItem>,
}

impl crate::FlatSerialize for FileGroupInfoFilePart {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.part, &format!("{}.Part", name), params);
    }
}

/// FileGroup类型文件的信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FileGroupInfo {
    /// FileGroup类型文件的信息
    #[serde(rename = "FilePart")]
    pub file_part: Option<FileGroupInfoFilePart>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    #[serde(rename = "FileLength")]
    pub file_length: Option<i64>,
}

impl crate::FlatSerialize for FileGroupInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.file_part,
            &format!("{}.FilePart", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.file_length,
            &format!("{}.FileLength", name),
            params,
        );
    }
}

/// 保存接入点网络来源信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointResultEndpoints {
    /// 接入点的外网Endpoint。
    #[serde(rename = "PublicEndpoint")]
    pub public_endpoint: Option<String>,
    /// 接入点的内网Endpoint。
    #[serde(rename = "InternalEndpoint")]
    pub internal_endpoint: Option<String>,
}

impl crate::FlatSerialize for GetAccessPointResultEndpoints {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.public_endpoint,
            &format!("{}.PublicEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.internal_endpoint,
            &format!("{}.InternalEndpoint", name),
            params,
        );
    }
}

/// 阻止公共访问的配置。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PublicAccessBlockConfiguration {
    /// 是否阻止公共访问。取值：
    /// - true：开启阻止公共访问。
    /// - false：关闭阻止公共访问。
    #[serde(rename = "BlockPublicAccess")]
    pub block_public_access: Option<bool>,
}

impl crate::FlatSerialize for PublicAccessBlockConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.block_public_access,
            &format!("{}.BlockPublicAccess", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for PublicAccessBlockConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 保存接入点信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointResult {
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    pub access_point_name: Option<String>,
    /// 配置接入点的Bucket名称。
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    /// 配置接入点的阿里云账号UID。
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    /// 接入点网络来源。返回值如下：  vpc：限制仅支持通过指定的VPC ID访问接入点。  internet：同时持通过外网和内网Endpoint互联网访问接入点。
    #[serde(rename = "NetworkOrigin")]
    pub network_origin: Option<String>,
    /// 保存VPC网络来源信息的容器。
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<AccessPointVpcConfiguration>,
    /// 接入点ARN。
    #[serde(rename = "AccessPointArn")]
    pub access_point_arn: Option<String>,
    /// 接入点别名。
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
    /// 接入点所处状态。
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 保存接入点网络来源信息的容器。
    #[serde(rename = "Endpoints")]
    pub endpoints: Option<GetAccessPointResultEndpoints>,
    /// 保存接入点阻止公共访问的配置
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
    /// 接入点创建时间。
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,
}

impl crate::FlatSerialize for GetAccessPointResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point_name,
            &format!("{}.AccessPointName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.account_id,
            &format!("{}.AccountId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.network_origin,
            &format!("{}.NetworkOrigin", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.vpc_configuration,
            &format!("{}.VpcConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_point_arn,
            &format!("{}.AccessPointArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.alias, &format!("{}.Alias", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
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
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for GetAccessPointResult {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// Bucket图片处理通道配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketProcessConfiguration {
    #[serde(rename = "Version")]
    pub version: Option<i32>,
    #[serde(rename = "CompliedHost")]
    pub complied_host: Option<String>,
    #[serde(rename = "SourceFileProtect")]
    pub source_file_protect: Option<String>,
    #[serde(rename = "SourceFileProtectSuffix")]
    pub source_file_protect_suffix: Option<String>,
    #[serde(rename = "StyleDelimiters")]
    pub style_delimiters: Option<String>,
    #[serde(rename = "BucketChannelConfig")]
    pub bucket_channel_config: Option<BucketChannelConfig>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
}

impl crate::FlatSerialize for GetBucketProcessConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.version, &format!("{}.Version", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.complied_host,
            &format!("{}.CompliedHost", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_file_protect,
            &format!("{}.SourceFileProtect", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_file_protect_suffix,
            &format!("{}.SourceFileProtectSuffix", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.style_delimiters,
            &format!("{}.StyleDelimiters", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bucket_channel_config,
            &format!("{}.BucketChannelConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
    }
}

/// xxx
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetChannelResult {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "OrigPicForbidden")]
    pub orig_pic_forbidden: Option<bool>,
    #[serde(rename = "UseSrcFormat")]
    pub use_src_format: Option<bool>,
    #[serde(rename = "SetAttachName")]
    pub set_attach_name: Option<bool>,
    #[serde(rename = "UseStyleOnly")]
    pub use_style_only: Option<bool>,
    #[serde(rename = "AutoSetContentType")]
    pub auto_set_content_type: Option<bool>,
    #[serde(rename = "Default404Pic")]
    pub default404_pic: Option<String>,
    #[serde(rename = "StyleDelimiters")]
    pub style_delimiters: Option<String>,
    #[serde(rename = "CreateTime")]
    pub create_time: Option<String>,
    #[serde(rename = "LastModifyTime")]
    pub last_modify_time: Option<String>,
}

impl crate::FlatSerialize for GetChannelResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.orig_pic_forbidden,
            &format!("{}.OrigPicForbidden", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.use_src_format,
            &format!("{}.UseSrcFormat", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.set_attach_name,
            &format!("{}.SetAttachName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.use_style_only,
            &format!("{}.UseStyleOnly", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.auto_set_content_type,
            &format!("{}.AutoSetContentType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.default404_pic,
            &format!("{}.Default404Pic", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.style_delimiters,
            &format!("{}.StyleDelimiters", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modify_time,
            &format!("{}.LastModifyTime", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectInfoResult {
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "EncryptFlag")]
    pub encrypt_flag: Option<i64>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    #[serde(rename = "HashCrc64ecma")]
    pub hash_crc64ecma: Option<String>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<String>,
    #[serde(rename = "Content-Type")]
    pub content_type: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<String>,
    #[serde(rename = "UploadId")]
    pub upload_id: Option<String>,
}

impl crate::FlatSerialize for GetObjectInfoResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.encrypt_flag,
            &format!("{}.EncryptFlag", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.hash_crc64ecma,
            &format!("{}.HashCrc64ecma", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.content_type,
            &format!("{}.Content-Type", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.size, &format!("{}.Size", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.upload_id,
            &format!("{}.UploadId", name),
            params,
        );
    }
}

/// 流控配额信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QoSConfiguration {
    /// 总上行带宽，单位Gbps
    #[serde(rename = "TotalUploadBandwidth")]
    pub total_upload_bandwidth: Option<i64>,
    /// 内网上行带宽，单位Gbps
    #[serde(rename = "IntranetUploadBandwidth")]
    pub intranet_upload_bandwidth: Option<i64>,
    /// 公网上行带宽，单位Gbps
    #[serde(rename = "ExtranetUploadBandwidth")]
    pub extranet_upload_bandwidth: Option<i64>,
    /// 总下行带宽，单位Gbps
    #[serde(rename = "TotalDownloadBandwidth")]
    pub total_download_bandwidth: Option<i64>,
    /// 内网下行带宽，单位Gbps
    #[serde(rename = "IntranetDownloadBandwidth")]
    pub intranet_download_bandwidth: Option<i64>,
    /// 公网下行带宽，单位Gbps
    #[serde(rename = "ExtranetDownloadBandwidth")]
    pub extranet_download_bandwidth: Option<i64>,
    /// 总QPS
    #[serde(rename = "TotalQps")]
    pub total_qps: Option<i64>,
    /// 内网QPS
    #[serde(rename = "IntranetQps")]
    pub intranet_qps: Option<i64>,
    /// 公网QPS
    #[serde(rename = "ExtranetQps")]
    pub extranet_qps: Option<i64>,
}

impl crate::FlatSerialize for QoSConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.total_upload_bandwidth,
            &format!("{}.TotalUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_upload_bandwidth,
            &format!("{}.IntranetUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_upload_bandwidth,
            &format!("{}.ExtranetUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_download_bandwidth,
            &format!("{}.TotalDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_download_bandwidth,
            &format!("{}.IntranetDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_download_bandwidth,
            &format!("{}.ExtranetDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_qps,
            &format!("{}.TotalQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_qps,
            &format!("{}.IntranetQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_qps,
            &format!("{}.ExtranetQps", name),
            params,
        );
    }
}

/// 获取资源池信息的响应体
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetResourcePoolInfoResp {
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    pub owner: Option<String>,
    #[serde(rename = "CreateTime")]
    pub create_time: Option<String>,
    #[serde(rename = "QosConfiguration")]
    pub qos_configuration: Option<QoSConfiguration>,
}

impl crate::FlatSerialize for GetResourcePoolInfoResp {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qos_configuration,
            &format!("{}.QosConfiguration", name),
            params,
        );
    }
}

/// 存储空间TLS版本配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpsConfigurationTls {
    /// 是否启用TLS版本配置
    #[serde(rename = "Enable")]
    pub enable: bool,
    /// TLS版本列表
    #[serde(rename = "TLSVersion")]
    #[serde(default)]
    pub tls_version: Vec<String>,
}

impl crate::FlatSerialize for HttpsConfigurationTls {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.enable, &format!("{}.Enable", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.tls_version,
            &format!("{}.TLSVersion", name),
            params,
        );
    }
}

/// 存储空间加密套件配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpsConfigurationCipherSuite {
    /// 是否启用加密套件配置
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,
    /// 是否使用强加密套件
    #[serde(rename = "StrongCipherSuite")]
    pub strong_cipher_suite: Option<bool>,
    /// 自定义加密套件
    #[serde(rename = "CustomCipherSuite")]
    #[serde(default)]
    pub custom_cipher_suite: Vec<String>,
    /// 用于TLS1.3版本的自定义加密套件
    #[serde(rename = "TLS13CustomCipherSuite")]
    #[serde(default)]
    pub tls13_custom_cipher_suite: Vec<String>,
}

impl crate::FlatSerialize for HttpsConfigurationCipherSuite {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.enable, &format!("{}.Enable", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.strong_cipher_suite,
            &format!("{}.StrongCipherSuite", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.custom_cipher_suite,
            &format!("{}.CustomCipherSuite", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.tls13_custom_cipher_suite,
            &format!("{}.TLS13CustomCipherSuite", name),
            params,
        );
    }
}

/// 存储空间TLS版本配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpsConfiguration {
    /// 存储空间TLS版本配置
    #[serde(rename = "TLS")]
    pub tls: Option<HttpsConfigurationTls>,
    /// 存储空间加密套件配置
    #[serde(rename = "CipherSuite")]
    pub cipher_suite: Option<HttpsConfigurationCipherSuite>,
}

impl crate::FlatSerialize for HttpsConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tls, &format!("{}.TLS", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.cipher_suite,
            &format!("{}.CipherSuite", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for HttpsConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IncrementInventorySchedule {
    #[serde(rename = "Frequency")]
    pub frequency: Option<i64>,
}

impl crate::FlatSerialize for IncrementInventorySchedule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.frequency,
            &format!("{}.Frequency", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IncrementalInventoryOptionalFields {
    #[serde(rename = "Field")]
    #[serde(default)]
    pub field: Vec<String>,
}

impl crate::FlatSerialize for IncrementalInventoryOptionalFields {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.field, &format!("{}.Field", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IncrementalInventory {
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "Schedule")]
    pub schedule: Option<IncrementInventorySchedule>,
    #[serde(rename = "OptionalFields")]
    pub optional_fields: Option<IncrementalInventoryOptionalFields>,
}

impl crate::FlatSerialize for IncrementalInventory {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.is_enabled,
            &format!("{}.IsEnabled", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.schedule, &format!("{}.Schedule", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.optional_fields,
            &format!("{}.OptionalFields", name),
            params,
        );
    }
}

/// 默认主页的容器。
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IndexDocument {
    /// 默认主页。
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    /// 访问子目录时，是否支持转到子目录下的默认主页。取值范围如下：
    ///   - **true**：转到子目录下的默认主页。
    ///   - **false**（默认）：不转到子目录下的默认主页，而是转到根目录下的默认主页。
    ///
    /// 假设默认主页为index.html，要访问`bucket.oss-cn-hangzhou.aliyuncs.com/subdir/`，如果设置**SupportSubDir**为false，则转到`bucket.oss-cn-hangzhou.aliyuncs.com/index.html`；如果设置**SupportSubDir**为true，则转到`bucket.oss-cn-hangzhou.aliyuncs.com/subdir/index.html`。
    #[serde(rename = "SupportSubDir")]
    pub support_sub_dir: Option<bool>,
    /// 设置默认主页后，访问以非正斜线（/）结尾的Object，且该Object不存在时的行为。 只有设置**SupportSubDir**为true时才生效，且生效的顺序在RoutingRule之后、ErrorFile之前。
    /// 假设默认主页为index.html，要访问的文件路径为`bucket.oss-cn-hangzhou.aliyuncs.com/abc`，且abc这个Object不存在，此时**Type**的不同取值对应的行为如下：
    ///   - **0**（默认）：检查abc/index.html是否存在（即`Object + 正斜线（/）+ 主页`的形式），如果存在则返回302，Location头为`/abc/`的URL编码（即`正斜线（/） + Object + 正斜线（/）`的形式），如果不存在则返回404，继续检查ErrorFile。
    ///   - **1**：直接返回404，报错NoSuchKey，继续检查ErrorFile。
    ///   - **2**：检查abc/index.html是否存在，如果存在则返回该Object的内容；如果不存在则返回404，继续检查ErrorFile。
    ///
    #[serde(rename = "Type")]
    pub r#type: Option<i64>,
}

impl crate::FlatSerialize for IndexDocument {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.suffix, &format!("{}.Suffix", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.support_sub_dir,
            &format!("{}.SupportSubDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
    }
}

/// 根节点。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InitiateWormConfiguration {
    /// 指定Object保留天数。
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: i32,
}

impl crate::FlatSerialize for InitiateWormConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.retention_period_in_days,
            &format!("{}.RetentionPeriodInDays", name),
            params,
        );
    }
}

/// 保存Select请求的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JSONInput {
    /// 指定输入JSON的类型。
    #[serde(rename = "Type")]
    pub r#type: Option<JSONType>,
    /// 指定查询文件的范围（可选）。支持两种格式：
    /// > 使用Range参数查询的文件需要有select meta。关于select meta的更多信息，请参见[CreateSelectObjectMeta](~~74054~~)。
    ///
    /// - 按行查询：line-range=start-end。例如line-range=10-20表示扫描第10行到第20行。
    ///
    /// - 按Split查询：split-range=start-end。例如split-range=10-20表示扫描第10到第20个split。
    ///
    /// <br>其中start和end均为inclusive。其格式和range get中的range参数一致。
    /// <br>仅在文档是CSV或者JSON Type为LINES时使用。
    #[serde(rename = "Range")]
    pub range: Option<String>,
    /// 将JSON中的数字（整数和浮点数）解析成字符串。目前JSON中的浮点数解析时会损失精度，如果要完整保留原始数据，则推荐用该选项。如果需要进行数值计算，则可以在SQL中cast成需要的格式，例如int、double、decimal。
    /// <br>默认值： false
    #[serde(rename = "ParseJsonNumberAsString")]
    pub parse_json_number_as_string: Option<bool>,
}

impl crate::FlatSerialize for JSONInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
        crate::FlatSerialize::flat_serialize(&self.range, &format!("{}.Range", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.parse_json_number_as_string,
            &format!("{}.ParseJsonNumberAsString", name),
            params,
        );
    }
}

/// 保存Select请求的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InputSerialization {
    /// 指定文件压缩类型（可选）。目前不支持任何压缩，即取值只能是None。
    #[serde(rename = "CompressionType")]
    pub compression_type: Option<CompressionType>,
    /// 指定CSV输入格式。
    #[serde(rename = "CSV")]
    pub csv: Option<CSVInput>,
    /// 指定JSON输入格式。
    #[serde(rename = "JSON")]
    pub json: Option<JSONInput>,
}

impl crate::FlatSerialize for InputSerialization {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.compression_type,
            &format!("{}.CompressionType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.csv, &format!("{}.CSV", name), params);
        crate::FlatSerialize::flat_serialize(&self.json, &format!("{}.JSON", name), params);
    }
}

/// 保存SSE-KMS加密密钥的容器。
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SSEKMS {
    /// KMS密钥ID。
    #[serde(rename = "KeyId")]
    pub key_id: Option<String>,
}

impl crate::FlatSerialize for SSEKMS {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key_id, &format!("{}.KeyId", name), params);
    }
}

/// 清单文件的加密方式。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryEncryption {
    /// 保存SSE-OSS加密方式的容器。
    #[serde(rename = "SSE-OSS")]
    pub sseoss: Option<String>,
    /// 保存SSE-KMS加密密钥的容器。
    #[serde(rename = "SSE-KMS")]
    pub ssekms: Option<SSEKMS>,
}

impl crate::FlatSerialize for InventoryEncryption {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.sseoss, &format!("{}.SSE-OSS", name), params);
        crate::FlatSerialize::flat_serialize(&self.ssekms, &format!("{}.SSE-KMS", name), params);
    }
}

/// 清单结果导出后存放的Bucket信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryOSSBucketDestination {
    /// 清单文件的文件格式。 清单文件导出后的格式为使用GZIP压缩的CSV文件。
    #[serde(rename = "Format")]
    pub format: Option<InventoryFormat>,
    /// Bucket所有者授予的账户ID。
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    /// 具有读取源Bucket所有文件和向目标Bucket写入文件权限的角色名，格式为`acs:ram::uid:role/rolename`。
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// 存放导出的清单文件的Bucket。
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    /// 清单文件的存储路径前缀。
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    /// 清单文件的加密方式。
    #[serde(rename = "Encryption")]
    pub encryption: Option<InventoryEncryption>,
}

impl crate::FlatSerialize for InventoryOSSBucketDestination {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.format, &format!("{}.Format", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.account_id,
            &format!("{}.AccountId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.role_arn, &format!("{}.RoleArn", name), params);
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.encryption,
            &format!("{}.Encryption", name),
            params,
        );
    }
}

/// 存放清单结果的信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryDestination {
    /// 清单结果导出后存放的Bucket信息。
    #[serde(rename = "OSSBucketDestination")]
    pub oss_bucket_destination: Option<InventoryOSSBucketDestination>,
}

impl crate::FlatSerialize for InventoryDestination {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.oss_bucket_destination,
            &format!("{}.OSSBucketDestination", name),
            params,
        );
    }
}

/// Contains the frequency that inventory lists are exported
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventorySchedule {
    /// 清单文件导出的周期。
    ///
    /// 有效值：
    ///
    /// Daily：按天导出清单文件。
    ///
    /// Weekly：按周导出清单文件。
    #[serde(rename = "Frequency")]
    pub frequency: Option<InventoryFrequency>,
}

impl crate::FlatSerialize for InventorySchedule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.frequency,
            &format!("{}.Frequency", name),
            params,
        );
    }
}

/// 清单筛选的前缀。指定前缀后，清单将筛选出符合前缀设置的对象。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryFilter {
    /// 筛选规则的匹配前缀。
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    /// 筛选文件最后修改时间的起始时间戳，单位为秒。  取值范围：[1262275200, 253402271999]
    #[serde(rename = "LastModifyBeginTimeStamp")]
    pub last_modify_begin_time_stamp: Option<i64>,
    /// 筛选文件最后修改时间的终止时间戳，单位为秒。  取值范围：[1262275200, 253402271999]
    #[serde(rename = "LastModifyEndTimeStamp")]
    pub last_modify_end_time_stamp: Option<i64>,
    /// 筛选文件的最小大小，单位为B。  取值范围：大于等于0 B，小于等于48.8 TB。
    #[serde(rename = "LowerSizeBound")]
    pub lower_size_bound: Option<i64>,
    /// 筛选文件的最大大小，单位为B。  取值范围：大于0 B，小于等于48.8 TB。
    #[serde(rename = "UpperSizeBound")]
    pub upper_size_bound: Option<i64>,
    /// 筛选文件的存储类型，支持指定多种存储类型。  可选值：  Standard：标准存储  IA：低频访问  Archive：归档存储  ColdArchive：冷归档存储  All（默认值）：所有存储类型
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<String>,
}

impl crate::FlatSerialize for InventoryFilter {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modify_begin_time_stamp,
            &format!("{}.LastModifyBeginTimeStamp", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modify_end_time_stamp,
            &format!("{}.LastModifyEndTimeStamp", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.lower_size_bound,
            &format!("{}.LowerSizeBound", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.upper_size_bound,
            &format!("{}.UpperSizeBound", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
    }
}

/// 清单结果中包含的配置项列表。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryConfigurationOptionalFields {
    /// 清单结果中包含的配置项。
    /// 可选的配置项包括：
    ///
    /// - Size：Object的大小。
    /// - LastModifiedDate：Object的最后修改时间。
    /// - ETag：Object的ETag值，用于标识Object的内容。
    /// - StorageClass：Object的存储类型。
    /// - IsMultipartUploaded：是否为通过分片上传方式上传的Object。
    /// - EncryptionStatus：Object是否加密。
    ///
    #[serde(rename = "Field")]
    #[serde(default)]
    pub field: Vec<InventoryOptionalField>,
}

impl crate::FlatSerialize for InventoryConfigurationOptionalFields {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.field, &format!("{}.Field", name), params);
    }
}

/// 存储清单配置信息的容器。
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryConfiguration {
    /// 自定义清单名称，清单名称在当前Bucket下必须唯一。
    #[serde(rename = "Id")]
    pub id: Option<String>,
    /// 是否启用清单功能。取值范围如下：
    ///
    /// - true：启用清单功能。
    /// - false：不启用清单功能。
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,
    /// 存放清单结果。
    #[serde(rename = "Destination")]
    pub destination: Option<InventoryDestination>,
    /// 存放清单导出周期信息的容器。
    #[serde(rename = "Schedule")]
    pub schedule: Option<InventorySchedule>,
    /// 清单筛选的前缀。指定前缀后，清单将筛选出符合前缀设置的对象。
    #[serde(rename = "Filter")]
    pub filter: Option<InventoryFilter>,
    /// 是否在清单中包含Object版本信息。
    /// 取值范围如下：
    ///
    /// - All：导出Object的所有版本信息。
    ///
    /// - Current：导出Object的当前版本信息。
    #[serde(rename = "IncludedObjectVersions")]
    pub included_object_versions: Option<String>,
    /// 清单结果中包含的配置项列表。
    #[serde(rename = "OptionalFields")]
    pub optional_fields: Option<InventoryConfigurationOptionalFields>,
    #[serde(rename = "IncrementalInventory")]
    pub incremental_inventory: Option<IncrementalInventory>,
}

impl crate::FlatSerialize for InventoryConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.Id", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.is_enabled,
            &format!("{}.IsEnabled", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.destination,
            &format!("{}.Destination", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.schedule, &format!("{}.Schedule", name), params);
        crate::FlatSerialize::flat_serialize(&self.filter, &format!("{}.Filter", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.included_object_versions,
            &format!("{}.IncludedObjectVersions", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.optional_fields,
            &format!("{}.OptionalFields", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.incremental_inventory,
            &format!("{}.IncrementalInventory", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for InventoryConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 保存Select请求的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JSONOutput {
    /// 指定换行符，以Base64编码。未编码前的值最多为两个字符，以字符的ANSI值表示，例如在Java中使用`\n`表示换行。
    /// <br>默认值：`\n`
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: Option<String>,
}

impl crate::FlatSerialize for JSONOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.record_delimiter,
            &format!("{}.RecordDelimiter", name),
            params,
        );
    }
}

/// 指定Object生命周期规则的过期属性。 对于受版本控制的Bucket，指定的过期属性只对Object的当前版本生效。
/// <br>Object的过期时间必须大于转储为IA或Archive类型的时间。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleExpiration {
    /// 指定一个日期，OSS会对最后更新时间早于该日期的数据执行生命周期规则。日期格式为yyyy-MM-ddT00:00:00.000Z。
    /// <br>日期需要按照ISO8601标准表示并使用UTC时间，且必须为UTC的零点。
    #[serde(rename = "CreatedBeforeDate")]
    pub created_before_date: Option<String>,
    /// 指定生命周期规则在距离Object最后更新多少天后生效。
    #[serde(rename = "Days")]
    pub days: Option<i32>,
    /// 指定是否自动移除过期删除标记。
    ///
    /// - true：表示自动移除过期删除标记。取值为true时，不支持指定Days或CreatedBeforeDate。
    ///
    /// - false：表示不会自动移除过期删除标记。取值为false时，则必须指定Days或CreatedBeforeDate。
    #[serde(rename = "ExpiredObjectDeleteMarker")]
    pub expired_object_delete_marker: Option<bool>,
}

impl crate::FlatSerialize for LifecycleRuleExpiration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.created_before_date,
            &format!("{}.CreatedBeforeDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.days, &format!("{}.Days", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.expired_object_delete_marker,
            &format!("{}.ExpiredObjectDeleteMarker", name),
            params,
        );
    }
}

/// 指定Object在有效生命周期中，OSS何时将Object转储为IA、Archive和ColdArchive存储类型 。
/// Standard Bucket中的Standard Object可以转储为IA、Archive或ColdArchive存储类型，但转储Archive存储类型的时间必须比转储IA存储类型的时间长。例如Transition
/// IA设置Days为30，Transition Archive设置Days必须大于30。
///
/// > Days或CreatedBeforeDate只能二选一。
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleTransitionItem {
    /// 指定一个日期，OSS会对最后更新时间早于该日期的数据执行生命周期规则。日期必须服从ISO8601的格式，且要求是UTC的零点。
    #[serde(rename = "CreatedBeforeDate")]
    pub created_before_date: Option<String>,
    /// 指定生命周期规则在距离Object最后更新多少天后生效。
    #[serde(rename = "Days")]
    pub days: Option<i32>,
    /// 指定Object转储的存储类型。
    ///
    /// - IA：低频访问
    /// - Archive：归档存储
    /// - ColdArchive：冷归档存储
    ///
    /// > IA Bucket中的Object可以转储为Archive或者ColdArchive存储类型，但不支持转储为Standard存储类型。
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    /// 指定是否基于last access time匹配规则。取值如下：
    ///
    /// - true：采用Object的last access time（最后一次访问时间）匹配规则。
    ///
    /// - false（默认值）：采用Object的last modify time（最后一次修改时间）匹配规则。
    #[serde(rename = "IsAccessTime")]
    pub is_access_time: Option<bool>,
    /// 指定Object转为非标准存储后，再次访问时是否将Object转为标准存储。只有当IsAccessTime设置为true时才有效。取值如下：
    ///
    /// - true：Object由非标准存储转为标准存储。
    ///
    /// - false（默认值）：Object仍为非标准存储。
    #[serde(rename = "ReturnToStdWhenVisit")]
    pub return_to_std_when_visit: Option<bool>,
    /// 基于最后一次访问时间设置生命周期规则时，指定是否将小于64 KB的Object转储为低频、归档、冷归档文件类型。取值如下：
    ///
    /// - true（默认值）：转储包含小于64 KB在内的所有Object。当Object小于64 KB时，按照64 KB计算。当Object大于或等于64 KB时，按照实际大小计算。设置为true时，可能会增加存储费用。
    ///
    /// - false：不转储小于64 KB的Object。
    #[serde(rename = "AllowSmallFile")]
    pub allow_small_file: Option<bool>,
}

impl crate::FlatSerialize for LifecycleRuleTransitionItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.created_before_date,
            &format!("{}.CreatedBeforeDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.days, &format!("{}.Days", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_access_time,
            &format!("{}.IsAccessTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.return_to_std_when_visit,
            &format!("{}.ReturnToStdWhenVisit", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.allow_small_file,
            &format!("{}.AllowSmallFile", name),
            params,
        );
    }
}

/// 指定未完成分片上传的过期属性。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleAbortMultipartUpload {
    /// 指定生命周期规则在距离Object最后更新多少天后生效。
    #[serde(rename = "Days")]
    pub days: Option<i32>,
    /// 指定一个日期，OSS会对最后更新时间早于该日期的数据执行生命周期规则。日期必须服从ISO8601的格式，且要求是UTC的零点。
    #[serde(rename = "CreatedBeforeDate")]
    pub created_before_date: Option<String>,
}

impl crate::FlatSerialize for LifecycleRuleAbortMultipartUpload {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.days, &format!("{}.Days", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.created_before_date,
            &format!("{}.CreatedBeforeDate", name),
            params,
        );
    }
}

/// 设置Bucket Tag的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    /// 指定Bucket Tag的Key。
    ///
    /// > - 最大长度为64字节。
    /// > - 不能以`http ://`、`https://`、`Aliyun`为前缀。
    /// > - 必须为UTF-8编码。
    /// > - 不能为空。
    #[serde(rename = "Key")]
    pub key: String,
    /// 指定Bucket Tag的Value。
    ///  > - 最大长度为128字节。
    ///  >  - 必须为UTF-8编码。
    ///  >  - 可以为空。
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl crate::FlatSerialize for Tag {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
    }
}

/// 指定Object非当前版本生命周期规则的过期属性。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleNoncurrentVersionExpiration {
    /// 指定生命周期规则在Object成为非当前版本多少天后生效。
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: Option<i32>,
}

impl crate::FlatSerialize for LifecycleRuleNoncurrentVersionExpiration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.noncurrent_days,
            &format!("{}.NoncurrentDays", name),
            params,
        );
    }
}

/// 在有效的生命周期规则中，OSS何时将指定Object的非当前版本转储为IA或者Archive存储类型 。
/// Standard类型的Object转储为Archive类型的时间必须大于转储为IA类型的时间。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleNoncurrentVersionTransitionItem {
    /// 指定生命周期规则在Object成为非当前版本多少天后生效。
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: Option<i32>,
    /// 指定Object转储的存储类型。
    ///
    /// - IA：低频访问
    /// - Archive：归档存储
    /// - ColdArchive：冷归档存储
    ///
    /// > IA Bucket中的Object可以转储为Archive或者ColdArchive存储类型，但不支持转储为Standard存储类型。
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    /// 指定是否基于last access time匹配规则。取值如下：
    ///
    /// - true：采用Object的last access time（最后一次访问时间）匹配规则。
    ///
    /// - false（默认值）：采用Object的last modify time（最后一次修改时间）匹配规则。
    #[serde(rename = "IsAccessTime")]
    pub is_access_time: Option<bool>,
    /// 指定Object转为非标准存储后，再次访问时是否将Object转为标准存储。只有当IsAccessTime设置为true时才有效。取值如下：
    ///
    /// - true：Object由非标准存储转为标准存储。
    ///
    /// - false（默认值）：Object仍为非标准存储。
    #[serde(rename = "ReturnToStdWhenVisit")]
    pub return_to_std_when_visit: Option<bool>,
    /// 基于最后一次访问时间设置生命周期规则时，指定是否将小于64 KB的Object转储为低频、归档、冷归档文件类型。取值如下：
    ///
    /// - true（默认值）：转储包含小于64 KB在内的所有Object。当Object小于64 KB时，按照64 KB计算。当Object大于或等于64 KB时，按照实际大小计算。设置为true时，可能会增加存储费用。
    ///
    /// - false：不转储小于64 KB的Object。
    #[serde(rename = "AllowSmallFile")]
    pub allow_small_file: Option<bool>,
}

impl crate::FlatSerialize for LifecycleRuleNoncurrentVersionTransitionItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.noncurrent_days,
            &format!("{}.NoncurrentDays", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_access_time,
            &format!("{}.IsAccessTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.return_to_std_when_visit,
            &format!("{}.ReturnToStdWhenVisit", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.allow_small_file,
            &format!("{}.AllowSmallFile", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleFilterNotItem {
    /// 本条排除规则所适用的Object前缀。不可为空。  如果Rule节点下配置了Prefix，则Not节点下的Prefix必须以Rule节点下的Prefix为前缀。例如，Rule节点下配置的Prefix为dir，则Not节点下的Prefix必须以dir开头，例如dir1、dir2等。  如果Not节点下未配置Tag，则Not节点下配置的Prefix不能和Rule节点下的Prefix相同。
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    /// 本条排除规则所适用的Object标签，至多一个，可置空。
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
}

impl crate::FlatSerialize for LifecycleRuleFilterNotItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
    }
}

/// 本条规则的排除条件。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleFilter {
    /// 满足何种规则跳过。
    #[serde(rename = "Not")]
    #[serde(default)]
    pub not: Vec<LifecycleRuleFilterNotItem>,
    /// 本条生命周期规则只对文件大小大于该值的文件生效
    #[serde(rename = "ObjectSizeGreaterThan")]
    pub object_size_greater_than: Option<i64>,
    /// 本条生命周期规则只对文件大小小于该值的文件生效
    #[serde(rename = "ObjectSizeLessThan")]
    pub object_size_less_than: Option<i64>,
}

impl crate::FlatSerialize for LifecycleRuleFilter {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.not, &format!("{}.Not", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.object_size_greater_than,
            &format!("{}.ObjectSizeGreaterThan", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.object_size_less_than,
            &format!("{}.ObjectSizeLessThan", name),
            params,
        );
    }
}

/// 生命周期规则的容器。
///   - 不支持Archive Bucket创建转储规则。
///   - Object设置过期时间必须大于转储为IA或者Archive存储类型的时间。
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRule {
    /// 标识规则的唯一ID。最多由255个字节组成。如没有指定，或者该值为空时，OSS会自动生成一个唯一ID。
    #[serde(rename = "ID")]
    pub id: String,
    /// 指定规则所适用的前缀（Prefix）。Prefix不可重复。
    ///   - 若指定了Prefix，则表示此规则仅适用于Bucket中与Prefix匹配的Object。
    ///   - 若Prefix置空，则表示此规则适用于Bucket中的所有Object。
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    /// 是否启用规则。取值范围如下：
    /// - Enabled：表示启用规则，OSS会定期执行该规则。
    /// - Disabled：表示不启用规则，OSS会忽略该规则。
    #[serde(rename = "Status")]
    pub status: String,
    /// 指定Object生命周期规则的过期属性。 对于受版本控制的Bucket，指定的过期属性只对Object的当前版本生效。
    /// <br>Object的过期时间必须大于转储为IA或Archive类型的时间。
    #[serde(rename = "Expiration")]
    pub expiration: Option<LifecycleRuleExpiration>,
    /// 指定Object在有效生命周期中，OSS何时将Object转储为IA、Archive和ColdArchive存储类型 。
    /// Standard Bucket中的Standard Object可以转储为IA、Archive或ColdArchive存储类型，但转储Archive存储类型的时间必须比转储IA存储类型的时间长。例如Transition
    /// IA设置Days为30，Transition Archive设置Days必须大于30。
    ///
    /// > Days或CreatedBeforeDate只能二选一。
    ///
    #[serde(rename = "Transition")]
    #[serde(default)]
    pub transition: Vec<LifecycleRuleTransitionItem>,
    /// 指定未完成分片上传的过期属性。
    #[serde(rename = "AbortMultipartUpload")]
    pub abort_multipart_upload: Option<LifecycleRuleAbortMultipartUpload>,
    /// 指定规则所适用的对象标签，可设置多个。
    #[serde(rename = "Tag")]
    #[serde(default)]
    pub tag: Vec<Tag>,
    /// 指定Object非当前版本生命周期规则的过期属性。
    #[serde(rename = "NoncurrentVersionExpiration")]
    pub noncurrent_version_expiration: Option<LifecycleRuleNoncurrentVersionExpiration>,
    /// 在有效的生命周期规则中，OSS何时将指定Object的非当前版本转储为IA或者Archive存储类型 。
    /// Standard类型的Object转储为Archive类型的时间必须大于转储为IA类型的时间。
    #[serde(rename = "NoncurrentVersionTransition")]
    #[serde(default)]
    pub noncurrent_version_transition: Vec<LifecycleRuleNoncurrentVersionTransitionItem>,
    /// 本条规则的排除条件。
    #[serde(rename = "Filter")]
    pub filter: Option<LifecycleRuleFilter>,
    /// 存储空间开启访问追踪的时间戳
    #[serde(rename = "AtimeBase")]
    pub atime_base: Option<i64>,
}

impl crate::FlatSerialize for LifecycleRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.expiration,
            &format!("{}.Expiration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.transition,
            &format!("{}.Transition", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.abort_multipart_upload,
            &format!("{}.AbortMultipartUpload", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.noncurrent_version_expiration,
            &format!("{}.NoncurrentVersionExpiration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.noncurrent_version_transition,
            &format!("{}.NoncurrentVersionTransition", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.filter, &format!("{}.Filter", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.atime_base,
            &format!("{}.AtimeBase", name),
            params,
        );
    }
}

/// Lifecycle配置的容器，最多可容纳1000条规则。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleConfiguration {
    /// 生命周期规则的容器。Object设置过期时间必须大于转储为IA或者Archive存储类型的时间。
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: Vec<LifecycleRule>,
}

impl crate::FlatSerialize for LifecycleConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

impl crate::ToCodeMessage for LifecycleConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 保存所有接入点信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsResultAccessPoints {
    /// 保存接入点信息的列表。
    #[serde(rename = "AccessPoint")]
    #[serde(default)]
    pub access_point: Vec<AccessPoint>,
}

impl crate::FlatSerialize for ListAccessPointsResultAccessPoints {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point,
            &format!("{}.AccessPoint", name),
            params,
        );
    }
}

/// 保存本次列举接入点信息结果的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsResult {
    /// 请求中返回的结果是否被截断。返回值如下：  true：表示本次未返回全部结果。  false：表示本次已返回全部结果。
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<String>,
    /// 表明本次ListAccessPoints请求包含后续结果，需要将NextContinuationToken指定为continuation-token继续获取结果。
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: Option<String>,
    /// 接入点所属的阿里云账号UID。
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    /// 保存所有接入点信息的容器。
    #[serde(rename = "AccessPoints")]
    pub access_points: Option<ListAccessPointsResultAccessPoints>,
    /// 本次列举操作所设置的结果最大数量。
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<i32>,
}

impl crate::FlatSerialize for ListAccessPointsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
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
        crate::FlatSerialize::flat_serialize(
            &self.account_id,
            &format!("{}.AccountId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_points,
            &format!("{}.AccessPoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
    }
}

impl crate::ToCodeMessage for ListAccessPointsResult {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAllMyCacheResultCaches {
    #[serde(rename = "Cache")]
    #[serde(default)]
    pub cache: Vec<CacheBaseInfo>,
}

impl crate::FlatSerialize for ListAllMyCacheResultCaches {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.cache, &format!("{}.Cache", name), params);
    }
}

/// 加速器列举结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAllMyCacheResult {
    #[serde(rename = "Caches")]
    pub caches: Option<ListAllMyCacheResultCaches>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Marker")]
    pub marker: Option<String>,
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<String>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,
}

impl crate::FlatSerialize for ListAllMyCacheResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.caches, &format!("{}.Caches", name), params);
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.marker, &format!("{}.Marker", name), params);
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_marker,
            &format!("{}.NextMarker", name),
            params,
        );
    }
}

/// 保存请求者QoS配置信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RequesterQoSInfo {
    /// 请求者ID
    #[serde(rename = "Requester")]
    pub requester: Option<String>,
    /// 请求者QoS配置
    #[serde(rename = "QoSConfiguration")]
    pub qo_s_configuration: Option<QoSConfiguration>,
}

impl crate::FlatSerialize for RequesterQoSInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.requester,
            &format!("{}.Requester", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qo_s_configuration,
            &format!("{}.QoSConfiguration", name),
            params,
        );
    }
}

/// 列举Bucket级别请求者流控的结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketRequesterQoSInfosResult {
    /// 存储空间的名称
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    /// 标识本次列举的起点
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<String>,
    /// 标识下次列举的起点
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: Option<String>,
    /// 列举结果是否截断
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<bool>,
    /// 请求者QoS配置信息列表
    #[serde(rename = "RequesterQoSInfo")]
    #[serde(default)]
    pub requester_qo_s_info: Vec<RequesterQoSInfo>,
}

impl crate::FlatSerialize for ListBucketRequesterQoSInfosResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.continuation_token,
            &format!("{}.ContinuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_continuation_token,
            &format!("{}.NextContinuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.requester_qo_s_info,
            &format!("{}.RequesterQoSInfo", name),
            params,
        );
    }
}

/// oss加速器异步预热任务执行历史记录
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListDataLakeCachePrefetchJobHistory {
    #[serde(rename = "DataLakeCachePrefetchJobHistory")]
    #[serde(default)]
    pub data_lake_cache_prefetch_job_history: Vec<DataLakeCachePrefetchJobHistory>,
}

impl crate::FlatSerialize for ListDataLakeCachePrefetchJobHistory {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.data_lake_cache_prefetch_job_history,
            &format!("{}.DataLakeCachePrefetchJobHistory", name),
            params,
        );
    }
}

/// 数据湖元数据转换历史任务列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListDataLakeStorageTransferJobHistory {
    #[serde(rename = "DataLakeStorageTransferJobHistory")]
    #[serde(default)]
    pub data_lake_storage_transfer_job_history: Vec<DataLakeStorageTransferJobHistory>,
}

impl crate::FlatSerialize for ListDataLakeStorageTransferJobHistory {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.data_lake_storage_transfer_job_history,
            &format!("{}.DataLakeStorageTransferJobHistory", name),
            params,
        );
    }
}

/// 资源池中Bucket的信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResourcePoolBucket {
    /// 存储空间的名称
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// 存储空间加入到资源池的时间
    #[serde(rename = "JoinTime")]
    pub join_time: Option<String>,
}

impl crate::FlatSerialize for ResourcePoolBucket {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.join_time,
            &format!("{}.JoinTime", name),
            params,
        );
    }
}

/// 列举资源池中Bucket列表的结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListResourcePoolBucketsResult {
    /// 目标资源池名称
    #[serde(rename = "ResourcePool")]
    pub resource_pool: Option<String>,
    /// 本次列举起点
    #[serde(rename = "ContionuationToken")]
    pub contionuation_token: Option<String>,
    /// 下次列举起点
    #[serde(rename = "NextContionuationToken")]
    pub next_contionuation_token: Option<String>,
    /// 列举结果是否截断
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<bool>,
    /// 资源池中的存储空间列表
    #[serde(rename = "ResourcePoolBucket")]
    #[serde(default)]
    pub resource_pool_bucket: Vec<ResourcePoolBucket>,
}

impl crate::FlatSerialize for ListResourcePoolBucketsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resource_pool,
            &format!("{}.ResourcePool", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.contionuation_token,
            &format!("{}.ContionuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_contionuation_token,
            &format!("{}.NextContionuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_pool_bucket,
            &format!("{}.ResourcePoolBucket", name),
            params,
        );
    }
}

/// 列举资源池级别请求者流控的结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListResourcePoolRequesterQoSInfosResult {
    /// 资源池名称
    #[serde(rename = "ResourcePool")]
    pub resource_pool: Option<String>,
    /// 标识本次列举的起点
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<String>,
    /// 标识下次列举的起点
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: Option<String>,
    /// 列举结果是否截断
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<bool>,
    /// 请求者QoS配置信息列表
    #[serde(rename = "RequesterQoSInfo")]
    #[serde(default)]
    pub requester_qo_s_info: Vec<RequesterQoSInfo>,
}

impl crate::FlatSerialize for ListResourcePoolRequesterQoSInfosResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resource_pool,
            &format!("{}.ResourcePool", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.continuation_token,
            &format!("{}.ContinuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_continuation_token,
            &format!("{}.NextContinuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.requester_qo_s_info,
            &format!("{}.RequesterQoSInfo", name),
            params,
        );
    }
}

/// 用户资源池简略信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResourcePoolSimpleInfo {
    /// 资源池名称
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// 资源池创建时间
    #[serde(rename = "CreateTime")]
    pub create_time: Option<String>,
}

impl crate::FlatSerialize for ResourcePoolSimpleInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
    }
}

/// 列举当前用户在特定地域资源池的结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListResourcePoolsResult {
    /// 目标地域
    #[serde(rename = "Region")]
    pub region: Option<String>,
    /// 列举的资源池的所有者
    #[serde(rename = "Owner")]
    pub owner: Option<String>,
    /// 本次列举的起点
    #[serde(rename = "ContionuationToken")]
    pub contionuation_token: Option<String>,
    /// 下次列举的起点
    #[serde(rename = "NextContionuationToken")]
    pub next_contionuation_token: Option<String>,
    /// 本次列举结果是否截断
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<bool>,
    /// 资源池列表
    #[serde(rename = "ResourcePool")]
    #[serde(default)]
    pub resource_pool: Vec<ResourcePoolSimpleInfo>,
}

impl crate::FlatSerialize for ListResourcePoolsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.contionuation_token,
            &format!("{}.ContionuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_contionuation_token,
            &format!("{}.NextContionuationToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_pool,
            &format!("{}.ResourcePool", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListUserRegionsResultRegions {
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: Vec<String>,
}

impl crate::FlatSerialize for ListUserRegionsResultRegions {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
    }
}

/// 列举用户 Bucket 所属的区域
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListUserRegionsResult {
    #[serde(rename = "Regions")]
    pub regions: Option<ListUserRegionsResultRegions>,
}

impl crate::FlatSerialize for ListUserRegionsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.regions, &format!("{}.Regions", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VirtualBucketRealBucketItem {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

impl crate::FlatSerialize for VirtualBucketRealBucketItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VirtualBucket {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "RealBucket")]
    #[serde(default)]
    pub real_bucket: Vec<VirtualBucketRealBucketItem>,
}

impl crate::FlatSerialize for VirtualBucket {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.real_bucket,
            &format!("{}.RealBucket", name),
            params,
        );
    }
}

/// 列举的虚拟 Bucket 配置信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListVirtualBucketResult {
    #[serde(rename = "VirtualBucket")]
    #[serde(default)]
    pub virtual_bucket: Vec<VirtualBucket>,
}

impl crate::FlatSerialize for ListVirtualBucketResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.virtual_bucket,
            &format!("{}.VirtualBucket", name),
            params,
        );
    }
}

/// 保存推流地址的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelPublishUrls {
    /// 推流地址。
    ///
    /// >   - 推流地址是未加签名的URL，如Bucket ACL非public-read-write，则需先进行签名才可访问。
    /// >   - 播放地址是未加签名的URL，如Bucket ACL为private，则需先进行签名才可访问。
    #[serde(rename = "Url")]
    pub url: Option<String>,
}

impl crate::FlatSerialize for LiveChannelPublishUrls {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.url, &format!("{}.Url", name), params);
    }
}

/// 保存播放地址的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelPlayUrls {
    /// 播放地址。
    #[serde(rename = "Url")]
    pub url: Option<String>,
}

impl crate::FlatSerialize for LiveChannelPlayUrls {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.url, &format!("{}.Url", name), params);
    }
}

/// 保存返回每个LiveChannel信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannel {
    /// LiveChannel的名称。
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// LiveChannel的描述信息。
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// LiveChannel的状态。
    /// 有效值：
    /// - disabled：表示禁用LiveChannel。
    /// - enabled：表示启用LiveChannel。
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// LiveChannel配置的最后修改时间。
    /// 格式：ISO8601
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
    /// 保存LiveChannel对应的推流地址的容器。
    #[serde(rename = "PublishUrls")]
    pub publish_urls: Option<LiveChannelPublishUrls>,
    /// 保存LiveChannel对应的播放地址的容器。
    #[serde(rename = "PlayUrls")]
    pub play_urls: Option<LiveChannelPlayUrls>,
}

impl crate::FlatSerialize for LiveChannel {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.Description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.publish_urls,
            &format!("{}.PublishUrls", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.play_urls,
            &format!("{}.PlayUrls", name),
            params,
        );
    }
}

/// 当Status为Live时，保存音频流信息的容器。
/// > Video、Audio容器只有在Status为Live时才会返回，但Status为Live时不一定返回这两个容器。例如，客户端已经连接到LiveChannel，但尚未发送音视频数据，这种情况不会返回这两个容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelAudio {
    /// 当前音频流的码率。
    /// > Bandwidth为音频流/视频流最近一段时间内的平均码率。LiveChannel刚切换到Live状态时，返回的Bandwidth值可能为0。
    /// 单位：B/s
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Option<i64>,
    /// 当前音频流的采样率。
    #[serde(rename = "SampleRate")]
    pub sample_rate: Option<i64>,
    /// 当前音频流的编码格式。
    #[serde(rename = "Codec")]
    pub codec: Option<String>,
}

impl crate::FlatSerialize for LiveChannelAudio {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bandwidth,
            &format!("{}.Bandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.sample_rate,
            &format!("{}.SampleRate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.codec, &format!("{}.Codec", name), params);
    }
}

/// 保存转储配置的容器。
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelTarget {
    /// 指定转储的类型。
    ///
    /// 有效值：**HLS**
    ///
    /// >   - 转储类型为HLS时，OSS会在生成每个ts文件后更新m3u8文件。m3u8文件中最多包含最近的FragCount个ts文件。
    /// >   - 转储类型为HLS时，写入当前ts文件的音视频数据时长达到FragDuration指定的时长后，OSS会在收到下一个关键帧的时切换到下一个ts文件；如果max(2*FragDuration,
    ///                                           60s)后仍未收到下一个关键帧，OSS将强制切换文件，此时可能引起播放时卡顿。
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
    /// 当Type为HLS时，指定每个ts文件的时长。
    /// 单位：秒
    ///
    /// 取值范围：[1, 100]
    /// 默认值：**5**
    /// > FragDuration和FragCount的默认值只有在两者都未指定时才会生效；指定了其中一个，则另一个的值也必须指定。
    #[serde(rename = "FragDuration")]
    pub frag_duration: Option<i64>,
    /// 当Type为HLS时，指定m3u8文件中包含ts文件的个数。
    ///
    /// 取值范围：[1, 100]
    /// 默认值：**3**
    /// > FragDuration和FragCount的默认值只有在两者都未指定时才会生效；指定了其中一个，则另一个的值也必须指定。
    #[serde(rename = "FragCount")]
    pub frag_count: Option<i64>,
    /// 当Type为HLS时，指定生成的m3u8文件的名称。必须以”.m3u8”结尾，长度范围为[6, 128]。
    ///
    /// 默认值：**playlist.m3u8**
    /// 取值范围：[6, 128]
    #[serde(rename = "PlaylistName")]
    pub playlist_name: Option<String>,
}

impl crate::FlatSerialize for LiveChannelTarget {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.frag_duration,
            &format!("{}.FragDuration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.frag_count,
            &format!("{}.FragCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.playlist_name,
            &format!("{}.PlaylistName", name),
            params,
        );
    }
}

/// 保存高频截图操作Snapshot 选项的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelSnapshot {
    /// 用于高频截图操作的角色名称，要求有DestBucket的写权限和向NotifyTopic发消息的权限。
    #[serde(rename = "RoleName")]
    pub role_name: Option<String>,
    /// 保存高频截图目标Bucket，要求与当前Bucket是同一个Owner。
    #[serde(rename = "DestBucket")]
    pub dest_bucket: Option<String>,
    /// 用于通知用户高频截图操作结果的MNS的Topic。
    #[serde(rename = "NotifyTopic")]
    pub notify_topic: Option<String>,
    /// 高频截图的间隔长度。如果该段间隔时间内没有关键帧（I 帧），那么该间隔时间不截图。
    /// 单位：秒
    /// 取值范围：[1, 100]
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,
}

impl crate::FlatSerialize for LiveChannelSnapshot {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.role_name,
            &format!("{}.RoleName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.dest_bucket,
            &format!("{}.DestBucket", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.notify_topic,
            &format!("{}.NotifyTopic", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.interval, &format!("{}.Interval", name), params);
    }
}

/// 保存LiveChannel配置的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelConfiguration {
    /// LiveChannel的描述信息，最长128字节。
    #[serde(rename = "Description")]
    pub description: Option<String>,
    ///  指定LiveChannel的状态。
    ///
    /// 有效值：**enabled**、**disabled**
    /// 默认值：**enabled**
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 保存转储配置的容器。
    #[serde(rename = "Target")]
    pub target: Option<LiveChannelTarget>,
    /// 保存高频截图操作Snapshot选项的容器。
    #[serde(rename = "Snapshot")]
    pub snapshot: Option<LiveChannelSnapshot>,
}

impl crate::FlatSerialize for LiveChannelConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.Description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(&self.target, &format!("{}.Target", name), params);
        crate::FlatSerialize::flat_serialize(&self.snapshot, &format!("{}.Snapshot", name), params);
    }
}

/// 当Status为Live时，保存视频流信息的容器。
/// > **说明** Video、Audio容器只有在Status为Live时才会返回，但Status为Live时不一定返回这两个容器。例如，客户端已经连接到LiveChannel，但尚未发送音视频数据，这种情况不会返回这两个容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelVideo {
    /// 当前视频流的画面宽度。
    /// 单位：像素
    #[serde(rename = "Width")]
    pub width: Option<i64>,
    /// 当前视频流的画面高度。
    /// 单位：像素
    #[serde(rename = "Height")]
    pub height: Option<i64>,
    /// 当前视频流的帧率。
    #[serde(rename = "FrameRate")]
    pub frame_rate: Option<i64>,
    /// 当前视频流的码率。
    /// 单位：B/s
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Option<i64>,
    /// 当前视频流的编码格式。
    #[serde(rename = "Codec")]
    pub codec: Option<String>,
}

impl crate::FlatSerialize for LiveChannelVideo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.frame_rate,
            &format!("{}.FrameRate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bandwidth,
            &format!("{}.Bandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.codec, &format!("{}.Codec", name), params);
    }
}

/// 保存一次推流记录信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveRecord {
    /// 推流开始时间，使用ISO8601格式表示。
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,
    /// 推流结束时间，使用ISO8601格式表示。
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,
    /// 推流客户端的IP地址。
    #[serde(rename = "RemoteAddr")]
    pub remote_addr: Option<String>,
}

impl crate::FlatSerialize for LiveRecord {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.StartTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.end_time, &format!("{}.EndTime", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.remote_addr,
            &format!("{}.RemoteAddr", name),
            params,
        );
    }
}

/// 传输类型容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LocationTransferTypeTransferTypes {
    /// 数据复制时使用的数据传输类型。
    /// 取值：
    ///   - internal（默认值）：OSS默认传输链路。
    ///   - oss_acc：传输加速链路。只有创建跨区域复制规则时才能使用传输加速链路。
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: Vec<String>,
}

impl crate::FlatSerialize for LocationTransferTypeTransferTypes {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
    }
}

/// 包含TransferType的Location信息容器。
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LocationTransferType {
    /// 可复制到的目标Bucket所在的地域。
    #[serde(rename = "Location")]
    pub location: Option<String>,
    /// 传输类型容器。
    #[serde(rename = "TransferTypes")]
    pub transfer_types: Option<LocationTransferTypeTransferTypes>,
}

impl crate::FlatSerialize for LocationTransferType {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.transfer_types,
            &format!("{}.TransferTypes", name),
            params,
        );
    }
}

/// 聚合操作信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregation {
    /// 字段名称
    #[serde(rename = "Field")]
    pub field: Option<String>,
    /// 聚合操作中的操作符。
    ///
    /// - min：最小值
    /// - max：最大值
    /// - average：平均数
    /// - sum：求和
    /// - count：计数
    /// - distinct：去重统计
    /// - group：分组计数
    #[serde(rename = "Operation")]
    pub operation: Option<String>,
}

impl crate::FlatSerialize for MetaQueryAggregation {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.field, &format!("{}.Field", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.operation,
            &format!("{}.Operation", name),
            params,
        );
    }
}

/// 聚合操作信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregations {
    /// 单个聚合操作信息的容器。
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    pub aggregation: Vec<MetaQueryAggregation>,
}

impl crate::FlatSerialize for MetaQueryAggregations {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.aggregation,
            &format!("{}.Aggregation", name),
            params,
        );
    }
}

/// 多媒体元数据检索条件。仅用于向量检索
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryMediaTypes {
    /// 选择检索的多媒体类型。
    #[serde(rename = "MediaType")]
    #[serde(default)]
    pub media_type: Vec<String>,
}

impl crate::FlatSerialize for MetaQueryMediaTypes {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.media_type,
            &format!("{}.MediaType", name),
            params,
        );
    }
}

/// 查询条件的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQuery {
    /// 翻页的Token。从NextToken开始按字典序返回Object信息列表。
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
    /// 返回对象的最大个数，取值范围为0~100。
    /// 不设置此参数或者设置为0时，则默认值为100。
    #[serde(rename = "MaxResults")]
    pub max_results: Option<i64>,
    /// 查询条件。包括如下选项：
    ///
    /// - Operation：操作符。取值范围为eq（等于）、gt（大于）、gte（大于等于）、lt（小于）、 lte（小于等于）、match（模糊查询）、prefix（前缀查询）、and（逻辑与）、or（逻辑或）和not（逻辑非）。
    ///
    /// - Field：字段名称。
    ///
    /// - Value：字段值。
    ///
    /// - SubQueries：子查询条件，包括的选项与简单查询条件相同。只有当Operations为逻辑运算符（and、or和not）时，才需要设置子查询条件。
    #[serde(rename = "Query")]
    pub query: Option<String>,
    /// 对指定字段排序。
    #[serde(rename = "Sort")]
    pub sort: Option<String>,
    /// 排序方式。
    #[serde(rename = "Order")]
    pub order: Option<MetaQueryOrder>,
    /// 聚合操作信息的容器。
    #[serde(rename = "Aggregations")]
    pub aggregations: Option<MetaQueryAggregations>,
    /// 多媒体元数据检索条件。仅用于向量检索
    #[serde(rename = "MediaTypes")]
    pub media_types: Option<MetaQueryMediaTypes>,
    /// 查询条件，仅用于向量查询。
    #[serde(rename = "SimpleQuery")]
    pub simple_query: Option<String>,
}

impl crate::FlatSerialize for MetaQuery {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.NextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_results,
            &format!("{}.MaxResults", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.query, &format!("{}.Query", name), params);
        crate::FlatSerialize::flat_serialize(&self.sort, &format!("{}.Sort", name), params);
        crate::FlatSerialize::flat_serialize(&self.order, &format!("{}.Order", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.aggregations,
            &format!("{}.Aggregations", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.media_types,
            &format!("{}.MediaTypes", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.simple_query,
            &format!("{}.SimpleQuery", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregationsResultGroupsGroupItem {
    /// 分组聚合的值
    #[serde(rename = "Value")]
    pub value: Option<String>,
    /// 分组聚合的总个数
    #[serde(rename = "Count")]
    pub count: Option<i64>,
}

impl crate::FlatSerialize for MetaQueryAggregationsResultGroupsGroupItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
        crate::FlatSerialize::flat_serialize(&self.count, &format!("{}.Count", name), params);
    }
}

/// 分组聚合的结果列表的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregationsResultGroups {
    /// 分组聚合的结果列表
    #[serde(rename = "Group")]
    #[serde(default)]
    pub group: Vec<MetaQueryAggregationsResultGroupsGroupItem>,
}

impl crate::FlatSerialize for MetaQueryAggregationsResultGroups {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.group, &format!("{}.Group", name), params);
    }
}

/// 数据索引查询聚合结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregationsResult {
    /// 字段名称
    #[serde(rename = "Field")]
    pub field: Option<String>,
    /// 聚合操作符
    #[serde(rename = "Operation")]
    pub operation: Option<String>,
    /// 符合操作的结果值
    #[serde(rename = "Value")]
    pub value: Option<f64>,
    /// 分组聚合的结果列表的容器
    #[serde(rename = "Groups")]
    pub groups: Option<MetaQueryAggregationsResultGroups>,
}

impl crate::FlatSerialize for MetaQueryAggregationsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.field, &format!("{}.Field", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.operation,
            &format!("{}.Operation", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
        crate::FlatSerialize::flat_serialize(&self.groups, &format!("{}.Groups", name), params);
    }
}

/// 对象所附标签
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryTagging {
    /// 对象标签键
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 对象标签值
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl crate::FlatSerialize for MetaQueryTagging {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
    }
}

/// 对象的标签列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileOssTagging {
    /// 对象的标签
    #[serde(rename = "Tagging")]
    #[serde(default)]
    pub tagging: Vec<MetaQueryTagging>,
}

impl crate::FlatSerialize for MetaQueryFileOssTagging {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tagging, &format!("{}.Tagging", name), params);
    }
}

/// 用户自定义元数据
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryUserMeta {
    /// 用户自定义元数据键
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 用户自定义元数据值
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl crate::FlatSerialize for MetaQueryUserMeta {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
    }
}

/// 对象的自定义元数据列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileOssUserMeta {
    /// 对象的自定义元数据
    #[serde(rename = "UserMeta")]
    #[serde(default)]
    pub user_meta: Vec<MetaQueryUserMeta>,
}

impl crate::FlatSerialize for MetaQueryFileOssUserMeta {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.user_meta,
            &format!("{}.UserMeta", name),
            params,
        );
    }
}

/// 数据索引向量检索结果中的视频流信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespVideoStream {
    /// 编码器名称
    #[serde(rename = "CodecName")]
    pub codec_name: Option<String>,
    /// 视频流中使用的语言，格式为BCP 47
    #[serde(rename = "Language")]
    pub language: Option<String>,
    /// 码率，单位为比特每秒（bit/s）
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<i64>,
    /// 视频流帧率
    #[serde(rename = "FrameRate")]
    pub frame_rate: Option<String>,
    /// 视频流起始时间，单位为秒（s）
    #[serde(rename = "StartTime")]
    pub start_time: Option<f64>,
    /// 视频流持续时长，单位为秒（s）
    #[serde(rename = "Duration")]
    pub duration: Option<f64>,
    /// 视频帧数
    #[serde(rename = "FrameCount")]
    pub frame_count: Option<i64>,
    /// 像素位宽
    #[serde(rename = "BitDepth")]
    pub bit_depth: Option<i64>,
    /// 视频流像素格式
    #[serde(rename = "PixelFormat")]
    pub pixel_format: Option<String>,
    /// 色彩空间
    #[serde(rename = "ColorSpace")]
    pub color_space: Option<String>,
    /// 视频流画面高度，单位为像素（px）
    #[serde(rename = "Height")]
    pub height: Option<i64>,
    /// 视频流画面宽度，单位为像素（px）
    #[serde(rename = "Width")]
    pub width: Option<i64>,
}

impl crate::FlatSerialize for MetaQueryRespVideoStream {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.codec_name,
            &format!("{}.CodecName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.language, &format!("{}.Language", name), params);
        crate::FlatSerialize::flat_serialize(&self.bitrate, &format!("{}.Bitrate", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.frame_rate,
            &format!("{}.FrameRate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.StartTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.duration, &format!("{}.Duration", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.frame_count,
            &format!("{}.FrameCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bit_depth,
            &format!("{}.BitDepth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.pixel_format,
            &format!("{}.PixelFormat", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.color_space,
            &format!("{}.ColorSpace", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

/// 视频流列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileVideoStreams {
    /// 视频流
    #[serde(rename = "VideoStream")]
    pub video_stream: Option<MetaQueryRespVideoStream>,
}

impl crate::FlatSerialize for MetaQueryFileVideoStreams {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.video_stream,
            &format!("{}.VideoStream", name),
            params,
        );
    }
}

/// 数据索引向量检索结果中的音频流信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespAudioStream {
    /// 编码器名称
    #[serde(rename = "CodecName")]
    pub codec_name: Option<String>,
    /// 码率，单位为比特每秒（bit/s）
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<i64>,
    /// 采样率，单位为赫兹（Hz）
    #[serde(rename = "SampleRate")]
    pub sample_rate: Option<i64>,
    /// 音频流起始时间，单位为秒（s）
    #[serde(rename = "StartTime")]
    pub start_time: Option<f64>,
    /// 音频流持续时长，单位为秒（s）
    #[serde(rename = "Duration")]
    pub duration: Option<f64>,
    /// 声道数量
    #[serde(rename = "Channels")]
    pub channels: Option<i64>,
    /// 音频流中使用的语言，格式为BCP 47
    #[serde(rename = "Language")]
    pub language: Option<String>,
}

impl crate::FlatSerialize for MetaQueryRespAudioStream {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.codec_name,
            &format!("{}.CodecName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.bitrate, &format!("{}.Bitrate", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.sample_rate,
            &format!("{}.SampleRate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.StartTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.duration, &format!("{}.Duration", name), params);
        crate::FlatSerialize::flat_serialize(&self.channels, &format!("{}.Channels", name), params);
        crate::FlatSerialize::flat_serialize(&self.language, &format!("{}.Language", name), params);
    }
}

/// 音频流列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileAudioStreams {
    /// 音频流
    #[serde(rename = "AudioStream")]
    pub audio_stream: Option<MetaQueryRespAudioStream>,
}

impl crate::FlatSerialize for MetaQueryFileAudioStreams {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.audio_stream,
            &format!("{}.AudioStream", name),
            params,
        );
    }
}

/// 数据索引向量检索结果中的字幕流信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespSubtitle {
    /// 编码器名称
    #[serde(rename = "CodecName")]
    pub codec_name: Option<String>,
    /// 字幕语言，格式为BCP 47
    #[serde(rename = "Language")]
    pub language: Option<String>,
    /// 字幕流起始时间，单位为秒（s）
    #[serde(rename = "StartTime")]
    pub start_time: Option<f64>,
    /// 字幕流持续时长，单位为秒（s）。
    #[serde(rename = "Duration")]
    pub duration: Option<f64>,
}

impl crate::FlatSerialize for MetaQueryRespSubtitle {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.codec_name,
            &format!("{}.CodecName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.language, &format!("{}.Language", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.StartTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.duration, &format!("{}.Duration", name), params);
    }
}

/// 字幕流列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileSubtitles {
    /// 字幕流
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<MetaQueryRespSubtitle>,
}

impl crate::FlatSerialize for MetaQueryFileSubtitles {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.subtitle, &format!("{}.Subtitle", name), params);
    }
}

/// 数据索引向量检索结果中的地址信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespAddress {
    /// 完整地址
    #[serde(rename = "AddressLine")]
    pub address_line: Option<String>,
    /// 城市
    #[serde(rename = "City")]
    pub city: Option<String>,
    /// 区
    #[serde(rename = "District")]
    pub district: Option<String>,
    /// 语言，格式为BCP 47
    #[serde(rename = "Language")]
    pub language: Option<String>,
    /// 省份
    #[serde(rename = "Province")]
    pub province: Option<String>,
    /// 街道
    #[serde(rename = "Township")]
    pub township: Option<String>,
}

impl crate::FlatSerialize for MetaQueryRespAddress {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.address_line,
            &format!("{}.AddressLine", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.city, &format!("{}.City", name), params);
        crate::FlatSerialize::flat_serialize(&self.district, &format!("{}.District", name), params);
        crate::FlatSerialize::flat_serialize(&self.language, &format!("{}.Language", name), params);
        crate::FlatSerialize::flat_serialize(&self.province, &format!("{}.Province", name), params);
        crate::FlatSerialize::flat_serialize(&self.township, &format!("{}.Township", name), params);
    }
}

/// 地址信息列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileAddresses {
    /// 地址信息
    #[serde(rename = "Address")]
    pub address: Option<MetaQueryRespAddress>,
}

impl crate::FlatSerialize for MetaQueryFileAddresses {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.address, &format!("{}.Address", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespFileInsightsVideo {
    #[serde(rename = "Caption")]
    pub caption: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

impl crate::FlatSerialize for MetaQueryRespFileInsightsVideo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.caption, &format!("{}.Caption", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.Description", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespFileInsightsImage {
    #[serde(rename = "Caption")]
    pub caption: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

impl crate::FlatSerialize for MetaQueryRespFileInsightsImage {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.caption, &format!("{}.Caption", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.Description", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespFileInsights {
    #[serde(rename = "Video")]
    pub video: Option<MetaQueryRespFileInsightsVideo>,
    #[serde(rename = "Image")]
    pub image: Option<MetaQueryRespFileInsightsImage>,
}

impl crate::FlatSerialize for MetaQueryRespFileInsights {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.video, &format!("{}.Video", name), params);
        crate::FlatSerialize::flat_serialize(&self.image, &format!("{}.Image", name), params);
    }
}

/// A short description of struct
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFile {
    /// 对象的完整路径
    #[serde(rename = "Filename")]
    pub filename: Option<String>,
    /// 对象大小
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    /// 最近一次修改时间
    #[serde(rename = "FileModifiedTime")]
    pub file_modified_time: Option<String>,
    /// 对象的类型
    #[serde(rename = "OSSObjectType")]
    pub oss_object_type: Option<String>,
    /// 对象的存储类型
    #[serde(rename = "OSSStorageClass")]
    pub oss_storage_class: Option<String>,
    /// 对象的访问权限
    #[serde(rename = "ObjectACL")]
    pub object_acl: Option<String>,
    /// 对象的ETAG
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    /// 对象的CRC64校验值
    #[serde(rename = "OSSCRC64")]
    pub osscrc64: Option<String>,
    /// 创建对象时的服务端加密密钥
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<String>,
    /// 创建对象时的服务端加密算法
    #[serde(rename = "ServerSideEncryptionCustomerAlgorithm")]
    pub server_side_encryption_customer_algorithm: Option<String>,
    /// 对象的标签个数
    #[serde(rename = "OSSTaggingCount")]
    pub oss_tagging_count: Option<i64>,
    /// 对象的标签列表
    #[serde(rename = "OSSTagging")]
    pub oss_tagging: Option<MetaQueryFileOssTagging>,
    /// 对象的自定义元数据列表
    #[serde(rename = "OSSUserMeta")]
    pub oss_user_meta: Option<MetaQueryFileOssUserMeta>,
    /// Object完整路径
    #[serde(rename = "URI")]
    pub uri: Option<String>,
    /// 设备记录的照片或视频的拍摄时间
    #[serde(rename = "ProduceTime")]
    pub produce_time: Option<String>,
    /// MIME类型
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,
    /// 多媒体类型
    #[serde(rename = "MediaType")]
    pub media_type: Option<String>,
    /// 经纬度信息
    #[serde(rename = "LatLong")]
    pub lat_long: Option<String>,
    /// 文件标题
    #[serde(rename = "Title")]
    pub title: Option<String>,
    /// 文件过期时间
    #[serde(rename = "OSSExpiration")]
    pub oss_expiration: Option<String>,
    /// 允许的跨域请求的来源
    #[serde(rename = "AccessControlAllowOrigin")]
    pub access_control_allow_origin: Option<String>,
    /// 跨域请求中用到的方法
    #[serde(rename = "AccessControlRequestMethod")]
    pub access_control_request_method: Option<String>,
    /// Object的加密算法
    #[serde(rename = "ServerSideDataEncryption")]
    pub server_side_data_encryption: Option<String>,
    /// KMS托管的用户主密钥
    #[serde(rename = "ServerSideEncryptionKeyId")]
    pub server_side_encryption_key_id: Option<String>,
    /// Object被下载时网页的缓存行为
    #[serde(rename = "CacheControl")]
    pub cache_control: Option<String>,
    /// Object被下载时的名称
    #[serde(rename = "ContentDisposition")]
    pub content_disposition: Option<String>,
    /// Object被下载时的内容编码格式
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: Option<String>,
    /// Object内容使用的语言
    #[serde(rename = "ContentLanguage")]
    pub content_language: Option<String>,
    /// 图片高度，单位为像素（px）
    #[serde(rename = "ImageHeight")]
    pub image_height: Option<i64>,
    /// 图片宽度，单位为像素（px）
    #[serde(rename = "ImageWidth")]
    pub image_width: Option<i64>,
    /// 视频画面宽度，单位为像素（px）
    #[serde(rename = "VideoWidth")]
    pub video_width: Option<i64>,
    /// 视频画面高度，单位为像素（px）
    #[serde(rename = "VideoHeight")]
    pub video_height: Option<i64>,
    /// 码率，单位为比特每秒（bit/s）
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<i64>,
    /// 艺术家
    #[serde(rename = "Artist")]
    pub artist: Option<String>,
    /// 演唱者
    #[serde(rename = "AlbumArtist")]
    pub album_artist: Option<String>,
    /// 作曲家
    #[serde(rename = "Composer")]
    pub composer: Option<String>,
    /// 演奏者
    #[serde(rename = "Performer")]
    pub performer: Option<String>,
    /// 专辑
    #[serde(rename = "Album")]
    pub album: Option<String>,
    /// 视频的总时长。单位秒
    #[serde(rename = "Duration")]
    pub duration: Option<f64>,
    /// 视频流列表
    #[serde(rename = "VideoStreams")]
    pub video_streams: Option<MetaQueryFileVideoStreams>,
    /// 音频流列表
    #[serde(rename = "AudioStreams")]
    pub audio_streams: Option<MetaQueryFileAudioStreams>,
    /// 字幕流列表
    #[serde(rename = "Subtitles")]
    pub subtitles: Option<MetaQueryFileSubtitles>,
    /// 地址信息列表
    #[serde(rename = "Addresses")]
    pub addresses: Option<MetaQueryFileAddresses>,
    /// 保存文件的描述信息
    #[serde(rename = "Insights")]
    pub insights: Option<MetaQueryRespFileInsights>,
}

impl crate::FlatSerialize for MetaQueryFile {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.filename, &format!("{}.Filename", name), params);
        crate::FlatSerialize::flat_serialize(&self.size, &format!("{}.Size", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.file_modified_time,
            &format!("{}.FileModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_object_type,
            &format!("{}.OSSObjectType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_storage_class,
            &format!("{}.OSSStorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.object_acl,
            &format!("{}.ObjectACL", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(&self.osscrc64, &format!("{}.OSSCRC64", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.server_side_encryption,
            &format!("{}.ServerSideEncryption", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.server_side_encryption_customer_algorithm,
            &format!("{}.ServerSideEncryptionCustomerAlgorithm", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_tagging_count,
            &format!("{}.OSSTaggingCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_tagging,
            &format!("{}.OSSTagging", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_user_meta,
            &format!("{}.OSSUserMeta", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.uri, &format!("{}.URI", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.produce_time,
            &format!("{}.ProduceTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.content_type,
            &format!("{}.ContentType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.media_type,
            &format!("{}.MediaType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.lat_long, &format!("{}.LatLong", name), params);
        crate::FlatSerialize::flat_serialize(&self.title, &format!("{}.Title", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.oss_expiration,
            &format!("{}.OSSExpiration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_control_allow_origin,
            &format!("{}.AccessControlAllowOrigin", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_control_request_method,
            &format!("{}.AccessControlRequestMethod", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.server_side_data_encryption,
            &format!("{}.ServerSideDataEncryption", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.server_side_encryption_key_id,
            &format!("{}.ServerSideEncryptionKeyId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cache_control,
            &format!("{}.CacheControl", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.content_disposition,
            &format!("{}.ContentDisposition", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.content_encoding,
            &format!("{}.ContentEncoding", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.content_language,
            &format!("{}.ContentLanguage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.image_height,
            &format!("{}.ImageHeight", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.image_width,
            &format!("{}.ImageWidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.video_width,
            &format!("{}.VideoWidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.video_height,
            &format!("{}.VideoHeight", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.bitrate, &format!("{}.Bitrate", name), params);
        crate::FlatSerialize::flat_serialize(&self.artist, &format!("{}.Artist", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.album_artist,
            &format!("{}.AlbumArtist", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.composer, &format!("{}.Composer", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.performer,
            &format!("{}.Performer", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.album, &format!("{}.Album", name), params);
        crate::FlatSerialize::flat_serialize(&self.duration, &format!("{}.Duration", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.video_streams,
            &format!("{}.VideoStreams", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.audio_streams,
            &format!("{}.AudioStreams", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.subtitles,
            &format!("{}.Subtitles", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.addresses,
            &format!("{}.Addresses", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.insights, &format!("{}.Insights", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryOpenRequestFilters {
    #[serde(rename = "Filter")]
    #[serde(default)]
    pub filter: Vec<String>,
}

impl crate::FlatSerialize for MetaQueryOpenRequestFilters {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.filter, &format!("{}.Filter", name), params);
    }
}

/// 为开启元数据管理功能的请求体
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryOpenRequest {
    #[serde(rename = "Filters")]
    pub filters: Option<MetaQueryOpenRequestFilters>,
}

impl crate::FlatSerialize for MetaQueryOpenRequest {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.filters, &format!("{}.Filters", name), params);
    }
}

/// Object信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespFiles {
    /// 保存单个Object信息的容器。
    #[serde(rename = "File")]
    #[serde(default)]
    pub file: Vec<MetaQueryFile>,
}

impl crate::FlatSerialize for MetaQueryRespFiles {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.file, &format!("{}.File", name), params);
    }
}

/// 聚合操作结果信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespAggregations {
    /// 聚合操作结果列表
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    pub aggregation: Vec<MetaQueryAggregationsResult>,
}

impl crate::FlatSerialize for MetaQueryRespAggregations {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.aggregation,
            &format!("{}.Aggregation", name),
            params,
        );
    }
}

/// 数据索引查询响应体
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryResp {
    /// 当Object总数大于设置的MaxResults时，用于翻页的token。  下一次列出Object信息时以此值为NextToken，将未返回的结果返回。  当Object未全部返回时，此参数才有值。
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
    /// Object信息的容器。
    #[serde(rename = "Files")]
    pub files: Option<MetaQueryRespFiles>,
    /// 聚合操作结果信息的容器
    #[serde(rename = "Aggregations")]
    pub aggregations: Option<MetaQueryRespAggregations>,
}

impl crate::FlatSerialize for MetaQueryResp {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.NextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.files, &format!("{}.Files", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.aggregations,
            &format!("{}.Aggregations", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for MetaQueryResp {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotificationConfigurationTopicConfigurationItem {
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

impl crate::FlatSerialize for NotificationConfigurationTopicConfigurationItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.Id", name), params);
    }
}

/// BucketNotification 配置信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotificationConfiguration {
    #[serde(rename = "TopicConfiguration")]
    #[serde(default)]
    pub topic_configuration: Vec<NotificationConfigurationTopicConfigurationItem>,
}

impl crate::FlatSerialize for NotificationConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.topic_configuration,
            &format!("{}.TopicConfiguration", name),
            params,
        );
    }
}

/// Object哈希算法配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectHashConfiguration {
    /// 对象哈希算法
    #[serde(rename = "ObjectHashFunction")]
    pub object_hash_function: Option<String>,
    /// 访问对象时或是否在响应头显示对象哈希值
    #[serde(rename = "DisplayObjectHash")]
    pub display_object_hash: Option<bool>,
}

impl crate::FlatSerialize for ObjectHashConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.object_hash_function,
            &format!("{}.ObjectHashFunction", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.display_object_hash,
            &format!("{}.DisplayObjectHash", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectLinkInfoPartItem {
    #[serde(rename = "PartNumber")]
    pub part_number: Option<i64>,
    #[serde(rename = "PartName")]
    pub part_name: Option<String>,
}

impl crate::FlatSerialize for ObjectLinkInfoPartItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.part_number,
            &format!("{}.PartNumber", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.part_name,
            &format!("{}.PartName", name),
            params,
        );
    }
}

/// ObjectLink类型文件信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectLinkInfo {
    #[serde(rename = "Part")]
    #[serde(default)]
    pub part: Vec<ObjectLinkInfoPartItem>,
}

impl crate::FlatSerialize for ObjectLinkInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.part, &format!("{}.Part", name), params);
    }
}

/// 返回的文件元信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectSummary {
    /// Object的Key。
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// Object最后被修改的时间。
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
    /// ETag (Entity Tag) 在每个Object生成时创建，用于标识一个Object的内容。
    ///
    /// - 对于PutObject请求创建的Object，ETag值是其内容的MD5值。
    ///
    /// - 对于其他方式创建的Object，ETag值是基于一定计算规则生成的唯一值，但不是其内容的MD5值。
    ///
    /// - ETag值可以用于检查Object内容是否发生变化。不建议使用ETag值作为Object内容的MD5校验数据完整性的依据。
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    /// Object的类型，包含以下三种：
    ///
    /// - 通过简单上传生成的Object类型为Normal。
    ///
    /// - 通过分片上传生成的Object类型为Multipart。
    ///
    /// - 通过追加上传生成的Object类型为Appendable，且仅支持在Appendable类型的Object后直接追加内容。
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
    /// 返回Object大小，单位为字节。
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    /// Object的存储类型。
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    /// 保存Bucket拥有者信息的容器。
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    /// Object的解冻状态
    #[serde(rename = "RestoreInfo")]
    pub restore_info: Option<String>,
    /// Object被生命周期转为冷归档或者深度冷归档的时间
    #[serde(rename = "TransitionTime")]
    pub transition_time: Option<String>,
}

impl crate::FlatSerialize for ObjectSummary {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
        crate::FlatSerialize::flat_serialize(&self.size, &format!("{}.Size", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.restore_info,
            &format!("{}.RestoreInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.transition_time,
            &format!("{}.TransitionTime", name),
            params,
        );
    }
}

/// 保存通过对象FC接入点发起请求信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationAllowedFeatures {
    /// 函数计算支持的功能特性列表
    #[serde(rename = "AllowedFeature")]
    #[serde(default)]
    pub allowed_feature: Vec<String>,
}

impl crate::FlatSerialize for ObjectProcessConfigurationAllowedFeatures {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.allowed_feature,
            &format!("{}.AllowedFeature", name),
            params,
        );
    }
}

/// 保存操作信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemActions
{
    /// 保存操作信息的列表
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: Vec<String>,
}

impl crate::FlatSerialize
    for ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemActions
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.action, &format!("{}.Action", name), params);
    }
}

/// 保存函数计算信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationFunctionCompute
{
    /// 函数计算用于访问您在其他云产品中的资源的角色ARN
    #[serde(rename = "FunctionAssumeRoleArn")]
    pub function_assume_role_arn: Option<String>,
    /// 函数ARN
    #[serde(rename = "FunctionArn")]
    pub function_arn: Option<String>,
}

impl crate::FlatSerialize for ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationFunctionCompute {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.function_assume_role_arn, &format!("{}.FunctionAssumeRoleArn", name), params);
        crate::FlatSerialize::flat_serialize(&self.function_arn, &format!("{}.FunctionArn", name), params);
    }
}

/// 保存自定义转发请求头列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeaturesCustomForwardHeaders
{
    /// 自定义转发请求头
    #[serde(rename = "CustomForwardHeader")]
    #[serde(default)]
    pub custom_forward_header: Vec<String>,
}

impl crate::FlatSerialize for ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeaturesCustomForwardHeaders {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.custom_forward_header, &format!("{}.CustomForwardHeader", name), params);
    }
}

/// 保存额外特性配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeatures {
    /// 保存自定义转发请求头列表
    #[serde(rename = "CustomForwardHeaders")]
    pub custom_forward_headers: Option<ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeaturesCustomForwardHeaders>,
}

impl crate::FlatSerialize for ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeatures {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.custom_forward_headers, &format!("{}.CustomForwardHeaders", name), params);
    }
}

/// 保存转换信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformation {
    /// 保存函数计算信息的容器
    #[serde(rename = "FunctionCompute")]
    pub function_compute: Option<ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationFunctionCompute>,
    /// 保存额外特性配置
    #[serde(rename = "AdditionalFeatures")]
    pub additional_features: Option<ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeatures>,
}

impl crate::FlatSerialize for ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformation {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.function_compute, &format!("{}.FunctionCompute", name), params);
        crate::FlatSerialize::flat_serialize(&self.additional_features, &format!("{}.AdditionalFeatures", name), params);
    }
}

/// 保存转换配置信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItem {
    /// 保存操作信息的容器
    #[serde(rename = "Actions")]
    pub actions: Option<ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemActions>,
    /// 保存转换信息的容器
    #[serde(rename = "ContentTransformation")]
    pub content_transformation: Option<ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformation>,
}

impl crate::FlatSerialize
    for ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItem
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.actions, &format!("{}.Actions", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.content_transformation,
            &format!("{}.ContentTransformation", name),
            params,
        );
    }
}

/// 保存转换配置信息的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurations {
    /// 保存转换配置信息的列表
    #[serde(rename = "TransformationConfiguration")]
    #[serde(default)]
    pub transformation_configuration:
        Vec<ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItem>,
}

impl crate::FlatSerialize for ObjectProcessConfigurationTransformationConfigurations {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.transformation_configuration,
            &format!("{}.TransformationConfiguration", name),
            params,
        );
    }
}

/// 对象FC接入点的配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfiguration {
    /// 保存通过对象FC接入点发起请求信息的容器。
    #[serde(rename = "AllowedFeatures")]
    pub allowed_features: Option<ObjectProcessConfigurationAllowedFeatures>,
    /// 保存转换配置信息的容器
    #[serde(rename = "TransformationConfigurations")]
    pub transformation_configurations:
        Option<ObjectProcessConfigurationTransformationConfigurations>,
}

impl crate::FlatSerialize for ObjectProcessConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.allowed_features,
            &format!("{}.AllowedFeatures", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.transformation_configurations,
            &format!("{}.TransformationConfigurations", name),
            params,
        );
    }
}

/// 保存除删除标记以外的Object版本的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectVersion {
    /// Object的名称。
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// Object的版本ID。
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
    /// Object是否为当前版本。
    /// 取值：
    ///
    /// - true：Object为当前版本。
    ///
    /// - false：Object为非当前版本。
    #[serde(rename = "IsLatest")]
    pub is_latest: Option<bool>,
    /// Object最后被修改的时间。
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
    /// 每个Object生成时创建的ETag ，用于标识Object的内容。
    ///   - 对于PutObject请求创建的Object，ETag值是其内容的MD5值。
    ///   - 对于其他方式创建的Object，ETag值是基于一定计算规则生成的唯一值，但不是其内容的MD5值。
    ///
    /// > ETag值仅用于检查Object内容是否发生变化。不建议使用ETag值作为Object内容的MD5数据完整性校验的依据。
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    /// Object的字节数。
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    /// Object的存储类型。
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    /// 保存Bucket拥有者信息的容器。
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    /// Object版本的解冻状态
    #[serde(rename = "RestoreInfo")]
    pub restore_info: Option<String>,
    /// Object版本被生命周期转为冷归档或者深度冷归档的时间
    #[serde(rename = "TransitionTime")]
    pub transition_time: Option<String>,
}

impl crate::FlatSerialize for ObjectVersion {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.version_id,
            &format!("{}.VersionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_latest,
            &format!("{}.IsLatest", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(&self.size, &format!("{}.Size", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.storage_class,
            &format!("{}.StorageClass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.restore_info,
            &format!("{}.RestoreInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.transition_time,
            &format!("{}.TransitionTime", name),
            params,
        );
    }
}

/// 保存Select请求的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OutputSerialization {
    /// 输出CSV的格式参数。
    #[serde(rename = "CSV")]
    pub csv: Option<CSVOutput>,
    /// 输出JSON的格式参数。
    #[serde(rename = "JSON")]
    pub json: Option<JSONOutput>,
    /// 指定返回结果中包含CSV所有列的位置（可选）。<br>默认值：false
    ///  <br>仅在select语句里出现的列会有值，不出现的列则为空，返回结果中每一行的数据按照CSV列的顺序从低到高排列。
    /// 例如以下语句：<br>
    /// `select _5, _1 from ossobject.`
    /// <br>如果KeepAllColumns = true，假设一共有6列数据，则返回以下数据：
    /// <br> `Value of 1st column,,,,Value of 5th column,\n`
    #[serde(rename = "KeepAllColumns")]
    pub keep_all_columns: Option<bool>,
    /// 在返回结果开头输出CSV头信息。
    /// <br>默认值：false
    #[serde(rename = "OutputHeader")]
    pub output_header: Option<bool>,
    /// 指定输出数据为纯数据。
    ///
    /// - 您在请求中指定OutputRawData值时，OSS服务端会按照请求中的要求返回数据。
    ///
    /// - 您在请求中不指定OutputRawData值时，OSS服务端会自动选择一种格式返回。
    ///
    /// - 当您显式地指定OutputRawData为True时，如果该SQL长时间内没有返回数据，则HTTP请求可能因没有数据返回而超时。
    #[serde(rename = "OutputRawData")]
    pub output_raw_data: Option<bool>,
    /// 在每个Frame中会有一个32位的CRC32校验值。客户端可以计算相应payload的CRC32值进行数据完整性校验。
    #[serde(rename = "EnablePayloadCrc")]
    pub enable_payload_crc: Option<bool>,
}

impl crate::FlatSerialize for OutputSerialization {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.csv, &format!("{}.CSV", name), params);
        crate::FlatSerialize::flat_serialize(&self.json, &format!("{}.JSON", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.keep_all_columns,
            &format!("{}.KeepAllColumns", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_header,
            &format!("{}.OutputHeader", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_raw_data,
            &format!("{}.OutputRawData", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_payload_crc,
            &format!("{}.EnablePayloadCrc", name),
            params,
        );
    }
}

/// 保存规则生效的账号与角色列表的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverwriteConfigurationRuleItemPrincipals {
    /// 规则生效的账号与角色列表。用法与Bucket Policy的Principal相仿，支持主账号、子账号或角色的输入。如果为空或不配置，则表明对于满足前后缀条件的Object，一律不允许覆盖写。
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: Vec<String>,
}

impl crate::FlatSerialize for OverwriteConfigurationRuleItemPrincipals {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.principal,
            &format!("{}.Principal", name),
            params,
        );
    }
}

/// 单个Bucket最多支持配置100条规则。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverwriteConfigurationRuleItem {
    /// 规则的唯一标识符。如果不填写，则会随机生成一个UUID来填充；如果填写，则必须为唯一值，即不同的Rule中不能设置相同的ID。
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// 操作类型。当前只支持设置forbid（禁止覆盖写）。
    #[serde(rename = "Action")]
    pub action: Option<String>,
    /// Object名称的前缀，用于筛选需要处理的Object。最大长度为1023个字符。单个Rule中最多一个Prefix。前后缀不支持正则表达。
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    /// Object名称的后缀，用于筛选需要处理的Object。最大长度为1023个字符。单个Rule中最多一个Suffix。前后缀不支持正则表达。
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    /// 保存规则生效的账号与角色列表的容器
    #[serde(rename = "Principals")]
    pub principals: Option<OverwriteConfigurationRuleItemPrincipals>,
}

impl crate::FlatSerialize for OverwriteConfigurationRuleItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
        crate::FlatSerialize::flat_serialize(&self.action, &format!("{}.Action", name), params);
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.suffix, &format!("{}.Suffix", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.principals,
            &format!("{}.Principals", name),
            params,
        );
    }
}

/// 禁止覆盖写的配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverwriteConfiguration {
    /// 保存单条禁止覆盖写规则的容器
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: Vec<OverwriteConfigurationRuleItem>,
}

impl crate::FlatSerialize for OverwriteConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

impl crate::ToCodeMessage for OverwriteConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 保存已上传Part信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Part {
    /// Part成功上传后，OSS返回的ETag值。
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    /// Part数目。
    #[serde(rename = "PartNumber")]
    pub part_number: Option<i64>,
    /// 已上传Part大小。
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    /// Part上传的时间。
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,
}

impl crate::FlatSerialize for Part {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.part_number,
            &format!("{}.PartNumber", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.size, &format!("{}.Size", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PromoteDataLakeCacheReqObject {
    #[serde(rename = "ObjectName")]
    pub object_name: Option<String>,
    #[serde(rename = "Range")]
    pub range: Option<String>,
}

impl crate::FlatSerialize for PromoteDataLakeCacheReqObject {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.object_name,
            &format!("{}.ObjectName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.range, &format!("{}.Range", name), params);
    }
}

/// 加速器缓存管理请求
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PromoteDataLakeCacheReq {
    #[serde(rename = "Object")]
    pub object: Option<PromoteDataLakeCacheReqObject>,
}

impl crate::FlatSerialize for PromoteDataLakeCacheReq {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.object, &format!("{}.Object", name), params);
    }
}

/// 图片处理频道配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutChannelConfiguration {
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "OrigPicForbidden")]
    pub orig_pic_forbidden: Option<bool>,
    #[serde(rename = "UseSrcFormat")]
    pub use_src_format: Option<bool>,
    #[serde(rename = "SetAttachName")]
    pub set_attach_name: Option<bool>,
    #[serde(rename = "UseStyleOnly")]
    pub use_style_only: Option<bool>,
    #[serde(rename = "AutoSetContentType")]
    pub auto_set_content_type: Option<bool>,
    #[serde(rename = "Default404Pic")]
    pub default404_pic: Option<String>,
    #[serde(rename = "StyleDelimiters")]
    pub style_delimiters: Option<String>,
}

impl crate::FlatSerialize for PutChannelConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.orig_pic_forbidden,
            &format!("{}.OrigPicForbidden", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.use_src_format,
            &format!("{}.UseSrcFormat", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.set_attach_name,
            &format!("{}.SetAttachName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.use_style_only,
            &format!("{}.UseStyleOnly", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.auto_set_content_type,
            &format!("{}.AutoSetContentType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.default404_pic,
            &format!("{}.Default404Pic", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.style_delimiters,
            &format!("{}.StyleDelimiters", name),
            params,
        );
    }
}

/// 保存Prefix的容器，每个复制规则中，最多能指定10个Prefix。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationPrefixSet {
    /// 设置待复制Object的Prefix。只有匹配该Prefix的Object才被复制到目标Bucket。
    ///   - Prefix最大长度为1023个字符。
    ///   - 如果配置了Prefix，则新写入的数据和历史数据的同步都会遵循Prefix指定的规则。
    ///
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: Vec<String>,
}

impl crate::FlatSerialize for ReplicationPrefixSet {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
    }
}

/// 保存目标Bucket信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationDestination {
    /// 指定数据要复制到的目标Bucket。
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    /// 目标Bucket所处的地域。
    #[serde(rename = "Location")]
    pub location: Option<String>,
    /// 指定数据复制时使用的数据传输链路。
    /// 取值：
    ///   - internal（默认值）：OSS默认传输链路。
    ///   - oss_acc：传输加速链路。只有创建跨区域复制规则时才能使用传输加速链路。
    #[serde(rename = "TransferType")]
    pub transfer_type: Option<ReplicationDestinationTransferType>,
}

impl crate::FlatSerialize for ReplicationDestination {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.transfer_type,
            &format!("{}.TransferType", name),
            params,
        );
    }
}

/// 用于筛选使用SSE-KMS加密对象的容器。如果在数据复制规则中指定了SourceSelectionCriteria，则必须指定该元素。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationSourceSelectionCriteriaSseKmsEncryptedObjects {
    /// 指定OSS是否复制通过SSE-KMS加密创建的对象。取值范围如下：
    ///   - Enabled：表示复制通过SSE-KMS加密创建的对象。
    ///   - Disabled：表示不复制通过SSE-KMS加密创建的对象。
    #[serde(rename = "Status")]
    pub status: Option<ReplicationSourceSelectionCriteriaSseKmsEncryptedObjectsStatus>,
}

impl crate::FlatSerialize for ReplicationSourceSelectionCriteriaSseKmsEncryptedObjects {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

/// 用于标识要复制的源对象的其他筛选条件的容器。当前OSS仅支持针对SSE-KMS加密的源对象指定筛选条件。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationSourceSelectionCriteria {
    /// 用于筛选使用SSE-KMS加密对象的容器。如果在数据复制规则中指定了SourceSelectionCriteria，则必须指定该元素。
    #[serde(rename = "SseKmsEncryptedObjects")]
    pub sse_kms_encrypted_objects: Option<ReplicationSourceSelectionCriteriaSseKmsEncryptedObjects>,
}

impl crate::FlatSerialize for ReplicationSourceSelectionCriteria {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.sse_kms_encrypted_objects,
            &format!("{}.SseKmsEncryptedObjects", name),
            params,
        );
    }
}

/// 目标对象加密配置。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationEncryptionConfiguration {
    /// 指定SSE-KMS密钥ID。如果指定复制通过SSE-KMS加密创建的对象，则必须指定该元素。
    #[serde(rename = "ReplicaKmsKeyID")]
    pub replica_kms_key_id: Option<String>,
}

impl crate::FlatSerialize for ReplicationEncryptionConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.replica_kms_key_id,
            &format!("{}.ReplicaKmsKeyID", name),
            params,
        );
    }
}

/// 数据复制时间控制功能的状态
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RTC {
    /// 是否开启复制时间控制（RTC）功能
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

impl crate::FlatSerialize for RTC {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

/// 数据复制规则
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutReplicationRule {
    /// 数据复制规则的唯一标识
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// 要复制的目标Object名称前缀配置容器
    #[serde(rename = "PrefixSet")]
    pub prefix_set: Option<ReplicationPrefixSet>,
    /// 指定可以被复制到目标Bucket的操作。如果配置了Action，则新写入的数据和历史数据的同步都会遵循Action指定的复制操作。  Action允许以下操作类型，您可以指定一项或多项。  取值：  ALL（默认值）：表示PUT、DELETE、ABORT操作均会被同步到目标Bucket。  PUT：表示被同步到目标Bucket的写入操作，包括PutObject、PostObject、AppendObject、CopyObject、PutObjectACL、InitiateMultipartUpload、UploadPart、UploadPartCopy、CompleteMultipartUpload。
    #[serde(rename = "Action")]
    pub action: Option<String>,
    /// 保存目的Bucket信息的容器
    #[serde(rename = "Destination")]
    pub destination: Option<ReplicationDestination>,
    /// 是否复制历史数据
    #[serde(rename = "HistoricalObjectReplication")]
    pub historical_object_replication: Option<PutReplicationRuleHistoricalObjectReplication>,
    /// 授权OSS使用哪个角色来进行数据复制。
    #[serde(rename = "SyncRole")]
    pub sync_role: Option<String>,
    /// 用于标识要复制的源对象的其他筛选条件的容器。
    #[serde(rename = "SourceSelectionCriteria")]
    pub source_selection_criteria: Option<ReplicationSourceSelectionCriteria>,
    /// 目标对象加密配置。
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<ReplicationEncryptionConfiguration>,
    /// 保存RTC配置规则的容器。
    #[serde(rename = "RTC")]
    pub rtc: Option<RTC>,
}

impl crate::FlatSerialize for PutReplicationRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.prefix_set,
            &format!("{}.PrefixSet", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.action, &format!("{}.Action", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.destination,
            &format!("{}.Destination", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.historical_object_replication,
            &format!("{}.HistoricalObjectReplication", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.sync_role,
            &format!("{}.SyncRole", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_selection_criteria,
            &format!("{}.SourceSelectionCriteria", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.encryption_configuration,
            &format!("{}.EncryptionConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.rtc, &format!("{}.RTC", name), params);
    }
}

/// 带有备注信息的QoS配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QoSConfigurationWithRemark {
    /// 总上行带宽，单位Gbps
    #[serde(rename = "TotalUploadBandwidth")]
    pub total_upload_bandwidth: Option<i64>,
    /// 内网上行带宽，单位Gbps
    #[serde(rename = "IntranetUploadBandwidth")]
    pub intranet_upload_bandwidth: Option<i64>,
    /// 公网上行带宽，单位Gbps
    #[serde(rename = "ExtranetUploadBandwidth")]
    pub extranet_upload_bandwidth: Option<i64>,
    /// 总下行带宽，单位Gbps
    #[serde(rename = "TotalDownloadBandwidth")]
    pub total_download_bandwidth: Option<i64>,
    /// 内网下行带宽，单位Gbps
    #[serde(rename = "IntranetDownloadBandwidth")]
    pub intranet_download_bandwidth: Option<i64>,
    /// 公网下行带宽，单位Gbps
    #[serde(rename = "ExtranetDownloadBandwidth")]
    pub extranet_download_bandwidth: Option<i64>,
    /// 总QPS
    #[serde(rename = "TotalQps")]
    pub total_qps: Option<i64>,
    /// 内网QPS
    #[serde(rename = "IntranetQps")]
    pub intranet_qps: Option<i64>,
    /// 公网QPS
    #[serde(rename = "ExtranetQps")]
    pub extranet_qps: Option<i64>,
    /// 备注
    #[serde(rename = "Remark")]
    pub remark: Option<i64>,
}

impl crate::FlatSerialize for QoSConfigurationWithRemark {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.total_upload_bandwidth,
            &format!("{}.TotalUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_upload_bandwidth,
            &format!("{}.IntranetUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_upload_bandwidth,
            &format!("{}.ExtranetUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_download_bandwidth,
            &format!("{}.TotalDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_download_bandwidth,
            &format!("{}.IntranetDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_download_bandwidth,
            &format!("{}.ExtranetDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_qps,
            &format!("{}.TotalQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_qps,
            &format!("{}.IntranetQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_qps,
            &format!("{}.ExtranetQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.remark, &format!("{}.Remark", name), params);
    }
}

/// 保存Referer访问白名单的容器。
/// > **说明**PutBucketReferer为覆盖语义，即RefererList中的新指定的白名单列表将覆盖已配置的白名单列表。当您上传的RefererList为空时（即不包含Referer请求元素），此操作将清空RefererList中已配置的白名单列表。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RefererConfigurationRefererList {
    /// 指定一条Referer访问白名单。
    #[serde(rename = "Referer")]
    #[serde(default)]
    pub referer: Vec<String>,
}

impl crate::FlatSerialize for RefererConfigurationRefererList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.referer, &format!("{}.Referer", name), params);
    }
}

/// 保存Referer访问黑名单
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RefererConfigurationRefererBlacklist {
    /// 指定一条Referer访问黑名单
    #[serde(rename = "Referer")]
    #[serde(default)]
    pub referer: Vec<String>,
}

impl crate::FlatSerialize for RefererConfigurationRefererBlacklist {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.referer, &format!("{}.Referer", name), params);
    }
}

/// 保存Referer配置内容的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RefererConfiguration {
    /// 指定是否允许Referer字段为空的请求访问OSS。取值如下：
    ///
    /// - true（默认值）：允许Referer字段为空的请求访问OSS。
    /// - false：不允许Referer字段为空的请求访问OSS。
    #[serde(rename = "AllowEmptyReferer")]
    pub allow_empty_referer: bool,
    /// 是否截断QueryString。
    ///
    /// - true（默认值）：截断QueryString。
    ///
    /// - false：不截断QueryString。
    #[serde(rename = "AllowTruncateQueryString")]
    pub allow_truncate_query_string: Option<bool>,
    /// 匹配时是否截断路径部分（即只匹配protocol://host/部分）。取值：
    ///
    /// - true：截断URL中包括Path在内的后续所有部分。
    ///
    /// - false：不截断URL中包括Path在内的后续所有部分。
    #[serde(rename = "TruncatePath")]
    pub truncate_path: Option<bool>,
    /// 保存Referer访问白名单的容器。
    /// > **说明**PutBucketReferer为覆盖语义，即RefererList中的新指定的白名单列表将覆盖已配置的白名单列表。当您上传的RefererList为空时（即不包含Referer请求元素），此操作将清空RefererList中已配置的白名单列表。
    #[serde(rename = "RefererList")]
    pub referer_list: RefererConfigurationRefererList,
    /// 保存Referer访问黑名单
    #[serde(rename = "RefererBlacklist")]
    pub referer_blacklist: Option<RefererConfigurationRefererBlacklist>,
}

impl crate::FlatSerialize for RefererConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.allow_empty_referer,
            &format!("{}.AllowEmptyReferer", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.allow_truncate_query_string,
            &format!("{}.AllowTruncateQueryString", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.truncate_path,
            &format!("{}.TruncatePath", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.referer_list,
            &format!("{}.RefererList", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.referer_blacklist,
            &format!("{}.RefererBlacklist", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for RefererConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 地域信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegionInfo {
    /// 地域ID。
    #[serde(rename = "Region")]
    pub region: Option<String>,
    /// 地域对应的外网Endpoint。
    #[serde(rename = "InternetEndpoint")]
    pub internet_endpoint: Option<String>,
    /// 地域对应的内网Endpoint。
    #[serde(rename = "InternalEndpoint")]
    pub internal_endpoint: Option<String>,
    /// 地域对应的传输加速Endpoint。取值固定为oss-accelerate.aliyuncs.com。
    #[serde(rename = "AccelerateEndpoint")]
    pub accelerate_endpoint: Option<String>,
}

impl crate::FlatSerialize for RegionInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.internet_endpoint,
            &format!("{}.InternetEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.internal_endpoint,
            &format!("{}.InternalEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.accelerate_endpoint,
            &format!("{}.AccelerateEndpoint", name),
            params,
        );
    }
}

/// 数据复制配置信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationConfiguration {
    /// 保存数据复制规则的容器。
    #[serde(rename = "Rule")]
    pub rule: Option<PutReplicationRule>,
}

impl crate::FlatSerialize for ReplicationConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

/// 保存复制进度的容器，仅当数据处于同步状态（doing）时才返回此元素。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationProgressRuleProgress {
    /// 显示已复制历史数据的百分比。仅对开启了历史数据复制的Bucket有效。
    #[serde(rename = "HistoricalObject")]
    pub historical_object: Option<String>,
    /// 显示数据复制到目标Bucket的时间点（GMT格式）。
    ///
    /// 例如Thu, 24 Sep 2015 15:39:18 GMT，表示早于该时间点写入的数据都已复制到目标Bucket。
    #[serde(rename = "NewObject")]
    pub new_object: Option<String>,
}

impl crate::FlatSerialize for ReplicationProgressRuleProgress {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.historical_object,
            &format!("{}.HistoricalObject", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.new_object,
            &format!("{}.NewObject", name),
            params,
        );
    }
}

/// 数据复制进度信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationProgressRule {
    /// rule id
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// 保存Prefix 的容器，每个复制规则中，最多能指定10个Prefix。
    #[serde(rename = "PrefixSet")]
    pub prefix_set: Option<ReplicationPrefixSet>,
    /// 表示被同步到目标Bucket的操作。
    ///
    /// Action允许以下操作类型，您可以指定一项或者多项。
    ///
    /// ALL（默认操作）：表示PUT、DELETE、ABORT操作均会被同步到目标Bucket。
    ///
    /// PUT：表示被同步到目标Bucket的写入操作，包括PutObject、PostObject、AppendObject、CopyObject、PutObjectACL、 InitiateMultipartUpload 、 UploadPart、UploadPartCopy和CompleteMultipartUpload。
    #[serde(rename = "Action")]
    pub action: Option<String>,
    /// 保存目标Bucket信息的容器。
    #[serde(rename = "Destination")]
    pub destination: Option<ReplicationDestination>,
    /// 复制状态。
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 是否复制历史数据。即开启数据复制前，是否将源Bucket中的数据复制到目标Bucket。
    ///
    /// 取值：
    ///
    /// enabled（默认值）：表示复制历史数据。
    ///
    /// disabled：表示不复制历史数据，仅复制跨区域复制规则生效后新写入的数据。
    #[serde(rename = "HistoricalObjectReplication")]
    pub historical_object_replication: Option<String>,
    /// 保存复制进度的容器，仅当数据处于同步状态（doing）时才返回此元素。
    #[serde(rename = "Progress")]
    pub progress: Option<ReplicationProgressRuleProgress>,
}

impl crate::FlatSerialize for ReplicationProgressRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.prefix_set,
            &format!("{}.PrefixSet", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.action, &format!("{}.Action", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.destination,
            &format!("{}.Destination", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.historical_object_replication,
            &format!("{}.HistoricalObjectReplication", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.progress, &format!("{}.Progress", name), params);
    }
}

/// 数据复制规则。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationRule {
    /// 请求规则对应的ID。
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// 保存前缀（Prefix）的容器。每条数据复制规则中，最多可指定10条Prefix。
    #[serde(rename = "PrefixSet")]
    pub prefix_set: Option<ReplicationPrefixSet>,
    /// 指定可以被复制到目标Bucket的操作。如果配置了Action，则新写入的数据和历史数据的同步都会遵循Action指定的复制操作。
    /// Action允许以下操作类型，您可以指定一项或多项。
    /// 取值：
    ///   - ALL（默认值）：表示PUT、DELETE、ABORT操作均会被同步到目标Bucket。
    ///   - PUT：表示被同步到目标Bucket的写入操作，包括PutObject、PostObject、AppendObject、CopyObject、PutObjectACL、InitiateMultipartUpload、UploadPart、UploadPartCopy、CompleteMultipartUpload。
    ///
    #[serde(rename = "Action")]
    pub action: Option<String>,
    /// 保存目标Bucket信息的容器。
    #[serde(rename = "Destination")]
    pub destination: Option<ReplicationDestination>,
    /// 表示复制状态。
    /// 取值：
    ///   - starting：设置数据复制规则后，OSS会为Bucket准备复制任务，此时的复制状态为starting。
    ///   - doing：当数据复制规则生效后，即数据处于同步状态时，此时的复制状态为doing。
    ///   - closing：删除数据复制规则后，OSS会自动完成清理工作，此时的复制状态为closing。
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// 指定是否复制历史数据。即开启数据复制前，是否将源Bucket中的数据复制到目标Bucket。
    /// 取值：
    ///   - enabled（默认值）：表示复制历史数据。
    ///   - disabled：表示不复制历史数据。即仅复制创建数据复制规则后新写入的数据。
    ///
    #[serde(rename = "HistoricalObjectReplication")]
    pub historical_object_replication: Option<ReplicationRuleHistoricalObjectReplication>,
    /// 授权OSS使用哪个角色来进行数据复制。如果指定使用SSE-KMS加密目标对象，则必须指定该元素。
    #[serde(rename = "SyncRole")]
    pub sync_role: Option<String>,
    /// 用于标识要复制的源对象的其他筛选条件的容器。当前OSS仅支持针对SSE-KMS加密的源对象指定筛选条件。
    #[serde(rename = "SourceSelectionCriteria")]
    pub source_selection_criteria: Option<ReplicationSourceSelectionCriteria>,
    /// 目标对象加密配置。如果指定Status为Enabled，则必须指定该元素。
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<ReplicationEncryptionConfiguration>,
    /// 保存RTC配置规则的容器。
    #[serde(rename = "RTC")]
    pub rtc: Option<RTC>,
}

impl crate::FlatSerialize for ReplicationRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.prefix_set,
            &format!("{}.PrefixSet", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.action, &format!("{}.Action", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.destination,
            &format!("{}.Destination", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.historical_object_replication,
            &format!("{}.HistoricalObjectReplication", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.sync_role,
            &format!("{}.SyncRole", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_selection_criteria,
            &format!("{}.SourceSelectionCriteria", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.encryption_configuration,
            &format!("{}.EncryptionConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.rtc, &format!("{}.RTC", name), params);
    }
}

/// 数据复制规则进度信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationRuleProgress {
    /// 复制规则对应的ID。
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// 保存Prefix 的容器，每个复制规则中，最多能指定10个Prefix。
    #[serde(rename = "PrefixSet")]
    pub prefix_set: Option<ReplicationPrefixSet>,
    /// 表示被同步到目标Bucket的操作。
    /// Action允许以下操作类型，您可以指定一项或者多项。
    ///
    /// - ALL（默认操作）：表示PUT、DELETE、ABORT操作均会被同步到目标Bucket。
    ///
    /// - PUT：表示被同步到目标Bucket的写入操作，包括PutObject、PostObject、AppendObject、CopyObject、PutObjectACL、InitiateMultipartUpload 、 UploadPart、UploadPartCopy和CompleteMultipartUpload。
    #[serde(rename = "Action")]
    pub action: Option<String>,
}

impl crate::FlatSerialize for ReplicationRuleProgress {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.prefix_set,
            &format!("{}.PrefixSet", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.action, &format!("{}.Action", name), params);
    }
}

/// 保存需要删除的数据复制规则的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationRules {
    /// 需要删除的复制规则对应的ID。规则ID可从GetBucketReplication中获取。
    #[serde(rename = "ID")]
    #[serde(default)]
    pub id: Vec<String>,
}

impl crate::FlatSerialize for ReplicationRules {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
    }
}

/// 请求付费配置的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RequestPaymentConfiguration {
    /// 指定Bucket付费类型。
    /// 取值：
    ///
    /// - BucketOwner：由Bucket拥有者付费。
    ///
    /// - Requester：由请求者付费。
    #[serde(rename = "Payer")]
    pub payer: Option<String>,
}

impl crate::FlatSerialize for RequestPaymentConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.payer, &format!("{}.Payer", name), params);
    }
}

/// 创建预留空间的配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReservedCapacityCreateConfiguration {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "Capacity")]
    pub capacity: Option<i64>,
    #[serde(rename = "DataRedundancyType")]
    pub data_redundancy_type: Option<String>,
    #[serde(rename = "Years")]
    pub years: Option<i64>,
}

impl crate::FlatSerialize for ReservedCapacityCreateConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
        crate::FlatSerialize::flat_serialize(&self.capacity, &format!("{}.Capacity", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.data_redundancy_type,
            &format!("{}.DataRedundancyType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.years, &format!("{}.Years", name), params);
    }
}

/// 创建预留空间的信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReservedCapacityRecord {
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<i64>,
    #[serde(rename = "Capacity")]
    pub capacity: Option<i64>,
    #[serde(rename = "CreateTime")]
    pub create_time: Option<i64>,
    #[serde(rename = "LastModifyTime")]
    pub last_modify_time: Option<i64>,
    #[serde(rename = "LastExpansionCapacity")]
    pub last_expansion_capacity: Option<i64>,
    #[serde(rename = "ExpansionTime")]
    pub expansion_time: Option<i64>,
    #[serde(rename = "DueTime")]
    pub due_time: Option<i64>,
    #[serde(rename = "Years")]
    pub years: Option<i64>,
    #[serde(rename = "FirstTimeEnabled")]
    pub first_time_enabled: Option<i64>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "DataRedundancyType")]
    pub data_redundancy_type: Option<String>,
}

impl crate::FlatSerialize for ReservedCapacityRecord {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
        crate::FlatSerialize::flat_serialize(&self.version, &format!("{}.Version", name), params);
        crate::FlatSerialize::flat_serialize(&self.capacity, &format!("{}.Capacity", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modify_time,
            &format!("{}.LastModifyTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_expansion_capacity,
            &format!("{}.LastExpansionCapacity", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.expansion_time,
            &format!("{}.ExpansionTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.due_time, &format!("{}.DueTime", name), params);
        crate::FlatSerialize::flat_serialize(&self.years, &format!("{}.Years", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.first_time_enabled,
            &format!("{}.FirstTimeEnabled", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.data_redundancy_type,
            &format!("{}.DataRedundancyType", name),
            params,
        );
    }
}

/// 创建预留空间的信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReservedCapacityRecordList {
    #[serde(rename = "ReservedCapacityRecord")]
    #[serde(default)]
    pub reserved_capacity_record: Vec<ReservedCapacityRecord>,
}

impl crate::FlatSerialize for ReservedCapacityRecordList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.reserved_capacity_record,
            &format!("{}.ReservedCapacityRecord", name),
            params,
        );
    }
}

/// 修改预留空间配置的请求
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReservedCapacityUpdateConfiguration {
    #[serde(rename = "Capacity")]
    pub capacity: Option<i64>,
    #[serde(rename = "Years")]
    pub years: Option<i64>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

impl crate::FlatSerialize for ReservedCapacityUpdateConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.capacity, &format!("{}.Capacity", name), params);
        crate::FlatSerialize::flat_serialize(&self.years, &format!("{}.Years", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

/// 指定规则适应的操作的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResponseHeaderConfigurationRuleItemFilters {
    /// 适用操作列表
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: Vec<String>,
}

impl crate::FlatSerialize for ResponseHeaderConfigurationRuleItemFilters {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.operation,
            &format!("{}.Operation", name),
            params,
        );
    }
}

/// 指定隐藏哪些响应头的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResponseHeaderConfigurationRuleItemHideHeaders {
    /// 隐藏的响应头列表
    #[serde(rename = "Header")]
    #[serde(default)]
    pub header: Vec<String>,
}

impl crate::FlatSerialize for ResponseHeaderConfigurationRuleItemHideHeaders {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.header, &format!("{}.Header", name), params);
    }
}

/// 保存响应头规则的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResponseHeaderConfigurationRuleItem {
    /// 规则名称
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// 指定规则适应的操作的容器
    #[serde(rename = "Filters")]
    pub filters: Option<ResponseHeaderConfigurationRuleItemFilters>,
    /// 指定隐藏哪些响应头的容器
    #[serde(rename = "HideHeaders")]
    pub hide_headers: Option<ResponseHeaderConfigurationRuleItemHideHeaders>,
}

impl crate::FlatSerialize for ResponseHeaderConfigurationRuleItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.filters, &format!("{}.Filters", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.hide_headers,
            &format!("{}.HideHeaders", name),
            params,
        );
    }
}

/// 保存响应头规则的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResponseHeaderConfiguration {
    /// 响应头规则列表
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: Vec<ResponseHeaderConfigurationRuleItem>,
}

impl crate::FlatSerialize for ResponseHeaderConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

/// 解冻优先级的容器。只有解冻冷归档类型的Object时才生效。
/// 如果不传入JobParameters节点，则解冻优先级默认为Standard。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RestoreRequestJobParameters {
    /// 解冻优先级。取值范围如下：
    ///   - 高优先级（Expedited）：表示1小时内完成解冻。
    ///   - 标准（Standard）：表示2~5小时内完成解冻。
    ///   - 批量（Bulk）：表示5~12小时内完成解冻。
    #[serde(rename = "Tier")]
    pub tier: Option<String>,
}

impl crate::FlatSerialize for RestoreRequestJobParameters {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tier, &format!("{}.Tier", name), params);
    }
}

/// 解冻请求信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RestoreRequest {
    /// 解冻的天数。取值范围为1~7天。
    #[serde(rename = "Days")]
    pub days: Option<i64>,
    /// 解冻优先级的容器。只有解冻冷归档类型的Object时才生效。
    /// 如果不传入JobParameters节点，则解冻优先级默认为Standard。
    #[serde(rename = "JobParameters")]
    pub job_parameters: Option<RestoreRequestJobParameters>,
}

impl crate::FlatSerialize for RestoreRequest {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.days, &format!("{}.Days", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.job_parameters,
            &format!("{}.JobParameters", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleConditionIncludeHeaderItem {
    /// 只有请求中包含了此Header且值满足条件，才能匹配此规则。
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 只有请求中包含了Key指定的Header且值为指定值时，才能匹配此规则。
    #[serde(rename = "Equals")]
    pub equals: Option<String>,
    /// 只有请求中包含了Key指定的Header且值以该值为开头时，才能匹配此规则。
    #[serde(rename = "StartsWith")]
    pub starts_with: Option<String>,
    /// 只有请求中包含了Key指定的Header且值以该值为结尾时，才能匹配此规则。
    #[serde(rename = "EndsWith")]
    pub ends_with: Option<String>,
}

impl crate::FlatSerialize for RoutingRuleConditionIncludeHeaderItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.equals, &format!("{}.Equals", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.starts_with,
            &format!("{}.StartsWith", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.ends_with,
            &format!("{}.EndsWith", name),
            params,
        );
    }
}

/// 匹配的条件。
/// 如果指定的项都满足，则执行此规则。只有满足此容器下的各个节点的所有条件才算匹配。
/// >如果指定了父节点RoutingRule，则必须指定此项。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleCondition {
    /// 只有匹配此前缀的Object才能匹配此规则。
    #[serde(rename = "KeyPrefixEquals")]
    pub key_prefix_equals: Option<String>,
    /// 只有匹配此后缀的Object才能匹配此规则。
    #[serde(rename = "KeySuffixEquals")]
    pub key_suffix_equals: Option<String>,
    /// 访问指定Object时，返回此status才能匹配此规则。当跳转规则是镜像回源类型时，此字段必须为404。
    #[serde(rename = "HttpErrorCodeReturnedEquals")]
    pub http_error_code_returned_equals: Option<i64>,
    /// 只有请求中包含了指定Header且值为指定值时，才能匹配此规则。该容器最多可指定10个。
    #[serde(rename = "IncludeHeader")]
    #[serde(default)]
    pub include_header: Vec<RoutingRuleConditionIncludeHeaderItem>,
}

impl crate::FlatSerialize for RoutingRuleCondition {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.key_prefix_equals,
            &format!("{}.KeyPrefixEquals", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.key_suffix_equals,
            &format!("{}.KeySuffixEquals", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.http_error_code_returned_equals,
            &format!("{}.HttpErrorCodeReturnedEquals", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.include_header,
            &format!("{}.IncludeHeader", name),
            params,
        );
    }
}

/// 设置一个Header传到源站，不管请求中是否携带这些指定的Header，回源时都会设置这些Header。只有设置RedirectType为Mirror时才生效。
/// 此容器最多可指定10组。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorHeadersSetItem {
    /// 设置Header的key，最多1024个字节，字符集与Pass相同。只有设置RedirectType为Mirror时才生效。
    /// >若指定了父节点Set，则必须指定此项。
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 设置Header的value，最多1024个字节，不能出现`\r\n`。只有设置RedirectType为Mirror时才生效。
    /// >若指定了父节点Set，则必须指定此项。
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl crate::FlatSerialize for RoutingRuleRedirectMirrorHeadersSetItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
    }
}

/// 指定镜像回源时携带的Header。只有设置RedirectType为Mirror时才生效。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorHeaders {
    ///  是否透传除以下Header之外的其他Header到源站。只有设置RedirectType为Mirror时才生效。
    ///   - content-length、authorization2、authorization、range、date等Header
    ///   - 以oss-/x-oss-/x-drs-开头的Header
    ///
    /// 默认值：false
    #[serde(rename = "PassAll")]
    pub pass_all: Option<bool>,
    /// 透传指定的Header到源站。只有设置RedirectType为Mirror时才生效。
    /// 每个Header长度最多为1024个字节，字符集为0~9、A~Z、a~z以及短划线（-）。
    /// 此字段最多可指定10个。
    #[serde(rename = "Pass")]
    #[serde(default)]
    pub pass: Vec<String>,
    /// 禁止透传指定的Header到源站。只有设置RedirectType为Mirror时才生效。
    /// 每个Header长度最多为1024个字节，字符集与Pass相同。
    /// 此字段最多可指定10个，通常与PassAll一起使用。
    #[serde(rename = "Remove")]
    #[serde(default)]
    pub remove: Vec<String>,
    /// 设置一个Header传到源站，不管请求中是否携带这些指定的Header，回源时都会设置这些Header。只有设置RedirectType为Mirror时才生效。
    /// 此容器最多可指定10组。
    #[serde(rename = "Set")]
    #[serde(default)]
    pub set: Vec<RoutingRuleRedirectMirrorHeadersSetItem>,
}

impl crate::FlatSerialize for RoutingRuleRedirectMirrorHeaders {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.pass_all, &format!("{}.PassAll", name), params);
        crate::FlatSerialize::flat_serialize(&self.pass, &format!("{}.Pass", name), params);
        crate::FlatSerialize::flat_serialize(&self.remove, &format!("{}.Remove", name), params);
        crate::FlatSerialize::flat_serialize(&self.set, &format!("{}.Set", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorTaggingsTaggingsItem {
    /// 当前规则对应标签键
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 保存标签值的规则
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl crate::FlatSerialize for RoutingRuleRedirectMirrorTaggingsTaggingsItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
    }
}

/// 镜像回源保存文件时根据参数保存标签
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorTaggings {
    /// 镜像回源保存标签规则列表
    #[serde(rename = "Taggings")]
    #[serde(default)]
    pub taggings: Vec<RoutingRuleRedirectMirrorTaggingsTaggingsItem>,
}

impl crate::FlatSerialize for RoutingRuleRedirectMirrorTaggings {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.taggings, &format!("{}.Taggings", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorReturnHeadersReturnHeaderItem {
    /// 当前规则对应响应头
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 返回响应头值的规则
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl crate::FlatSerialize for RoutingRuleRedirectMirrorReturnHeadersReturnHeaderItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
    }
}

/// 保存镜像回源返回响应头规则的容器
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorReturnHeaders {
    /// 镜像回源返回响应头规则列表
    #[serde(rename = "ReturnHeader")]
    #[serde(default)]
    pub return_header: Vec<RoutingRuleRedirectMirrorReturnHeadersReturnHeaderItem>,
}

impl crate::FlatSerialize for RoutingRuleRedirectMirrorReturnHeaders {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.return_header,
            &format!("{}.ReturnHeader", name),
            params,
        );
    }
}

/// 镜像回源源站认证信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorAuth {
    /// 镜像回源源站认证类型
    #[serde(rename = "AuthType")]
    pub auth_type: Option<String>,
    /// 镜像回源源站签名Region
    #[serde(rename = "Region")]
    pub region: Option<String>,
    /// 镜像回源源站回源AK
    #[serde(rename = "AccessKeyId")]
    pub access_key_id: Option<String>,
    /// 镜像回源源站回源SK，获取配置时会自动脱敏。
    #[serde(rename = "AccessKeySecret")]
    pub access_key_secret: Option<String>,
}

impl crate::FlatSerialize for RoutingRuleRedirectMirrorAuth {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.auth_type,
            &format!("{}.AuthType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.access_key_id,
            &format!("{}.AccessKeyId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_key_secret,
            &format!("{}.AccessKeySecret", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorMultiAlternatesMirrorMultiAlternateItem {
    /// 镜像回源多源站序号
    #[serde(rename = "MirrorMultiAlternateNumber")]
    pub mirror_multi_alternate_number: Option<i64>,
    /// 镜像回源多源站URL
    #[serde(rename = "MirrorMultiAlternateURL")]
    pub mirror_multi_alternate_url: Option<String>,
    /// 镜像回源多源站VpcId
    #[serde(rename = "MirrorMultiAlternateVpcId")]
    pub mirror_multi_alternate_vpc_id: Option<String>,
    /// 镜像回源多源站Region
    #[serde(rename = "MirrorMultiAlternateDstRegion")]
    pub mirror_multi_alternate_dst_region: Option<String>,
}

impl crate::FlatSerialize for RoutingRuleRedirectMirrorMultiAlternatesMirrorMultiAlternateItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.mirror_multi_alternate_number,
            &format!("{}.MirrorMultiAlternateNumber", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_multi_alternate_url,
            &format!("{}.MirrorMultiAlternateURL", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_multi_alternate_vpc_id,
            &format!("{}.MirrorMultiAlternateVpcId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_multi_alternate_dst_region,
            &format!("{}.MirrorMultiAlternateDstRegion", name),
            params,
        );
    }
}

/// 镜像回源多源站配置列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorMultiAlternates {
    /// 镜像回源多源站配置
    #[serde(rename = "MirrorMultiAlternate")]
    #[serde(default)]
    pub mirror_multi_alternate:
        Vec<RoutingRuleRedirectMirrorMultiAlternatesMirrorMultiAlternateItem>,
}

impl crate::FlatSerialize for RoutingRuleRedirectMirrorMultiAlternates {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.mirror_multi_alternate,
            &format!("{}.MirrorMultiAlternate", name),
            params,
        );
    }
}

/// 指定匹配此规则后执行的动作。
/// >如果指定了父节点RoutingRule，则必须指定此项。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirect {
    /// 指定跳转的类型。取值范围如下：
    ///   - **Mirror**：镜像回源。
    ///   - **External**：外部跳转，即OSS会返回一个3xx请求，指定跳转到另外一个地址。
    ///   - **AliCDN**：阿里云CDN跳转，主要用于阿里云的CDN。与External不同的是，OSS会额外添加一个Header。阿里云CDN识别到此Header后会主动跳转到指定的地址，返回给用户获取到的数据，而不是将3xx跳转请求返回给用户。
    ///
    /// >如果指定了父节点Redirect，则必须指定此项。
    #[serde(rename = "RedirectType")]
    pub redirect_type: Option<String>,
    /// 执行跳转或者镜像回源规则时，是否携带请求参数。
    /// 用户请求OSS时携带了请求参数`?a=b&c=d`，并且设置**PassQueryString**为true，如果规则为302跳转，则跳转的Location头中会添加此请求参数。例如`Location:example.com?a=b&c=d`，跳转类型为镜像回源，则在发起的回源请求中也会携带此请求参数。
    /// 取值：true、false（默认）
    #[serde(rename = "PassQueryString")]
    pub pass_query_string: Option<bool>,
    /// 镜像回源的源站地址。只有设置RedirectType为Mirror时才生效。
    /// 源站地址必须以**http://**或者**https://**开头，并且以正斜线（/）结尾，OSS会在此地址后带上Object名称组成回源URL。
    /// 例如要访问的Object名称为myobject，如果指定此项为`http://example.com/`，则回源URL为`http://example.com/myobject`，如果指定此项为`http://example.com/dir1/`，则回源URL为`http://example.com/dir1/myobject`。
    /// >如果RedirectType指定为Mirror，则必须指定此项。
    #[serde(rename = "MirrorURL")]
    pub mirror_url: Option<String>,
    /// 是否透传SNI
    #[serde(rename = "MirrorSNI")]
    pub mirror_sni: Option<bool>,
    /// 与PassQueryString作用相同，优先级高于PassQueryString。只有设置RedirectType为Mirror时才生效。
    /// 默认值：false
    #[serde(rename = "MirrorPassQueryString")]
    pub mirror_pass_query_string: Option<bool>,
    /// 如果镜像回源获取的结果为3xx，是否继续跳转到指定的Location获取数据。 只有设置RedirectType为Mirror时才生效。
    /// 例如发起镜像回源请求时，源站返回了302，并且指定了Location。
    ///   - 如果设置此项为true，则OSS会继续请求Location对应的地址。
    /// 最多能跳转10次，如果跳转超过10次，则返回镜像回源失败。
    ///   - 如果设置此项为false，则OSS会返回302，并透传Location。
    ///
    /// 默认值：true
    #[serde(rename = "MirrorFollowRedirect")]
    pub mirror_follow_redirect: Option<bool>,
    /// 是否检查回源body的MD5。 只有设置RedirectType为Mirror时才生效。
    /// 当设置**MirrorCheckMd5**为true，并且源站返回的response中含有Content-Md5头时，OSS检查拉取的数据MD5是否与此Header匹配，如果不匹配，则不保存在OSS上。
    /// 默认值：false
    #[serde(rename = "MirrorCheckMd5")]
    pub mirror_check_md5: Option<bool>,
    /// 指定镜像回源时携带的Header。只有设置RedirectType为Mirror时才生效。
    #[serde(rename = "MirrorHeaders")]
    pub mirror_headers: Option<RoutingRuleRedirectMirrorHeaders>,
    /// 跳转时的协议。只有设置RedirectType为External或者AliCDN时才生效。
    /// 如果要访问的文件为test，设置跳转到`example.com`，并且设置Protocol为https，则Location头为`https://example.com/test`。
    /// 取值：**http**、**https**。
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    /// 跳转时的域名，域名需符合域名规范。
    /// 如果要访问的文件为test，设置Protocol为https，并且设置Hostname为`example.com`，则Location头为`https://example.com/test`。
    #[serde(rename = "HostName")]
    pub host_name: Option<String>,
    /// Redirect时Object名称的前缀将替换成该值。如果前缀为空，则将这个字符串插入Object名称的前面。
    /// > 仅允许存在ReplaceKeyWith或ReplaceKeyPrefixWith节点。
    /// 假设要访问的Object为abc/test.txt，如果设置KeyPrefixEquals为abc/，ReplaceKeyPrefixWith为def/，则Location头为`http://example.com/def/test.txt`。
    #[serde(rename = "ReplaceKeyPrefixWith")]
    pub replace_key_prefix_with: Option<String>,
    /// 如果设置此字段为true，则Object的前缀将被替换为ReplaceKeyPrefixWith指定的值。如果未指定此字段或为空，则表示截断Object前缀。
    /// > 当ReplaceKeyWith字段不为空时，不能设置此字段为true。
    ///
    /// 默认值：false
    #[serde(rename = "EnableReplacePrefix")]
    pub enable_replace_prefix: Option<bool>,
    /// Redirect时Object名称将替换成ReplaceKeyWith指定的值，ReplaceKeyWith支持设置变量。目前支持的变量为${key}，表示该请求中的Object名称。
    /// 假设要访问的Object为test，如果设置ReplaceKeyWith为`prefix/${key}.suffix`，则Location头为`http://example.com/prefix/test.suffix`。
    #[serde(rename = "ReplaceKeyWith")]
    pub replace_key_with: Option<String>,
    /// 跳转时返回的状态码。只有设置RedirectType为External或者AliCDN时才生效。
    /// 取值：301（默认）、302、307。
    #[serde(rename = "HttpRedirectCode")]
    pub http_redirect_code: Option<i64>,
    /// 是否透传/到源站
    #[serde(rename = "MirrorPassOriginalSlashes")]
    pub mirror_pass_original_slashes: Option<bool>,
    /// 镜像回源主备回源备站URL
    #[serde(rename = "MirrorURLSlave")]
    pub mirror_url_slave: Option<String>,
    /// 镜像回源主备回源切换判定URL
    #[serde(rename = "MirrorURLProbe")]
    pub mirror_url_probe: Option<String>,
    /// 镜像回源回源OSS是否自动保存用户元数据
    #[serde(rename = "MirrorSaveOssMeta")]
    pub mirror_save_oss_meta: Option<bool>,
    /// 镜像回源是否不保存数据
    #[serde(rename = "MirrorProxyPass")]
    pub mirror_proxy_pass: Option<bool>,
    /// 镜像回源允许获取Image信息
    #[serde(rename = "MirrorAllowGetImageInfo")]
    pub mirror_allow_get_image_info: Option<bool>,
    /// 镜像回源允许支持视频截帧
    #[serde(rename = "MirrorAllowVideoSnapshot")]
    pub mirror_allow_video_snapshot: Option<bool>,
    /// 是否是镜像回源高速通道
    #[serde(rename = "MirrorIsExpressTunnel")]
    pub mirror_is_express_tunnel: Option<bool>,
    /// 镜像回源高速通道VpcRegion
    #[serde(rename = "MirrorDstRegion")]
    pub mirror_dst_region: Option<String>,
    /// 镜像回源高速通道VpcId
    #[serde(rename = "MirrorDstVpcId")]
    pub mirror_dst_vpc_id: Option<String>,
    /// 镜像回源高速通道备站VpcId
    #[serde(rename = "MirrorDstSlaveVpcId")]
    pub mirror_dst_slave_vpc_id: Option<String>,
    /// 镜像回源保存文件是否使用源站LastModifiedTime
    #[serde(rename = "MirrorUserLastModified")]
    pub mirror_user_last_modified: Option<bool>,
    /// 用于主备切换的状态判断，主备切换的判断逻辑是源站返回错误，如果MirrorSwitchAllErrors为true，则除了以下状态码外都认为是失败：200,206,301,302,303,307,404；而如果为false，则只有源站返回5xx或者超时才认为是失败。
    #[serde(rename = "MirrorSwitchAllErrors")]
    pub mirror_switch_all_errors: Option<bool>,
    /// 镜像回源专线回源TunnelId
    #[serde(rename = "MirrorTunnelId")]
    pub mirror_tunnel_id: Option<String>,
    /// 镜像回源是否使用角色
    #[serde(rename = "MirrorUsingRole")]
    pub mirror_using_role: Option<bool>,
    /// 镜像回源回源时使用的角色
    #[serde(rename = "MirrorRole")]
    pub mirror_role: Option<String>,
    /// 镜像回源是否允许HeadObject
    #[serde(rename = "MirrorAllowHeadObject")]
    pub mirror_allow_head_object: Option<bool>,
    /// 指定当源站返回哪些状态码时需要透传该状态码以及body到客户端，取值为4xx、5xx等HTTP状态码，多个HTTP状态码之间用英文逗号（,）分隔，例如`400,404`。只有设置RedirectType为Mirror时才生效。
    /// 当OSS向源站请求内容时，如果源站返回了此参数中的某个状态码，则OSS将透传源站返回的该状态码以及body到客户端。
    /// > 如果在此参数中设置了404状态码，则设置的ErrorDocument会失效。
    #[serde(rename = "TransparentMirrorResponseCodes")]
    pub transparent_mirror_response_codes: Option<String>,
    /// 镜像回源触发异步拉取模式的状态码
    #[serde(rename = "MirrorAsyncStatus")]
    pub mirror_async_status: Option<i64>,
    /// 镜像回源保存文件时根据参数保存标签
    #[serde(rename = "MirrorTaggings")]
    pub mirror_taggings: Option<RoutingRuleRedirectMirrorTaggings>,
    /// 保存镜像回源返回响应头规则的容器
    #[serde(rename = "MirrorReturnHeaders")]
    pub mirror_return_headers: Option<RoutingRuleRedirectMirrorReturnHeaders>,
    /// 镜像回源源站认证信息
    #[serde(rename = "MirrorAuth")]
    pub mirror_auth: Option<RoutingRuleRedirectMirrorAuth>,
    /// 镜像回源多源站配置列表
    #[serde(rename = "MirrorMultiAlternates")]
    pub mirror_multi_alternates: Option<RoutingRuleRedirectMirrorMultiAlternates>,
}

impl crate::FlatSerialize for RoutingRuleRedirect {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.redirect_type,
            &format!("{}.RedirectType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.pass_query_string,
            &format!("{}.PassQueryString", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_url,
            &format!("{}.MirrorURL", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_sni,
            &format!("{}.MirrorSNI", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_pass_query_string,
            &format!("{}.MirrorPassQueryString", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_follow_redirect,
            &format!("{}.MirrorFollowRedirect", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_check_md5,
            &format!("{}.MirrorCheckMd5", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_headers,
            &format!("{}.MirrorHeaders", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.protocol, &format!("{}.Protocol", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.host_name,
            &format!("{}.HostName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.replace_key_prefix_with,
            &format!("{}.ReplaceKeyPrefixWith", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_replace_prefix,
            &format!("{}.EnableReplacePrefix", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.replace_key_with,
            &format!("{}.ReplaceKeyWith", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.http_redirect_code,
            &format!("{}.HttpRedirectCode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_pass_original_slashes,
            &format!("{}.MirrorPassOriginalSlashes", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_url_slave,
            &format!("{}.MirrorURLSlave", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_url_probe,
            &format!("{}.MirrorURLProbe", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_save_oss_meta,
            &format!("{}.MirrorSaveOssMeta", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_proxy_pass,
            &format!("{}.MirrorProxyPass", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_allow_get_image_info,
            &format!("{}.MirrorAllowGetImageInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_allow_video_snapshot,
            &format!("{}.MirrorAllowVideoSnapshot", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_is_express_tunnel,
            &format!("{}.MirrorIsExpressTunnel", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_dst_region,
            &format!("{}.MirrorDstRegion", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_dst_vpc_id,
            &format!("{}.MirrorDstVpcId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_dst_slave_vpc_id,
            &format!("{}.MirrorDstSlaveVpcId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_user_last_modified,
            &format!("{}.MirrorUserLastModified", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_switch_all_errors,
            &format!("{}.MirrorSwitchAllErrors", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_tunnel_id,
            &format!("{}.MirrorTunnelId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_using_role,
            &format!("{}.MirrorUsingRole", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_role,
            &format!("{}.MirrorRole", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_allow_head_object,
            &format!("{}.MirrorAllowHeadObject", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.transparent_mirror_response_codes,
            &format!("{}.TransparentMirrorResponseCodes", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_async_status,
            &format!("{}.MirrorAsyncStatus", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_taggings,
            &format!("{}.MirrorTaggings", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_return_headers,
            &format!("{}.MirrorReturnHeaders", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_auth,
            &format!("{}.MirrorAuth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mirror_multi_alternates,
            &format!("{}.MirrorMultiAlternates", name),
            params,
        );
    }
}

/// 需要执行的Lua脚本配置。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleLuaConfig {
    /// Lua脚本名称。
    #[serde(rename = "Script")]
    pub script: Option<String>,
}

impl crate::FlatSerialize for RoutingRuleLuaConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.script, &format!("{}.Script", name), params);
    }
}

/// 指定跳转规则或者镜像回源规则，最多指定20个RoutingRule。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRule {
    /// 匹配和执行RoutingRule的序号，OSS将按照此序号依次匹配规则。如果匹配成功，则执行此规则，后续的规则不再执行。
    ///
    /// >如果指定了父节点RoutingRule，则必须指定此项。
    #[serde(rename = "RuleNumber")]
    pub rule_number: Option<i64>,
    /// 匹配的条件。
    /// 如果指定的项都满足，则执行此规则。只有满足此容器下的各个节点的所有条件才算匹配。
    /// >如果指定了父节点RoutingRule，则必须指定此项。
    #[serde(rename = "Condition")]
    pub condition: Option<RoutingRuleCondition>,
    /// 指定匹配此规则后执行的动作。
    /// >如果指定了父节点RoutingRule，则必须指定此项。
    #[serde(rename = "Redirect")]
    pub redirect: Option<RoutingRuleRedirect>,
    /// 该规则需要执行的Lua脚本配置。
    #[serde(rename = "LuaConfig")]
    pub lua_config: Option<RoutingRuleLuaConfig>,
}

impl crate::FlatSerialize for RoutingRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.rule_number,
            &format!("{}.RuleNumber", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.condition,
            &format!("{}.Condition", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.redirect, &format!("{}.Redirect", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.lua_config,
            &format!("{}.LuaConfig", name),
            params,
        );
    }
}

/// 数据复制时间控制配置容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RtcConfiguration {
    /// 保存RTC状态规则的容器。
    #[serde(rename = "RTC")]
    pub rtc: Option<RTC>,
    /// 需要设置RTC状态的复制规则ID。
    #[serde(rename = "ID")]
    pub id: Option<String>,
}

impl crate::FlatSerialize for RtcConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rtc, &format!("{}.RTC", name), params);
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
    }
}

/// 保存SelectMetaRequest信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SelectMetaRequest {
    /// 输入序列化参数（可选）。
    #[serde(rename = "InputSerialization")]
    pub input_serialization: Option<InputSerialization>,
    /// 重新计算SelectMeta，覆盖已有数据。
    /// <br>默认值：false（即如果Select Meta已存在则直接返回）。
    #[serde(rename = "OverwriteIfExists")]
    pub overwrite_if_exists: Option<bool>,
}

impl crate::FlatSerialize for SelectMetaRequest {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.input_serialization,
            &format!("{}.InputSerialization", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.overwrite_if_exists,
            &format!("{}.OverwriteIfExists", name),
            params,
        );
    }
}

/// 保存CreateSelectObjectMeta信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SelectMetaStatus {
    /// 8位整数，扫描结束时的文件偏移。
    #[serde(rename = "Offset")]
    pub offset: Option<i64>,
    /// 8位整数，最终扫描过的数据大小。
    #[serde(rename = "TotalScannedBytes")]
    pub total_scanned_bytes: Option<i64>,
    /// 4位整数，最终的status。
    #[serde(rename = "Status")]
    pub status: Option<i64>,
    /// 4位整数，总split个数。
    #[serde(rename = "SplitsCount")]
    pub splits_count: Option<i64>,
    /// 8位整数，总行数。
    #[serde(rename = "RowsCount")]
    pub rows_count: Option<i64>,
    /// 4位整数，总列数。
    #[serde(rename = "ColsCount")]
    pub cols_count: Option<i64>,
    /// 详细的错误信息。如果无错误，则error_message为空。
    #[serde(rename = "ErrorMessage")]
    pub error_message: Option<String>,
}

impl crate::FlatSerialize for SelectMetaStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.offset, &format!("{}.Offset", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.total_scanned_bytes,
            &format!("{}.TotalScannedBytes", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.splits_count,
            &format!("{}.SplitsCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.rows_count,
            &format!("{}.RowsCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cols_count,
            &format!("{}.ColsCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.error_message,
            &format!("{}.ErrorMessage", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for SelectMetaStatus {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 保存Select请求的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SelectRequestOptions {
    /// 忽略缺失数据的行。
    ///
    /// - 当该参数为false时，OSS会忽略缺失某些列（该列值当做null）而不报错。
    ///
    /// - 当该参数为true时，该行数据因为不完整而被整体跳过。当跳过的行数超过指定的最大跳过行数时OSS会报错并停止处理。
    #[serde(rename = "SkipPartialDataRecord")]
    pub skip_partial_data_record: Option<bool>,
    /// 指定最大能容忍的跳过的行数。当某一行数据因为不匹配SQL中期望的类型、或者某一列或者多列数据缺失且SkipPartialDataRecord为True时，该行数据会被跳过。如果跳过的行数超过该参数的值，OSS会停止处理并报错。
    /// <br>默认值：0
    ///
    /// > 如果某一行是非法CSV行，例如在一列中间连续含有奇数个quote字符，则OSS会马上停止处理并报错，因为该错误很可能会影响对整个CSV文件的解析。即该参数用来调整对非整齐数据的容忍度，但不应用于非法的CSV文件。
    #[serde(rename = "MaxSkippedRecordsAllowed")]
    pub max_skipped_records_allowed: Option<i64>,
}

impl crate::FlatSerialize for SelectRequestOptions {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.skip_partial_data_record,
            &format!("{}.SkipPartialDataRecord", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_skipped_records_allowed,
            &format!("{}.MaxSkippedRecordsAllowed", name),
            params,
        );
    }
}

/// 保存Select请求的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SelectRequest {
    /// 以Base64 编码的SQL语句。
    #[serde(rename = "Expression")]
    pub expression: Option<String>,
    /// 输入序列化参数。
    #[serde(rename = "InputSerialization")]
    pub input_serialization: Option<InputSerialization>,
    /// 输出序列化参数。
    #[serde(rename = "OutputSerialization")]
    pub output_serialization: Option<OutputSerialization>,
    /// 其他可选参数。
    #[serde(rename = "Options")]
    pub options: Option<SelectRequestOptions>,
}

impl crate::FlatSerialize for SelectRequest {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.expression,
            &format!("{}.Expression", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.input_serialization,
            &format!("{}.InputSerialization", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_serialization,
            &format!("{}.OutputSerialization", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.options, &format!("{}.Options", name), params);
    }
}

/// 服务器端加密规则的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServerSideEncryptionRule {
    /// 服务器端默认加密方式的容器。
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    pub apply_server_side_encryption_by_default: Option<ApplyServerSideEncryptionByDefault>,
}

impl crate::FlatSerialize for ServerSideEncryptionRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.apply_server_side_encryption_by_default,
            &format!("{}.ApplyServerSideEncryptionByDefault", name),
            params,
        );
    }
}

/// 保存Content信息列表的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Style {
    /// 图片样式内容，图片样式可以包含一个或多个图片处理操作。
    ///
    /// - 包含单个图片处理操作的图片样式，例如image/resize,p_50，表示将原图缩放50%。
    ///
    /// - 包含多个图片处理操作的图片样式，例如image/resize,p_63/quality,q_90，表示先将图片缩放到原图的63%，再设置图片相对质量为90%。
    #[serde(rename = "Content")]
    pub content: Option<String>,
}

impl crate::FlatSerialize for Style {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.content, &format!("{}.Content", name), params);
    }
}

/// 包含存储空间图片处理样式信息的结构体
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StyleInfo {
    /// 样式名称
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// 样式内容
    #[serde(rename = "Content")]
    pub content: Option<String>,
    /// 样式创建时间
    #[serde(rename = "CreateTime")]
    pub create_time: Option<String>,
    /// 样式创建时间
    #[serde(rename = "LastModifyTime")]
    pub last_modify_time: Option<String>,
    /// 样式分类。  取值：image、document、video。
    #[serde(rename = "Category")]
    pub category: Option<String>,
}

impl crate::FlatSerialize for StyleInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.content, &format!("{}.Content", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modify_time,
            &format!("{}.LastModifyTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.category, &format!("{}.Category", name), params);
    }
}

impl crate::ToCodeMessage for StyleInfo {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 标签集合。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TagSet {
    /// 标签集合。
    #[serde(rename = "Tag")]
    #[serde(default)]
    pub tag: Vec<Tag>,
}

impl crate::FlatSerialize for TagSet {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
    }
}

/// 设置Bucket TagSet的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tagging {
    /// 包含一系列Bucket Tag的容器。
    #[serde(rename = "TagSet")]
    pub tag_set: Option<TagSet>,
}

impl crate::FlatSerialize for Tagging {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tag_set, &format!("{}.TagSet", name), params);
    }
}

/// 传输加速配置的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TransferAccelerationConfiguration {
    /// 目标Bucket是否开启传输加速。取值如下：
    ///   - **true**：开启传输加速。
    ///   - **false**：关闭传输加速。
    ///
    /// > 传输加速开启及关闭操作在30分钟内生效。
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
}

impl crate::FlatSerialize for TransferAccelerationConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.enabled, &format!("{}.Enabled", name), params);
    }
}

/// 保存Multipart Upload事件信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Upload {
    /// 初始化Multipart Upload事件的Object名称。
    ///
    ///
    ///
    /// > OSS的返回结果按照Object名称字典序升序排列，对于同一个Object，则按照UploadId的字典序升序排列。
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// Multipart Upload事件的ID。
    #[serde(rename = "UploadId")]
    pub upload_id: Option<String>,
    /// Multipart Upload事件初始化的时间。
    #[serde(rename = "Initiated")]
    pub initiated: Option<String>,
}

impl crate::FlatSerialize for Upload {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.upload_id,
            &format!("{}.UploadId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.initiated,
            &format!("{}.Initiated", name),
            params,
        );
    }
}

/// 高防实例信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserAntiDDOSInfo {
    /// 高防实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,
    /// 高防实例拥有者的UID。
    #[serde(rename = "Owner")]
    pub owner: Option<String>,
    /// 高防实例创建时间，格式为时间戳。
    #[serde(rename = "Ctime")]
    pub ctime: Option<i64>,
    /// 高防实例更新时间，格式为时间戳。
    #[serde(rename = "Mtime")]
    pub mtime: Option<i64>,
    /// 高防实例激活时间，格式为时间戳。
    #[serde(rename = "ActiveTime")]
    pub active_time: Option<i64>,
    /// 高防实例所处状态。
    ///
    /// - Init：初始化防护状态。
    ///
    /// - Defending：防护中状态。
    ///
    /// - HaltDefending：解除防护状态。
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

impl crate::FlatSerialize for UserAntiDDOSInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.instance_id,
            &format!("{}.InstanceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(&self.ctime, &format!("{}.Ctime", name), params);
        crate::FlatSerialize::flat_serialize(&self.mtime, &format!("{}.Mtime", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.active_time,
            &format!("{}.ActiveTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

/// 自定义请求头配置信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserDefinedLogFieldsConfigurationHeaderSet {
    /// 自定义请求头列表。
    #[serde(rename = "header")]
    #[serde(default)]
    pub header: Vec<String>,
}

impl crate::FlatSerialize for UserDefinedLogFieldsConfigurationHeaderSet {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.header, &format!("{}.header", name), params);
    }
}

/// 自定义查询参数配置信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserDefinedLogFieldsConfigurationParamSet {
    /// 自定义查询参数列表。
    #[serde(rename = "parameter")]
    #[serde(default)]
    pub parameter: Vec<String>,
}

impl crate::FlatSerialize for UserDefinedLogFieldsConfigurationParamSet {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.parameter,
            &format!("{}.parameter", name),
            params,
        );
    }
}

/// 存储空间实时日志中用户自定义字段配置。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserDefinedLogFieldsConfiguration {
    /// 自定义请求头配置信息的容器。
    #[serde(rename = "HeaderSet")]
    pub header_set: Option<UserDefinedLogFieldsConfigurationHeaderSet>,
    /// 自定义查询参数配置信息的容器。
    #[serde(rename = "ParamSet")]
    pub param_set: Option<UserDefinedLogFieldsConfigurationParamSet>,
}

impl crate::FlatSerialize for UserDefinedLogFieldsConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.header_set,
            &format!("{}.HeaderSet", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.param_set,
            &format!("{}.ParamSet", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for UserDefinedLogFieldsConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 用户级别流控配额
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserQosConfiguration {
    /// 总上行带宽，单位Gbps
    #[serde(rename = "TotalUploadBandwidth")]
    pub total_upload_bandwidth: Option<i64>,
    /// 内网上行带宽，单位Gbps
    #[serde(rename = "IntranetUploadBandwidth")]
    pub intranet_upload_bandwidth: Option<i64>,
    /// 公网上行带宽，单位Gbps
    #[serde(rename = "ExtranetUploadBandwidth")]
    pub extranet_upload_bandwidth: Option<i64>,
    /// 总下行带宽，单位Gbps
    #[serde(rename = "TotalDownloadBandwidth")]
    pub total_download_bandwidth: Option<i64>,
    /// 内网下行带宽，单位Gbps
    #[serde(rename = "IntranetDownloadBandwidth")]
    pub intranet_download_bandwidth: Option<i64>,
    /// 公网下行带宽，单位Gbps
    #[serde(rename = "ExtranetDownloadBandwidth")]
    pub extranet_download_bandwidth: Option<i64>,
    /// 总QPS
    #[serde(rename = "TotalQps")]
    pub total_qps: Option<i64>,
    /// 内网QPS
    #[serde(rename = "IntranetQps")]
    pub intranet_qps: Option<i64>,
    /// 公网QPS
    #[serde(rename = "ExtranetQps")]
    pub extranet_qps: Option<i64>,
    /// 备注
    #[serde(rename = "Remark")]
    pub remark: Option<i64>,
    /// 所在区域
    #[serde(rename = "Region")]
    pub region: Option<String>,
    /// 用户级别默认QoS配置信息
    #[serde(rename = "DefaultQoSConfiguration")]
    pub default_qo_s_configuration: Option<QoSConfigurationWithRemark>,
}

impl crate::FlatSerialize for UserQosConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.total_upload_bandwidth,
            &format!("{}.TotalUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_upload_bandwidth,
            &format!("{}.IntranetUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_upload_bandwidth,
            &format!("{}.ExtranetUploadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_download_bandwidth,
            &format!("{}.TotalDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_download_bandwidth,
            &format!("{}.IntranetDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_download_bandwidth,
            &format!("{}.ExtranetDownloadBandwidth", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_qps,
            &format!("{}.TotalQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.intranet_qps,
            &format!("{}.IntranetQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.extranet_qps,
            &format!("{}.ExtranetQps", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.remark, &format!("{}.Remark", name), params);
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.default_qo_s_configuration,
            &format!("{}.DefaultQoSConfiguration", name),
            params,
        );
    }
}

/// 保存版本控制状态的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VersioningConfiguration {
    /// 版本控制状态。取值如下：
    ///
    /// - Enabled：开启版本控制状态。
    ///
    /// - Suspended：暂停版本控制状态。
    #[serde(rename = "Status")]
    pub status: Option<BucketVersioningStatus>,
}

impl crate::FlatSerialize for VersioningConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VirtualBucketConfigurationRealBucketItem {
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl crate::FlatSerialize for VirtualBucketConfigurationRealBucketItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
    }
}

/// 虚拟Bucket配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VirtualBucketConfiguration {
    #[serde(rename = "RealBucket")]
    #[serde(default)]
    pub real_bucket: Vec<VirtualBucketConfigurationRealBucketItem>,
}

impl crate::FlatSerialize for VirtualBucketConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.real_bucket,
            &format!("{}.RealBucket", name),
            params,
        );
    }
}

/// RoutingRule的容器。
///
/// >至少指定IndexDocument、ErrorDocument、RoutingRules三个容器中的一个。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebsiteConfigurationRoutingRules {
    /// 指定跳转规则或者镜像回源规则，最多指定20个RoutingRule。
    #[serde(rename = "RoutingRule")]
    #[serde(default)]
    pub routing_rule: Vec<RoutingRule>,
}

impl crate::FlatSerialize for WebsiteConfigurationRoutingRules {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.routing_rule,
            &format!("{}.RoutingRule", name),
            params,
        );
    }
}

/// 根节点。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebsiteConfiguration {
    /// 默认主页的容器。
    ///
    /// >至少指定IndexDocument、ErrorDocument、RoutingRules三个容器中的一个。
    #[serde(rename = "IndexDocument")]
    pub index_document: Option<IndexDocument>,
    /// 404页面的容器。
    ///
    /// >至少指定IndexDocument、ErrorDocument、RoutingRules三个容器中的一个。
    #[serde(rename = "ErrorDocument")]
    pub error_document: Option<ErrorDocument>,
    /// RoutingRule的容器。
    ///
    /// >至少指定IndexDocument、ErrorDocument、RoutingRules三个容器中的一个。
    #[serde(rename = "RoutingRules")]
    pub routing_rules: Option<WebsiteConfigurationRoutingRules>,
}

impl crate::FlatSerialize for WebsiteConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.index_document,
            &format!("{}.IndexDocument", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.error_document,
            &format!("{}.ErrorDocument", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.routing_rules,
            &format!("{}.RoutingRules", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for WebsiteConfiguration {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// 保存Bucket信息列表的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResponseBuckets {
    /// 保存多个Bucket信息的列表。
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: Vec<Bucket>,
}

impl crate::FlatSerialize for ResponseBuckets {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
    }
}

/// 存储ACL信息的容器类。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAclResponseAccessControlList {
    /// Bucket的ACL权限。
    #[serde(rename = "Grant")]
    #[serde(default)]
    pub grant: BucketACL,
}

impl crate::FlatSerialize for BucketAclResponseAccessControlList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.grant, &format!("{}.Grant", name), params);
    }
}

/// 保存数据复制配置信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketReplicationbody {
    #[serde(rename = "Rule")]
    pub rule: Option<ReplicationRule>,
}

impl crate::FlatSerialize for PutBucketReplicationbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

/// 包含TransferType约束的Location信息容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TransferTypeConstraint {
    /// 包含TransferType的Location信息容器。
    #[serde(rename = "LocationTransferType")]
    #[serde(default)]
    pub location_transfer_type: Vec<LocationTransferType>,
}

impl crate::FlatSerialize for TransferTypeConstraint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.location_transfer_type,
            &format!("{}.LocationTransferType", name),
            params,
        );
    }
}

/// 包含RTC约束的Location信息容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CConstraint {
    /// 支持RTC的Location信息列表。
    #[serde(rename = "Location")]
    #[serde(default)]
    pub location: Vec<String>,
}

impl crate::FlatSerialize for CConstraint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
    }
}

/// 保存需要删除的数据复制规则的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketReplicationbody {
    /// 需要删除的数据复制规则的名称。
    #[serde(rename = "ID")]
    pub id: Option<String>,
}

impl crate::FlatSerialize for DeleteBucketReplicationbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
    }
}

/// 保存目标Cname域名的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnamebodyCname {
    /// 要删除的Cname。
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
}

impl crate::FlatSerialize for CnamebodyCname {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.domain, &format!("{}.Domain", name), params);
    }
}

/// 保存Cname配置的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeleteCnamebody {
    /// 保存目标Cname域名的容器。
    #[serde(rename = "Cname")]
    pub cname: Option<CnamebodyCname>,
}

impl crate::FlatSerialize for DeleteCnamebody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.cname, &format!("{}.Cname", name), params);
    }
}

/// 保存要生成Token的Cname域名。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TokenbodyCname {
    /// 目标Cname域名。
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
}

impl crate::FlatSerialize for TokenbodyCname {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.domain, &format!("{}.Domain", name), params);
    }
}

/// 保存Cname配置的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameTokenbody {
    /// 保存要生成Token的Cname域名。
    #[serde(rename = "Cname")]
    pub cname: Option<TokenbodyCname>,
}

impl crate::FlatSerialize for CnameTokenbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.cname, &format!("{}.Cname", name), params);
    }
}

/// 保存对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PointForObjectProcessbody {
    /// OSS接入点名称。更多信息，请参见[创建接入点](~~2365063~~)。
    #[serde(rename = "AccessPointName")]
    pub access_point_name: Option<String>,
    /// 保存对象处理信息的容器。
    #[serde(rename = "ObjectProcessConfiguration")]
    pub object_process_configuration: Option<ObjectProcessConfiguration>,
    /// 是否允许匿名访问。
    ///
    /// > 开启该选项之后将会允许匿名账号通过ObjectFC接入点访问您的Bucket，会产生相关费用。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: Option<String>,
}

impl crate::FlatSerialize for PointForObjectProcessbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point_name,
            &format!("{}.AccessPointName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.object_process_configuration,
            &format!("{}.ObjectProcessConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.allow_anonymous_access_for_object_process,
            &format!("{}.AllowAnonymousAccessForObjectProcess", name),
            params,
        );
    }
}

/// 保存对象FC接入点访问域名信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResponseEndpoints {
    /// 对象FC接入点的外网Endpoint。
    #[serde(rename = "PublicEndpoint")]
    #[serde(default)]
    pub public_endpoint: String,
    /// 对象FC接入点的内网Endpoint。
    #[serde(rename = "InternalEndpoint")]
    #[serde(default)]
    pub internal_endpoint: String,
}

impl crate::FlatSerialize for ResponseEndpoints {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.public_endpoint,
            &format!("{}.PublicEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.internal_endpoint,
            &format!("{}.InternalEndpoint", name),
            params,
        );
    }
}

/// 保存单个对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessPointForObjectProcess {
    /// 对象FC接入点名称。
    #[serde(rename = "AccessPointNameForObjectProcess")]
    #[serde(default)]
    pub access_point_name_for_object_process: String,
    /// 对象FC接入点别名。
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    #[serde(default)]
    pub access_point_for_object_process_alias: String,
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    #[serde(default)]
    pub access_point_name: String,
    /// 对象FC接入点所处状态。返回值如下：
    ///
    /// - enable：对象FC接入点已创建完成。
    /// - disable：对象FC接入点已禁用。
    /// - creating：对象FC接入点正在创建中。
    /// - deleting：对象FC接入点已删除。
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    /// 是否允许匿名访问。
    ///
    /// > 开启该选项之后将会允许匿名账号通过ObjectFC接入点访问您的Bucket，会产生相关费用。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    #[serde(default)]
    pub allow_anonymous_access_for_object_process: String,
}

impl crate::FlatSerialize for AccessPointForObjectProcess {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point_name_for_object_process,
            &format!("{}.AccessPointNameForObjectProcess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_point_for_object_process_alias,
            &format!("{}.AccessPointForObjectProcessAlias", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_point_name,
            &format!("{}.AccessPointName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.allow_anonymous_access_for_object_process,
            &format!("{}.AllowAnonymousAccessForObjectProcess", name),
            params,
        );
    }
}

/// 保存所有对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessPointsForObjectProcess {
    /// 保存单个对象FC接入点信息的容器。
    #[serde(rename = "AccessPointForObjectProcess")]
    #[serde(default)]
    pub access_point_for_object_process: Vec<AccessPointForObjectProcess>,
}

impl crate::FlatSerialize for AccessPointsForObjectProcess {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point_for_object_process,
            &format!("{}.AccessPointForObjectProcess", name),
            params,
        );
    }
}

/// 保存对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigForObjectProcessbody {
    /// 保存对象处理信息的容器。
    #[serde(rename = "ObjectProcessConfiguration")]
    pub object_process_configuration: Option<ObjectProcessConfiguration>,
    /// 是否允许匿名访问。
    ///
    /// > 开启该选项之后将会允许匿名账号通过ObjectFC接入点访问您的Bucket，会产生相关费用。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: Option<String>,
    /// 保存阻止公共访问信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

impl crate::FlatSerialize for ConfigForObjectProcessbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.object_process_configuration,
            &format!("{}.ObjectProcessConfiguration", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.allow_anonymous_access_for_object_process,
            &format!("{}.AllowAnonymousAccessForObjectProcess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.public_access_block_configuration,
            &format!("{}.PublicAccessBlockConfiguration", name),
            params,
        );
    }
}

/// 存储ACL信息的容器。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectAclResponseAccessControlList {
    /// Object的ACL权限。
    #[serde(rename = "Grant")]
    #[serde(default)]
    pub grant: ObjectACL,
}

impl crate::FlatSerialize for ObjectAclResponseAccessControlList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.grant, &format!("{}.Grant", name), params);
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ObjectACL {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public-read")]
    PublicRead,
    #[serde(rename = "public-read-write")]
    PublicReadWrite,
    #[serde(rename = "default")]
    Default,
}

impl Default for ObjectACL {
    fn default() -> Self {
        Self::Private
    }
}

impl ObjectACL {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::PublicRead => "public-read",
            Self::PublicReadWrite => "public-read-write",
            Self::Default => "default",
        }
    }
}

impl std::fmt::Display for ObjectACL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a ObjectACL> for crate::QueryValue<'a> {
    fn from(value: &'a ObjectACL) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for ObjectACL {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AccessMonitorStatus {
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "Disabled")]
    Disabled,
}

impl Default for AccessMonitorStatus {
    fn default() -> Self {
        Self::Enabled
    }
}

impl AccessMonitorStatus {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}

impl std::fmt::Display for AccessMonitorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a AccessMonitorStatus> for crate::QueryValue<'a> {
    fn from(value: &'a AccessMonitorStatus) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for AccessMonitorStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StorageClass {
    /// Standard
    #[serde(rename = "Standard")]
    Standard,
    /// IA
    #[serde(rename = "IA")]
    IA,
    /// Archive
    #[serde(rename = "Archive")]
    Archive,
    /// ColdArchive
    #[serde(rename = "ColdArchive")]
    ColdArchive,
    #[serde(rename = "DeepColdArchive")]
    DeepColdArchive,
}

impl Default for StorageClass {
    fn default() -> Self {
        Self::Standard
    }
}

impl StorageClass {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::IA => "IA",
            Self::Archive => "Archive",
            Self::ColdArchive => "ColdArchive",
            Self::DeepColdArchive => "DeepColdArchive",
        }
    }
}

impl std::fmt::Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a StorageClass> for crate::QueryValue<'a> {
    fn from(value: &'a StorageClass) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for StorageClass {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BucketACL {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public-read")]
    PublicRead,
    #[serde(rename = "public-read-write")]
    PublicReadWrite,
}

impl Default for BucketACL {
    fn default() -> Self {
        Self::Private
    }
}

impl BucketACL {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::PublicRead => "public-read",
            Self::PublicReadWrite => "public-read-write",
        }
    }
}

impl std::fmt::Display for BucketACL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a BucketACL> for crate::QueryValue<'a> {
    fn from(value: &'a BucketACL) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for BucketACL {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DataRedundancyType {
    #[serde(rename = "LRS")]
    LRS,
    #[serde(rename = "ZRS")]
    ZRS,
}

impl Default for DataRedundancyType {
    fn default() -> Self {
        Self::LRS
    }
}

impl DataRedundancyType {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LRS => "LRS",
            Self::ZRS => "ZRS",
        }
    }
}

impl std::fmt::Display for DataRedundancyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a DataRedundancyType> for crate::QueryValue<'a> {
    fn from(value: &'a DataRedundancyType) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for DataRedundancyType {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BucketVersioningStatus {
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "Suspended")]
    Suspended,
}

impl Default for BucketVersioningStatus {
    fn default() -> Self {
        Self::Enabled
    }
}

impl BucketVersioningStatus {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Suspended => "Suspended",
        }
    }
}

impl std::fmt::Display for BucketVersioningStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a BucketVersioningStatus> for crate::QueryValue<'a> {
    fn from(value: &'a BucketVersioningStatus) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for BucketVersioningStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BucketWormState {
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Locked")]
    Locked,
}

impl Default for BucketWormState {
    fn default() -> Self {
        Self::InProgress
    }
}

impl BucketWormState {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InProgress => "InProgress",
            Self::Locked => "Locked",
        }
    }
}

impl std::fmt::Display for BucketWormState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a BucketWormState> for crate::QueryValue<'a> {
    fn from(value: &'a BucketWormState) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for BucketWormState {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FileHeaderInfo {
    #[serde(rename = "USE")]
    USE,
    #[serde(rename = "IGNORE")]
    IGNORE,
    #[serde(rename = "NONE")]
    NONE,
}

impl Default for FileHeaderInfo {
    fn default() -> Self {
        Self::USE
    }
}

impl FileHeaderInfo {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::USE => "USE",
            Self::IGNORE => "IGNORE",
            Self::NONE => "NONE",
        }
    }
}

impl std::fmt::Display for FileHeaderInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a FileHeaderInfo> for crate::QueryValue<'a> {
    fn from(value: &'a FileHeaderInfo) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for FileHeaderInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CompressionType {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "GZIP")]
    GZIP,
}

impl Default for CompressionType {
    fn default() -> Self {
        Self::None
    }
}

impl CompressionType {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::GZIP => "GZIP",
        }
    }
}

impl std::fmt::Display for CompressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a CompressionType> for crate::QueryValue<'a> {
    fn from(value: &'a CompressionType) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for CompressionType {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EncodeType {
    #[serde(rename = "url")]
    Url,
}

impl Default for EncodeType {
    fn default() -> Self {
        Self::Url
    }
}

impl EncodeType {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Url => "url",
        }
    }
}

impl std::fmt::Display for EncodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a EncodeType> for crate::QueryValue<'a> {
    fn from(value: &'a EncodeType) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for EncodeType {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum JSONType {
    /// DOCUMENT
    #[serde(rename = "DOCUMENT")]
    DOCUMENT,
    /// LINES
    #[serde(rename = "LINES")]
    LINES,
}

impl Default for JSONType {
    fn default() -> Self {
        Self::DOCUMENT
    }
}

impl JSONType {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DOCUMENT => "DOCUMENT",
            Self::LINES => "LINES",
        }
    }
}

impl std::fmt::Display for JSONType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a JSONType> for crate::QueryValue<'a> {
    fn from(value: &'a JSONType) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for JSONType {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InventoryFormat {
    #[serde(rename = "CSV")]
    CSV,
}

impl Default for InventoryFormat {
    fn default() -> Self {
        Self::CSV
    }
}

impl InventoryFormat {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CSV => "CSV",
        }
    }
}

impl std::fmt::Display for InventoryFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a InventoryFormat> for crate::QueryValue<'a> {
    fn from(value: &'a InventoryFormat) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for InventoryFormat {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InventoryFrequency {
    /// Daily
    #[serde(rename = "Daily")]
    Daily,
    /// Weekly
    #[serde(rename = "Weekly")]
    Weekly,
}

impl Default for InventoryFrequency {
    fn default() -> Self {
        Self::Daily
    }
}

impl InventoryFrequency {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Daily => "Daily",
            Self::Weekly => "Weekly",
        }
    }
}

impl std::fmt::Display for InventoryFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a InventoryFrequency> for crate::QueryValue<'a> {
    fn from(value: &'a InventoryFrequency) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for InventoryFrequency {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InventoryOptionalField {
    #[serde(rename = "Size")]
    Size,
    #[serde(rename = "LastModifiedDate")]
    LastModifiedDate,
    #[serde(rename = "ETag")]
    ETag,
    #[serde(rename = "StorageClass")]
    StorageClass,
    #[serde(rename = "IsMultipartUploaded")]
    IsMultipartUploaded,
    #[serde(rename = "EncryptionStatus")]
    EncryptionStatus,
}

impl Default for InventoryOptionalField {
    fn default() -> Self {
        Self::Size
    }
}

impl InventoryOptionalField {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Size => "Size",
            Self::LastModifiedDate => "LastModifiedDate",
            Self::ETag => "ETag",
            Self::StorageClass => "StorageClass",
            Self::IsMultipartUploaded => "IsMultipartUploaded",
            Self::EncryptionStatus => "EncryptionStatus",
        }
    }
}

impl std::fmt::Display for InventoryOptionalField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a InventoryOptionalField> for crate::QueryValue<'a> {
    fn from(value: &'a InventoryOptionalField) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for InventoryOptionalField {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InventoryIncludedObjectVersions {
    /// All
    #[serde(rename = "All")]
    All,
    /// Current
    #[serde(rename = "Current")]
    Current,
}

impl Default for InventoryIncludedObjectVersions {
    fn default() -> Self {
        Self::All
    }
}

impl InventoryIncludedObjectVersions {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Current => "Current",
        }
    }
}

impl std::fmt::Display for InventoryIncludedObjectVersions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a InventoryIncludedObjectVersions> for crate::QueryValue<'a> {
    fn from(value: &'a InventoryIncludedObjectVersions) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for InventoryIncludedObjectVersions {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MetaQueryOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for MetaQueryOrder {
    fn default() -> Self {
        Self::Asc
    }
}

impl MetaQueryOrder {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

impl std::fmt::Display for MetaQueryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a MetaQueryOrder> for crate::QueryValue<'a> {
    fn from(value: &'a MetaQueryOrder) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for MetaQueryOrder {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReplicationDestinationTransferType {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "oss_acc")]
    OssAcc,
}

impl Default for ReplicationDestinationTransferType {
    fn default() -> Self {
        Self::Internal
    }
}

impl ReplicationDestinationTransferType {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::OssAcc => "oss_acc",
        }
    }
}

impl std::fmt::Display for ReplicationDestinationTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a ReplicationDestinationTransferType> for crate::QueryValue<'a> {
    fn from(value: &'a ReplicationDestinationTransferType) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for ReplicationDestinationTransferType {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PutReplicationRuleHistoricalObjectReplication {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for PutReplicationRuleHistoricalObjectReplication {
    fn default() -> Self {
        Self::Enabled
    }
}

impl PutReplicationRuleHistoricalObjectReplication {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        }
    }
}

impl std::fmt::Display for PutReplicationRuleHistoricalObjectReplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a PutReplicationRuleHistoricalObjectReplication> for crate::QueryValue<'a> {
    fn from(value: &'a PutReplicationRuleHistoricalObjectReplication) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for PutReplicationRuleHistoricalObjectReplication {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReplicationSourceSelectionCriteriaSseKmsEncryptedObjectsStatus {
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "Disabled")]
    Disabled,
}

impl Default for ReplicationSourceSelectionCriteriaSseKmsEncryptedObjectsStatus {
    fn default() -> Self {
        Self::Enabled
    }
}

impl ReplicationSourceSelectionCriteriaSseKmsEncryptedObjectsStatus {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}

impl std::fmt::Display for ReplicationSourceSelectionCriteriaSseKmsEncryptedObjectsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a ReplicationSourceSelectionCriteriaSseKmsEncryptedObjectsStatus>
    for crate::QueryValue<'a>
{
    fn from(value: &'a ReplicationSourceSelectionCriteriaSseKmsEncryptedObjectsStatus) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for ReplicationSourceSelectionCriteriaSseKmsEncryptedObjectsStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReplicationRuleHistoricalObjectReplication {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for ReplicationRuleHistoricalObjectReplication {
    fn default() -> Self {
        Self::Enabled
    }
}

impl ReplicationRuleHistoricalObjectReplication {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        }
    }
}

impl std::fmt::Display for ReplicationRuleHistoricalObjectReplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a ReplicationRuleHistoricalObjectReplication> for crate::QueryValue<'a> {
    fn from(value: &'a ReplicationRuleHistoricalObjectReplication) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for ReplicationRuleHistoricalObjectReplication {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        params.push((name.to_string().into(), self.into()));
    }
}

/// - 您也可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
/// - 要列举存储空间，您必须有oss:GetService (ListBuckets)权限。具体操作，请参见[为RAM用户授权自定义的权限策略](https://help.aliyun.com/document_detail/199058.htm?spm=a2c4g.11186623.0.0.48be7590hCA8LI#section-ucu-jv0-zip)。
/// - 调用接口时，如果所有Bucket已返回，则返回参数的XML中不包含Prefix、Marker、MaxKeys、IsTruncated和NextMarker响应元素。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存Bucket拥有者信息的容器。
    #[serde(rename = "Owner")]
    #[serde(default)]
    pub owner: Owner,
    /// 本次查询结果的前缀。
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
    /// 本次ListBuckets（GetSerivce）的起点。
    #[serde(rename = "Marker")]
    #[serde(default)]
    pub marker: String,
    /// 响应请求内返回结果的最大数。
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    pub max_keys: i64,
    /// 是否所有的结果都已经返回。取值范围如下：
    /// - true：表示本次没有返回全部结果。
    /// - false：表示本次已经返回了全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 用于继续查询时给marker赋值。表示下一次ListBuckets（GetService）可以以此为marker，将未返回的结果返回。
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    pub next_marker: String,
    /// 保存Bucket信息列表的容器。
    #[serde(rename = "Buckets")]
    #[serde(default)]
    pub buckets: ResponseBuckets,
}

impl crate::ToCodeMessage for ListBucketsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 只支持在二级域名（例如oss-cn-hangzhou.aliyuncs.com）上调用DescribeRegions接口。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存多个地域信息的容器。
    #[serde(rename = "RegionInfo")]
    #[serde(default)]
    pub region_info: Vec<RegionInfo>,
}

impl crate::ToCodeMessage for DescribeRegionsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - GetBucket (ListObjects)接口已修订为GetBucketV2 (ListObjectsV2)。建议您在开发应用程序时使用较新的版本GetBucketV2 (ListObjectsV2)。为保证向后兼容性，OSS继续支持GetBucket (ListObjects)。有关GetBucketV2 (ListObjectsV2)的更多信息，请参见[GetBucketV2 (ListObjectsV2)](~~187544~~)。
///
/// - 执行GetBucket (ListObjects)请求时不会返回Object中自定义的元信息。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Bucket名称。
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    /// 本次查询结果的前缀。
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
    /// 标识此次GetBucket（ListObjects）的起点。
    #[serde(rename = "Marker")]
    #[serde(default)]
    pub marker: String,
    /// 响应请求内返回结果的最大数目。
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    pub max_keys: i32,
    /// 对Object名字进行分组的字符。所有名字包含指定的前缀且第一次出现Delimiter字符之间的Object作为一组元素CommonPrefixes。
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    pub delimiter: String,
    /// 请求中返回的结果是否被截断。
    ///
    /// 返回值：true、false
    ///
    /// true表示本次没有返回全部结果。
    ///
    /// false表示本次已经返回了全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 指明了返回结果中编码使用的类型。如果请求的参数中指定了encoding-type，则会对返回结果中的Delimiter、Marker、Prefix、NextMarker和Key这些元素进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    pub encoding_type: String,
    /// 下一次列举文件的起点。
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    pub next_marker: String,
    /// 保存每个返回Object元数据的容器。
    #[serde(rename = "Contents")]
    #[serde(default)]
    pub contents: Vec<ObjectSummary>,
    /// 如果请求中指定了Delimiter参数，则会在返回的响应中包含CommonPrefixes元素。该元素表明以Delimiter结尾，并有共同前缀的Object名称的集合。
    #[serde(rename = "CommonPrefixes")]
    #[serde(default)]
    pub common_prefixes: Vec<CommonPrefix>,
}

impl crate::ToCodeMessage for ListObjectsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 执行GetBucketV2 (ListObjectsV2)请求时不会返回Object中自定义的元信息。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectsV2Response {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Bucket名称。
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    /// 本次查询结果的前缀。
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
    /// 如果请求中指定了StartAfter参数，则会在返回的响应中包含StartAfter元素。
    #[serde(rename = "StartAfter")]
    #[serde(default)]
    pub start_after: String,
    /// 响应请求内返回结果的最大数目。
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    pub max_keys: i32,
    /// 对Object名字进行分组的字符。所有名字包含指定的前缀且第一次出现Delimiter字符之间的Object作为一组元素CommonPrefixes。
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    pub delimiter: String,
    /// 请求中返回的结果是否被截断。
    ///
    /// 返回值：true、false
    ///
    /// true表示本次没有返回全部结果。
    ///
    /// false表示本次已经返回了全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 此次请求返回的Key的个数。如果指定了Delimiter，则KeyCount为Key和CommonPrefixes的元素之和。
    #[serde(rename = "KeyCount")]
    #[serde(default)]
    pub key_count: i32,
    /// 指明返回结果中编码使用的类型。如果请求的参数中指定了Encoding-type，则会对返回结果中的Delimiter、StartAfter、Prefix、NextContinuationToken和Key这些元素进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    pub encoding_type: String,
    /// 如果请求中指定了ContinuationToken参数，则会在返回的响应中包含ContinuationToken元素。
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    pub continuation_token: String,
    /// 表明此次ListObjectsV2（GetBucketV2）请求包含后续结果，需要将NextContinuationToken指定为ContinuationToken继续获取结果。
    #[serde(rename = "NextContinuationToken")]
    #[serde(default)]
    pub next_continuation_token: String,
    /// 保存每个返回Object元数据的容器。
    #[serde(rename = "Contents")]
    #[serde(default)]
    pub contents: Vec<ObjectSummary>,
    /// 如果请求中指定了Delimiter参数，则会在返回的响应中包含CommonPrefixes元素。该元素表明以Delimiter结尾，并有共同前缀的Object名称的集合。
    #[serde(rename = "CommonPrefixes")]
    #[serde(default)]
    pub common_prefixes: Vec<CommonPrefix>,
}

impl crate::ToCodeMessage for ListObjectsV2Response {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketLocationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetBucketLocationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointPolicyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetAccessPointPolicyResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 对象存储OSS支持WORM（Write Once Read Many）特性，允许以不可删除、不可篡改的方式保存和使用数据。OSS允许针对存储空间（Bucket）设置基于时间的合规保留策略，保护周期为1天到70年。
///
/// - 当基于时间的合规保留策略创建24小时后未提交锁定，则该策略自动失效。当合规保留策略锁定后，您可以在Bucket中上传和读取文件（Object），但是在Object的保留时间到期之前，不允许删除Object及合规保留策略。Object的保留时间到期后，才可以删除Object。关于合规保留策略的更多信息，请参见[合规保留策略](~~90564~~)。
///
/// - 同一个Bucket中，版本控制和合规保留策略无法同时配置。如果Bucket已开启版本控制功能，则无法再配置保留策略。关于版本控制功能更多信息，请参见[版本控制介绍](~~109685~~)。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct InitiateBucketWormResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-worm-id
    #[serde(skip)]
    pub x_oss_worm_id: Option<String>,
}

impl crate::ToCodeMessage for InitiateBucketWormResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 对象存储OSS支持WORM（Write Once Read Many）特性，允许您以不可删除、不可篡改的方式保存和使用数据。OSS允许针对Bucket设置基于时间的合规保留策略，保护周期为1天到70年。<br>
/// 当合规保留策略锁定后，您可以在Bucket中上传和读取文件（Object），但是在Object的保留时间到期之前，不允许删除Object及合规保留策略。Object的保留时间到期后，才可以删除Object。
///
///
/// > 若指定用来获取Bucket的合规保留策略信息对应的WORM ID不存在，则返回404。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketWormResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 合规保留策略的ID。
    #[serde(rename = "WormId")]
    #[serde(default)]
    pub worm_id: String,
    /// 合规保留策略所处的状态。可选值：
    ///
    /// InProgress：合规保留策略创建后，该策略默认处于“InProgress”状态，且该状态的有效期为24小时。
    ///
    /// Locked：合规保留策略处于锁定状态。
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: BucketWormState,
    /// Object的指定保留天数。
    #[serde(rename = "RetentionPeriodInDays")]
    #[serde(default)]
    pub retention_period_in_days: i32,
    /// 合规保留策略的创建时间。
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    pub creation_date: String,
    /// 合规保留策略的过期时间。
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    pub expiration_date: String,
}

impl crate::ToCodeMessage for GetBucketWormResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketAclResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存Bucket拥有者信息的容器。
    #[serde(rename = "Owner")]
    #[serde(default)]
    pub owner: Owner,
    /// 存储ACL信息的容器类。
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    pub access_control_list: BucketAclResponseAccessControlList,
}

impl crate::ToCodeMessage for GetBucketAclResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - 只有Bucket拥有者以及被授予oss:GetBucketTransferAcceleration权限的RAM用户才能发起获取传输加速配置的请求。
///
/// - 如果Bucket未配置过传输加速，调用该接口时不返回加速配置状态。
///
/// 有关传输加速的更多信息，请参见开发指南的[传输加速](~~131312~~)。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketTransferAccelerationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 是否开启传出加速。
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

impl crate::ToCodeMessage for GetBucketTransferAccelerationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketVersioningResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 版本控制状态。
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: BucketVersioningStatus,
}

impl crate::ToCodeMessage for GetBucketVersioningResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - GetBucket(ListObjects)接口仅返回Object的当前版本，且当前版本不能为删除标记。
/// - GetBucketVersions(ListObjectVersions)接口返回Bucket中所有Object的所有版本。
///
/// 不同Object之间按字母排序返回，同一个Object的不同版本则按从新到旧排序，与版本ID的字母序无关。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectVersionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Bucket名称。
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    /// 本次查询结果的前缀。
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
    /// 标识此次GetBucketVersions的起点Object。
    #[serde(rename = "KeyMarker")]
    #[serde(default)]
    pub key_marker: String,
    /// 与KeyMarker参数一同使用，以指定ListObjectVersions（GetBucketVersions）的起点。
    #[serde(rename = "VersionIdMarker")]
    #[serde(default)]
    pub version_id_marker: String,
    /// 响应请求内返回结果的最大数目。
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    pub max_keys: i64,
    /// 用于对Object名字进行分组的字符。所有名字包含指定的前缀且第一次出现Delimiter字符之间的Object作为一组元素CommonPrefixes。
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    pub delimiter: String,
    /// 指明是否已返回所有结果。
    ///
    /// true：表示本次没有返回全部结果。
    ///
    /// false：表示本次已返回全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 指明返回结果中编码使用的类型。如果请求的参数中指定了encoding-type，则表示对返回结果中的Delimiter、Marker、Prefix、NextMarker和Key这些元素进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    pub encoding_type: String,
    /// 如果本次没有返回全部结果，响应请求中将包含NextKeyMarker元素，用于标明接下来请求的key-marker。
    #[serde(rename = "NextKeyMarker")]
    #[serde(default)]
    pub next_key_marker: String,
    /// 如果本次没有返回全部结果，响应请求中将包含NextVersionIdMarker元素，用于标明接下来请求的version-id-marker。
    #[serde(rename = "NextVersionIdMarker")]
    #[serde(default)]
    pub next_version_id_marker: String,
    /// 保存除删除标记以外的Object版本信息的列表。
    #[serde(rename = "Version")]
    #[serde(default)]
    pub version: Vec<ObjectVersion>,
    /// 保存删除标记信息的列表。
    #[serde(rename = "DeleteMarker")]
    #[serde(default)]
    pub delete_marker: Vec<DeleteMarkerEntry>,
    /// 如果请求中指定了delimiter参数，则OSS返回的响应中包含CommonPrefixes元素。该元素标明以delimiter结尾，并有共同前缀的Object名称的集合。
    #[serde(rename = "CommonPrefixes")]
    #[serde(default)]
    pub common_prefixes: Vec<CommonPrefix>,
}

impl crate::ToCodeMessage for ListObjectVersionsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - 向其他账号的RAM用户授权访问。
///
/// 您可以授予其他账号的RAM用户访问您的OSS资源的权限。
///
/// - 向匿名用户授予带特定IP条件限制的访问权限。
///
/// 某些场景下，您需要向匿名用户授予带IP限制的访问策略。例如，企业内部的机密文档，只允许在企业内部访问，不允许在其他区域访问。由于企业内部人员较多，如果针对每个人配置RAM Policy，工作量非常大。此时，您可以基于Bucket Policy设置带IP限制的访问策略，从而高效方便地进行授权。
///
/// 有关Bucket Policy的配置详情及场景案例，请参见[使用Bucket Policy授权其他用户访问OSS资源](~~85111~~)。有关Policy语法，请参见[权限策略语法和结构](~~93739~~)。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketPolicyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetBucketPolicyResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 阿里云账号默认拥有查看当前Bucket Policy是否允许公共访问的权限。如果您希望通过RAM用户或者STS的方式进行查看，您必须拥有`oss:GetBucketPolicyStatus`权限。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketPolicyStatusResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 当前Bucket Policy是否包含公共访问的语义。
    /// - true：包含公共访问的语义。
    /// - false：不包含公共访问的语义或者没有设置Bucket Policy。
    #[serde(rename = "IsPublic")]
    #[serde(default)]
    pub is_public: bool,
}

impl crate::ToCodeMessage for GetBucketPolicyStatusResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 数据复制是以异步（近实时）方式将源Bucket中的文件（Object）以及对Object的创建、更新和删除等操作自动复制到目标Bucket。使用数据复制时，有如下注意事项：
///
/// - 数据复制采用异步复制，数据复制到目标Bucket需要一定的时间，通常几分钟到几小时不等，具体取决于数据的大小。
///
/// - 源Bucket与目标Bucket的名称不能相同。
///
/// - 使用跨区域复制时，源Bucket与目标Bucket必须处于不同的数据中心；使用同地域复制时，源Bucket与目标Bucket必须处于相同的数据中心。
///
/// 关于数据复制的更多信息，请分别参见[跨区域复制介绍](https://help.aliyun.com/document_detail/31864.htm?spm=a2c4g.11186623.0.0.32af6265m8tpXg#concept-zjp-31z-5db)和[同区域复制介绍](https://help.aliyun.com/document_detail/254865.htm?spm=a2c4g.11186623.0.0.32af6265m8tpXg#concept-2067125)。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketReplicationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-replication-rule-id
    #[serde(skip)]
    pub x_oss_replication_rule_id: Option<String>,
}

impl crate::ToCodeMessage for PutBucketReplicationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存复制规则的容器。
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: Vec<ReplicationRule>,
}

impl crate::ToCodeMessage for GetBucketReplicationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationLocationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 可复制到的目标Bucket所在的地域列表。
    ///
    /// > 如果有多个可复制到的目标地域，那么返回的结果中包含多个Location。如果没有可复制到的目标地域，则返回的Location为空。
    #[serde(rename = "Location")]
    #[serde(default)]
    pub location: Vec<String>,
    /// 包含TransferType约束的Location信息容器。
    #[serde(rename = "LocationTransferTypeConstraint")]
    #[serde(default)]
    pub location_transfer_type_constraint: TransferTypeConstraint,
    /// 包含RTC约束的Location信息容器。
    #[serde(rename = "LocationRTCConstraint")]
    #[serde(default)]
    pub location_rtc_constraint: CConstraint,
}

impl crate::ToCodeMessage for GetBucketReplicationLocationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketReplicationProgressResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存复制规则进度信息的容器。
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: Vec<ReplicationProgressRule>,
}

impl crate::ToCodeMessage for GetBucketReplicationProgressResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - 单次请求最多可获取100条清单配置项内容。若需获取超过100条清单配置项，则需发送多次请求，并保留相应的token，作为下一次请求的参数。
///
/// - 调用该请求时，请确保您有足够的权限对存储空间的清单任务进行操作。存储空间所有者默认拥有该权限，若您无该项权限，请先向存储空间所有者申请该项操作的权限。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketInventoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 存放清单配置参数的容器。
    #[serde(rename = "InventoryConfiguration")]
    #[serde(default)]
    pub inventory_configuration: Vec<InventoryConfiguration>,
    /// 是否列举全部的清单任务。
    ///
    /// 有效值：true或false
    ///
    /// 如果值为false，则表明存储空间中的所有清单任务已全部列出。
    ///
    /// 如果值为true，表示还未完整列出存储空间中的所有清单任务，您可以将NextContinuationToken字段的值作为下一次list请求的continuation-token参数，以获取下一页的清单配置列表。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 当响应中的IsTruncated为true且NextContinuationToken非空时，使用该字段作为下一次list请求的continuation-token参数。
    #[serde(rename = "NextContinuationToken")]
    #[serde(default)]
    pub next_continuation_token: String,
}

impl crate::ToCodeMessage for ListBucketInventoryResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketTagsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存标签集合的容器。
    #[serde(rename = "TagSet")]
    #[serde(default)]
    pub tag_set: TagSet,
}

impl crate::ToCodeMessage for GetBucketTagsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListUserDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求中返回的结果是否被截断。取值如下：
    ///
    /// true：表示本次未返回全部结果。
    ///
    /// false：表示本次已返回全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 表明本次ListUserDataRedundancyTransition请求包含后续结果，需要将NextContinuationToken指定为continuation-token继续获取结果。
    #[serde(rename = "NextContinuationToken")]
    #[serde(default)]
    pub next_continuation_token: String,
    /// 存储冗余转换任务的容器。
    #[serde(rename = "BucketDataRedundancyTransition")]
    #[serde(default)]
    pub bucket_data_redundancy_transition: Vec<BucketDataRedundancyTransition>,
}

impl crate::ToCodeMessage for ListUserDataRedundancyTransitionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - 要列举Bucket下所有的存储冗余转换任务，您必须具有`oss:ListBucketDataRedundancyTransition`权限。具体操作，请参见[为RAM用户授权自定义的权限策略](~~199058~~)。
/// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](~~31837~~)。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 存储冗余转换任务的信息。
    #[serde(rename = "BucketDataRedundancyTransition")]
    #[serde(default)]
    pub bucket_data_redundancy_transition: BucketDataRedundancyTransition,
}

impl crate::ToCodeMessage for ListBucketDataRedundancyTransitionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - Bucket所属地域支持转换存储冗余类型。支持转换存储冗余类型的地域：华东1（杭州）、华东2（上海）、华北2（北京）、华北3（张家口）、华北6（乌兰察布）、华南1（深圳）、中国香港、日本（东京）、新加坡、印度尼西亚（雅加达）、德国（法兰克福）
/// - Bucket的存储冗余类型必须为本地冗余存储。OSS仅支持将本地冗余存储转换为同城冗余存储。
/// - Bucket的存储类型必须为标准存储、低频访问存储或归档存储，但Bucket中的文件的存储类型可以为冷归档存储和深度冷归档存储。冷归档存储和深度冷归档存储的文件转换后依然为本地冗余存储。冷归档存储和深度冷归档存储的Bucket不支持转换存储冗余类型。
/// - 要创建存储冗余转换任务，您必须具有oss:CreateBucketDataRedundancyTransition权限。具体操作，请参见为[RAM用户授权自定义的权限策略](~~199058~~)。
/// - 每个地域都有对应的访问域名（Endpoint）。关于地域与访问域名对应关系的更多信息，请参见[访问域名和数据中心](~~31837~~)。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateBucketDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 存储冗余转换任务的ID。该ID可以用于后续查看、删除存储冗余转换任务。
    #[serde(rename = "TaskId")]
    #[serde(default)]
    pub task_id: String,
}

impl crate::ToCodeMessage for CreateBucketDataRedundancyTransitionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 只有Bucket的拥有者及授权的RAM用户才能获取Bucket的加密规则，否则返回403错误。有关Bucket加密的更多信息，请参见**[服务器端加密](~~31871~~)**。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketEncryptionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 服务端加密规则信息。
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    #[serde(default)]
    pub apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault,
}

impl crate::ToCodeMessage for GetBucketEncryptionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketRequestPaymentResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 指定Bucket付费类型。
    ///
    /// 取值：BucketOwner、Requester
    #[serde(rename = "Payer")]
    #[serde(default)]
    pub payer: String,
}

impl crate::ToCodeMessage for GetBucketRequestPaymentResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

///  
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketCorsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存CORS规则的列表。
    #[serde(rename = "CORSRule")]
    #[serde(default)]
    pub cors_rule: Vec<CORSRule>,
    /// 是否返回Vary: Origin头。
    ///
    /// true：不管发送的是否是跨域请求或跨域请求是否成功，均会返回Vary: Origin头。
    ///
    /// false：任何情况下均不返回Vary: Origin头。
    #[serde(rename = "ResponseVary")]
    #[serde(default)]
    pub response_vary: bool,
}

impl crate::ToCodeMessage for GetBucketCorsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// Options请求是由浏览器自动根据是否跨域来决定是否发送。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct OptionObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: Access-Control-Allow-Origin
    #[serde(skip)]
    pub access_control_allow_origin: Option<String>,
    /// Header field from response: Access-Control-Allow-Methods
    #[serde(skip)]
    pub access_control_allow_methods: Option<String>,
    /// Header field from response: Access-Control-Allow-Headers
    #[serde(skip)]
    pub access_control_allow_headers: Option<String>,
    /// Header field from response: Access-Control-Expose-Headers
    #[serde(skip)]
    pub access_control_expose_headers: Option<String>,
    /// Header field from response: Access-Control-Max-Age
    #[serde(skip)]
    pub access_control_max_age: Option<i64>,
}

impl crate::ToCodeMessage for OptionObjectResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetMetaQueryStatusResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 元数据索引库的状态。取值范围如下：
    ///
    /// Ready：创建后准备中
    ///
    /// 元数据索引库正在准备中，您无法通过元数据索引库查询到数据。
    ///
    /// Stop：已暂停
    ///
    /// Running：运行中
    ///
    /// Retrying：创建失败后重试中
    ///
    /// Failed：创建失败
    ///
    /// Deleted：已删除
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
    /// 当前扫描类型。取值范围如下：
    ///
    /// FullScanning：全量扫描中
    ///
    /// IncrementalScanning：增量扫描中
    #[serde(rename = "Phase")]
    #[serde(default)]
    pub phase: String,
    /// 元数据索引库的创建时间，遵循RFC 3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE。其中YYYY-MM-DD表示年月日，T表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区。
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    pub create_time: String,
    /// 元数据索引库的创建时间，遵循RFC 3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE。其中YYYY-MM-DD表示年月日，T表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区。
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    pub update_time: String,
}

impl crate::ToCodeMessage for GetMetaQueryStatusResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 返回字母排序在指定marker之后的高防实例。
    #[serde(rename = "Marker")]
    #[serde(default)]
    pub marker: String,
    /// 是否已返回所有高防实例。
    ///
    /// true：本次请求未返回所有高防实例。
    ///
    /// false：本次请求已返回所有高防实例。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 保存高防实例信息的列表。
    #[serde(rename = "AntiDDOSConfiguration")]
    #[serde(default)]
    pub anti_ddos_configuration: Vec<BucketAntiDDOSInfo>,
}

impl crate::ToCodeMessage for ListBucketAntiDDosInfoResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct InitUserAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-defender-instance
    #[serde(skip)]
    pub x_oss_defender_instance: Option<String>,
}

impl crate::ToCodeMessage for InitUserAntiDDosInfoResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetUserAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存高防实例信息的容器。
    #[serde(rename = "AntiDDOSConfiguration")]
    #[serde(default)]
    pub anti_ddos_configuration: Vec<UserAntiDDOSInfo>,
}

impl crate::ToCodeMessage for GetUserAntiDDosInfoResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketResourceGroupResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Bucket所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(default)]
    pub resource_group_id: String,
}

impl crate::ToCodeMessage for GetBucketResourceGroupResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListCnameResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 存储空间名称。
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    /// Bucket拥有者的用户ID。
    #[serde(rename = "Owner")]
    #[serde(default)]
    pub owner: String,
    /// 保存Cname信息的列表。
    #[serde(rename = "Cname")]
    #[serde(default)]
    pub cname: Vec<CnameInfo>,
}

impl crate::ToCodeMessage for ListCnameResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListStyleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存图片样式信息的列表。
    #[serde(rename = "Style")]
    #[serde(default)]
    pub style: Vec<StyleInfo>,
}

impl crate::ToCodeMessage for ListStyleResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - 阿里云账号默认拥有创建对象FC接入点的权限。如果您希望通过RAM用户或者STS的方式创建对象FC接入点，您必须拥有`oss:CreateAccessPointForObjectProcess`权限。
/// - 单个阿里云账号支持创建1000个对象FC接入点。
/// - 单个Bucket支持创建100个对象FC接入点。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 对象FC接入点ARN。
    #[serde(rename = "AccessPointForObjectProcessArn")]
    #[serde(default)]
    pub access_point_for_object_process_arn: String,
    /// 对象FC接入点别名。
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    #[serde(default)]
    pub access_point_for_object_process_alias: String,
}

impl crate::ToCodeMessage for CreateAccessPointForObjectProcessResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 阿里云账号默认拥有获取对象FC接入点基础信息的权限。如果您希望通过RAM用户或者STS的方式获取对象FC接入点基础信息，您必须拥有`oss:GetAccessPointForObjectProcess`权限。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 对象FC接入点名称。
    #[serde(rename = "AccessPointNameForObjectProcess")]
    #[serde(default)]
    pub access_point_name_for_object_process: String,
    /// 对象FC接入点别名。
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    #[serde(default)]
    pub access_point_for_object_process_alias: String,
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    #[serde(default)]
    pub access_point_name: String,
    /// 配置对象FC接入点的阿里云账号UID。
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    /// 对象FC接入点ARN。
    #[serde(rename = "AccessPointForObjectProcessArn")]
    #[serde(default)]
    pub access_point_for_object_process_arn: String,
    /// 对象FC接入点创建时间，格式为时间戳。
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    pub creation_date: String,
    /// 对象FC接入点所处状态。返回值如下：
    /// - enable：对象FC接入点已创建完成。
    /// - disable：对象FC接入点已禁用。
    /// - creating：对象FC接入点正在创建中。
    /// - deleting：对象FC接入点已删除。
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    /// 保存对象FC接入点访问域名信息的容器。
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    pub endpoints: ResponseEndpoints,
    /// 是否允许匿名访问。
    ///
    /// > 开启该选项之后将会允许匿名账号通过ObjectFC接入点访问您的Bucket，会产生相关费用。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    #[serde(default)]
    pub allow_anonymous_access_for_object_process: String,
    /// 保存阻止公共访问信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::ToCodeMessage for GetAccessPointForObjectProcessResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 阿里云账号默认拥有获取对象FC接入点信息的权限。如果您希望通过RAM用户或者STS的方式获取对象FC接入点信息，您必须拥有`oss:ListAccessPointsForObjectProcess`权限。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求中返回的结果是否被截断。返回值如下：
    ///
    /// - true：表示本次未返回全部结果。
    /// - false：表示本次已返回全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 表明本次ListAccessPointsForObjectProcess请求包含后续结果，需要将NextContinuationToken指定为continuation-token继续获取结果。
    #[serde(rename = "NextContinuationToken")]
    #[serde(default)]
    pub next_continuation_token: String,
    /// 对象FC接入点所属的阿里云账号UID。
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    /// 保存所有对象FC接入点信息的容器。
    #[serde(rename = "AccessPointsForObjectProcess")]
    #[serde(default)]
    pub access_points_for_object_process: AccessPointsForObjectProcess,
}

impl crate::ToCodeMessage for ListAccessPointsForObjectProcessResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 阿里云账号默认拥有获取对象FC接入点配置信息的权限。如果您希望通过RAM用户或者STS的方式获取对象FC接入点配置信息，您必须拥有`oss:GetAccessPointConfigForObjectProcess`权限。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointConfigForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存对象处理信息的容器。
    #[serde(rename = "ObjectProcessConfiguration")]
    #[serde(default)]
    pub object_process_configuration: ObjectProcessConfiguration,
    /// 是否允许匿名访问。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    #[serde(default)]
    pub allow_anonymous_access_for_object_process: String,
    /// 保存阻止公共访问信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::ToCodeMessage for GetAccessPointConfigForObjectProcessResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 阿里云账号默认拥有获取对象FC接入点权限策略配置的权限。如果您希望通过RAM用户或者STS的方式获取对象FC接入点的权限策略配置，您必须拥有`oss:GetAccessPointPolicyForObjectProcess`权限。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointPolicyForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetAccessPointPolicyForObjectProcessResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutAccessPointPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for PutAccessPointPublicAccessBlockResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutObjectResponse {
    /// Header field from response: x-oss-hash-crc64ecma
    #[serde(skip)]
    pub x_oss_hash_crc64ecma: Option<i64>,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for PutObjectResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CopyObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 目标Object的ETag值。
    #[serde(rename = "ETag")]
    #[serde(default)]
    pub e_tag: String,
    /// 目标Object最后更新时间。
    #[serde(rename = "LastModified")]
    #[serde(default)]
    pub last_modified: String,
    /// Header field from response: x-oss-copy-source-version-id
    #[serde(skip)]
    pub x_oss_copy_source_version_id: Option<String>,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for CopyObjectResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectResponse {
    pub body: Vec<u8>,
    /// Header field from response: x-oss-server-side-encryption
    #[serde(skip)]
    pub x_oss_server_side_encryption: Option<String>,
    /// Header field from response: x-oss-server-side-encryption-key-id
    #[serde(skip)]
    pub x_oss_server_side_encryption_key_id: Option<String>,
    /// Header field from response: x-oss-storage-class
    #[serde(skip)]
    pub x_oss_storage_class: Option<String>,
    /// Header field from response: x-oss-object-type
    #[serde(skip)]
    pub x_oss_object_type: Option<String>,
    /// Header field from response: x-oss-next-append-position
    #[serde(skip)]
    pub x_oss_next_append_position: Option<i64>,
    /// Header field from response: x-oss-hash-crc64ecma
    #[serde(skip)]
    pub x_oss_hash_crc64ecma: Option<String>,
    /// Header field from response: x-oss-expiration
    #[serde(skip)]
    pub x_oss_expiration: Option<String>,
    /// Header field from response: x-oss-restore
    #[serde(skip)]
    pub x_oss_restore: Option<String>,
    /// Header field from response: x-oss-process-status
    #[serde(skip)]
    pub x_oss_process_status: Option<String>,
    /// Header field from response: x-oss-request-charged
    #[serde(skip)]
    pub x_oss_request_charged: Option<String>,
    /// Header field from response: Content-Md5
    #[serde(skip)]
    pub content_md5: Option<String>,
    /// Header field from response: Content-Length
    #[serde(skip)]
    pub content_length: Option<i64>,
    /// Header field from response: Last-Modified
    #[serde(skip)]
    pub last_modified: Option<String>,
    /// Header field from response: Content-Type
    #[serde(skip)]
    pub content_type: Option<String>,
    /// Header field from response: ETag
    #[serde(skip)]
    pub e_tag: Option<String>,
    /// Wildcard header field from response: x-oss-meta-* (prefix: x-oss-meta-)
    #[serde(skip)]
    pub x_oss_meta: std::collections::HashMap<String, String>,
    /// Header field from response: x-oss-tagging-count
    #[serde(skip)]
    pub x_oss_tagging_count: Option<i64>,
    /// Header field from response: Content-Disposition
    #[serde(skip)]
    pub content_disposition: Option<String>,
}
impl crate::BinaryWithMeta for GetObjectResponse {
    fn set_binary(&mut self, bytes: Vec<u8>) {
        self.body = bytes;
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct AppendObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-next-append-position
    #[serde(skip)]
    pub x_oss_next_append_position: Option<i64>,
    /// Header field from response: x-oss-hash-crc64ecma
    #[serde(skip)]
    pub x_oss_hash_crc64ecma: Option<String>,
}

impl crate::ToCodeMessage for AppendObjectResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct SealAppendObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-sealed-time
    #[serde(skip)]
    pub x_oss_sealed_time: Option<String>,
}

impl crate::ToCodeMessage for SealAppendObjectResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DeleteObjectResponse {
    /// Header field from response: x-oss-delete-marker
    #[serde(skip)]
    pub x_oss_delete_marker: Option<bool>,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for DeleteObjectResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

/// - 使用此接口不会返回文件内容。
///
/// - HeadObject操作默认获取Object当前版本的元信息。如果Object的当前版本为删除标记，则返回404 Not Found。请求参数中指定versionId则返回指定版本Object的元信息。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct HeadObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-server-side-encryption
    #[serde(skip)]
    pub x_oss_server_side_encryption: Option<String>,
    /// Header field from response: x-oss-server-side-encryption-key-id
    #[serde(skip)]
    pub x_oss_server_side_encryption_key_id: Option<String>,
    /// Header field from response: x-oss-storage-class
    #[serde(skip)]
    pub x_oss_storage_class: Option<String>,
    /// Header field from response: x-oss-object-type
    #[serde(skip)]
    pub x_oss_object_type: Option<String>,
    /// Header field from response: x-oss-next-append-position
    #[serde(skip)]
    pub x_oss_next_append_position: Option<i64>,
    /// Header field from response: x-oss-hash-crc64ecma
    #[serde(skip)]
    pub x_oss_hash_crc64ecma: Option<String>,
    /// Header field from response: x-oss-expiration
    #[serde(skip)]
    pub x_oss_expiration: Option<String>,
    /// Header field from response: x-oss-restore
    #[serde(skip)]
    pub x_oss_restore: Option<String>,
    /// Header field from response: x-oss-process-status
    #[serde(skip)]
    pub x_oss_process_status: Option<String>,
    /// Header field from response: x-oss-request-charged
    #[serde(skip)]
    pub x_oss_request_charged: Option<String>,
    /// Header field from response: Content-Md5
    #[serde(skip)]
    pub content_md5: Option<String>,
    /// Header field from response: Content-Length
    #[serde(skip)]
    pub content_length: Option<i64>,
    /// Header field from response: Last-Modified
    #[serde(skip)]
    pub last_modified: Option<String>,
    /// Header field from response: Content-Type
    #[serde(skip)]
    pub content_type: Option<String>,
    /// Header field from response: ETag
    #[serde(skip)]
    pub e_tag: Option<String>,
    /// Wildcard header field from response: x-oss-meta-* (prefix: x-oss-meta-)
    #[serde(skip)]
    pub x_oss_meta: std::collections::HashMap<String, String>,
    /// Header field from response: x-oss-transition-time
    #[serde(skip)]
    pub x_oss_transition_time: Option<String>,
    /// Header field from response: x-oss-tagging-count
    #[serde(skip)]
    pub x_oss_tagging_count: Option<i64>,
}

impl crate::ToCodeMessage for HeadObjectResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// **注意事项**
/// - 文件（Object）的元数据信息包括该Object的ETag、Size、LastModified信息，且不返回该Object的内容。如果Object类型为软链接，则会返回软链接自身信息。
/// - 当Bucket未启用版本控制时，要获取文件的元数据信息，您必须有oss:GetObject权限。当Bucket已启用版本控制时，要获取文件指定版本（请求中携带了x-oss-version-id请求头）的元数据信息，您必须有oss:GetObjectVersion权限。具体操作，请参见[为RAM用户授权自定义的RAM Policy](~~199058~~)。
///
/// **版本控制**
///
/// GetObjectMeta操作默认获取Object当前版本的元数据信息。如果Object的当前版本为删除标记，则返回404 Not Found。请求参数中指定versionId则返回指定版本Object的元数据信息。
///
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectMetaResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
    /// Header field from response: ETag
    #[serde(skip)]
    pub e_tag: Option<String>,
    /// Header field from response: Content-Length
    #[serde(skip)]
    pub content_length: Option<i64>,
    /// Header field from response: x-oss-last-access-time
    #[serde(skip)]
    pub x_oss_last_access_time: Option<String>,
    /// Header field from response: Last-Modifed
    #[serde(skip)]
    pub last_modifed: Option<String>,
    /// Header field from response: x-oss-transition-time
    #[serde(skip)]
    pub x_oss_transition_time: Option<String>,
}

impl crate::ToCodeMessage for GetObjectMetaResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RestoreObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-object-restore-priority
    #[serde(skip)]
    pub x_oss_object_restore_priority: Option<String>,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for RestoreObjectResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - 调用接口会返回一个OSS服务器创建的全局唯一的Upload ID，用于标识本次Multipart Upload事件。您可以根据这个ID来发起相关的操作，例如中止Multipart Upload、查询Multipart Upload等。
///
/// - 初始化MultipartUpload请求，并不影响已存在的同名Object。
///
/// - 该操作计算认证签名时，需要添加`?uploads`到`CanonicalizedResource`中。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct InitiateMultipartUploadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 初始化一个Multipart Upload事件的Bucket名称。
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    /// 初始化一个Multipart Upload事件的Object名称。
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    /// 唯一标识此次Multipart Upload事件的ID，用于后续调用UploadPart和CompleteMultipartUpload接口。
    #[serde(rename = "UploadId")]
    #[serde(default)]
    pub upload_id: String,
    /// 指明返回结果中编码使用的类型。如果请求的参数中指定了encoding-type，那返回的结果会对Key进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    pub encoding_type: String,
}

impl crate::ToCodeMessage for InitiateMultipartUploadResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CompleteMultipartUploadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 是否对返回的key进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    pub encoding_type: String,
    /// 新创建Object的URL。
    #[serde(rename = "Location")]
    #[serde(default)]
    pub location: String,
    /// Bucket名称。
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    /// 新创建Object的名字。
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    /// Object生成时会创建相应的ETag ，ETag用于标识一个Object的内容。
    ///
    /// 通过CompleteMultipartUpload请求创建的Object，ETag值是基于一定计算规则生成的唯一值，但不是其内容的MD5值。
    ///
    /// >ETag值可以用于检查Object内容是否发生变化。不建议使用ETag作为Object内容的MD5来校验数据完整性。
    #[serde(rename = "ETag")]
    #[serde(default)]
    pub e_tag: String,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for CompleteMultipartUploadResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UploadPartCopyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 最近一次修改时间。
    #[serde(rename = "LastModified")]
    #[serde(default)]
    pub last_modified: String,
    /// 被拷贝Object的ETag值。
    #[serde(rename = "ETag")]
    #[serde(default)]
    pub e_tag: String,
    /// Header field from response: x-oss-copy-source-version-id
    #[serde(skip)]
    pub x_oss_copy_source_version_id: Option<String>,
}

impl crate::ToCodeMessage for UploadPartCopyResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 调用ListMultipartUploads接口列举所有执行中的Multipart Upload事件，即已经初始化但还未完成（Complete）或者还未中止（Abort）的Multipart Upload事件。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListMultipartUploadsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Bucket名称。
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    /// 指明返回结果中编码使用的类型。如果请求参数中指定了encoding-type，那返回的结果会对Delimiter、KeyMarker、Prefix、NextKeyMarker和Key这些元素进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    pub encoding_type: String,
    /// 列表的起始Object位置。
    #[serde(rename = "KeyMarker")]
    #[serde(default)]
    pub key_marker: String,
    /// 列表的起始UploadId位置。
    #[serde(rename = "UploadIdMarker")]
    #[serde(default)]
    pub upload_id_marker: String,
    /// 如果本次没有返回全部结果，响应请求中将包含NextKeyMarker元素，用于表示接下来请求的KeyMarker值。
    #[serde(rename = "NextKeyMarker")]
    #[serde(default)]
    pub next_key_marker: String,
    /// 如果本次没有返回全部结果，响应请求中将包含NextUploadMarker元素，用于表示接下来请求的UploadMarker值。
    #[serde(rename = "NextUploadIdMarker")]
    #[serde(default)]
    pub next_upload_id_marker: String,
    /// 返回的最大Upload个数。
    #[serde(rename = "MaxUploads")]
    #[serde(default)]
    pub max_uploads: i64,
    /// 表示本次返回的MultipartUpload结果列表是否被截断。取值范围如下：
    ///
    /// true：表示本次没有返回全部结果。
    ///
    /// false（默认）：表示本次已经返回了全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 本次查询所用的前缀。
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
    /// 本次查询所用的Object名称分组字符。
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    pub delimiter: String,
    /// 保存Multipart Upload事件信息的列表。
    #[serde(rename = "Upload")]
    #[serde(default)]
    pub upload: Vec<Upload>,
    /// 保存列举结果中Object名称公共前缀的列表。
    #[serde(rename = "CommonPrefixes")]
    #[serde(default)]
    pub common_prefixes: Vec<CommonPrefix>,
}

impl crate::ToCodeMessage for ListMultipartUploadsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// - OSS的返回结果按照Part号码升序排列。
///
/// - 由于网络传输可能出错，所以不推荐使用ListParts返回结果中的Part Number和ETag值来生成已经上传成功的Part列表。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListPartsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Bucket名称。
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    /// Object名称。
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    /// Upload事件ID。
    #[serde(rename = "UploadId")]
    #[serde(default)]
    pub upload_id: String,
    /// 本次List结果的Part Number起始位置。
    #[serde(rename = "PartNumberMarker")]
    #[serde(default)]
    pub part_number_marker: i64,
    /// 如果本次没有返回全部结果，响应请求中将包含NextPartNumberMarker元素，用于标明接下来请求的PartNumberMarker值。
    #[serde(rename = "NextPartNumberMarker")]
    #[serde(default)]
    pub next_part_number_marker: i64,
    /// 返回请求中最大的Part数目。
    #[serde(rename = "MaxParts")]
    #[serde(default)]
    pub max_parts: i64,
    /// 标明本次返回的ListParts结果列表是否被截断。“true”表示本次没有返回全部结果；“false”表示本次已经返回了全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 保存Part信息的列表。
    #[serde(rename = "Part")]
    #[serde(default)]
    pub part: Vec<Part>,
}

impl crate::ToCodeMessage for ListPartsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutObjectAclResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for PutObjectAclResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// **版本控制**
///
/// 调用GetObjectACL接口时，默认只能获取Object当前版本的ACL。您可以通过指定versionId参数来获取指定Object版本的ACL。如果Object的对应版本为删除标记（Delete Marker），则OSS将返回404 Not Found。
///
/// >如果一个Object从未设置过ACL，则调用GetObjectACL时，返回的ObjectACL为default，表示该Object的ACL遵循Bucket ACL。即如果Bucket的访问权限是private，则该Object的访问权限也是private。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectAclResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存Bucket拥有者信息的容器。
    #[serde(rename = "Owner")]
    #[serde(default)]
    pub owner: Owner,
    /// 存储ACL信息的容器。
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    pub access_control_list: ObjectAclResponseAccessControlList,
}

impl crate::ToCodeMessage for GetObjectAclResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutSymlinkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for PutSymlinkResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// **版本控制**
///
/// GetSymlink接口默认获取软链接的当前版本。允许通过指定versionId来获取指定版本。如果软链接的当前版本为删除标记，OSS会返回404 Not Found，在响应header中返回x-oss-delete-marker = true以及版本ID : x-oss-version-id。删除标记没有关联数据，因此也没有软链接指向的TargetObject。
///
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetSymlinkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-symlink-target
    #[serde(skip)]
    pub x_oss_symlink_target: Option<String>,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for GetSymlinkResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutObjectTaggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for PutObjectTaggingResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// **版本控制**
///
/// 调用GetObjectTagging接口时，默认只能获取Object当前版本的标签信息。您可以通过指定versionId参数来获取指定Object版本的标签信息。如果Object的对应版本为删除标记（Delete Marker），则OSS将返回404 Not Found。
///
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectTaggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 标签集合。
    #[serde(rename = "TagSet")]
    #[serde(default)]
    pub tag_set: TagSet,
}

impl crate::ToCodeMessage for GetObjectTaggingResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 通过RTMP协议上传音视频数据前，必须先调用该接口创建一个LiveChannel。调用该接口会返回RTMP推流地址，以及对应的播放地址。
/// <br>您可以使用返回的地址进行推流、播放，您还可以根据该LiveChannel的名称来发起相关的操作，如查询推流状态、查询推流记录、禁止推流等。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutLiveChannelResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存推流地址的容器。
    #[serde(rename = "PublishUrls")]
    #[serde(default)]
    pub publish_urls: LiveChannelPublishUrls,
    /// 保存播放地址的容器。
    #[serde(rename = "PlayUrls")]
    #[serde(default)]
    pub play_urls: LiveChannelPlayUrls,
}

impl crate::ToCodeMessage for PutLiveChannelResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListLiveChannelResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次查询结果的开始前缀。
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
    /// 本次ListLiveChannel的起点。
    #[serde(rename = "Marker")]
    #[serde(default)]
    pub marker: String,
    /// 响应请求内返回结果的最大数目。
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    pub max_keys: i64,
    /// 是否已返回所有的结果。
    ///
    /// true：表示本次请求已返回全部结果。
    ///
    /// false：表示本次请求未返回全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    pub is_truncated: bool,
    /// 如果本次没有返回全部结果，响应请求中将包含NextMarker元素，用于标明接下来请求的Marker值。
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    pub next_marker: String,
    /// 保存返回的LiveChannel信息的列表。
    #[serde(rename = "LiveChannel")]
    #[serde(default)]
    pub live_channel: Vec<LiveChannel>,
}

impl crate::ToCodeMessage for ListLiveChannelResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// LiveChannel的描述信息。
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    /// LiveChannel的状态信息。有效值：
    ///
    /// enabled：启用状态
    ///
    /// disabled：禁用状态
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    /// 保存LiveChannel转储配置的容器。
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: LiveChannelTarget,
}

impl crate::ToCodeMessage for GetLiveChannelInfoResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

/// 使用GetLiveChannelHistory接口最多会返回指定LiveChannel最近的10次推流记录。
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelHistoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 保存推流记录信息的列表。
    #[serde(rename = "LiveRecord")]
    #[serde(default)]
    pub live_record: Vec<LiveRecord>,
}

impl crate::ToCodeMessage for GetLiveChannelHistoryResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelStatResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// LiveChannel当前的推流状态描述。有效值：Disabled、Live、Idle。
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    /// 当Status为Live时，表示当前客户端开始推流的时间。此元素使用ISO8601格式表示。
    #[serde(rename = "ConnectedTime")]
    #[serde(default)]
    pub connected_time: String,
    /// 当Status为Live时，表示当前推流客户端的IP地址。
    #[serde(rename = "RemoteAddr")]
    #[serde(default)]
    pub remote_addr: String,
    /// 当Status为Live时，保存视频流信息的容器。
    ///
    /// >Video、Audio容器只有在Status为Live时才会返回，但Status为Live时不一定返回这两个容器。例如，客户端已经连接到LiveChannel，但尚未发送音视频数据，这种情况不会返回这两个容器。
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: LiveChannelVideo,
    /// 当Status为Live时，保存音频流信息的容器。
    ///
    /// >Video、Audio容器只有在Status为Live时才会返回，但Status为Live时不一定返回这两个容器。例如，客户端已经连接到LiveChannel，但尚未发送音视频数据，这种情况不会返回这两个容器。
    #[serde(rename = "Audio")]
    #[serde(default)]
    pub audio: LiveChannelAudio,
}

impl crate::ToCodeMessage for GetLiveChannelStatResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutDataLakeCachePrefetchJobResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ID")]
    #[serde(default)]
    pub id: String,
}

impl crate::ToCodeMessage for PutDataLakeCachePrefetchJobResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

impl Endpoint {
    fn internal_dns(self) -> &'static str {
        match self {
            Endpoint::CnWuhanLr => "oss-cn-wuhan-lr-internal.aliyuncs.com",
            Endpoint::CnQingdao => "oss-cn-qingdao-internal.aliyuncs.com",
            Endpoint::CnBeijing => "oss-cn-beijing-internal.aliyuncs.com",
            Endpoint::CnZhangjiakou => "oss-cn-zhangjiakou-internal.aliyuncs.com",
            Endpoint::CnHuhehaote => "oss-cn-huhehaote-internal.aliyuncs.com",
            Endpoint::CnWulanchabu => "oss-cn-wulanchabu-internal.aliyuncs.com",
            Endpoint::CnHangzhou => "oss-cn-hangzhou-internal.aliyuncs.com",
            Endpoint::CnShanghai => "oss-cn-shanghai-internal.aliyuncs.com",
            Endpoint::CnNanjing => "oss-cn-nanjing-internal.aliyuncs.com",
            Endpoint::CnFuzhou => "oss-cn-fuzhou-internal.aliyuncs.com",
            Endpoint::CnShenzhen => "oss-cn-shenzhen-internal.aliyuncs.com",
            Endpoint::CnHeyuan => "oss-cn-heyuan-internal.aliyuncs.com",
            Endpoint::CnGuangzhou => "oss-cn-guangzhou-internal.aliyuncs.com",
            Endpoint::CnChengdu => "oss-cn-chengdu-internal.aliyuncs.com",
            Endpoint::CnHongkong => "oss-cn-hongkong-internal.aliyuncs.com",
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

impl Connection {
    /// Create connection connect to oss internal dns
    pub fn internal_with_client(
        endpoint: Endpoint,
        app_key_secret: crate::v3::AccessKeySecret,
        client: reqwest::Client,
    ) -> Self {
        Self(crate::common::Connection::with_client(
            crate::auth::Oss4HmacSha256::new(app_key_secret, endpoint.name()),
            "2019-05-17",
            endpoint.internal_dns(),
            client,
        ))
    }
}
