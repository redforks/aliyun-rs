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
    ) -> impl std::future::Future<
        Output = crate::Result<<R::ResponseWrap as crate::IntoResponse>::Response>,
    > + Send {
        self.0.call(req)
    }

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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketStatResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketInfoResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListAccessPointsResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetAccessPointResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<CreateAccessPointResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketLifecycleResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketInventoryResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketLoggingResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetUserDefinedLogFieldsConfigResponse>> + Send
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketWebsiteResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketRefererResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketDataRedundancyTransitionResponse>> + Send
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
                r##"Only HttpMethod::Get, HttpMethod::Post, HttpMethod::Put, or HttpMethod::Delete supported"##
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketAccessMonitorResponse>> + Send
    {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<DoMetaQueryResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetCnameTokenResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<CreateCnameTokenResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetStyleResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketHttpsConfigResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetPublicAccessBlockResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketPublicAccessBlockResponse>> + Send
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetAccessPointPublicAccessBlockResponse>> + Send
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketArchiveDirectReadResponse>> + Send
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketOverwriteConfigResponse>> + Send
    {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<Vec<u8>>> + Send {
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
        async {
            todo!(
                r##"Only HttpMethod::Get, HttpMethod::Post, HttpMethod::Put, or HttpMethod::Delete supported
Header 'x-oss-meta-*': Schema with additional_properties of type String is not supported. Only 'object' type is supported."##
            );
        }
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
        async {
            todo!(
                r##"Only HttpMethod::Get, HttpMethod::Post, HttpMethod::Put, or HttpMethod::Delete supported"##
            );
        }
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<CreateSelectObjectMetaResponse>> + Send
    {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetBucketEventNotificationResponse>> + Send
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
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListDataLakeStorageTransferJobResponse>> + Send
    {
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
    const URL_PATH: &'static str = "/?regions";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<DescribeRegionsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

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
    const URL_PATH: &'static str = "/?stat";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketStatResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucket {
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
    body: Option<PutBucketbody>,
}

impl sealed::Bound for PutBucket {}

impl PutBucket {
    pub fn new() -> Self {
        Self {
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

    type Body = crate::XmlBody<PutBucketbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

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

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
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
impl crate::ToFormData for DeleteBucket {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucket {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucket";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    encoding_type: Option<EncodeType>,
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

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?list-type=2";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListObjectsV2Response>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(7);

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

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?bucketInfo";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?location";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketLocationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?accessPoint";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListAccessPointsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

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
    /// 接入点名称。
    x_oss_access_point_name: String,
}

impl sealed::Bound for GetAccessPoint {}

impl GetAccessPoint {
    pub fn new(x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_name: x_oss_access_point_name.into(),
        }
    }
}

impl crate::Request for GetAccessPoint {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPoint";
    const URL_PATH: &'static str = "/?accessPoint";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetAccessPointResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-name".into(),
            self.x_oss_access_point_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointPolicy {
    /// 接入点名称。
    x_oss_access_point_name: String,
}

impl sealed::Bound for GetAccessPointPolicy {}

impl GetAccessPointPolicy {
    pub fn new(x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_name: x_oss_access_point_name.into(),
        }
    }
}

impl crate::Request for GetAccessPointPolicy {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointPolicy";
    const URL_PATH: &'static str = "/?accessPointPolicy";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<GetAccessPointPolicyResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-name".into(),
            self.x_oss_access_point_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointPolicy {
    /// 接入点名称。
    x_oss_access_point_name: String,
}

impl sealed::Bound for DeleteAccessPointPolicy {}

impl DeleteAccessPointPolicy {
    pub fn new(x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/?accessPointPolicy";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-name".into(),
            self.x_oss_access_point_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointPolicy {
    /// 接入点名称。
    #[setters(generate = true, strip_option)]
    x_oss_access_point_name: Option<String>,
    /// 接入点策略配置内容。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for PutAccessPointPolicy {}

impl PutAccessPointPolicy {
    pub fn new() -> Self {
        Self {
            x_oss_access_point_name: None,
            body: None,
        }
    }
}

impl crate::Request for PutAccessPointPolicy {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAccessPointPolicy";
    const URL_PATH: &'static str = "/?accessPointPolicy";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);

        if let Some(f) = &self.x_oss_access_point_name {
            headers.push(("x-oss-access-point-name".into(), f.to_string()));
        }

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPoint {
    /// 接入点名称。
    x_oss_access_point_name: String,
}

impl sealed::Bound for DeleteAccessPoint {}

impl DeleteAccessPoint {
    pub fn new(x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/?accessPoint";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-name".into(),
            self.x_oss_access_point_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateAccessPoint {
    /// 保存接入点信息的容器。
    #[setters(generate = true, strip_option)]
    body: Option<AccessPointbody>,
}

impl sealed::Bound for CreateAccessPoint {}

impl CreateAccessPoint {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for CreateAccessPoint {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "CreateAccessPoint";
    const URL_PATH: &'static str = "/?accessPoint";

    type Body = crate::XmlBody<AccessPointbody>;

    type ResponseWrap = crate::XmlResponseWrap<CreateAccessPointResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct InitiateBucketWorm {
    /// 请求体。
    #[setters(generate = true, strip_option)]
    body: Option<InitiateBucketWormbody>,
}

impl sealed::Bound for InitiateBucketWorm {}

impl InitiateBucketWorm {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for InitiateBucketWorm {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "InitiateBucketWorm";
    const URL_PATH: &'static str = "/?worm";

    type Body = crate::XmlBody<InitiateBucketWormbody>;

    type ResponseWrap = crate::XmlResponseWrap<InitiateBucketWormResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
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
pub struct AbortBucketWorm {}

impl sealed::Bound for AbortBucketWorm {}

impl AbortBucketWorm {
    pub fn new() -> Self {
        Self {}
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
    const URL_PATH: &'static str = "/?worm";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CompleteBucketWorm {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CompleteBucketWorm";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("wormId".into(), (&self.worm_id).into()));

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
pub struct ExtendBucketWorm {
    /// 合规保留策略的ID。
    ///
    /// > 如果指定用于延长Object保留天数对应的合规保留策略ID不存在，则返回404。
    worm_id: String,
    /// 保存合规保留策略的容器。
    #[setters(generate = true, strip_option)]
    body: Option<ExtendBucketWormbody>,
}

impl sealed::Bound for ExtendBucketWorm {}

impl ExtendBucketWorm {
    pub fn new(worm_id: impl Into<String>) -> Self {
        Self {
            worm_id: worm_id.into(),
            body: None,
        }
    }
}

impl crate::Request for ExtendBucketWorm {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "ExtendBucketWorm";
    const URL_PATH: &'static str = "/?wormExtend";

    type Body = crate::XmlBody<ExtendBucketWormbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("wormId".into(), (&self.worm_id).into()));

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
    const URL_PATH: &'static str = "/?worm";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketWormResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketAcl {
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
    pub fn new(x_oss_acl: impl Into<BucketACL>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/?acl";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push(("x-oss-acl".into(), self.x_oss_acl.to_string()));

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
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
    const URL_PATH: &'static str = "/?acl";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketAclResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketLifecycle {
    /// 指定生命周期规则是否允许前缀重叠。取值如下：
    ///
    /// true：允许前缀重叠。
    ///
    /// false：不允许前缀重叠。
    #[setters(generate = true, strip_option)]
    x_oss_allow_same_action_overlap: Option<String>,
    /// 保存Lifecycle配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketLifecyclebody>,
}

impl sealed::Bound for PutBucketLifecycle {}

impl PutBucketLifecycle {
    pub fn new() -> Self {
        Self {
            x_oss_allow_same_action_overlap: None,
            body: None,
        }
    }
}

impl crate::Request for PutBucketLifecycle {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketLifecycle";
    const URL_PATH: &'static str = "/?lifecycle";

    type Body = crate::XmlBody<BucketLifecyclebody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);

        if let Some(f) = &self.x_oss_allow_same_action_overlap {
            headers.push(("x-oss-allow-same-action-overlap".into(), f.to_string()));
        }

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
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
    const URL_PATH: &'static str = "/?lifecycle";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketLifecycleResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for DeleteBucketLifecycle {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketLifecycle {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketLifecycle";
    const URL_PATH: &'static str = "/?lifecycle";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutBucketTransferAcceleration {
    /// 传输加速配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<TransferAccelerationbody>,
}

impl sealed::Bound for PutBucketTransferAcceleration {}

impl PutBucketTransferAcceleration {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketTransferAcceleration {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketTransferAcceleration";
    const URL_PATH: &'static str = "/?transferAcceleration";

    type Body = crate::XmlBody<TransferAccelerationbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?transferAcceleration";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketTransferAccelerationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketVersioning {
    /// 请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketVersioningbody>,
}

impl sealed::Bound for PutBucketVersioning {}

impl PutBucketVersioning {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketVersioning {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketVersioning";
    const URL_PATH: &'static str = "/?versioning";

    type Body = crate::XmlBody<BucketVersioningbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?versioning";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketVersioningResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    encoding_type: Option<EncodeType>,
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
    const URL_PATH: &'static str = "/?versions";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListObjectVersionsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(6);

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

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketPolicy {
    /// 请求体。
    body: Vec<u8>,
}

impl sealed::Bound for PutBucketPolicy {}

impl PutBucketPolicy {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }
}

impl crate::Request for PutBucketPolicy {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketPolicy";
    const URL_PATH: &'static str = "/?policy";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body)
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
    const URL_PATH: &'static str = "/?policy";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<GetBucketPolicyResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for DeleteBucketPolicy {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketPolicy {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketPolicy";
    const URL_PATH: &'static str = "/?policy";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?policyStatus";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketPolicyStatusResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketRtc {
    /// 保存RTC配置规则的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketRtcbody>,
}

impl sealed::Bound for PutBucketRtc {}

impl PutBucketRtc {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketRtc {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketRtc";
    const URL_PATH: &'static str = "/?rtc";

    type Body = crate::XmlBody<BucketRtcbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutBucketReplication {
    /// 指定数据复制配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<PutBucketReplicationbody>,
}

impl sealed::Bound for PutBucketReplication {}

impl PutBucketReplication {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketReplication {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutBucketReplication";
    const URL_PATH: &'static str = "/?replication&comp=add";

    type Body = crate::XmlBody<PutBucketReplicationbody>;

    type ResponseWrap = crate::XmlResponseWrap<PutBucketReplicationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?replication";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketReplicationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?replicationLocation";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketReplicationLocationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?replicationProgress";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketReplicationProgressResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("rule-id".into(), (&self.rule_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteBucketReplication {
    /// 保存需要删除的数据复制规则的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<DeleteBucketReplicationbody>,
}

impl sealed::Bound for DeleteBucketReplication {}

impl DeleteBucketReplication {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for DeleteBucketReplication {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DeleteBucketReplication";
    const URL_PATH: &'static str = "/?replication&comp=delete";

    type Body = crate::XmlBody<DeleteBucketReplicationbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutBucketInventory {
    /// 配置的清单规则Id。
    inventory_id: String,
    /// 存储清单配置信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketInventorybody>,
}

impl sealed::Bound for PutBucketInventory {}

impl PutBucketInventory {
    pub fn new(inventory_id: impl Into<String>) -> Self {
        Self {
            inventory_id: inventory_id.into(),
            body: None,
        }
    }
}

impl crate::Request for PutBucketInventory {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketInventory";
    const URL_PATH: &'static str = "/?inventory";

    type Body = crate::XmlBody<BucketInventorybody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("inventoryId".into(), (&self.inventory_id).into()));

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
    const URL_PATH: &'static str = "/?inventory";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketInventoryResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("inventoryId".into(), (&self.inventory_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?inventory";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListBucketInventoryResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.continuation_token {
            params.push(("continuation-token".into(), (f).into()));
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
impl crate::ToFormData for DeleteBucketInventory {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketInventory {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketInventory";
    const URL_PATH: &'static str = "/?inventory";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("inventoryId".into(), (&self.inventory_id).into()));

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
pub struct PutBucketLogging {
    /// 存储访问日志状态信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketLoggingbody>,
}

impl sealed::Bound for PutBucketLogging {}

impl PutBucketLogging {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketLogging {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketLogging";
    const URL_PATH: &'static str = "/?logging";

    type Body = crate::XmlBody<BucketLoggingbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?logging";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketLoggingResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for DeleteBucketLogging {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketLogging {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketLogging";
    const URL_PATH: &'static str = "/?logging";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutUserDefinedLogFieldsConfig {
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<FieldsConfigbody>,
}

impl sealed::Bound for PutUserDefinedLogFieldsConfig {}

impl PutUserDefinedLogFieldsConfig {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutUserDefinedLogFieldsConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutUserDefinedLogFieldsConfig";
    const URL_PATH: &'static str = "/?userDefinedLogFieldsConfig";

    type Body = crate::XmlBody<FieldsConfigbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?userDefinedLogFieldsConfig";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetUserDefinedLogFieldsConfigResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for DeleteUserDefinedLogFieldsConfig {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteUserDefinedLogFieldsConfig {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteUserDefinedLogFieldsConfig";
    const URL_PATH: &'static str = "/?userDefinedLogFieldsConfig";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?website";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketWebsiteResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketWebsite {
    /// 保存静态网站配置的容器。
    #[setters(generate = true, strip_option)]
    body: Option<BucketWebsitebody>,
}

impl sealed::Bound for PutBucketWebsite {}

impl PutBucketWebsite {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketWebsite {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketWebsite";
    const URL_PATH: &'static str = "/?website";

    type Body = crate::XmlBody<BucketWebsitebody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct DeleteBucketWebsite {}

impl sealed::Bound for DeleteBucketWebsite {}

impl DeleteBucketWebsite {
    pub fn new() -> Self {
        Self {}
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
    const URL_PATH: &'static str = "/?website";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutBucketReferer {
    /// 保存Referer配置内容的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketRefererbody>,
}

impl sealed::Bound for PutBucketReferer {}

impl PutBucketReferer {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketReferer {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketReferer";
    const URL_PATH: &'static str = "/?referer";

    type Body = crate::XmlBody<BucketRefererbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?referer";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketRefererResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketTags {
    /// 设置Bucket TagSet的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketTagsbody>,
}

impl sealed::Bound for PutBucketTags {}

impl PutBucketTags {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketTags {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketTags";
    const URL_PATH: &'static str = "/?tagging";

    type Body = crate::XmlBody<BucketTagsbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?tagging";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketTagsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for DeleteBucketTags {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketTags {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketTags";
    const URL_PATH: &'static str = "/?tagging";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?redundancyTransition";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListUserDataRedundancyTransitionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

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
    const URL_PATH: &'static str = "/?redundancyTransition";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListBucketDataRedundancyTransitionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?redundancyTransition";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketDataRedundancyTransitionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push((
            "x-oss-redundancy-transition-taskid".into(),
            (&self.x_oss_redundancy_transition_taskid).into(),
        ));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CreateBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateBucketDataRedundancyTransition";
    const URL_PATH: &'static str = "/?redundancyTransition";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<CreateBucketDataRedundancyTransitionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push((
            "x-oss-target-redundancy-type".into(),
            (&self.x_oss_target_redundancy_type).into(),
        ));

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
impl crate::ToFormData for DeleteBucketDataRedundancyTransition {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketDataRedundancyTransition {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketDataRedundancyTransition";
    const URL_PATH: &'static str = "/?redundancyTransition";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push((
            "x-oss-redundancy-transition-taskid".into(),
            (&self.x_oss_redundancy_transition_taskid).into(),
        ));

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
pub struct PutBucketEncryption {
    /// 配置服务器端加密规则的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketEncryptionbody>,
}

impl sealed::Bound for PutBucketEncryption {}

impl PutBucketEncryption {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketEncryption {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketEncryption";
    const URL_PATH: &'static str = "/?encryption";

    type Body = crate::XmlBody<BucketEncryptionbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?encryption";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketEncryptionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for DeleteBucketEncryption {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketEncryption {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketEncryption";
    const URL_PATH: &'static str = "/?encryption";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutBucketRequestPayment {
    /// 配置请求者付费的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<RequestPaymentbody>,
}

impl sealed::Bound for PutBucketRequestPayment {}

impl PutBucketRequestPayment {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketRequestPayment {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketRequestPayment";
    const URL_PATH: &'static str = "/?requestPayment";

    type Body = crate::XmlBody<RequestPaymentbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?requestPayment";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketRequestPaymentResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketCors {
    /// 设置跨域资源共享规则的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<BucketCorsbody>,
}

impl sealed::Bound for PutBucketCors {}

impl PutBucketCors {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketCors {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketCors";
    const URL_PATH: &'static str = "/?cors";

    type Body = crate::XmlBody<BucketCorsbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?cors";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketCorsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for DeleteBucketCors {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketCors {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketCors";
    const URL_PATH: &'static str = "/?cors";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct OptionObject {
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
    pub fn new(key: impl Into<String>) -> Self {
        Self {
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
        Box::new([("key", self.key.to_string())])
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
    /// 修改访问跟踪状态配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<AccessMonitorbody>,
}

impl sealed::Bound for PutBucketAccessMonitor {}

impl PutBucketAccessMonitor {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketAccessMonitor {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketAccessMonitor";
    const URL_PATH: &'static str = "/?accessmonitor";

    type Body = crate::XmlBody<AccessMonitorbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?accessmonitor";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketAccessMonitorResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?metaQuery";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetMetaQueryStatusResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CloseMetaQuery {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CloseMetaQuery";
    const URL_PATH: &'static str = "/?metaQuery&comp=delete";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct DoMetaQuery {
    /// 指定检索模式。
    /// - basic：标量检索（默认）
    /// - semantic：向量检索
    #[setters(generate = true, strip_option)]
    mode: Option<String>,
    /// 保存查询条件的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<DoMetaQuerybody>,
}

impl sealed::Bound for DoMetaQuery {}

impl DoMetaQuery {
    pub fn new() -> Self {
        Self {
            mode: None,
            body: None,
        }
    }
}

impl crate::Request for DoMetaQuery {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DoMetaQuery";
    const URL_PATH: &'static str = "/?metaQuery&comp=query";

    type Body = crate::XmlBody<DoMetaQuerybody>;

    type ResponseWrap = crate::XmlResponseWrap<DoMetaQueryResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.mode {
            params.push(("mode".into(), (f).into()));
        }

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
    /// 接口请求体参数
    #[setters(generate = true, strip_option)]
    body: Option<OpenMetaQuerybody>,
}

impl sealed::Bound for OpenMetaQuery {}

impl OpenMetaQuery {
    pub fn new() -> Self {
        Self {
            mode: None,
            role: None,
            body: None,
        }
    }
}

impl crate::Request for OpenMetaQuery {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "OpenMetaQuery";
    const URL_PATH: &'static str = "/?metaQuery&comp=add";

    type Body = crate::XmlBody<OpenMetaQuerybody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

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
    const URL_PATH: &'static str = "/?antiDDos";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    body: Option<UpdateBucketAntiDDosInfobody>,
}

impl sealed::Bound for UpdateBucketAntiDDosInfo {}

impl UpdateBucketAntiDDosInfo {
    pub fn new(
        x_oss_defender_instance: impl Into<String>,
        x_oss_defender_status: impl Into<String>,
    ) -> Self {
        Self {
            x_oss_defender_instance: x_oss_defender_instance.into(),
            x_oss_defender_status: x_oss_defender_status.into(),
            body: None,
        }
    }
}

impl crate::Request for UpdateBucketAntiDDosInfo {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UpdateBucketAntiDDosInfo";
    const URL_PATH: &'static str = "/?antiDDos";

    type Body = crate::XmlBody<UpdateBucketAntiDDosInfobody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?bucketAntiDDos";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListBucketAntiDDosInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

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
    const URL_PATH: &'static str = "/?antiDDos";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<InitUserAntiDDosInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    /// 高防实例ID。
    x_oss_defender_instance: String,
    /// 高防实例类型。取值固定为AntiDDosPremimum。
    x_oss_defender_type: String,
    /// 保存高防实例配置信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<InitBucketAntiDDosInfobody>,
}

impl sealed::Bound for InitBucketAntiDDosInfo {}

impl InitBucketAntiDDosInfo {
    pub fn new(
        x_oss_defender_instance: impl Into<String>,
        x_oss_defender_type: impl Into<String>,
    ) -> Self {
        Self {
            x_oss_defender_instance: x_oss_defender_instance.into(),
            x_oss_defender_type: x_oss_defender_type.into(),
            body: None,
        }
    }
}

impl crate::Request for InitBucketAntiDDosInfo {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "InitBucketAntiDDosInfo";
    const URL_PATH: &'static str = "/?antiDDos";

    type Body = crate::XmlBody<InitBucketAntiDDosInfobody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?antiDDos";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetUserAntiDDosInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?resourceGroup";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketResourceGroupResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketResourceGroup {
    /// 配置资源组ID的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<ResourceGroupbody>,
}

impl sealed::Bound for PutBucketResourceGroup {}

impl PutBucketResourceGroup {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketResourceGroup {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketResourceGroup";
    const URL_PATH: &'static str = "/?resourceGroup";

    type Body = crate::XmlBody<ResourceGroupbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutCname {
    /// 保存Cname配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<PutCnamebody>,
}

impl sealed::Bound for PutCname {}

impl PutCname {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutCname {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PutCname";
    const URL_PATH: &'static str = "/?cname&comp=add";

    type Body = crate::XmlBody<PutCnamebody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?cname";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListCnameResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteCname {
    /// 删除Cname配置信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<DeleteCnamebody>,
}

impl sealed::Bound for DeleteCname {}

impl DeleteCname {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for DeleteCname {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DeleteCname";
    const URL_PATH: &'static str = "/?cname&comp=delete";

    type Body = crate::XmlBody<DeleteCnamebody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?comp=token";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetCnameTokenResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("cname".into(), (&self.cname).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateCnameToken {
    /// 创建CnameToken的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<CnameTokenbody>,
}

impl sealed::Bound for CreateCnameToken {}

impl CreateCnameToken {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for CreateCnameToken {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateCnameToken";
    const URL_PATH: &'static str = "/?cname&comp=token";

    type Body = crate::XmlBody<CnameTokenbody>;

    type ResponseWrap = crate::XmlResponseWrap<CreateCnameTokenResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutStyle {
    /// 图片样式名称。
    style_name: String,
    /// 样式分类。
    ///
    /// 取值：image、document、video。
    #[setters(generate = true, strip_option)]
    category: Option<String>,
    /// 保存图片样式信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<PutStylebody>,
}

impl sealed::Bound for PutStyle {}

impl PutStyle {
    pub fn new(style_name: impl Into<String>) -> Self {
        Self {
            style_name: style_name.into(),
            category: None,
            body: None,
        }
    }
}

impl crate::Request for PutStyle {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutStyle";
    const URL_PATH: &'static str = "/?style";

    type Body = crate::XmlBody<PutStylebody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

        if let Some(f) = &self.category {
            params.push(("category".into(), (f).into()));
        }
        params.push(("styleName".into(), (&self.style_name).into()));

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
    const URL_PATH: &'static str = "/?style";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListStyleResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
    const URL_PATH: &'static str = "/?style";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetStyleResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("styleName".into(), (&self.style_name).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for DeleteStyle {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteStyle {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteStyle";
    const URL_PATH: &'static str = "/?style";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("styleName".into(), (&self.style_name).into()));

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
    const URL_PATH: &'static str = "/?httpsConfig";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketHttpsConfigResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketHttpsConfig {
    /// 保存HTTPS配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<HttpsConfigbody>,
}

impl sealed::Bound for PutBucketHttpsConfig {}

impl PutBucketHttpsConfig {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketHttpsConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketHttpsConfig";
    const URL_PATH: &'static str = "/?httpsConfig";

    type Body = crate::XmlBody<HttpsConfigbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct CreateAccessPointForObjectProcess {
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
    pub fn new(x_oss_access_point_for_object_process_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: None,
        }
    }
}

impl crate::Request for CreateAccessPointForObjectProcess {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "CreateAccessPointForObjectProcess";
    const URL_PATH: &'static str = "/?accessPointForObjectProcess";

    type Body = crate::XmlBody<PointForObjectProcessbody>;

    type ResponseWrap = crate::XmlResponseWrap<CreateAccessPointForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointForObjectProcess {
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
}

impl sealed::Bound for GetAccessPointForObjectProcess {}

impl GetAccessPointForObjectProcess {
    pub fn new(x_oss_access_point_for_object_process_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
        }
    }
}

impl crate::Request for GetAccessPointForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointForObjectProcess";
    const URL_PATH: &'static str = "/?accessPointForObjectProcess";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetAccessPointForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
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
    const URL_PATH: &'static str = "/?accessPointForObjectProcess";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListAccessPointsForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

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
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
}

impl sealed::Bound for DeleteAccessPointForObjectProcess {}

impl DeleteAccessPointForObjectProcess {
    pub fn new(x_oss_access_point_for_object_process_name: impl Into<String>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/?accessPointForObjectProcess";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointConfigForObjectProcess {
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
}

impl sealed::Bound for GetAccessPointConfigForObjectProcess {}

impl GetAccessPointConfigForObjectProcess {
    pub fn new(x_oss_access_point_for_object_process_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
        }
    }
}

impl crate::Request for GetAccessPointConfigForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointConfigForObjectProcess";
    const URL_PATH: &'static str = "/?accessPointConfigForObjectProcess";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetAccessPointConfigForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointConfigForObjectProcess {
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
    pub fn new(x_oss_access_point_for_object_process_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: None,
        }
    }
}

impl crate::Request for PutAccessPointConfigForObjectProcess {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAccessPointConfigForObjectProcess";
    const URL_PATH: &'static str = "/?accessPointConfigForObjectProcess";

    type Body = crate::XmlBody<ConfigForObjectProcessbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutAccessPointPolicyForObjectProcess {
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for PutAccessPointPolicyForObjectProcess {}

impl PutAccessPointPolicyForObjectProcess {
    pub fn new(x_oss_access_point_for_object_process_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: None,
        }
    }
}

impl crate::Request for PutAccessPointPolicyForObjectProcess {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAccessPointPolicyForObjectProcess";
    const URL_PATH: &'static str = "/?accessPointPolicyForObjectProcess";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAccessPointPolicyForObjectProcess {
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
    body: Vec<u8>,
}

impl sealed::Bound for GetAccessPointPolicyForObjectProcess {}

impl GetAccessPointPolicyForObjectProcess {
    pub fn new(x_oss_access_point_for_object_process_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for GetAccessPointPolicyForObjectProcess {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAccessPointPolicyForObjectProcess";
    const URL_PATH: &'static str = "/?accessPointPolicyForObjectProcess";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::XmlResponseWrap<GetAccessPointPolicyForObjectProcessResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAccessPointPolicyForObjectProcess {
    /// 填写对象FC接入点名称。
    x_oss_access_point_for_object_process_name: String,
    body: Vec<u8>,
}

impl sealed::Bound for DeleteAccessPointPolicyForObjectProcess {}

impl DeleteAccessPointPolicyForObjectProcess {
    pub fn new(x_oss_access_point_for_object_process_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_for_object_process_name: x_oss_access_point_for_object_process_name
                .into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for DeleteAccessPointPolicyForObjectProcess {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteAccessPointPolicyForObjectProcess";
    const URL_PATH: &'static str = "/?accessPointPolicyForObjectProcess";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(1);
        headers.push((
            "x-oss-access-point-for-object-process-name".into(),
            self.x_oss_access_point_for_object_process_name.to_string(),
        ));

        headers
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
    const URL_PATH: &'static str = "/?publicAccessBlock";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetPublicAccessBlockResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    body: Option<PutPublicAccessBlockbody>,
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
    const URL_PATH: &'static str = "/?publicAccessBlock";

    type Body = crate::XmlBody<PutPublicAccessBlockbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?publicAccessBlock";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?publicAccessBlock";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketPublicAccessBlockResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketPublicAccessBlock {
    /// 接口请求体参数。
    #[setters(generate = true, strip_option)]
    body: Option<BucketPublicAccessBlockbody>,
}

impl sealed::Bound for PutBucketPublicAccessBlock {}

impl PutBucketPublicAccessBlock {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketPublicAccessBlock {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketPublicAccessBlock";
    const URL_PATH: &'static str = "/?publicAccessBlock";

    type Body = crate::XmlBody<BucketPublicAccessBlockbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct DeleteBucketPublicAccessBlock {}

impl sealed::Bound for DeleteBucketPublicAccessBlock {}

impl DeleteBucketPublicAccessBlock {
    pub fn new() -> Self {
        Self {}
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
    const URL_PATH: &'static str = "/?publicAccessBlock";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?publicAccessBlock";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetAccessPointPublicAccessBlockResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.x_oss_access_point_name {
            params.push(("x-oss-access-point-name".into(), (f).into()));
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
pub struct PutAccessPointPublicAccessBlock {
    /// 接入点名称。
    x_oss_access_point_name: String,
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<PointPublicAccessBlockbody>,
}

impl sealed::Bound for PutAccessPointPublicAccessBlock {}

impl PutAccessPointPublicAccessBlock {
    pub fn new(x_oss_access_point_name: impl Into<String>) -> Self {
        Self {
            x_oss_access_point_name: x_oss_access_point_name.into(),
            body: None,
        }
    }
}

impl crate::Request for PutAccessPointPublicAccessBlock {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAccessPointPublicAccessBlock";
    const URL_PATH: &'static str = "/?publicAccessBlock";

    type Body = crate::XmlBody<PointPublicAccessBlockbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push((
            "x-oss-access-point-name".into(),
            (&self.x_oss_access_point_name).into(),
        ));

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
impl crate::ToFormData for DeleteAccessPointPublicAccessBlock {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteAccessPointPublicAccessBlock {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteAccessPointPublicAccessBlock";
    const URL_PATH: &'static str = "/?publicAccessBlock";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.x_oss_access_point_name {
            params.push(("x-oss-access-point-name".into(), (f).into()));
        }

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
    const URL_PATH: &'static str = "/?bucketArchiveDirectRead";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketArchiveDirectReadResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutBucketArchiveDirectRead {
    /// 接口请求体。
    #[setters(generate = true, strip_option)]
    body: Option<DirectReadbody>,
}

impl sealed::Bound for PutBucketArchiveDirectRead {}

impl PutBucketArchiveDirectRead {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketArchiveDirectRead {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketArchiveDirectRead";
    const URL_PATH: &'static str = "/?bucketArchiveDirectRead";

    type Body = crate::XmlBody<DirectReadbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutBucketOverwriteConfig {
    /// 接口请求体结构
    #[setters(generate = true, strip_option)]
    body: Option<OverwriteConfigbody>,
}

impl sealed::Bound for PutBucketOverwriteConfig {}

impl PutBucketOverwriteConfig {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketOverwriteConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketOverwriteConfig";
    const URL_PATH: &'static str = "/?overwriteConfig";

    type Body = crate::XmlBody<OverwriteConfigbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?overwriteConfig";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketOverwriteConfigResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
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
impl crate::ToFormData for DeleteBucketOverwriteConfig {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteBucketOverwriteConfig {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteBucketOverwriteConfig";
    const URL_PATH: &'static str = "/?overwriteConfig";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutObject {
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
}

impl sealed::Bound for PutObject {}

impl PutObject {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
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
        }
    }
}

impl crate::Request for PutObject {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutObject";
    const URL_PATH: &'static str = "/{key}";

    type Body = crate::OctetStream;

    type ResponseWrap = PutObjectResponse;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(8);

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
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body.unwrap_or_default())
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Headers-only response: resp is the unwrapped response struct directly
        let inner = resp;
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
    pub fn new(key: impl Into<String>, x_oss_copy_source: impl Into<String>) -> Self {
        Self {
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
        Box::new([("key", self.key.to_string())])
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
    pub fn new(key: impl Into<String>) -> Self {
        Self {
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

    type ResponseWrap = Vec<u8>;

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
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AppendObject {
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
    pub fn new(key: impl Into<String>, position: impl Into<i64>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/{key}?append";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::XmlResponseWrap<AppendObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
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
        Box::new([("key", self.key.to_string())])
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
    /// Appendable Object的名称
    key: String,
    /// 用于指定从用户期望Seal时该文件的长度。
    position: i64,
}

impl sealed::Bound for SealAppendObject {}

impl SealAppendObject {
    pub fn new(key: impl Into<String>, position: impl Into<i64>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/{key}?seal";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<SealAppendObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("position".into(), (&self.position).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("key", self.key.to_string())])
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
    /// Object的完整路径。
    key: String,
    /// Object对应的版本ID。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for DeleteObject {}

impl DeleteObject {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
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

    type ResponseWrap = DeleteObjectResponse;

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
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }

    fn from_headers(resp: &mut Self::ResponseWrap, headers: &http::HeaderMap<http::HeaderValue>) {
        // Headers-only response: resp is the unwrapped response struct directly
        let inner = resp;
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
    pub fn new(key: impl Into<String>) -> Self {
        Self {
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
    const METHOD: http::Method = http::Method::GET;

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
        Box::new([("key", self.key.to_string())])
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
        if let Some(value) = headers.get("x-oss-transition-time") {
            if let Ok(s) = value.to_str() {
                inner.x_oss_transition_time = Some(s.to_string());
            }
        }
        if let Some(value) = headers.get("x‑oss‑tagging‑count") {
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
    /// Object的完整路径。
    key: String,
    /// Object的版本ID。只有查看Object指定版本的元数据信息时才显示该字段。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObjectMeta {}

impl GetObjectMeta {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: None,
        }
    }
}

impl crate::Request for GetObjectMeta {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObjectMeta";
    const URL_PATH: &'static str = "/{key}?objectMeta";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetObjectMetaResponse>;

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
        Box::new([("key", self.key.to_string())])
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
    /// Object的完整路径。
    key: String,
    /// 请求解冻的Obejct的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
    /// 保存解冻请求信息的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<RestoreObjectbody>,
}

impl sealed::Bound for RestoreObject {}

impl RestoreObject {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: None,
            body: None,
        }
    }
}

impl crate::Request for RestoreObject {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RestoreObject";
    const URL_PATH: &'static str = "/{key}?restore";

    type Body = crate::XmlBody<RestoreObjectbody>;

    type ResponseWrap = crate::XmlResponseWrap<RestoreObjectResponse>;

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
        Box::new([("key", self.key.to_string())])
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
    /// 要清理的解冻副本文件名称
    key: String,
}

impl sealed::Bound for CleanRestoredObject {}

impl CleanRestoredObject {
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
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
    const URL_PATH: &'static str = "/{key}?cleanRestoredObject";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SelectObject {
    /// Object的完整路径。
    key: String,
    /// 如果是csv文件，该值需要设置为csv/select；如果是json文件，则需要设置为json/select。
    x_oss_process: String,
    /// 保存SelectObject请求的容器。
    #[setters(generate = true, strip_option)]
    body: Option<SelectObjectbody>,
}

impl sealed::Bound for SelectObject {}

impl SelectObject {
    pub fn new(key: impl Into<String>, x_oss_process: impl Into<String>) -> Self {
        Self {
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

    type Body = crate::XmlBody<SelectObjectbody>;

    type ResponseWrap = Vec<u8>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("x-oss-process".into(), (&self.x_oss_process).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateSelectObjectMeta {
    /// Object的完整路径。
    key: String,
    /// 如果是csv文件，该值需要设置为csv/meta；如果是json文件，则需要设置为json/meta。
    x_oss_process: String,
    /// 保存CreateSelectObjectMeta请求的容器。
    #[setters(generate = true, strip_option)]
    body: Option<ObjectMetabody>,
}

impl sealed::Bound for CreateSelectObjectMeta {}

impl CreateSelectObjectMeta {
    pub fn new(key: impl Into<String>, x_oss_process: impl Into<String>) -> Self {
        Self {
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

    type Body = crate::XmlBody<ObjectMetabody>;

    type ResponseWrap = crate::XmlResponseWrap<CreateSelectObjectMetaResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("x-oss-process".into(), (&self.x_oss_process).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct InitiateMultipartUpload {
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
    pub fn new(key: impl Into<String>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/{key}?uploads";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<InitiateMultipartUploadResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

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
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UploadPart {
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
        key: impl Into<String>,
        part_number: impl Into<i64>,
        upload_id: impl Into<String>,
    ) -> Self {
        Self {
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

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

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
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CompleteMultipartUpload {
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
    body: Option<MultipartUploadbody>,
}

impl sealed::Bound for CompleteMultipartUpload {}

impl CompleteMultipartUpload {
    pub fn new(key: impl Into<String>, upload_id: impl Into<String>) -> Self {
        Self {
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

    type Body = crate::XmlBody<MultipartUploadbody>;

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
        Box::new([("key", self.key.to_string())])
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
        key: impl Into<String>,
        x_oss_copy_source: impl Into<String>,
        part_number: impl Into<i64>,
        upload_id: impl Into<String>,
    ) -> Self {
        Self {
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
        Box::new([("key", self.key.to_string())])
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
    /// Object完整路径。
    key: String,
    /// 此次MultipartUpload事件的唯一标识。
    upload_id: String,
}

impl sealed::Bound for AbortMultipartUpload {}

impl AbortMultipartUpload {
    pub fn new(key: impl Into<String>, upload_id: impl Into<String>) -> Self {
        Self {
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

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("uploadId".into(), (&self.upload_id).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
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
    encoding_type: Option<EncodeType>,
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
    const URL_PATH: &'static str = "/?uploads";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListMultipartUploadsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(6);

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

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListParts {
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
    pub fn new(key: impl Into<String>, upload_id: impl Into<String>) -> Self {
        Self {
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
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutObjectAcl {
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
    pub fn new(key: impl Into<String>, x_oss_object_acl: impl Into<ObjectACL>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/{key}?acl";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<PutObjectAclResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

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
        Box::new([("key", self.key.to_string())])
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
    /// Object的完整路径。
    key: String,
    /// Object对应的版本。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObjectAcl {}

impl GetObjectAcl {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: None,
        }
    }
}

impl crate::Request for GetObjectAcl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObjectAcl";
    const URL_PATH: &'static str = "/{key}?acl";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetObjectAclResponse>;

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
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutSymlink {
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
    pub fn new(key: impl Into<String>, x_oss_symlink_target: impl Into<String>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/{key}?symlink";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<PutSymlinkResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
        Box::new([("key", self.key.to_string())])
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
    /// Object完整路径。
    key: String,
    /// 获取软链接的当前Object版本。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetSymlink {}

impl GetSymlink {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: None,
        }
    }
}

impl crate::Request for GetSymlink {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetSymlink";
    const URL_PATH: &'static str = "/{key}?symlink";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetSymlinkResponse>;

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
        Box::new([("key", self.key.to_string())])
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
    /// Object的完整路径。
    key: String,
    /// 版本对应的ID。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
    /// 保存标签集合的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<ObjectTaggingbody>,
}

impl sealed::Bound for PutObjectTagging {}

impl PutObjectTagging {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: None,
            body: None,
        }
    }
}

impl crate::Request for PutObjectTagging {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutObjectTagging";
    const URL_PATH: &'static str = "/{key}?tagging";

    type Body = crate::XmlBody<ObjectTaggingbody>;

    type ResponseWrap = crate::XmlResponseWrap<PutObjectTaggingResponse>;

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
        Box::new([("key", self.key.to_string())])
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
    /// Object的完整路径。
    key: String,
    /// 获取的目标Object的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for GetObjectTagging {}

impl GetObjectTagging {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: None,
        }
    }
}

impl crate::Request for GetObjectTagging {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetObjectTagging";
    const URL_PATH: &'static str = "/{key}?tagging";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetObjectTaggingResponse>;

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
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteObjectTagging {
    /// Object完整路径。
    key: String,
    /// 删除的Object的版本号。
    #[setters(generate = true, strip_option)]
    version_id: Option<String>,
}

impl sealed::Bound for DeleteObjectTagging {}

impl DeleteObjectTagging {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/{key}?tagging";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

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
        Box::new([("key", self.key.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutLiveChannel {
    /// 频道名称。不能包含斜杠（/）。
    channel: String,
    /// 保存LiveChannel配置的请求体。
    #[setters(generate = true, strip_option)]
    body: Option<LiveChannelbody>,
}

impl sealed::Bound for PutLiveChannel {}

impl PutLiveChannel {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            body: None,
        }
    }
}

impl crate::Request for PutLiveChannel {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutLiveChannel";
    const URL_PATH: &'static str = "/{channel}?live";

    type Body = crate::XmlBody<LiveChannelbody>;

    type ResponseWrap = crate::XmlResponseWrap<PutLiveChannelResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("channel", self.channel.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::XmlBody(self.body.unwrap_or_default())
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
    const URL_PATH: &'static str = "/?live";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListLiveChannelResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

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

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteLiveChannel {
    /// 指定LiveChannel名称，该LiveChannel必须存在。
    channel: String,
}

impl sealed::Bound for DeleteLiveChannel {}

impl DeleteLiveChannel {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/{channel}?live";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("channel", self.channel.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutLiveChannelStatus {
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
    pub fn new(channel: impl Into<String>, status: impl Into<String>) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/{channel}?live";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("status".into(), (&self.status).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("channel", self.channel.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLiveChannelInfo {
    /// 频道名称。不能包含斜杠（/）。
    channel: String,
}

impl sealed::Bound for GetLiveChannelInfo {}

impl GetLiveChannelInfo {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
}

impl crate::Request for GetLiveChannelInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLiveChannelInfo";
    const URL_PATH: &'static str = "/{channel}?live";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetLiveChannelInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("channel", self.channel.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLiveChannelHistory {
    /// LiveChannel名称。
    channel: String,
}

impl sealed::Bound for GetLiveChannelHistory {}

impl GetLiveChannelHistory {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
}

impl crate::Request for GetLiveChannelHistory {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLiveChannelHistory";
    const URL_PATH: &'static str = "/{channel}?live&comp=history";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetLiveChannelHistoryResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("channel", self.channel.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLiveChannelStat {
    /// Livechannel名称。
    channel: String,
}

impl sealed::Bound for GetLiveChannelStat {}

impl GetLiveChannelStat {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
}

impl crate::Request for GetLiveChannelStat {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLiveChannelStat";
    const URL_PATH: &'static str = "/{channel}?live&comp=stat";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetLiveChannelStatResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("channel", self.channel.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetVodPlaylist {
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
        channel: impl Into<String>,
        end_time: impl Into<String>,
        start_time: impl Into<String>,
    ) -> Self {
        Self {
            channel: channel.into(),
            end_time: end_time.into(),
            start_time: start_time.into(),
        }
    }
}

impl crate::Request for GetVodPlaylist {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetVodPlaylist";
    const URL_PATH: &'static str = "/{channel}?vod";

    type Body = ();

    type ResponseWrap = Vec<u8>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("endTime".into(), (&self.end_time).into()));
        params.push(("startTime".into(), (&self.start_time).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("channel", self.channel.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PostVodPlaylist {
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
        channel: impl Into<String>,
        playlist: impl Into<String>,
        end_time: impl Into<String>,
        start_time: impl Into<String>,
    ) -> Self {
        Self {
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
    const URL_PATH: &'static str = "/{channel}/{playlist}?vod";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("endTime".into(), (&self.end_time).into()));
        params.push(("startTime".into(), (&self.start_time).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("channel", self.channel.to_string()),
            ("playlist", self.playlist.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutChannel {
    /// 接口请求体
    #[setters(generate = true, strip_option)]
    body: Option<PutChannelbody>,
}

impl sealed::Bound for PutChannel {}

impl PutChannel {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutChannel {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutChannel";
    const URL_PATH: &'static str = "/?img";

    type Body = crate::XmlBody<PutChannelbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutBucketHash {
    /// 接口请求体
    #[setters(generate = true, strip_option)]
    body: Option<BucketHashbody>,
}

impl sealed::Bound for PutBucketHash {}

impl PutBucketHash {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketHash {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketHash";
    const URL_PATH: &'static str = "/?objectHash";

    type Body = crate::XmlBody<BucketHashbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutBucketCommonHeader {
    /// 接口请求体
    #[setters(generate = true, strip_option)]
    body: Option<CommonHeaderbody>,
}

impl sealed::Bound for PutBucketCommonHeader {}

impl PutBucketCommonHeader {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutBucketCommonHeader {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutBucketCommonHeader";
    const URL_PATH: &'static str = "/?x-oss-common-header";

    type Body = crate::XmlBody<CommonHeaderbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct DeleteBucketCommonHeader {}

impl sealed::Bound for DeleteBucketCommonHeader {}

impl DeleteBucketCommonHeader {
    pub fn new() -> Self {
        Self {}
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
    const URL_PATH: &'static str = "/?x-oss-common-header";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
pub struct PutProcessConfiguration {
    /// 请求结构体
    #[setters(generate = true, strip_option)]
    body: Option<ProcessConfigurationbody>,
}

impl sealed::Bound for PutProcessConfiguration {}

impl PutProcessConfiguration {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for PutProcessConfiguration {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutProcessConfiguration";
    const URL_PATH: &'static str = "/?processConfiguration";

    type Body = crate::XmlBody<ProcessConfigurationbody>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
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
    const URL_PATH: &'static str = "/?eventNotification";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<GetBucketEventNotificationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutDataLakeCachePrefetchJob {
    #[setters(generate = true, strip_option)]
    x_oss_datalake_job_id: Option<String>,
    #[setters(generate = true, strip_option)]
    body: Option<PrefetchJobbody>,
}

impl sealed::Bound for PutDataLakeCachePrefetchJob {}

impl PutDataLakeCachePrefetchJob {
    pub fn new() -> Self {
        Self {
            x_oss_datalake_job_id: None,
            body: None,
        }
    }
}

impl crate::Request for PutDataLakeCachePrefetchJob {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutDataLakeCachePrefetchJob";
    const URL_PATH: &'static str = "/?x-oss-datalake-cache-prefetch-job";

    type Body = crate::XmlBody<PrefetchJobbody>;

    type ResponseWrap = crate::XmlResponseWrap<PutDataLakeCachePrefetchJobResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.x_oss_datalake_job_id {
            params.push(("x-oss-datalake-job-id".into(), (f).into()));
        }

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
impl crate::ToFormData for StartDataLakeCachePrefetchJob {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for StartDataLakeCachePrefetchJob {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "StartDataLakeCachePrefetchJob";
    const URL_PATH: &'static str = "/?x-oss-datalake-cache-prefetch-job&x-oss-datalake-job-start";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::XmlResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push((
            "x-oss-datalake-job-id".into(),
            (&self.x_oss_datalake_job_id).into(),
        ));

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
    const URL_PATH: &'static str = "/?x-oss-datalake-storage-transfer-job";

    type Body = ();

    type ResponseWrap = crate::XmlResponseWrap<ListDataLakeStorageTransferJobResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Bucket {
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: String,
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: String,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "Region")]
    pub region: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResultBuckets {
    #[serde(rename = "Bucket")]
    pub bucket: Vec<Bucket>,
}

impl crate::FlatSerialize for ResultBuckets {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketsResult {
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Marker")]
    pub marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i64,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
    #[serde(rename = "Buckets")]
    pub buckets: ResultBuckets,
}

impl crate::FlatSerialize for BucketsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
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
        crate::FlatSerialize::flat_serialize(&self.buckets, &format!("{}.Buckets", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegionInfo {
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "InternetEndpoint")]
    pub internet_endpoint: String,
    #[serde(rename = "InternalEndpoint")]
    pub internal_endpoint: String,
    #[serde(rename = "AccelerateEndpoint")]
    pub accelerate_endpoint: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InfoList {
    #[serde(rename = "RegionInfo")]
    pub region_info: Vec<RegionInfo>,
}

impl crate::FlatSerialize for InfoList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
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
pub struct BucketStat {
    #[serde(rename = "Storage")]
    pub storage: i64,
    #[serde(rename = "ObjectCount")]
    pub object_count: i64,
    #[serde(rename = "MultipartUploadCount")]
    pub multipart_upload_count: i64,
    #[serde(rename = "LiveChannelCount")]
    pub live_channel_count: i64,
    #[serde(rename = "MultipartPartCount")]
    pub multipart_part_count: i64,
    #[serde(rename = "DeleteMarkerCount")]
    pub delete_marker_count: i64,
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: i64,
    #[serde(rename = "StandardStorage")]
    pub standard_storage: i64,
    #[serde(rename = "StandardObjectCount")]
    pub standard_object_count: i64,
    #[serde(rename = "InfrequentAccessStorage")]
    pub infrequent_access_storage: i64,
    #[serde(rename = "InfrequentAccessRealStorage")]
    pub infrequent_access_real_storage: i64,
    #[serde(rename = "InfrequentAccessObjectCount")]
    pub infrequent_access_object_count: i64,
    #[serde(rename = "ArchiveStorage")]
    pub archive_storage: i64,
    #[serde(rename = "ArchiveRealStorage")]
    pub archive_real_storage: i64,
    #[serde(rename = "ArchiveObjectCount")]
    pub archive_object_count: i64,
    #[serde(rename = "ColdArchiveStorage")]
    pub cold_archive_storage: i64,
    #[serde(rename = "ColdArchiveRealStorage")]
    pub cold_archive_real_storage: i64,
    #[serde(rename = "ColdArchiveObjectCount")]
    pub cold_archive_object_count: i64,
    #[serde(rename = "DeepColdArchiveStorage")]
    pub deep_cold_archive_storage: i64,
    #[serde(rename = "DeepColdArchiveRealStorage")]
    pub deep_cold_archive_real_storage: i64,
    #[serde(rename = "DeepColdArchiveObjectCount")]
    pub deep_cold_archive_object_count: i64,
    #[serde(rename = "MultipartPartStorage")]
    pub multipart_part_storage: i64,
    #[serde(rename = "StandardMultipartPartCount")]
    pub standard_multipart_part_count: i64,
    #[serde(rename = "StandardMultipartPartStorage")]
    pub standard_multipart_part_storage: i64,
    #[serde(rename = "InfrequentMultipartPartCount")]
    pub infrequent_multipart_part_count: i64,
    #[serde(rename = "InfrequentMultipartPartStorage")]
    pub infrequent_multipart_part_storage: i64,
    #[serde(rename = "ArchiveMultipartPartCount")]
    pub archive_multipart_part_count: i64,
    #[serde(rename = "ArchiveMultipartPartStorage")]
    pub archive_multipart_part_storage: i64,
    #[serde(rename = "ColdArchiveMultipartPartCount")]
    pub cold_archive_multipart_part_count: i64,
    #[serde(rename = "ColdArchiveMultipartPartStorage")]
    pub cold_archive_multipart_part_storage: i64,
    #[serde(rename = "DeepColdArchiveMultipartPartCount")]
    pub deep_cold_archive_multipart_part_count: i64,
    #[serde(rename = "DeepColdArchiveMultipartPartStorage")]
    pub deep_cold_archive_multipart_part_storage: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateBucketConfiguration {
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "DataRedundancyType")]
    pub data_redundancy_type: DataRedundancyType,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketbody {
    #[serde(rename = "CreateBucketConfiguration")]
    pub create_bucket_configuration: CreateBucketConfiguration,
}

impl crate::FlatSerialize for PutBucketbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.create_bucket_configuration,
            &format!("{}.CreateBucketConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectSummary {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "RestoreInfo")]
    pub restore_info: String,
    #[serde(rename = "TransitionTime")]
    pub transition_time: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommonPrefix {
    #[serde(rename = "Prefix")]
    pub prefix: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectsResponseListBucketResult {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Marker")]
    pub marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i32,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
    #[serde(rename = "Contents")]
    pub contents: Vec<ObjectSummary>,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Vec<CommonPrefix>,
}

impl crate::FlatSerialize for ObjectsResponseListBucketResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.marker, &format!("{}.Marker", name), params);
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.delimiter,
            &format!("{}.Delimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_marker,
            &format!("{}.NextMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.contents, &format!("{}.Contents", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.common_prefixes,
            &format!("{}.CommonPrefixes", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct V2ResponseListBucketResult {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "StartAfter")]
    pub start_after: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i32,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "KeyCount")]
    pub key_count: i32,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: String,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: String,
    #[serde(rename = "Contents")]
    pub contents: Vec<ObjectSummary>,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Vec<CommonPrefix>,
}

impl crate::FlatSerialize for V2ResponseListBucketResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.start_after,
            &format!("{}.StartAfter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.delimiter,
            &format!("{}.Delimiter", name),
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
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
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
        crate::FlatSerialize::flat_serialize(&self.contents, &format!("{}.Contents", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.common_prefixes,
            &format!("{}.CommonPrefixes", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessControlList {
    #[serde(rename = "Grant")]
    pub grant: ObjectACL,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketInfoBucketServerSideEncryptionRule {
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: String,
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_master_key_id: String,
    #[serde(rename = "KMSDataEncryption")]
    pub kms_data_encryption: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketInfoBucketBucketPolicy {
    #[serde(rename = "LogBucket")]
    pub log_bucket: String,
    #[serde(rename = "LogPrefix")]
    pub log_prefix: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketInfoBucket {
    #[serde(rename = "AccessMonitor")]
    pub access_monitor: String,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    #[serde(rename = "CrossRegionReplication")]
    pub cross_region_replication: String,
    #[serde(rename = "DataRedundancyType")]
    pub data_redundancy_type: DataRedundancyType,
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: String,
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: String,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: String,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "TransferAcceleration")]
    pub transfer_acceleration: String,
    #[serde(rename = "Versioning")]
    pub versioning: BucketVersioningStatus,
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "AccessControlList")]
    pub access_control_list: AccessControlList,
    #[serde(rename = "ServerSideEncryptionRule")]
    pub server_side_encryption_rule: BucketInfoBucketServerSideEncryptionRule,
    #[serde(rename = "BucketPolicy")]
    pub bucket_policy: BucketInfoBucketBucketPolicy,
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "BlockPublicAccess")]
    pub block_public_access: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketInfo {
    #[serde(rename = "Bucket")]
    pub bucket: BucketInfoBucket,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessPointVpcConfiguration {
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessPoint {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "AccessPointName")]
    pub access_point_name: String,
    #[serde(rename = "Alias")]
    pub alias: String,
    #[serde(rename = "NetworkOrigin")]
    pub network_origin: String,
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: AccessPointVpcConfiguration,
    #[serde(rename = "Status")]
    pub status: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsResultAccessPoints {
    #[serde(rename = "AccessPoint")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsResult {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: String,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: String,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "AccessPoints")]
    pub access_points: ListAccessPointsResultAccessPoints,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i32,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointResultEndpoints {
    #[serde(rename = "PublicEndpoint")]
    pub public_endpoint: String,
    #[serde(rename = "InternalEndpoint")]
    pub internal_endpoint: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PublicAccessBlockConfiguration {
    #[serde(rename = "BlockPublicAccess")]
    pub block_public_access: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointResult {
    #[serde(rename = "AccessPointName")]
    pub access_point_name: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "NetworkOrigin")]
    pub network_origin: String,
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: AccessPointVpcConfiguration,
    #[serde(rename = "AccessPointArn")]
    pub access_point_arn: String,
    #[serde(rename = "Alias")]
    pub alias: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Endpoints")]
    pub endpoints: GetAccessPointResultEndpoints,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointConfiguration {
    #[serde(rename = "AccessPointName")]
    pub access_point_name: String,
    #[serde(rename = "NetworkOrigin")]
    pub network_origin: String,
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: AccessPointVpcConfiguration,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessPointbody {
    #[serde(rename = "CreateAccessPointConfiguration")]
    pub create_access_point_configuration: CreateAccessPointConfiguration,
}

impl crate::FlatSerialize for AccessPointbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.create_access_point_configuration,
            &format!("{}.CreateAccessPointConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointResult {
    #[serde(rename = "AccessPointArn")]
    pub access_point_arn: String,
    #[serde(rename = "Alias")]
    pub alias: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InitiateWormConfiguration {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InitiateBucketWormbody {
    #[serde(rename = "InitiateWormConfiguration")]
    pub initiate_worm_configuration: InitiateWormConfiguration,
}

impl crate::FlatSerialize for InitiateBucketWormbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.initiate_worm_configuration,
            &format!("{}.InitiateWormConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ExtendWormConfiguration {
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: i32,
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
pub struct ExtendBucketWormbody {
    #[serde(rename = "ExtendWormConfiguration")]
    pub extend_worm_configuration: ExtendWormConfiguration,
}

impl crate::FlatSerialize for ExtendBucketWormbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.extend_worm_configuration,
            &format!("{}.ExtendWormConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WormConfiguration {
    #[serde(rename = "WormId")]
    pub worm_id: String,
    #[serde(rename = "State")]
    pub state: BucketWormState,
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: i32,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    #[serde(rename = "ExpirationDate")]
    pub expiration_date: String,
}

impl crate::FlatSerialize for WormConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.worm_id, &format!("{}.WormId", name), params);
        crate::FlatSerialize::flat_serialize(&self.state, &format!("{}.State", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.retention_period_in_days,
            &format!("{}.RetentionPeriodInDays", name),
            params,
        );
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
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAclResponseAccessControlPolicyAccessControlList {
    #[serde(rename = "Grant")]
    pub grant: BucketACL,
}

impl crate::FlatSerialize for BucketAclResponseAccessControlPolicyAccessControlList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.grant, &format!("{}.Grant", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAclResponseAccessControlPolicy {
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "AccessControlList")]
    pub access_control_list: BucketAclResponseAccessControlPolicyAccessControlList,
}

impl crate::FlatSerialize for BucketAclResponseAccessControlPolicy {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleExpiration {
    #[serde(rename = "CreatedBeforeDate")]
    pub created_before_date: String,
    #[serde(rename = "Days")]
    pub days: i32,
    #[serde(rename = "ExpiredObjectDeleteMarker")]
    pub expired_object_delete_marker: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleTransitionItem {
    #[serde(rename = "CreatedBeforeDate")]
    pub created_before_date: String,
    #[serde(rename = "Days")]
    pub days: i32,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "IsAccessTime")]
    pub is_access_time: bool,
    #[serde(rename = "ReturnToStdWhenVisit")]
    pub return_to_std_when_visit: bool,
    #[serde(rename = "AllowSmallFile")]
    pub allow_small_file: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleAbortMultipartUpload {
    #[serde(rename = "Days")]
    pub days: i32,
    #[serde(rename = "CreatedBeforeDate")]
    pub created_before_date: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleNoncurrentVersionExpiration {
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: i32,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleNoncurrentVersionTransitionItem {
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: i32,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "IsAccessTime")]
    pub is_access_time: bool,
    #[serde(rename = "ReturnToStdWhenVisit")]
    pub return_to_std_when_visit: bool,
    #[serde(rename = "AllowSmallFile")]
    pub allow_small_file: bool,
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
pub struct LifecycleRuleFilterNot {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Tag")]
    pub tag: Tag,
}

impl crate::FlatSerialize for LifecycleRuleFilterNot {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRuleFilter {
    #[serde(rename = "Not")]
    pub not: LifecycleRuleFilterNot,
    #[serde(rename = "ObjectSizeGreaterThan")]
    pub object_size_greater_than: i64,
    #[serde(rename = "ObjectSizeLessThan")]
    pub object_size_less_than: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleRule {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Expiration")]
    pub expiration: LifecycleRuleExpiration,
    #[serde(rename = "Transition")]
    pub transition: Vec<LifecycleRuleTransitionItem>,
    #[serde(rename = "AbortMultipartUpload")]
    pub abort_multipart_upload: LifecycleRuleAbortMultipartUpload,
    #[serde(rename = "Tag")]
    pub tag: Vec<Tag>,
    #[serde(rename = "NoncurrentVersionExpiration")]
    pub noncurrent_version_expiration: LifecycleRuleNoncurrentVersionExpiration,
    #[serde(rename = "NoncurrentVersionTransition")]
    pub noncurrent_version_transition: Vec<LifecycleRuleNoncurrentVersionTransitionItem>,
    #[serde(rename = "Filter")]
    pub filter: LifecycleRuleFilter,
    #[serde(rename = "AtimeBase")]
    pub atime_base: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rule")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketLifecyclebody {
    #[serde(rename = "LifecycleConfiguration")]
    pub lifecycle_configuration: LifecycleConfiguration,
}

impl crate::FlatSerialize for BucketLifecyclebody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.lifecycle_configuration,
            &format!("{}.LifecycleConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TransferAccelerationConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TransferAccelerationbody {
    #[serde(rename = "TransferAccelerationConfiguration")]
    pub transfer_acceleration_configuration: TransferAccelerationConfiguration,
}

impl crate::FlatSerialize for TransferAccelerationbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.transfer_acceleration_configuration,
            &format!("{}.TransferAccelerationConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccelerationConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl crate::FlatSerialize for AccelerationConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.enabled, &format!("{}.Enabled", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VersioningConfiguration {
    #[serde(rename = "Status")]
    pub status: BucketVersioningStatus,
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
pub struct BucketVersioningbody {
    #[serde(rename = "VersioningConfiguration")]
    pub versioning_configuration: VersioningConfiguration,
}

impl crate::FlatSerialize for BucketVersioningbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.versioning_configuration,
            &format!("{}.VersioningConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResponseVersioningConfiguration {
    #[serde(rename = "Status")]
    pub status: BucketVersioningStatus,
}

impl crate::FlatSerialize for ResponseVersioningConfiguration {
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
pub struct ObjectVersion {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
    #[serde(rename = "IsLatest")]
    pub is_latest: bool,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "RestoreInfo")]
    pub restore_info: String,
    #[serde(rename = "TransitionTime")]
    pub transition_time: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeleteMarkerEntry {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
    #[serde(rename = "IsLatest")]
    pub is_latest: bool,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "Owner")]
    pub owner: Owner,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VersionsResult {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "KeyMarker")]
    pub key_marker: String,
    #[serde(rename = "VersionIdMarker")]
    pub version_id_marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i64,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "NextKeyMarker")]
    pub next_key_marker: String,
    #[serde(rename = "NextVersionIdMarker")]
    pub next_version_id_marker: String,
    #[serde(rename = "Version")]
    pub version: Vec<ObjectVersion>,
    #[serde(rename = "DeleteMarker")]
    pub delete_marker: Vec<DeleteMarkerEntry>,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Vec<CommonPrefix>,
}

impl crate::FlatSerialize for VersionsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.Name", name), params);
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.key_marker,
            &format!("{}.KeyMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.version_id_marker,
            &format!("{}.VersionIdMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.max_keys, &format!("{}.MaxKeys", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.delimiter,
            &format!("{}.Delimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
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
        crate::FlatSerialize::flat_serialize(&self.version, &format!("{}.Version", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.delete_marker,
            &format!("{}.DeleteMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.common_prefixes,
            &format!("{}.CommonPrefixes", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PolicyStatus {
    #[serde(rename = "IsPublic")]
    pub is_public: bool,
}

impl crate::FlatSerialize for PolicyStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
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
pub struct RTC {
    #[serde(rename = "Status")]
    pub status: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RtcConfiguration {
    #[serde(rename = "RTC")]
    pub rtc: RTC,
    #[serde(rename = "ID")]
    pub id: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketRtcbody {
    #[serde(rename = "ReplicationRule")]
    pub replication_rule: RtcConfiguration,
}

impl crate::FlatSerialize for BucketRtcbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.replication_rule,
            &format!("{}.ReplicationRule", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationPrefixSet {
    #[serde(rename = "Prefix")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationDestination {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "TransferType")]
    pub transfer_type: ReplicationDestinationTransferType,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationSourceSelectionCriteriaSseKmsEncryptedObjects {
    #[serde(rename = "Status")]
    pub status: ReplicationSourceSelectionCriteriaSseKmsEncryptedObjectsStatus,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationSourceSelectionCriteria {
    #[serde(rename = "SseKmsEncryptedObjects")]
    pub sse_kms_encrypted_objects: ReplicationSourceSelectionCriteriaSseKmsEncryptedObjects,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationEncryptionConfiguration {
    #[serde(rename = "ReplicaKmsKeyID")]
    pub replica_kms_key_id: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationRule {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "PrefixSet")]
    pub prefix_set: ReplicationPrefixSet,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "Destination")]
    pub destination: ReplicationDestination,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "HistoricalObjectReplication")]
    pub historical_object_replication: ReplicationRuleHistoricalObjectReplication,
    #[serde(rename = "SyncRole")]
    pub sync_role: String,
    #[serde(rename = "SourceSelectionCriteria")]
    pub source_selection_criteria: ReplicationSourceSelectionCriteria,
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: ReplicationEncryptionConfiguration,
    #[serde(rename = "RTC")]
    pub rtc: RTC,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationConfiguration {
    #[serde(rename = "Rule")]
    pub rule: ReplicationRule,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutBucketReplicationbody {
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: ReplicationConfiguration,
}

impl crate::FlatSerialize for PutBucketReplicationbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.replication_configuration,
            &format!("{}.ReplicationConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResponseReplicationConfiguration {
    #[serde(rename = "Rule")]
    pub rule: Vec<ReplicationRule>,
}

impl crate::FlatSerialize for ResponseReplicationConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LocationTransferTypeTransferTypes {
    #[serde(rename = "Type")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LocationTransferType {
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "TransferTypes")]
    pub transfer_types: LocationTransferTypeTransferTypes,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TypeConstraint {
    #[serde(rename = "LocationTransferType")]
    pub location_transfer_type: Vec<LocationTransferType>,
}

impl crate::FlatSerialize for TypeConstraint {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RtcConstraint {
    #[serde(rename = "Location")]
    pub location: Vec<String>,
}

impl crate::FlatSerialize for RtcConstraint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationLocation {
    #[serde(rename = "Location")]
    pub location: Vec<String>,
    #[serde(rename = "LocationTransferTypeConstraint")]
    pub location_transfer_type_constraint: TypeConstraint,
    #[serde(rename = "LocationRTCConstraint")]
    pub location_rtc_constraint: RtcConstraint,
}

impl crate::FlatSerialize for ReplicationLocation {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.location_transfer_type_constraint,
            &format!("{}.LocationTransferTypeConstraint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.location_rtc_constraint,
            &format!("{}.LocationRTCConstraint", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationProgressRuleProgress {
    #[serde(rename = "HistoricalObject")]
    pub historical_object: String,
    #[serde(rename = "NewObject")]
    pub new_object: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationProgressRule {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "PrefixSet")]
    pub prefix_set: ReplicationPrefixSet,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "Destination")]
    pub destination: ReplicationDestination,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "HistoricalObjectReplication")]
    pub historical_object_replication: String,
    #[serde(rename = "Progress")]
    pub progress: ReplicationProgressRuleProgress,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationProgress {
    #[serde(rename = "Rule")]
    pub rule: Vec<ReplicationProgressRule>,
}

impl crate::FlatSerialize for ReplicationProgress {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.rule, &format!("{}.Rule", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReplicationbodyReplicationRules {
    #[serde(rename = "ID")]
    pub id: String,
}

impl crate::FlatSerialize for ReplicationbodyReplicationRules {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeleteBucketReplicationbody {
    #[serde(rename = "ReplicationRules")]
    pub replication_rules: ReplicationbodyReplicationRules,
}

impl crate::FlatSerialize for DeleteBucketReplicationbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.replication_rules,
            &format!("{}.ReplicationRules", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SSEKMS {
    #[serde(rename = "KeyId")]
    pub key_id: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryEncryption {
    #[serde(rename = "SSE-OSS")]
    pub sseoss: String,
    #[serde(rename = "SSE-KMS")]
    pub ssekms: SSEKMS,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryOSSBucketDestination {
    #[serde(rename = "Format")]
    pub format: InventoryFormat,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Encryption")]
    pub encryption: InventoryEncryption,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryDestination {
    #[serde(rename = "OSSBucketDestination")]
    pub oss_bucket_destination: InventoryOSSBucketDestination,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventorySchedule {
    #[serde(rename = "Frequency")]
    pub frequency: InventoryFrequency,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryFilter {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "LastModifyBeginTimeStamp")]
    pub last_modify_begin_time_stamp: i64,
    #[serde(rename = "LastModifyEndTimeStamp")]
    pub last_modify_end_time_stamp: i64,
    #[serde(rename = "LowerSizeBound")]
    pub lower_size_bound: i64,
    #[serde(rename = "UpperSizeBound")]
    pub upper_size_bound: i64,
    #[serde(rename = "StorageClass")]
    pub storage_class: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryConfigurationOptionalFields {
    #[serde(rename = "Field")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InventoryConfiguration {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "Destination")]
    pub destination: InventoryDestination,
    #[serde(rename = "Schedule")]
    pub schedule: InventorySchedule,
    #[serde(rename = "Filter")]
    pub filter: InventoryFilter,
    #[serde(rename = "IncludedObjectVersions")]
    pub included_object_versions: String,
    #[serde(rename = "OptionalFields")]
    pub optional_fields: InventoryConfigurationOptionalFields,
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
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketInventorybody {
    #[serde(rename = "InventoryConfiguration")]
    pub inventory_configuration: InventoryConfiguration,
}

impl crate::FlatSerialize for BucketInventorybody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.inventory_configuration,
            &format!("{}.InventoryConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigurationsResult {
    #[serde(rename = "InventoryConfiguration")]
    pub inventory_configuration: Vec<InventoryConfiguration>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: String,
}

impl crate::FlatSerialize for ConfigurationsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
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
pub struct LoggingEnabled {
    #[serde(rename = "TargetBucket")]
    pub target_bucket: String,
    #[serde(rename = "TargetPrefix")]
    pub target_prefix: String,
    #[serde(rename = "LoggingRole")]
    pub logging_role: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketLoggingStatus {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketLoggingbody {
    #[serde(rename = "BucketLoggingStatus")]
    pub bucket_logging_status: BucketLoggingStatus,
}

impl crate::FlatSerialize for BucketLoggingbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bucket_logging_status,
            &format!("{}.BucketLoggingStatus", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserDefinedLogFieldsConfigurationHeaderSet {
    #[serde(rename = "header")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserDefinedLogFieldsConfigurationParamSet {
    #[serde(rename = "parameter")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserDefinedLogFieldsConfiguration {
    #[serde(rename = "HeaderSet")]
    pub header_set: UserDefinedLogFieldsConfigurationHeaderSet,
    #[serde(rename = "ParamSet")]
    pub param_set: UserDefinedLogFieldsConfigurationParamSet,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FieldsConfigbody {
    #[serde(rename = "UserDefinedLogFieldsConfiguration")]
    pub user_defined_log_fields_configuration: UserDefinedLogFieldsConfiguration,
}

impl crate::FlatSerialize for FieldsConfigbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.user_defined_log_fields_configuration,
            &format!("{}.UserDefinedLogFieldsConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IndexDocument {
    #[serde(rename = "Suffix")]
    pub suffix: String,
    #[serde(rename = "SupportSubDir")]
    pub support_sub_dir: bool,
    #[serde(rename = "Type")]
    pub r#type: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ErrorDocument {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "HttpStatus")]
    pub http_status: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleConditionIncludeHeaderItem {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Equals")]
    pub equals: String,
    #[serde(rename = "StartsWith")]
    pub starts_with: String,
    #[serde(rename = "EndsWith")]
    pub ends_with: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleCondition {
    #[serde(rename = "KeyPrefixEquals")]
    pub key_prefix_equals: String,
    #[serde(rename = "KeySuffixEquals")]
    pub key_suffix_equals: String,
    #[serde(rename = "HttpErrorCodeReturnedEquals")]
    pub http_error_code_returned_equals: i64,
    #[serde(rename = "IncludeHeader")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorHeadersSetItem {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorHeaders {
    #[serde(rename = "PassAll")]
    pub pass_all: bool,
    #[serde(rename = "Pass")]
    pub pass: Vec<String>,
    #[serde(rename = "Remove")]
    pub remove: Vec<String>,
    #[serde(rename = "Set")]
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
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorTaggings {
    #[serde(rename = "Taggings")]
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
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorReturnHeaders {
    #[serde(rename = "ReturnHeader")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorAuth {
    #[serde(rename = "AuthType")]
    pub auth_type: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "AccessKeyId")]
    pub access_key_id: String,
    #[serde(rename = "AccessKeySecret")]
    pub access_key_secret: String,
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
    #[serde(rename = "MirrorMultiAlternateNumber")]
    pub mirror_multi_alternate_number: i64,
    #[serde(rename = "MirrorMultiAlternateURL")]
    pub mirror_multi_alternate_url: String,
    #[serde(rename = "MirrorMultiAlternateVpcId")]
    pub mirror_multi_alternate_vpc_id: String,
    #[serde(rename = "MirrorMultiAlternateDstRegion")]
    pub mirror_multi_alternate_dst_region: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirectMirrorMultiAlternates {
    #[serde(rename = "MirrorMultiAlternate")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleRedirect {
    #[serde(rename = "RedirectType")]
    pub redirect_type: String,
    #[serde(rename = "PassQueryString")]
    pub pass_query_string: bool,
    #[serde(rename = "MirrorURL")]
    pub mirror_url: String,
    #[serde(rename = "MirrorSNI")]
    pub mirror_sni: bool,
    #[serde(rename = "MirrorPassQueryString")]
    pub mirror_pass_query_string: bool,
    #[serde(rename = "MirrorFollowRedirect")]
    pub mirror_follow_redirect: bool,
    #[serde(rename = "MirrorCheckMd5")]
    pub mirror_check_md5: bool,
    #[serde(rename = "MirrorHeaders")]
    pub mirror_headers: RoutingRuleRedirectMirrorHeaders,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "HostName")]
    pub host_name: String,
    #[serde(rename = "ReplaceKeyPrefixWith")]
    pub replace_key_prefix_with: String,
    #[serde(rename = "EnableReplacePrefix")]
    pub enable_replace_prefix: bool,
    #[serde(rename = "ReplaceKeyWith")]
    pub replace_key_with: String,
    #[serde(rename = "HttpRedirectCode")]
    pub http_redirect_code: i64,
    #[serde(rename = "MirrorPassOriginalSlashes")]
    pub mirror_pass_original_slashes: bool,
    #[serde(rename = "MirrorURLSlave")]
    pub mirror_url_slave: String,
    #[serde(rename = "MirrorURLProbe")]
    pub mirror_url_probe: String,
    #[serde(rename = "MirrorSaveOssMeta")]
    pub mirror_save_oss_meta: bool,
    #[serde(rename = "MirrorProxyPass")]
    pub mirror_proxy_pass: bool,
    #[serde(rename = "MirrorAllowGetImageInfo")]
    pub mirror_allow_get_image_info: bool,
    #[serde(rename = "MirrorAllowVideoSnapshot")]
    pub mirror_allow_video_snapshot: bool,
    #[serde(rename = "MirrorIsExpressTunnel")]
    pub mirror_is_express_tunnel: bool,
    #[serde(rename = "MirrorDstRegion")]
    pub mirror_dst_region: String,
    #[serde(rename = "MirrorDstVpcId")]
    pub mirror_dst_vpc_id: String,
    #[serde(rename = "MirrorDstSlaveVpcId")]
    pub mirror_dst_slave_vpc_id: String,
    #[serde(rename = "MirrorUserLastModified")]
    pub mirror_user_last_modified: bool,
    #[serde(rename = "MirrorSwitchAllErrors")]
    pub mirror_switch_all_errors: bool,
    #[serde(rename = "MirrorTunnelId")]
    pub mirror_tunnel_id: String,
    #[serde(rename = "MirrorUsingRole")]
    pub mirror_using_role: bool,
    #[serde(rename = "MirrorRole")]
    pub mirror_role: String,
    #[serde(rename = "MirrorAllowHeadObject")]
    pub mirror_allow_head_object: bool,
    #[serde(rename = "TransparentMirrorResponseCodes")]
    pub transparent_mirror_response_codes: String,
    #[serde(rename = "MirrorAsyncStatus")]
    pub mirror_async_status: i64,
    #[serde(rename = "MirrorTaggings")]
    pub mirror_taggings: RoutingRuleRedirectMirrorTaggings,
    #[serde(rename = "MirrorReturnHeaders")]
    pub mirror_return_headers: RoutingRuleRedirectMirrorReturnHeaders,
    #[serde(rename = "MirrorAuth")]
    pub mirror_auth: RoutingRuleRedirectMirrorAuth,
    #[serde(rename = "MirrorMultiAlternates")]
    pub mirror_multi_alternates: RoutingRuleRedirectMirrorMultiAlternates,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRuleLuaConfig {
    #[serde(rename = "Script")]
    pub script: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RoutingRule {
    #[serde(rename = "RuleNumber")]
    pub rule_number: i64,
    #[serde(rename = "Condition")]
    pub condition: RoutingRuleCondition,
    #[serde(rename = "Redirect")]
    pub redirect: RoutingRuleRedirect,
    #[serde(rename = "LuaConfig")]
    pub lua_config: RoutingRuleLuaConfig,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebsiteConfigurationRoutingRules {
    #[serde(rename = "RoutingRule")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebsiteConfiguration {
    #[serde(rename = "IndexDocument")]
    pub index_document: IndexDocument,
    #[serde(rename = "ErrorDocument")]
    pub error_document: ErrorDocument,
    #[serde(rename = "RoutingRules")]
    pub routing_rules: WebsiteConfigurationRoutingRules,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketWebsitebody {
    #[serde(rename = "WebsiteConfiguration")]
    pub website_configuration: WebsiteConfiguration,
}

impl crate::FlatSerialize for BucketWebsitebody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.website_configuration,
            &format!("{}.WebsiteConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RefererConfigurationRefererList {
    #[serde(rename = "Referer")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RefererConfigurationRefererBlacklist {
    #[serde(rename = "Referer")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RefererConfiguration {
    #[serde(rename = "AllowEmptyReferer")]
    pub allow_empty_referer: bool,
    #[serde(rename = "AllowTruncateQueryString")]
    pub allow_truncate_query_string: bool,
    #[serde(rename = "TruncatePath")]
    pub truncate_path: bool,
    #[serde(rename = "RefererList")]
    pub referer_list: RefererConfigurationRefererList,
    #[serde(rename = "RefererBlacklist")]
    pub referer_blacklist: RefererConfigurationRefererBlacklist,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketRefererbody {
    #[serde(rename = "RefererConfiguration")]
    pub referer_configuration: RefererConfiguration,
}

impl crate::FlatSerialize for BucketRefererbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.referer_configuration,
            &format!("{}.RefererConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TagSet {
    #[serde(rename = "Tag")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tagging {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketTagsbody {
    #[serde(rename = "Tagging")]
    pub tagging: Tagging,
}

impl crate::FlatSerialize for BucketTagsbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tagging, &format!("{}.Tagging", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TagsResponseTagging {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}

impl crate::FlatSerialize for TagsResponseTagging {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tag_set, &format!("{}.TagSet", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketDataRedundancyTransition {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "TaskId")]
    pub task_id: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "CreateTime")]
    pub create_time: String,
    #[serde(rename = "StartTime")]
    pub start_time: String,
    #[serde(rename = "EndTime")]
    pub end_time: String,
    #[serde(rename = "ProcessPercentage")]
    pub process_percentage: i32,
    #[serde(rename = "EstimatedRemainingTime")]
    pub estimated_remaining_time: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserDataRedundancyTransitionResponseListBucketDataRedundancyTransition {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: String,
    #[serde(rename = "BucketDataRedundancyTransition")]
    pub bucket_data_redundancy_transition: Vec<BucketDataRedundancyTransition>,
}

impl crate::FlatSerialize
    for UserDataRedundancyTransitionResponseListBucketDataRedundancyTransition
{
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
            &self.bucket_data_redundancy_transition,
            &format!("{}.BucketDataRedundancyTransition", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketDataRedundancyTransitionResponseListBucketDataRedundancyTransition {
    #[serde(rename = "BucketDataRedundancyTransition")]
    pub bucket_data_redundancy_transition: BucketDataRedundancyTransition,
}

impl crate::FlatSerialize
    for BucketDataRedundancyTransitionResponseListBucketDataRedundancyTransition
{
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
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
pub struct ResponseBucketDataRedundancyTransition {
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

impl crate::FlatSerialize for ResponseBucketDataRedundancyTransition {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.task_id, &format!("{}.TaskId", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ApplyServerSideEncryptionByDefault {
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: String,
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_master_key_id: String,
    #[serde(rename = "KMSDataEncryption")]
    pub kms_data_encryption: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    pub apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketEncryptionbody {
    #[serde(rename = "ServerSideEncryptionRule")]
    pub server_side_encryption_rule: ServerSideEncryptionRule,
}

impl crate::FlatSerialize for BucketEncryptionbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.server_side_encryption_rule,
            &format!("{}.ServerSideEncryptionRule", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    pub apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault,
}

impl crate::FlatSerialize for EncryptionRule {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RequestPaymentConfiguration {
    #[serde(rename = "Payer")]
    pub payer: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RequestPaymentbody {
    #[serde(rename = "RequestPaymentConfiguration")]
    pub request_payment_configuration: RequestPaymentConfiguration,
}

impl crate::FlatSerialize for RequestPaymentbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.request_payment_configuration,
            &format!("{}.RequestPaymentConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PaymentConfiguration {
    #[serde(rename = "Payer")]
    pub payer: String,
}

impl crate::FlatSerialize for PaymentConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.payer, &format!("{}.Payer", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CORSRule {
    #[serde(rename = "AllowedOrigin")]
    pub allowed_origin: Vec<String>,
    #[serde(rename = "AllowedMethod")]
    pub allowed_method: Vec<String>,
    #[serde(rename = "AllowedHeader")]
    pub allowed_header: Vec<String>,
    #[serde(rename = "ExposeHeader")]
    pub expose_header: Vec<String>,
    #[serde(rename = "MaxAgeSeconds")]
    pub max_age_seconds: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CORSConfiguration {
    #[serde(rename = "CORSRule")]
    pub cors_rule: Vec<CORSRule>,
    #[serde(rename = "ResponseVary")]
    pub response_vary: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketCorsbody {
    #[serde(rename = "CORSConfiguration")]
    pub cors_configuration: CORSConfiguration,
}

impl crate::FlatSerialize for BucketCorsbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.cors_configuration,
            &format!("{}.CORSConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SConfiguration {
    #[serde(rename = "CORSRule")]
    pub cors_rule: Vec<CORSRule>,
    #[serde(rename = "ResponseVary")]
    pub response_vary: bool,
}

impl crate::FlatSerialize for SConfiguration {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessMonitorConfiguration {
    #[serde(rename = "Status")]
    pub status: AccessMonitorStatus,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessMonitorbody {
    #[serde(rename = "AccessMonitorConfiguration")]
    pub access_monitor_configuration: AccessMonitorConfiguration,
}

impl crate::FlatSerialize for AccessMonitorbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_monitor_configuration,
            &format!("{}.AccessMonitorConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QueryStatus {
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Phase")]
    pub phase: String,
    #[serde(rename = "CreateTime")]
    pub create_time: String,
    #[serde(rename = "UpdateTime")]
    pub update_time: String,
}

impl crate::FlatSerialize for QueryStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.state, &format!("{}.State", name), params);
        crate::FlatSerialize::flat_serialize(&self.phase, &format!("{}.Phase", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.CreateTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.update_time,
            &format!("{}.UpdateTime", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregation {
    #[serde(rename = "Field")]
    pub field: String,
    #[serde(rename = "Operation")]
    pub operation: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregations {
    #[serde(rename = "Aggregation")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryMediaTypes {
    #[serde(rename = "MediaType")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQuery {
    #[serde(rename = "NextToken")]
    pub next_token: String,
    #[serde(rename = "MaxResults")]
    pub max_results: i64,
    #[serde(rename = "Query")]
    pub query: String,
    #[serde(rename = "Sort")]
    pub sort: String,
    #[serde(rename = "Order")]
    pub order: MetaQueryOrder,
    #[serde(rename = "Aggregations")]
    pub aggregations: MetaQueryAggregations,
    #[serde(rename = "MediaTypes")]
    pub media_types: MetaQueryMediaTypes,
    #[serde(rename = "SimpleQuery")]
    pub simple_query: String,
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
pub struct DoMetaQuerybody {
    #[serde(rename = "MetaQuery")]
    pub meta_query: MetaQuery,
}

impl crate::FlatSerialize for DoMetaQuerybody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.meta_query,
            &format!("{}.MetaQuery", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryTagging {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileOssTagging {
    #[serde(rename = "Tagging")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryUserMeta {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileOssUserMeta {
    #[serde(rename = "UserMeta")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespVideoStream {
    #[serde(rename = "CodecName")]
    pub codec_name: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Bitrate")]
    pub bitrate: i64,
    #[serde(rename = "FrameRate")]
    pub frame_rate: String,
    #[serde(rename = "StartTime")]
    pub start_time: f64,
    #[serde(rename = "Duration")]
    pub duration: f64,
    #[serde(rename = "FrameCount")]
    pub frame_count: i64,
    #[serde(rename = "BitDepth")]
    pub bit_depth: i64,
    #[serde(rename = "PixelFormat")]
    pub pixel_format: String,
    #[serde(rename = "ColorSpace")]
    pub color_space: String,
    #[serde(rename = "Height")]
    pub height: i64,
    #[serde(rename = "Width")]
    pub width: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileVideoStreams {
    #[serde(rename = "VideoStream")]
    pub video_stream: MetaQueryRespVideoStream,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespAudioStream {
    #[serde(rename = "CodecName")]
    pub codec_name: String,
    #[serde(rename = "Bitrate")]
    pub bitrate: i64,
    #[serde(rename = "SampleRate")]
    pub sample_rate: i64,
    #[serde(rename = "StartTime")]
    pub start_time: f64,
    #[serde(rename = "Duration")]
    pub duration: f64,
    #[serde(rename = "Channels")]
    pub channels: i64,
    #[serde(rename = "Language")]
    pub language: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileAudioStreams {
    #[serde(rename = "AudioStream")]
    pub audio_stream: MetaQueryRespAudioStream,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespSubtitle {
    #[serde(rename = "CodecName")]
    pub codec_name: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "StartTime")]
    pub start_time: f64,
    #[serde(rename = "Duration")]
    pub duration: f64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileSubtitles {
    #[serde(rename = "Subtitle")]
    pub subtitle: MetaQueryRespSubtitle,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespAddress {
    #[serde(rename = "AddressLine")]
    pub address_line: String,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "District")]
    pub district: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Province")]
    pub province: String,
    #[serde(rename = "Township")]
    pub township: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryFileAddresses {
    #[serde(rename = "Address")]
    pub address: MetaQueryRespAddress,
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
pub struct MetaQueryFile {
    #[serde(rename = "Filename")]
    pub filename: String,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "FileModifiedTime")]
    pub file_modified_time: String,
    #[serde(rename = "OSSObjectType")]
    pub oss_object_type: String,
    #[serde(rename = "OSSStorageClass")]
    pub oss_storage_class: String,
    #[serde(rename = "ObjectACL")]
    pub object_acl: String,
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "OSSCRC64")]
    pub osscrc64: String,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: String,
    #[serde(rename = "ServerSideEncryptionCustomerAlgorithm")]
    pub server_side_encryption_customer_algorithm: String,
    #[serde(rename = "OSSTaggingCount")]
    pub oss_tagging_count: i64,
    #[serde(rename = "OSSTagging")]
    pub oss_tagging: MetaQueryFileOssTagging,
    #[serde(rename = "OSSUserMeta")]
    pub oss_user_meta: MetaQueryFileOssUserMeta,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(rename = "ProduceTime")]
    pub produce_time: String,
    #[serde(rename = "ContentType")]
    pub content_type: String,
    #[serde(rename = "MediaType")]
    pub media_type: String,
    #[serde(rename = "LatLong")]
    pub lat_long: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "OSSExpiration")]
    pub oss_expiration: String,
    #[serde(rename = "AccessControlAllowOrigin")]
    pub access_control_allow_origin: String,
    #[serde(rename = "AccessControlRequestMethod")]
    pub access_control_request_method: String,
    #[serde(rename = "ServerSideDataEncryption")]
    pub server_side_data_encryption: String,
    #[serde(rename = "ServerSideEncryptionKeyId")]
    pub server_side_encryption_key_id: String,
    #[serde(rename = "CacheControl")]
    pub cache_control: String,
    #[serde(rename = "ContentDisposition")]
    pub content_disposition: String,
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: String,
    #[serde(rename = "ContentLanguage")]
    pub content_language: String,
    #[serde(rename = "ImageHeight")]
    pub image_height: i64,
    #[serde(rename = "ImageWidth")]
    pub image_width: i64,
    #[serde(rename = "VideoWidth")]
    pub video_width: i64,
    #[serde(rename = "VideoHeight")]
    pub video_height: i64,
    #[serde(rename = "Bitrate")]
    pub bitrate: i64,
    #[serde(rename = "Artist")]
    pub artist: String,
    #[serde(rename = "AlbumArtist")]
    pub album_artist: String,
    #[serde(rename = "Composer")]
    pub composer: String,
    #[serde(rename = "Performer")]
    pub performer: String,
    #[serde(rename = "Album")]
    pub album: String,
    #[serde(rename = "Duration")]
    pub duration: f64,
    #[serde(rename = "VideoStreams")]
    pub video_streams: MetaQueryFileVideoStreams,
    #[serde(rename = "AudioStreams")]
    pub audio_streams: MetaQueryFileAudioStreams,
    #[serde(rename = "Subtitles")]
    pub subtitles: MetaQueryFileSubtitles,
    #[serde(rename = "Addresses")]
    pub addresses: MetaQueryFileAddresses,
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
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespFiles {
    #[serde(rename = "File")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregationsResultGroupsGroupItem {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Count")]
    pub count: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregationsResultGroups {
    #[serde(rename = "Group")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryAggregationsResult {
    #[serde(rename = "Field")]
    pub field: String,
    #[serde(rename = "Operation")]
    pub operation: String,
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Groups")]
    pub groups: MetaQueryAggregationsResultGroups,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryRespAggregations {
    #[serde(rename = "Aggregation")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryResp {
    #[serde(rename = "NextToken")]
    pub next_token: String,
    #[serde(rename = "Files")]
    pub files: MetaQueryRespFiles,
    #[serde(rename = "Aggregations")]
    pub aggregations: MetaQueryRespAggregations,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryOpenRequestFilters {
    #[serde(rename = "Filter")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetaQueryOpenRequest {
    #[serde(rename = "Filters")]
    pub filters: MetaQueryOpenRequestFilters,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpenMetaQuerybody {
    #[serde(rename = "MetaQuery")]
    pub meta_query: MetaQueryOpenRequest,
}

impl crate::FlatSerialize for OpenMetaQuerybody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.meta_query,
            &format!("{}.MetaQuery", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAntiDDOSConfigurationCnames {
    #[serde(rename = "Domain")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAntiDDOSConfiguration {
    #[serde(rename = "Cnames")]
    pub cnames: BucketAntiDDOSConfigurationCnames,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateBucketAntiDDosInfobody {
    #[serde(rename = "AntiDDOSConfiguration")]
    pub anti_ddos_configuration: BucketAntiDDOSConfiguration,
}

impl crate::FlatSerialize for UpdateBucketAntiDDosInfobody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
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
pub struct BucketAntiDDOSInfoCnames {
    #[serde(rename = "Domain")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketAntiDDOSInfo {
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Ctime")]
    pub ctime: i64,
    #[serde(rename = "Mtime")]
    pub mtime: i64,
    #[serde(rename = "ActiveTime")]
    pub active_time: i64,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Cnames")]
    pub cnames: BucketAntiDDOSInfoCnames,
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
pub struct BucketAntiDDosInfoResponseAntiDDOSListConfiguration {
    #[serde(rename = "Marker")]
    pub marker: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "AntiDDOSConfiguration")]
    pub anti_ddos_configuration: Vec<BucketAntiDDOSInfo>,
}

impl crate::FlatSerialize for BucketAntiDDosInfoResponseAntiDDOSListConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.marker, &format!("{}.Marker", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.anti_ddos_configuration,
            &format!("{}.AntiDDOSConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InitBucketAntiDDosInfobody {
    #[serde(rename = "AntiDDOSConfiguration")]
    pub anti_ddos_configuration: BucketAntiDDOSConfiguration,
}

impl crate::FlatSerialize for InitBucketAntiDDosInfobody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
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
pub struct UserAntiDDOSInfo {
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "Ctime")]
    pub ctime: i64,
    #[serde(rename = "Mtime")]
    pub mtime: i64,
    #[serde(rename = "ActiveTime")]
    pub active_time: i64,
    #[serde(rename = "Status")]
    pub status: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserAntiDDosInfoResponseAntiDDOSListConfiguration {
    #[serde(rename = "AntiDDOSConfiguration")]
    pub anti_ddos_configuration: Vec<UserAntiDDOSInfo>,
}

impl crate::FlatSerialize for UserAntiDDosInfoResponseAntiDDOSListConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
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
pub struct GroupConfiguration {
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: String,
}

impl crate::FlatSerialize for GroupConfiguration {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketResourceGroupConfiguration {
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResourceGroupbody {
    #[serde(rename = "BucketResourceGroupConfiguration")]
    pub bucket_resource_group_configuration: BucketResourceGroupConfiguration,
}

impl crate::FlatSerialize for ResourceGroupbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bucket_resource_group_configuration,
            &format!("{}.BucketResourceGroupConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CertificateConfiguration {
    #[serde(rename = "CertId")]
    pub cert_id: String,
    #[serde(rename = "Certificate")]
    pub certificate: String,
    #[serde(rename = "PrivateKey")]
    pub private_key: String,
    #[serde(rename = "PreviousCertId")]
    pub previous_cert_id: String,
    #[serde(rename = "Force")]
    pub force: bool,
    #[serde(rename = "DeleteCertificate")]
    pub delete_certificate: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketCnameConfigurationCname {
    #[serde(rename = "Domain")]
    pub domain: String,
    #[serde(rename = "CertificateConfiguration")]
    pub certificate_configuration: CertificateConfiguration,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketCnameConfiguration {
    #[serde(rename = "Cname")]
    pub cname: BucketCnameConfigurationCname,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutCnamebody {
    #[serde(rename = "BucketCnameConfiguration")]
    pub bucket_cname_configuration: BucketCnameConfiguration,
}

impl crate::FlatSerialize for PutCnamebody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bucket_cname_configuration,
            &format!("{}.BucketCnameConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameCertificate {
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "CertId")]
    pub cert_id: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    #[serde(rename = "Fingerprint")]
    pub fingerprint: String,
    #[serde(rename = "ValidStartDate")]
    pub valid_start_date: String,
    #[serde(rename = "ValidEndDate")]
    pub valid_end_date: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameInfo {
    #[serde(rename = "Domain")]
    pub domain: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Certificate")]
    pub certificate: CnameCertificate,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "Cname")]
    pub cname: Vec<CnameInfo>,
}

impl crate::FlatSerialize for CnameResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.owner, &format!("{}.Owner", name), params);
        crate::FlatSerialize::flat_serialize(&self.cname, &format!("{}.Cname", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnamebodyBucketCnameConfigurationCname {
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl crate::FlatSerialize for CnamebodyBucketCnameConfigurationCname {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.domain, &format!("{}.Domain", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnamebodyBucketCnameConfiguration {
    #[serde(rename = "Cname")]
    pub cname: CnamebodyBucketCnameConfigurationCname,
}

impl crate::FlatSerialize for CnamebodyBucketCnameConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.cname, &format!("{}.Cname", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeleteCnamebody {
    #[serde(rename = "BucketCnameConfiguration")]
    pub bucket_cname_configuration: CnamebodyBucketCnameConfiguration,
}

impl crate::FlatSerialize for DeleteCnamebody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bucket_cname_configuration,
            &format!("{}.BucketCnameConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameToken {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Cname")]
    pub cname: String,
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "ExpireTime")]
    pub expire_time: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TokenbodyBucketCnameConfigurationCname {
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl crate::FlatSerialize for TokenbodyBucketCnameConfigurationCname {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.domain, &format!("{}.Domain", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TokenbodyBucketCnameConfiguration {
    #[serde(rename = "Cname")]
    pub cname: TokenbodyBucketCnameConfigurationCname,
}

impl crate::FlatSerialize for TokenbodyBucketCnameConfiguration {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.cname, &format!("{}.Cname", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CnameTokenbody {
    #[serde(rename = "BucketCnameConfiguration")]
    pub bucket_cname_configuration: TokenbodyBucketCnameConfiguration,
}

impl crate::FlatSerialize for CnameTokenbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bucket_cname_configuration,
            &format!("{}.BucketCnameConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Style {
    #[serde(rename = "Content")]
    pub content: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutStylebody {
    #[serde(rename = "Style")]
    pub style: Style,
}

impl crate::FlatSerialize for PutStylebody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.style, &format!("{}.Style", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StyleInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Content")]
    pub content: String,
    #[serde(rename = "CreateTime")]
    pub create_time: String,
    #[serde(rename = "LastModifyTime")]
    pub last_modify_time: String,
    #[serde(rename = "Category")]
    pub category: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StyleList {
    #[serde(rename = "Style")]
    pub style: Vec<StyleInfo>,
}

impl crate::FlatSerialize for StyleList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.style, &format!("{}.Style", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpsConfigurationTls {
    #[serde(rename = "Enable")]
    pub enable: bool,
    #[serde(rename = "TLSVersion")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpsConfigurationCipherSuite {
    #[serde(rename = "Enable")]
    pub enable: bool,
    #[serde(rename = "StrongCipherSuite")]
    pub strong_cipher_suite: bool,
    #[serde(rename = "CustomCipherSuite")]
    pub custom_cipher_suite: Vec<String>,
    #[serde(rename = "TLS13CustomCipherSuite")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpsConfiguration {
    #[serde(rename = "TLS")]
    pub tls: HttpsConfigurationTls,
    #[serde(rename = "CipherSuite")]
    pub cipher_suite: HttpsConfigurationCipherSuite,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpsConfigbody {
    #[serde(rename = "HttpsConfiguration")]
    pub https_configuration: HttpsConfiguration,
}

impl crate::FlatSerialize for HttpsConfigbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.https_configuration,
            &format!("{}.HttpsConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationAllowedFeatures {
    #[serde(rename = "AllowedFeature")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemActions
{
    #[serde(rename = "Action")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationFunctionCompute
{
    #[serde(rename = "FunctionAssumeRoleArn")]
    pub function_assume_role_arn: String,
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeaturesCustomForwardHeaders
{
    #[serde(rename = "CustomForwardHeader")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeatures {
    #[serde(rename = "CustomForwardHeaders")]
    pub custom_forward_headers: ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeaturesCustomForwardHeaders,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformation {
    #[serde(rename = "FunctionCompute")]
    pub function_compute: ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationFunctionCompute,
    #[serde(rename = "AdditionalFeatures")]
    pub additional_features: ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformationAdditionalFeatures,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItem {
    #[serde(rename = "Actions")]
    pub actions: ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemActions,
    #[serde(rename = "ContentTransformation")]
    pub content_transformation: ObjectProcessConfigurationTransformationConfigurationsTransformationConfigurationItemContentTransformation,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfigurationTransformationConfigurations {
    #[serde(rename = "TransformationConfiguration")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectProcessConfiguration {
    #[serde(rename = "AllowedFeatures")]
    pub allowed_features: ObjectProcessConfigurationAllowedFeatures,
    #[serde(rename = "TransformationConfigurations")]
    pub transformation_configurations: ObjectProcessConfigurationTransformationConfigurations,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessPointForObjectProcessConfiguration {
    #[serde(rename = "AccessPointName")]
    pub access_point_name: String,
    #[serde(rename = "ObjectProcessConfiguration")]
    pub object_process_configuration: ObjectProcessConfiguration,
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: String,
}

impl crate::FlatSerialize for AccessPointForObjectProcessConfiguration {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PointForObjectProcessbody {
    #[serde(rename = "CreateAccessPointForObjectProcessConfiguration")]
    pub create_access_point_for_object_process_configuration:
        AccessPointForObjectProcessConfiguration,
}

impl crate::FlatSerialize for PointForObjectProcessbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.create_access_point_for_object_process_configuration,
            &format!("{}.CreateAccessPointForObjectProcessConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointForObjectProcessResult {
    #[serde(rename = "AccessPointForObjectProcessArn")]
    pub access_point_for_object_process_arn: String,
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    pub access_point_for_object_process_alias: String,
}

impl crate::FlatSerialize for CreateAccessPointForObjectProcessResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.access_point_for_object_process_arn,
            &format!("{}.AccessPointForObjectProcessArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_point_for_object_process_alias,
            &format!("{}.AccessPointForObjectProcessAlias", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResultEndpoints {
    #[serde(rename = "PublicEndpoint")]
    pub public_endpoint: String,
    #[serde(rename = "InternalEndpoint")]
    pub internal_endpoint: String,
}

impl crate::FlatSerialize for ResultEndpoints {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointForObjectProcessResult {
    #[serde(rename = "AccessPointNameForObjectProcess")]
    pub access_point_name_for_object_process: String,
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    pub access_point_for_object_process_alias: String,
    #[serde(rename = "AccessPointName")]
    pub access_point_name: String,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "AccessPointForObjectProcessArn")]
    pub access_point_for_object_process_arn: String,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Endpoints")]
    pub endpoints: ResultEndpoints,
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: String,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::FlatSerialize for GetAccessPointForObjectProcessResult {
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
        crate::FlatSerialize::flat_serialize(
            &self.account_id,
            &format!("{}.AccountId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.access_point_for_object_process_arn,
            &format!("{}.AccessPointForObjectProcessArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.creation_date,
            &format!("{}.CreationDate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.endpoints,
            &format!("{}.Endpoints", name),
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PointForObjectProcess {
    #[serde(rename = "AccessPointNameForObjectProcess")]
    pub access_point_name_for_object_process: String,
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    pub access_point_for_object_process_alias: String,
    #[serde(rename = "AccessPointName")]
    pub access_point_name: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: String,
}

impl crate::FlatSerialize for PointForObjectProcess {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PointsForObjectProcess {
    #[serde(rename = "AccessPointForObjectProcess")]
    pub access_point_for_object_process: Vec<PointForObjectProcess>,
}

impl crate::FlatSerialize for PointsForObjectProcess {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PointsForObjectProcessResult {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: String,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "AccessPointsForObjectProcess")]
    pub access_points_for_object_process: PointsForObjectProcess,
}

impl crate::FlatSerialize for PointsForObjectProcessResult {
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
            &self.access_points_for_object_process,
            &format!("{}.AccessPointsForObjectProcess", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigForObjectProcessResult {
    #[serde(rename = "ObjectProcessConfiguration")]
    pub object_process_configuration: ObjectProcessConfiguration,
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: String,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::FlatSerialize for ConfigForObjectProcessResult {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PointConfigForObjectProcessConfiguration {
    #[serde(rename = "ObjectProcessConfiguration")]
    pub object_process_configuration: ObjectProcessConfiguration,
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    pub allow_anonymous_access_for_object_process: String,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::FlatSerialize for PointConfigForObjectProcessConfiguration {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigForObjectProcessbody {
    #[serde(rename = "PutAccessPointConfigForObjectProcessConfiguration")]
    pub put_access_point_config_for_object_process_configuration:
        PointConfigForObjectProcessConfiguration,
}

impl crate::FlatSerialize for ConfigForObjectProcessbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.put_access_point_config_for_object_process_configuration,
            &format!("{}.PutAccessPointConfigForObjectProcessConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutPublicAccessBlockbody {
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::FlatSerialize for PutPublicAccessBlockbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.public_access_block_configuration,
            &format!("{}.PublicAccessBlockConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketPublicAccessBlockbody {
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::FlatSerialize for BucketPublicAccessBlockbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.public_access_block_configuration,
            &format!("{}.PublicAccessBlockConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PointPublicAccessBlockbody {
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::FlatSerialize for PointPublicAccessBlockbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.public_access_block_configuration,
            &format!("{}.PublicAccessBlockConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ArchiveDirectReadConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DirectReadbody {
    #[serde(rename = "ArchiveDirectReadConfiguration")]
    pub archive_direct_read_configuration: ArchiveDirectReadConfiguration,
}

impl crate::FlatSerialize for DirectReadbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.archive_direct_read_configuration,
            &format!("{}.ArchiveDirectReadConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverwriteConfigurationRuleItemPrincipals {
    #[serde(rename = "Principal")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverwriteConfigurationRuleItem {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Suffix")]
    pub suffix: String,
    #[serde(rename = "Principals")]
    pub principals: OverwriteConfigurationRuleItemPrincipals,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverwriteConfiguration {
    #[serde(rename = "Rule")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverwriteConfigbody {
    #[serde(rename = "OverwriteConfiguration")]
    pub overwrite_configuration: OverwriteConfiguration,
}

impl crate::FlatSerialize for OverwriteConfigbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.overwrite_configuration,
            &format!("{}.OverwriteConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectResult {
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
}

impl crate::FlatSerialize for ObjectResult {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RestoreRequestJobParameters {
    #[serde(rename = "Tier")]
    pub tier: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RestoreRequest {
    #[serde(rename = "Days")]
    pub days: i64,
    #[serde(rename = "JobParameters")]
    pub job_parameters: RestoreRequestJobParameters,
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
pub struct RestoreObjectbody {
    #[serde(rename = "RestoreRequest")]
    pub restore_request: RestoreRequest,
}

impl crate::FlatSerialize for RestoreObjectbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.restore_request,
            &format!("{}.RestoreRequest", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CSVInput {
    #[serde(rename = "FileHeaderInfo")]
    pub file_header_info: FileHeaderInfo,
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: String,
    #[serde(rename = "FieldDelimiter")]
    pub field_delimiter: String,
    #[serde(rename = "QuoteCharacter")]
    pub quote_character: String,
    #[serde(rename = "CommentCharacter")]
    pub comment_character: String,
    #[serde(rename = "Range")]
    pub range: String,
    #[serde(rename = "AllowQuotedRecordDelimiter")]
    pub allow_quoted_record_delimiter: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JSONInput {
    #[serde(rename = "Type")]
    pub r#type: JSONType,
    #[serde(rename = "Range")]
    pub range: String,
    #[serde(rename = "ParseJsonNumberAsString")]
    pub parse_json_number_as_string: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InputSerialization {
    #[serde(rename = "CompressionType")]
    pub compression_type: CompressionType,
    #[serde(rename = "CSV")]
    pub csv: CSVInput,
    #[serde(rename = "JSON")]
    pub json: JSONInput,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CSVOutput {
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: String,
    #[serde(rename = "FieldDelimiter")]
    pub field_delimiter: String,
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
pub struct JSONOutput {
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OutputSerialization {
    #[serde(rename = "CSV")]
    pub csv: CSVOutput,
    #[serde(rename = "JSON")]
    pub json: JSONOutput,
    #[serde(rename = "KeepAllColumns")]
    pub keep_all_columns: bool,
    #[serde(rename = "OutputHeader")]
    pub output_header: bool,
    #[serde(rename = "OutputRawData")]
    pub output_raw_data: bool,
    #[serde(rename = "EnablePayloadCrc")]
    pub enable_payload_crc: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SelectRequestOptions {
    #[serde(rename = "SkipPartialDataRecord")]
    pub skip_partial_data_record: bool,
    #[serde(rename = "MaxSkippedRecordsAllowed")]
    pub max_skipped_records_allowed: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SelectRequest {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "InputSerialization")]
    pub input_serialization: InputSerialization,
    #[serde(rename = "OutputSerialization")]
    pub output_serialization: OutputSerialization,
    #[serde(rename = "Options")]
    pub options: SelectRequestOptions,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SelectObjectbody {
    #[serde(rename = "SelectRequest")]
    pub select_request: SelectRequest,
}

impl crate::FlatSerialize for SelectObjectbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.select_request,
            &format!("{}.SelectRequest", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SelectMetaRequest {
    #[serde(rename = "InputSerialization")]
    pub input_serialization: InputSerialization,
    #[serde(rename = "OverwriteIfExists")]
    pub overwrite_if_exists: bool,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectMetabody {
    #[serde(rename = "CsvMetaRequest")]
    pub csv_meta_request: SelectMetaRequest,
}

impl crate::FlatSerialize for ObjectMetabody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.csv_meta_request,
            &format!("{}.CsvMetaRequest", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InitiateMultipartUploadResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
}

impl crate::FlatSerialize for InitiateMultipartUploadResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.upload_id,
            &format!("{}.UploadId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompleteMultipartUploadPartItem {
    #[serde(rename = "PartNumber")]
    pub part_number: i64,
    #[serde(rename = "ETag")]
    pub e_tag: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompleteMultipartUploadSchema {
    #[serde(rename = "Part")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MultipartUploadbody {
    #[serde(rename = "CompleteMultipartUpload")]
    pub complete_multipart_upload: CompleteMultipartUploadSchema,
}

impl crate::FlatSerialize for MultipartUploadbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.complete_multipart_upload,
            &format!("{}.CompleteMultipartUpload", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompleteMultipartUploadResult {
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "ETag")]
    pub e_tag: String,
}

impl crate::FlatSerialize for CompleteMultipartUploadResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.Location", name), params);
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CopyPartResult {
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub e_tag: String,
}

impl crate::FlatSerialize for CopyPartResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.last_modified,
            &format!("{}.LastModified", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.e_tag, &format!("{}.ETag", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Upload {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    #[serde(rename = "Initiated")]
    pub initiated: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UploadsResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "EncodingType")]
    pub encoding_type: String,
    #[serde(rename = "KeyMarker")]
    pub key_marker: String,
    #[serde(rename = "UploadIdMarker")]
    pub upload_id_marker: String,
    #[serde(rename = "NextKeyMarker")]
    pub next_key_marker: String,
    #[serde(rename = "NextUploadIdMarker")]
    pub next_upload_id_marker: String,
    #[serde(rename = "MaxUploads")]
    pub max_uploads: i64,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "Upload")]
    pub upload: Vec<Upload>,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Vec<CommonPrefix>,
}

impl crate::FlatSerialize for UploadsResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.encoding_type,
            &format!("{}.EncodingType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.key_marker,
            &format!("{}.KeyMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.upload_id_marker,
            &format!("{}.UploadIdMarker", name),
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
        crate::FlatSerialize::flat_serialize(
            &self.max_uploads,
            &format!("{}.MaxUploads", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.Prefix", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.delimiter,
            &format!("{}.Delimiter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.upload, &format!("{}.Upload", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.common_prefixes,
            &format!("{}.CommonPrefixes", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Part {
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "PartNumber")]
    pub part_number: i64,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
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
pub struct ListPartResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    #[serde(rename = "PartNumberMarker")]
    pub part_number_marker: i64,
    #[serde(rename = "NextPartNumberMarker")]
    pub next_part_number_marker: i64,
    #[serde(rename = "MaxParts")]
    pub max_parts: i64,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "Part")]
    pub part: Vec<Part>,
}

impl crate::FlatSerialize for ListPartResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.bucket, &format!("{}.Bucket", name), params);
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.upload_id,
            &format!("{}.UploadId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.part_number_marker,
            &format!("{}.PartNumberMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_part_number_marker,
            &format!("{}.NextPartNumberMarker", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_parts,
            &format!("{}.MaxParts", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_truncated,
            &format!("{}.IsTruncated", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.part, &format!("{}.Part", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectAclResponseAccessControlPolicyAccessControlList {
    #[serde(rename = "Grant")]
    pub grant: ObjectACL,
}

impl crate::FlatSerialize for ObjectAclResponseAccessControlPolicyAccessControlList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.grant, &format!("{}.Grant", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectAclResponseAccessControlPolicy {
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "AccessControlList")]
    pub access_control_list: ObjectAclResponseAccessControlPolicyAccessControlList,
}

impl crate::FlatSerialize for ObjectAclResponseAccessControlPolicy {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectTaggingbody {
    #[serde(rename = "Tagging")]
    pub tagging: Tagging,
}

impl crate::FlatSerialize for ObjectTaggingbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tagging, &format!("{}.Tagging", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TaggingResponseTagging {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}

impl crate::FlatSerialize for TaggingResponseTagging {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.tag_set, &format!("{}.TagSet", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelTarget {
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "FragDuration")]
    pub frag_duration: i64,
    #[serde(rename = "FragCount")]
    pub frag_count: i64,
    #[serde(rename = "PlaylistName")]
    pub playlist_name: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelSnapshot {
    #[serde(rename = "RoleName")]
    pub role_name: String,
    #[serde(rename = "DestBucket")]
    pub dest_bucket: String,
    #[serde(rename = "NotifyTopic")]
    pub notify_topic: String,
    #[serde(rename = "Interval")]
    pub interval: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelConfiguration {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Target")]
    pub target: LiveChannelTarget,
    #[serde(rename = "Snapshot")]
    pub snapshot: LiveChannelSnapshot,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelbody {
    #[serde(rename = "LiveChannelConfiguration")]
    pub live_channel_configuration: LiveChannelConfiguration,
}

impl crate::FlatSerialize for LiveChannelbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.live_channel_configuration,
            &format!("{}.LiveChannelConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelPublishUrls {
    #[serde(rename = "Url")]
    pub url: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelPlayUrls {
    #[serde(rename = "Url")]
    pub url: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateLiveChannelResult {
    #[serde(rename = "PublishUrls")]
    pub publish_urls: LiveChannelPublishUrls,
    #[serde(rename = "PlayUrls")]
    pub play_urls: LiveChannelPlayUrls,
}

impl crate::FlatSerialize for CreateLiveChannelResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannel {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "PublishUrls")]
    pub publish_urls: LiveChannelPublishUrls,
    #[serde(rename = "PlayUrls")]
    pub play_urls: LiveChannelPlayUrls,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListLiveChannelResult {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Marker")]
    pub marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i64,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
    #[serde(rename = "LiveChannel")]
    pub live_channel: Vec<LiveChannel>,
}

impl crate::FlatSerialize for ListLiveChannelResult {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
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
        crate::FlatSerialize::flat_serialize(
            &self.live_channel,
            &format!("{}.LiveChannel", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelConfiguration {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Target")]
    pub target: LiveChannelTarget,
}

impl crate::FlatSerialize for ChannelConfiguration {
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
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveRecord {
    #[serde(rename = "StartTime")]
    pub start_time: String,
    #[serde(rename = "EndTime")]
    pub end_time: String,
    #[serde(rename = "RemoteAddr")]
    pub remote_addr: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelHistory {
    #[serde(rename = "LiveRecord")]
    pub live_record: Vec<LiveRecord>,
}

impl crate::FlatSerialize for ChannelHistory {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
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
pub struct LiveChannelVideo {
    #[serde(rename = "Width")]
    pub width: i64,
    #[serde(rename = "Height")]
    pub height: i64,
    #[serde(rename = "FrameRate")]
    pub frame_rate: i64,
    #[serde(rename = "Bandwidth")]
    pub bandwidth: i64,
    #[serde(rename = "Codec")]
    pub codec: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveChannelAudio {
    #[serde(rename = "Bandwidth")]
    pub bandwidth: i64,
    #[serde(rename = "SampleRate")]
    pub sample_rate: i64,
    #[serde(rename = "Codec")]
    pub codec: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelStat {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "ConnectedTime")]
    pub connected_time: String,
    #[serde(rename = "RemoteAddr")]
    pub remote_addr: String,
    #[serde(rename = "Video")]
    pub video: LiveChannelVideo,
    #[serde(rename = "Audio")]
    pub audio: LiveChannelAudio,
}

impl crate::FlatSerialize for ChannelStat {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.Status", name), params);
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
        crate::FlatSerialize::flat_serialize(&self.video, &format!("{}.Video", name), params);
        crate::FlatSerialize::flat_serialize(&self.audio, &format!("{}.Audio", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Channel {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "OrigPicForbidden")]
    pub orig_pic_forbidden: bool,
    #[serde(rename = "UseStyleOnly")]
    pub use_style_only: bool,
    #[serde(rename = "AutoSetContentType")]
    pub auto_set_content_type: bool,
    #[serde(rename = "UseSrcFormat")]
    pub use_src_format: bool,
    #[serde(rename = "SetAttachName")]
    pub set_attach_name: bool,
    #[serde(rename = "Default404Pic")]
    pub default404_pic: String,
    #[serde(rename = "StyleDelimiters")]
    pub style_delimiters: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutChannelbody {
    #[serde(rename = "Channel")]
    pub channel: Channel,
}

impl crate::FlatSerialize for PutChannelbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.channel, &format!("{}.Channel", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ObjectHashConfiguration {
    #[serde(rename = "ObjectHashFunction")]
    pub object_hash_function: String,
    #[serde(rename = "DisplayObjectHash")]
    pub display_object_hash: bool,
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
pub struct BucketHashbody {
    #[serde(rename = "ObjectHashConfiguration")]
    pub object_hash_configuration: ObjectHashConfiguration,
}

impl crate::FlatSerialize for BucketHashbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.object_hash_configuration,
            &format!("{}.ObjectHashConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommonHeadersHeaderItem {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommonHeaders {
    #[serde(rename = "Header")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CommonHeaderbody {
    #[serde(rename = "CommonHeaders")]
    pub common_headers: CommonHeaders,
}

impl crate::FlatSerialize for CommonHeaderbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.common_headers,
            &format!("{}.CommonHeaders", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketChannelConfigRuleListRuleItem {
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    #[serde(rename = "RuleRegex")]
    pub rule_regex: String,
    #[serde(rename = "FrontContent")]
    pub front_content: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketChannelConfigRuleList {
    #[serde(rename = "Rule")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketChannelConfig {
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "DebugInfo")]
    pub debug_info: String,
    #[serde(rename = "RuleList")]
    pub rule_list: BucketChannelConfigRuleList,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BucketProcessConfiguration {
    #[serde(rename = "CompliedHost")]
    pub complied_host: String,
    #[serde(rename = "OssDomainSupportAtProcess")]
    pub oss_domain_support_at_process: String,
    #[serde(rename = "SourceFileProtect")]
    pub source_file_protect: String,
    #[serde(rename = "SourceFileProtectSuffix")]
    pub source_file_protect_suffix: String,
    #[serde(rename = "BucketChannelConfig")]
    pub bucket_channel_config: BucketChannelConfig,
    #[serde(rename = "StyleDelimiters")]
    pub style_delimiters: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProcessConfigurationbody {
    #[serde(rename = "BucketProcessConfiguration")]
    pub bucket_process_configuration: BucketProcessConfiguration,
}

impl crate::FlatSerialize for ProcessConfigurationbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bucket_process_configuration,
            &format!("{}.BucketProcessConfiguration", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionComputeConfigurationFilterKey {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Suffix")]
    pub suffix: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionComputeConfigurationFilter {
    #[serde(rename = "Key")]
    pub key: FunctionComputeConfigurationFilterKey,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionComputeConfigurationFunction {
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "AssumeRole")]
    pub assume_role: String,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionComputeConfiguration {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Event")]
    pub event: Vec<String>,
    #[serde(rename = "Filter")]
    pub filter: FunctionComputeConfigurationFilter,
    #[serde(rename = "Function")]
    pub function: FunctionComputeConfigurationFunction,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventNotificationConfiguration {
    #[serde(rename = "FunctionComputeConfiguration")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateDataLakeCachePrefetchJob {
    #[serde(rename = "Includes")]
    pub includes: Vec<String>,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "Excludes")]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PrefetchJobbody {
    #[serde(rename = "CreateDataLakeCachePrefetchJob")]
    pub create_data_lake_cache_prefetch_job: CreateDataLakeCachePrefetchJob,
}

impl crate::FlatSerialize for PrefetchJobbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.create_data_lake_cache_prefetch_job,
            &format!("{}.CreateDataLakeCachePrefetchJob", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JobID {
    #[serde(rename = "ID")]
    pub id: String,
}

impl crate::FlatSerialize for JobID {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.id, &format!("{}.ID", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobRulePrefixFilterIncludes {
    #[serde(rename = "Include")]
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
    pub includes: DataLakeStorageTransferJobRulePrefixFilterIncludes,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJobRule {
    #[serde(rename = "PrefixFilter")]
    pub prefix_filter: DataLakeStorageTransferJobRulePrefixFilter,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "ExecutorRoleId")]
    pub executor_role_id: String,
    #[serde(rename = "LogBaseDir")]
    pub log_base_dir: String,
    #[serde(rename = "NeedVerify")]
    pub need_verify: bool,
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
    pub percent: i64,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataLakeStorageTransferJob {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "HistoryId")]
    pub history_id: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Type")]
    pub r#type: i32,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename = "LastModifyTime")]
    pub last_modify_time: i64,
    #[serde(rename = "Rule")]
    pub rule: DataLakeStorageTransferJobRule,
    #[serde(rename = "ProgressInfo")]
    pub progress_info: DataLakeStorageTransferJobProgressInfo,
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
pub struct DataLakeStorageTransferJobs {
    #[serde(rename = "Truncated")]
    pub truncated: String,
    #[serde(rename = "NextMarkerBucket")]
    pub next_marker_bucket: String,
    #[serde(rename = "NextMarkerJobId")]
    pub next_marker_job_id: String,
    #[serde(rename = "DataLakeStorageTransferJob")]
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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListAllMyBucketsResult")]
    pub list_all_my_buckets_result: BucketsResult,
}

impl crate::ToCodeMessage for ListBucketsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "RegionInfoList")]
    pub region_info_list: InfoList,
}

impl crate::ToCodeMessage for DescribeRegionsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketStatResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketStat")]
    pub bucket_stat: BucketStat,
}

impl crate::ToCodeMessage for GetBucketStatResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListBucketResult")]
    pub list_bucket_result: ObjectsResponseListBucketResult,
}

impl crate::ToCodeMessage for ListObjectsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectsV2Response {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListBucketResult")]
    pub list_bucket_result: V2ResponseListBucketResult,
}

impl crate::ToCodeMessage for ListObjectsV2Response {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketInfo")]
    pub bucket_info: BucketInfo,
}

impl crate::ToCodeMessage for GetBucketInfoResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
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

impl crate::ToCodeMessage for GetBucketLocationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListAccessPointsResult")]
    pub list_access_points_result: ListAccessPointsResult,
}

impl crate::ToCodeMessage for ListAccessPointsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "GetAccessPointResult")]
    pub get_access_point_result: GetAccessPointResult,
}

impl crate::ToCodeMessage for GetAccessPointResponse {
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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CreateAccessPointResult")]
    pub create_access_point_result: CreateAccessPointResult,
}

impl crate::ToCodeMessage for CreateAccessPointResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketWormResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "WormConfiguration")]
    pub worm_configuration: WormConfiguration,
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
    #[serde(rename = "AccessControlPolicy")]
    pub access_control_policy: BucketAclResponseAccessControlPolicy,
}

impl crate::ToCodeMessage for GetBucketAclResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketLifecycleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "LifecycleConfiguration")]
    pub lifecycle_configuration: LifecycleConfiguration,
}

impl crate::ToCodeMessage for GetBucketLifecycleResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketTransferAccelerationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "TransferAccelerationConfiguration")]
    pub transfer_acceleration_configuration: AccelerationConfiguration,
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
    #[serde(rename = "VersioningConfiguration")]
    pub versioning_configuration: ResponseVersioningConfiguration,
}

impl crate::ToCodeMessage for GetBucketVersioningResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListObjectVersionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListVersionsResult")]
    pub list_versions_result: VersionsResult,
}

impl crate::ToCodeMessage for ListObjectVersionsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketPolicyStatusResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "PolicyStatus")]
    pub policy_status: PolicyStatus,
}

impl crate::ToCodeMessage for GetBucketPolicyStatusResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: ResponseReplicationConfiguration,
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
    #[serde(rename = "ReplicationLocation")]
    pub replication_location: ReplicationLocation,
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
    #[serde(rename = "ReplicationProgress")]
    pub replication_progress: ReplicationProgress,
}

impl crate::ToCodeMessage for GetBucketReplicationProgressResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketInventoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "InventoryConfiguration")]
    pub inventory_configuration: InventoryConfiguration,
}

impl crate::ToCodeMessage for GetBucketInventoryResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketInventoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListInventoryConfigurationsResult")]
    pub list_inventory_configurations_result: ConfigurationsResult,
}

impl crate::ToCodeMessage for ListBucketInventoryResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketLoggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketLoggingStatus")]
    pub bucket_logging_status: BucketLoggingStatus,
}

impl crate::ToCodeMessage for GetBucketLoggingResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetUserDefinedLogFieldsConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "UserDefinedLogFieldsConfiguration")]
    pub user_defined_log_fields_configuration: UserDefinedLogFieldsConfiguration,
}

impl crate::ToCodeMessage for GetUserDefinedLogFieldsConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketWebsiteResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "WebsiteConfiguration")]
    pub website_configuration: WebsiteConfiguration,
}

impl crate::ToCodeMessage for GetBucketWebsiteResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketRefererResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "RefererConfiguration")]
    pub referer_configuration: RefererConfiguration,
}

impl crate::ToCodeMessage for GetBucketRefererResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketTagsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Tagging")]
    pub tagging: TagsResponseTagging,
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
    #[serde(rename = "ListBucketDataRedundancyTransition")]
    pub list_bucket_data_redundancy_transition:
        UserDataRedundancyTransitionResponseListBucketDataRedundancyTransition,
}

impl crate::ToCodeMessage for ListUserDataRedundancyTransitionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
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
        BucketDataRedundancyTransitionResponseListBucketDataRedundancyTransition,
}

impl crate::ToCodeMessage for ListBucketDataRedundancyTransitionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketDataRedundancyTransition")]
    pub bucket_data_redundancy_transition: BucketDataRedundancyTransition,
}

impl crate::ToCodeMessage for GetBucketDataRedundancyTransitionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateBucketDataRedundancyTransitionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "BucketDataRedundancyTransition")]
    pub bucket_data_redundancy_transition: ResponseBucketDataRedundancyTransition,
}

impl crate::ToCodeMessage for CreateBucketDataRedundancyTransitionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketEncryptionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ServerSideEncryptionRule")]
    pub server_side_encryption_rule: EncryptionRule,
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
    #[serde(rename = "RequestPaymentConfiguration")]
    pub request_payment_configuration: PaymentConfiguration,
}

impl crate::ToCodeMessage for GetBucketRequestPaymentResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketCorsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CORSConfiguration")]
    pub cors_configuration: SConfiguration,
}

impl crate::ToCodeMessage for GetBucketCorsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
pub struct GetBucketAccessMonitorResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "AccessMonitorConfiguration")]
    pub access_monitor_configuration: AccessMonitorConfiguration,
}

impl crate::ToCodeMessage for GetBucketAccessMonitorResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetMetaQueryStatusResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "MetaQueryStatus")]
    pub meta_query_status: QueryStatus,
}

impl crate::ToCodeMessage for GetMetaQueryStatusResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DoMetaQueryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "MetaQuery")]
    pub meta_query: MetaQueryResp,
}

impl crate::ToCodeMessage for DoMetaQueryResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListBucketAntiDDosInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "AntiDDOSListConfiguration")]
    pub anti_ddos_list_configuration: BucketAntiDDosInfoResponseAntiDDOSListConfiguration,
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
    #[serde(rename = "AntiDDOSListConfiguration")]
    pub anti_ddos_list_configuration: UserAntiDDosInfoResponseAntiDDOSListConfiguration,
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
    #[serde(rename = "BucketResourceGroupConfiguration")]
    pub bucket_resource_group_configuration: GroupConfiguration,
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
    #[serde(rename = "ListCnameResult")]
    pub list_cname_result: CnameResult,
}

impl crate::ToCodeMessage for ListCnameResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetCnameTokenResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CnameToken")]
    pub cname_token: CnameToken,
}

impl crate::ToCodeMessage for GetCnameTokenResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateCnameTokenResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CnameToken")]
    pub cname_token: CnameToken,
}

impl crate::ToCodeMessage for CreateCnameTokenResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListStyleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "StyleList")]
    pub style_list: StyleList,
}

impl crate::ToCodeMessage for ListStyleResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetStyleResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Style")]
    pub style: StyleInfo,
}

impl crate::ToCodeMessage for GetStyleResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketHttpsConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "HttpsConfiguration")]
    pub https_configuration: HttpsConfiguration,
}

impl crate::ToCodeMessage for GetBucketHttpsConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateAccessPointForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CreateAccessPointForObjectProcessResult")]
    pub create_access_point_for_object_process_result: CreateAccessPointForObjectProcessResult,
}

impl crate::ToCodeMessage for CreateAccessPointForObjectProcessResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "GetAccessPointForObjectProcessResult")]
    pub get_access_point_for_object_process_result: GetAccessPointForObjectProcessResult,
}

impl crate::ToCodeMessage for GetAccessPointForObjectProcessResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAccessPointsForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListAccessPointsForObjectProcessResult")]
    pub list_access_points_for_object_process_result: PointsForObjectProcessResult,
}

impl crate::ToCodeMessage for ListAccessPointsForObjectProcessResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointConfigForObjectProcessResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "GetAccessPointConfigForObjectProcessResult")]
    pub get_access_point_config_for_object_process_result: ConfigForObjectProcessResult,
}

impl crate::ToCodeMessage for GetAccessPointConfigForObjectProcessResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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
pub struct GetPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::ToCodeMessage for GetPublicAccessBlockResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::ToCodeMessage for GetBucketPublicAccessBlockResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAccessPointPublicAccessBlockResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

impl crate::ToCodeMessage for GetAccessPointPublicAccessBlockResponse {
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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketArchiveDirectReadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ArchiveDirectReadConfiguration")]
    pub archive_direct_read_configuration: ArchiveDirectReadConfiguration,
}

impl crate::ToCodeMessage for GetBucketArchiveDirectReadResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketOverwriteConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "OverwriteConfiguration")]
    pub overwrite_configuration: OverwriteConfiguration,
}

impl crate::ToCodeMessage for GetBucketOverwriteConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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

impl crate::IntoResponse for PutObjectResponse {
    type Response = Self;
    fn into_response(self) -> Self::Response {
        self
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CopyObjectResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CopyObjectResult")]
    pub copy_object_result: ObjectResult,
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

impl crate::IntoResponse for DeleteObjectResponse {
    type Response = Self;
    fn into_response(self) -> Self::Response {
        self
    }
}

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
    /// Header field from response: x-oss-transition-time
    #[serde(skip)]
    pub x_oss_transition_time: Option<String>,
    /// Header field from response: x‑oss‑tagging‑count
    #[serde(skip)]
    pub x_oss_tagging_count: Option<i64>,
}

impl crate::ToCodeMessage for HeadObjectResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateSelectObjectMetaResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for CreateSelectObjectMetaResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct InitiateMultipartUploadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "InitiateMultipartUploadResult")]
    pub initiate_multipart_upload_result: InitiateMultipartUploadResult,
}

impl crate::ToCodeMessage for InitiateMultipartUploadResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CompleteMultipartUploadResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CompleteMultipartUploadResult")]
    pub complete_multipart_upload_result: CompleteMultipartUploadResult,
    /// Header field from response: x-oss-version-id
    #[serde(skip)]
    pub x_oss_version_id: Option<String>,
}

impl crate::ToCodeMessage for CompleteMultipartUploadResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UploadPartCopyResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CopyPartResult")]
    pub copy_part_result: CopyPartResult,
    /// Header field from response: x-oss-copy-source-version-id
    #[serde(skip)]
    pub x_oss_copy_source_version_id: Option<String>,
}

impl crate::ToCodeMessage for UploadPartCopyResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListMultipartUploadsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListMultipartUploadsResult")]
    pub list_multipart_uploads_result: UploadsResult,
}

impl crate::ToCodeMessage for ListMultipartUploadsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListPartsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "ListPartResult")]
    pub list_part_result: ListPartResult,
}

impl crate::ToCodeMessage for ListPartsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectAclResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "AccessControlPolicy")]
    pub access_control_policy: ObjectAclResponseAccessControlPolicy,
}

impl crate::ToCodeMessage for GetObjectAclResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetObjectTaggingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Tagging")]
    pub tagging: TaggingResponseTagging,
}

impl crate::ToCodeMessage for GetObjectTaggingResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutLiveChannelResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "CreateLiveChannelResult")]
    pub create_live_channel_result: CreateLiveChannelResult,
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
    #[serde(rename = "ListLiveChannelResult")]
    pub list_live_channel_result: ListLiveChannelResult,
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
    #[serde(rename = "LiveChannelConfiguration")]
    pub live_channel_configuration: ChannelConfiguration,
}

impl crate::ToCodeMessage for GetLiveChannelInfoResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLiveChannelHistoryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "LiveChannelHistory")]
    pub live_channel_history: ChannelHistory,
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
    #[serde(rename = "LiveChannelStat")]
    pub live_channel_stat: ChannelStat,
}

impl crate::ToCodeMessage for GetLiveChannelStatResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetBucketEventNotificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: EventNotificationConfiguration,
}

impl crate::ToCodeMessage for GetBucketEventNotificationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutDataLakeCachePrefetchJobResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "DataLakeCachePrefetchJobID")]
    pub data_lake_cache_prefetch_job_id: JobID,
}

impl crate::ToCodeMessage for PutDataLakeCachePrefetchJobResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListDataLakeStorageTransferJobResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "DataLakeStorageTransferJobs")]
    pub data_lake_storage_transfer_jobs: DataLakeStorageTransferJobs,
}

impl crate::ToCodeMessage for ListDataLakeStorageTransferJobResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}
