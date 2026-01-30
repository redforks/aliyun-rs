#[derive(Clone, Copy)]
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
pub struct Connection(crate::common::Connection);

impl Connection {
    pub fn new(endpoint: Endpoint, app_key_secret: crate::v3::AccessKeySecret) -> Self {
        Self(crate::common::Connection::new(
            app_key_secret,
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
            app_key_secret,
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
    ) -> impl std::future::Future<Output = crate::Result<ChangeResourceGroupResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<DescribeRegionsResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<CreateCustomDomainResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetCustomDomainResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListCustomDomainsResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<UpdateCustomDomainResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetFunctionCodeResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListFunctionVersionsResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<PublishFunctionVersionResponse>> + Send
    {
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
    ) -> impl std::future::Future<Output = crate::Result<CreateFunctionResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetFunctionResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListFunctionsResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<UpdateFunctionResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetAliasResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListAliasesResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<UpdateAliasResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<CreateAliasResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<CreateTriggerResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetTriggerResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListTriggersResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<UpdateTriggerResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetAsyncInvokeConfigResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListAsyncInvokeConfigsResponse>> + Send
    {
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
    ) -> impl std::future::Future<Output = crate::Result<PutAsyncInvokeConfigResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetProvisionConfigResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListProvisionConfigsResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<PutProvisionConfigResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetConcurrencyConfigResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListConcurrencyConfigsResponse>> + Send
    {
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
    ) -> impl std::future::Future<Output = crate::Result<PutConcurrencyConfigResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<CreateLayerVersionResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetLayerVersionResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetLayerVersionByArnResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListLayerVersionsResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListLayersResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListInstancesResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListVpcBindingsResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListTagResourcesResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetAsyncTaskResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListAsyncTasksResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<CreateSessionResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetSessionResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<UpdateSessionResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListSessionsResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<PutScalingConfigResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<GetScalingConfigResponse>> + Send {
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
    ) -> impl std::future::Future<Output = crate::Result<ListScalingConfigsResponse>> + Send {
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

    type ResponseWrap = crate::JsonResponseWrap<ChangeResourceGroupResponse>;

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

    type ResponseWrap = crate::JsonResponseWrap<DescribeRegionsResponse>;

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

    type ResponseWrap = crate::JsonResponseWrap<CreateCustomDomainResponse>;

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
        Box::new([("domainName", self.domain_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<GetCustomDomainResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("domainName", self.domain_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListCustomDomainsResponse>;

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

    type ResponseWrap = crate::JsonResponseWrap<UpdateCustomDomainResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("domainName", self.domain_name.to_string())])
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
            ("functionName", self.function_name.to_string()),
            ("versionId", self.version_id.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<GetFunctionCodeResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListFunctionVersionsResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<PublishFunctionVersionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<CreateFunctionResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<GetFunctionResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListFunctionsResponse>;

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

    type ResponseWrap = Vec<u8>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<UpdateFunctionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("functionName", self.function_name.to_string())])
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
        Box::new([("functionName", self.function_name.to_string())])
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
        Box::new([("functionName", self.function_name.to_string())])
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
            ("aliasName", self.alias_name.to_string()),
            ("functionName", self.function_name.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<GetAliasResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("aliasName", self.alias_name.to_string()),
            ("functionName", self.function_name.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<ListAliasesResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<UpdateAliasResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("aliasName", self.alias_name.to_string()),
            ("functionName", self.function_name.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<CreateAliasResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<CreateTriggerResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("functionName", self.function_name.to_string())])
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
            ("functionName", self.function_name.to_string()),
            ("triggerName", self.trigger_name.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<GetTriggerResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("functionName", self.function_name.to_string()),
            ("triggerName", self.trigger_name.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<ListTriggersResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<UpdateTriggerResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("functionName", self.function_name.to_string()),
            ("triggerName", self.trigger_name.to_string()),
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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<GetAsyncInvokeConfigResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListAsyncInvokeConfigsResponse>;

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

    type ResponseWrap = crate::JsonResponseWrap<PutAsyncInvokeConfigResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<GetProvisionConfigResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListProvisionConfigsResponse>;

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

    type ResponseWrap = crate::JsonResponseWrap<PutProvisionConfigResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<GetConcurrencyConfigResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListConcurrencyConfigsResponse>;

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

    type ResponseWrap = crate::JsonResponseWrap<PutConcurrencyConfigResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<CreateLayerVersionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("layerName", self.layer_name.to_string())])
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
            ("layerName", self.layer_name.to_string()),
            ("version", self.version.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<GetLayerVersionResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([
            ("layerName", self.layer_name.to_string()),
            ("version", self.version.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<GetLayerVersionByArnResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("arn", self.arn.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListLayerVersionsResponse>;

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
        Box::new([("layerName", self.layer_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListLayersResponse>;

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
        Box::new([("layerName", self.layer_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListInstancesResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListVpcBindingsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([("functionName", self.function_name.to_string())])
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
        Box::new([("functionName", self.function_name.to_string())])
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
            ("functionName", self.function_name.to_string()),
            ("vpcId", self.vpc_id.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<ListTagResourcesResponse>;

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

    type ResponseWrap = crate::JsonResponseWrap<GetAsyncTaskResponse>;

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
            ("functionName", self.function_name.to_string()),
            ("taskId", self.task_id.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<ListAsyncTasksResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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
            ("functionName", self.function_name.to_string()),
            ("taskId", self.task_id.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<CreateSessionResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<GetSessionResponse>;

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
            ("functionName", self.function_name.to_string()),
            ("sessionId", self.session_id.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<UpdateSessionResponse>;

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
            ("functionName", self.function_name.to_string()),
            ("sessionId", self.session_id.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<ListSessionsResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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
            ("functionName", self.function_name.to_string()),
            ("sessionId", self.session_id.to_string()),
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

    type ResponseWrap = crate::JsonResponseWrap<PutScalingConfigResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<GetScalingConfigResponse>;

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
        Box::new([("functionName", self.function_name.to_string())])
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

    type ResponseWrap = crate::JsonResponseWrap<ListScalingConfigsResponse>;

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
pub struct ChangeResourceGroupResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ChangeResourceGroupResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct DescribeRegionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for DescribeRegionsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateCustomDomainResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for CreateCustomDomainResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetCustomDomainResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetCustomDomainResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListCustomDomainsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListCustomDomainsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UpdateCustomDomainResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for UpdateCustomDomainResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetFunctionCodeResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetFunctionCodeResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListFunctionVersionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListFunctionVersionsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PublishFunctionVersionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for PublishFunctionVersionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateFunctionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for CreateFunctionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetFunctionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetFunctionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListFunctionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListFunctionsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UpdateFunctionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for UpdateFunctionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAliasResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetAliasResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAliasesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListAliasesResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAliasResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for UpdateAliasResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateAliasResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for CreateAliasResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateTriggerResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for CreateTriggerResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetTriggerResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetTriggerResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListTriggersResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListTriggersResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTriggerResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for UpdateTriggerResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAsyncInvokeConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetAsyncInvokeConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAsyncInvokeConfigsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListAsyncInvokeConfigsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutAsyncInvokeConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for PutAsyncInvokeConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetProvisionConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetProvisionConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListProvisionConfigsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListProvisionConfigsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutProvisionConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for PutProvisionConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetConcurrencyConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetConcurrencyConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListConcurrencyConfigsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListConcurrencyConfigsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutConcurrencyConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for PutConcurrencyConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateLayerVersionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for CreateLayerVersionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLayerVersionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetLayerVersionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLayerVersionByArnResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetLayerVersionByArnResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListLayerVersionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListLayerVersionsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListLayersResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListLayersResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListInstancesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListInstancesResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListVpcBindingsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListVpcBindingsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListTagResourcesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListTagResourcesResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetAsyncTaskResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetAsyncTaskResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListAsyncTasksResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListAsyncTasksResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateSessionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for CreateSessionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetSessionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetSessionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct UpdateSessionResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for UpdateSessionResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListSessionsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListSessionsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PutScalingConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for PutScalingConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetScalingConfigResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for GetScalingConfigResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ListScalingConfigsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}

impl crate::ToCodeMessage for ListScalingConfigsResponse {
    fn to_code_message(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}
