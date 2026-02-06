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
    body: Option<Vec<u8>>,
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

    type Body = crate::OctetStream;

    type ResponseWrap = crate::JsonResponseWrap<ChangeResourceGroupOutput>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::OctetStream(self.body.unwrap_or_default())
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
    body: Vec<u8>,
}

impl sealed::Bound for CreateCustomDomain {}

impl CreateCustomDomain {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }
}

impl crate::Request for CreateCustomDomain {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateCustomDomain";
    const URL_PATH: &'static str = "/2023-03-30/custom-domains";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::JsonResponseWrap<CustomDomain>;

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
    body: Vec<u8>,
}

impl sealed::Bound for UpdateCustomDomain {}

impl UpdateCustomDomain {
    pub fn new(domain_name: impl Into<String>) -> Self {
        Self {
            domain_name: domain_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for UpdateCustomDomain {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UpdateCustomDomain";
    const URL_PATH: &'static str = "/2023-03-30/custom-domains/{domainName}";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
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
    body: Vec<u8>,
}

impl sealed::Bound for PublishFunctionVersion {}

impl PublishFunctionVersion {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for PublishFunctionVersion {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "PublishFunctionVersion";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/versions";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateFunction {
    /// 函数配置信息
    body: Vec<u8>,
}

impl sealed::Bound for CreateFunction {}

impl CreateFunction {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }
}

impl crate::Request for CreateFunction {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateFunction";
    const URL_PATH: &'static str = "/2023-03-30/functions";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::JsonResponseWrap<Function>;

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
    body: Vec<u8>,
}

impl sealed::Bound for UpdateFunction {}

impl UpdateFunction {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for UpdateFunction {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UpdateFunction";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
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
    body: Option<Vec<u8>>,
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

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body.unwrap_or_default())
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
    body: Vec<u8>,
}

impl sealed::Bound for UpdateAlias {}

impl UpdateAlias {
    pub fn new(function_name: impl Into<String>, alias_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            alias_name: alias_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for UpdateAlias {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UpdateAlias";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/aliases/{aliasName}";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateAlias {
    /// 函数名称
    function_name: String,
    /// 创建别名的请求参数
    body: Vec<u8>,
}

impl sealed::Bound for CreateAlias {}

impl CreateAlias {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for CreateAlias {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateAlias";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/aliases";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateTrigger {
    /// 函数名称
    function_name: String,
    /// 触发器配置
    body: Vec<u8>,
}

impl sealed::Bound for CreateTrigger {}

impl CreateTrigger {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for CreateTrigger {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateTrigger";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/triggers";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
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
    body: Vec<u8>,
}

impl sealed::Bound for UpdateTrigger {}

impl UpdateTrigger {
    pub fn new(function_name: impl Into<String>, trigger_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            trigger_name: trigger_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for UpdateTrigger {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "UpdateTrigger";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/triggers/{triggerName}";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
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
    body: Vec<u8>,
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
            body: Vec::new(),
            function_name: function_name.into(),
            qualifier: None,
        }
    }
}

impl crate::Request for PutAsyncInvokeConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutAsyncInvokeConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/async-invoke-config";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
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
    body: Vec<u8>,
}

impl sealed::Bound for PutProvisionConfig {}

impl PutProvisionConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            qualifier: None,
            body: Vec::new(),
        }
    }
}

impl crate::Request for PutProvisionConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutProvisionConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/provision-config";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
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
    body: Vec<u8>,
}

impl sealed::Bound for PutConcurrencyConfig {}

impl PutConcurrencyConfig {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for PutConcurrencyConfig {
    const METHOD: http::Method = http::Method::PUT;

    const ACTION: &'static str = "PutConcurrencyConfig";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/concurrency";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateLayerVersion {
    /// 层名称
    layer_name: String,
    /// 层配置信息
    body: Vec<u8>,
}

impl sealed::Bound for CreateLayerVersion {}

impl CreateLayerVersion {
    pub fn new(layer_name: impl Into<String>) -> Self {
        Self {
            layer_name: layer_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for CreateLayerVersion {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateLayerVersion";
    const URL_PATH: &'static str = "/2023-03-30/layers/{layerName}/versions";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
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
    body: Vec<u8>,
}

impl sealed::Bound for CreateVpcBinding {}

impl CreateVpcBinding {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            body: Vec::new(),
        }
    }
}

impl crate::Request for CreateVpcBinding {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateVpcBinding";
    const URL_PATH: &'static str = "/2023-03-30/functions/{functionName}/vpc-bindings";

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body)
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
    body: Vec<u8>,
}

impl sealed::Bound for TagResources {}

impl TagResources {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }
}

impl crate::Request for TagResources {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "TagResources";
    const URL_PATH: &'static str = "/2023-03-30/tags-v2";

    type Body = crate::OctetStream;

    type ResponseWrap = crate::JsonResponseWrap<crate::OpenObjectResponse>;

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
    body: Option<Vec<u8>>,
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

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body.unwrap_or_default())
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
    body: Option<Vec<u8>>,
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

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body.unwrap_or_default())
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
    body: Option<Vec<u8>>,
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

    type Body = crate::OctetStream;

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
        crate::OctetStream(self.body.unwrap_or_default())
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
    #[serde(rename = "additionalVersionWeight")]
    pub additional_version_weight: std::collections::HashMap<String, f64>,
    #[serde(rename = "aliasName")]
    pub alias_name: String,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
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
    #[serde(rename = "onFailure")]
    pub on_failure: Destination,
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
    #[serde(rename = "asyncTask")]
    pub async_task: bool,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "destinationConfig")]
    pub destination_config: DestinationConfig,
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    #[serde(rename = "maxAsyncEventAgeInSeconds")]
    pub max_async_event_age_in_seconds: i64,
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
    #[serde(rename = "eventDetail")]
    pub event_detail: String,
    #[serde(rename = "eventId")]
    pub event_id: i64,
    #[serde(rename = "status")]
    pub status: String,
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
    #[serde(rename = "alreadyRetriedTimes")]
    pub already_retried_times: i64,
    #[serde(rename = "destinationStatus")]
    pub destination_status: String,
    #[serde(rename = "durationMs")]
    pub duration_ms: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "events")]
    pub events: Vec<AsyncTaskEvent>,
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "returnPayload")]
    pub return_payload: String,
    #[serde(rename = "startedTime")]
    pub started_time: i64,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "taskErrorMessage")]
    pub task_error_message: String,
    #[serde(rename = "taskId")]
    pub task_id: String,
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
    #[serde(rename = "authInfo")]
    pub auth_info: String,
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
    #[serde(rename = "CountBasedWindow")]
    pub count_based_window: i32,
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
    #[serde(rename = "certName")]
    pub cert_name: String,
    #[serde(rename = "certificate")]
    pub certificate: String,
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
    #[serde(rename = "functionArn")]
    pub function_arn: String,
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
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    #[serde(rename = "sessionConcurrencyPerInstance")]
    pub session_concurrency_per_instance: i64,
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
    #[serde(rename = "additionalVersionWeight")]
    pub additional_version_weight: std::collections::HashMap<String, f64>,
    #[serde(rename = "aliasName")]
    pub alias_name: String,
    #[serde(rename = "description")]
    pub description: String,
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
    #[serde(rename = "match")]
    pub r#match: String,
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
    #[serde(rename = "match")]
    pub r#match: String,
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
    #[serde(rename = "match")]
    pub r#match: String,
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
    #[serde(rename = "equalRules")]
    pub equal_rules: Vec<EqualRule>,
    #[serde(rename = "regexRules")]
    pub regex_rules: Vec<RegexRule>,
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
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "methods")]
    pub methods: Vec<String>,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "qualifier")]
    pub qualifier: String,
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
    #[serde(rename = "cipherSuites")]
    pub cipher_suites: Vec<String>,
    #[serde(rename = "maxVersion")]
    pub max_version: String,
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
    #[serde(rename = "authConfig")]
    pub auth_config: AuthConfig,
    #[serde(rename = "certConfig")]
    pub cert_config: CertConfig,
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "routeConfig")]
    pub route_config: RouteConfig,
    #[serde(rename = "tlsConfig")]
    pub tls_config: TLSConfig,
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
    #[serde(rename = "checksum")]
    pub checksum: String,
    #[serde(rename = "ossBucketName")]
    pub oss_bucket_name: String,
    #[serde(rename = "ossObjectName")]
    pub oss_object_name: String,
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
    #[serde(rename = "failureThreshold")]
    pub failure_threshold: i32,
    #[serde(rename = "httpGetUrl")]
    pub http_get_url: String,
    #[serde(rename = "initialDelaySeconds")]
    pub initial_delay_seconds: i32,
    #[serde(rename = "periodSeconds")]
    pub period_seconds: i32,
    #[serde(rename = "successThreshold")]
    pub success_threshold: i32,
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
    #[serde(rename = "password")]
    pub password: String,
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
    #[serde(rename = "insecure")]
    pub insecure: bool,
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
    #[serde(rename = "securityGroupId")]
    pub security_group_id: String,
    #[serde(rename = "vSwitchId")]
    pub v_switch_id: String,
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
    #[serde(rename = "authConfig")]
    pub auth_config: RegistryAuthConfig,
    #[serde(rename = "certConfig")]
    pub cert_config: RegistryCertConfig,
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
    #[serde(rename = "accelerationInfo")]
    pub acceleration_info: AccelerationInfo,
    #[serde(rename = "accelerationType")]
    pub acceleration_type: String,
    #[serde(rename = "acrInstanceId")]
    pub acr_instance_id: String,
    #[serde(rename = "command")]
    pub command: Vec<String>,
    #[serde(rename = "entrypoint")]
    pub entrypoint: Vec<String>,
    #[serde(rename = "healthCheckConfig")]
    pub health_check_config: CustomHealthCheckConfig,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "registryConfig")]
    pub registry_config: RegistryConfig,
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
    #[serde(rename = "name")]
    pub name: String,
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
    #[serde(rename = "dnsOptions")]
    pub dns_options: Vec<DNSOption>,
    #[serde(rename = "nameServers")]
    pub name_servers: Vec<String>,
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
    #[serde(rename = "args")]
    pub args: Vec<String>,
    #[serde(rename = "command")]
    pub command: Vec<String>,
    #[serde(rename = "healthCheckConfig")]
    pub health_check_config: CustomHealthCheckConfig,
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
    #[serde(rename = "gpuMemorySize")]
    pub gpu_memory_size: i32,
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
    #[serde(rename = "handler")]
    pub handler: String,
    #[serde(rename = "timeout")]
    pub timeout: i32,
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
    #[serde(rename = "initializer")]
    pub initializer: LifecycleHook,
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
    #[serde(rename = "enableInstanceMetrics")]
    pub enable_instance_metrics: bool,
    #[serde(rename = "enableRequestMetrics")]
    pub enable_request_metrics: bool,
    #[serde(rename = "logBeginRule")]
    pub log_begin_rule: String,
    #[serde(rename = "logstore")]
    pub logstore: String,
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
    #[serde(rename = "enableTLS")]
    pub enable_tls: bool,
    #[serde(rename = "mountDir")]
    pub mount_dir: String,
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
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "mountPoints")]
    pub mount_points: Vec<NASMountConfig>,
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
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    #[serde(rename = "bucketPath")]
    pub bucket_path: String,
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[serde(rename = "mountDir")]
    pub mount_dir: String,
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
    #[serde(rename = "params")]
    pub params: std::collections::HashMap<String, String>,
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
    #[serde(rename = "securityGroupId")]
    pub security_group_id: String,
    #[serde(rename = "vSwitchIds")]
    pub v_switch_ids: Vec<String>,
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
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
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
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
    #[serde(rename = "code")]
    pub code: InputCodeLocation,
    #[serde(rename = "cpu")]
    pub cpu: f32,
    #[serde(rename = "customContainerConfig")]
    pub custom_container_config: CustomContainerConfig,
    #[serde(rename = "customDNS")]
    pub custom_dns: CustomDNS,
    #[serde(rename = "customRuntimeConfig")]
    pub custom_runtime_config: CustomRuntimeConfig,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "diskSize")]
    pub disk_size: i32,
    #[serde(rename = "environmentVariables")]
    pub environment_variables: std::collections::HashMap<String, String>,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "gpuConfig")]
    pub gpu_config: GPUConfig,
    #[serde(rename = "handler")]
    pub handler: String,
    #[serde(rename = "instanceConcurrency")]
    pub instance_concurrency: i32,
    #[serde(rename = "instanceLifecycleConfig")]
    pub instance_lifecycle_config: InstanceLifecycleConfig,
    #[serde(rename = "internetAccess")]
    pub internet_access: bool,
    #[serde(rename = "layers")]
    pub layers: Vec<String>,
    #[serde(rename = "logConfig")]
    pub log_config: LogConfig,
    #[serde(rename = "memorySize")]
    pub memory_size: i32,
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: OSSMountConfig,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "runtime")]
    pub runtime: String,
    #[serde(rename = "timeout")]
    pub timeout: i32,
    #[serde(rename = "tracingConfig")]
    pub tracing_config: TracingConfig,
    #[serde(rename = "vpcConfig")]
    pub vpc_config: VPCConfig,
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "disableOndemand")]
    pub disable_ondemand: bool,
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: String,
    #[serde(rename = "enableLongLiving")]
    pub enable_long_living: bool,
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: String,
    #[serde(rename = "instanceIsolationMode")]
    pub instance_isolation_mode: CreateFunctionInputInstanceIsolationMode,
    #[serde(rename = "sessionAffinityConfig")]
    pub session_affinity_config: String,
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: i32,
    #[serde(rename = "disableInjectCredentials")]
    pub disable_inject_credentials: CreateFunctionInputDisableInjectCredentials,
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
    #[serde(rename = "code")]
    pub code: InputCodeLocation,
    #[serde(rename = "compatibleRuntime")]
    pub compatible_runtime: Vec<String>,
    #[serde(rename = "description")]
    pub description: String,
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
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
    #[serde(rename = "sessionId")]
    pub session_id: String,
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
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "invocationRole")]
    pub invocation_role: String,
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    #[serde(rename = "sourceArn")]
    pub source_arn: String,
    #[serde(rename = "triggerConfig")]
    pub trigger_config: String,
    #[serde(rename = "triggerName")]
    pub trigger_name: String,
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
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "authConfig")]
    pub auth_config: AuthConfig,
    #[serde(rename = "certConfig")]
    pub cert_config: CertConfig,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "routeConfig")]
    pub route_config: RouteConfig,
    #[serde(rename = "subdomainCount")]
    pub subdomain_count: String,
    #[serde(rename = "tlsConfig")]
    pub tls_config: TLSConfig,
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
    #[serde(rename = "concurrency")]
    pub concurrency: i64,
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
    #[serde(rename = "RegionId")]
    pub region_id: String,
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
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
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
    #[serde(rename = "BrokerUrl")]
    pub broker_url: String,
    #[serde(rename = "InitCheckPoint")]
    pub init_check_point: i32,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "RegionId")]
    pub region_id: String,
    #[serde(rename = "Sid")]
    pub sid: String,
    #[serde(rename = "TaskId")]
    pub task_id: String,
    #[serde(rename = "Topic")]
    pub topic: String,
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
    #[serde(rename = "ConsumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "Network")]
    pub network: String,
    #[serde(rename = "OffsetReset")]
    pub offset_reset: String,
    #[serde(rename = "RegionId")]
    pub region_id: String,
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: String,
    #[serde(rename = "Topic")]
    pub topic: String,
    #[serde(rename = "VSwitchIds")]
    pub v_switch_ids: String,
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
    #[serde(rename = "IsBase64Decode")]
    pub is_base64_decode: bool,
    #[serde(rename = "QueueName")]
    pub queue_name: String,
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
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "RegionId")]
    pub region_id: String,
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
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "QueueName")]
    pub queue_name: String,
    #[serde(rename = "RegionId")]
    pub region_id: String,
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
    #[serde(rename = "AuthType")]
    pub auth_type: String,
    #[serde(rename = "FilterType")]
    pub filter_type: String,
    #[serde(rename = "GroupID")]
    pub group_id: String,
    #[serde(rename = "InstanceEndpoint")]
    pub instance_endpoint: String,
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "InstanceNetwork")]
    pub instance_network: String,
    #[serde(rename = "InstancePassword")]
    pub instance_password: String,
    #[serde(rename = "InstanceSecurityGroupId")]
    pub instance_security_group_id: String,
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "InstanceUsername")]
    pub instance_username: String,
    #[serde(rename = "InstanceVSwitchIds")]
    pub instance_v_switch_ids: String,
    #[serde(rename = "InstanceVpcId")]
    pub instance_vpc_id: String,
    #[serde(rename = "Offset")]
    pub offset: String,
    #[serde(rename = "RegionId")]
    pub region_id: String,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: i32,
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
    #[serde(rename = "sourceDTSParameters")]
    pub source_dts_parameters: SourceDTSParameters,
    #[serde(rename = "sourceKafkaParameters")]
    pub source_kafka_parameters: SourceKafkaParameters,
    #[serde(rename = "sourceMNSParameters")]
    pub source_mns_parameters: SourceMNSParameters,
    #[serde(rename = "sourceMQTTParameters")]
    pub source_mqtt_parameters: SourceMQTTParameters,
    #[serde(rename = "sourceRabbitMQParameters")]
    pub source_rabbit_mq_parameters: SourceRabbitMQParameters,
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
    #[serde(rename = "eventSourceParameters")]
    pub event_source_parameters: EventSourceParameters,
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
    #[serde(rename = "batchWindow")]
    pub batch_window: BatchWindow,
    #[serde(rename = "deadLetterQueue")]
    pub dead_letter_queue: DeadLetterQueue,
    #[serde(rename = "errorsTolerance")]
    pub errors_tolerance: String,
    #[serde(rename = "mode")]
    pub mode: String,
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
    #[serde(rename = "asyncInvocationType")]
    pub async_invocation_type: bool,
    #[serde(rename = "eventRuleFilterPattern")]
    pub event_rule_filter_pattern: String,
    #[serde(rename = "eventSinkConfig")]
    pub event_sink_config: EventSinkConfig,
    #[serde(rename = "eventSourceConfig")]
    pub event_source_config: EventSourceConfig,
    #[serde(rename = "runOptions")]
    pub run_options: RunOptions,
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
    #[serde(rename = "prefix")]
    pub prefix: String,
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
    #[serde(rename = "arn")]
    pub arn: String,
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
    #[serde(rename = "codeChecksum")]
    pub code_checksum: String,
    #[serde(rename = "codeSize")]
    pub code_size: i64,
    #[serde(rename = "cpu")]
    pub cpu: f32,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "customContainerConfig")]
    pub custom_container_config: CustomContainerConfig,
    #[serde(rename = "customDNS")]
    pub custom_dns: CustomDNS,
    #[serde(rename = "customRuntimeConfig")]
    pub custom_runtime_config: CustomRuntimeConfig,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "diskSize")]
    pub disk_size: i32,
    #[serde(rename = "environmentVariables")]
    pub environment_variables: std::collections::HashMap<String, String>,
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    #[serde(rename = "functionId")]
    pub function_id: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "gpuConfig")]
    pub gpu_config: GPUConfig,
    #[serde(rename = "handler")]
    pub handler: String,
    #[serde(rename = "instanceConcurrency")]
    pub instance_concurrency: i32,
    #[serde(rename = "instanceLifecycleConfig")]
    pub instance_lifecycle_config: InstanceLifecycleConfig,
    #[serde(rename = "internetAccess")]
    pub internet_access: bool,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    #[serde(rename = "lastUpdateStatus")]
    pub last_update_status: String,
    #[serde(rename = "lastUpdateStatusReason")]
    pub last_update_status_reason: String,
    #[serde(rename = "lastUpdateStatusReasonCode")]
    pub last_update_status_reason_code: String,
    #[serde(rename = "layers")]
    pub layers: Vec<FunctionLayer>,
    #[serde(rename = "logConfig")]
    pub log_config: LogConfig,
    #[serde(rename = "memorySize")]
    pub memory_size: i32,
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: OSSMountConfig,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "runtime")]
    pub runtime: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "stateReason")]
    pub state_reason: String,
    #[serde(rename = "stateReasonCode")]
    pub state_reason_code: String,
    #[serde(rename = "timeout")]
    pub timeout: i32,
    #[serde(rename = "tracingConfig")]
    pub tracing_config: TracingConfig,
    #[serde(rename = "vpcConfig")]
    pub vpc_config: VPCConfig,
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "disableOndemand")]
    pub disable_ondemand: bool,
    #[serde(rename = "invocationRestriction")]
    pub invocation_restriction: FunctionRestriction,
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: String,
    #[serde(rename = "enableLongLiving")]
    pub enable_long_living: bool,
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: String,
    #[serde(rename = "instanceIsolationMode")]
    pub instance_isolation_mode: FunctionInstanceIsolationMode,
    #[serde(rename = "sessionAffinityConfig")]
    pub session_affinity_config: String,
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: i32,
    #[serde(rename = "disableInjectCredentials")]
    pub disable_inject_credentials: FunctionDisableInjectCredentials,
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
    #[serde(rename = "resouceType")]
    pub resouce_type: String,
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
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
    #[serde(rename = "urlInternet")]
    pub url_internet: String,
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
    #[serde(rename = "authConfig")]
    pub auth_config: String,
    #[serde(rename = "authType")]
    pub auth_type: String,
    #[serde(rename = "disableURLInternet")]
    pub disable_url_internet: bool,
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
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    #[serde(rename = "sessionConcurrencyPerInstance")]
    pub session_concurrency_per_instance: i64,
    #[serde(rename = "affinityHeaderFieldName")]
    pub affinity_header_field_name: String,
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
    #[serde(rename = "instanceId")]
    pub instance_id: String,
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
    #[serde(rename = "maxRetryTime")]
    pub max_retry_time: i32,
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
    #[serde(rename = "location")]
    pub location: String,
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
    #[serde(rename = "acl")]
    pub acl: String,
    #[serde(rename = "code")]
    pub code: OutputCodeLocation,
    #[serde(rename = "codeChecksum")]
    pub code_checksum: String,
    #[serde(rename = "codeSize")]
    pub code_size: i64,
    #[serde(rename = "compatibleRuntime")]
    pub compatible_runtime: Vec<String>,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "layerName")]
    pub layer_name: String,
    #[serde(rename = "layerVersionArn")]
    pub layer_version_arn: String,
    #[serde(rename = "license")]
    pub license: String,
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
    #[serde(rename = "aliases")]
    pub aliases: Vec<Alias>,
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
    #[serde(rename = "configs")]
    pub configs: Vec<AsyncConfig>,
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
    #[serde(rename = "nextToken")]
    pub next_token: String,
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
    #[serde(rename = "configs")]
    pub configs: Vec<ConcurrencyConfig>,
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
    #[serde(rename = "customDomains")]
    pub custom_domains: Vec<CustomDomain>,
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
    #[serde(rename = "functions")]
    pub functions: Vec<Function>,
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
    #[serde(rename = "layers")]
    pub layers: Vec<Layer>,
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
    #[serde(rename = "layers")]
    pub layers: Vec<Layer>,
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
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "scheduleExpression")]
    pub schedule_expression: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "target")]
    pub target: i64,
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
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "maxCapacity")]
    pub max_capacity: i64,
    #[serde(rename = "metricTarget")]
    pub metric_target: f32,
    #[serde(rename = "metricType")]
    pub metric_type: String,
    #[serde(rename = "minCapacity")]
    pub min_capacity: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
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
    #[serde(rename = "scheduledActions")]
    pub scheduled_actions: Vec<ScheduledAction>,
    #[serde(rename = "currentError")]
    pub current_error: String,
    #[serde(rename = "defaultTarget")]
    pub default_target: i64,
    #[serde(rename = "current")]
    pub current: i64,
    #[serde(rename = "alwaysAllocateCPU")]
    pub always_allocate_cpu: bool,
    #[serde(rename = "alwaysAllocateGPU")]
    pub always_allocate_gpu: bool,
    #[serde(rename = "targetTrackingPolicies")]
    pub target_tracking_policies: Vec<TargetTrackingPolicy>,
    #[serde(rename = "functionArn")]
    pub function_arn: String,
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
    #[serde(rename = "nextToken")]
    pub next_token: String,
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
    #[serde(rename = "totalGpuCards")]
    pub total_gpu_cards: i64,
    #[serde(rename = "totalMemorySize")]
    pub total_memory_size: i64,
    #[serde(rename = "totalDiskSize")]
    pub total_disk_size: i64,
    #[serde(rename = "totalGpuMemorySize")]
    pub total_gpu_memory_size: i64,
    #[serde(rename = "totalCpuCores")]
    pub total_cpu_cores: i64,
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
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    #[serde(rename = "instanceCount")]
    pub instance_count: i32,
    #[serde(rename = "totalCpuCores")]
    pub total_cpu_cores: f64,
    #[serde(rename = "totalMemorySize")]
    pub total_memory_size: f64,
    #[serde(rename = "totalGpuMemorySize")]
    pub total_gpu_memory_size: f64,
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
    #[serde(rename = "residentResourcePoolName")]
    pub resident_resource_pool_name: String,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    #[serde(rename = "expireTime")]
    pub expire_time: String,
    #[serde(rename = "resourcePoolConfig")]
    pub resource_pool_config: ResidentResourceCapacity,
    #[serde(rename = "residentResourcePoolId")]
    pub resident_resource_pool_id: String,
    #[serde(rename = "resourcePoolCapacity")]
    pub resource_pool_capacity: ResidentResourceCapacity,
    #[serde(rename = "createdTime")]
    pub created_time: String,
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
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "sessionAffinityType")]
    pub session_affinity_type: String,
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    #[serde(rename = "sessionStatus")]
    pub session_status: String,
    #[serde(rename = "containerId")]
    pub container_id: String,
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
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
    #[serde(rename = "nextToken")]
    pub next_token: String,
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
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    #[serde(rename = "TagKey")]
    pub tag_key: String,
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
    #[serde(rename = "NextToken")]
    pub next_token: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
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
    #[serde(rename = "resouceType")]
    pub resouce_type: String,
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
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
    #[serde(rename = "nextToken")]
    pub next_token: String,
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
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "httpTrigger")]
    pub http_trigger: HTTPTrigger,
    #[serde(rename = "invocationRole")]
    pub invocation_role: String,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    #[serde(rename = "qualifier")]
    pub qualifier: String,
    #[serde(rename = "sourceArn")]
    pub source_arn: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "targetArn")]
    pub target_arn: String,
    #[serde(rename = "triggerConfig")]
    pub trigger_config: String,
    #[serde(rename = "triggerId")]
    pub trigger_id: String,
    #[serde(rename = "triggerName")]
    pub trigger_name: String,
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
    #[serde(rename = "nextToken")]
    pub next_token: String,
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
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
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
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(rename = "nextToken")]
    pub next_token: String,
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
    #[serde(rename = "sseEndpointPath")]
    pub sse_endpoint_path: String,
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
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
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
    #[serde(rename = "filterTag")]
    pub filter_tag: String,
    #[serde(rename = "notifyContentFormat")]
    pub notify_content_format: String,
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
    #[serde(rename = "events")]
    pub events: Vec<String>,
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
    #[serde(rename = "checksum")]
    pub checksum: String,
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
    #[serde(rename = "asyncTask")]
    pub async_task: bool,
    #[serde(rename = "destinationConfig")]
    pub destination_config: DestinationConfig,
    #[serde(rename = "maxAsyncEventAgeInSeconds")]
    pub max_async_event_age_in_seconds: i64,
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
    #[serde(rename = "scheduledActions")]
    pub scheduled_actions: Vec<ScheduledAction>,
    #[serde(rename = "defaultTarget")]
    pub default_target: i64,
    #[serde(rename = "alwaysAllocateCPU")]
    pub always_allocate_cpu: bool,
    #[serde(rename = "alwaysAllocateGPU")]
    pub always_allocate_gpu: bool,
    #[serde(rename = "targetTrackingPolicies")]
    pub target_tracking_policies: Vec<TargetTrackingPolicy>,
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
    #[serde(rename = "logstore")]
    pub logstore: String,
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
    #[serde(rename = "logstore")]
    pub logstore: String,
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
    #[serde(rename = "enable")]
    pub enable: bool,
    #[serde(rename = "functionParameter")]
    pub function_parameter: std::collections::HashMap<String, String>,
    #[serde(rename = "jobConfig")]
    pub job_config: JobConfig,
    #[serde(rename = "logConfig")]
    pub log_config: SLSTriggerLogConfig,
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
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
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
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
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
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    #[serde(rename = "enable")]
    pub enable: bool,
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
    #[serde(rename = "additionalVersionWeight")]
    pub additional_version_weight: std::collections::HashMap<String, f64>,
    #[serde(rename = "description")]
    pub description: String,
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
    #[serde(rename = "authConfig")]
    pub auth_config: AuthConfig,
    #[serde(rename = "certConfig")]
    pub cert_config: CertConfig,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "routeConfig")]
    pub route_config: RouteConfig,
    #[serde(rename = "tlsConfig")]
    pub tls_config: TLSConfig,
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
    #[serde(rename = "code")]
    pub code: InputCodeLocation,
    #[serde(rename = "cpu")]
    pub cpu: f32,
    #[serde(rename = "customContainerConfig")]
    pub custom_container_config: CustomContainerConfig,
    #[serde(rename = "customDNS")]
    pub custom_dns: CustomDNS,
    #[serde(rename = "customRuntimeConfig")]
    pub custom_runtime_config: CustomRuntimeConfig,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "diskSize")]
    pub disk_size: i32,
    #[serde(rename = "environmentVariables")]
    pub environment_variables: std::collections::HashMap<String, String>,
    #[serde(rename = "gpuConfig")]
    pub gpu_config: GPUConfig,
    #[serde(rename = "handler")]
    pub handler: String,
    #[serde(rename = "instanceConcurrency")]
    pub instance_concurrency: i32,
    #[serde(rename = "instanceLifecycleConfig")]
    pub instance_lifecycle_config: InstanceLifecycleConfig,
    #[serde(rename = "internetAccess")]
    pub internet_access: bool,
    #[serde(rename = "layers")]
    pub layers: Vec<String>,
    #[serde(rename = "logConfig")]
    pub log_config: LogConfig,
    #[serde(rename = "memorySize")]
    pub memory_size: i32,
    #[serde(rename = "nasConfig")]
    pub nas_config: NASConfig,
    #[serde(rename = "ossMountConfig")]
    pub oss_mount_config: OSSMountConfig,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "runtime")]
    pub runtime: String,
    #[serde(rename = "timeout")]
    pub timeout: i32,
    #[serde(rename = "tracingConfig")]
    pub tracing_config: TracingConfig,
    #[serde(rename = "vpcConfig")]
    pub vpc_config: VPCConfig,
    #[serde(rename = "disableOndemand")]
    pub disable_ondemand: bool,
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: String,
    #[serde(rename = "enableLongLiving")]
    pub enable_long_living: bool,
    #[serde(rename = "instanceIsolationMode")]
    pub instance_isolation_mode: UpdateFunctionInputInstanceIsolationMode,
    #[serde(rename = "sessionAffinityConfig")]
    pub session_affinity_config: String,
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: i32,
    #[serde(rename = "disableInjectCredentials")]
    pub disable_inject_credentials: UpdateFunctionInputDisableInjectCredentials,
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
    #[serde(rename = "sessionTTLInSeconds")]
    pub session_ttl_in_seconds: i64,
    #[serde(rename = "sessionIdleTimeoutInSeconds")]
    pub session_idle_timeout_in_seconds: i64,
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
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "invocationRole")]
    pub invocation_role: String,
    #[serde(rename = "qualifier")]
    pub qualifier: String,
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
pub struct ResourcesTag {
    #[serde(rename = "Key")]
    pub key: String,
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
    #[serde(rename = "success")]
    pub success: bool,
}

impl crate::ToCodeMessage for DisableFunctionInvocationResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}
