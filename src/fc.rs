#[derive(Clone, Copy, Debug, strum::EnumString)]
pub enum Endpoint {
    CnQingdao,
    CnBeijing,
    CnZhangjiakou,
    CnHuhehaote,
    CnHangzhou,
    CnShanghai,
    CnShenzhen,
    CnChengdu,
    CnHongkong,
    ApNortheast1,
    ApSoutheast1,
    ApSoutheast2,
    ApSoutheast3,
    ApSoutheast5,
    UsEast1,
    UsWest1,
    EuWest1,
    EuCentral1,
    ApSouth1,
    CnHangzhouFinance,
    CnShanghaiFinance1,
    ApSoutheast7,
    MeCentral1,
    CnHeyuanAcdr1,
    CnWulanchabu,
    ApNortheast2,
}

impl Endpoint {
    pub fn name(self) -> &'static str {
        match self {
            Endpoint::CnQingdao => "cn-qingdao",
            Endpoint::CnBeijing => "cn-beijing",
            Endpoint::CnZhangjiakou => "cn-zhangjiakou",
            Endpoint::CnHuhehaote => "cn-huhehaote",
            Endpoint::CnHangzhou => "cn-hangzhou",
            Endpoint::CnShanghai => "cn-shanghai",
            Endpoint::CnShenzhen => "cn-shenzhen",
            Endpoint::CnChengdu => "cn-chengdu",
            Endpoint::CnHongkong => "cn-hongkong",
            Endpoint::ApNortheast1 => "ap-northeast-1",
            Endpoint::ApSoutheast1 => "ap-southeast-1",
            Endpoint::ApSoutheast2 => "ap-southeast-2",
            Endpoint::ApSoutheast3 => "ap-southeast-3",
            Endpoint::ApSoutheast5 => "ap-southeast-5",
            Endpoint::UsEast1 => "us-east-1",
            Endpoint::UsWest1 => "us-west-1",
            Endpoint::EuWest1 => "eu-west-1",
            Endpoint::EuCentral1 => "eu-central-1",
            Endpoint::ApSouth1 => "ap-south-1",
            Endpoint::CnHangzhouFinance => "cn-hangzhou-finance",
            Endpoint::CnShanghaiFinance1 => "cn-shanghai-finance-1",
            Endpoint::ApSoutheast7 => "ap-southeast-7",
            Endpoint::MeCentral1 => "me-central-1",
            Endpoint::CnHeyuanAcdr1 => "cn-heyuan-acdr-1",
            Endpoint::CnWulanchabu => "cn-wulanchabu",
            Endpoint::ApNortheast2 => "ap-northeast-2",
        }
    }
}

impl From<Endpoint> for &'static str {
    fn from(ep: Endpoint) -> Self {
        match ep {
            Endpoint::CnQingdao => "fcv3.cn-qingdao.aliyuncs.com",
            Endpoint::CnBeijing => "fcv3.cn-beijing.aliyuncs.com",
            Endpoint::CnZhangjiakou => "fcv3.cn-zhangjiakou.aliyuncs.com",
            Endpoint::CnHuhehaote => "fcv3.cn-huhehaote.aliyuncs.com",
            Endpoint::CnHangzhou => "fcv3.cn-hangzhou.aliyuncs.com",
            Endpoint::CnShanghai => "fcv3.cn-shanghai.aliyuncs.com",
            Endpoint::CnShenzhen => "fcv3.cn-shenzhen.aliyuncs.com",
            Endpoint::CnChengdu => "fcv3.cn-chengdu.aliyuncs.com",
            Endpoint::CnHongkong => "fcv3.cn-hongkong.aliyuncs.com",
            Endpoint::ApNortheast1 => "fcv3.ap-northeast-1.aliyuncs.com",
            Endpoint::ApSoutheast1 => "fcv3.ap-southeast-1.aliyuncs.com",
            Endpoint::ApSoutheast2 => "fcv3.ap-southeast-2.aliyuncs.com",
            Endpoint::ApSoutheast3 => "fcv3.ap-southeast-3.aliyuncs.com",
            Endpoint::ApSoutheast5 => "fcv3.ap-southeast-5.aliyuncs.com	",
            Endpoint::UsEast1 => "fcv3.us-east-1.aliyuncs.com",
            Endpoint::UsWest1 => "fcv3.us-west-1.aliyuncs.com",
            Endpoint::EuWest1 => "fcv3.eu-west-1.aliyuncs.com",
            Endpoint::EuCentral1 => "fcv3.eu-central-1.aliyuncs.com",
            Endpoint::ApSouth1 => "fcv3.ap-south-1.aliyuncs.com",
            Endpoint::CnHangzhouFinance => "cn-hangzhou-finance.fc.aliyuncs.com",
            Endpoint::CnShanghaiFinance1 => "cn-shanghai-finance-1.fc.aliyuncs.com",
            Endpoint::ApSoutheast7 => "fcv3.ap-southeast-7.aliyuncs.com",
            Endpoint::MeCentral1 => "me-central-1.fc.aliyuncs.com",
            Endpoint::CnHeyuanAcdr1 => "cn-heyuan-acdr-1.fc.aliyuncs.com",
            Endpoint::CnWulanchabu => "fcv3.cn-wulanchabu.aliyuncs.com",
            Endpoint::ApNortheast2 => "fcv3.ap-northeast-2.aliyuncs.com",
        }
    }
}

mod sealed {
    /// prevent Request type used with Connection of other mod.
    pub trait Bound {}
}

#[derive(Clone)]
pub struct Connection(crate::common::Connection<crate::auth::Acs3HmacSha256>);

impl Connection {
    pub fn new(endpoint: Endpoint, app_key_secret: crate::v3::AccessKeySecret) -> Self {
        Self(crate::common::Connection::new(
            crate::auth::Acs3HmacSha256(app_key_secret),
            "2023-03-30",
            endpoint.into(),
        ))
    }

    pub fn with_client(
        endpoint: Endpoint,
        app_key_secret: crate::v3::AccessKeySecret,
        client: reqwest::Client,
    ) -> Self {
        Self(crate::common::Connection::with_client(
            crate::auth::Acs3HmacSha256(app_key_secret),
            "2023-03-30",
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
    /// # 更新资源组
    ///
    /// 更新函数计算资源的资源组。
    ///
    /// 更新函数计算资源所在的资源组，需要为用户授予资源所在资源组以及目标资源组的 ChangeResourceGroup 权限
    ///
    /// # Path
    /// `/2023-03-30/resource-groups`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn change_resource_group(
        &self,
        req: ChangeResourceGroup,
    ) -> impl std::future::Future<Output = crate::Result<ChangeResourceGroupOutput>> + Send {
        self.call(req)
    }

    /// # 查询FC 3.0产品地域信息
    ///
    /// 查询FC 3.0产品支持的地域信息。
    ///
    /// # Path
    /// `/2023-03-30/regions`
    ///
    /// # Error Codes
    /// - `IdempotentParameterMismatch`: The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical.///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - GET
    ///
    pub fn describe_regions(
        &self,
        req: DescribeRegions,
    ) -> impl std::future::Future<Output = crate::Result<DescribeRegionsOutput>> + Send {
        self.call(req)
    }

    /// # 创建自定义域名
    ///
    /// 创建自定义域名。
    ///
    /// 如果您希望在生产环境中通过固定域名访问函数计算中创建的应用或函数，或者解决访问HTTP触发器时强制下载行为，可以通过为应用或函数绑定自定义域名来实现。
    ///
    /// # Path
    /// `/2023-03-30/custom-domains`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn create_custom_domain(
        &self,
        req: CreateCustomDomain,
    ) -> impl std::future::Future<Output = crate::Result<CustomDomain>> + Send {
        self.call(req)
    }

    /// # 删除自定义域名
    ///
    /// 删除自定义域名。
    ///
    /// # Path
    /// `/2023-03-30/custom-domains/{domainName}`
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
    pub fn delete_custom_domain(
        &self,
        req: DeleteCustomDomain,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取自定义域名
    ///
    /// 获取自定义域名配置。
    ///
    /// # Path
    /// `/2023-03-30/custom-domains/{domainName}`
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
    pub fn get_custom_domain(
        &self,
        req: GetCustomDomain,
    ) -> impl std::future::Future<Output = crate::Result<CustomDomain>> + Send {
        self.call(req)
    }

    /// # 列出自定义域名
    ///
    /// 获取自定义域名信息列表。
    ///
    /// # Path
    /// `/2023-03-30/custom-domains`
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
    pub fn list_custom_domains(
        &self,
        req: ListCustomDomains,
    ) -> impl std::future::Future<Output = crate::Result<ListCustomDomainOutput>> + Send {
        self.call(req)
    }

    /// # 更新自定义域名
    ///
    /// 更新自定义域名。
    ///
    /// # Path
    /// `/2023-03-30/custom-domains/{domainName}`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn update_custom_domain(
        &self,
        req: UpdateCustomDomain,
    ) -> impl std::future::Future<Output = crate::Result<CustomDomain>> + Send {
        self.call(req)
    }

    /// # 删除函数版本
    ///
    /// 删除函数版本。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/versions/{versionId}`
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
    pub fn delete_function_version(
        &self,
        req: DeleteFunctionVersion,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取函数代码
    ///
    /// 获取函数代码包的详情。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/code`
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
    pub fn get_function_code(
        &self,
        req: GetFunctionCode,
    ) -> impl std::future::Future<Output = crate::Result<OutputFuncCode>> + Send {
        self.call(req)
    }

    /// # 列出函数版本
    ///
    /// 查询指定函数的版本列表。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/versions`
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
    pub fn list_function_versions(
        &self,
        req: ListFunctionVersions,
    ) -> impl std::future::Future<Output = crate::Result<ListVersionsOutput>> + Send {
        self.call(req)
    }

    /// # 发布函数版本
    ///
    /// 发布函数版本。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/versions`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn publish_function_version(
        &self,
        req: PublishFunctionVersion,
    ) -> impl std::future::Future<Output = crate::Result<Version>> + Send {
        self.call(req)
    }

    /// # 创建函数
    ///
    /// 调用CreateFunction接口创建函数。
    ///
    /// 通过OSS代码包创建函数时，如果提示报错unable to access object xxx in bucket xxx，需要为当前用户授予OSS Bucket的访问权限，例如系统权限策略AliyunOSSReadOnlyAccess或自定义更小粒度的授权oss:GetObject，策略内容详情请参见[授予RAM用户读取某个Bucket下所有资源的权限](~~199058~~)。
    ///
    /// # Path
    /// `/2023-03-30/functions`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn create_function(
        &self,
        req: CreateFunction,
    ) -> impl std::future::Future<Output = crate::Result<Function>> + Send {
        self.call(req)
    }

    /// # 删除函数
    ///
    /// 删除函数。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}`
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
    pub fn delete_function(
        &self,
        req: DeleteFunction,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取函数
    ///
    /// 获取函数信息。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}`
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
    pub fn get_function(
        &self,
        req: GetFunction,
    ) -> impl std::future::Future<Output = crate::Result<Function>> + Send {
        self.call(req)
    }

    /// # 列出函数
    ///
    /// 获取函数列表。
    ///
    /// 执行ListFunctions仅返回函数属性的一部分字段，如果需要获取某个函数的更多属性的字段，包括state、stateReasonCode、stateReason、lastUpdateStatus、lastUpdateStatusReasonCode和lastUpdateStatusReason，请使用[GetFunction](~~2618610~~)。
    ///
    /// # Path
    /// `/2023-03-30/functions`
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
    pub fn list_functions(
        &self,
        req: ListFunctions,
    ) -> impl std::future::Future<Output = crate::Result<ListFunctionsOutput>> + Send {
        self.call(req)
    }

    /// # 调用函数
    ///
    /// 调用执行函数。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/invocations`
    ///
    /// # Request Content-Type
    /// - `application/octet-stream`
    ///
    /// # Response Content-Type
    /// - `application/octet-stream`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn invoke_function(
        &self,
        req: InvokeFunction,
    ) -> impl std::future::Future<Output = crate::Result<Vec<u8>>> + Send {
        self.call(req)
    }

    /// # 更新函数
    ///
    /// 更新函数信息。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn update_function(
        &self,
        req: UpdateFunction,
    ) -> impl std::future::Future<Output = crate::Result<Function>> + Send {
        self.call(req)
    }

    /// # 允许函数调用
    ///
    /// 允许函数被调用，并恢复预留实例的创建。该 OpenAPI 处于内测阶段。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/invoke/enable`
    ///
    /// # Error Codes
    /// - `IdempotentParameterMismatch`: The request uses the same client token as a previous, but non-identical request. Do not reuse a client token with different requests, unless the requests are identical.///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn enable_function_invocation(
        &self,
        req: EnableFunctionInvocation,
    ) -> impl std::future::Future<Output = crate::Result<EnableFunctionInvocationResponse>> + Send
    {
        self.call(req)
    }

    /// # 禁止函数调用
    ///
    /// 禁止函数被调用，可选终止所有正在处理的请求。函数被禁止调用后，将无法创建新实例，同时预留实例会被销毁。该 OpenAPI 处于内测阶段。
    ///
    /// 请谨慎对生产环境的函数调用该接口，避免由于函数被禁止调用导致业务受损。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/invoke/disable`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn disable_function_invocation(
        &self,
        req: DisableFunctionInvocation,
    ) -> impl std::future::Future<Output = crate::Result<DisableFunctionInvocationResponse>> + Send
    {
        self.call(req)
    }

    /// # 删除别名
    ///
    /// 删除别名。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/aliases/{aliasName}`
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
    pub fn delete_alias(
        &self,
        req: DeleteAlias,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取别名
    ///
    /// 获取别名信息。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/aliases/{aliasName}`
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
    pub fn get_alias(
        &self,
        req: GetAlias,
    ) -> impl std::future::Future<Output = crate::Result<Alias>> + Send {
        self.call(req)
    }

    /// # 列出别名
    ///
    /// 查询别名列表信息。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/aliases`
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
    pub fn list_aliases(
        &self,
        req: ListAliases,
    ) -> impl std::future::Future<Output = crate::Result<ListAliasesOutput>> + Send {
        self.call(req)
    }

    /// # 更新别名
    ///
    /// 更新别名。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/aliases/{aliasName}`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn update_alias(
        &self,
        req: UpdateAlias,
    ) -> impl std::future::Future<Output = crate::Result<Alias>> + Send {
        self.call(req)
    }

    /// # 创建别名
    ///
    /// 创建别名。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/aliases`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn create_alias(
        &self,
        req: CreateAlias,
    ) -> impl std::future::Future<Output = crate::Result<Alias>> + Send {
        self.call(req)
    }

    /// # 创建触发器
    ///
    /// 创建触发器。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/triggers`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn create_trigger(
        &self,
        req: CreateTrigger,
    ) -> impl std::future::Future<Output = crate::Result<Trigger>> + Send {
        self.call(req)
    }

    /// # 删除触发器
    ///
    /// 删除指定的触发器。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/triggers/{triggerName}`
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
    pub fn delete_trigger(
        &self,
        req: DeleteTrigger,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取触发器
    ///
    /// 获取指定的触发器详情。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/triggers/{triggerName}`
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
    pub fn get_trigger(
        &self,
        req: GetTrigger,
    ) -> impl std::future::Future<Output = crate::Result<Trigger>> + Send {
        self.call(req)
    }

    /// # 列出触发器
    ///
    /// 查询指定函数的触发器列表。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/triggers`
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
    pub fn list_triggers(
        &self,
        req: ListTriggers,
    ) -> impl std::future::Future<Output = crate::Result<ListTriggersOutput>> + Send {
        self.call(req)
    }

    /// # 更新触发器
    ///
    /// 更新触发器信息。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/triggers/{triggerName}`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn update_trigger(
        &self,
        req: UpdateTrigger,
    ) -> impl std::future::Future<Output = crate::Result<Trigger>> + Send {
        self.call(req)
    }

    /// # 删除异步配置
    ///
    /// 删除异步调用配置。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/async-invoke-config`
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
    pub fn delete_async_invoke_config(
        &self,
        req: DeleteAsyncInvokeConfig,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取异步配置
    ///
    /// 获取指定函数的异步调用配置。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/async-invoke-config`
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
    pub fn get_async_invoke_config(
        &self,
        req: GetAsyncInvokeConfig,
    ) -> impl std::future::Future<Output = crate::Result<AsyncConfig>> + Send {
        self.call(req)
    }

    /// # 列出函数异步配置
    ///
    /// 查询指定函数的所有异步配置信息。
    ///
    /// # Path
    /// `/2023-03-30/async-invoke-configs`
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
    pub fn list_async_invoke_configs(
        &self,
        req: ListAsyncInvokeConfigs,
    ) -> impl std::future::Future<Output = crate::Result<ListAsyncInvokeConfigOutput>> + Send {
        self.call(req)
    }

    /// # 设置函数异步配置
    ///
    /// 创建或更新函数的异步调用配置。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/async-invoke-config`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn put_async_invoke_config(
        &self,
        req: PutAsyncInvokeConfig,
    ) -> impl std::future::Future<Output = crate::Result<AsyncConfig>> + Send {
        self.call(req)
    }

    /// # 删除预留配置
    ///
    /// 删除预留配置。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/provision-config`
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
    pub fn delete_provision_config(
        &self,
        req: DeleteProvisionConfig,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取预留配置
    ///
    /// 获取预留配置。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/provision-config`
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
    pub fn get_provision_config(
        &self,
        req: GetProvisionConfig,
    ) -> impl std::future::Future<Output = crate::Result<ProvisionConfig>> + Send {
        self.call(req)
    }

    /// # 列出函数预留配置
    ///
    /// 查询预留配置列表。
    ///
    /// # Path
    /// `/2023-03-30/provision-configs`
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
    pub fn list_provision_configs(
        &self,
        req: ListProvisionConfigs,
    ) -> impl std::future::Future<Output = crate::Result<ListProvisionConfigsOutput>> + Send {
        self.call(req)
    }

    /// # 设置函数预留实例
    ///
    /// 创建预留配置。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/provision-config`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn put_provision_config(
        &self,
        req: PutProvisionConfig,
    ) -> impl std::future::Future<Output = crate::Result<ProvisionConfig>> + Send {
        self.call(req)
    }

    /// # 删除并发度配置
    ///
    /// 删除并发度配置。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/concurrency`
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
    pub fn delete_concurrency_config(
        &self,
        req: DeleteConcurrencyConfig,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取并发度配置
    ///
    /// 获取并发度配置。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/concurrency`
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
    pub fn get_concurrency_config(
        &self,
        req: GetConcurrencyConfig,
    ) -> impl std::future::Future<Output = crate::Result<ConcurrencyConfig>> + Send {
        self.call(req)
    }

    /// # 列出函数并发度配置
    ///
    /// 查询并发度配置列表。
    ///
    /// # Path
    /// `/2023-03-30/concurrency-configs`
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
    pub fn list_concurrency_configs(
        &self,
        req: ListConcurrencyConfigs,
    ) -> impl std::future::Future<Output = crate::Result<ListConcurrencyConfigsOutput>> + Send {
        self.call(req)
    }

    /// # 设置函数并发度
    ///
    /// 设置函数并发度。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/concurrency`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn put_concurrency_config(
        &self,
        req: PutConcurrencyConfig,
    ) -> impl std::future::Future<Output = crate::Result<ConcurrencyConfig>> + Send {
        self.call(req)
    }

    /// # 创建层版本
    ///
    /// 发布层版本。
    ///
    /// # Path
    /// `/2023-03-30/layers/{layerName}/versions`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn create_layer_version(
        &self,
        req: CreateLayerVersion,
    ) -> impl std::future::Future<Output = crate::Result<Layer>> + Send {
        self.call(req)
    }

    /// # 删除层版本
    ///
    /// 删除层版本。
    ///
    /// # Path
    /// `/2023-03-30/layers/{layerName}/versions/{version}`
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
    pub fn delete_layer_version(
        &self,
        req: DeleteLayerVersion,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取层版本
    ///
    /// 获取层的版本信息。
    ///
    /// # Path
    /// `/2023-03-30/layers/{layerName}/versions/{version}`
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
    pub fn get_layer_version(
        &self,
        req: GetLayerVersion,
    ) -> impl std::future::Future<Output = crate::Result<Layer>> + Send {
        self.call(req)
    }

    /// # 通过ARN获取层版本
    ///
    /// 通过ARN获取层的版本信息。
    ///
    /// # Path
    /// `/2023-03-30/layerarn/{arn}`
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
    pub fn get_layer_version_by_arn(
        &self,
        req: GetLayerVersionByArn,
    ) -> impl std::future::Future<Output = crate::Result<Layer>> + Send {
        self.call(req)
    }

    /// # 列出层版本
    ///
    /// 获取层的版本列表。
    ///
    /// # Path
    /// `/2023-03-30/layers/{layerName}/versions`
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
    pub fn list_layer_versions(
        &self,
        req: ListLayerVersions,
    ) -> impl std::future::Future<Output = crate::Result<ListLayerVersionOutput>> + Send {
        self.call(req)
    }

    /// # 列出层
    ///
    /// 获取层列表。
    ///
    /// # Path
    /// `/2023-03-30/layers`
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
    pub fn list_layers(
        &self,
        req: ListLayers,
    ) -> impl std::future::Future<Output = crate::Result<ListLayersOutput>> + Send {
        self.call(req)
    }

    /// # 设置层访问权限
    ///
    /// 修改层的权限。
    ///
    /// # Path
    /// `/2023-03-30/layers/{layerName}/acl`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn put_layer_acl(
        &self,
        req: PutLayerACL,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 列出函数实例
    ///
    /// 查询函数实例列表。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/instances`
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
    pub fn list_instances(
        &self,
        req: ListInstances,
    ) -> impl std::future::Future<Output = crate::Result<ListInstancesOutput>> + Send {
        self.call(req)
    }

    /// # 列出VPC绑定配置
    ///
    /// 查询已创建的VPC连接。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/vpc-bindings`
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
    pub fn list_vpc_bindings(
        &self,
        req: ListVpcBindings,
    ) -> impl std::future::Future<Output = crate::Result<ListVpcBindingsOutput>> + Send {
        self.call(req)
    }

    /// # 创建VPC绑定
    ///
    /// 创建VPC连接。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/vpc-bindings`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn create_vpc_binding(
        &self,
        req: CreateVpcBinding,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 删除VPC绑定
    ///
    /// 删除指定VPC防火墙策略组的访问控制策略。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/vpc-bindings/{vpcId}`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - DELETE
    ///
    pub fn delete_vpc_binding(
        &self,
        req: DeleteVpcBinding,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 设置资源标签
    ///
    /// 给指定资源打标签。
    ///
    /// # Path
    /// `/2023-03-30/tags-v2`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn tag_resources(
        &self,
        req: TagResources,
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
        self.call(req)
    }

    /// # 查询资源标签关系
    ///
    /// 列出所有被打标签的资源。
    ///
    /// # Path
    /// `/2023-03-30/tags-v2`
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
    pub fn list_tag_resources(
        &self,
        req: ListTagResources,
    ) -> impl std::future::Future<Output = crate::Result<ListTagResourcesOutput>> + Send {
        self.call(req)
    }

    /// # 删除资源标签
    ///
    /// 删除资源的标签。
    ///
    /// # Path
    /// `/2023-03-30/tags-v2`
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
    pub fn untag_resources(
        &self,
        req: UntagResources,
    ) -> impl std::future::Future<Output = crate::Result<crate::OpenObjectResponse>> + Send {
        self.call(req)
    }

    /// # 获取异步任务
    ///
    /// 获取指定异步任务详情。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/async-tasks/{taskId}`
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
    pub fn get_async_task(
        &self,
        req: GetAsyncTask,
    ) -> impl std::future::Future<Output = crate::Result<AsyncTask>> + Send {
        self.call(req)
    }

    /// # 获取异步任务列表
    ///
    /// 获取异步任务详情列表。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/async-tasks`
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
    pub fn list_async_tasks(
        &self,
        req: ListAsyncTasks,
    ) -> impl std::future::Future<Output = crate::Result<ListAsyncTaskOutput>> + Send {
        self.call(req)
    }

    /// # 停止异步任务
    ///
    /// 停止异步任务。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/async-tasks/{taskId}/stop`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn stop_async_task(
        &self,
        req: StopAsyncTask,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 创建会话资源
    ///
    /// 创建一个显式会话资源。系统自动生成唯一 SessionID，预分配函数实例并绑定会话，支持指定 TTL 和 IdleTimeout。适用于 HEADER_FIELD 或 GENERATED_COOKIE 亲和类型，实现会话预热与配置初始化，调用后即可在 InvokeFunction 请求携带，用于路由请求。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/sessions`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - POST
    ///
    pub fn create_session(
        &self,
        req: CreateSession,
    ) -> impl std::future::Future<Output = crate::Result<Session>> + Send {
        self.call(req)
    }

    /// # 获取会话配置信息
    ///
    /// 获取指定会话的详细信息，包括 SessionID、关联函数、亲和类型、生命周期配置、状态及实例信息。用于查询单个会话当前元数据，支持按 functionName 和 qualifier 精确定位，便于外部系统监控与调试，仅支持查询 Active 状态的会话。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/sessions/{sessionId}`
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
    pub fn get_session(
        &self,
        req: GetSession,
    ) -> impl std::future::Future<Output = crate::Result<Session>> + Send {
        self.call(req)
    }

    /// # 更新会话配置
    ///
    /// 更新Active状态下的会话配置，如 SessionTTLInSeconds 和 SessionIdleTimeoutInSeconds 等生命周期配置。更新后生效，LastModifiedTime 自动刷新。可用于延长或缩短会话有效期，实现动态管理，不改变会话绑定的执行环境。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/sessions/{sessionId}`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn update_session(
        &self,
        req: UpdateSession,
    ) -> impl std::future::Future<Output = crate::Result<Session>> + Send {
        self.call(req)
    }

    /// # 查询会话信息列表
    ///
    /// 列举指定函数下Active/Expired 状态的会话列表，支持按 qualifier、状态、会话ID过滤，分页查询。返回会话基础属性，用于批量查看会话分布与状态，助力运维监控和外部系统集成，提升会话可见性与管理能力。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/sessions`
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
    pub fn list_sessions(
        &self,
        req: ListSessions,
    ) -> impl std::future::Future<Output = crate::Result<ListSessionsOutput>> + Send {
        self.call(req)
    }

    /// # 删除会话资源
    ///
    /// 删除指定会话，禁止新请求路由，会话元数据从数据库清除，客户携带相同 SessionID 的后续请求视为新会话。实现资源释放与会话清理。在会话隔离场景下，系统将中止运行的请求，释放会话绑定的实例。非会话隔离场景下，正在运行中的请求将继续运行，优雅终止。
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/sessions/{sessionId}`
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
    pub fn delete_session(
        &self,
        req: DeleteSession,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 设置函数弹性配置
    ///
    /// 设置函数弹性配置
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/scaling-config`
    ///
    /// # Request Content-Type
    /// - `application/json`
    ///
    /// # Response Content-Type
    /// - `application/json`
    ///
    /// # Methods
    /// - PUT
    ///
    pub fn put_scaling_config(
        &self,
        req: PutScalingConfig,
    ) -> impl std::future::Future<Output = crate::Result<ScalingConfigStatus>> + Send {
        self.call(req)
    }

    /// # 删除函数弹性配置
    ///
    /// 删除函数弹性配置
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/scaling-config`
    ///
    /// # Methods
    /// - DELETE
    ///
    pub fn delete_scaling_config(
        &self,
        req: DeleteScalingConfig,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.call(req)
    }

    /// # 获取函数弹性配置
    ///
    /// 获取函数弹性配置
    ///
    /// # Path
    /// `/2023-03-30/functions/{functionName}/scaling-config`
    ///
    /// # Methods
    /// - GET
    ///
    pub fn get_scaling_config(
        &self,
        req: GetScalingConfig,
    ) -> impl std::future::Future<Output = crate::Result<ScalingConfigStatus>> + Send {
        self.call(req)
    }

    /// # 列出函数弹性配置
    ///
    /// 列出函数弹性配置
    ///
    /// # Path
    /// `/2023-03-30/scaling-configs`
    ///
    /// # Methods
    /// - GET
    ///
    pub fn list_scaling_configs(
        &self,
        req: ListScalingConfigs,
    ) -> impl std::future::Future<Output = crate::Result<ListScalingConfigsOutput>> + Send {
        self.call(req)
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ChangeResourceGroup {
    /// 更新资源组请求信息
    #[setters(generate = true, strip_option)]
    body: Option<ChangeResourceGroupInput>,
}

impl sealed::Bound for ChangeResourceGroup {}

impl ChangeResourceGroup {
    pub fn new() -> Self {
        Self { body: None }
    }
}

impl crate::Request for ChangeResourceGroup {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "ChangeResourceGroup";
    const URL_PATH: &'static str = "/2023-03-30/resource-groups";

    type Body = crate::JsonBody<ChangeResourceGroupInput>;

    type ResponseWrap = crate::JsonResponseWrap<ChangeResourceGroupOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DescribeRegions {
    /// 用户指定返回中文、英文等语言的地域列表信息。更多详情，请参见RFC 7231。取值范围：
    ///
    /// - zh-CN：简体中文。
    ///
    /// - en-US：英文。
    ///
    /// - ja：日文
    ///
    /// 默认值：zh-CN。
    #[setters(generate = true, strip_option)]
    accept_language: Option<AcceptLanguage>,
}

impl sealed::Bound for DescribeRegions {}

impl DescribeRegions {
    pub fn new() -> Self {
        Self {
            accept_language: None,
        }
    }
}

impl crate::Request for DescribeRegions {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DescribeRegions";
    const URL_PATH: &'static str = "/2023-03-30/regions";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<DescribeRegionsOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.accept_language {
            params.push(("AcceptLanguage".into(), (f).into()));
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
pub struct CreateCustomDomain {
    /// 自定义域名信息
    body: CreateCustomDomainInput,
}

impl sealed::Bound for CreateCustomDomain {}

impl CreateCustomDomain {
    pub fn new() -> Self {
        Self {
            body: Default::default(),
        }
    }
}

impl crate::Request for CreateCustomDomain {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateCustomDomain";
    const URL_PATH: &'static str = "/2023-03-30/custom-domains";

    type Body = crate::JsonBody<CreateCustomDomainInput>;

    type ResponseWrap = crate::JsonResponseWrap<CustomDomain>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteCustomDomain {
    /// 域名
    domain_name: String,
}

impl sealed::Bound for DeleteCustomDomain {}

impl DeleteCustomDomain {
    pub fn new(domain_name: impl Into<String>) -> Self {
        Self {
            domain_name: domain_name.into(),
        }
    }
}
impl crate::ToFormData for DeleteCustomDomain {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteCustomDomain {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteCustomDomain";
    const URL_PATH: &'static str = "/2023-03-30/custom-domains/{domainName}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{domainName}", self.domain_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetCustomDomain {
    /// 域名
    domain_name: String,
}

impl sealed::Bound for GetCustomDomain {}

impl GetCustomDomain {
    pub fn new(domain_name: impl Into<String>) -> Self {
        Self {
            domain_name: domain_name.into(),
        }
    }
}

impl crate::Request for GetCustomDomain {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetCustomDomain";
    const URL_PATH: &'static str = "/2023-03-30/custom-domains/{domainName}";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<CustomDomain>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{domainName}", self.domain_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListCustomDomains {
    /// 域名前缀
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回的自定义域名数量
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListCustomDomains {}

impl ListCustomDomains {
    pub fn new() -> Self {
        Self {
            prefix: None,
            next_token: None,
            limit: None,
        }
    }
}

impl crate::Request for ListCustomDomains {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListCustomDomains";
    const URL_PATH: &'static str = "/2023-03-30/custom-domains";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListCustomDomainOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
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
pub struct UpdateCustomDomain {
    /// 域名
    domain_name: String,
    /// 自定义域名信息
    body: UpdateCustomDomainInput,
}

impl sealed::Bound for UpdateCustomDomain {}

impl UpdateCustomDomain {
    pub fn new(domain_name: impl Into<String>) -> Self {
        Self {
            domain_name: domain_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for UpdateCustomDomain {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UpdateCustomDomain";
    const URL_PATH: &'static str = "/2023-03-30/custom-domains/{domainName}";

    type Body = crate::JsonBody<UpdateCustomDomainInput>;

    type ResponseWrap = crate::JsonResponseWrap<CustomDomain>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{domainName}", self.domain_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteFunctionVersion {
    /// 函数名称
    function_name: String,
    /// 函数版本
    version_id: i32,
}

impl sealed::Bound for DeleteFunctionVersion {}

impl DeleteFunctionVersion {
    pub fn new(function_name: impl Into<String>, version_id: impl Into<i32>) -> Self {
        Self {
            function_name: function_name.into(),
            version_id: version_id.into(),
        }
    }
}
impl crate::ToFormData for DeleteFunctionVersion {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteFunctionVersion {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteFunctionVersion";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/versions/{versionId}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{versionId}", self.version_id.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetFunctionCode {
    /// 函数名称
    function_name: String,
    /// 函数版本或别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for GetFunctionCode {}

impl GetFunctionCode {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}

impl crate::Request for GetFunctionCode {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetFunctionCode";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/code";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<OutputFuncCode>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListFunctionVersions {
    /// 函数名称
    function_name: String,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 版本排序方式。BACKWARD或者FORWARD。
    #[setters(generate = true, strip_option)]
    direction: Option<String>,
    /// 返回的版本数量
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListFunctionVersions {}

impl ListFunctionVersions {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            next_token: None,
            direction: None,
            limit: None,
        }
    }
}

impl crate::Request for ListFunctionVersions {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListFunctionVersions";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/versions";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListVersionsOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.direction {
            params.push(("direction".into(), (f).into()));
        }

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PublishFunctionVersion {
    /// 函数名称
    function_name: String,
    /// 函数版本信息
    body: PublishVersionInput,
}

impl sealed::Bound for PublishFunctionVersion {}

impl PublishFunctionVersion {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for PublishFunctionVersion {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PublishFunctionVersion";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/versions";

    type Body = crate::JsonBody<PublishVersionInput>;

    type ResponseWrap = crate::JsonResponseWrap<Version>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateFunction {
    /// 函数配置信息
    body: CreateFunctionInput,
}

impl sealed::Bound for CreateFunction {}

impl CreateFunction {
    pub fn new() -> Self {
        Self {
            body: Default::default(),
        }
    }
}

impl crate::Request for CreateFunction {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateFunction";
    const URL_PATH: &'static str = "/2023-03-30/functions";

    type Body = crate::JsonBody<CreateFunctionInput>;

    type ResponseWrap = crate::JsonResponseWrap<Function>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteFunction {
    /// 函数名称
    function_name: String,
}

impl sealed::Bound for DeleteFunction {}

impl DeleteFunction {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
        }
    }
}
impl crate::ToFormData for DeleteFunction {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteFunction {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteFunction";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetFunction {
    /// 函数名称
    function_name: String,
    /// 函数版本或别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for GetFunction {}

impl GetFunction {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}

impl crate::Request for GetFunction {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetFunction";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<Function>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListFunctions {
    /// 函数名称前缀。
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 分页起始标记。
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回函数的数量，最小值为1，最大值为100。
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
    /// 函数所属的版本。
    /// - v3：只列出FC3.0的函数。
    /// - v2：只列出FC2.0的函数。
    ///
    /// 默认不填，即列出FC3.0和FC2.0的函数。
    #[setters(generate = true, strip_option)]
    fc_version: Option<FunctionsfcVersion>,
    /// 需要过滤的函数标签。
    #[setters(generate = true, strip_option)]
    tags: Option<Vec<Tag>>,
    /// 需要过滤的函数运行时。
    #[setters(generate = true, strip_option)]
    runtime: Option<String>,
    /// 需要过滤的函数GPU卡型。
    #[setters(generate = true, strip_option)]
    gpu_type: Option<String>,
    /// 需要过滤的函数描述。
    #[setters(generate = true, strip_option)]
    description: Option<String>,
    /// 函数名称。
    #[setters(generate = true, strip_option)]
    function_name: Option<String>,
    /// 资源组 ID
    #[setters(generate = true, strip_option)]
    resource_group_id: Option<String>,
}

impl sealed::Bound for ListFunctions {}

impl ListFunctions {
    pub fn new() -> Self {
        Self {
            prefix: None,
            next_token: None,
            limit: None,
            fc_version: None,
            tags: None,
            runtime: None,
            gpu_type: None,
            description: None,
            function_name: None,
            resource_group_id: None,
        }
    }
}

impl crate::Request for ListFunctions {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListFunctions";
    const URL_PATH: &'static str = "/2023-03-30/functions";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListFunctionsOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(10);

        if let Some(f) = &self.description {
            params.push(("description".into(), (f).into()));
        }

        if let Some(f) = &self.fc_version {
            params.push(("fcVersion".into(), (f).into()));
        }

        if let Some(f) = &self.function_name {
            params.push(("functionName".into(), (f).into()));
        }

        if let Some(f) = &self.gpu_type {
            params.push(("gpuType".into(), (f).into()));
        }

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        if let Some(f) = &self.resource_group_id {
            params.push(("resourceGroupId".into(), (f).into()));
        }

        if let Some(f) = &self.runtime {
            params.push(("runtime".into(), (f).into()));
        }

        if let Some(f) = &self.tags {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("tags".into(), json.into()));
            }
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
pub struct InvokeFunction {
    /// 函数名称
    function_name: String,
    /// 函数版本或别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
    /// 函数调用类型。Sync或者Async。
    #[setters(generate = true, strip_option)]
    x_fc_invocation_type: Option<String>,
    /// 函数调用返回日志类型。None或者Tail。
    #[setters(generate = true, strip_option)]
    x_fc_log_type: Option<String>,
    /// 函数调用参数
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 异步任务 ID。您需要事先开启异步任务。
    ///
    /// > 建议您在使用 SDK 调用时设置与业务相关的 ID，方便对相关执行进行后续操作。例如，一个视频处理函数可以使用视频文件名作为调用 ID，通过该 ID 可以查看视频是否处理完成或终止视频的处理。该 ID 的命名规则只能以英文大小写字母或下划线（_）开头，由英文大小写字母、数字（0-9）、下划线（_）及短划线（-）组成，不超过 128 个字符。如果您未设置异步调用的 ID，系统则会自动生成一个 ID。
    #[setters(generate = true, strip_option)]
    x_fc_async_task_id: Option<String>,
}

impl sealed::Bound for InvokeFunction {}

impl InvokeFunction {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
            x_fc_invocation_type: None,
            x_fc_log_type: None,
            body: None,
            x_fc_async_task_id: None,
        }
    }
}

impl crate::Request for InvokeFunction {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "InvokeFunction";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/invocations";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::BinaryResponseWrap;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        let mut headers = Vec::with_capacity(3);

        if let Some(f) = &self.x_fc_async_task_id {
            headers.push(("x-fc-async-task-id".into(), f.to_string()));
        }

        if let Some(f) = &self.x_fc_invocation_type {
            headers.push(("x-fc-invocation-type".into(), f.to_string()));
        }

        if let Some(f) = &self.x_fc_log_type {
            headers.push(("x-fc-log-type".into(), f.to_string()));
        }

        headers
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateFunction {
    /// 函数名称
    function_name: String,
    /// 函数信息
    body: UpdateFunctionInput,
}

impl sealed::Bound for UpdateFunction {}

impl UpdateFunction {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for UpdateFunction {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UpdateFunction";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}";

    type Body = crate::JsonBody<UpdateFunctionInput>;

    type ResponseWrap = crate::JsonResponseWrap<Function>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct EnableFunctionInvocation {
    /// 允许调用的函数名称
    function_name: String,
}

impl sealed::Bound for EnableFunctionInvocation {}

impl EnableFunctionInvocation {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
        }
    }
}
impl crate::ToFormData for EnableFunctionInvocation {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for EnableFunctionInvocation {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "EnableFunctionInvocation";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/invoke/enable";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<EnableFunctionInvocationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DisableFunctionInvocation {
    /// 禁止调用的请求参数
    #[setters(generate = true, strip_option)]
    body: Option<FunctionInvocationbody>,
    /// 禁止调用的函数名称
    function_name: String,
}

impl sealed::Bound for DisableFunctionInvocation {}

impl DisableFunctionInvocation {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            body: None,
            function_name: function_name.into(),
        }
    }
}

impl crate::Request for DisableFunctionInvocation {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DisableFunctionInvocation";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/invoke/disable";

    type Body = crate::JsonBody<FunctionInvocationbody>;

    type ResponseWrap = crate::JsonResponseWrap<DisableFunctionInvocationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAlias {
    /// 函数名称
    function_name: String,
    /// 函数别名名称
    alias_name: String,
}

impl sealed::Bound for DeleteAlias {}

impl DeleteAlias {
    pub fn new(function_name: impl Into<String>, alias_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            alias_name: alias_name.into(),
        }
    }
}
impl crate::ToFormData for DeleteAlias {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteAlias {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteAlias";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/aliases/{aliasName}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{aliasName}", self.alias_name.to_string()),
            ("{functionName}", self.function_name.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAlias {
    /// 函数名称
    function_name: String,
    /// 函数别名名称
    alias_name: String,
}

impl sealed::Bound for GetAlias {}

impl GetAlias {
    pub fn new(function_name: impl Into<String>, alias_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            alias_name: alias_name.into(),
        }
    }
}

impl crate::Request for GetAlias {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAlias";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/aliases/{aliasName}";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<Alias>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{aliasName}", self.alias_name.to_string()),
            ("{functionName}", self.function_name.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListAliases {
    /// 函数名称
    function_name: String,
    /// 别名名称前缀
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回的别名数量
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListAliases {}

impl ListAliases {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            prefix: None,
            next_token: None,
            limit: None,
        }
    }
}

impl crate::Request for ListAliases {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListAliases";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/aliases";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListAliasesOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateAlias {
    /// 函数名称
    function_name: String,
    /// 函数别名名称
    alias_name: String,
    /// 待更新的别名信息
    body: UpdateAliasInput,
}

impl sealed::Bound for UpdateAlias {}

impl UpdateAlias {
    pub fn new(function_name: impl Into<String>, alias_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            alias_name: alias_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for UpdateAlias {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UpdateAlias";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/aliases/{aliasName}";

    type Body = crate::JsonBody<UpdateAliasInput>;

    type ResponseWrap = crate::JsonResponseWrap<Alias>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{aliasName}", self.alias_name.to_string()),
            ("{functionName}", self.function_name.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateAlias {
    /// 函数名称
    function_name: String,
    /// 创建别名的请求参数
    body: CreateAliasInput,
}

impl sealed::Bound for CreateAlias {}

impl CreateAlias {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for CreateAlias {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateAlias";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/aliases";

    type Body = crate::JsonBody<CreateAliasInput>;

    type ResponseWrap = crate::JsonResponseWrap<Alias>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateTrigger {
    /// 函数名称
    function_name: String,
    /// 触发器配置
    body: CreateTriggerInput,
}

impl sealed::Bound for CreateTrigger {}

impl CreateTrigger {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for CreateTrigger {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateTrigger";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/triggers";

    type Body = crate::JsonBody<CreateTriggerInput>;

    type ResponseWrap = crate::JsonResponseWrap<Trigger>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteTrigger {
    /// 函数名称
    function_name: String,
    /// 触发器名称
    trigger_name: String,
}

impl sealed::Bound for DeleteTrigger {}

impl DeleteTrigger {
    pub fn new(function_name: impl Into<String>, trigger_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            trigger_name: trigger_name.into(),
        }
    }
}
impl crate::ToFormData for DeleteTrigger {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteTrigger {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteTrigger";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/triggers/{triggerName}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{triggerName}", self.trigger_name.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetTrigger {
    /// 函数名称
    function_name: String,
    /// 触发器名称
    trigger_name: String,
}

impl sealed::Bound for GetTrigger {}

impl GetTrigger {
    pub fn new(function_name: impl Into<String>, trigger_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            trigger_name: trigger_name.into(),
        }
    }
}

impl crate::Request for GetTrigger {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetTrigger";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/triggers/{triggerName}";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<Trigger>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{triggerName}", self.trigger_name.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListTriggers {
    /// 函数名称
    function_name: String,
    /// 触发器名称前缀
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回的触发器数量
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListTriggers {}

impl ListTriggers {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            prefix: None,
            next_token: None,
            limit: None,
        }
    }
}

impl crate::Request for ListTriggers {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListTriggers";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/triggers";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListTriggersOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateTrigger {
    /// 函数名称
    function_name: String,
    /// 触发器名称
    trigger_name: String,
    /// 触发器配置
    body: UpdateTriggerInput,
}

impl sealed::Bound for UpdateTrigger {}

impl UpdateTrigger {
    pub fn new(function_name: impl Into<String>, trigger_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            trigger_name: trigger_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for UpdateTrigger {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UpdateTrigger";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/triggers/{triggerName}";

    type Body = crate::JsonBody<UpdateTriggerInput>;

    type ResponseWrap = crate::JsonResponseWrap<Trigger>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{triggerName}", self.trigger_name.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteAsyncInvokeConfig {
    /// 函数名称
    function_name: String,
    /// 函数版本或别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for DeleteAsyncInvokeConfig {}

impl DeleteAsyncInvokeConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}
impl crate::ToFormData for DeleteAsyncInvokeConfig {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteAsyncInvokeConfig {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteAsyncInvokeConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/async-invoke-config";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetAsyncInvokeConfig {
    /// 函数名称
    function_name: String,
    /// 函数版本或别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for GetAsyncInvokeConfig {}

impl GetAsyncInvokeConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}

impl crate::Request for GetAsyncInvokeConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAsyncInvokeConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/async-invoke-config";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<AsyncConfig>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListAsyncInvokeConfigs {
    /// 函数名称，若不指定则列出所有函数的异步调用配置
    #[setters(generate = true, strip_option)]
    function_name: Option<String>,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回的数量限制
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListAsyncInvokeConfigs {}

impl ListAsyncInvokeConfigs {
    pub fn new() -> Self {
        Self {
            function_name: None,
            next_token: None,
            limit: None,
        }
    }
}

impl crate::Request for ListAsyncInvokeConfigs {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListAsyncInvokeConfigs";
    const URL_PATH: &'static str = "/2023-03-30/async-invoke-configs";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListAsyncInvokeConfigOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.function_name {
            params.push(("functionName".into(), (f).into()));
        }

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
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
pub struct PutAsyncInvokeConfig {
    /// 函数异步调用配置
    body: PutAsyncInvokeConfigInput,
    /// 函数名称
    function_name: String,
    /// 函数版本或别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for PutAsyncInvokeConfig {}

impl PutAsyncInvokeConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            body: Default::default(),
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}

impl crate::Request for PutAsyncInvokeConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAsyncInvokeConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/async-invoke-config";

    type Body = crate::JsonBody<PutAsyncInvokeConfigInput>;

    type ResponseWrap = crate::JsonResponseWrap<AsyncConfig>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteProvisionConfig {
    /// 函数名称
    function_name: String,
    /// 函数别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for DeleteProvisionConfig {}

impl DeleteProvisionConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}
impl crate::ToFormData for DeleteProvisionConfig {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteProvisionConfig {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteProvisionConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/provision-config";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetProvisionConfig {
    /// 函数名称
    function_name: String,
    /// 函数别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for GetProvisionConfig {}

impl GetProvisionConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}

impl crate::Request for GetProvisionConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetProvisionConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/provision-config";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ProvisionConfig>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListProvisionConfigs {
    /// 函数名称，若不指定则列出所有函数的预留配置
    #[setters(generate = true, strip_option)]
    function_name: Option<String>,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回的预留配置数量
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListProvisionConfigs {}

impl ListProvisionConfigs {
    pub fn new() -> Self {
        Self {
            function_name: None,
            next_token: None,
            limit: None,
        }
    }
}

impl crate::Request for ListProvisionConfigs {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListProvisionConfigs";
    const URL_PATH: &'static str = "/2023-03-30/provision-configs";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListProvisionConfigsOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.function_name {
            params.push(("functionName".into(), (f).into()));
        }

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
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
pub struct PutProvisionConfig {
    /// 函数名称
    function_name: String,
    /// 函数别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
    /// 预留配置信息
    body: PutProvisionConfigInput,
}

impl sealed::Bound for PutProvisionConfig {}

impl PutProvisionConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
            body: Default::default(),
        }
    }
}

impl crate::Request for PutProvisionConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutProvisionConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/provision-config";

    type Body = crate::JsonBody<PutProvisionConfigInput>;

    type ResponseWrap = crate::JsonResponseWrap<ProvisionConfig>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteConcurrencyConfig {
    /// 函数名称
    function_name: String,
}

impl sealed::Bound for DeleteConcurrencyConfig {}

impl DeleteConcurrencyConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
        }
    }
}
impl crate::ToFormData for DeleteConcurrencyConfig {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteConcurrencyConfig {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteConcurrencyConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/concurrency";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetConcurrencyConfig {
    /// 函数名称
    function_name: String,
}

impl sealed::Bound for GetConcurrencyConfig {}

impl GetConcurrencyConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
        }
    }
}

impl crate::Request for GetConcurrencyConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetConcurrencyConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/concurrency";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ConcurrencyConfig>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListConcurrencyConfigs {
    /// 函数名称，若不指定则列出所有函数的并发度配置
    #[setters(generate = true, strip_option)]
    function_name: Option<String>,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回的数量限制
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListConcurrencyConfigs {}

impl ListConcurrencyConfigs {
    pub fn new() -> Self {
        Self {
            function_name: None,
            next_token: None,
            limit: None,
        }
    }
}

impl crate::Request for ListConcurrencyConfigs {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListConcurrencyConfigs";
    const URL_PATH: &'static str = "/2023-03-30/concurrency-configs";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListConcurrencyConfigsOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.function_name {
            params.push(("functionName".into(), (f).into()));
        }

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
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
pub struct PutConcurrencyConfig {
    /// 函数名称
    function_name: String,
    /// 函数并发度配置信息
    body: PutConcurrencyInput,
}

impl sealed::Bound for PutConcurrencyConfig {}

impl PutConcurrencyConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for PutConcurrencyConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutConcurrencyConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/concurrency";

    type Body = crate::JsonBody<PutConcurrencyInput>;

    type ResponseWrap = crate::JsonResponseWrap<ConcurrencyConfig>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateLayerVersion {
    /// 层名称
    layer_name: String,
    /// 层配置信息
    body: CreateLayerVersionInput,
}

impl sealed::Bound for CreateLayerVersion {}

impl CreateLayerVersion {
    pub fn new(layer_name: impl Into<String>) -> Self {
        Self {
            layer_name: layer_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for CreateLayerVersion {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateLayerVersion";
    const URL_PATH: &'static str = "/2023-03-30/layers/{layerName}/versions";

    type Body = crate::JsonBody<CreateLayerVersionInput>;

    type ResponseWrap = crate::JsonResponseWrap<Layer>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{layerName}", self.layer_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteLayerVersion {
    /// 层名称
    layer_name: String,
    /// 层版本
    version: i32,
}

impl sealed::Bound for DeleteLayerVersion {}

impl DeleteLayerVersion {
    pub fn new(layer_name: impl Into<String>, version: impl Into<i32>) -> Self {
        Self {
            layer_name: layer_name.into(),
            version: version.into(),
        }
    }
}
impl crate::ToFormData for DeleteLayerVersion {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteLayerVersion {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteLayerVersion";
    const URL_PATH: &'static str = "/2023-03-30/layers/{layerName}/versions/{version}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{layerName}", self.layer_name.to_string()),
            ("{version}", self.version.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLayerVersion {
    /// 层名称
    layer_name: String,
    /// 层版本
    version: i32,
}

impl sealed::Bound for GetLayerVersion {}

impl GetLayerVersion {
    pub fn new(layer_name: impl Into<String>, version: impl Into<i32>) -> Self {
        Self {
            layer_name: layer_name.into(),
            version: version.into(),
        }
    }
}

impl crate::Request for GetLayerVersion {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLayerVersion";
    const URL_PATH: &'static str = "/2023-03-30/layers/{layerName}/versions/{version}";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<Layer>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{layerName}", self.layer_name.to_string()),
            ("{version}", self.version.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetLayerVersionByArn {
    /// 层资源标识
    arn: String,
}

impl sealed::Bound for GetLayerVersionByArn {}

impl GetLayerVersionByArn {
    pub fn new(arn: impl Into<String>) -> Self {
        Self { arn: arn.into() }
    }
}

impl crate::Request for GetLayerVersionByArn {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetLayerVersionByArn";
    const URL_PATH: &'static str = "/2023-03-30/layerarn/{arn}";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<Layer>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{arn}", self.arn.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListLayerVersions {
    /// 层名称
    layer_name: String,
    /// 起始版本
    #[setters(generate = true, strip_option)]
    start_version: Option<String>,
    /// 返回的版本数量
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListLayerVersions {}

impl ListLayerVersions {
    pub fn new(layer_name: impl Into<String>) -> Self {
        Self {
            layer_name: layer_name.into(),
            start_version: None,
            limit: None,
        }
    }
}

impl crate::Request for ListLayerVersions {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListLayerVersions";
    const URL_PATH: &'static str = "/2023-03-30/layers/{layerName}/versions";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListLayerVersionOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.start_version {
            params.push(("startVersion".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{layerName}", self.layer_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListLayers {
    /// 层名称前缀
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回的层数量
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
    /// 是否公开层。true或者false。
    #[setters(generate = true, strip_option)]
    public: Option<String>,
    /// 是否官方层。true或者false。
    #[setters(generate = true, strip_option)]
    official: Option<String>,
}

impl sealed::Bound for ListLayers {}

impl ListLayers {
    pub fn new() -> Self {
        Self {
            prefix: None,
            next_token: None,
            limit: None,
            public: None,
            official: None,
        }
    }
}

impl crate::Request for ListLayers {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListLayers";
    const URL_PATH: &'static str = "/2023-03-30/layers";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListLayersOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
        }

        if let Some(f) = &self.official {
            params.push(("official".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        if let Some(f) = &self.public {
            params.push(("public".into(), (f).into()));
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
pub struct PutLayerACL {
    /// 层名称
    layer_name: String,
    /// 是否公开层。true或者false。
    #[setters(generate = true, strip_option)]
    public: Option<String>,
    /// 设置层的访问权限，1：公开，0：私有，默认为私有
    #[setters(generate = true, strip_option)]
    acl: Option<String>,
}

impl sealed::Bound for PutLayerACL {}

impl PutLayerACL {
    pub fn new(layer_name: impl Into<String>) -> Self {
        Self {
            layer_name: layer_name.into(),
            public: None,
            acl: None,
        }
    }
}
impl crate::ToFormData for PutLayerACL {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for PutLayerACL {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutLayerACL";
    const URL_PATH: &'static str = "/2023-03-30/layers/{layerName}/acl";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

        if let Some(f) = &self.acl {
            params.push(("acl".into(), (f).into()));
        }

        if let Some(f) = &self.public {
            params.push(("public".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{layerName}", self.layer_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListInstances {
    /// 函数名称
    function_name: String,
    /// 函数版本或别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
    /// 是否列出所有实例。true或者false。
    #[setters(generate = true, strip_option)]
    with_all_active: Option<bool>,
    #[setters(generate = true, strip_option)]
    instance_status: Option<Vec<InstancesinstanceStatus>>,
    #[setters(generate = true, strip_option)]
    start_time_ms: Option<i64>,
    #[setters(generate = true, strip_option)]
    end_time_ms: Option<i64>,
    #[setters(generate = true, strip_option)]
    start_key: Option<String>,
    #[setters(generate = true, strip_option)]
    limit: Option<String>,
    #[setters(generate = true, strip_option)]
    instance_ids: Option<Vec<String>>,
}

impl sealed::Bound for ListInstances {}

impl ListInstances {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
            with_all_active: None,
            instance_status: None,
            start_time_ms: None,
            end_time_ms: None,
            start_key: None,
            limit: None,
            instance_ids: None,
        }
    }
}

impl crate::Request for ListInstances {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListInstances";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/instances";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListInstancesOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(8);

        if let Some(f) = &self.end_time_ms {
            params.push(("endTimeMs".into(), (f).into()));
        }

        if let Some(f) = &self.instance_ids {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("instanceIds".into(), json.into()));
            }
        }

        if let Some(f) = &self.instance_status {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("instanceStatus".into(), json.into()));
            }
        }

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        if let Some(f) = &self.start_key {
            params.push(("startKey".into(), (f).into()));
        }

        if let Some(f) = &self.start_time_ms {
            params.push(("startTimeMs".into(), (f).into()));
        }

        if let Some(f) = &self.with_all_active {
            params.push(("withAllActive".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListVpcBindings {
    /// 函数名称
    function_name: String,
}

impl sealed::Bound for ListVpcBindings {}

impl ListVpcBindings {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
        }
    }
}

impl crate::Request for ListVpcBindings {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListVpcBindings";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/vpc-bindings";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListVpcBindingsOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateVpcBinding {
    /// 函数名称
    function_name: String,
    /// VPC绑定配置
    body: CreateVpcBindingInput,
}

impl sealed::Bound for CreateVpcBinding {}

impl CreateVpcBinding {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Default::default(),
        }
    }
}

impl crate::Request for CreateVpcBinding {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateVpcBinding";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/vpc-bindings";

    type Body = crate::JsonBody<CreateVpcBindingInput>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteVpcBinding {
    /// 函数名称
    function_name: String,
    /// VPC实例ID
    vpc_id: String,
}

impl sealed::Bound for DeleteVpcBinding {}

impl DeleteVpcBinding {
    pub fn new(function_name: impl Into<String>, vpc_id: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            vpc_id: vpc_id.into(),
        }
    }
}
impl crate::ToFormData for DeleteVpcBinding {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteVpcBinding {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteVpcBinding";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/vpc-bindings/{vpcId}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{vpcId}", self.vpc_id.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct TagResources {
    /// 资源标签配置
    body: TagResourcesInput,
}

impl sealed::Bound for TagResources {}

impl TagResources {
    pub fn new() -> Self {
        Self {
            body: Default::default(),
        }
    }
}

impl crate::Request for TagResources {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "TagResources";
    const URL_PATH: &'static str = "/2023-03-30/tags-v2";

    type Body = crate::JsonBody<TagResourcesInput>;

    type ResponseWrap = crate::JsonResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListTagResources {
    /// 资源类型。
    resource_type: String,
    /// 资源ID列表。
    #[setters(generate = true, strip_option)]
    resource_id: Option<Vec<String>>,
    /// 标签列表。
    ///
    /// 一次最多支持查询20个标签。
    #[setters(generate = true, strip_option)]
    tag: Option<Vec<ResourcesTag>>,
    /// 下一个查询开始Token。
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回的资源数量。
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListTagResources {}

impl ListTagResources {
    pub fn new(resource_type: impl Into<String>) -> Self {
        Self {
            resource_type: resource_type.into(),
            resource_id: None,
            tag: None,
            next_token: None,
            limit: None,
        }
    }
}

impl crate::Request for ListTagResources {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListTagResources";
    const URL_PATH: &'static str = "/2023-03-30/tags-v2";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListTagResourcesOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);

        if let Some(f) = &self.limit {
            params.push(("Limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("NextToken".into(), (f).into()));
        }

        if let Some(f) = &self.resource_id {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("ResourceId".into(), json.into()));
            }
        }
        params.push(("ResourceType".into(), (&self.resource_type).into()));

        if let Some(f) = &self.tag {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("Tag".into(), json.into()));
            }
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
pub struct UntagResources {
    /// 资源类型
    resource_type: String,
    /// 资源标识列表
    resource_id: Vec<String>,
    /// 要移除的标签。最多指定50个。
    #[setters(generate = true, strip_option)]
    tag_key: Option<Vec<String>>,
    /// 是否删除所有标签
    #[setters(generate = true, strip_option)]
    all: Option<bool>,
}

impl sealed::Bound for UntagResources {}

impl UntagResources {
    pub fn new(resource_type: impl Into<String>, resource_id: impl Into<Vec<String>>) -> Self {
        Self {
            resource_type: resource_type.into(),
            resource_id: resource_id.into(),
            tag_key: None,
            all: None,
        }
    }
}
impl crate::ToFormData for UntagResources {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for UntagResources {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "UntagResources";
    const URL_PATH: &'static str = "/2023-03-30/tags-v2";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<crate::OpenObjectResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(4);

        if let Some(f) = &self.all {
            params.push(("All".into(), (f).into()));
        }
        if let Ok(json) = serde_json::to_string(&self.resource_id) {
            params.push(("ResourceId".into(), json.into()));
        }

        params.push(("ResourceType".into(), (&self.resource_type).into()));

        if let Some(f) = &self.tag_key {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("TagKey".into(), json.into()));
            }
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
pub struct GetAsyncTask {
    /// 函数名称。
    function_name: String,
    /// 异步任务ID。
    task_id: String,
    /// 函数版本或别名。
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for GetAsyncTask {}

impl GetAsyncTask {
    pub fn new(function_name: impl Into<String>, task_id: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            task_id: task_id.into(),
            qualifier: None,
        }
    }
}

impl crate::Request for GetAsyncTask {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetAsyncTask";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/async-tasks/{taskId}";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<AsyncTask>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{taskId}", self.task_id.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListAsyncTasks {
    /// 函数名称。
    function_name: String,
    /// 异步任务ID前缀，指定后会返回符合前缀的异步任务列表。
    #[setters(generate = true, strip_option)]
    prefix: Option<String>,
    /// 分页标记，用来返回更多结果。第一次查询不需要提供这个参数，后续查询的 Token 从返回结果中获取。
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回异步任务的数量。默认返回 20 个，取值范围[1,100]。
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
    /// 函数版本或别名。
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
    /// 异步任务执行状态。
    /// - Enqueued：异步消息已入队，等待处理。
    ///
    /// - Dequeued：异步消息已出队，等待触发。
    ///
    /// - Running：调用执行中。
    ///
    /// - Succeeded：调用执行成功。
    ///
    /// - Failed：调用执行失败。
    ///
    /// - Stopped：调用执行终止。
    ///
    /// - Stopping：执行停止中。
    ///
    /// - Expired：您为任务配置了最长排队等待的期限。该任务因为超期被丢弃（任务未被执行）。
    ///
    /// - Invalid：您的执行因函数被删除等原因处于无效状态（任务未被执行）。
    ///
    /// - Retrying：异步调用因执行错误重试中。
    #[setters(generate = true, strip_option)]
    status: Option<String>,
    /// 异步任务启动时间段的开始时间。
    #[setters(generate = true, strip_option)]
    started_time_begin: Option<i64>,
    /// 异步任务启动时间段的结束时间。
    #[setters(generate = true, strip_option)]
    started_time_end: Option<i64>,
    /// 返回异步任务列表的排序方式。
    ///
    /// - asc 表示升序
    ///
    /// - desc 表示降序
    #[setters(generate = true, strip_option)]
    sort_order_by_time: Option<String>,
    /// 是否返回异步任务的入参。
    ///
    /// - true：当该参数设置为true时，异步任务的列表将返回`invocationPayload`字段。
    ///
    /// - false：当该参数设置为false时，则不返回`invocationPayload`字段。
    ///
    /// > `invocationPayload`字段表示异步任务函数运行时的输入参数。
    #[setters(generate = true, strip_option)]
    include_payload: Option<bool>,
}

impl sealed::Bound for ListAsyncTasks {}

impl ListAsyncTasks {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            prefix: None,
            next_token: None,
            limit: None,
            qualifier: None,
            status: None,
            started_time_begin: None,
            started_time_end: None,
            sort_order_by_time: None,
            include_payload: None,
        }
    }
}

impl crate::Request for ListAsyncTasks {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListAsyncTasks";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/async-tasks";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListAsyncTaskOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(9);

        if let Some(f) = &self.include_payload {
            params.push(("includePayload".into(), (f).into()));
        }

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
        }

        if let Some(f) = &self.prefix {
            params.push(("prefix".into(), (f).into()));
        }

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        if let Some(f) = &self.sort_order_by_time {
            params.push(("sortOrderByTime".into(), (f).into()));
        }

        if let Some(f) = &self.started_time_begin {
            params.push(("startedTimeBegin".into(), (f).into()));
        }

        if let Some(f) = &self.started_time_end {
            params.push(("startedTimeEnd".into(), (f).into()));
        }

        if let Some(f) = &self.status {
            params.push(("status".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct StopAsyncTask {
    /// 函数名称。
    function_name: String,
    /// 异步任务ID。
    task_id: String,
    /// 函数版本或别名。
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for StopAsyncTask {}

impl StopAsyncTask {
    pub fn new(function_name: impl Into<String>, task_id: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            task_id: task_id.into(),
            qualifier: None,
        }
    }
}
impl crate::ToFormData for StopAsyncTask {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for StopAsyncTask {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "StopAsyncTask";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/async-tasks/{taskId}/stop";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{taskId}", self.task_id.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateSession {
    /// 创建Session的函数名称
    function_name: String,
    /// 用于指定Sesion所属的版本或别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
    /// 创建会话配置
    #[setters(generate = true, strip_option)]
    body: Option<CreateSessionInput>,
}

impl sealed::Bound for CreateSession {}

impl CreateSession {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
            body: None,
        }
    }
}

impl crate::Request for CreateSession {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateSession";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/sessions";

    type Body = crate::JsonBody<CreateSessionInput>;

    type ResponseWrap = crate::JsonResponseWrap<Session>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetSession {
    /// 会话所属函数名称
    function_name: String,
    /// 查询的会话ID值
    session_id: String,
    /// 查询的会话ID关联的函数别名或版本信息
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for GetSession {}

impl GetSession {
    pub fn new(function_name: impl Into<String>, session_id: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            session_id: session_id.into(),
            qualifier: None,
        }
    }
}

impl crate::Request for GetSession {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetSession";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/sessions/{sessionId}";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<Session>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{sessionId}", self.session_id.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateSession {
    /// Session所属的函数名称
    function_name: String,
    /// 待更新的会话ID值
    session_id: String,
    /// 待更新的SessionID关联的函数别名或版本信息
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
    /// 会话更新配置
    #[setters(generate = true, strip_option)]
    body: Option<UpdateSessionInput>,
}

impl sealed::Bound for UpdateSession {}

impl UpdateSession {
    pub fn new(function_name: impl Into<String>, session_id: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            session_id: session_id.into(),
            qualifier: None,
            body: None,
        }
    }
}

impl crate::Request for UpdateSession {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UpdateSession";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/sessions/{sessionId}";

    type Body = crate::JsonBody<UpdateSessionInput>;

    type ResponseWrap = crate::JsonResponseWrap<Session>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{sessionId}", self.session_id.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListSessions {
    /// 函数名
    function_name: String,
    /// 函数别名或版本信息
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
    /// 返回的会话数量，如不填写默认返回 20条。
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 需要过滤的会话状态，默认返回所有Active/Expired状态的会话信息，也可指定 Active 仅获取活跃的会话信息，或指定Expired仅获取过期的会话信息。
    #[setters(generate = true, strip_option)]
    session_status: Option<String>,
    /// 过滤的SessionId值，如指定，可返回此会话关联的所有 Active/Expired 状态信息。
    #[setters(generate = true, strip_option)]
    session_id: Option<String>,
}

impl sealed::Bound for ListSessions {}

impl ListSessions {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
            limit: None,
            next_token: None,
            session_status: None,
            session_id: None,
        }
    }
}

impl crate::Request for ListSessions {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListSessions";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/sessions";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListSessionsOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
        }

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        if let Some(f) = &self.session_id {
            params.push(("sessionId".into(), (f).into()));
        }

        if let Some(f) = &self.session_status {
            params.push(("sessionStatus".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteSession {
    /// 函数名称
    function_name: String,
    /// 需要删除的会话Id值
    session_id: String,
    /// 需要删除的SessionId关联的函数别名或版本信息
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for DeleteSession {}

impl DeleteSession {
    pub fn new(function_name: impl Into<String>, session_id: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            session_id: session_id.into(),
            qualifier: None,
        }
    }
}
impl crate::ToFormData for DeleteSession {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteSession {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteSession";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/sessions/{sessionId}";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("{functionName}", self.function_name.to_string()),
            ("{sessionId}", self.session_id.to_string()),
        ])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct PutScalingConfig {
    /// 函数名称
    function_name: String,
    /// 函数别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
    /// 函数弹性配置
    #[setters(generate = true, strip_option)]
    body: Option<PutScalingConfigInput>,
}

impl sealed::Bound for PutScalingConfig {}

impl PutScalingConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
            body: None,
        }
    }
}

impl crate::Request for PutScalingConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutScalingConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/scaling-config";

    type Body = crate::JsonBody<PutScalingConfigInput>;

    type ResponseWrap = crate::JsonResponseWrap<ScalingConfigStatus>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::JsonBody(self.body.unwrap_or_default())
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteScalingConfig {
    /// 函数名称
    function_name: String,
    /// 函数别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for DeleteScalingConfig {}

impl DeleteScalingConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}
impl crate::ToFormData for DeleteScalingConfig {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteScalingConfig {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteScalingConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/scaling-config";

    type Body = crate::Form<Self>;

    type ResponseWrap = ();

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetScalingConfig {
    /// 函数名称
    function_name: String,
    /// 函数别名
    #[setters(generate = true, strip_option)]
    qualifier: Option<String>,
}

impl sealed::Bound for GetScalingConfig {}

impl GetScalingConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}

impl crate::Request for GetScalingConfig {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetScalingConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/scaling-config";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ScalingConfigStatus>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.qualifier {
            params.push(("qualifier".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("{functionName}", self.function_name.to_string())])
    }

    fn to_body(self) -> Self::Body {}
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListScalingConfigs {
    /// 函数名称
    #[setters(generate = true, strip_option)]
    function_name: Option<String>,
    /// 分页起始标记
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 返回的弹性配置数量
    #[setters(generate = true, strip_option)]
    limit: Option<i32>,
}

impl sealed::Bound for ListScalingConfigs {}

impl ListScalingConfigs {
    pub fn new() -> Self {
        Self {
            function_name: None,
            next_token: None,
            limit: None,
        }
    }
}

impl crate::Request for ListScalingConfigs {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListScalingConfigs";
    const URL_PATH: &'static str = "/2023-03-30/scaling-configs";

    type Body = ();

    type ResponseWrap = crate::JsonResponseWrap<ListScalingConfigsOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.function_name {
            params.push(("functionName".into(), (f).into()));
        }

        if let Some(f) = &self.limit {
            params.push(("limit".into(), (f).into()));
        }

        if let Some(f) = &self.next_token {
            params.push(("nextToken".into(), (f).into()));
        }

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {}
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccelerationInfo {
    /// 镜像加速状态
    #[serde(rename = "status")]
    pub status: String,
}

impl crate::FlatSerialize for AccelerationInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.status", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Alias {
    /// 灰度版本权重。
    #[serde(rename = "additionalVersionWeight")]
    pub additional_version_weight: std::collections::HashMap<String, f64>,
    /// 别名名称。
    #[serde(rename = "aliasName")]
    pub alias_name: String,
    /// 创建时间。
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// 别名描述信息。
    #[serde(rename = "description")]
    pub description: String,
    /// 修改时间。
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    /// 别名指向的版本。
    #[serde(rename = "versionId")]
    pub version_id: String,
}

impl crate::FlatSerialize for Alias {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.additional_version_weight,
            &format!("{}.additionalVersionWeight", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.alias_name,
            &format!("{}.aliasName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.created_time,
            &format!("{}.createdTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.lastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.version_id,
            &format!("{}.versionId", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for Alias {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Destination {
    /// 异步调用目标资源描述符
    #[serde(rename = "destination")]
    pub destination: String,
}

impl crate::FlatSerialize for Destination {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.destination,
            &format!("{}.destination", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DestinationConfig {
    /// 失败的回调目标结构体。
    #[serde(rename = "onFailure")]
    pub on_failure: Destination,
    /// 成功的回调目标结构体。
    #[serde(rename = "onSuccess")]
    pub on_success: Destination,
}

impl crate::FlatSerialize for DestinationConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.on_failure,
            &format!("{}.onFailure", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.on_success,
            &format!("{}.onSuccess", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AsyncConfig {
    /// 是否开启异步任务
    #[serde(rename = "asyncTask")]
    pub async_task: bool,
    /// 创建时间
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// 目标配置
    #[serde(rename = "destinationConfig")]
    pub destination_config: DestinationConfig,
    /// 函数资源标识
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    /// 最后修改时间
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    /// 事件最大存活时间
    #[serde(rename = "maxAsyncEventAgeInSeconds")]
    pub max_async_event_age_in_seconds: i64,
    /// 异步调用重试次数
    #[serde(rename = "maxAsyncRetryAttempts")]
    pub max_async_retry_attempts: i64,
}

impl crate::FlatSerialize for AsyncConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.async_task,
            &format!("{}.asyncTask", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.created_time,
            &format!("{}.createdTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.destination_config,
            &format!("{}.destinationConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.function_arn,
            &format!("{}.functionArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.lastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_async_event_age_in_seconds,
            &format!("{}.maxAsyncEventAgeInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_async_retry_attempts,
            &format!("{}.maxAsyncRetryAttempts", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for AsyncConfig {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AsyncTaskEvent {
    /// 事件负载内容
    #[serde(rename = "eventDetail")]
    pub event_detail: String,
    /// 事件 ID
    #[serde(rename = "eventId")]
    pub event_id: i64,
    /// 事件执行状态。
    ///
    /// - Enqueued：异步消息已入队，等待处理。
    ///
    /// - Succeeded：调用执行成功。
    ///
    /// - Failed：调用执行失败。
    ///
    /// - Running：调用执行中。
    ///
    /// - Stopped：调用执行终止。
    ///
    /// - Stopping：执行停止中。
    ///
    /// - Invalid：您的执行因函数被删除等原因处于无效状态（任务未被执行）。
    ///
    /// - Expired：您为任务配置了最长排队等待的期限。该任务因为超期被丢弃（任务未被执行）。
    ///
    /// - Retrying：异步调用因执行错误重试中。
    #[serde(rename = "status")]
    pub status: String,
    /// 事件发生时间，单位毫秒
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}

impl crate::FlatSerialize for AsyncTaskEvent {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.event_detail,
            &format!("{}.eventDetail", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.event_id, &format!("{}.eventId", name), params);
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.timestamp,
            &format!("{}.timestamp", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AsyncTask {
    /// 异步任务失败后的已重试次数
    #[serde(rename = "alreadyRetriedTimes")]
    pub already_retried_times: i64,
    /// 异步任务的最终状态
    #[serde(rename = "destinationStatus")]
    pub destination_status: String,
    /// 异步任务的执行耗时
    #[serde(rename = "durationMs")]
    pub duration_ms: i64,
    /// 异步任务结束时间，单位为毫秒
    #[serde(rename = "endTime")]
    pub end_time: i64,
    /// 异步任务事件列表
    #[serde(rename = "events")]
    pub events: Vec<AsyncTaskEvent>,
    /// 函数资源标识
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    /// 异步任务对应的实例 ID
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// 函数的版本或别名。
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    /// 本次异步任务对应的请求 ID
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// 异步任务执行完成后的响应内容。大小限制为 1 MB。
    /// 该字段当前处于内测阶段，如您需要使用，请[联系我们](~~2513733~~)为您开通。
    #[serde(rename = "returnPayload")]
    pub return_payload: String,
    /// 异步任务开始时间，单位为毫秒
    #[serde(rename = "startedTime")]
    pub started_time: i64,
    /// 异步任务的执行状态。
    ///
    /// - Enqueued：异步消息已入队，等待处理。
    ///
    /// - Succeeded：调用执行成功。
    ///
    /// - Failed：调用执行失败。
    ///
    /// - Running：调用执行中。
    ///
    /// - Stopped：调用执行终止。
    ///
    /// - Stopping：执行停止中。
    ///
    /// - Invalid：您的执行因函数被删除等原因处于无效状态（任务未被执行）。
    ///
    /// - Expired：您为任务配置了最长排队等待的期限。该任务因为超期被丢弃（任务未被执行）。
    ///
    /// - Retrying：异步调用因执行错误重试中。
    #[serde(rename = "status")]
    pub status: String,
    /// 异步任务失败的错误消息
    #[serde(rename = "taskErrorMessage")]
    pub task_error_message: String,
    /// 异步任务 ID
    #[serde(rename = "taskId")]
    pub task_id: String,
    /// 异步任务执行时的入参内容
    #[serde(rename = "taskPayload")]
    pub task_payload: String,
}

impl crate::FlatSerialize for AsyncTask {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.already_retried_times,
            &format!("{}.alreadyRetriedTimes", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.destination_status,
            &format!("{}.destinationStatus", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.duration_ms,
            &format!("{}.durationMs", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.end_time, &format!("{}.endTime", name), params);
        crate::FlatSerialize::flat_serialize(&self.events, &format!("{}.events", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.function_arn,
            &format!("{}.functionArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_id,
            &format!("{}.instanceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qualifier,
            &format!("{}.qualifier", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.request_id,
            &format!("{}.requestId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.return_payload,
            &format!("{}.returnPayload", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.started_time,
            &format!("{}.startedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.task_error_message,
            &format!("{}.taskErrorMessage", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.task_id, &format!("{}.taskId", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.task_payload,
            &format!("{}.taskPayload", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for AsyncTask {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuthConfig {
    /// 认证信息
    #[serde(rename = "authInfo")]
    pub auth_info: String,
    /// 认证类型。anonymous, function或者jwt。
    #[serde(rename = "authType")]
    pub auth_type: String,
}

impl crate::FlatSerialize for AuthConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.auth_info,
            &format!("{}.authInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.auth_type,
            &format!("{}.authType", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BatchWindow {
    /// 窗口中最大可容纳的事件数量。当达到此阈值，会触发窗口内的数据往下游推送。当存在多个窗口时，有一个窗口满足即触发。
    #[serde(rename = "CountBasedWindow")]
    pub count_based_window: i32,
    /// 窗口中最大可容纳的时间范围内的事件（单位秒）。当达到此阈值，会触发窗口内的数据往下游推送。当存在多个窗口时，有一个窗口满足即触发。
    #[serde(rename = "TimeBasedWindow")]
    pub time_based_window: i32,
}

impl crate::FlatSerialize for BatchWindow {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.count_based_window,
            &format!("{}.CountBasedWindow", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.time_based_window,
            &format!("{}.TimeBasedWindow", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CertConfig {
    /// 证书名称
    #[serde(rename = "certName")]
    pub cert_name: String,
    /// PEM格式证书
    #[serde(rename = "certificate")]
    pub certificate: String,
    /// PEM格式私钥
    #[serde(rename = "privateKey")]
    pub private_key: String,
}

impl crate::FlatSerialize for CertConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.cert_name,
            &format!("{}.certName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.certificate,
            &format!("{}.certificate", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.private_key,
            &format!("{}.privateKey", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChangeResourceGroupInput {
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "NewResourceGroupId")]
    pub new_resource_group_id: String,
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

impl crate::FlatSerialize for ChangeResourceGroupInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resource_id,
            &format!("{}.ResourceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.new_resource_group_id,
            &format!("{}.NewResourceGroupId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_type,
            &format!("{}.ResourceType", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChangeResourceGroupOutput {
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "NewResourceGroupId")]
    pub new_resource_group_id: String,
    #[serde(rename = "OldResourceGroupId")]
    pub old_resource_group_id: String,
}

impl crate::FlatSerialize for ChangeResourceGroupOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resource_id,
            &format!("{}.ResourceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.new_resource_group_id,
            &format!("{}.NewResourceGroupId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.old_resource_group_id,
            &format!("{}.OldResourceGroupId", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ChangeResourceGroupOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConcurrencyConfig {
    /// 阿里云资源的标识
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    /// 预留并发，函数预留账号并发的一部份，其他函数不可以使用这部份并发。预留并发包括预留实例和按量实例的总并发。
    #[serde(rename = "reservedConcurrency")]
    pub reserved_concurrency: i64,
}

impl crate::FlatSerialize for ConcurrencyConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.function_arn,
            &format!("{}.functionArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.reserved_concurrency,
            &format!("{}.reservedConcurrency", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ConcurrencyConfig {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CookieSessionAffinityConfig {
    /// 用户在一段时间内没有进行任何操作，导致会话进入空闲状态，最大时长为单个 Session 生命周期上限，取值范围为[0, 21600]。
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    /// 指从 Session 创建、使用到最终销毁的全过程。 超过生命周期，函数计算将会自动销毁Session， 不再保证亲和性，取值范围为[1, 21600]。
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    /// 单实例在同一个时间内能同时处理的最大 Session 数，取值范围为[1, 200]。
    #[serde(rename = "sessionConcurrencyPerInstance")]
    pub session_concurrency_per_instance: i64,
    /// 默认值 False，表示在 SessionID 会话过期后，可携带相同SessionID继续发起请求，系统将视为新会话绑定新实例。当配置为 True，表示在 SessionID 会话过期后，不可复用 SessionID。
    #[serde(rename = "disableSessionIdReuse")]
    pub disable_session_id_reuse: bool,
}

impl crate::FlatSerialize for CookieSessionAffinityConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.session_idle_timeout_in_seconds,
            &format!("{}.sessionIdleTimeoutInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_ttl_in_seconds,
            &format!("{}.sessionTTLInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_concurrency_per_instance,
            &format!("{}.sessionConcurrencyPerInstance", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_session_id_reuse,
            &format!("{}.disableSessionIdReuse", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAliasInput {
    /// 灰度版本权重
    #[serde(rename = "additionalVersionWeight")]
    pub additional_version_weight: std::collections::HashMap<String, f64>,
    /// 别名名称
    #[serde(rename = "aliasName")]
    pub alias_name: String,
    /// 别名描述信息
    #[serde(rename = "description")]
    pub description: String,
    /// 别名指向的版本
    #[serde(rename = "versionId")]
    pub version_id: String,
}

impl crate::FlatSerialize for CreateAliasInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.additional_version_weight,
            &format!("{}.additionalVersionWeight", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.alias_name,
            &format!("{}.aliasName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.version_id,
            &format!("{}.versionId", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EqualRule {
    /// 匹配规则
    #[serde(rename = "match")]
    pub r#match: String,
    /// 替换规则
    #[serde(rename = "replacement")]
    pub replacement: String,
}

impl crate::FlatSerialize for EqualRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.r#match, &format!("{}.match", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.replacement,
            &format!("{}.replacement", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegexRule {
    /// 匹配规则
    #[serde(rename = "match")]
    pub r#match: String,
    /// 替换规则
    #[serde(rename = "replacement")]
    pub replacement: String,
}

impl crate::FlatSerialize for RegexRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.r#match, &format!("{}.match", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.replacement,
            &format!("{}.replacement", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WildcardRule {
    /// 匹配规则
    #[serde(rename = "match")]
    pub r#match: String,
    /// 替换规则
    #[serde(rename = "replacement")]
    pub replacement: String,
}

impl crate::FlatSerialize for WildcardRule {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.r#match, &format!("{}.match", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.replacement,
            &format!("{}.replacement", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RewriteConfig {
    /// 精确匹配规则列表
    #[serde(rename = "equalRules")]
    pub equal_rules: Vec<EqualRule>,
    /// 正则匹配规则列表
    #[serde(rename = "regexRules")]
    pub regex_rules: Vec<RegexRule>,
    /// 通配匹配规则列表
    #[serde(rename = "wildcardRules")]
    pub wildcard_rules: Vec<WildcardRule>,
}

impl crate::FlatSerialize for RewriteConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.equal_rules,
            &format!("{}.equalRules", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.regex_rules,
            &format!("{}.regexRules", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.wildcard_rules,
            &format!("{}.wildcardRules", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathConfig {
    /// 函数名
    #[serde(rename = "functionName")]
    pub function_name: String,
    /// 支持的方法
    #[serde(rename = "methods")]
    pub methods: Vec<String>,
    /// 路由匹配规则
    #[serde(rename = "path")]
    pub path: String,
    /// 版本或者别名
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    /// 重写配置
    #[serde(rename = "rewriteConfig")]
    pub rewrite_config: RewriteConfig,
}

impl crate::FlatSerialize for PathConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.function_name,
            &format!("{}.functionName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.methods, &format!("{}.methods", name), params);
        crate::FlatSerialize::flat_serialize(&self.path, &format!("{}.path", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.qualifier,
            &format!("{}.qualifier", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.rewrite_config,
            &format!("{}.rewriteConfig", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RouteConfig {
    /// 路由配置列表
    #[serde(rename = "routes")]
    pub routes: Vec<PathConfig>,
}

impl crate::FlatSerialize for RouteConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.routes, &format!("{}.routes", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TLSConfig {
    /// TLS加密套件列表。
    #[serde(rename = "cipherSuites")]
    pub cipher_suites: Vec<String>,
    /// TLS最大版本号。枚举值：TLSv1.3, TLSv1.2
    #[serde(rename = "maxVersion")]
    pub max_version: String,
    /// TLS最小版本号。枚举值：TLSv1.3, TLSv1.2
    #[serde(rename = "minVersion")]
    pub min_version: String,
}

impl crate::FlatSerialize for TLSConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.cipher_suites,
            &format!("{}.cipherSuites", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_version,
            &format!("{}.maxVersion", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.min_version,
            &format!("{}.minVersion", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WAFConfig {
    /// 是否开启WAF防护
    #[serde(rename = "enableWAF")]
    pub enable_waf: bool,
}

impl crate::FlatSerialize for WAFConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.enable_waf,
            &format!("{}.enableWAF", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateCustomDomainInput {
    /// 权限认证配置。
    #[serde(rename = "authConfig")]
    pub auth_config: AuthConfig,
    /// HTTPS证书的信息。
    #[serde(rename = "certConfig")]
    pub cert_config: CertConfig,
    /// 域名。填写已在阿里云备案或接入备案的自定义域名名称。
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// 域名支持的协议类型。HTTP：仅支持HTTP协议。HTTPS：仅支持HTTPS协议。HTTP,HTTPS：支持HTTP及HTTPS协议。
    #[serde(rename = "protocol")]
    pub protocol: String,
    /// 路由表：自定义域名访问时的PATH到Function的映射。
    #[serde(rename = "routeConfig")]
    pub route_config: RouteConfig,
    /// TLS配置信息。
    #[serde(rename = "tlsConfig")]
    pub tls_config: TLSConfig,
    /// Web应用防火墙配置信息。
    #[serde(rename = "wafConfig")]
    pub waf_config: WAFConfig,
}

impl crate::FlatSerialize for CreateCustomDomainInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.auth_config,
            &format!("{}.authConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cert_config,
            &format!("{}.certConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.domain_name,
            &format!("{}.domainName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.protocol, &format!("{}.protocol", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.route_config,
            &format!("{}.routeConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.tls_config,
            &format!("{}.tlsConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.waf_config,
            &format!("{}.wafConfig", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InputCodeLocation {
    /// 函数代码包的CRC-64值。如果提供了checksum，则函数计算会校验代码包的checksum是否和提供的一致。
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// 用户存放函数代码ZIP包的OSS Bucket名称。
    #[serde(rename = "ossBucketName")]
    pub oss_bucket_name: String,
    /// 用户存放函数代码ZIP包的OSS Object名称。
    #[serde(rename = "ossObjectName")]
    pub oss_object_name: String,
    /// 函数代码ZIP包的Base 64编码。
    #[serde(rename = "zipFile")]
    pub zip_file: String,
}

impl crate::FlatSerialize for InputCodeLocation {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.checksum, &format!("{}.checksum", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.oss_bucket_name,
            &format!("{}.ossBucketName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_object_name,
            &format!("{}.ossObjectName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.zip_file, &format!("{}.zipFile", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomHealthCheckConfig {
    /// 健康检查失败次数阈值，达到该值后系统认为检查失败。取值范围1~120。默认值为3。
    #[serde(rename = "failureThreshold")]
    pub failure_threshold: i32,
    /// 容器自定义健康检查URL地址。长度不超过2048个字符。
    #[serde(rename = "httpGetUrl")]
    pub http_get_url: String,
    /// 容器启动到发起健康检查的延迟。取值范围0~120。默认值为0。
    #[serde(rename = "initialDelaySeconds")]
    pub initial_delay_seconds: i32,
    /// 健康检查周期。取值范围1~120。默认值为3。
    #[serde(rename = "periodSeconds")]
    pub period_seconds: i32,
    /// 健康检查成功次数阈值，达到该值后系统认为检查成功。取值范围1~120。默认值为1。
    #[serde(rename = "successThreshold")]
    pub success_threshold: i32,
    /// 健康检查超时时间。取值范围1~3。默认值为1。
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: i32,
}

impl crate::FlatSerialize for CustomHealthCheckConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.failure_threshold,
            &format!("{}.failureThreshold", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.http_get_url,
            &format!("{}.httpGetUrl", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.initial_delay_seconds,
            &format!("{}.initialDelaySeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.period_seconds,
            &format!("{}.periodSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.success_threshold,
            &format!("{}.successThreshold", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.timeout_seconds,
            &format!("{}.timeoutSeconds", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegistryAuthConfig {
    /// 镜像仓库密码
    #[serde(rename = "password")]
    pub password: String,
    /// 镜像仓库用户名
    #[serde(rename = "userName")]
    pub user_name: String,
}

impl crate::FlatSerialize for RegistryAuthConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.password, &format!("{}.password", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.user_name,
            &format!("{}.userName", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegistryCertConfig {
    /// 是否跳过证书验证
    #[serde(rename = "insecure")]
    pub insecure: bool,
    /// 镜像仓库CA证书
    #[serde(rename = "rootCaCertBase64")]
    pub root_ca_cert_base64: String,
}

impl crate::FlatSerialize for RegistryCertConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.insecure, &format!("{}.insecure", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.root_ca_cert_base64,
            &format!("{}.rootCaCertBase64", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegistryNetworkConfig {
    /// 可以连通镜像仓库的SecurityGroup ID
    #[serde(rename = "securityGroupId")]
    pub security_group_id: String,
    /// 可以连通镜像仓库的VSwitch ID
    #[serde(rename = "vSwitchId")]
    pub v_switch_id: String,
    /// 可以连通镜像仓库的VPC ID
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
}

impl crate::FlatSerialize for RegistryNetworkConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.security_group_id,
            &format!("{}.securityGroupId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.v_switch_id,
            &format!("{}.vSwitchId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.vpc_id, &format!("{}.vpcId", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegistryConfig {
    /// 权限认证配置
    #[serde(rename = "authConfig")]
    pub auth_config: RegistryAuthConfig,
    /// 证书配置
    #[serde(rename = "certConfig")]
    pub cert_config: RegistryCertConfig,
    /// 网络配置。
    #[serde(rename = "networkConfig")]
    pub network_config: RegistryNetworkConfig,
}

impl crate::FlatSerialize for RegistryConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.auth_config,
            &format!("{}.authConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cert_config,
            &format!("{}.certConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.network_config,
            &format!("{}.networkConfig", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomContainerConfig {
    /// 镜像加速信息。
    #[serde(rename = "accelerationInfo")]
    pub acceleration_info: AccelerationInfo,
    /// 是否开启镜像加速。Default表示开启镜像加速，None表示关闭镜像加速。
    #[serde(rename = "accelerationType")]
    pub acceleration_type: String,
    /// ACR企业版镜像仓库ID，使用ACR企业版镜像时须传入。
    #[serde(rename = "acrInstanceId")]
    pub acr_instance_id: String,
    /// 容器启动参数。
    #[serde(rename = "command")]
    pub command: Vec<String>,
    /// 容器启动命令。
    #[serde(rename = "entrypoint")]
    pub entrypoint: Vec<String>,
    /// 函数自定义健康检查配置。
    #[serde(rename = "healthCheckConfig")]
    pub health_check_config: CustomHealthCheckConfig,
    /// 容器镜像地址。
    #[serde(rename = "image")]
    pub image: String,
    /// 自定义容器运行时HTTP Server的监听端口。
    #[serde(rename = "port")]
    pub port: i32,
    /// registry related
    #[serde(rename = "registryConfig")]
    pub registry_config: RegistryConfig,
    /// 所部署的镜像的实际digest版本，函数启动时实际使用此digest指定的代码版本。由GetFunction时返回，作为参数时无需提供。
    #[serde(rename = "resolvedImageUri")]
    pub resolved_image_uri: String,
}

impl crate::FlatSerialize for CustomContainerConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.acceleration_info,
            &format!("{}.accelerationInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.acceleration_type,
            &format!("{}.accelerationType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.acr_instance_id,
            &format!("{}.acrInstanceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.command, &format!("{}.command", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.entrypoint,
            &format!("{}.entrypoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.health_check_config,
            &format!("{}.healthCheckConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.image, &format!("{}.image", name), params);
        crate::FlatSerialize::flat_serialize(&self.port, &format!("{}.port", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.registry_config,
            &format!("{}.registryConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resolved_image_uri,
            &format!("{}.resolvedImageUri", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DNSOption {
    /// 配置项名称
    #[serde(rename = "name")]
    pub name: String,
    /// 配置项值
    #[serde(rename = "value")]
    pub value: String,
}

impl crate::FlatSerialize for DNSOption {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.name", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.value", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomDNS {
    /// resolv.conf文件中的DNS解析配置列表。每一项对应一个键值对，格式为key:value，其中键为必填。
    #[serde(rename = "dnsOptions")]
    pub dns_options: Vec<DNSOption>,
    /// DNS服务器的IP地址列表。
    #[serde(rename = "nameServers")]
    pub name_servers: Vec<String>,
    /// DNS搜索域列表。
    #[serde(rename = "searches")]
    pub searches: Vec<String>,
}

impl crate::FlatSerialize for CustomDNS {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.dns_options,
            &format!("{}.dnsOptions", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.name_servers,
            &format!("{}.nameServers", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.searches, &format!("{}.searches", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomRuntimeConfig {
    /// 实例启动参数。
    #[serde(rename = "args")]
    pub args: Vec<String>,
    /// 实例启动命令。
    #[serde(rename = "command")]
    pub command: Vec<String>,
    /// 函数自定义健康检查配置。
    #[serde(rename = "healthCheckConfig")]
    pub health_check_config: CustomHealthCheckConfig,
    /// HTTP Server的监听端口。
    #[serde(rename = "port")]
    pub port: i32,
}

impl crate::FlatSerialize for CustomRuntimeConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.args, &format!("{}.args", name), params);
        crate::FlatSerialize::flat_serialize(&self.command, &format!("{}.command", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.health_check_config,
            &format!("{}.healthCheckConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.port, &format!("{}.port", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GPUConfig {
    /// GPU显存规格，单位为MB，为1024MB的倍数
    #[serde(rename = "gpuMemorySize")]
    pub gpu_memory_size: i32,
    /// GPU实例类型。
    ///
    ///  - fc.gpu.tesla.1 表示 GPU Tesla 系列实例类型。
    ///
    ///  - fc.gpu.ampere.1 表示 GPU Ampere 系列实例类型。
    ///
    ///  - fc.gpu.ada.1 表示 GPU Ada 系列实例类型。
    #[serde(rename = "gpuType")]
    pub gpu_type: String,
}

impl crate::FlatSerialize for GPUConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.gpu_memory_size,
            &format!("{}.gpuMemorySize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.gpu_type, &format!("{}.gpuType", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleHook {
    /// 回调方法的执行入口，含义与请求处理程序类似。
    #[serde(rename = "handler")]
    pub handler: String,
    /// 回调方法的超时时间，单位为秒。
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// 函数生命周期初始化阶段回调指令，生命周期回调方法的执行入口 handler 和 command 不允许同时配置，只能有一个生效，同时配置会产生错误提示
    #[serde(rename = "command")]
    pub command: Vec<String>,
}

impl crate::FlatSerialize for LifecycleHook {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.handler, &format!("{}.handler", name), params);
        crate::FlatSerialize::flat_serialize(&self.timeout, &format!("{}.timeout", name), params);
        crate::FlatSerialize::flat_serialize(&self.command, &format!("{}.command", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InstanceLifecycleConfig {
    /// Initializer回调方法配置
    #[serde(rename = "initializer")]
    pub initializer: LifecycleHook,
    /// PreStop回调方法配置
    #[serde(rename = "preStop")]
    pub pre_stop: LifecycleHook,
}

impl crate::FlatSerialize for InstanceLifecycleConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.initializer,
            &format!("{}.initializer", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.pre_stop, &format!("{}.preStop", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LogConfig {
    /// 是否开启实例级别指标。开启该功能后，您可以查看实例级别的CPU使用情况、内存使用情况、实例网络情况和实例内请求数等核心指标信息。false：默认值，表示关闭实例级别指标。true：表示开启实例级别指标。
    #[serde(rename = "enableInstanceMetrics")]
    pub enable_instance_metrics: bool,
    /// 是否开启请求级别指标。开启该功能后，您可以查看该服务下所有函数的某次调用所消耗的时间及内存。false：表示关闭请求级别指标。true：默认值，表示开启请求级别指标。
    #[serde(rename = "enableRequestMetrics")]
    pub enable_request_metrics: bool,
    /// 日志行首匹配规则
    #[serde(rename = "logBeginRule")]
    pub log_begin_rule: String,
    /// 日志服务的Logstore名称。
    #[serde(rename = "logstore")]
    pub logstore: String,
    /// 日志服务的Project名称
    #[serde(rename = "project")]
    pub project: String,
}

impl crate::FlatSerialize for LogConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.enable_instance_metrics,
            &format!("{}.enableInstanceMetrics", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_request_metrics,
            &format!("{}.enableRequestMetrics", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.log_begin_rule,
            &format!("{}.logBeginRule", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.logstore, &format!("{}.logstore", name), params);
        crate::FlatSerialize::flat_serialize(&self.project, &format!("{}.project", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NASMountConfig {
    /// 使用传输加密方式挂载。 说明：仅通用型NAS支持传输加密
    #[serde(rename = "enableTLS")]
    pub enable_tls: bool,
    /// 本地挂载目录。
    #[serde(rename = "mountDir")]
    pub mount_dir: String,
    /// NAS服务器地址。
    #[serde(rename = "serverAddr")]
    pub server_addr: String,
}

impl crate::FlatSerialize for NASMountConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.enable_tls,
            &format!("{}.enableTLS", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mount_dir,
            &format!("{}.mountDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.server_addr,
            &format!("{}.serverAddr", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NASConfig {
    /// 群组ID。
    #[serde(rename = "groupId")]
    pub group_id: i32,
    /// 挂载点列表。
    #[serde(rename = "mountPoints")]
    pub mount_points: Vec<NASMountConfig>,
    /// 账号ID。
    #[serde(rename = "userId")]
    pub user_id: i32,
}

impl crate::FlatSerialize for NASConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.group_id, &format!("{}.groupId", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.mount_points,
            &format!("{}.mountPoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.user_id, &format!("{}.userId", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OSSMountPoint {
    /// 挂载的OSS Bucket。
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// 挂载的OSS Bucket路径。
    #[serde(rename = "bucketPath")]
    pub bucket_path: String,
    /// OSS访问地址。
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// 挂载目录。
    #[serde(rename = "mountDir")]
    pub mount_dir: String,
    /// 是否只读。
    #[serde(rename = "readOnly")]
    pub read_only: bool,
}

impl crate::FlatSerialize for OSSMountPoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bucket_name,
            &format!("{}.bucketName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bucket_path,
            &format!("{}.bucketPath", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.endpoint, &format!("{}.endpoint", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.mount_dir,
            &format!("{}.mountDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.read_only,
            &format!("{}.readOnly", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OSSMountConfig {
    /// OSS挂载点列表。
    #[serde(rename = "mountPoints")]
    pub mount_points: Vec<OSSMountPoint>,
}

impl crate::FlatSerialize for OSSMountConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.mount_points,
            &format!("{}.mountPoints", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TracingConfig {
    /// 链路追踪参数。参数为map[string]string，其中key为"endpoint"，value为您的链路追踪内网接入点。 例如 endpoint: http://tracing-analysis-dc-hz.aliyuncs.com/adapt_xxx/api/otlp/traces 。
    #[serde(rename = "params")]
    pub params: std::collections::HashMap<String, String>,
    /// 链路追踪协议类型，目前只支持Jaeger。
    #[serde(rename = "type")]
    pub r#type: String,
}

impl crate::FlatSerialize for TracingConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.params, &format!("{}.params", name), params);
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.type", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VPCConfig {
    /// 安全组ID。
    #[serde(rename = "securityGroupId")]
    pub security_group_id: String,
    /// 交换机列表。
    #[serde(rename = "vSwitchIds")]
    pub v_switch_ids: Vec<String>,
    /// VPC网络ID。
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
    /// 授予函数计算访问用户VPC所需权限的RAM角色
    #[serde(rename = "role")]
    pub role: String,
}

impl crate::FlatSerialize for VPCConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.security_group_id,
            &format!("{}.securityGroupId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.v_switch_ids,
            &format!("{}.vSwitchIds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.vpc_id, &format!("{}.vpcId", name), params);
        crate::FlatSerialize::flat_serialize(&self.role, &format!("{}.role", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    /// 标签名
    #[serde(rename = "Key")]
    pub key: String,
    /// 标签值
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
pub struct PolarFsMountConfig {
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "mountDir")]
    pub mount_dir: String,
    #[serde(rename = "remoteDir")]
    pub remote_dir: String,
}

impl crate::FlatSerialize for PolarFsMountConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.instance_id,
            &format!("{}.instanceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.mount_dir,
            &format!("{}.mountDir", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.remote_dir,
            &format!("{}.remoteDir", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PolarFsConfig {
    /// 群组ID。
    #[serde(rename = "groupId")]
    pub group_id: i32,
    /// 账号ID。
    #[serde(rename = "userId")]
    pub user_id: i32,
    /// 挂载点列表。
    #[serde(rename = "mountPoints")]
    pub mount_points: Vec<PolarFsMountConfig>,
}

impl crate::FlatSerialize for PolarFsConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.group_id, &format!("{}.groupId", name), params);
        crate::FlatSerialize::flat_serialize(&self.user_id, &format!("{}.userId", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.mount_points,
            &format!("{}.mountPoints", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFunctionInput {
    /// 函数代码ZIP包。code和customContainerConfig二选一。
    #[serde(rename = "code")]
    pub code: InputCodeLocation,
    /// 函数的CPU规格，单位为vCPU，为0.05 vCPU的倍数。最小值为0.05，最大值为16。同时cpu和memorySize（按GB算）比例要在1:1和1:4之间。
    #[serde(rename = "cpu")]
    pub cpu: f32,
    /// 自定义容器运行时的相关配置，成功配置后函数可以使用自定义容器镜像执行函数。code和customContainerConfig二选一。
    #[serde(rename = "customContainerConfig")]
    pub custom_container_config: CustomContainerConfig,
    /// 自定义DNS配置。
    #[serde(rename = "customDNS")]
    pub custom_dns: CustomDNS,
    /// 自定义运行时配置。
    #[serde(rename = "customRuntimeConfig")]
    pub custom_runtime_config: CustomRuntimeConfig,
    /// 函数的描述。
    #[serde(rename = "description")]
    pub description: String,
    /// 函数的磁盘规格，单位为MB，可选值为512 MB或10240 MB。
    #[serde(rename = "diskSize")]
    pub disk_size: i32,
    /// 函数的环境变量，可以在运行环境中访问设置的环境变量。
    #[serde(rename = "environmentVariables")]
    pub environment_variables: std::collections::HashMap<String, String>,
    /// 函数的名称。只能包含字母、数字、下划线（_）和短划线（-），不能以数字、短划线（-）开头，长度范围为1~64个字符。
    #[serde(rename = "functionName")]
    pub function_name: String,
    /// 函数GPU配置。
    #[serde(rename = "gpuConfig")]
    pub gpu_config: GPUConfig,
    /// 函数执行的入口，具体格式和运行时相关。
    #[serde(rename = "handler")]
    pub handler: String,
    /// 实例最大并发度。
    #[serde(rename = "instanceConcurrency")]
    pub instance_concurrency: i32,
    /// 实例生命周期回调方法配置。
    #[serde(rename = "instanceLifecycleConfig")]
    pub instance_lifecycle_config: InstanceLifecycleConfig,
    /// 是否允许函数访问公网。默认值为true。
    #[serde(rename = "internetAccess")]
    pub internet_access: bool,
    /// 层的列表。多个层会按照数组下标从大到小的顺序进行合并，下标小的层的内容会覆盖下标大的层的同名文件。
    #[serde(rename = "layers")]
    pub layers: Vec<String>,
    /// 日志配置。函数产生的日志会被写入到配置的日志库中。
    #[serde(rename = "logConfig")]
    pub log_config: LogConfig,
    /// 函数的内存规格，单位为MB，内存大小为64 MB的倍数。最小值为128MB，最大值为32GB。同时cpu和memorySize（按GB算）比例要在1:1和1:4之间。
    #[serde(rename = "memorySize")]
    pub memory_size: i32,
    /// NAS配置。配置此参数后，函数可以访问指定的NAS资源。
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
    /// OSS挂载配置。
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: OSSMountConfig,
    /// 用户授权给函数计算的RAM角色，设置后函数计算将扮演该角色生成临时访问凭证。在函数中可以使用该角色的临时访问凭证来访问指定的阿里云服务，例如OSS和OTS。
    #[serde(rename = "role")]
    pub role: String,
    /// 函数的运行时环境。目前支持的运行环境有：nodejs12, nodejs14, nodejs16, nodejs18, nodejs20, go1, python3, python3.9, python3.10, python3.12, java8, java11, php7.2, dotnetcore3.1, custom, custom.debian10, custom.debian11, custom.debian12, custom-container。
    #[serde(rename = "runtime")]
    pub runtime: String,
    /// 函数运行的超时时间，单位为秒，最小1秒，最大值为86400秒，默认值是3秒。函数超过这个时间后会被终止执行。
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// 链路追踪配置。当函数计算与链路追踪集成后，您可以记录请求在函数计算的耗时时间、查看函数的冷启动时间、记录函数内部时间的消耗等。
    #[serde(rename = "tracingConfig")]
    pub tracing_config: TracingConfig,
    /// VPC配置。配置此参数后，函数可以访问指定的VPC资源。
    #[serde(rename = "vpcConfig")]
    pub vpc_config: VPCConfig,
    /// 标签列表
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
    /// 是否禁止创建按量实例，功能开启后，不会创建按量实例，只能使用预留实例
    #[serde(rename = "disableOndemand")]
    pub disable_ondemand: bool,
    /// 函数计算调用请求的亲和策略，如需实现 MCP SSE协议的请求亲和，可设置为 MCP_SSE。如使用Cookie亲和，可设置为 GENERATED_COOKIE。如使用 Header亲和，可设置为 HEADER_FIELD。如不设置或设置为 NONE，则无亲和效果，按函数计算系统默认调度策略路由请求。
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: String,
    /// 是否允许 GPU 函数的预留实例常驻，启用该功能时，创建的函数实例不会被注入 STS token。
    #[serde(rename = "enableLongLiving")]
    pub enable_long_living: bool,
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: String,
    /// 实例隔离模式
    #[serde(rename = "instanceIsolationMode")]
    pub instance_isolation_mode: CreateFunctionInputInstanceIsolationMode,
    /// 当设置sessionAffinity亲和类型时，需设置相关的亲和配置。如MCP_SSE亲和需填充 MCPSSESessionAffinityConfig 配置。Cookie亲和需填充CookieSessionAffinityConfig配置，Header Field 亲和需填充HeaderFieldSessionAffinityConfig配置。
    #[serde(rename = "sessionAffinityConfig")]
    pub session_affinity_config: String,
    /// 实例延迟释放时间。
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: i32,
    /// 是否不注入 STS token，取值None/Env/Request/All
    /// None: 都注入
    /// Env: 环境变量不注入
    /// Request: 请求中不注入包括context/header
    /// All: 都不注入
    #[serde(rename = "disableInjectCredentials")]
    pub disable_inject_credentials: CreateFunctionInputDisableInjectCredentials,
    /// PolarFs配置。配置此参数后，函数可以访问指定的PolarFs资源。
    #[serde(rename = "polarFsConfig")]
    pub polar_fs_config: PolarFsConfig,
}

impl crate::FlatSerialize for CreateFunctionInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.code, &format!("{}.code", name), params);
        crate::FlatSerialize::flat_serialize(&self.cpu, &format!("{}.cpu", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.custom_container_config,
            &format!("{}.customContainerConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.custom_dns,
            &format!("{}.customDNS", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.custom_runtime_config,
            &format!("{}.customRuntimeConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disk_size,
            &format!("{}.diskSize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.environment_variables,
            &format!("{}.environmentVariables", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.function_name,
            &format!("{}.functionName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.gpu_config,
            &format!("{}.gpuConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.handler, &format!("{}.handler", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.instance_concurrency,
            &format!("{}.instanceConcurrency", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_lifecycle_config,
            &format!("{}.instanceLifecycleConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.internet_access,
            &format!("{}.internetAccess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.layers, &format!("{}.layers", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.log_config,
            &format!("{}.logConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.memory_size,
            &format!("{}.memorySize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.nas_config,
            &format!("{}.nasConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_mount_config,
            &format!("{}.ossMountConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.role, &format!("{}.role", name), params);
        crate::FlatSerialize::flat_serialize(&self.runtime, &format!("{}.runtime", name), params);
        crate::FlatSerialize::flat_serialize(&self.timeout, &format!("{}.timeout", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.tracing_config,
            &format!("{}.tracingConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.vpc_config,
            &format!("{}.vpcConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tags, &format!("{}.tags", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.disable_ondemand,
            &format!("{}.disableOndemand", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_affinity,
            &format!("{}.sessionAffinity", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_long_living,
            &format!("{}.enableLongLiving", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_group_id,
            &format!("{}.resourceGroupId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_isolation_mode,
            &format!("{}.instanceIsolationMode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_affinity_config,
            &format!("{}.sessionAffinityConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.idle_timeout,
            &format!("{}.idleTimeout", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_inject_credentials,
            &format!("{}.disableInjectCredentials", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.polar_fs_config,
            &format!("{}.polarFsConfig", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateLayerVersionInput {
    /// 层的代码信息。
    #[serde(rename = "code")]
    pub code: InputCodeLocation,
    /// 层支持的运行时环境列表。
    #[serde(rename = "compatibleRuntime")]
    pub compatible_runtime: Vec<String>,
    /// 层版本的描述信息。
    #[serde(rename = "description")]
    pub description: String,
    /// 层的许可协议。
    #[serde(rename = "license")]
    pub license: String,
}

impl crate::FlatSerialize for CreateLayerVersionInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.code, &format!("{}.code", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.compatible_runtime,
            &format!("{}.compatibleRuntime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.license, &format!("{}.license", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateSessionInput {
    /// Session生命周期
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    /// Session 闲置过期时间
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    /// NAS配置，配置后Session关联的实例可以访问指定NAS资源。
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
    /// 可自定义会话ID。不配置时由服务端生成。若配置则将此配置作为会话ID。仅适用于HEADER_FIELD亲和模式，
    /// 格式规范：长度限制[0,64]，仅以 **a-zA-Z0-9_** 字符做首字符，非首字符可为 **a-zA-Z0-9_-**。
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// 默认值 False，表示在 SessionID 会话过期后，可携带相同SessionID继续发起请求，系统将视为新会话绑定新实例。当配置为 True，表示在 SessionID 会话过期后，不可复用 SessionID。
    #[serde(rename = "disableSessionIdReuse")]
    pub disable_session_id_reuse: bool,
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: OSSMountConfig,
    #[serde(rename = "polarFsConfig")]
    pub polar_fs_config: PolarFsConfig,
}

impl crate::FlatSerialize for CreateSessionInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.session_ttl_in_seconds,
            &format!("{}.sessionTTLInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_idle_timeout_in_seconds,
            &format!("{}.sessionIdleTimeoutInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.nas_config,
            &format!("{}.nasConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_id,
            &format!("{}.sessionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_session_id_reuse,
            &format!("{}.disableSessionIdReuse", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_mount_config,
            &format!("{}.ossMountConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.polar_fs_config,
            &format!("{}.polarFsConfig", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTriggerInput {
    /// 触发器的描述。
    #[serde(rename = "description")]
    pub description: String,
    /// 事件源（如OSS）调用函数所需的角色。
    #[serde(rename = "invocationRole")]
    pub invocation_role: String,
    /// 函数的版本或别名。
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    /// 触发器事件源的Aliyun Resource Name。
    #[serde(rename = "sourceArn")]
    pub source_arn: String,
    /// 触发器配置，针对不同类型的触发器，配置有所不同。具体格式请参见如下对应的数据结构：
    ///   - OSS触发器：请参见[OSSTriggerConfig](~~2766465~~)。
    ///   - 日志服务触发器：请参见[SLSTriggerConfig](~~2766469~~)。
    ///   - 定时触发器：请参见[TimerTriggerConfig](~~2754638~~)。
    ///   - HTTP触发器：请参见[HTTPTriggerConfig](~~2766461~~)。
    ///   - Tablestore触发器：只需要填写完整的**SourceArn**参数便可, 这里无需额外配置，取值为空{}。
    ///   - CDN事件触发器：请参见[CDNTriggerConfig](~~2766462~~)。
    ///   - MNS主题触发器：请参见[MNSTopicTriggerConfig](~~2766464~~)。
    ///   - EventBridge触发器：请参见[EventBridgeTriggerConfig](~~2766447~~)。
    #[serde(rename = "triggerConfig")]
    pub trigger_config: String,
    /// 触发器的名称。要求只能包含字母、数字、下划线(_)和短划线(-)。不能以数字、短划线(-)开头，长度限制为1~128个字符。
    #[serde(rename = "triggerName")]
    pub trigger_name: String,
    /// 触发器的类型。
    /// 具体取值和触发器类型对应关系如下所示：
    ///   - **oss**：OSS触发器。更多信息，请参见[OSS触发器概述](~~2513613~~)。
    ///   - **log**：日志服务触发器。更多信息，请参见[日志服务触发器概述](~~2513638~~)。
    ///   - **timer**：定时触发器。更多信息，请参见[定时触发器概述](~~2513611~~)。
    ///   - **http**：HTTP触发器。更多信息，请参见[HTTP触发器概述](~~2513634~~)。
    ///   - **tablestore**：Tablestore触发器。更多信息，请参见[Tablestore触发器概述](~~2513640~~)。
    ///   - **cdn_events**：CDN事件触发器。更多信息，请参见[CDN事件触发器概述](~~2513636~~)。
    ///   - **mns_topic**：MNS主题触发器。更多信息，请参见[MNS主题触发器概述](~~2513641~~)。
    ///   - **eventbridge**：EventBridge触发器。
    #[serde(rename = "triggerType")]
    pub trigger_type: String,
}

impl crate::FlatSerialize for CreateTriggerInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.invocation_role,
            &format!("{}.invocationRole", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qualifier,
            &format!("{}.qualifier", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_arn,
            &format!("{}.sourceArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_config,
            &format!("{}.triggerConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_name,
            &format!("{}.triggerName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_type,
            &format!("{}.triggerType", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateVpcBindingInput {
    /// VPC实例ID
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
}

impl crate::FlatSerialize for CreateVpcBindingInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.vpc_id, &format!("{}.vpcId", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomDomain {
    /// 您的阿里云账号（主账号）ID。
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// 函数计算的API版本。
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// 权限认证配置
    #[serde(rename = "authConfig")]
    pub auth_config: AuthConfig,
    /// HTTPS证书的信息。
    #[serde(rename = "certConfig")]
    pub cert_config: CertConfig,
    /// 自定义域名的创建时间。
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// 域名。
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// 自定义域名上一次被更新的时间。
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    /// 域名支持的协议类型。HTTP：仅支持HTTP协议。HTTPS：仅支持HTTPS协议。HTTP,HTTPS：支持HTTP及HTTPS协议。
    #[serde(rename = "protocol")]
    pub protocol: String,
    /// 路由表：自定义域名访问时的PATH到Function的映射。
    #[serde(rename = "routeConfig")]
    pub route_config: RouteConfig,
    /// 子域名的数量。
    #[serde(rename = "subdomainCount")]
    pub subdomain_count: String,
    /// TLS配置信息。
    #[serde(rename = "tlsConfig")]
    pub tls_config: TLSConfig,
    /// Web应用防火墙配置信息。
    #[serde(rename = "wafConfig")]
    pub waf_config: WAFConfig,
}

impl crate::FlatSerialize for CustomDomain {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.account_id,
            &format!("{}.accountId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.api_version,
            &format!("{}.apiVersion", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.auth_config,
            &format!("{}.authConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cert_config,
            &format!("{}.certConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.created_time,
            &format!("{}.createdTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.domain_name,
            &format!("{}.domainName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.lastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.protocol, &format!("{}.protocol", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.route_config,
            &format!("{}.routeConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.subdomain_count,
            &format!("{}.subdomainCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.tls_config,
            &format!("{}.tlsConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.waf_config,
            &format!("{}.wafConfig", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for CustomDomain {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeadLetterQueue {
    /// 死信队列的 ARN
    #[serde(rename = "Arn")]
    pub arn: String,
}

impl crate::FlatSerialize for DeadLetterQueue {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.arn, &format!("{}.Arn", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeliveryOption {
    /// 上游事件源投递事件到函数计算的并发最大值，目前仅针对在 Kafka 作为事件源时有效。
    #[serde(rename = "concurrency")]
    pub concurrency: i64,
    /// 函数入口参数 event 中每个数据元素的格式。CloudEvents：以通用格式描述事件数据的规范，包含事件描述以及事件负载数据，旨在简化不同服务、平台间的事件声明和传输，默认值。RawData：只投递事件负载数据，不包含CloudEvents格式中的其它元数据信息。
    #[serde(rename = "eventSchema")]
    pub event_schema: String,
}

impl crate::FlatSerialize for DeliveryOption {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.concurrency,
            &format!("{}.concurrency", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.event_schema,
            &format!("{}.eventSchema", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsOutputRegionsRegionItem {
    /// 地域ID
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 地域名称
    #[serde(rename = "LocalName")]
    pub local_name: String,
}

impl crate::FlatSerialize for DescribeRegionsOutputRegionsRegionItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.region_id,
            &format!("{}.RegionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.local_name,
            &format!("{}.LocalName", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsOutputRegions {
    /// 地域信息集合
    #[serde(rename = "Region")]
    pub region: Vec<DescribeRegionsOutputRegionsRegionItem>,
}

impl crate::FlatSerialize for DescribeRegionsOutputRegions {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.region, &format!("{}.Region", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsOutput {
    /// 地域信息
    #[serde(rename = "Regions")]
    pub regions: DescribeRegionsOutputRegions,
}

impl crate::FlatSerialize for DescribeRegionsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.regions, &format!("{}.Regions", name), params);
    }
}

impl crate::ToCodeMessage for DescribeRegionsOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScheduledPolicy {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "target")]
    pub target: i64,
    #[serde(rename = "scheduleExpression")]
    pub schedule_expression: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

impl crate::FlatSerialize for ScheduledPolicy {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.startTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.end_time, &format!("{}.endTime", name), params);
        crate::FlatSerialize::flat_serialize(&self.target, &format!("{}.target", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.schedule_expression,
            &format!("{}.scheduleExpression", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.time_zone,
            &format!("{}.timeZone", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScalingPolicy {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "metricType")]
    pub metric_type: String,
    #[serde(rename = "metricTarget")]
    pub metric_target: f32,
    #[serde(rename = "minInstances")]
    pub min_instances: i64,
    #[serde(rename = "maxInstances")]
    pub max_instances: i64,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

impl crate::FlatSerialize for ScalingPolicy {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.startTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.end_time, &format!("{}.endTime", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.metric_type,
            &format!("{}.metricType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.metric_target,
            &format!("{}.metricTarget", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.min_instances,
            &format!("{}.minInstances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_instances,
            &format!("{}.maxInstances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.time_zone,
            &format!("{}.timeZone", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ElasticConfigStatus {
    #[serde(rename = "residentPoolId")]
    pub resident_pool_id: String,
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    #[serde(rename = "currentInstances")]
    pub current_instances: i64,
    #[serde(rename = "currentError")]
    pub current_error: String,
    #[serde(rename = "minInstances")]
    pub min_instances: i64,
    #[serde(rename = "scheduledPolicies")]
    pub scheduled_policies: Vec<ScheduledPolicy>,
    #[serde(rename = "scalingPolicies")]
    pub scaling_policies: Vec<ScalingPolicy>,
    #[serde(rename = "targetInstances")]
    pub target_instances: i64,
}

impl crate::FlatSerialize for ElasticConfigStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resident_pool_id,
            &format!("{}.residentPoolId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.function_arn,
            &format!("{}.functionArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.current_instances,
            &format!("{}.currentInstances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.current_error,
            &format!("{}.currentError", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.min_instances,
            &format!("{}.minInstances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.scheduled_policies,
            &format!("{}.scheduledPolicies", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.scaling_policies,
            &format!("{}.scalingPolicies", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.target_instances,
            &format!("{}.targetInstances", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Error {
    /// 错误码
    #[serde(rename = "Code")]
    pub code: String,
    /// 错误信息
    #[serde(rename = "Message")]
    pub message: String,
    /// 请求ID
    #[serde(rename = "RequestId")]
    pub request_id: String,
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
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventSinkConfig {
    /// 事件推送配置
    #[serde(rename = "deliveryOption")]
    pub delivery_option: DeliveryOption,
}

impl crate::FlatSerialize for EventSinkConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.delivery_option,
            &format!("{}.deliveryOption", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceDTSParameters {
    /// 数据订阅通道的网络地址及端口号信息。
    #[serde(rename = "BrokerUrl")]
    pub broker_url: String,
    /// 消费位点，即SDK客户端消费第一条数据的时间戳，格式为Unix时间戳。
    #[serde(rename = "InitCheckPoint")]
    pub init_check_point: i32,
    /// 消费组的账号密码。
    #[serde(rename = "Password")]
    pub password: String,
    /// 数据传输服务DTS版的实例所属地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 消费组ID。
    #[serde(rename = "Sid")]
    pub sid: String,
    /// 任务 ID。
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// 数据订阅通道的订阅Topic。
    #[serde(rename = "Topic")]
    pub topic: String,
    /// 消费组的账号。
    #[serde(rename = "Username")]
    pub username: String,
}

impl crate::FlatSerialize for SourceDTSParameters {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.broker_url,
            &format!("{}.BrokerUrl", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.init_check_point,
            &format!("{}.InitCheckPoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.password, &format!("{}.Password", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.region_id,
            &format!("{}.RegionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.sid, &format!("{}.Sid", name), params);
        crate::FlatSerialize::flat_serialize(&self.task_id, &format!("{}.TaskId", name), params);
        crate::FlatSerialize::flat_serialize(&self.topic, &format!("{}.Topic", name), params);
        crate::FlatSerialize::flat_serialize(&self.username, &format!("{}.Username", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceKafkaParameters {
    /// 订阅该Topic的消费者所对应的Group ID。
    #[serde(rename = "ConsumerGroup")]
    pub consumer_group: String,
    /// 消息队列Kafka版的实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 网络配置 ：默认值Default，VPC网络是PublicNetwork。
    #[serde(rename = "Network")]
    pub network: String,
    /// 偏移量。earliest：最早消费位点。latest：最新消费位点。
    #[serde(rename = "OffsetReset")]
    pub offset_reset: String,
    /// 消息队列Kafka版的实例所属地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 安全组 ID
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: String,
    /// 消息队列 Kafka 版实例下的 Topic 名称
    #[serde(rename = "Topic")]
    pub topic: String,
    /// 交换机 ID
    #[serde(rename = "VSwitchIds")]
    pub v_switch_ids: String,
    /// VPC 网络的 ID
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

impl crate::FlatSerialize for SourceKafkaParameters {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.consumer_group,
            &format!("{}.ConsumerGroup", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_id,
            &format!("{}.InstanceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.network, &format!("{}.Network", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.offset_reset,
            &format!("{}.OffsetReset", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.region_id,
            &format!("{}.RegionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.security_group_id,
            &format!("{}.SecurityGroupId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.topic, &format!("{}.Topic", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.v_switch_ids,
            &format!("{}.VSwitchIds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.vpc_id, &format!("{}.VpcId", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceMNSParameters {
    /// 是否开启Base64编码。true：开启 Base64 编码，默认值。false：不开启 Base64 编码。
    #[serde(rename = "IsBase64Decode")]
    pub is_base64_decode: bool,
    /// 轻量消息队列（原 MNS）的Queue的名称。
    #[serde(rename = "QueueName")]
    pub queue_name: String,
    /// 轻量消息队列（原 MNS）Queue所属地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
}

impl crate::FlatSerialize for SourceMNSParameters {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.is_base64_decode,
            &format!("{}.IsBase64Decode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.queue_name,
            &format!("{}.QueueName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.region_id,
            &format!("{}.RegionId", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceMQTTParameters {
    /// 消息队列MQTT版的实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 消息队列MQTT版的实例所属地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 消息队列MQTT版实例的Topic的名称。
    #[serde(rename = "Topic")]
    pub topic: String,
}

impl crate::FlatSerialize for SourceMQTTParameters {
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
        crate::FlatSerialize::flat_serialize(
            &self.region_id,
            &format!("{}.RegionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.topic, &format!("{}.Topic", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceRabbitMQParameters {
    /// 消息队列RabbitMQ版的实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 消息队列RabbitMQ版实例的Queue的名称。
    #[serde(rename = "QueueName")]
    pub queue_name: String,
    /// 消息队列RabbitMQ版的实例所属地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 消息队列RabbitMQ版实例的Vhost的名称。
    #[serde(rename = "VirtualHostName")]
    pub virtual_host_name: String,
}

impl crate::FlatSerialize for SourceRabbitMQParameters {
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
        crate::FlatSerialize::flat_serialize(
            &self.queue_name,
            &format!("{}.QueueName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.region_id,
            &format!("{}.RegionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.virtual_host_name,
            &format!("{}.VirtualHostName", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceRocketMQParameters {
    /// 认证类型。取值为 ACL 或为空。取值为 ACL 表示开启鉴权，需填写 InstanceUsername 和 InstancePassword。
    #[serde(rename = "AuthType")]
    pub auth_type: String,
    /// 消息过滤类型。
    #[serde(rename = "FilterType")]
    pub filter_type: String,
    /// 消息队列 RocketMQ 版的 Group ID。
    #[serde(rename = "GroupID")]
    pub group_id: String,
    /// 消息队列 RocketMQ 版的实例接入点信息。
    #[serde(rename = "InstanceEndpoint")]
    pub instance_endpoint: String,
    /// 消息队列 RocketMQ 版的实例 ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 网络类型。
    #[serde(rename = "InstanceNetwork")]
    pub instance_network: String,
    /// 消息队列 RocketMQ 版的实例的密码。
    #[serde(rename = "InstancePassword")]
    pub instance_password: String,
    /// 安全组 ID。
    #[serde(rename = "InstanceSecurityGroupId")]
    pub instance_security_group_id: String,
    /// 消息队列 RocketMQ 版的实例类型。
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// 消息队列 RocketMQ 版的实例的用户名。使用公网访问时，SDK 代码中需要配置实例的用户名和密码，进行身份验证。
    #[serde(rename = "InstanceUsername")]
    pub instance_username: String,
    /// 实例所关联的交换机 ID。
    #[serde(rename = "InstanceVSwitchIds")]
    pub instance_v_switch_ids: String,
    /// 实例所关联的专有网络的 ID。
    #[serde(rename = "InstanceVpcId")]
    pub instance_vpc_id: String,
    /// 消息的消费位点。
    ///
    /// - CONSUME_FROM_LAST_OFFSET：从最新位点开始消费，默认值。
    /// - CONSUME_FROM_FIRST_OFFSET：从最早位点开始消费。
    /// - CONSUME_FROM_TIMESTAMP：从指定时间点的位点开始消费。
    #[serde(rename = "Offset")]
    pub offset: String,
    /// 消息队列RocketMQ版Queue所属地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 消息的过滤标签。
    #[serde(rename = "Tag")]
    pub tag: String,
    /// 时间戳。仅当参数 Offset 取值为 CONSUME_FROM_TIMESTAMP 时，该参数有效。
    #[serde(rename = "Timestamp")]
    pub timestamp: i32,
    /// 消息队列 RocketMQ 版实例的 Topic 名称。
    #[serde(rename = "Topic")]
    pub topic: String,
}

impl crate::FlatSerialize for SourceRocketMQParameters {
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
        crate::FlatSerialize::flat_serialize(
            &self.filter_type,
            &format!("{}.FilterType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.group_id, &format!("{}.GroupID", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.instance_endpoint,
            &format!("{}.InstanceEndpoint", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_id,
            &format!("{}.InstanceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_network,
            &format!("{}.InstanceNetwork", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_password,
            &format!("{}.InstancePassword", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_security_group_id,
            &format!("{}.InstanceSecurityGroupId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_type,
            &format!("{}.InstanceType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_username,
            &format!("{}.InstanceUsername", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_v_switch_ids,
            &format!("{}.InstanceVSwitchIds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_vpc_id,
            &format!("{}.InstanceVpcId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.offset, &format!("{}.Offset", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.region_id,
            &format!("{}.RegionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.timestamp,
            &format!("{}.Timestamp", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.topic, &format!("{}.Topic", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventSourceParameters {
    /// 事件源为数据传输服务DTS时的自定义参数配置
    #[serde(rename = "sourceDTSParameters")]
    pub source_dts_parameters: SourceDTSParameters,
    /// 事件源为消息队列Kafka版时的自定义参数配置
    #[serde(rename = "sourceKafkaParameters")]
    pub source_kafka_parameters: SourceKafkaParameters,
    /// 事件源为轻量消息队列（原 MNS）时的自定义参数配置
    #[serde(rename = "sourceMNSParameters")]
    pub source_mns_parameters: SourceMNSParameters,
    /// 事件源为云消息队列MQTT版时的自定义参数配置
    #[serde(rename = "sourceMQTTParameters")]
    pub source_mqtt_parameters: SourceMQTTParameters,
    /// 事件源为消息队列RabbitMQ版时的自定义参数配置
    #[serde(rename = "sourceRabbitMQParameters")]
    pub source_rabbit_mq_parameters: SourceRabbitMQParameters,
    /// 事件源为消息队列RockerMQ版时的自定义参数配置
    #[serde(rename = "sourceRocketMQParameters")]
    pub source_rocket_mq_parameters: SourceRocketMQParameters,
}

impl crate::FlatSerialize for EventSourceParameters {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.source_dts_parameters,
            &format!("{}.sourceDTSParameters", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_kafka_parameters,
            &format!("{}.sourceKafkaParameters", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_mns_parameters,
            &format!("{}.sourceMNSParameters", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_mqtt_parameters,
            &format!("{}.sourceMQTTParameters", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_rabbit_mq_parameters,
            &format!("{}.sourceRabbitMQParameters", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_rocket_mq_parameters,
            &format!("{}.sourceRocketMQParameters", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventSourceConfig {
    /// 自定义事件源参数，自定义事件源包括：MNS、RocketMQ、RabbitMQ、Kafka、MQTT、DTS。
    #[serde(rename = "eventSourceParameters")]
    pub event_source_parameters: EventSourceParameters,
    /// 触发器事件源类型，包含如下几种类型：
    ///
    /// - **Default**：表示EventBridge 官方触发源。
    ///
    /// - **MNS**：消息队列 MNS 队列作为触发源。
    ///
    /// - **RocketMQ**：消息队列 RocketMQ 版作为触发源。
    ///
    /// - **RabbitMQ**：消息队列 RabbitMQ 版作为触发源。
    ///
    /// - **Kafka**：云消息队列 Kafka 版作为触发源。
    ///
    /// - **MQTT**：云消息队列 MQTT 版作为触发源。
    ///
    /// - **DTS**：数据传输服务 DTS 作为触发源。
    ///
    /// > 注意，该字段不可更新，更新时传入该字段将被忽略。
    #[serde(rename = "eventSourceType")]
    pub event_source_type: String,
}

impl crate::FlatSerialize for EventSourceConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.event_source_parameters,
            &format!("{}.eventSourceParameters", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.event_source_type,
            &format!("{}.eventSourceType", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RetryStrategy {
    /// 重试策略，包含如下两种策略：
    ///
    /// - **BACKOFF_RETRY**：退避重试。
    ///
    /// - **EXPONENTIAL_DECAY_RETRY**：指数衰减重试。
    #[serde(rename = "PushRetryStrategy")]
    pub push_retry_strategy: String,
}

impl crate::FlatSerialize for RetryStrategy {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.push_retry_strategy,
            &format!("{}.PushRetryStrategy", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RunOptions {
    /// 攒批窗口配置。
    #[serde(rename = "batchWindow")]
    pub batch_window: BatchWindow,
    /// 是否启用死信队列。配置该参数即为启用死信队列，默认不启用死信队列，超过重试策略后的消息将被丢弃。支持配置轻量消息队列（原 MNS）、云消息队列 RocketMQ 版、消息队列 Kafka 版以及事件总线 EventBridge。
    #[serde(rename = "deadLetterQueue")]
    pub dead_letter_queue: DeadLetterQueue,
    /// 异常容忍策略，包含如下两种策略：
    ///
    /// - **NONE**：不容忍异常，默认值。
    ///
    /// - **ALL**：容忍所有异常。
    ///
    /// > 默认策略为**NONE**
    #[serde(rename = "errorsTolerance")]
    pub errors_tolerance: String,
    /// 消息数据推送到函数计算时的底层应用模式，包含如下两种模式：
    ///
    /// - **event-streaming**：事件流模式，按照数组格式推送事件，会根据用户推送配置将一个或多个消息事件以批的形式推送到函数中进行处理，适合端到端的流式数据处理场景，该模式下支持的事件源类型有轻量消息队列（原 MNS）、RocketMQ、RabbitMQ、Kafka、MQTT 以及 DTS。
    ///
    /// - **event-driven**：事件模式，每次会将单个消息作为事件参数传入函数中，事件遵循CloudEvents规范，该模式下支持的事件源有 Default、轻量消息队列（原 MNS）、RocketMQ 以及 RabbitMQ，注意，该模式下，不支持攒批配置。
    #[serde(rename = "mode")]
    pub mode: String,
    /// 事件推送失败时的重试策略。
    #[serde(rename = "retryStrategy")]
    pub retry_strategy: RetryStrategy,
}

impl crate::FlatSerialize for RunOptions {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.batch_window,
            &format!("{}.batchWindow", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.dead_letter_queue,
            &format!("{}.deadLetterQueue", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.errors_tolerance,
            &format!("{}.errorsTolerance", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.mode, &format!("{}.mode", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.retry_strategy,
            &format!("{}.retryStrategy", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventBridgeTriggerConfig {
    /// 触发器调用函数的方式，取值如下：
    ///
    /// - **true**：同步调用。
    ///
    /// - **false**：异步调用。
    ///
    /// > 默认取值为**false**
    #[serde(rename = "asyncInvocationType")]
    pub async_invocation_type: bool,
    /// 事件模式。JSON格式，详细规则请参考[事件模式](~~181432~~)。
    #[serde(rename = "eventRuleFilterPattern")]
    pub event_rule_filter_pattern: String,
    /// 事件目标配置
    #[serde(rename = "eventSinkConfig")]
    pub event_sink_config: EventSinkConfig,
    /// 事件源配置。
    #[serde(rename = "eventSourceConfig")]
    pub event_source_config: EventSourceConfig,
    /// 运行环境参数配置
    #[serde(rename = "runOptions")]
    pub run_options: RunOptions,
    /// 是否启用触发器，取值如下：
    ///
    /// - **true**：启用触发器。
    ///
    /// - **false**：禁用触发器。
    ///
    /// > 默认取值为**true**
    #[serde(rename = "triggerEnable")]
    pub trigger_enable: bool,
}

impl crate::FlatSerialize for EventBridgeTriggerConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.async_invocation_type,
            &format!("{}.asyncInvocationType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.event_rule_filter_pattern,
            &format!("{}.eventRuleFilterPattern", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.event_sink_config,
            &format!("{}.eventSinkConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.event_source_config,
            &format!("{}.eventSourceConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.run_options,
            &format!("{}.runOptions", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_enable,
            &format!("{}.triggerEnable", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Key {
    /// 限定事件相关的资源名称，只关注以Prefix作为前缀的资源相关的事件，例如Prefix是“serverless_"，则只有以"serverless_"开始的资源相关的事件才能触发当前函数。
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// 限定事件相关的资源名称，只关注以Suffix作为后缀的资源相关的事件，例如Suffix是“.zip"，则只有以".zip"为后缀的资源相关的事件才能触发当前函数。
    #[serde(rename = "suffix")]
    pub suffix: String,
}

impl crate::FlatSerialize for Key {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.prefix, &format!("{}.prefix", name), params);
        crate::FlatSerialize::flat_serialize(&self.suffix, &format!("{}.suffix", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Filter {
    /// 事件过滤规则描述。
    #[serde(rename = "key")]
    pub key: Key,
}

impl crate::FlatSerialize for Filter {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.key", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionLayer {
    /// 层版本的资源标识。
    #[serde(rename = "arn")]
    pub arn: String,
    /// 层的代码包大小，单位为Byte。
    #[serde(rename = "size")]
    pub size: i64,
}

impl crate::FlatSerialize for FunctionLayer {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.arn, &format!("{}.arn", name), params);
        crate::FlatSerialize::flat_serialize(&self.size, &format!("{}.size", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionRestriction {
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    #[serde(rename = "disable")]
    pub disable: bool,
}

impl crate::FlatSerialize for FunctionRestriction {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.reason, &format!("{}.reason", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.lastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.disable, &format!("{}.disable", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Function {
    /// 函数代码包的CRC-64值。
    #[serde(rename = "codeChecksum")]
    pub code_checksum: String,
    /// 系统返回的函数代码包的大小，单位Byte。
    #[serde(rename = "codeSize")]
    pub code_size: i64,
    /// 函数的CPU规格，单位为vCPU，为0.05 vCPU的倍数。最小值为0.05，最大值为16。同时cpu和memorySize（按GB算）比例要在1:1和1:4之间。
    #[serde(rename = "cpu")]
    pub cpu: f32,
    /// 函数的创建时间。
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// 自定义容器运行时的相关配置，成功配置后函数可以使用自定义容器镜像执行函数。code和customContainerConfig二选一。
    #[serde(rename = "customContainerConfig")]
    pub custom_container_config: CustomContainerConfig,
    /// 自定义DNS配置。
    #[serde(rename = "customDNS")]
    pub custom_dns: CustomDNS,
    /// 自定义运行时配置。
    #[serde(rename = "customRuntimeConfig")]
    pub custom_runtime_config: CustomRuntimeConfig,
    /// 函数的描述。
    #[serde(rename = "description")]
    pub description: String,
    /// 函数的磁盘规格，单位为MB，可选值为512 MB或10240 MB。
    #[serde(rename = "diskSize")]
    pub disk_size: i32,
    /// 函数的环境变量，可以在运行环境中访问设置的环境变量。
    #[serde(rename = "environmentVariables")]
    pub environment_variables: std::collections::HashMap<String, String>,
    /// 函数资源标识。
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    /// 系统为每个函数生成的ID，全局唯一。
    #[serde(rename = "functionId")]
    pub function_id: String,
    /// 函数的名称。
    #[serde(rename = "functionName")]
    pub function_name: String,
    /// 函数GPU配置。
    #[serde(rename = "gpuConfig")]
    pub gpu_config: GPUConfig,
    /// 函数执行的入口，具体格式和运行时相关。
    #[serde(rename = "handler")]
    pub handler: String,
    /// 实例最大并发度
    #[serde(rename = "instanceConcurrency")]
    pub instance_concurrency: i32,
    /// 实例生命周期回调方法配置。
    #[serde(rename = "instanceLifecycleConfig")]
    pub instance_lifecycle_config: InstanceLifecycleConfig,
    /// 是否允许函数访问公网。默认值为true。
    #[serde(rename = "internetAccess")]
    pub internet_access: bool,
    /// 函数上一次被更新的时间。
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    /// 最近一次函数更新操作的状态，当函数新建成功时，此值为Successful，可选值：Successful、 Failed、 InProgress。
    #[serde(rename = "lastUpdateStatus")]
    pub last_update_status: String,
    /// 导致最近一次函数更新操作状态为当前值的原因。
    #[serde(rename = "lastUpdateStatusReason")]
    pub last_update_status_reason: String,
    /// 导致最近一次函数更新操作状态为当前值的原因的状态码。
    #[serde(rename = "lastUpdateStatusReasonCode")]
    pub last_update_status_reason_code: String,
    /// 层的列表。
    #[serde(rename = "layers")]
    pub layers: Vec<FunctionLayer>,
    /// 日志配置。函数产生的日志会被写入到配置的日志库中。
    #[serde(rename = "logConfig")]
    pub log_config: LogConfig,
    /// 函数的内存规格，单位为MB，内存大小为64 MB的倍数。最小值为128MB，最大值为32GB。同时cpu和memorySize（按GB算）比例要在1:1和1:4之间。
    #[serde(rename = "memorySize")]
    pub memory_size: i32,
    /// NAS配置。配置此参数后，函数可以访问指定的NAS资源。
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
    /// OSS挂载配置。
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: OSSMountConfig,
    /// 用户授权给函数计算的RAM角色，设置后函数计算将扮演该角色生成临时访问凭证。在函数中可以使用该角色的临时访问凭证来访问指定的阿里云服务，例如OSS和OTS。
    #[serde(rename = "role")]
    pub role: String,
    /// 函数的运行时环境。目前支持的运行环境有：nodejs12, nodejs14, nodejs16, nodejs18, nodejs20, go1, python3, python3.9, python3.10, python3.12, java8, java11, php7.2, dotnetcore3.1, custom, custom.debian10, custom.debian11, custom.debian12, custom-container。
    #[serde(rename = "runtime")]
    pub runtime: String,
    /// 函数当前的状态。
    #[serde(rename = "state")]
    pub state: String,
    /// 函数处于当前状态的原因。
    #[serde(rename = "stateReason")]
    pub state_reason: String,
    /// 函数处于当前状态的原因的状态码。
    #[serde(rename = "stateReasonCode")]
    pub state_reason_code: String,
    /// 函数运行的超时时间，单位为秒，最小1秒，最大值为86400秒，默认值是3秒。函数超过这个时间后会被终止执行。
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// 链路追踪配置。当函数计算与链路追踪集成后，您可以记录请求在函数计算的耗时时间、查看函数的冷启动时间、记录函数内部时间的消耗等。
    #[serde(rename = "tracingConfig")]
    pub tracing_config: TracingConfig,
    /// VPC配置。配置此参数后，函数可以访问指定的VPC资源。
    #[serde(rename = "vpcConfig")]
    pub vpc_config: VPCConfig,
    /// 标签列表
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
    /// 是否禁止创建按量实例，功能开启后，不会创建按量实例，只能使用预留实例
    #[serde(rename = "disableOndemand")]
    pub disable_ondemand: bool,
    #[serde(rename = "invocationRestriction")]
    pub invocation_restriction: FunctionRestriction,
    /// 函数计算调用请求的亲和策略，如需实现 MCP SSE协议的请求亲和，可设置为 MCP_SSE。如使用Cookie亲和，可设置为 GENERATED_COOKIE。如使用 Header亲和，可设置为 HEADER_FIELD。如不设置或设置为 NONE，则无亲和效果，按函数计算系统默认调度策略路由请求。
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: String,
    /// 当设置sessionAffinity亲和类型时，需设置相关的亲和配置。如MCP_SSE亲和需填充 MCPSSESessionAffinityConfig 配置。Cookie亲和需填充CookieSessionAffinityConfig配置，Header Field 亲和需填充HeaderFieldSessionAffinityConfig配置。
    #[serde(rename = "enableLongLiving")]
    pub enable_long_living: bool,
    /// 资源组 ID
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: String,
    /// 实例隔离模式
    #[serde(rename = "instanceIsolationMode")]
    pub instance_isolation_mode: FunctionInstanceIsolationMode,
    /// 当设置sessionAffinity亲和类型时，需设置相关的亲和配置。如MCP_SSE亲和需填充 MCPSSESessionAffinityConfig 配置。Cookie亲和需填充CookieSessionAffinityConfig配置，Header Field 亲和需填充HeaderFieldSessionAffinityConfig配置。
    #[serde(rename = "sessionAffinityConfig")]
    pub session_affinity_config: String,
    /// 实例延迟释放时间
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: i32,
    /// 是否不注入 STS token，取值None/Env/Request/All
    /// None: 都注入
    /// Env: 环境变量不注入
    /// Request: 请求中不注入包括context/header
    /// All: 都不注入
    #[serde(rename = "disableInjectCredentials")]
    pub disable_inject_credentials: FunctionDisableInjectCredentials,
    /// PolarFs配置。配置此参数后，函数可以访问指定的PolarFs资源。
    #[serde(rename = "polarFsConfig")]
    pub polar_fs_config: PolarFsConfig,
}

impl crate::FlatSerialize for Function {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.code_checksum,
            &format!("{}.codeChecksum", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.code_size,
            &format!("{}.codeSize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.cpu, &format!("{}.cpu", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.created_time,
            &format!("{}.createdTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.custom_container_config,
            &format!("{}.customContainerConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.custom_dns,
            &format!("{}.customDNS", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.custom_runtime_config,
            &format!("{}.customRuntimeConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disk_size,
            &format!("{}.diskSize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.environment_variables,
            &format!("{}.environmentVariables", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.function_arn,
            &format!("{}.functionArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.function_id,
            &format!("{}.functionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.function_name,
            &format!("{}.functionName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.gpu_config,
            &format!("{}.gpuConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.handler, &format!("{}.handler", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.instance_concurrency,
            &format!("{}.instanceConcurrency", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_lifecycle_config,
            &format!("{}.instanceLifecycleConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.internet_access,
            &format!("{}.internetAccess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.lastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_update_status,
            &format!("{}.lastUpdateStatus", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_update_status_reason,
            &format!("{}.lastUpdateStatusReason", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_update_status_reason_code,
            &format!("{}.lastUpdateStatusReasonCode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.layers, &format!("{}.layers", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.log_config,
            &format!("{}.logConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.memory_size,
            &format!("{}.memorySize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.nas_config,
            &format!("{}.nasConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_mount_config,
            &format!("{}.ossMountConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.role, &format!("{}.role", name), params);
        crate::FlatSerialize::flat_serialize(&self.runtime, &format!("{}.runtime", name), params);
        crate::FlatSerialize::flat_serialize(&self.state, &format!("{}.state", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.state_reason,
            &format!("{}.stateReason", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.state_reason_code,
            &format!("{}.stateReasonCode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.timeout, &format!("{}.timeout", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.tracing_config,
            &format!("{}.tracingConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.vpc_config,
            &format!("{}.vpcConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tags, &format!("{}.tags", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.disable_ondemand,
            &format!("{}.disableOndemand", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.invocation_restriction,
            &format!("{}.invocationRestriction", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_affinity,
            &format!("{}.sessionAffinity", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_long_living,
            &format!("{}.enableLongLiving", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_group_id,
            &format!("{}.resourceGroupId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_isolation_mode,
            &format!("{}.instanceIsolationMode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_affinity_config,
            &format!("{}.sessionAffinityConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.idle_timeout,
            &format!("{}.idleTimeout", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_inject_credentials,
            &format!("{}.disableInjectCredentials", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.polar_fs_config,
            &format!("{}.polarFsConfig", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for Function {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InstanceEventItem {
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "children")]
    pub children: Vec<String>,
    #[serde(rename = "time")]
    pub time: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl crate::FlatSerialize for InstanceEventItem {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.level, &format!("{}.level", name), params);
        crate::FlatSerialize::flat_serialize(&self.children, &format!("{}.children", name), params);
        crate::FlatSerialize::flat_serialize(&self.time, &format!("{}.time", name), params);
        crate::FlatSerialize::flat_serialize(&self.message, &format!("{}.message", name), params);
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.type", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetInstanceLifecycleEventsOutput {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "events")]
    pub events: Vec<InstanceEventItem>,
}

impl crate::FlatSerialize for GetInstanceLifecycleEventsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.request_id,
            &format!("{}.requestId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.events, &format!("{}.events", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GetResourceTagsOutput {
    /// 资源类型名称。函数计算3.0中的函数：ALIYUN::FC::FUNCTION,函数计算旧版本中的服务：ALIYUN::FC::SERVICE。
    #[serde(rename = "resouceType")]
    pub resouce_type: String,
    /// 阿里云资源描述符。
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// 标签字典。
    #[serde(rename = "tags")]
    pub tags: std::collections::HashMap<String, String>,
}

impl crate::FlatSerialize for GetResourceTagsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resouce_type,
            &format!("{}.resouceType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_arn,
            &format!("{}.resourceArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tags, &format!("{}.tags", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HTTPTrigger {
    /// 公网域名地址。在互联网可以通过HTTP协议或者HTTPS协议访问HTTP Trigger。
    #[serde(rename = "urlInternet")]
    pub url_internet: String,
    /// 私网域名地址。在VPC可以通过HTTP协议或者HTTPS协议访问HTTP Trigger。
    #[serde(rename = "urlIntranet")]
    pub url_intranet: String,
}

impl crate::FlatSerialize for HTTPTrigger {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.url_internet,
            &format!("{}.urlInternet", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.url_intranet,
            &format!("{}.urlIntranet", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HTTPTriggerConfig {
    /// 鉴权配置
    #[serde(rename = "authConfig")]
    pub auth_config: String,
    /// 认证类型，包含如下两种类型：
    ///
    /// - **function**：表示需要认证。
    ///
    /// - **anonymous**：表示不需要认证。
    ///
    /// > 默认类型为 **function**
    #[serde(rename = "authType")]
    pub auth_type: String,
    /// 禁用默认公网域名访问的开关。设置为true时，访问函数默认提供的公网URL地址会返回403错误。设置为false时，不会有任何影响。
    #[serde(rename = "disableURLInternet")]
    pub disable_url_internet: bool,
    /// 请求方法列表。允许同时支持多种方法。
    #[serde(rename = "methods")]
    pub methods: Vec<String>,
}

impl crate::FlatSerialize for HTTPTriggerConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.auth_config,
            &format!("{}.authConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.auth_type,
            &format!("{}.authType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_url_internet,
            &format!("{}.disableURLInternet", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.methods, &format!("{}.methods", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HeaderFieldSessionAffinityConfig {
    /// 用户在一段时间内没有进行任何操作，导致会话进入空闲状态，最大时长为单个 Session 生命周期上限，取值范围为[0, 21600]。
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    /// 指从 Session 创建、使用到最终销毁的全过程。 超过生命周期，函数计算将会自动销毁Session， 不再保证亲和性，取值范围为[1, 21600]。
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    /// 单实例在同一个时间内能同时处理的最大 Session 数，取值范围为[1, 200]。
    #[serde(rename = "sessionConcurrencyPerInstance")]
    pub session_concurrency_per_instance: i64,
    /// 通过 HTTP 请求头传递客户端会话标识，不能以 x-fc- 前缀开头，以字母开头，非首字符可包含数字、中划线、下划线、字母，长度大于等于5个字符并且不超过40个字符。
    #[serde(rename = "affinityHeaderFieldName")]
    pub affinity_header_field_name: String,
    /// 默认值 False，表示在 SessionID 会话过期后，可携带相同SessionID继续发起请求，系统将视为新会话绑定新实例。当配置为 True，表示在 SessionID 会话过期后，不可复用 SessionID。
    #[serde(rename = "disableSessionIdReuse")]
    pub disable_session_id_reuse: bool,
}

impl crate::FlatSerialize for HeaderFieldSessionAffinityConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.session_idle_timeout_in_seconds,
            &format!("{}.sessionIdleTimeoutInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_ttl_in_seconds,
            &format!("{}.sessionTTLInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_concurrency_per_instance,
            &format!("{}.sessionConcurrencyPerInstance", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.affinity_header_field_name,
            &format!("{}.affinityHeaderFieldName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_session_id_reuse,
            &format!("{}.disableSessionIdReuse", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InstanceInfo {
    /// 实例ID。
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// 实例所属函数的版本。如果是LATEST别名下的函数实例，则返回版本号为0。
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "createdTimeMs")]
    pub created_time_ms: i64,
    #[serde(rename = "destroyedTimeMs")]
    pub destroyed_time_ms: i64,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

impl crate::FlatSerialize for InstanceInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.instance_id,
            &format!("{}.instanceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.version_id,
            &format!("{}.versionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qualifier,
            &format!("{}.qualifier", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.created_time_ms,
            &format!("{}.createdTimeMs", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.destroyed_time_ms,
            &format!("{}.destroyedTimeMs", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_type,
            &format!("{}.resourceType", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JobConfig {
    /// 最大重试次数。取值范围为0～100。日志服务根据触发间隔每次触发函数执行时，如果遇到错误（例如权限不足、网络失败、函数执行异常返回等），该参数定义本次触发所允许的最大重试次数。对于本次触发，如果超过最大重试次数仍无法成功的，需要等到下一次触发间隔到来时，由日志服务再次触发函数执行。
    #[serde(rename = "maxRetryTime")]
    pub max_retry_time: i32,
    /// 日志服务触发函数运行的间隔。比如每隔120秒将logstore在最近120秒内的数据取出到函数服务，以执行自定义计算。
    #[serde(rename = "triggerInterval")]
    pub trigger_interval: i32,
}

impl crate::FlatSerialize for JobConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.max_retry_time,
            &format!("{}.maxRetryTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_interval,
            &format!("{}.triggerInterval", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OutputCodeLocation {
    /// 代码包的地址。
    #[serde(rename = "location")]
    pub location: String,
    /// 代码包的类型。
    #[serde(rename = "repositoryType")]
    pub repository_type: String,
}

impl crate::FlatSerialize for OutputCodeLocation {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.location, &format!("{}.location", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.repository_type,
            &format!("{}.repositoryType", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Layer {
    /// 层的权限。取值0代表私有，取值1代表公有。官方公共层默认为公有，自定义层可以设置为私有或者公有。
    #[serde(rename = "acl")]
    pub acl: String,
    /// 层的代码包信息
    #[serde(rename = "code")]
    pub code: OutputCodeLocation,
    /// 层代码包的crc64校验码，根据ECMA-182标准计算得出。
    #[serde(rename = "codeChecksum")]
    pub code_checksum: String,
    /// 层的代码包大小，单位为Byte。
    #[serde(rename = "codeSize")]
    pub code_size: i64,
    /// 层支持的运行时环境列表。
    #[serde(rename = "compatibleRuntime")]
    pub compatible_runtime: Vec<String>,
    /// 层版本的创建时间。
    #[serde(rename = "createTime")]
    pub create_time: String,
    /// 版本的描述信息。
    #[serde(rename = "description")]
    pub description: String,
    /// 层的名称。
    #[serde(rename = "layerName")]
    pub layer_name: String,
    /// 层版本资源的名称，格式为 acs:fc:{region}:{accountID}:layers/{layerName}/versions/{layerVersion}.
    #[serde(rename = "layerVersionArn")]
    pub layer_version_arn: String,
    /// 许可协议。
    #[serde(rename = "license")]
    pub license: String,
    /// 层的版本。
    #[serde(rename = "version")]
    pub version: i32,
}

impl crate::FlatSerialize for Layer {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.acl, &format!("{}.acl", name), params);
        crate::FlatSerialize::flat_serialize(&self.code, &format!("{}.code", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.code_checksum,
            &format!("{}.codeChecksum", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.code_size,
            &format!("{}.codeSize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.compatible_runtime,
            &format!("{}.compatibleRuntime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.create_time,
            &format!("{}.createTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.layer_name,
            &format!("{}.layerName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.layer_version_arn,
            &format!("{}.layerVersionArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.license, &format!("{}.license", name), params);
        crate::FlatSerialize::flat_serialize(&self.version, &format!("{}.version", name), params);
    }
}

impl crate::ToCodeMessage for Layer {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAliasesOutput {
    /// 别名列表
    #[serde(rename = "aliases")]
    pub aliases: Vec<Alias>,
    /// 下一个版本名
    #[serde(rename = "nextToken")]
    pub next_token: String,
}

impl crate::FlatSerialize for ListAliasesOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.aliases, &format!("{}.aliases", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListAliasesOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAsyncInvokeConfigOutput {
    /// 异步调用配置列表。
    #[serde(rename = "configs")]
    pub configs: Vec<AsyncConfig>,
    /// 用来返回更多结果。第一次查询不需要提供这个参数，后续查询的Token从返回结果中获取。
    #[serde(rename = "nextToken")]
    pub next_token: String,
}

impl crate::FlatSerialize for ListAsyncInvokeConfigOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.configs, &format!("{}.configs", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListAsyncInvokeConfigOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListAsyncTaskOutput {
    /// 分页标记，用来返回更多结果。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: String,
    /// 异步任务信息列表。
    #[serde(rename = "tasks")]
    pub tasks: Vec<AsyncTask>,
}

impl crate::FlatSerialize for ListAsyncTaskOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tasks, &format!("{}.tasks", name), params);
    }
}

impl crate::ToCodeMessage for ListAsyncTaskOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListConcurrencyConfigsOutput {
    /// 并发配置列表。
    #[serde(rename = "configs")]
    pub configs: Vec<ConcurrencyConfig>,
    /// 用来返回更多的查询结果。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: String,
}

impl crate::FlatSerialize for ListConcurrencyConfigsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.configs, &format!("{}.configs", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListConcurrencyConfigsOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListCustomDomainOutput {
    /// 自定义域名列表
    #[serde(rename = "customDomains")]
    pub custom_domains: Vec<CustomDomain>,
    /// 当符合查询条件的数据未读取完时，服务端会返回nextToken，此时可以使用nextToken继续读取后面的数据。第一次查询不需要提供这个参数。
    #[serde(rename = "nextToken")]
    pub next_token: String,
}

impl crate::FlatSerialize for ListCustomDomainOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.custom_domains,
            &format!("{}.customDomains", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListCustomDomainOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListElasticConfigsOutput {
    #[serde(rename = "nextToken")]
    pub next_token: String,
    #[serde(rename = "elasticConfigs")]
    pub elastic_configs: Vec<ElasticConfigStatus>,
}

impl crate::FlatSerialize for ListElasticConfigsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.elastic_configs,
            &format!("{}.elasticConfigs", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListFunctionsOutput {
    /// 函数信息列表
    #[serde(rename = "functions")]
    pub functions: Vec<Function>,
    /// 用来返回更多的查询结果。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: String,
}

impl crate::FlatSerialize for ListFunctionsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.functions,
            &format!("{}.functions", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListFunctionsOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListInstancesOutput {
    /// 实例列表信息
    #[serde(rename = "instances")]
    pub instances: Vec<InstanceInfo>,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

impl crate::FlatSerialize for ListInstancesOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.instances,
            &format!("{}.instances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.request_id,
            &format!("{}.requestId", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListInstancesOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListLayerVersionOutput {
    /// 层版本的列表。
    #[serde(rename = "layers")]
    pub layers: Vec<Layer>,
    /// 剩余列表的起始版本名，用来返回更多结果。
    #[serde(rename = "nextVersion")]
    pub next_version: i32,
}

impl crate::FlatSerialize for ListLayerVersionOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.layers, &format!("{}.layers", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_version,
            &format!("{}.nextVersion", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListLayerVersionOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListLayersOutput {
    /// 层的列表。
    #[serde(rename = "layers")]
    pub layers: Vec<Layer>,
    /// 剩余列表的起始层名，用来返回更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: String,
}

impl crate::FlatSerialize for ListLayersOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.layers, &format!("{}.layers", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListLayersOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScheduledAction {
    /// 策略失效时间。
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// 策略名称。
    #[serde(rename = "name")]
    pub name: String,
    /// 定时配置。
    #[serde(rename = "scheduleExpression")]
    pub schedule_expression: String,
    /// 策略生效时间。
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// 预留的目标资源个数。
    #[serde(rename = "target")]
    pub target: i64,
    /// 时区。时区参数为空时，startTime、endTime和scheduleExpression的时间需为UTC格式。
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

impl crate::FlatSerialize for ScheduledAction {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.end_time, &format!("{}.endTime", name), params);
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.schedule_expression,
            &format!("{}.scheduleExpression", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.startTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.target, &format!("{}.target", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.time_zone,
            &format!("{}.timeZone", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TargetTrackingPolicy {
    /// 策略结束时间（UTC）。
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// 扩容的最大值。
    #[serde(rename = "maxCapacity")]
    pub max_capacity: i64,
    /// 指标的追踪值。
    #[serde(rename = "metricTarget")]
    pub metric_target: f32,
    /// 跟踪的指标类型：ProvisionedConcurrencyUtilization：预留模式实例并发度利用率。CPUUtilization：CPU利用率。GPUMemUtilization：GPU利用率。
    #[serde(rename = "metricType")]
    pub metric_type: String,
    /// 缩容的最小值。
    #[serde(rename = "minCapacity")]
    pub min_capacity: i64,
    /// 策略名称。
    #[serde(rename = "name")]
    pub name: String,
    /// 策略开始生效时间（UTC）。
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// 时区。时区参数为空时，startTime和endTime的时间需为UTC格式。
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

impl crate::FlatSerialize for TargetTrackingPolicy {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.end_time, &format!("{}.endTime", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.max_capacity,
            &format!("{}.maxCapacity", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.metric_target,
            &format!("{}.metricTarget", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.metric_type,
            &format!("{}.metricType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.min_capacity,
            &format!("{}.minCapacity", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.startTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.time_zone,
            &format!("{}.timeZone", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvisionConfig {
    /// 定时策略配置。
    #[serde(rename = "scheduledActions")]
    pub scheduled_actions: Vec<ScheduledAction>,
    /// 预留实例创建失败时的错误信息。
    #[serde(rename = "currentError")]
    pub current_error: String,
    /// 所有指标追踪伸缩策略和定时策略失效时的默认资源个数。
    #[serde(rename = "defaultTarget")]
    pub default_target: i64,
    /// 实际资源个数。
    #[serde(rename = "current")]
    pub current: i64,
    /// 是否始终分配CPU给函数实例。
    #[serde(rename = "alwaysAllocateCPU")]
    pub always_allocate_cpu: bool,
    /// 是否始终分配GPU给函数实例。
    #[serde(rename = "alwaysAllocateGPU")]
    pub always_allocate_gpu: bool,
    /// 指标追踪伸缩策略配置。
    #[serde(rename = "targetTrackingPolicies")]
    pub target_tracking_policies: Vec<TargetTrackingPolicy>,
    /// 函数的资源描述
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    /// 当前目标资源个数，如果存在指标追踪伸缩策略或定时策略，为策略计算的资源个数，否则为默认预留实例数。
    ///
    ///
    /// > 与 defaultTarget 有什么区别？\
    /// > 假设配置预留实例数为1后，新增了定时伸缩策略，设置某个时间段内的预留实例数为5。
    /// > - 在定时伸缩策略**生效期间**，target 与 defaultTarget 分别为 5 和 1。
    /// >-  在定时伸缩策略**失效期间**，target 与 defaultTarget 都为 1。
    #[serde(rename = "target")]
    pub target: i64,
}

impl crate::FlatSerialize for ProvisionConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.scheduled_actions,
            &format!("{}.scheduledActions", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.current_error,
            &format!("{}.currentError", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.default_target,
            &format!("{}.defaultTarget", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.current, &format!("{}.current", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.always_allocate_cpu,
            &format!("{}.alwaysAllocateCPU", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.always_allocate_gpu,
            &format!("{}.alwaysAllocateGPU", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.target_tracking_policies,
            &format!("{}.targetTrackingPolicies", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.function_arn,
            &format!("{}.functionArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.target, &format!("{}.target", name), params);
    }
}

impl crate::ToCodeMessage for ProvisionConfig {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListProvisionConfigsOutput {
    /// 下次查询的起始Token。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: String,
    /// 函数预留配置列表。
    #[serde(rename = "provisionConfigs")]
    pub provision_configs: Vec<ProvisionConfig>,
}

impl crate::FlatSerialize for ListProvisionConfigsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.provision_configs,
            &format!("{}.provisionConfigs", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListProvisionConfigsOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResidentResourceCapacity {
    /// GPU总卡数
    #[serde(rename = "totalGpuCards")]
    pub total_gpu_cards: i64,
    /// 总内存大小，单位 GB
    #[serde(rename = "totalMemorySize")]
    pub total_memory_size: i64,
    /// 总磁盘大小，单位 GB
    #[serde(rename = "totalDiskSize")]
    pub total_disk_size: i64,
    /// 总显存大小，单位 GB
    #[serde(rename = "totalGpuMemorySize")]
    pub total_gpu_memory_size: i64,
    /// CPU 总核数
    #[serde(rename = "totalCpuCores")]
    pub total_cpu_cores: i64,
    /// GPU 卡型
    #[serde(rename = "gpuType")]
    pub gpu_type: String,
}

impl crate::FlatSerialize for ResidentResourceCapacity {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.total_gpu_cards,
            &format!("{}.totalGpuCards", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_memory_size,
            &format!("{}.totalMemorySize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_disk_size,
            &format!("{}.totalDiskSize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_gpu_memory_size,
            &format!("{}.totalGpuMemorySize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_cpu_cores,
            &format!("{}.totalCpuCores", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.gpu_type, &format!("{}.gpuType", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResidentResourceAllocation {
    /// 使用该资源池的函数名
    #[serde(rename = "functionName")]
    pub function_name: String,
    /// 函数的别名
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    /// 实例数
    #[serde(rename = "instanceCount")]
    pub instance_count: i32,
    /// CPU 占用总核数
    #[serde(rename = "totalCpuCores")]
    pub total_cpu_cores: f64,
    /// 内存占用大小，单位 GB
    #[serde(rename = "totalMemorySize")]
    pub total_memory_size: f64,
    /// 占用显存大小，单位 GB
    #[serde(rename = "totalGpuMemorySize")]
    pub total_gpu_memory_size: f64,
    /// 占用磁盘大小，单位 GB
    #[serde(rename = "totalDiskSize")]
    pub total_disk_size: f64,
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}

impl crate::FlatSerialize for ResidentResourceAllocation {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.function_name,
            &format!("{}.functionName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qualifier,
            &format!("{}.qualifier", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_count,
            &format!("{}.instanceCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_cpu_cores,
            &format!("{}.totalCpuCores", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_memory_size,
            &format!("{}.totalMemorySize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_gpu_memory_size,
            &format!("{}.totalGpuMemorySize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.total_disk_size,
            &format!("{}.totalDiskSize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_type,
            &format!("{}.instanceType", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResidentResourceAllocationStatus {
    #[serde(rename = "lastAllocatedTime")]
    pub last_allocated_time: String,
    #[serde(rename = "lastAllocation")]
    pub last_allocation: Vec<ResidentResourceAllocation>,
}

impl crate::FlatSerialize for ResidentResourceAllocationStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.last_allocated_time,
            &format!("{}.lastAllocatedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_allocation,
            &format!("{}.lastAllocation", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResidentResourcePool {
    /// 代表资源名称的资源属性字段
    #[serde(rename = "residentResourcePoolName")]
    pub resident_resource_pool_name: String,
    /// 上次修改时间，包含扩容、续费、更名等操作
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    /// 资源池过期时间
    #[serde(rename = "expireTime")]
    pub expire_time: String,
    #[serde(rename = "resourcePoolConfig")]
    pub resource_pool_config: ResidentResourceCapacity,
    #[serde(rename = "residentResourcePoolId")]
    pub resident_resource_pool_id: String,
    /// 资源池总体规格
    #[serde(rename = "resourcePoolCapacity")]
    pub resource_pool_capacity: ResidentResourceCapacity,
    /// 代表创建时间的资源属性字段
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// 资源池实时分配情况，包含每个函数的具体分配情况
    #[serde(rename = "allocationStatus")]
    pub allocation_status: ResidentResourceAllocationStatus,
}

impl crate::FlatSerialize for ResidentResourcePool {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resident_resource_pool_name,
            &format!("{}.residentResourcePoolName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.lastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.expire_time,
            &format!("{}.expireTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_pool_config,
            &format!("{}.resourcePoolConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resident_resource_pool_id,
            &format!("{}.residentResourcePoolId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_pool_capacity,
            &format!("{}.resourcePoolCapacity", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.created_time,
            &format!("{}.createdTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.allocation_status,
            &format!("{}.allocationStatus", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListResidentResourcePoolsOutput {
    #[serde(rename = "nextToken")]
    pub next_token: String,
    #[serde(rename = "residentResourcePools")]
    pub resident_resource_pools: Vec<ResidentResourcePool>,
}

impl crate::FlatSerialize for ListResidentResourcePoolsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resident_resource_pools,
            &format!("{}.residentResourcePools", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScalingConfigStatus {
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    #[serde(rename = "residentPoolId")]
    pub resident_pool_id: String,
    #[serde(rename = "minInstances")]
    pub min_instances: i64,
    #[serde(rename = "currentInstances")]
    pub current_instances: i64,
    #[serde(rename = "currentError")]
    pub current_error: String,
    #[serde(rename = "targetInstances")]
    pub target_instances: i64,
    #[serde(rename = "enableOnDemandScaling")]
    pub enable_on_demand_scaling: bool,
    #[serde(rename = "scheduledPolicies")]
    pub scheduled_policies: Vec<ScheduledPolicy>,
    #[serde(rename = "horizontalScalingPolicies")]
    pub horizontal_scaling_policies: Vec<ScalingPolicy>,
    #[serde(rename = "enableMixMode")]
    pub enable_mix_mode: bool,
    #[serde(rename = "requestDispatchPolicy")]
    pub request_dispatch_policy: String,
}

impl crate::FlatSerialize for ScalingConfigStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.function_arn,
            &format!("{}.functionArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resident_pool_id,
            &format!("{}.residentPoolId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.min_instances,
            &format!("{}.minInstances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.current_instances,
            &format!("{}.currentInstances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.current_error,
            &format!("{}.currentError", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.target_instances,
            &format!("{}.targetInstances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_on_demand_scaling,
            &format!("{}.enableOnDemandScaling", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.scheduled_policies,
            &format!("{}.scheduledPolicies", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.horizontal_scaling_policies,
            &format!("{}.horizontalScalingPolicies", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_mix_mode,
            &format!("{}.enableMixMode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.request_dispatch_policy,
            &format!("{}.requestDispatchPolicy", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ScalingConfigStatus {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListScalingConfigsOutput {
    #[serde(rename = "nextToken")]
    pub next_token: String,
    #[serde(rename = "scalingConfigs")]
    pub scaling_configs: Vec<ScalingConfigStatus>,
}

impl crate::FlatSerialize for ListScalingConfigsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.scaling_configs,
            &format!("{}.scalingConfigs", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListScalingConfigsOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Session {
    /// 函数会话唯一标识
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// Session所属函数名称
    #[serde(rename = "functionName")]
    pub function_name: String,
    /// 会话亲和类型
    #[serde(rename = "sessionAffinityType")]
    pub session_affinity_type: String,
    /// Session 生命周期最大值
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    /// Session 闲置过期时间
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    /// Session 的创建时间
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// Session上一次被更新的时间。
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    /// Session 状态：Active 代表 Session 有效，Expired代表 Session已过期
    #[serde(rename = "sessionStatus")]
    pub session_status: String,
    /// Session关联的函数实例ID
    #[serde(rename = "containerId")]
    pub container_id: String,
    /// 客户创建 Session 时传入的 Qualifier，如未传则为默认值 LATEST
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    /// NAS配置，配置后Session关联的实例可以访问指定NAS资源。
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
    /// 默认值 False，表示在 SessionID 会话过期后，可携带相同SessionID继续发起请求，系统将视为新会话绑定新实例。当配置为 True，表示在 SessionID 会话过期后，不可复用 SessionID。
    #[serde(rename = "disableSessionIdReuse")]
    pub disable_session_id_reuse: bool,
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: OSSMountConfig,
    #[serde(rename = "polarFsConfig")]
    pub polar_fs_config: PolarFsConfig,
}

impl crate::FlatSerialize for Session {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.session_id,
            &format!("{}.sessionId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.function_name,
            &format!("{}.functionName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_affinity_type,
            &format!("{}.sessionAffinityType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_ttl_in_seconds,
            &format!("{}.sessionTTLInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_idle_timeout_in_seconds,
            &format!("{}.sessionIdleTimeoutInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.created_time,
            &format!("{}.createdTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.lastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_status,
            &format!("{}.sessionStatus", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.container_id,
            &format!("{}.containerId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qualifier,
            &format!("{}.qualifier", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.nas_config,
            &format!("{}.nasConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_session_id_reuse,
            &format!("{}.disableSessionIdReuse", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_mount_config,
            &format!("{}.ossMountConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.polar_fs_config,
            &format!("{}.polarFsConfig", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for Session {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListSessionsOutput {
    /// 下一次查询会话列表的起始位置。
    #[serde(rename = "nextToken")]
    pub next_token: String,
    /// 会话列表信息
    #[serde(rename = "sessions")]
    pub sessions: Vec<Session>,
}

impl crate::FlatSerialize for ListSessionsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.sessions, &format!("{}.sessions", name), params);
    }
}

impl crate::ToCodeMessage for ListSessionsOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TagResource {
    /// 阿里云资源描述符。
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// 资源类型名称。
    ///
    /// 函数计算3.0中的函数类型：ALIYUN::FC::FUNCTION，简写：function。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 标签键。
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    /// 标签值。
    #[serde(rename = "TagValue")]
    pub tag_value: String,
}

impl crate::FlatSerialize for TagResource {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resource_id,
            &format!("{}.ResourceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_type,
            &format!("{}.ResourceType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tag_key, &format!("{}.TagKey", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.tag_value,
            &format!("{}.TagValue", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListTagResourcesOutput {
    /// 用来返回更多结果。第一次查询不需要提供这个参数，后续查询的Token从返回结果中获取。
    #[serde(rename = "NextToken")]
    pub next_token: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
    /// 查询到的资源和标签的信息。
    #[serde(rename = "TagResources")]
    pub tag_resources: Vec<TagResource>,
}

impl crate::FlatSerialize for ListTagResourcesOutput {
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
            &self.request_id,
            &format!("{}.RequestId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.tag_resources,
            &format!("{}.TagResources", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for ListTagResourcesOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Resource {
    /// 资源类型名称。函数计算3.0中的函数：ALIYUN::FC::FUNCTION,函数计算旧版本中的服务：ALIYUN::FC::SERVICE。
    #[serde(rename = "resouceType")]
    pub resouce_type: String,
    /// 阿里云资源描述符。
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// 标签字典。
    #[serde(rename = "tags")]
    pub tags: std::collections::HashMap<String, String>,
}

impl crate::FlatSerialize for Resource {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resouce_type,
            &format!("{}.resouceType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_arn,
            &format!("{}.resourceArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tags, &format!("{}.tags", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListTaggedResourcesOutput {
    /// 用来返回更多结果。第一次查询不需要提供这个参数，后续查询的Token从返回结果中获取。
    #[serde(rename = "nextToken")]
    pub next_token: String,
    /// 被打标签的资源列表。
    #[serde(rename = "resources")]
    pub resources: Vec<Resource>,
}

impl crate::FlatSerialize for ListTaggedResourcesOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resources,
            &format!("{}.resources", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Trigger {
    /// 触发器的创建时间。
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// 触发器的描述。
    #[serde(rename = "description")]
    pub description: String,
    /// HTTP 触发器信息。
    #[serde(rename = "httpTrigger")]
    pub http_trigger: HTTPTrigger,
    /// 事件源（如OSS）调用函数所需的角色。
    #[serde(rename = "invocationRole")]
    pub invocation_role: String,
    /// 触发器的上次修改时间。
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    /// 函数的版本或别名。
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    /// 触发器事件源的Aliyun Resource Name。
    #[serde(rename = "sourceArn")]
    pub source_arn: String,
    /// 触发器的状态。
    #[serde(rename = "status")]
    pub status: String,
    /// 函数的资源标识。
    #[serde(rename = "targetArn")]
    pub target_arn: String,
    /// 触发器配置，针对不同类型的触发器，配置有所不同。
    #[serde(rename = "triggerConfig")]
    pub trigger_config: String,
    /// 触发器的唯一ID。
    #[serde(rename = "triggerId")]
    pub trigger_id: String,
    /// 触发器的名称。要求只能包含字母、数字、下划线(_)和短划线(-)。不能以数字、短划线(-)开头，长度限制为1~128个字符。
    #[serde(rename = "triggerName")]
    pub trigger_name: String,
    /// 触发器的类型。
    #[serde(rename = "triggerType")]
    pub trigger_type: String,
}

impl crate::FlatSerialize for Trigger {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.created_time,
            &format!("{}.createdTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.http_trigger,
            &format!("{}.httpTrigger", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.invocation_role,
            &format!("{}.invocationRole", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.lastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qualifier,
            &format!("{}.qualifier", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_arn,
            &format!("{}.sourceArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.status, &format!("{}.status", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.target_arn,
            &format!("{}.targetArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_config,
            &format!("{}.triggerConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_id,
            &format!("{}.triggerId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_name,
            &format!("{}.triggerName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_type,
            &format!("{}.triggerType", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for Trigger {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListTriggersOutput {
    /// 下一个触发器的名称。用来返回更多的查询结果。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: String,
    /// 触发器列表。
    #[serde(rename = "triggers")]
    pub triggers: Vec<Trigger>,
}

impl crate::FlatSerialize for ListTriggersOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.triggers, &format!("{}.triggers", name), params);
    }
}

impl crate::ToCodeMessage for ListTriggersOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Version {
    /// 创建时间
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// 版本描述信息
    #[serde(rename = "description")]
    pub description: String,
    /// 更新时间
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    /// 版本ID
    #[serde(rename = "versionId")]
    pub version_id: String,
}

impl crate::FlatSerialize for Version {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.created_time,
            &format!("{}.createdTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.last_modified_time,
            &format!("{}.lastModifiedTime", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.version_id,
            &format!("{}.versionId", name),
            params,
        );
    }
}

impl crate::ToCodeMessage for Version {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListVersionsOutput {
    /// 版本排序方式
    #[serde(rename = "direction")]
    pub direction: String,
    /// 下一个版本ID
    #[serde(rename = "nextToken")]
    pub next_token: String,
    /// 版本ID列表
    #[serde(rename = "versions")]
    pub versions: Vec<Version>,
}

impl crate::FlatSerialize for ListVersionsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.direction,
            &format!("{}.direction", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.next_token,
            &format!("{}.nextToken", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.versions, &format!("{}.versions", name), params);
    }
}

impl crate::ToCodeMessage for ListVersionsOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ListVpcBindingsOutput {
    /// VPC实例ID列表
    #[serde(rename = "vpcIds")]
    pub vpc_ids: Vec<String>,
}

impl crate::FlatSerialize for ListVpcBindingsOutput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.vpc_ids, &format!("{}.vpcIds", name), params);
    }
}

impl crate::ToCodeMessage for ListVpcBindingsOutput {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MCPSSESessionAffinityConfig {
    /// SSE 路径
    #[serde(rename = "sseEndpointPath")]
    pub sse_endpoint_path: String,
    /// 单实例在同一个时间内能同时处理的最大 Session 数，取值范围为[1, 200]。
    #[serde(rename = "sessionConcurrencyPerInstance")]
    pub session_concurrency_per_instance: i64,
}

impl crate::FlatSerialize for MCPSSESessionAffinityConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.sse_endpoint_path,
            &format!("{}.sseEndpointPath", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_concurrency_per_instance,
            &format!("{}.sessionConcurrencyPerInstance", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MCPStreamableSessionAffinityConfig {
    /// 指从 Session 创建、使用到最终销毁的全过程。 超过生命周期，函数计算将会自动销毁Session， 不再保证亲和性，取值范围为[1, 21600]。
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    /// 用户在一段时间内没有进行任何操作，导致会话进入空闲状态，最大时长为单个 Session 生命周期上限，取值范围为[0, 21600]。
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    /// 单实例在同一个时间内能同时处理的最大 Session 数，取值范围为[1, 200]。
    #[serde(rename = "sessionConcurrencyPerInstance")]
    pub session_concurrency_per_instance: i64,
}

impl crate::FlatSerialize for MCPStreamableSessionAffinityConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.session_ttl_in_seconds,
            &format!("{}.sessionTTLInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_idle_timeout_in_seconds,
            &format!("{}.sessionIdleTimeoutInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_concurrency_per_instance,
            &format!("{}.sessionConcurrencyPerInstance", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MNSTopicTriggerConfig {
    /// 过滤标签。只有收到包含了此处设置的过滤标签字符串的消息时，才会触发函数执行。
    #[serde(rename = "filterTag")]
    pub filter_tag: String,
    /// 触发事件的内容格式，目前支持如下两种格式：
    ///
    /// - **JSON**
    ///
    /// - **STREAM**
    ///
    /// > 默认格式为 STREAM。
    #[serde(rename = "notifyContentFormat")]
    pub notify_content_format: String,
    /// 重试策略。
    ///
    /// - **BACKOFF_RETRY**：表示退避重试；重试3次，每次重试的间隔时间是10秒到20秒之间的随机值，默认值。
    ///
    /// - **EXPONENTIAL_DECAY_RETRY**：表示指数衰减重试；重试176次，每次重试的间隔时间指数递增至512秒，总计重试时间为1天。每次重试的具体间隔为：1，2，4，8，16，32，64，128，256，512，512...512（共167个512）。
    #[serde(rename = "notifyStrategy")]
    pub notify_strategy: String,
}

impl crate::FlatSerialize for MNSTopicTriggerConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.filter_tag,
            &format!("{}.filterTag", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.notify_content_format,
            &format!("{}.notifyContentFormat", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.notify_strategy,
            &format!("{}.notifyStrategy", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OSSTriggerConfig {
    /// 事件列表，请填写OSS对象存储相关的事件。事件类型请参见[OSS 触发事件列表](~~2513613~~)。
    #[serde(rename = "events")]
    pub events: Vec<String>,
    /// 事件过滤规则。
    #[serde(rename = "filter")]
    pub filter: Filter,
}

impl crate::FlatSerialize for OSSTriggerConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.events, &format!("{}.events", name), params);
        crate::FlatSerialize::flat_serialize(&self.filter, &format!("{}.filter", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OutputFuncCode {
    /// 函数代码包的CRC64值。
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// 函数代码包的URL。
    #[serde(rename = "url")]
    pub url: String,
}

impl crate::FlatSerialize for OutputFuncCode {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.checksum, &format!("{}.checksum", name), params);
        crate::FlatSerialize::flat_serialize(&self.url, &format!("{}.url", name), params);
    }
}

impl crate::ToCodeMessage for OutputFuncCode {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &crate::CODE_MESSAGE
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PublishVersionInput {
    /// 版本描述信息
    #[serde(rename = "description")]
    pub description: String,
}

impl crate::FlatSerialize for PublishVersionInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutAsyncInvokeConfigInput {
    /// 是否开启异步任务（非必填）
    #[serde(rename = "asyncTask")]
    pub async_task: bool,
    /// 异步调用目标的配置结构体（非必填）。
    #[serde(rename = "destinationConfig")]
    pub destination_config: DestinationConfig,
    /// 消息最大存活时长（非必填），取值范围[1,604800]，默认为86400，单位为秒。
    #[serde(rename = "maxAsyncEventAgeInSeconds")]
    pub max_async_event_age_in_seconds: i64,
    /// 异步调用失败后的最大重试次数，非必填。取值范围[0,8]。当您未进行配置时，系统内部默认重试次数为3。
    #[serde(rename = "maxAsyncRetryAttempts")]
    pub max_async_retry_attempts: i64,
}

impl crate::FlatSerialize for PutAsyncInvokeConfigInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.async_task,
            &format!("{}.asyncTask", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.destination_config,
            &format!("{}.destinationConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_async_event_age_in_seconds,
            &format!("{}.maxAsyncEventAgeInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.max_async_retry_attempts,
            &format!("{}.maxAsyncRetryAttempts", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutConcurrencyInput {
    /// 预留并发，函数预留账号并发的一部份，其他函数不可以使用这部份并发。预留并发包括预留实例和按量实例的总并发。
    #[serde(rename = "reservedConcurrency")]
    pub reserved_concurrency: i64,
}

impl crate::FlatSerialize for PutConcurrencyInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.reserved_concurrency,
            &format!("{}.reservedConcurrency", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutElasticConfigInput {
    #[serde(rename = "minInstances")]
    pub min_instances: i64,
    #[serde(rename = "residentPoolId")]
    pub resident_pool_id: String,
    #[serde(rename = "scheduledPolicies")]
    pub scheduled_policies: Vec<ScheduledPolicy>,
    #[serde(rename = "scalingPolicies")]
    pub scaling_policies: Vec<ScalingPolicy>,
}

impl crate::FlatSerialize for PutElasticConfigInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.min_instances,
            &format!("{}.minInstances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resident_pool_id,
            &format!("{}.residentPoolId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.scheduled_policies,
            &format!("{}.scheduledPolicies", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.scaling_policies,
            &format!("{}.scalingPolicies", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutProvisionConfigInput {
    /// 定时策略配置。
    #[serde(rename = "scheduledActions")]
    pub scheduled_actions: Vec<ScheduledAction>,
    /// 默认最小实例数，取值范围为[0,10000]。
    /// > - 未配置指标追踪弹性策略或定时弹性策略时，当前最小实例数等于您配置最小实例数。
    /// > - 如果您配置了多条最小实例数弹性策略，系统会计算每条策略触发时的最小实例数，并取当前时间有效的弹性策略中最小实例数的最大值作为当前最小实例数。
    #[serde(rename = "defaultTarget")]
    pub default_target: i64,
    /// 是否始终分配CPU，默认为true。
    #[serde(rename = "alwaysAllocateCPU")]
    pub always_allocate_cpu: bool,
    /// 是否始终分配GPU，默认为true。
    #[serde(rename = "alwaysAllocateGPU")]
    pub always_allocate_gpu: bool,
    /// 指标追踪伸缩策略配置。
    #[serde(rename = "targetTrackingPolicies")]
    pub target_tracking_policies: Vec<TargetTrackingPolicy>,
    /// ><notice>建议不再使用该参数，请使用 defaultTarget 参数。 </notice>
    /// 预留的目标资源个数。取值范围为[0,10000]。
    #[serde(rename = "target")]
    pub target: i64,
}

impl crate::FlatSerialize for PutProvisionConfigInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.scheduled_actions,
            &format!("{}.scheduledActions", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.default_target,
            &format!("{}.defaultTarget", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.always_allocate_cpu,
            &format!("{}.alwaysAllocateCPU", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.always_allocate_gpu,
            &format!("{}.alwaysAllocateGPU", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.target_tracking_policies,
            &format!("{}.targetTrackingPolicies", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.target, &format!("{}.target", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PutScalingConfigInput {
    #[serde(rename = "residentPoolId")]
    pub resident_pool_id: String,
    #[serde(rename = "minInstances")]
    pub min_instances: i64,
    #[serde(rename = "enableOnDemandScaling")]
    pub enable_on_demand_scaling: bool,
    #[serde(rename = "scheduledPolicies")]
    pub scheduled_policies: Vec<ScheduledPolicy>,
    #[serde(rename = "horizontalScalingPolicies")]
    pub horizontal_scaling_policies: Vec<ScalingPolicy>,
    #[serde(rename = "enableMixMode")]
    pub enable_mix_mode: bool,
    #[serde(rename = "requestDispatchPolicy")]
    pub request_dispatch_policy: String,
}

impl crate::FlatSerialize for PutScalingConfigInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resident_pool_id,
            &format!("{}.residentPoolId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.min_instances,
            &format!("{}.minInstances", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_on_demand_scaling,
            &format!("{}.enableOnDemandScaling", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.scheduled_policies,
            &format!("{}.scheduledPolicies", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.horizontal_scaling_policies,
            &format!("{}.horizontalScalingPolicies", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_mix_mode,
            &format!("{}.enableMixMode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.request_dispatch_policy,
            &format!("{}.requestDispatchPolicy", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResidentConfig {
    #[serde(rename = "poolId")]
    pub pool_id: String,
    #[serde(rename = "count")]
    pub count: i64,
}

impl crate::FlatSerialize for ResidentConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.pool_id, &format!("{}.poolId", name), params);
        crate::FlatSerialize::flat_serialize(&self.count, &format!("{}.count", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SLSTriggerLogConfig {
    /// Logstore名称。触发函数的过程中发生的异常和函数执行统计信息会记录到该Logstore。
    #[serde(rename = "logstore")]
    pub logstore: String,
    /// Project名称。触发函数的过程中发生的异常和函数执行统计信息会记录到该Project下的Logstore中。
    #[serde(rename = "project")]
    pub project: String,
}

impl crate::FlatSerialize for SLSTriggerLogConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.logstore, &format!("{}.logstore", name), params);
        crate::FlatSerialize::flat_serialize(&self.project, &format!("{}.project", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceConfig {
    /// Logstore名称。当前触发器会定时从此Logstore中订阅数据，然后触发函数。
    #[serde(rename = "logstore")]
    pub logstore: String,
    /// 消费起始时间，单位为秒，若不指定，则会从最新数据开始消费。若指定，则会对指定时间之后写入的数据产生触发事件，针对存量数据消费，在追上实时触发进度以前会忽略触发间隔来追赶消费延迟，当追赶完成后，此时触发没有延迟，开始按照设置的触发事件间隔来触发函数调用。
    #[serde(rename = "startTime")]
    pub start_time: i64,
}

impl crate::FlatSerialize for SourceConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.logstore, &format!("{}.logstore", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.start_time,
            &format!("{}.startTime", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SLSTriggerConfig {
    /// 是否启用触发器
    #[serde(rename = "enable")]
    pub enable: bool,
    /// 调用参数。日志服务将该配置内容作为event的一部分传入函数，该配置内容必须是JSON格式的字符串。
    #[serde(rename = "functionParameter")]
    pub function_parameter: std::collections::HashMap<String, String>,
    /// 触发器读取日志间隔及错误重试配置。
    #[serde(rename = "jobConfig")]
    pub job_config: JobConfig,
    /// 触发操作自身的日志配置。
    #[serde(rename = "logConfig")]
    pub log_config: SLSTriggerLogConfig,
    /// 触发源配置。
    #[serde(rename = "sourceConfig")]
    pub source_config: SourceConfig,
}

impl crate::FlatSerialize for SLSTriggerConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.enable, &format!("{}.enable", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.function_parameter,
            &format!("{}.functionParameter", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.job_config,
            &format!("{}.jobConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.log_config,
            &format!("{}.logConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.source_config,
            &format!("{}.sourceConfig", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScalingStatus {
    #[serde(rename = "currentError")]
    pub current_error: String,
    #[serde(rename = "resourceCount")]
    pub resource_count: i64,
}

impl crate::FlatSerialize for ScalingStatus {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.current_error,
            &format!("{}.currentError", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_count,
            &format!("{}.resourceCount", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TagResourceInput {
    /// 资源描述符。
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// 标签字典。
    #[serde(rename = "tags")]
    pub tags: std::collections::HashMap<String, String>,
}

impl crate::FlatSerialize for TagResourceInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resource_arn,
            &format!("{}.resourceArn", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tags, &format!("{}.tags", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TagResourcesInput {
    /// 资源ID列表
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 资源类型
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 标签列表
    #[serde(rename = "Tag")]
    pub tag: Vec<Tag>,
}

impl crate::FlatSerialize for TagResourcesInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.resource_id,
            &format!("{}.ResourceId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.resource_type,
            &format!("{}.ResourceType", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.tag, &format!("{}.Tag", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TimerTriggerConfig {
    /// 触发周期表达式。按照时间间隔触发：以每间隔4分钟为例，表达式为 “@every 4m”。按照CRON表达式触发：常见格式类似 “0 0 4 * * *”
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    /// 是否启用触发器
    #[serde(rename = "enable")]
    pub enable: bool,
    /// 输入自定义的参数，该触发消息将会作为event中payload的值。
    #[serde(rename = "payload")]
    pub payload: String,
}

impl crate::FlatSerialize for TimerTriggerConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.cron_expression,
            &format!("{}.cronExpression", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.enable, &format!("{}.enable", name), params);
        crate::FlatSerialize::flat_serialize(&self.payload, &format!("{}.payload", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAliasInput {
    /// 灰度版本权重
    #[serde(rename = "additionalVersionWeight")]
    pub additional_version_weight: std::collections::HashMap<String, f64>,
    /// 别名的描述信息
    #[serde(rename = "description")]
    pub description: String,
    /// 别名指向的版本
    #[serde(rename = "versionId")]
    pub version_id: String,
}

impl crate::FlatSerialize for UpdateAliasInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.additional_version_weight,
            &format!("{}.additionalVersionWeight", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.version_id,
            &format!("{}.versionId", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateCustomDomainInput {
    /// 权限认证配置
    #[serde(rename = "authConfig")]
    pub auth_config: AuthConfig,
    /// HTTPS证书的信息。
    #[serde(rename = "certConfig")]
    pub cert_config: CertConfig,
    /// 域名支持的协议类型。HTTP：仅支持HTTP协议。HTTPS：仅支持HTTPS协议。HTTP,HTTPS：支持HTTP及HTTPS协议。
    #[serde(rename = "protocol")]
    pub protocol: String,
    /// 路由表：自定义域名访问时的PATH到Function的映射。
    #[serde(rename = "routeConfig")]
    pub route_config: RouteConfig,
    /// TLS配置信息。
    #[serde(rename = "tlsConfig")]
    pub tls_config: TLSConfig,
    /// Web应用防火墙配置信息。
    #[serde(rename = "wafConfig")]
    pub waf_config: WAFConfig,
}

impl crate::FlatSerialize for UpdateCustomDomainInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.auth_config,
            &format!("{}.authConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cert_config,
            &format!("{}.certConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.protocol, &format!("{}.protocol", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.route_config,
            &format!("{}.routeConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.tls_config,
            &format!("{}.tlsConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.waf_config,
            &format!("{}.wafConfig", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateFunctionInput {
    /// 函数代码ZIP包。code和customContainerConfig二选一。
    #[serde(rename = "code")]
    pub code: InputCodeLocation,
    /// 函数的CPU规格，单位为vCPU，取值需要为0.05 vCPU的倍数。
    #[serde(rename = "cpu")]
    pub cpu: f32,
    /// 自定义容器运行时的相关配置，成功配置后函数可以使用自定义容器镜像执行函数。code和customContainerConfig二选一。
    #[serde(rename = "customContainerConfig")]
    pub custom_container_config: CustomContainerConfig,
    /// 自定义DNS配置。
    #[serde(rename = "customDNS")]
    pub custom_dns: CustomDNS,
    /// 自定义运行时配置。
    #[serde(rename = "customRuntimeConfig")]
    pub custom_runtime_config: CustomRuntimeConfig,
    /// 函数的描述。
    #[serde(rename = "description")]
    pub description: String,
    /// 函数的磁盘规格，单位为MB，可选值为512 MB或10240 MB。
    #[serde(rename = "diskSize")]
    pub disk_size: i32,
    /// 函数的环境变量，可以在运行环境中访问设置的环境变量。
    #[serde(rename = "environmentVariables")]
    pub environment_variables: std::collections::HashMap<String, String>,
    /// 函数GPU配置。
    #[serde(rename = "gpuConfig")]
    pub gpu_config: GPUConfig,
    /// 函数执行的入口，具体格式和运行时相关。
    #[serde(rename = "handler")]
    pub handler: String,
    /// 实例最大并发度。
    #[serde(rename = "instanceConcurrency")]
    pub instance_concurrency: i32,
    /// 实例生命周期回调方法配置。
    #[serde(rename = "instanceLifecycleConfig")]
    pub instance_lifecycle_config: InstanceLifecycleConfig,
    /// 是否允许访问公网。
    #[serde(rename = "internetAccess")]
    pub internet_access: bool,
    /// 层的列表。多个层会按照数组下标从大到小的顺序进行合并，下标小的层的内容会覆盖下标大的层的同名文件。
    #[serde(rename = "layers")]
    pub layers: Vec<String>,
    /// 日志配置。函数产生的日志会被写入到配置的日志库中。
    #[serde(rename = "logConfig")]
    pub log_config: LogConfig,
    /// 函数的内存规格，单位为MB，内存大小为64 MB的倍数。不同的函数实例类型，内存规格存在差异。
    #[serde(rename = "memorySize")]
    pub memory_size: i32,
    /// NAS配置。配置此参数后，函数可以访问指定的NAS资源。
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
    /// OSS挂载配置。
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: OSSMountConfig,
    /// 授予函数计算所需权限的RAM角色，使用场景包含：1. 把函数产生的日志发送到您的日志库中。2. 为函数在执行过程中访问其他云资源生成的临时访问令牌。
    #[serde(rename = "role")]
    pub role: String,
    /// 函数的运行时环境。
    #[serde(rename = "runtime")]
    pub runtime: String,
    /// 函数运行的超时时间，单位为秒，最小1秒，默认3秒。函数超过这个时间后会被终止执行。
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// 链路追踪配置。当函数计算与链路追踪集成后，您可以记录请求在函数计算的耗时时间、查看函数的冷启动时间、记录函数内部时间的消耗等。
    #[serde(rename = "tracingConfig")]
    pub tracing_config: TracingConfig,
    /// VPC配置。配置此参数后，函数可以访问指定的VPC资源。
    #[serde(rename = "vpcConfig")]
    pub vpc_config: VPCConfig,
    /// 是否禁止创建按量实例，功能开启后，不会创建按量实例，只能使用预留实例
    #[serde(rename = "disableOndemand")]
    pub disable_ondemand: bool,
    /// 函数计算调用请求的亲和策略，如需实现 MCP SSE协议的请求亲和，可设置为 MCP_SSE。如使用Cookie亲和，可设置为 GENERATED_COOKIE。如使用 Header亲和，可设置为 HEADER_FIELD。如不设置或设置为 NONE，则无亲和效果，按函数计算系统默认调度策略路由请求。
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: String,
    /// 是否允许 GPU 函数的预留实例常驻，启用该功能时，创建的函数实例不会被注入 STS token。
    #[serde(rename = "enableLongLiving")]
    pub enable_long_living: bool,
    /// 实例隔离模式
    #[serde(rename = "instanceIsolationMode")]
    pub instance_isolation_mode: UpdateFunctionInputInstanceIsolationMode,
    /// 当设置sessionAffinity亲和类型时，需设置相关的亲和配置。如MCP_SSE亲和需填充 MCPSSESessionAffinityConfig 配置。Cookie亲和需填充CookieSessionAffinityConfig配置，Header Field 亲和需填充HeaderFieldSessionAffinityConfig配置。
    #[serde(rename = "sessionAffinityConfig")]
    pub session_affinity_config: String,
    /// 实例延迟释放时间
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: i32,
    /// 是否不注入 STS token，取值None/Env/Request/All
    /// None: 都注入
    /// Env: 环境变量不注入
    /// Request: 请求中不注入包括context/header
    /// All: 都不注入
    #[serde(rename = "disableInjectCredentials")]
    pub disable_inject_credentials: UpdateFunctionInputDisableInjectCredentials,
    /// PolarFs配置。配置此参数后，函数可以访问指定的PolarFs资源。
    #[serde(rename = "polarFsConfig")]
    pub polar_fs_config: PolarFsConfig,
}

impl crate::FlatSerialize for UpdateFunctionInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.code, &format!("{}.code", name), params);
        crate::FlatSerialize::flat_serialize(&self.cpu, &format!("{}.cpu", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.custom_container_config,
            &format!("{}.customContainerConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.custom_dns,
            &format!("{}.customDNS", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.custom_runtime_config,
            &format!("{}.customRuntimeConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disk_size,
            &format!("{}.diskSize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.environment_variables,
            &format!("{}.environmentVariables", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.gpu_config,
            &format!("{}.gpuConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.handler, &format!("{}.handler", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.instance_concurrency,
            &format!("{}.instanceConcurrency", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_lifecycle_config,
            &format!("{}.instanceLifecycleConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.internet_access,
            &format!("{}.internetAccess", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.layers, &format!("{}.layers", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.log_config,
            &format!("{}.logConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.memory_size,
            &format!("{}.memorySize", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.nas_config,
            &format!("{}.nasConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.oss_mount_config,
            &format!("{}.ossMountConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.role, &format!("{}.role", name), params);
        crate::FlatSerialize::flat_serialize(&self.runtime, &format!("{}.runtime", name), params);
        crate::FlatSerialize::flat_serialize(&self.timeout, &format!("{}.timeout", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.tracing_config,
            &format!("{}.tracingConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.vpc_config,
            &format!("{}.vpcConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_ondemand,
            &format!("{}.disableOndemand", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_affinity,
            &format!("{}.sessionAffinity", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.enable_long_living,
            &format!("{}.enableLongLiving", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.instance_isolation_mode,
            &format!("{}.instanceIsolationMode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_affinity_config,
            &format!("{}.sessionAffinityConfig", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.idle_timeout,
            &format!("{}.idleTimeout", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_inject_credentials,
            &format!("{}.disableInjectCredentials", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.polar_fs_config,
            &format!("{}.polarFsConfig", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateResidentResourcePoolInput {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "useScaling")]
    pub use_scaling: bool,
}

impl crate::FlatSerialize for UpdateResidentResourcePoolInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.name, &format!("{}.name", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.use_scaling,
            &format!("{}.useScaling", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateSessionInput {
    /// Session生命周期
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    /// Session 闲置过期时间
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    /// 默认值 False，表示在 SessionID 会话过期后，可携带相同SessionID继续发起请求，系统将视为新会话绑定新实例。当配置为 True，表示在 SessionID 会话过期后，不可复用 SessionID。
    #[serde(rename = "disableSessionIdReuse")]
    pub disable_session_id_reuse: bool,
}

impl crate::FlatSerialize for UpdateSessionInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.session_ttl_in_seconds,
            &format!("{}.sessionTTLInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.session_idle_timeout_in_seconds,
            &format!("{}.sessionIdleTimeoutInSeconds", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.disable_session_id_reuse,
            &format!("{}.disableSessionIdReuse", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTriggerInput {
    /// 触发器的描述。
    #[serde(rename = "description")]
    pub description: String,
    /// 事件源（如OSS）调用函数所需的角色。
    #[serde(rename = "invocationRole")]
    pub invocation_role: String,
    /// 函数的版本或别名。
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    /// 触发器配置，针对不同类型的触发器，配置有所不同。
    #[serde(rename = "triggerConfig")]
    pub trigger_config: String,
}

impl crate::FlatSerialize for UpdateTriggerInput {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.description,
            &format!("{}.description", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.invocation_role,
            &format!("{}.invocationRole", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qualifier,
            &format!("{}.qualifier", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.trigger_config,
            &format!("{}.triggerConfig", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunctionInvocationbody {
    /// 禁止函数调用的原因
    #[serde(rename = "reason")]
    pub reason: String,
    /// 是否立即终止正在处理的所有请求
    #[serde(rename = "abortOngoingRequest")]
    pub abort_ongoing_request: bool,
}

impl crate::FlatSerialize for FunctionInvocationbody {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.reason, &format!("{}.reason", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.abort_ongoing_request,
            &format!("{}.abortOngoingRequest", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResourcesTag {
    /// 标签键。
    ///
    /// 最多支持64个字符，不能以`aliyun`和`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Key")]
    pub key: String,
    /// 标签值。
    ///
    /// 标签值最多支持128个字符，可以为空字符串。
    #[serde(rename = "Value")]
    pub value: String,
}

impl crate::FlatSerialize for ResourcesTag {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateFunctionInputInstanceIsolationMode {
    #[serde(rename = "SHARE")]
    SHARE,
    #[serde(rename = "REQUEST_EXCLUSIVE")]
    REQUEST_EXCLUSIVE,
    #[serde(rename = "SESSION_EXCLUSIVE")]
    SESSION_EXCLUSIVE,
}

impl Default for CreateFunctionInputInstanceIsolationMode {
    fn default() -> Self {
        Self::SHARE
    }
}

impl CreateFunctionInputInstanceIsolationMode {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SHARE => "SHARE",
            Self::REQUEST_EXCLUSIVE => "REQUEST_EXCLUSIVE",
            Self::SESSION_EXCLUSIVE => "SESSION_EXCLUSIVE",
        }
    }
}

impl std::fmt::Display for CreateFunctionInputInstanceIsolationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a CreateFunctionInputInstanceIsolationMode> for crate::QueryValue<'a> {
    fn from(value: &'a CreateFunctionInputInstanceIsolationMode) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for CreateFunctionInputInstanceIsolationMode {
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
pub enum CreateFunctionInputDisableInjectCredentials {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Env")]
    Env,
    #[serde(rename = "Request")]
    Request,
    #[serde(rename = "All")]
    All,
}

impl Default for CreateFunctionInputDisableInjectCredentials {
    fn default() -> Self {
        Self::None
    }
}

impl CreateFunctionInputDisableInjectCredentials {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Env => "Env",
            Self::Request => "Request",
            Self::All => "All",
        }
    }
}

impl std::fmt::Display for CreateFunctionInputDisableInjectCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a CreateFunctionInputDisableInjectCredentials> for crate::QueryValue<'a> {
    fn from(value: &'a CreateFunctionInputDisableInjectCredentials) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for CreateFunctionInputDisableInjectCredentials {
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
pub enum FunctionInstanceIsolationMode {
    #[serde(rename = "SHARE")]
    SHARE,
    #[serde(rename = "SESSION_EXCLUSIVE")]
    SESSION_EXCLUSIVE,
    #[serde(rename = "REQUEST_EXCLUSIVE")]
    REQUEST_EXCLUSIVE,
}

impl Default for FunctionInstanceIsolationMode {
    fn default() -> Self {
        Self::SHARE
    }
}

impl FunctionInstanceIsolationMode {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SHARE => "SHARE",
            Self::SESSION_EXCLUSIVE => "SESSION_EXCLUSIVE",
            Self::REQUEST_EXCLUSIVE => "REQUEST_EXCLUSIVE",
        }
    }
}

impl std::fmt::Display for FunctionInstanceIsolationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a FunctionInstanceIsolationMode> for crate::QueryValue<'a> {
    fn from(value: &'a FunctionInstanceIsolationMode) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for FunctionInstanceIsolationMode {
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
pub enum FunctionDisableInjectCredentials {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Env")]
    Env,
    #[serde(rename = "Request")]
    Request,
    #[serde(rename = "All")]
    All,
}

impl Default for FunctionDisableInjectCredentials {
    fn default() -> Self {
        Self::None
    }
}

impl FunctionDisableInjectCredentials {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Env => "Env",
            Self::Request => "Request",
            Self::All => "All",
        }
    }
}

impl std::fmt::Display for FunctionDisableInjectCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a FunctionDisableInjectCredentials> for crate::QueryValue<'a> {
    fn from(value: &'a FunctionDisableInjectCredentials) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for FunctionDisableInjectCredentials {
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
pub enum UpdateFunctionInputInstanceIsolationMode {
    #[serde(rename = "SHARE")]
    SHARE,
    #[serde(rename = "REQUEST_EXCLUSIVE")]
    REQUEST_EXCLUSIVE,
    #[serde(rename = "SESSION_EXCLUSIVE")]
    SESSION_EXCLUSIVE,
}

impl Default for UpdateFunctionInputInstanceIsolationMode {
    fn default() -> Self {
        Self::SHARE
    }
}

impl UpdateFunctionInputInstanceIsolationMode {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SHARE => "SHARE",
            Self::REQUEST_EXCLUSIVE => "REQUEST_EXCLUSIVE",
            Self::SESSION_EXCLUSIVE => "SESSION_EXCLUSIVE",
        }
    }
}

impl std::fmt::Display for UpdateFunctionInputInstanceIsolationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a UpdateFunctionInputInstanceIsolationMode> for crate::QueryValue<'a> {
    fn from(value: &'a UpdateFunctionInputInstanceIsolationMode) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for UpdateFunctionInputInstanceIsolationMode {
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
pub enum UpdateFunctionInputDisableInjectCredentials {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Env")]
    Env,
    #[serde(rename = "Request")]
    Request,
    #[serde(rename = "All")]
    All,
}

impl Default for UpdateFunctionInputDisableInjectCredentials {
    fn default() -> Self {
        Self::None
    }
}

impl UpdateFunctionInputDisableInjectCredentials {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Env => "Env",
            Self::Request => "Request",
            Self::All => "All",
        }
    }
}

impl std::fmt::Display for UpdateFunctionInputDisableInjectCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a UpdateFunctionInputDisableInjectCredentials> for crate::QueryValue<'a> {
    fn from(value: &'a UpdateFunctionInputDisableInjectCredentials) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for UpdateFunctionInputDisableInjectCredentials {
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
pub enum AcceptLanguage {
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "ja")]
    Ja,
}

impl Default for AcceptLanguage {
    fn default() -> Self {
        Self::ZhCn
    }
}

impl AcceptLanguage {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ZhCn => "zh-CN",
            Self::EnUs => "en-US",
            Self::Ja => "ja",
        }
    }
}

impl std::fmt::Display for AcceptLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a AcceptLanguage> for crate::QueryValue<'a> {
    fn from(value: &'a AcceptLanguage) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for AcceptLanguage {
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
pub enum FunctionsfcVersion {
    #[serde(rename = "v3")]
    V3,
    #[serde(rename = "v2")]
    V2,
}

impl Default for FunctionsfcVersion {
    fn default() -> Self {
        Self::V3
    }
}

impl FunctionsfcVersion {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::V3 => "v3",
            Self::V2 => "v2",
        }
    }
}

impl std::fmt::Display for FunctionsfcVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a FunctionsfcVersion> for crate::QueryValue<'a> {
    fn from(value: &'a FunctionsfcVersion) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for FunctionsfcVersion {
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
pub enum InstancesinstanceStatus {
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Terminating")]
    Terminating,
    #[serde(rename = "Destroyed")]
    Destroyed,
}

impl Default for InstancesinstanceStatus {
    fn default() -> Self {
        Self::Running
    }
}

impl InstancesinstanceStatus {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Terminating => "Terminating",
            Self::Destroyed => "Destroyed",
        }
    }
}

impl std::fmt::Display for InstancesinstanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a InstancesinstanceStatus> for crate::QueryValue<'a> {
    fn from(value: &'a InstancesinstanceStatus) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for InstancesinstanceStatus {
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
pub struct EnableFunctionInvocationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 是否成功允许调用
    #[serde(rename = "success")]
    pub success: bool,
}

impl crate::ToCodeMessage for EnableFunctionInvocationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DisableFunctionInvocationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 是否成功禁止调用
    #[serde(rename = "success")]
    pub success: bool,
}

impl crate::ToCodeMessage for DisableFunctionInvocationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}
