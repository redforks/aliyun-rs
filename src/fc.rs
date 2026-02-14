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
    pub fn new(body: impl Into<CreateCustomDomainInput>) -> Self {
        Self { body: body.into() }
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
    pub fn new(domain_name: impl Into<String>, body: impl Into<UpdateCustomDomainInput>) -> Self {
        Self {
            domain_name: domain_name.into(),
            body: body.into(),
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
    pub fn new(function_name: impl Into<String>, body: impl Into<PublishVersionInput>) -> Self {
        Self {
            function_name: function_name.into(),
            body: body.into(),
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
    pub fn new(body: impl Into<CreateFunctionInput>) -> Self {
        Self { body: body.into() }
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
    pub fn new(function_name: impl Into<String>, body: impl Into<UpdateFunctionInput>) -> Self {
        Self {
            function_name: function_name.into(),
            body: body.into(),
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
    pub fn new(
        function_name: impl Into<String>,
        alias_name: impl Into<String>,
        body: impl Into<UpdateAliasInput>,
    ) -> Self {
        Self {
            function_name: function_name.into(),
            alias_name: alias_name.into(),
            body: body.into(),
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
    pub fn new(function_name: impl Into<String>, body: impl Into<CreateAliasInput>) -> Self {
        Self {
            function_name: function_name.into(),
            body: body.into(),
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
    pub fn new(function_name: impl Into<String>, body: impl Into<CreateTriggerInput>) -> Self {
        Self {
            function_name: function_name.into(),
            body: body.into(),
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
    pub fn new(
        function_name: impl Into<String>,
        trigger_name: impl Into<String>,
        body: impl Into<UpdateTriggerInput>,
    ) -> Self {
        Self {
            function_name: function_name.into(),
            trigger_name: trigger_name.into(),
            body: body.into(),
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
    pub fn new(
        body: impl Into<PutAsyncInvokeConfigInput>,
        function_name: impl Into<String>,
    ) -> Self {
        Self {
            body: body.into(),
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
    pub fn new(function_name: impl Into<String>, body: impl Into<PutProvisionConfigInput>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
            body: body.into(),
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
    pub fn new(function_name: impl Into<String>, body: impl Into<PutConcurrencyInput>) -> Self {
        Self {
            function_name: function_name.into(),
            body: body.into(),
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
    pub fn new(layer_name: impl Into<String>, body: impl Into<CreateLayerVersionInput>) -> Self {
        Self {
            layer_name: layer_name.into(),
            body: body.into(),
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
    pub fn new(function_name: impl Into<String>, body: impl Into<CreateVpcBindingInput>) -> Self {
        Self {
            function_name: function_name.into(),
            body: body.into(),
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
    pub fn new(body: impl Into<TagResourcesInput>) -> Self {
        Self { body: body.into() }
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

/// 镜像加速信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccelerationInfo {
    /// 镜像加速状态
    #[serde(rename = "status")]
    pub status: Option<String>,
}

/// 别名信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct Alias {
    /// 灰度版本权重。
    #[serde(rename = "additionalVersionWeight")]
    pub additional_version_weight: std::collections::HashMap<String, f64>,
    /// 别名名称。
    #[serde(rename = "aliasName")]
    pub alias_name: Option<String>,
    /// 创建时间。
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    /// 别名描述信息。
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 修改时间。
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// 别名指向的版本。
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,
}

/// 异步调用目标结构体
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Destination {
    /// 异步调用目标资源描述符
    #[serde(rename = "destination")]
    pub destination: Option<String>,
}

/// 异步调用目标的配置信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DestinationConfig {
    /// 失败的回调目标结构体。
    #[serde(rename = "onFailure")]
    pub on_failure: Option<Destination>,
    /// 成功的回调目标结构体。
    #[serde(rename = "onSuccess")]
    pub on_success: Option<Destination>,
}

/// 异步调用配置
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct AsyncConfig {
    /// 是否开启异步任务
    #[serde(rename = "asyncTask")]
    pub async_task: Option<bool>,
    /// 创建时间
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    /// 目标配置
    #[serde(rename = "destinationConfig")]
    pub destination_config: Option<DestinationConfig>,
    /// 函数资源标识
    #[serde(rename = "functionArn")]
    pub function_arn: Option<String>,
    /// 最后修改时间
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// 事件最大存活时间
    #[serde(rename = "maxAsyncEventAgeInSeconds")]
    pub max_async_event_age_in_seconds: Option<i64>,
    /// 异步调用重试次数
    #[serde(rename = "maxAsyncRetryAttempts")]
    pub max_async_retry_attempts: Option<i64>,
}

/// 任务事件内容
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct AsyncTaskEvent {
    /// 事件负载内容
    #[serde(rename = "eventDetail")]
    pub event_detail: Option<String>,
    /// 事件 ID
    #[serde(rename = "eventId")]
    pub event_id: Option<i64>,
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
    pub status: Option<String>,
    /// 事件发生时间，单位毫秒
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// 异步任务信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct AsyncTask {
    /// 异步任务失败后的已重试次数
    #[serde(rename = "alreadyRetriedTimes")]
    pub already_retried_times: Option<i64>,
    /// 异步任务的最终状态
    #[serde(rename = "destinationStatus")]
    pub destination_status: Option<String>,
    /// 异步任务的执行耗时
    #[serde(rename = "durationMs")]
    pub duration_ms: Option<i64>,
    /// 异步任务结束时间，单位为毫秒
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
    /// 异步任务事件列表
    #[serde(rename = "events")]
    pub events: Vec<AsyncTaskEvent>,
    /// 函数资源标识
    #[serde(rename = "functionArn")]
    pub function_arn: Option<String>,
    /// 异步任务对应的实例 ID
    #[serde(rename = "instanceId")]
    pub instance_id: Option<String>,
    /// 函数的版本或别名。
    #[serde(rename = "qualifier")]
    pub qualifier: Option<String>,
    /// 本次异步任务对应的请求 ID
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    /// 异步任务执行完成后的响应内容。大小限制为 1 MB。
    /// 该字段当前处于内测阶段，如您需要使用，请[联系我们](~~2513733~~)为您开通。
    #[serde(rename = "returnPayload")]
    pub return_payload: Option<String>,
    /// 异步任务开始时间，单位为毫秒
    #[serde(rename = "startedTime")]
    pub started_time: Option<i64>,
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
    pub status: Option<String>,
    /// 异步任务失败的错误消息
    #[serde(rename = "taskErrorMessage")]
    pub task_error_message: Option<String>,
    /// 异步任务 ID
    #[serde(rename = "taskId")]
    pub task_id: Option<String>,
    /// 异步任务执行时的入参内容
    #[serde(rename = "taskPayload")]
    pub task_payload: Option<String>,
}

/// 权限认证配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuthConfig {
    /// 认证信息
    #[serde(rename = "authInfo")]
    pub auth_info: Option<String>,
    /// 认证类型。anonymous, function或者jwt。
    #[serde(rename = "authType")]
    pub auth_type: Option<String>,
}

/// 证书配置
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

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ChangeResourceGroupInput {
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    #[serde(rename = "NewResourceGroupId")]
    pub new_resource_group_id: Option<String>,
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ChangeResourceGroupOutput {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "NewResourceGroupId")]
    #[serde(default)]
    pub new_resource_group_id: String,
    #[serde(rename = "OldResourceGroupId")]
    #[serde(default)]
    pub old_resource_group_id: String,
}

/// 函数并发配置
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ConcurrencyConfig {
    /// 阿里云资源的标识
    #[serde(rename = "functionArn")]
    pub function_arn: Option<String>,
    /// 预留并发，函数预留账号并发的一部份，其他函数不可以使用这部份并发。预留并发包括预留实例和按量实例的总并发。
    #[serde(rename = "reservedConcurrency")]
    pub reserved_concurrency: Option<i64>,
}

/// 创建别名的请求参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CreateAliasInput {
    /// 灰度版本权重
    #[serde(rename = "additionalVersionWeight")]
    pub additional_version_weight: std::collections::HashMap<String, f64>,
    /// 别名名称
    #[serde(rename = "aliasName")]
    pub alias_name: String,
    /// 别名描述信息
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 别名指向的版本
    #[serde(rename = "versionId")]
    pub version_id: String,
}

/// 自定义域名完全匹配重写规则配置
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

/// 自定义域名正则重写规则配置
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

/// 自定义域名通配符重写规则配置
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

/// 重写规则配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RewriteConfig {
    /// 精确匹配规则列表
    #[serde(rename = "equalRules")]
    #[serde(default)]
    pub equal_rules: Vec<EqualRule>,
    /// 正则匹配规则列表
    #[serde(rename = "regexRules")]
    #[serde(default)]
    pub regex_rules: Vec<RegexRule>,
    /// 通配匹配规则列表
    #[serde(rename = "wildcardRules")]
    #[serde(default)]
    pub wildcard_rules: Vec<WildcardRule>,
}

/// 自定义域名路由配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathConfig {
    /// 函数名
    #[serde(rename = "functionName")]
    pub function_name: String,
    /// 支持的方法
    #[serde(rename = "methods")]
    #[serde(default)]
    pub methods: Vec<String>,
    /// 路由匹配规则
    #[serde(rename = "path")]
    pub path: String,
    /// 版本或者别名
    #[serde(rename = "qualifier")]
    pub qualifier: Option<String>,
    /// 重写配置
    #[serde(rename = "rewriteConfig")]
    pub rewrite_config: Option<RewriteConfig>,
}

/// 路由匹配规则配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RouteConfig {
    /// 路由配置列表
    #[serde(rename = "routes")]
    #[serde(default)]
    pub routes: Vec<PathConfig>,
}

/// TLS配置。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TLSConfig {
    /// TLS加密套件列表。
    #[serde(rename = "cipherSuites")]
    pub cipher_suites: Vec<String>,
    /// TLS最大版本号。枚举值：TLSv1.3, TLSv1.2
    #[serde(rename = "maxVersion")]
    pub max_version: Option<String>,
    /// TLS最小版本号。枚举值：TLSv1.3, TLSv1.2
    #[serde(rename = "minVersion")]
    pub min_version: String,
}

/// 自定义域名WAF配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WAFConfig {
    /// 是否开启WAF防护
    #[serde(rename = "enableWAF")]
    pub enable_waf: Option<bool>,
}

/// 创建自定义域名的请求信息。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CreateCustomDomainInput {
    /// 权限认证配置。
    #[serde(rename = "authConfig")]
    pub auth_config: Option<AuthConfig>,
    /// HTTPS证书的信息。
    #[serde(rename = "certConfig")]
    pub cert_config: Option<CertConfig>,
    /// 域名。填写已在阿里云备案或接入备案的自定义域名名称。
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// 域名支持的协议类型。HTTP：仅支持HTTP协议。HTTPS：仅支持HTTPS协议。HTTP,HTTPS：支持HTTP及HTTPS协议。
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
    /// 路由表：自定义域名访问时的PATH到Function的映射。
    #[serde(rename = "routeConfig")]
    pub route_config: Option<RouteConfig>,
    /// TLS配置信息。
    #[serde(rename = "tlsConfig")]
    pub tls_config: Option<TLSConfig>,
    /// Web应用防火墙配置信息。
    #[serde(rename = "wafConfig")]
    pub waf_config: Option<WAFConfig>,
}

/// 函数代码配置。可以使用两种方式提供代码包：
/// 1. 直接将代码包的内容包含在请求中，此时zipFile必须设置。
/// 2. 将代码包上传到OSS，在请求中包含OSS文件的地址，此时ossBucketName和ossObjectName必须设置。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct InputCodeLocation {
    /// 函数代码包的CRC-64值。如果提供了checksum，则函数计算会校验代码包的checksum是否和提供的一致。
    #[serde(rename = "checksum")]
    pub checksum: Option<String>,
    /// 用户存放函数代码ZIP包的OSS Bucket名称。
    #[serde(rename = "ossBucketName")]
    pub oss_bucket_name: Option<String>,
    /// 用户存放函数代码ZIP包的OSS Object名称。
    #[serde(rename = "ossObjectName")]
    pub oss_object_name: Option<String>,
    /// 函数代码ZIP包的Base 64编码。
    #[serde(rename = "zipFile")]
    pub zip_file: Option<String>,
}

/// 函数自定义健康检查配置，仅适用于Custom Runtime和Custom Container。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomHealthCheckConfig {
    /// 健康检查失败次数阈值，达到该值后系统认为检查失败。取值范围1~120。默认值为3。
    #[serde(rename = "failureThreshold")]
    pub failure_threshold: Option<i32>,
    /// 容器自定义健康检查URL地址。长度不超过2048个字符。
    #[serde(rename = "httpGetUrl")]
    pub http_get_url: Option<String>,
    /// 容器启动到发起健康检查的延迟。取值范围0~120。默认值为0。
    #[serde(rename = "initialDelaySeconds")]
    pub initial_delay_seconds: Option<i32>,
    /// 健康检查周期。取值范围1~120。默认值为3。
    #[serde(rename = "periodSeconds")]
    pub period_seconds: Option<i32>,
    /// 健康检查成功次数阈值，达到该值后系统认为检查成功。取值范围1~120。默认值为1。
    #[serde(rename = "successThreshold")]
    pub success_threshold: Option<i32>,
    /// 健康检查超时时间。取值范围1~3。默认值为1。
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i32>,
}

/// 镜像仓库的认证信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegistryAuthConfig {
    /// 镜像仓库密码
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// 镜像仓库用户名
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
}

/// 镜像仓库的证书信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegistryCertConfig {
    /// 是否跳过证书验证
    #[serde(rename = "insecure")]
    pub insecure: Option<bool>,
    /// 镜像仓库CA证书
    #[serde(rename = "rootCaCertBase64")]
    pub root_ca_cert_base64: Option<String>,
}

/// 镜像仓库的网络信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegistryNetworkConfig {
    /// 可以连通镜像仓库的SecurityGroup ID
    #[serde(rename = "securityGroupId")]
    pub security_group_id: Option<String>,
    /// 可以连通镜像仓库的VSwitch ID
    #[serde(rename = "vSwitchId")]
    pub v_switch_id: Option<String>,
    /// 可以连通镜像仓库的VPC ID
    #[serde(rename = "vpcId")]
    pub vpc_id: Option<String>,
}

/// 镜像仓库的配置信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegistryConfig {
    /// 权限认证配置
    #[serde(rename = "authConfig")]
    pub auth_config: Option<RegistryAuthConfig>,
    /// 证书配置
    #[serde(rename = "certConfig")]
    pub cert_config: Option<RegistryCertConfig>,
    /// 网络配置。
    #[serde(rename = "networkConfig")]
    pub network_config: Option<RegistryNetworkConfig>,
}

/// 自定义容器运行时的配置信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomContainerConfig {
    /// 镜像加速信息。
    #[serde(rename = "accelerationInfo")]
    pub acceleration_info: Option<AccelerationInfo>,
    /// 是否开启镜像加速。Default表示开启镜像加速，None表示关闭镜像加速。
    #[serde(rename = "accelerationType")]
    pub acceleration_type: Option<String>,
    /// ACR企业版镜像仓库ID，使用ACR企业版镜像时须传入。
    #[serde(rename = "acrInstanceId")]
    pub acr_instance_id: Option<String>,
    /// 容器启动参数。
    #[serde(rename = "command")]
    #[serde(default)]
    pub command: Vec<String>,
    /// 容器启动命令。
    #[serde(rename = "entrypoint")]
    #[serde(default)]
    pub entrypoint: Vec<String>,
    /// 函数自定义健康检查配置。
    #[serde(rename = "healthCheckConfig")]
    pub health_check_config: Option<CustomHealthCheckConfig>,
    /// 容器镜像地址。
    #[serde(rename = "image")]
    pub image: Option<String>,
    /// 自定义容器运行时HTTP Server的监听端口。
    #[serde(rename = "port")]
    pub port: Option<i32>,
    /// registry related
    #[serde(rename = "registryConfig")]
    pub registry_config: Option<RegistryConfig>,
    /// 所部署的镜像的实际digest版本，函数启动时实际使用此digest指定的代码版本。由GetFunction时返回，作为参数时无需提供。
    #[serde(rename = "resolvedImageUri")]
    pub resolved_image_uri: Option<String>,
}

/// DNS解析配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DNSOption {
    /// 配置项名称
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// 配置项值
    #[serde(rename = "value")]
    pub value: Option<String>,
}

/// 函数自定义DNS配置。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomDNS {
    /// resolv.conf文件中的DNS解析配置列表。每一项对应一个键值对，格式为key:value，其中键为必填。
    #[serde(rename = "dnsOptions")]
    #[serde(default)]
    pub dns_options: Vec<DNSOption>,
    /// DNS服务器的IP地址列表。
    #[serde(rename = "nameServers")]
    #[serde(default)]
    pub name_servers: Vec<String>,
    /// DNS搜索域列表。
    #[serde(rename = "searches")]
    #[serde(default)]
    pub searches: Vec<String>,
}

/// 自定义运行时的配置信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomRuntimeConfig {
    /// 实例启动参数。
    #[serde(rename = "args")]
    #[serde(default)]
    pub args: Vec<String>,
    /// 实例启动命令。
    #[serde(rename = "command")]
    #[serde(default)]
    pub command: Vec<String>,
    /// 函数自定义健康检查配置。
    #[serde(rename = "healthCheckConfig")]
    pub health_check_config: Option<CustomHealthCheckConfig>,
    /// HTTP Server的监听端口。
    #[serde(rename = "port")]
    pub port: Option<i32>,
}

/// 函数GPU配置信息。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GPUConfig {
    /// GPU显存规格，单位为MB，为1024MB的倍数
    #[serde(rename = "gpuMemorySize")]
    pub gpu_memory_size: Option<i32>,
    /// GPU实例类型。
    ///
    ///  - fc.gpu.tesla.1 表示 GPU Tesla 系列实例类型。
    ///
    ///  - fc.gpu.ampere.1 表示 GPU Ampere 系列实例类型。
    ///
    ///  - fc.gpu.ada.1 表示 GPU Ada 系列实例类型。
    #[serde(rename = "gpuType")]
    pub gpu_type: Option<String>,
}

/// 实例生命周期回调方法配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleHook {
    /// 回调方法的执行入口，含义与请求处理程序类似。
    #[serde(rename = "handler")]
    pub handler: Option<String>,
    /// 回调方法的超时时间，单位为秒。
    #[serde(rename = "timeout")]
    pub timeout: Option<i32>,
    /// 函数生命周期初始化阶段回调指令，生命周期回调方法的执行入口 handler 和 command 不允许同时配置，只能有一个生效，同时配置会产生错误提示
    #[serde(rename = "command")]
    #[serde(default)]
    pub command: Vec<String>,
}

/// 实例生命周期回调方法配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InstanceLifecycleConfig {
    /// Initializer回调方法配置
    #[serde(rename = "initializer")]
    pub initializer: Option<LifecycleHook>,
    /// PreStop回调方法配置
    #[serde(rename = "preStop")]
    pub pre_stop: Option<LifecycleHook>,
}

/// 函数日志配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LogConfig {
    /// 是否开启实例级别指标。开启该功能后，您可以查看实例级别的CPU使用情况、内存使用情况、实例网络情况和实例内请求数等核心指标信息。false：默认值，表示关闭实例级别指标。true：表示开启实例级别指标。
    #[serde(rename = "enableInstanceMetrics")]
    pub enable_instance_metrics: Option<bool>,
    /// 是否开启请求级别指标。开启该功能后，您可以查看该服务下所有函数的某次调用所消耗的时间及内存。false：表示关闭请求级别指标。true：默认值，表示开启请求级别指标。
    #[serde(rename = "enableRequestMetrics")]
    pub enable_request_metrics: Option<bool>,
    /// 日志行首匹配规则
    #[serde(rename = "logBeginRule")]
    pub log_begin_rule: Option<String>,
    /// 日志服务的Logstore名称。
    #[serde(rename = "logstore")]
    pub logstore: Option<String>,
    /// 日志服务的Project名称
    #[serde(rename = "project")]
    pub project: Option<String>,
}

/// NAS挂载点列表
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NASMountConfig {
    /// 使用传输加密方式挂载。 说明：仅通用型NAS支持传输加密
    #[serde(rename = "enableTLS")]
    pub enable_tls: Option<bool>,
    /// 本地挂载目录。
    #[serde(rename = "mountDir")]
    pub mount_dir: Option<String>,
    /// NAS服务器地址。
    #[serde(rename = "serverAddr")]
    pub server_addr: Option<String>,
}

/// NAS配置。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NASConfig {
    /// 群组ID。
    #[serde(rename = "groupId")]
    pub group_id: Option<i32>,
    /// 挂载点列表。
    #[serde(rename = "mountPoints")]
    #[serde(default)]
    pub mount_points: Vec<NASMountConfig>,
    /// 账号ID。
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
}

/// OSS挂载点配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OSSMountPoint {
    /// 挂载的OSS Bucket。
    #[serde(rename = "bucketName")]
    pub bucket_name: Option<String>,
    /// 挂载的OSS Bucket路径。
    #[serde(rename = "bucketPath")]
    pub bucket_path: Option<String>,
    /// OSS访问地址。
    #[serde(rename = "endpoint")]
    pub endpoint: Option<String>,
    /// 挂载目录。
    #[serde(rename = "mountDir")]
    pub mount_dir: Option<String>,
    /// 是否只读。
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
}

/// OSS挂载配置。
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OSSMountConfig {
    /// OSS挂载点列表。
    #[serde(rename = "mountPoints")]
    #[serde(default)]
    pub mount_points: Vec<OSSMountPoint>,
}

/// 链路追踪配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TracingConfig {
    /// 链路追踪参数。参数为map[string]string，其中key为"endpoint"，value为您的链路追踪内网接入点。 例如 endpoint: http://tracing-analysis-dc-hz.aliyuncs.com/adapt_xxx/api/otlp/traces 。
    #[serde(rename = "params")]
    #[serde(default)]
    pub params: std::collections::HashMap<String, String>,
    /// 链路追踪协议类型，目前只支持Jaeger。
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

/// VPC配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VPCConfig {
    /// 安全组ID。
    #[serde(rename = "securityGroupId")]
    pub security_group_id: Option<String>,
    /// 交换机列表。
    #[serde(rename = "vSwitchIds")]
    #[serde(default)]
    pub v_switch_ids: Vec<String>,
    /// VPC网络ID。
    #[serde(rename = "vpcId")]
    pub vpc_id: Option<String>,
    /// 授予函数计算访问用户VPC所需权限的RAM角色
    #[serde(rename = "role")]
    #[serde(default)]
    pub role: String,
}

/// 标签
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    /// 标签名
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 标签值
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PolarFsMountConfig {
    #[serde(rename = "instanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "mountDir")]
    #[serde(default)]
    pub mount_dir: String,
    #[serde(rename = "remoteDir")]
    #[serde(default)]
    pub remote_dir: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PolarFsConfig {
    /// 群组ID。
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: i32,
    /// 账号ID。
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: i32,
    /// 挂载点列表。
    #[serde(rename = "mountPoints")]
    #[serde(default)]
    pub mount_points: Vec<PolarFsMountConfig>,
}

/// 函数创建请求参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CreateFunctionInput {
    /// 函数代码ZIP包。code和customContainerConfig二选一。
    #[serde(rename = "code")]
    pub code: Option<InputCodeLocation>,
    /// 函数的CPU规格，单位为vCPU，为0.05 vCPU的倍数。最小值为0.05，最大值为16。同时cpu和memorySize（按GB算）比例要在1:1和1:4之间。
    #[serde(rename = "cpu")]
    pub cpu: Option<f32>,
    /// 自定义容器运行时的相关配置，成功配置后函数可以使用自定义容器镜像执行函数。code和customContainerConfig二选一。
    #[serde(rename = "customContainerConfig")]
    pub custom_container_config: Option<CustomContainerConfig>,
    /// 自定义DNS配置。
    #[serde(rename = "customDNS")]
    pub custom_dns: Option<CustomDNS>,
    /// 自定义运行时配置。
    #[serde(rename = "customRuntimeConfig")]
    pub custom_runtime_config: Option<CustomRuntimeConfig>,
    /// 函数的描述。
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 函数的磁盘规格，单位为MB，可选值为512 MB或10240 MB。
    #[serde(rename = "diskSize")]
    pub disk_size: Option<i32>,
    /// 函数的环境变量，可以在运行环境中访问设置的环境变量。
    #[serde(rename = "environmentVariables")]
    pub environment_variables: std::collections::HashMap<String, String>,
    /// 函数的名称。只能包含字母、数字、下划线（_）和短划线（-），不能以数字、短划线（-）开头，长度范围为1~64个字符。
    #[serde(rename = "functionName")]
    pub function_name: String,
    /// 函数GPU配置。
    #[serde(rename = "gpuConfig")]
    pub gpu_config: Option<GPUConfig>,
    /// 函数执行的入口，具体格式和运行时相关。
    #[serde(rename = "handler")]
    pub handler: String,
    /// 实例最大并发度。
    #[serde(rename = "instanceConcurrency")]
    pub instance_concurrency: Option<i32>,
    /// 实例生命周期回调方法配置。
    #[serde(rename = "instanceLifecycleConfig")]
    pub instance_lifecycle_config: Option<InstanceLifecycleConfig>,
    /// 是否允许函数访问公网。默认值为true。
    #[serde(rename = "internetAccess")]
    pub internet_access: Option<bool>,
    /// 层的列表。多个层会按照数组下标从大到小的顺序进行合并，下标小的层的内容会覆盖下标大的层的同名文件。
    #[serde(rename = "layers")]
    pub layers: Vec<String>,
    /// 日志配置。函数产生的日志会被写入到配置的日志库中。
    #[serde(rename = "logConfig")]
    pub log_config: Option<LogConfig>,
    /// 函数的内存规格，单位为MB，内存大小为64 MB的倍数。最小值为128MB，最大值为32GB。同时cpu和memorySize（按GB算）比例要在1:1和1:4之间。
    #[serde(rename = "memorySize")]
    pub memory_size: Option<i32>,
    /// NAS配置。配置此参数后，函数可以访问指定的NAS资源。
    #[serde(rename = "nasConfig")]
    pub nas_config: Option<NASConfig>,
    /// OSS挂载配置。
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: Option<OSSMountConfig>,
    /// 用户授权给函数计算的RAM角色，设置后函数计算将扮演该角色生成临时访问凭证。在函数中可以使用该角色的临时访问凭证来访问指定的阿里云服务，例如OSS和OTS。
    #[serde(rename = "role")]
    pub role: Option<String>,
    /// 函数的运行时环境。目前支持的运行环境有：nodejs12, nodejs14, nodejs16, nodejs18, nodejs20, go1, python3, python3.9, python3.10, python3.12, java8, java11, php7.2, dotnetcore3.1, custom, custom.debian10, custom.debian11, custom.debian12, custom-container。
    #[serde(rename = "runtime")]
    pub runtime: String,
    /// 函数运行的超时时间，单位为秒，最小1秒，最大值为86400秒，默认值是3秒。函数超过这个时间后会被终止执行。
    #[serde(rename = "timeout")]
    pub timeout: Option<i32>,
    /// 链路追踪配置。当函数计算与链路追踪集成后，您可以记录请求在函数计算的耗时时间、查看函数的冷启动时间、记录函数内部时间的消耗等。
    #[serde(rename = "tracingConfig")]
    pub tracing_config: Option<TracingConfig>,
    /// VPC配置。配置此参数后，函数可以访问指定的VPC资源。
    #[serde(rename = "vpcConfig")]
    pub vpc_config: Option<VPCConfig>,
    /// 标签列表
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
    /// 是否禁止创建按量实例，功能开启后，不会创建按量实例，只能使用预留实例
    #[serde(rename = "disableOndemand")]
    pub disable_ondemand: Option<bool>,
    /// 函数计算调用请求的亲和策略，如需实现 MCP SSE协议的请求亲和，可设置为 MCP_SSE。如使用Cookie亲和，可设置为 GENERATED_COOKIE。如使用 Header亲和，可设置为 HEADER_FIELD。如不设置或设置为 NONE，则无亲和效果，按函数计算系统默认调度策略路由请求。
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: Option<String>,
    /// 是否允许 GPU 函数的预留实例常驻，启用该功能时，创建的函数实例不会被注入 STS token。
    #[serde(rename = "enableLongLiving")]
    pub enable_long_living: Option<bool>,
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: Option<String>,
    /// 实例隔离模式
    #[serde(rename = "instanceIsolationMode")]
    pub instance_isolation_mode: Option<CreateFunctionInputInstanceIsolationMode>,
    /// 当设置sessionAffinity亲和类型时，需设置相关的亲和配置。如MCP_SSE亲和需填充 MCPSSESessionAffinityConfig 配置。Cookie亲和需填充CookieSessionAffinityConfig配置，Header Field 亲和需填充HeaderFieldSessionAffinityConfig配置。
    #[serde(rename = "sessionAffinityConfig")]
    pub session_affinity_config: Option<String>,
    /// 实例延迟释放时间。
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: Option<i32>,
    /// 是否不注入 STS token，取值None/Env/Request/All
    /// None: 都注入
    /// Env: 环境变量不注入
    /// Request: 请求中不注入包括context/header
    /// All: 都不注入
    #[serde(rename = "disableInjectCredentials")]
    pub disable_inject_credentials: Option<CreateFunctionInputDisableInjectCredentials>,
    /// PolarFs配置。配置此参数后，函数可以访问指定的PolarFs资源。
    #[serde(rename = "polarFsConfig")]
    pub polar_fs_config: Option<PolarFsConfig>,
}

/// 创建层版本的请求参数。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CreateLayerVersionInput {
    /// 层的代码信息。
    #[serde(rename = "code")]
    pub code: Option<InputCodeLocation>,
    /// 层支持的运行时环境列表。
    #[serde(rename = "compatibleRuntime")]
    pub compatible_runtime: Vec<String>,
    /// 层版本的描述信息。
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 层的许可协议。
    #[serde(rename = "license")]
    pub license: Option<String>,
}

/// 创建会话的参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CreateSessionInput {
    /// Session生命周期
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: Option<i64>,
    /// Session 闲置过期时间
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: Option<i64>,
    /// NAS配置，配置后Session关联的实例可以访问指定NAS资源。
    #[serde(rename = "nasConfig")]
    pub nas_config: Option<NASConfig>,
    /// 可自定义会话ID。不配置时由服务端生成。若配置则将此配置作为会话ID。仅适用于HEADER_FIELD亲和模式，
    /// 格式规范：长度限制[0,64]，仅以 **a-zA-Z0-9_** 字符做首字符，非首字符可为 **a-zA-Z0-9_-**。
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    /// 默认值 False，表示在 SessionID 会话过期后，可携带相同SessionID继续发起请求，系统将视为新会话绑定新实例。当配置为 True，表示在 SessionID 会话过期后，不可复用 SessionID。
    #[serde(rename = "disableSessionIdReuse")]
    pub disable_session_id_reuse: Option<bool>,
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: Option<OSSMountConfig>,
    #[serde(rename = "polarFsConfig")]
    pub polar_fs_config: Option<PolarFsConfig>,
}

/// 创建触发器的请求参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CreateTriggerInput {
    /// 触发器的描述。
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 事件源（如OSS）调用函数所需的角色。
    #[serde(rename = "invocationRole")]
    pub invocation_role: Option<String>,
    /// 函数的版本或别名。
    #[serde(rename = "qualifier")]
    pub qualifier: Option<String>,
    /// 触发器事件源的Aliyun Resource Name。
    #[serde(rename = "sourceArn")]
    pub source_arn: Option<String>,
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

/// 创建VPC绑定请求参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CreateVpcBindingInput {
    /// VPC实例ID
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
}

/// 自定义域名信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct CustomDomain {
    /// 您的阿里云账号（主账号）ID。
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    /// 函数计算的API版本。
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    /// 权限认证配置
    #[serde(rename = "authConfig")]
    pub auth_config: Option<AuthConfig>,
    /// HTTPS证书的信息。
    #[serde(rename = "certConfig")]
    pub cert_config: Option<CertConfig>,
    /// 自定义域名的创建时间。
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    /// 域名。
    #[serde(rename = "domainName")]
    pub domain_name: Option<String>,
    /// 自定义域名上一次被更新的时间。
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// 域名支持的协议类型。HTTP：仅支持HTTP协议。HTTPS：仅支持HTTPS协议。HTTP,HTTPS：支持HTTP及HTTPS协议。
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
    /// 路由表：自定义域名访问时的PATH到Function的映射。
    #[serde(rename = "routeConfig")]
    pub route_config: Option<RouteConfig>,
    /// 子域名的数量。
    #[serde(rename = "subdomainCount")]
    pub subdomain_count: Option<String>,
    /// TLS配置信息。
    #[serde(rename = "tlsConfig")]
    pub tls_config: Option<TLSConfig>,
    /// Web应用防火墙配置信息。
    #[serde(rename = "wafConfig")]
    pub waf_config: Option<WAFConfig>,
}

/// 地域对象
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsOutputRegionsRegionItem {
    /// 地域ID
    #[serde(rename = "RegionId")]
    #[serde(default)]
    pub region_id: String,
    /// 地域名称
    #[serde(rename = "LocalName")]
    #[serde(default)]
    pub local_name: String,
}

/// 地域信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsOutputRegions {
    /// 地域信息集合
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: Vec<DescribeRegionsOutputRegionsRegionItem>,
}

/// 响应
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsOutput {
    /// 地域信息
    #[serde(rename = "Regions")]
    #[serde(default)]
    pub regions: DescribeRegionsOutputRegions,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScheduledPolicy {
    #[serde(rename = "name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: String,
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: String,
    #[serde(rename = "target")]
    #[serde(default)]
    pub target: i64,
    #[serde(rename = "scheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
    #[serde(rename = "timeZone")]
    #[serde(default)]
    pub time_zone: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScalingPolicy {
    #[serde(rename = "name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: String,
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: String,
    #[serde(rename = "metricType")]
    #[serde(default)]
    pub metric_type: String,
    #[serde(rename = "metricTarget")]
    #[serde(default)]
    pub metric_target: f32,
    #[serde(rename = "minInstances")]
    #[serde(default)]
    pub min_instances: i64,
    #[serde(rename = "maxInstances")]
    #[serde(default)]
    pub max_instances: i64,
    #[serde(rename = "timeZone")]
    #[serde(default)]
    pub time_zone: String,
}

/// 函数配置的层信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct FunctionLayer {
    /// 层版本的资源标识。
    #[serde(rename = "arn")]
    pub arn: Option<String>,
    /// 层的代码包大小，单位为Byte。
    #[serde(rename = "size")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct FunctionRestriction {
    #[serde(rename = "reason")]
    #[serde(default)]
    pub reason: String,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    pub last_modified_time: String,
    #[serde(rename = "disable")]
    #[serde(default)]
    pub disable: bool,
}

/// 函数配置信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct Function {
    /// 函数代码包的CRC-64值。
    #[serde(rename = "codeChecksum")]
    pub code_checksum: Option<String>,
    /// 系统返回的函数代码包的大小，单位Byte。
    #[serde(rename = "codeSize")]
    pub code_size: Option<i64>,
    /// 函数的CPU规格，单位为vCPU，为0.05 vCPU的倍数。最小值为0.05，最大值为16。同时cpu和memorySize（按GB算）比例要在1:1和1:4之间。
    #[serde(rename = "cpu")]
    pub cpu: Option<f32>,
    /// 函数的创建时间。
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    /// 自定义容器运行时的相关配置，成功配置后函数可以使用自定义容器镜像执行函数。code和customContainerConfig二选一。
    #[serde(rename = "customContainerConfig")]
    pub custom_container_config: Option<CustomContainerConfig>,
    /// 自定义DNS配置。
    #[serde(rename = "customDNS")]
    pub custom_dns: Option<CustomDNS>,
    /// 自定义运行时配置。
    #[serde(rename = "customRuntimeConfig")]
    pub custom_runtime_config: Option<CustomRuntimeConfig>,
    /// 函数的描述。
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 函数的磁盘规格，单位为MB，可选值为512 MB或10240 MB。
    #[serde(rename = "diskSize")]
    pub disk_size: Option<i32>,
    /// 函数的环境变量，可以在运行环境中访问设置的环境变量。
    #[serde(rename = "environmentVariables")]
    pub environment_variables: std::collections::HashMap<String, String>,
    /// 函数资源标识。
    #[serde(rename = "functionArn")]
    pub function_arn: Option<String>,
    /// 系统为每个函数生成的ID，全局唯一。
    #[serde(rename = "functionId")]
    pub function_id: Option<String>,
    /// 函数的名称。
    #[serde(rename = "functionName")]
    pub function_name: Option<String>,
    /// 函数GPU配置。
    #[serde(rename = "gpuConfig")]
    pub gpu_config: Option<GPUConfig>,
    /// 函数执行的入口，具体格式和运行时相关。
    #[serde(rename = "handler")]
    pub handler: Option<String>,
    /// 实例最大并发度
    #[serde(rename = "instanceConcurrency")]
    pub instance_concurrency: Option<i32>,
    /// 实例生命周期回调方法配置。
    #[serde(rename = "instanceLifecycleConfig")]
    pub instance_lifecycle_config: Option<InstanceLifecycleConfig>,
    /// 是否允许函数访问公网。默认值为true。
    #[serde(rename = "internetAccess")]
    pub internet_access: Option<bool>,
    /// 函数上一次被更新的时间。
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// 最近一次函数更新操作的状态，当函数新建成功时，此值为Successful，可选值：Successful、 Failed、 InProgress。
    #[serde(rename = "lastUpdateStatus")]
    pub last_update_status: Option<String>,
    /// 导致最近一次函数更新操作状态为当前值的原因。
    #[serde(rename = "lastUpdateStatusReason")]
    pub last_update_status_reason: Option<String>,
    /// 导致最近一次函数更新操作状态为当前值的原因的状态码。
    #[serde(rename = "lastUpdateStatusReasonCode")]
    pub last_update_status_reason_code: Option<String>,
    /// 层的列表。
    #[serde(rename = "layers")]
    pub layers: Vec<FunctionLayer>,
    /// 日志配置。函数产生的日志会被写入到配置的日志库中。
    #[serde(rename = "logConfig")]
    pub log_config: Option<LogConfig>,
    /// 函数的内存规格，单位为MB，内存大小为64 MB的倍数。最小值为128MB，最大值为32GB。同时cpu和memorySize（按GB算）比例要在1:1和1:4之间。
    #[serde(rename = "memorySize")]
    pub memory_size: Option<i32>,
    /// NAS配置。配置此参数后，函数可以访问指定的NAS资源。
    #[serde(rename = "nasConfig")]
    pub nas_config: Option<NASConfig>,
    /// OSS挂载配置。
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: Option<OSSMountConfig>,
    /// 用户授权给函数计算的RAM角色，设置后函数计算将扮演该角色生成临时访问凭证。在函数中可以使用该角色的临时访问凭证来访问指定的阿里云服务，例如OSS和OTS。
    #[serde(rename = "role")]
    pub role: Option<String>,
    /// 函数的运行时环境。目前支持的运行环境有：nodejs12, nodejs14, nodejs16, nodejs18, nodejs20, go1, python3, python3.9, python3.10, python3.12, java8, java11, php7.2, dotnetcore3.1, custom, custom.debian10, custom.debian11, custom.debian12, custom-container。
    #[serde(rename = "runtime")]
    pub runtime: Option<String>,
    /// 函数当前的状态。
    #[serde(rename = "state")]
    pub state: Option<String>,
    /// 函数处于当前状态的原因。
    #[serde(rename = "stateReason")]
    pub state_reason: Option<String>,
    /// 函数处于当前状态的原因的状态码。
    #[serde(rename = "stateReasonCode")]
    pub state_reason_code: Option<String>,
    /// 函数运行的超时时间，单位为秒，最小1秒，最大值为86400秒，默认值是3秒。函数超过这个时间后会被终止执行。
    #[serde(rename = "timeout")]
    pub timeout: Option<i32>,
    /// 链路追踪配置。当函数计算与链路追踪集成后，您可以记录请求在函数计算的耗时时间、查看函数的冷启动时间、记录函数内部时间的消耗等。
    #[serde(rename = "tracingConfig")]
    pub tracing_config: Option<TracingConfig>,
    /// VPC配置。配置此参数后，函数可以访问指定的VPC资源。
    #[serde(rename = "vpcConfig")]
    pub vpc_config: Option<VPCConfig>,
    /// 标签列表
    #[serde(rename = "tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// 是否禁止创建按量实例，功能开启后，不会创建按量实例，只能使用预留实例
    #[serde(rename = "disableOndemand")]
    #[serde(default)]
    pub disable_ondemand: bool,
    #[serde(rename = "invocationRestriction")]
    #[serde(default)]
    pub invocation_restriction: FunctionRestriction,
    /// 函数计算调用请求的亲和策略，如需实现 MCP SSE协议的请求亲和，可设置为 MCP_SSE。如使用Cookie亲和，可设置为 GENERATED_COOKIE。如使用 Header亲和，可设置为 HEADER_FIELD。如不设置或设置为 NONE，则无亲和效果，按函数计算系统默认调度策略路由请求。
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: Option<String>,
    /// 当设置sessionAffinity亲和类型时，需设置相关的亲和配置。如MCP_SSE亲和需填充 MCPSSESessionAffinityConfig 配置。Cookie亲和需填充CookieSessionAffinityConfig配置，Header Field 亲和需填充HeaderFieldSessionAffinityConfig配置。
    #[serde(rename = "enableLongLiving")]
    #[serde(default)]
    pub enable_long_living: bool,
    /// 资源组 ID
    #[serde(rename = "resourceGroupId")]
    #[serde(default)]
    pub resource_group_id: String,
    /// 实例隔离模式
    #[serde(rename = "instanceIsolationMode")]
    #[serde(default)]
    pub instance_isolation_mode: FunctionInstanceIsolationMode,
    /// 当设置sessionAffinity亲和类型时，需设置相关的亲和配置。如MCP_SSE亲和需填充 MCPSSESessionAffinityConfig 配置。Cookie亲和需填充CookieSessionAffinityConfig配置，Header Field 亲和需填充HeaderFieldSessionAffinityConfig配置。
    #[serde(rename = "sessionAffinityConfig")]
    #[serde(default)]
    pub session_affinity_config: String,
    /// 实例延迟释放时间
    #[serde(rename = "idleTimeout")]
    #[serde(default)]
    pub idle_timeout: i32,
    /// 是否不注入 STS token，取值None/Env/Request/All
    /// None: 都注入
    /// Env: 环境变量不注入
    /// Request: 请求中不注入包括context/header
    /// All: 都不注入
    #[serde(rename = "disableInjectCredentials")]
    #[serde(default)]
    pub disable_inject_credentials: FunctionDisableInjectCredentials,
    /// PolarFs配置。配置此参数后，函数可以访问指定的PolarFs资源。
    #[serde(rename = "polarFsConfig")]
    #[serde(default)]
    pub polar_fs_config: PolarFsConfig,
}

/// HTTP 触发器配置
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct HTTPTrigger {
    /// 公网域名地址。在互联网可以通过HTTP协议或者HTTPS协议访问HTTP Trigger。
    #[serde(rename = "urlInternet")]
    pub url_internet: Option<String>,
    /// 私网域名地址。在VPC可以通过HTTP协议或者HTTPS协议访问HTTP Trigger。
    #[serde(rename = "urlIntranet")]
    pub url_intranet: Option<String>,
}

/// 函数实例信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct InstanceInfo {
    /// 实例ID。
    #[serde(rename = "instanceId")]
    pub instance_id: Option<String>,
    /// 实例所属函数的版本。如果是LATEST别名下的函数实例，则返回版本号为0。
    #[serde(rename = "versionId")]
    #[serde(default)]
    pub version_id: String,
    #[serde(rename = "qualifier")]
    #[serde(default)]
    pub qualifier: String,
    #[serde(rename = "status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "createdTimeMs")]
    #[serde(default)]
    pub created_time_ms: i64,
    #[serde(rename = "destroyedTimeMs")]
    #[serde(default)]
    pub destroyed_time_ms: i64,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
}

/// 代码包配置
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct OutputCodeLocation {
    /// 代码包的地址。
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// 代码包的类型。
    #[serde(rename = "repositoryType")]
    pub repository_type: Option<String>,
}

/// 层的详细信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct Layer {
    /// 层的权限。取值0代表私有，取值1代表公有。官方公共层默认为公有，自定义层可以设置为私有或者公有。
    #[serde(rename = "acl")]
    pub acl: Option<String>,
    /// 层的代码包信息
    #[serde(rename = "code")]
    pub code: Option<OutputCodeLocation>,
    /// 层代码包的crc64校验码，根据ECMA-182标准计算得出。
    #[serde(rename = "codeChecksum")]
    pub code_checksum: Option<String>,
    /// 层的代码包大小，单位为Byte。
    #[serde(rename = "codeSize")]
    pub code_size: Option<i64>,
    /// 层支持的运行时环境列表。
    #[serde(rename = "compatibleRuntime")]
    pub compatible_runtime: Vec<String>,
    /// 层版本的创建时间。
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    /// 版本的描述信息。
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 层的名称。
    #[serde(rename = "layerName")]
    pub layer_name: String,
    /// 层版本资源的名称，格式为 acs:fc:{region}:{accountID}:layers/{layerName}/versions/{layerVersion}.
    #[serde(rename = "layerVersionArn")]
    pub layer_version_arn: Option<String>,
    /// 许可协议。
    #[serde(rename = "license")]
    pub license: Option<String>,
    /// 层的版本。
    #[serde(rename = "version")]
    pub version: Option<i32>,
}

/// 函数别名列表
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAliasesOutput {
    /// 别名列表
    #[serde(rename = "aliases")]
    pub aliases: Vec<Alias>,
    /// 下一个版本名
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

/// 获取函数异步配置列表的响应信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAsyncInvokeConfigOutput {
    /// 异步调用配置列表。
    #[serde(rename = "configs")]
    pub configs: Vec<AsyncConfig>,
    /// 用来返回更多结果。第一次查询不需要提供这个参数，后续查询的Token从返回结果中获取。
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

/// 列举异步任务信息响应内容。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAsyncTaskOutput {
    /// 分页标记，用来返回更多结果。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    /// 异步任务信息列表。
    #[serde(rename = "tasks")]
    pub tasks: Vec<AsyncTask>,
}

/// 获取函数的并发配置列表信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListConcurrencyConfigsOutput {
    /// 并发配置列表。
    #[serde(rename = "configs")]
    pub configs: Vec<ConcurrencyConfig>,
    /// 用来返回更多的查询结果。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

/// 获取自定义域名列表的响应信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListCustomDomainOutput {
    /// 自定义域名列表
    #[serde(rename = "customDomains")]
    pub custom_domains: Vec<CustomDomain>,
    /// 当符合查询条件的数据未读取完时，服务端会返回nextToken，此时可以使用nextToken继续读取后面的数据。第一次查询不需要提供这个参数。
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

/// 获取函数列表响应信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListFunctionsOutput {
    /// 函数信息列表
    #[serde(rename = "functions")]
    pub functions: Vec<Function>,
    /// 用来返回更多的查询结果。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

/// 查询返回的实例信息列表
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListInstancesOutput {
    /// 实例列表信息
    #[serde(rename = "instances")]
    pub instances: Vec<InstanceInfo>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
}

/// 获取指定层的版本列表。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListLayerVersionOutput {
    /// 层版本的列表。
    #[serde(rename = "layers")]
    pub layers: Vec<Layer>,
    /// 剩余列表的起始版本名，用来返回更多结果。
    #[serde(rename = "nextVersion")]
    pub next_version: Option<i32>,
}

/// 获取层列表
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListLayersOutput {
    /// 层的列表。
    #[serde(rename = "layers")]
    pub layers: Vec<Layer>,
    /// 剩余列表的起始层名，用来返回更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

/// 定时策略配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScheduledAction {
    /// 策略失效时间。
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    /// 策略名称。
    #[serde(rename = "name")]
    pub name: String,
    /// 定时配置。
    #[serde(rename = "scheduleExpression")]
    pub schedule_expression: String,
    /// 策略生效时间。
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// 预留的目标资源个数。
    #[serde(rename = "target")]
    pub target: i64,
    /// 时区。时区参数为空时，startTime、endTime和scheduleExpression的时间需为UTC格式。
    #[serde(rename = "timeZone")]
    #[serde(default)]
    pub time_zone: String,
}

/// 指标追踪伸缩策略配置
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TargetTrackingPolicy {
    /// 策略结束时间（UTC）。
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
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
    pub start_time: Option<String>,
    /// 时区。时区参数为空时，startTime和endTime的时间需为UTC格式。
    #[serde(rename = "timeZone")]
    #[serde(default)]
    pub time_zone: String,
}

/// 函数预留配置
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ProvisionConfig {
    /// 定时策略配置。
    #[serde(rename = "scheduledActions")]
    pub scheduled_actions: Vec<ScheduledAction>,
    /// 预留实例创建失败时的错误信息。
    #[serde(rename = "currentError")]
    pub current_error: Option<String>,
    /// 所有指标追踪伸缩策略和定时策略失效时的默认资源个数。
    #[serde(rename = "defaultTarget")]
    #[serde(default)]
    pub default_target: i64,
    /// 实际资源个数。
    #[serde(rename = "current")]
    pub current: Option<i64>,
    /// 是否始终分配CPU给函数实例。
    #[serde(rename = "alwaysAllocateCPU")]
    pub always_allocate_cpu: Option<bool>,
    /// 是否始终分配GPU给函数实例。
    #[serde(rename = "alwaysAllocateGPU")]
    pub always_allocate_gpu: Option<bool>,
    /// 指标追踪伸缩策略配置。
    #[serde(rename = "targetTrackingPolicies")]
    pub target_tracking_policies: Vec<TargetTrackingPolicy>,
    /// 函数的资源描述
    #[serde(rename = "functionArn")]
    pub function_arn: Option<String>,
    /// 当前目标资源个数，如果存在指标追踪伸缩策略或定时策略，为策略计算的资源个数，否则为默认预留实例数。
    ///
    ///
    /// > 与 defaultTarget 有什么区别？\
    /// > 假设配置预留实例数为1后，新增了定时伸缩策略，设置某个时间段内的预留实例数为5。
    /// > - 在定时伸缩策略**生效期间**，target 与 defaultTarget 分别为 5 和 1。
    /// >-  在定时伸缩策略**失效期间**，target 与 defaultTarget 都为 1。
    #[serde(rename = "target")]
    pub target: Option<i64>,
}

/// 获取函数预留配置信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListProvisionConfigsOutput {
    /// 下次查询的起始Token。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    /// 函数预留配置列表。
    #[serde(rename = "provisionConfigs")]
    pub provision_configs: Vec<ProvisionConfig>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ScalingConfigStatus {
    #[serde(rename = "functionArn")]
    #[serde(default)]
    pub function_arn: String,
    #[serde(rename = "residentPoolId")]
    #[serde(default)]
    pub resident_pool_id: String,
    #[serde(rename = "minInstances")]
    #[serde(default)]
    pub min_instances: i64,
    #[serde(rename = "currentInstances")]
    #[serde(default)]
    pub current_instances: i64,
    #[serde(rename = "currentError")]
    #[serde(default)]
    pub current_error: String,
    #[serde(rename = "targetInstances")]
    #[serde(default)]
    pub target_instances: i64,
    #[serde(rename = "enableOnDemandScaling")]
    #[serde(default)]
    pub enable_on_demand_scaling: bool,
    #[serde(rename = "scheduledPolicies")]
    #[serde(default)]
    pub scheduled_policies: Vec<ScheduledPolicy>,
    #[serde(rename = "horizontalScalingPolicies")]
    #[serde(default)]
    pub horizontal_scaling_policies: Vec<ScalingPolicy>,
    #[serde(rename = "enableMixMode")]
    #[serde(default)]
    pub enable_mix_mode: bool,
    #[serde(rename = "requestDispatchPolicy")]
    #[serde(default)]
    pub request_dispatch_policy: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListScalingConfigsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    pub next_token: String,
    #[serde(rename = "scalingConfigs")]
    #[serde(default)]
    pub scaling_configs: Vec<ScalingConfigStatus>,
}

/// 会话信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct Session {
    /// 函数会话唯一标识
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    /// Session所属函数名称
    #[serde(rename = "functionName")]
    #[serde(default)]
    pub function_name: String,
    /// 会话亲和类型
    #[serde(rename = "sessionAffinityType")]
    #[serde(default)]
    pub session_affinity_type: String,
    /// Session 生命周期最大值
    #[serde(rename = "sessionTTLInSeconds")]
    #[serde(default)]
    pub session_ttl_in_seconds: i64,
    /// Session 闲置过期时间
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    #[serde(default)]
    pub session_idle_timeout_in_seconds: i64,
    /// Session 的创建时间
    #[serde(rename = "createdTime")]
    #[serde(default)]
    pub created_time: String,
    /// Session上一次被更新的时间。
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    pub last_modified_time: String,
    /// Session 状态：Active 代表 Session 有效，Expired代表 Session已过期
    #[serde(rename = "sessionStatus")]
    #[serde(default)]
    pub session_status: String,
    /// Session关联的函数实例ID
    #[serde(rename = "containerId")]
    #[serde(default)]
    pub container_id: String,
    /// 客户创建 Session 时传入的 Qualifier，如未传则为默认值 LATEST
    #[serde(rename = "qualifier")]
    #[serde(default)]
    pub qualifier: String,
    /// NAS配置，配置后Session关联的实例可以访问指定NAS资源。
    #[serde(rename = "nasConfig")]
    #[serde(default)]
    pub nas_config: NASConfig,
    /// 默认值 False，表示在 SessionID 会话过期后，可携带相同SessionID继续发起请求，系统将视为新会话绑定新实例。当配置为 True，表示在 SessionID 会话过期后，不可复用 SessionID。
    #[serde(rename = "disableSessionIdReuse")]
    #[serde(default)]
    pub disable_session_id_reuse: bool,
    #[serde(rename = "ossMountConfig")]
    #[serde(default)]
    pub oss_mount_config: OSSMountConfig,
    #[serde(rename = "polarFsConfig")]
    #[serde(default)]
    pub polar_fs_config: PolarFsConfig,
}

/// 函数会话列表
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListSessionsOutput {
    /// 下一次查询会话列表的起始位置。
    #[serde(rename = "nextToken")]
    #[serde(default)]
    pub next_token: String,
    /// 会话列表信息
    #[serde(rename = "sessions")]
    #[serde(default)]
    pub sessions: Vec<Session>,
}

/// 资源标签信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct TagResource {
    /// 阿里云资源描述符。
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    /// 资源类型名称。
    ///
    /// 函数计算3.0中的函数类型：ALIYUN::FC::FUNCTION，简写：function。
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,
    /// 标签键。
    #[serde(rename = "TagKey")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    pub tag_value: Option<String>,
}

/// 查询绑定指定标签的资源信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListTagResourcesOutput {
    /// 用来返回更多结果。第一次查询不需要提供这个参数，后续查询的Token从返回结果中获取。
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
    /// 查询到的资源和标签的信息。
    #[serde(rename = "TagResources")]
    pub tag_resources: Vec<TagResource>,
}

/// 触发器的配置信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct Trigger {
    /// 触发器的创建时间。
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    /// 触发器的描述。
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// HTTP 触发器信息。
    #[serde(rename = "httpTrigger")]
    pub http_trigger: Option<HTTPTrigger>,
    /// 事件源（如OSS）调用函数所需的角色。
    #[serde(rename = "invocationRole")]
    pub invocation_role: Option<String>,
    /// 触发器的上次修改时间。
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// 函数的版本或别名。
    #[serde(rename = "qualifier")]
    pub qualifier: Option<String>,
    /// 触发器事件源的Aliyun Resource Name。
    #[serde(rename = "sourceArn")]
    pub source_arn: Option<String>,
    /// 触发器的状态。
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// 函数的资源标识。
    #[serde(rename = "targetArn")]
    pub target_arn: Option<String>,
    /// 触发器配置，针对不同类型的触发器，配置有所不同。
    #[serde(rename = "triggerConfig")]
    pub trigger_config: Option<String>,
    /// 触发器的唯一ID。
    #[serde(rename = "triggerId")]
    pub trigger_id: Option<String>,
    /// 触发器的名称。要求只能包含字母、数字、下划线(_)和短划线(-)。不能以数字、短划线(-)开头，长度限制为1~128个字符。
    #[serde(rename = "triggerName")]
    pub trigger_name: Option<String>,
    /// 触发器的类型。
    #[serde(rename = "triggerType")]
    pub trigger_type: Option<String>,
}

/// 获取触发器列表的响应结构体。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListTriggersOutput {
    /// 下一个触发器的名称。用来返回更多的查询结果。如果这个值没有返回，则说明没有更多结果。
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    /// 触发器列表。
    #[serde(rename = "triggers")]
    pub triggers: Vec<Trigger>,
}

/// 版本信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct Version {
    /// 创建时间
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    /// 版本描述信息
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 更新时间
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// 版本ID
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,
}

/// 列出版本的请求参数
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListVersionsOutput {
    /// 版本排序方式
    #[serde(rename = "direction")]
    pub direction: Option<String>,
    /// 下一个版本ID
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    /// 版本ID列表
    #[serde(rename = "versions")]
    pub versions: Vec<Version>,
}

/// 列出服务/函数下的全部VPC绑定的请求参数
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListVpcBindingsOutput {
    /// VPC实例ID列表
    #[serde(rename = "vpcIds")]
    pub vpc_ids: Vec<String>,
}

/// 函数代码包信息
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct OutputFuncCode {
    /// 函数代码包的CRC64值。
    #[serde(rename = "checksum")]
    pub checksum: Option<String>,
    /// 函数代码包的URL。
    #[serde(rename = "url")]
    pub url: Option<String>,
}

/// 发布版本的请求参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct PublishVersionInput {
    /// 版本描述信息
    #[serde(rename = "description")]
    pub description: Option<String>,
}

/// 函数异步配置信息
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct PutAsyncInvokeConfigInput {
    /// 是否开启异步任务（非必填）
    #[serde(rename = "asyncTask")]
    pub async_task: Option<bool>,
    /// 异步调用目标的配置结构体（非必填）。
    #[serde(rename = "destinationConfig")]
    pub destination_config: Option<DestinationConfig>,
    /// 消息最大存活时长（非必填），取值范围[1,604800]，默认为86400，单位为秒。
    #[serde(rename = "maxAsyncEventAgeInSeconds")]
    pub max_async_event_age_in_seconds: Option<i64>,
    /// 异步调用失败后的最大重试次数，非必填。取值范围[0,8]。当您未进行配置时，系统内部默认重试次数为3。
    #[serde(rename = "maxAsyncRetryAttempts")]
    pub max_async_retry_attempts: Option<i64>,
}

/// 设置函数的并发配置
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct PutConcurrencyInput {
    /// 预留并发，函数预留账号并发的一部份，其他函数不可以使用这部份并发。预留并发包括预留实例和按量实例的总并发。
    #[serde(rename = "reservedConcurrency")]
    pub reserved_concurrency: i64,
}

/// 申请或更新预留资源的配置。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct PutProvisionConfigInput {
    /// 定时策略配置。
    #[serde(rename = "scheduledActions")]
    pub scheduled_actions: Vec<ScheduledAction>,
    /// 默认最小实例数，取值范围为[0,10000]。
    /// > - 未配置指标追踪弹性策略或定时弹性策略时，当前最小实例数等于您配置最小实例数。
    /// > - 如果您配置了多条最小实例数弹性策略，系统会计算每条策略触发时的最小实例数，并取当前时间有效的弹性策略中最小实例数的最大值作为当前最小实例数。
    #[serde(rename = "defaultTarget")]
    pub default_target: Option<i64>,
    /// 是否始终分配CPU，默认为true。
    #[serde(rename = "alwaysAllocateCPU")]
    pub always_allocate_cpu: Option<bool>,
    /// 是否始终分配GPU，默认为true。
    #[serde(rename = "alwaysAllocateGPU")]
    pub always_allocate_gpu: Option<bool>,
    /// 指标追踪伸缩策略配置。
    #[serde(rename = "targetTrackingPolicies")]
    pub target_tracking_policies: Vec<TargetTrackingPolicy>,
    /// ><notice>建议不再使用该参数，请使用 defaultTarget 参数。 </notice>
    /// 预留的目标资源个数。取值范围为[0,10000]。
    #[serde(rename = "target")]
    pub target: i64,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct PutScalingConfigInput {
    #[serde(rename = "residentPoolId")]
    pub resident_pool_id: Option<String>,
    #[serde(rename = "minInstances")]
    pub min_instances: Option<i64>,
    #[serde(rename = "enableOnDemandScaling")]
    pub enable_on_demand_scaling: Option<bool>,
    #[serde(rename = "scheduledPolicies")]
    pub scheduled_policies: Vec<ScheduledPolicy>,
    #[serde(rename = "horizontalScalingPolicies")]
    pub horizontal_scaling_policies: Vec<ScalingPolicy>,
    #[serde(rename = "enableMixMode")]
    pub enable_mix_mode: Option<bool>,
    #[serde(rename = "requestDispatchPolicy")]
    pub request_dispatch_policy: Option<String>,
}

/// 给指定资源打标签
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct TagResourcesInput {
    /// 资源ID列表
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 资源类型
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,
    /// 标签列表
    #[serde(rename = "Tag")]
    pub tag: Vec<Tag>,
}

/// 更新别名的请求参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UpdateAliasInput {
    /// 灰度版本权重
    #[serde(rename = "additionalVersionWeight")]
    pub additional_version_weight: std::collections::HashMap<String, f64>,
    /// 别名的描述信息
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 别名指向的版本
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,
}

/// 更新自定义域名的请求信息
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UpdateCustomDomainInput {
    /// 权限认证配置
    #[serde(rename = "authConfig")]
    pub auth_config: Option<AuthConfig>,
    /// HTTPS证书的信息。
    #[serde(rename = "certConfig")]
    pub cert_config: Option<CertConfig>,
    /// 域名支持的协议类型。HTTP：仅支持HTTP协议。HTTPS：仅支持HTTPS协议。HTTP,HTTPS：支持HTTP及HTTPS协议。
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
    /// 路由表：自定义域名访问时的PATH到Function的映射。
    #[serde(rename = "routeConfig")]
    pub route_config: Option<RouteConfig>,
    /// TLS配置信息。
    #[serde(rename = "tlsConfig")]
    pub tls_config: Option<TLSConfig>,
    /// Web应用防火墙配置信息。
    #[serde(rename = "wafConfig")]
    pub waf_config: Option<WAFConfig>,
}

/// 函数更新请求参数。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UpdateFunctionInput {
    /// 函数代码ZIP包。code和customContainerConfig二选一。
    #[serde(rename = "code")]
    pub code: Option<InputCodeLocation>,
    /// 函数的CPU规格，单位为vCPU，取值需要为0.05 vCPU的倍数。
    #[serde(rename = "cpu")]
    pub cpu: Option<f32>,
    /// 自定义容器运行时的相关配置，成功配置后函数可以使用自定义容器镜像执行函数。code和customContainerConfig二选一。
    #[serde(rename = "customContainerConfig")]
    pub custom_container_config: Option<CustomContainerConfig>,
    /// 自定义DNS配置。
    #[serde(rename = "customDNS")]
    pub custom_dns: Option<CustomDNS>,
    /// 自定义运行时配置。
    #[serde(rename = "customRuntimeConfig")]
    pub custom_runtime_config: Option<CustomRuntimeConfig>,
    /// 函数的描述。
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 函数的磁盘规格，单位为MB，可选值为512 MB或10240 MB。
    #[serde(rename = "diskSize")]
    pub disk_size: Option<i32>,
    /// 函数的环境变量，可以在运行环境中访问设置的环境变量。
    #[serde(rename = "environmentVariables")]
    pub environment_variables: std::collections::HashMap<String, String>,
    /// 函数GPU配置。
    #[serde(rename = "gpuConfig")]
    pub gpu_config: Option<GPUConfig>,
    /// 函数执行的入口，具体格式和运行时相关。
    #[serde(rename = "handler")]
    pub handler: Option<String>,
    /// 实例最大并发度。
    #[serde(rename = "instanceConcurrency")]
    pub instance_concurrency: Option<i32>,
    /// 实例生命周期回调方法配置。
    #[serde(rename = "instanceLifecycleConfig")]
    pub instance_lifecycle_config: Option<InstanceLifecycleConfig>,
    /// 是否允许访问公网。
    #[serde(rename = "internetAccess")]
    pub internet_access: Option<bool>,
    /// 层的列表。多个层会按照数组下标从大到小的顺序进行合并，下标小的层的内容会覆盖下标大的层的同名文件。
    #[serde(rename = "layers")]
    pub layers: Vec<String>,
    /// 日志配置。函数产生的日志会被写入到配置的日志库中。
    #[serde(rename = "logConfig")]
    pub log_config: Option<LogConfig>,
    /// 函数的内存规格，单位为MB，内存大小为64 MB的倍数。不同的函数实例类型，内存规格存在差异。
    #[serde(rename = "memorySize")]
    pub memory_size: Option<i32>,
    /// NAS配置。配置此参数后，函数可以访问指定的NAS资源。
    #[serde(rename = "nasConfig")]
    pub nas_config: Option<NASConfig>,
    /// OSS挂载配置。
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: Option<OSSMountConfig>,
    /// 授予函数计算所需权限的RAM角色，使用场景包含：1. 把函数产生的日志发送到您的日志库中。2. 为函数在执行过程中访问其他云资源生成的临时访问令牌。
    #[serde(rename = "role")]
    pub role: Option<String>,
    /// 函数的运行时环境。
    #[serde(rename = "runtime")]
    pub runtime: Option<String>,
    /// 函数运行的超时时间，单位为秒，最小1秒，默认3秒。函数超过这个时间后会被终止执行。
    #[serde(rename = "timeout")]
    pub timeout: Option<i32>,
    /// 链路追踪配置。当函数计算与链路追踪集成后，您可以记录请求在函数计算的耗时时间、查看函数的冷启动时间、记录函数内部时间的消耗等。
    #[serde(rename = "tracingConfig")]
    pub tracing_config: Option<TracingConfig>,
    /// VPC配置。配置此参数后，函数可以访问指定的VPC资源。
    #[serde(rename = "vpcConfig")]
    pub vpc_config: Option<VPCConfig>,
    /// 是否禁止创建按量实例，功能开启后，不会创建按量实例，只能使用预留实例
    #[serde(rename = "disableOndemand")]
    pub disable_ondemand: Option<bool>,
    /// 函数计算调用请求的亲和策略，如需实现 MCP SSE协议的请求亲和，可设置为 MCP_SSE。如使用Cookie亲和，可设置为 GENERATED_COOKIE。如使用 Header亲和，可设置为 HEADER_FIELD。如不设置或设置为 NONE，则无亲和效果，按函数计算系统默认调度策略路由请求。
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: Option<String>,
    /// 是否允许 GPU 函数的预留实例常驻，启用该功能时，创建的函数实例不会被注入 STS token。
    #[serde(rename = "enableLongLiving")]
    pub enable_long_living: Option<bool>,
    /// 实例隔离模式
    #[serde(rename = "instanceIsolationMode")]
    pub instance_isolation_mode: Option<UpdateFunctionInputInstanceIsolationMode>,
    /// 当设置sessionAffinity亲和类型时，需设置相关的亲和配置。如MCP_SSE亲和需填充 MCPSSESessionAffinityConfig 配置。Cookie亲和需填充CookieSessionAffinityConfig配置，Header Field 亲和需填充HeaderFieldSessionAffinityConfig配置。
    #[serde(rename = "sessionAffinityConfig")]
    pub session_affinity_config: Option<String>,
    /// 实例延迟释放时间
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: Option<i32>,
    /// 是否不注入 STS token，取值None/Env/Request/All
    /// None: 都注入
    /// Env: 环境变量不注入
    /// Request: 请求中不注入包括context/header
    /// All: 都不注入
    #[serde(rename = "disableInjectCredentials")]
    pub disable_inject_credentials: Option<UpdateFunctionInputDisableInjectCredentials>,
    /// PolarFs配置。配置此参数后，函数可以访问指定的PolarFs资源。
    #[serde(rename = "polarFsConfig")]
    pub polar_fs_config: Option<PolarFsConfig>,
}

/// 更新会话的参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UpdateSessionInput {
    /// Session生命周期
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: Option<i64>,
    /// Session 闲置过期时间
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: Option<i64>,
    /// 默认值 False，表示在 SessionID 会话过期后，可携带相同SessionID继续发起请求，系统将视为新会话绑定新实例。当配置为 True，表示在 SessionID 会话过期后，不可复用 SessionID。
    #[serde(rename = "disableSessionIdReuse")]
    pub disable_session_id_reuse: Option<bool>,
}

/// 更新触发器的请求参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UpdateTriggerInput {
    /// 触发器的描述。
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 事件源（如OSS）调用函数所需的角色。
    #[serde(rename = "invocationRole")]
    pub invocation_role: Option<String>,
    /// 函数的版本或别名。
    #[serde(rename = "qualifier")]
    pub qualifier: Option<String>,
    /// 触发器配置，针对不同类型的触发器，配置有所不同。
    #[serde(rename = "triggerConfig")]
    pub trigger_config: Option<String>,
}

/// 禁止调用的请求参数
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct FunctionInvocationbody {
    /// 禁止函数调用的原因
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    /// 是否立即终止正在处理的所有请求
    #[serde(rename = "abortOngoingRequest")]
    pub abort_ongoing_request: Option<bool>,
}

/// 标签信息。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ResourcesTag {
    /// 标签键。
    ///
    /// 最多支持64个字符，不能以`aliyun`和`acs:`开头，不能包含`http://`或者`https://`。
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// 标签值。
    ///
    /// 标签值最多支持128个字符，可以为空字符串。
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateFunctionInputInstanceIsolationMode {
    #[serde(rename = "SHARE")]
    Share,
    #[serde(rename = "REQUEST_EXCLUSIVE")]
    RequestExclusive,
    #[serde(rename = "SESSION_EXCLUSIVE")]
    SessionExclusive,
}

impl Default for CreateFunctionInputInstanceIsolationMode {
    fn default() -> Self {
        Self::Share
    }
}

impl CreateFunctionInputInstanceIsolationMode {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Share => "SHARE",
            Self::RequestExclusive => "REQUEST_EXCLUSIVE",
            Self::SessionExclusive => "SESSION_EXCLUSIVE",
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

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FunctionInstanceIsolationMode {
    #[serde(rename = "SHARE")]
    Share,
    #[serde(rename = "SESSION_EXCLUSIVE")]
    SessionExclusive,
    #[serde(rename = "REQUEST_EXCLUSIVE")]
    RequestExclusive,
}

impl Default for FunctionInstanceIsolationMode {
    fn default() -> Self {
        Self::Share
    }
}

impl FunctionInstanceIsolationMode {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Share => "SHARE",
            Self::SessionExclusive => "SESSION_EXCLUSIVE",
            Self::RequestExclusive => "REQUEST_EXCLUSIVE",
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

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UpdateFunctionInputInstanceIsolationMode {
    #[serde(rename = "SHARE")]
    Share,
    #[serde(rename = "REQUEST_EXCLUSIVE")]
    RequestExclusive,
    #[serde(rename = "SESSION_EXCLUSIVE")]
    SessionExclusive,
}

impl Default for UpdateFunctionInputInstanceIsolationMode {
    fn default() -> Self {
        Self::Share
    }
}

impl UpdateFunctionInputInstanceIsolationMode {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Share => "SHARE",
            Self::RequestExclusive => "REQUEST_EXCLUSIVE",
            Self::SessionExclusive => "SESSION_EXCLUSIVE",
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

#[derive(Debug, Default, serde::Deserialize)]
pub struct EnableFunctionInvocationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 是否成功允许调用
    #[serde(rename = "success")]
    #[serde(default)]
    pub success: bool,
}
/// 请谨慎对生产环境的函数调用该接口，避免由于函数被禁止调用导致业务受损。
#[derive(Debug, Default, serde::Deserialize)]
pub struct DisableFunctionInvocationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 是否成功禁止调用
    #[serde(rename = "success")]
    #[serde(default)]
    pub success: bool,
}

crate::impl_to_code_message!(
    EnableFunctionInvocationResponse,
    DisableFunctionInvocationResponse
);

crate::impl_default_to_code_message!(
    Alias,
    AsyncConfig,
    AsyncTask,
    ChangeResourceGroupOutput,
    ConcurrencyConfig,
    CustomDomain,
    DescribeRegionsOutput,
    Function,
    Layer,
    ListAliasesOutput,
    ListAsyncInvokeConfigOutput,
    ListAsyncTaskOutput,
    ListConcurrencyConfigsOutput,
    ListCustomDomainOutput,
    ListFunctionsOutput,
    ListInstancesOutput,
    ListLayerVersionOutput,
    ListLayersOutput,
    ProvisionConfig,
    ListProvisionConfigsOutput,
    ScalingConfigStatus,
    ListScalingConfigsOutput,
    Session,
    ListSessionsOutput,
    ListTagResourcesOutput,
    Trigger,
    ListTriggersOutput,
    Version,
    ListVersionsOutput,
    ListVpcBindingsOutput,
    OutputFuncCode
);
