#[derive(Clone, Copy, Debug, strum::EnumString)]
pub enum Endpoint {
    ApSoutheast1,
    ApSoutheast5,
    CnHongkong,
    ApNortheast2Pop,
    CnBeijingFinance1,
    CnBeijingFinancePop,
    CnBeijingGov1,
    CnBeijingNu16B01,
    CnChengdu,
    CnEdge1,
    CnFujian,
    CnHaidianCm12C01,
    CnHangzhouBjB01,
    CnHangzhouFinance,
    CnHangzhouInternalProd1,
    CnHangzhouInternalTest1,
    CnHangzhouInternalTest2,
    CnHangzhouInternalTest3,
    CnHangzhouTest306,
    CnHongkongFinancePop,
    CnHuhehaoteNebula1,
    CnNorth2Gov1,
    CnQingdaoNebula,
    CnShanghaiEt15B01,
    CnShanghaiEt2B01,
    CnShanghaiFinance1,
    CnShanghaiInner,
    CnShanghaiInternalTest1,
    CnShenzhen,
    CnShenzhenFinance1,
    CnShenzhenInner,
    CnShenzhenSt4D01,
    CnShenzhenSu18B01,
    CnWuhan,
    CnYushanfang,
    CnZhangbei,
    CnZhangbeiNa61B01,
    CnZhangjiakouNa62A01,
    CnZhengzhouNebula1,
    EuCentral1,
    EuWest1Oxs,
    RusWest1Pop,
    CnQingdao,
    CnZhangjiakou,
    CnHuhehaote,
    CnHangzhou,
}

impl Endpoint {
    pub fn name(self) -> &'static str {
        match self {
            Endpoint::ApSoutheast1 => "ap-southeast-1",
            Endpoint::ApSoutheast5 => "ap-southeast-5",
            Endpoint::CnHongkong => "cn-hongkong",
            Endpoint::ApNortheast2Pop => "ap-northeast-2-pop",
            Endpoint::CnBeijingFinance1 => "cn-beijing-finance-1",
            Endpoint::CnBeijingFinancePop => "cn-beijing-finance-pop",
            Endpoint::CnBeijingGov1 => "cn-beijing-gov-1",
            Endpoint::CnBeijingNu16B01 => "cn-beijing-nu16-b01",
            Endpoint::CnChengdu => "cn-chengdu",
            Endpoint::CnEdge1 => "cn-edge-1",
            Endpoint::CnFujian => "cn-fujian",
            Endpoint::CnHaidianCm12C01 => "cn-haidian-cm12-c01",
            Endpoint::CnHangzhouBjB01 => "cn-hangzhou-bj-b01",
            Endpoint::CnHangzhouFinance => "cn-hangzhou-finance",
            Endpoint::CnHangzhouInternalProd1 => "cn-hangzhou-internal-prod-1",
            Endpoint::CnHangzhouInternalTest1 => "cn-hangzhou-internal-test-1",
            Endpoint::CnHangzhouInternalTest2 => "cn-hangzhou-internal-test-2",
            Endpoint::CnHangzhouInternalTest3 => "cn-hangzhou-internal-test-3",
            Endpoint::CnHangzhouTest306 => "cn-hangzhou-test-306",
            Endpoint::CnHongkongFinancePop => "cn-hongkong-finance-pop",
            Endpoint::CnHuhehaoteNebula1 => "cn-huhehaote-nebula-1",
            Endpoint::CnNorth2Gov1 => "cn-north-2-gov-1",
            Endpoint::CnQingdaoNebula => "cn-qingdao-nebula",
            Endpoint::CnShanghaiEt15B01 => "cn-shanghai-et15-b01",
            Endpoint::CnShanghaiEt2B01 => "cn-shanghai-et2-b01",
            Endpoint::CnShanghaiFinance1 => "cn-shanghai-finance-1",
            Endpoint::CnShanghaiInner => "cn-shanghai-inner",
            Endpoint::CnShanghaiInternalTest1 => "cn-shanghai-internal-test-1",
            Endpoint::CnShenzhen => "cn-shenzhen",
            Endpoint::CnShenzhenFinance1 => "cn-shenzhen-finance-1",
            Endpoint::CnShenzhenInner => "cn-shenzhen-inner",
            Endpoint::CnShenzhenSt4D01 => "cn-shenzhen-st4-d01",
            Endpoint::CnShenzhenSu18B01 => "cn-shenzhen-su18-b01",
            Endpoint::CnWuhan => "cn-wuhan",
            Endpoint::CnYushanfang => "cn-yushanfang",
            Endpoint::CnZhangbei => "cn-zhangbei",
            Endpoint::CnZhangbeiNa61B01 => "cn-zhangbei-na61-b01",
            Endpoint::CnZhangjiakouNa62A01 => "cn-zhangjiakou-na62-a01",
            Endpoint::CnZhengzhouNebula1 => "cn-zhengzhou-nebula-1",
            Endpoint::EuCentral1 => "eu-central-1",
            Endpoint::EuWest1Oxs => "eu-west-1-oxs",
            Endpoint::RusWest1Pop => "rus-west-1-pop",
            Endpoint::CnQingdao => "cn-qingdao",
            Endpoint::CnZhangjiakou => "cn-zhangjiakou",
            Endpoint::CnHuhehaote => "cn-huhehaote",
            Endpoint::CnHangzhou => "cn-hangzhou",
        }
    }
}

impl From<Endpoint> for &'static str {
    fn from(ep: Endpoint) -> Self {
        match ep {
            Endpoint::ApSoutheast1 => "dysmsapi.ap-southeast-1.aliyuncs.com",
            Endpoint::ApSoutheast5 => "dysmsapi.ap-southeast-5.aliyuncs.com",
            Endpoint::CnHongkong => "dysmsapi.aliyuncs.com",
            Endpoint::ApNortheast2Pop => "dysmsapi.ap-southeast-1.aliyuncs.com",
            Endpoint::CnBeijingFinance1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnBeijingFinancePop => "dysmsapi.aliyuncs.com",
            Endpoint::CnBeijingGov1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnBeijingNu16B01 => "dysmsapi.aliyuncs.com",
            Endpoint::CnChengdu => "dysmsapi.aliyuncs.com",
            Endpoint::CnEdge1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnFujian => "dysmsapi.aliyuncs.com",
            Endpoint::CnHaidianCm12C01 => "dysmsapi.aliyuncs.com",
            Endpoint::CnHangzhouBjB01 => "dysmsapi.aliyuncs.com",
            Endpoint::CnHangzhouFinance => "dysmsapi.aliyuncs.com",
            Endpoint::CnHangzhouInternalProd1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnHangzhouInternalTest1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnHangzhouInternalTest2 => "dysmsapi.aliyuncs.com",
            Endpoint::CnHangzhouInternalTest3 => "dysmsapi.aliyuncs.com",
            Endpoint::CnHangzhouTest306 => "dysmsapi.aliyuncs.com",
            Endpoint::CnHongkongFinancePop => "dysmsapi.aliyuncs.com",
            Endpoint::CnHuhehaoteNebula1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnNorth2Gov1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnQingdaoNebula => "dysmsapi.aliyuncs.com",
            Endpoint::CnShanghaiEt15B01 => "dysmsapi.aliyuncs.com",
            Endpoint::CnShanghaiEt2B01 => "dysmsapi.aliyuncs.com",
            Endpoint::CnShanghaiFinance1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnShanghaiInner => "dysmsapi.aliyuncs.com",
            Endpoint::CnShanghaiInternalTest1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnShenzhen => "dysmsapi.aliyuncs.com",
            Endpoint::CnShenzhenFinance1 => "dysmsapi.aliyuncs.com",
            Endpoint::CnShenzhenInner => "dysmsapi.aliyuncs.com",
            Endpoint::CnShenzhenSt4D01 => "dysmsapi.aliyuncs.com",
            Endpoint::CnShenzhenSu18B01 => "dysmsapi.aliyuncs.com",
            Endpoint::CnWuhan => "dysmsapi.aliyuncs.com",
            Endpoint::CnYushanfang => "dysmsapi.aliyuncs.com",
            Endpoint::CnZhangbei => "dysmsapi.aliyuncs.com",
            Endpoint::CnZhangbeiNa61B01 => "dysmsapi.aliyuncs.com",
            Endpoint::CnZhangjiakouNa62A01 => "dysmsapi.aliyuncs.com",
            Endpoint::CnZhengzhouNebula1 => "dysmsapi.aliyuncs.com",
            Endpoint::EuCentral1 => "dysmsapi.eu-central-1.aliyuncs.com",
            Endpoint::EuWest1Oxs => "dysmsapi.ap-southeast-1.aliyuncs.com",
            Endpoint::RusWest1Pop => "dysmsapi.ap-southeast-1.aliyuncs.com",
            Endpoint::CnQingdao => "dysmsapi.aliyuncs.com",
            Endpoint::CnZhangjiakou => "dysmsapi.aliyuncs.com",
            Endpoint::CnHuhehaote => "dysmsapi.aliyuncs.com",
            Endpoint::CnHangzhou => "dysmsapi.aliyuncs.com",
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
            "2017-05-25",
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
            "2017-05-25",
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
    /// # 申请短信资质
    ///
    /// 根据工信部及运营商实名制发送短信的要求，国内短信需提供签名归属方的资质证件信息。请先申请短信资质，然后再申请签名和模板。
    ///
    /// - 在发起申请前，请您阅读[资质材料说明](~~2384377~~)并准备相关资质材料。
    /// - 目前仅**企业认证**用户可使用API申请短信资质。若您当前阿里云账号为个人认证，请通过短信服务[控制台](https://dysms.console.aliyun.com/domestic/text/qualification/add)申请资质，或[升级为企业认证](~~37178~~)。[查看我的帐户认证类型](https://myaccount.console.aliyun.com/cert-info)
    /// - 不支持批量申请短信资质，建议每次申请至少间隔5秒。
    ///
    /// # Error Codes
    /// - `AdminBackOssFileNotUploadError`: Administrator ID card portrait photo not uploaded.
    /// - `AdminDateNotValid`: Current time is outside the administrator ID card validity period.
    /// - `AdminIdcardExpdateNotMatchRegexError`: Invalid administrator ID card expiration time format.
    /// - `AdminIdcardFrontFaceFileError`: Invalid format for administrator's ID card national emblem photo.
    /// - `AdminIdcardFrontFaceNullError`: Administrator's ID card national emblem photo cannot be empty.
    /// - `AdminIdcardNoNullError`: Administrator's ID number cannot be empty.
    /// - `AdminIdcardNotMatchRegex`: Invalid administrator ID number format.
    /// - `AdminIdcardPicsFileError`: Invalid format for administrator's ID card portrait photo.
    /// - `AdminIdcardPicsNullError`: Administrator's ID card portrait photo cannot be empty.
    /// - `AdminIdcardTypeError`: Invalid administrator ID card type.
    /// - `AdminNameNullError`: Administrator's name cannot be empty.
    /// - `BusinessLicenseDateNotMatchRegexError`: Invalid business license expiration time format.
    /// - `BusinessLicenseDateNotValid`: Current time is outside the business license validity period.
    /// - `BusinessLicenseOssFileNotUploadError`: Business license file not uploaded.
    /// - `BusinessLicensePicsFileError`: Invalid business license file format.
    /// - `BusinessLicensePicsNullError`: Business license documents cannot be empty.
    /// - `BusinessLicenseTypeError`: Invalid business license type.
    /// - `CertifyCodeError`: SMS verification code is incorrect.
    /// - `CompanyNameNullError`: Company name cannot be empty.
    /// - `CompanyTypeError`: Invalid company type.
    /// - `QualificationNameNullError`: Qualification name cannot be empty.
    /// - `QualificationNameNotMatchRegex`: Qualification names must be in Chinese, English, or alphanumeric combinations. Symbols or pure numbers are not supported.
    /// - `QualificationNameAlreadyExist`: The qualification name already exists. Please modify and resubmit.
    /// - `OtherFileTypeError`: Invalid file format for other documents.
    /// - `LegalPersonIdcardTypeError`: Invalid legal person ID card type.
    /// - `LegalIdCardNoNullError`: Legal person's ID number cannot be empty.
    /// - `LegalIdCardNotMatchRegex`: Invalid legal person ID number format.
    /// - `LegalPersonNameNullError`: Legal person's name cannot be empty.
    /// - `OrganizationCodeNullError`: Unified Social Credit Code cannot be empty.
    /// - `LegalPersonIdcardEfftimeNotMatchRegexError`: Invalid legal person ID card expiration time format.
    /// - `LegalDateNotValid`: Current time is outside the legal person ID card validity period.
    /// - `LegalFrontOssFileNotUploadError`: Legal person ID card national emblem photo not uploaded.
    /// - `LegalBackOssFileNotUploadError`: Legal person ID card portrait photo not uploaded.
    /// - `AdminFrontOssFileNotUploadError`: Administrator ID card national emblem photo not uploaded.
    /// - `OtherOssFileNotUploadError`: Other files not uploaded.
    /// - `GrayCustAccessError`: This customer is not authorized to use the OpenAPI. Please contact support for whitelisting.
    /// - `CustNotExistError`: Customer's cloud communication information is invalid.
    /// - `NotEnterpriseCertifyCustCheckError`: Non-enterprise certified customers are not allowed to access.
    /// - `PhoneNoCertifyCodeNullError`: Phone number and verification code cannot be empty.
    /// - `CompanyVerificationFailedCompanyStateInvalid`: Four Elements Verification Failed: Company is not in normal operation.
    /// - `CompanyVerificationFailedFourElementsError`: Four Elements Verification Failed: Authentication Failed.
    /// - `CompanyVerificationFailedMismatch`: Four Elements Verification Failed: Mismatch between Legal Representative and Company Information.
    /// - `CompanyVerificationFailedNoCompany`: Four Elements Verification Failed: Company Not Found.
    /// - `CompanyVerificationFailedNoLegalPerson`: Four Elements Verification Failed: Legal Representative Not Found.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn submit_sms_qualification(
        &self,
        req: SubmitSmsQualification,
    ) -> impl std::future::Future<Output = crate::Result<SubmitSmsQualificationResponse>> + Send
    {
        self.call(req)
    }

    /// # 查询资质列表
    ///
    /// 当您在申请短信资质后，可以通过此接口查询资质列表及其审核详情，支持条件筛选查询。
    ///
    /// - 支持全量查询或条件查询：
    ///   - **全量查询**：查询您当前帐户下所有短信资质，无需传参。默认全量查询。
    ///   - **条件查询**：支持根据资质名称、企业名称、法人姓名、审核状态、审核工单ID、资质用途进行查询，传入您希望筛选的参数即可。
    ///
    /// - 本接口用于查询资质及其审核信息，如果需要查询单个资质的具体信息（企业信息、法人信息、管理员信息），请调用[查询单个资质详情](~~QuerySingleSmsQualification~~)接口。
    ///
    /// - 受短信签名实名制报备要求影响，当前资质审核工单量增长快速，审核时间可能会延长，请耐心等待，预计2个工作日内完成（审核工作时间：周一至周日 9:00~21:00，法定节假日顺延）。特殊情况可能延长审核时间，请耐心等待。
    /// - 如果资质未通过审核，审核备注`AuditRemark`会返回审核失败的原因，请参考[审核失败的处理建议](~~2384377#a96cc318b94x1~~)，调用[修改短信资质](~~UpdateSmsQualification~~)接口或在控制台[资质管理](https://dysms.console.aliyun.com/domestic/text/qualification)页面修改资质信息后，重新发起审核。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn query_sms_qualification_record(
        &self,
        req: QuerySmsQualificationRecord,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsQualificationRecordResponse>> + Send
    {
        self.call(req)
    }

    /// # 查询单个资质详情
    ///
    /// 当您在申请短信资质后，可以通过此接口查询单个资质详情。
    ///
    /// - 本接口查询单个资质的详情（企业信息、法人信息、管理员信息）。
    /// - 如果需要查询您当前账号下所有资质信息，或需要查询审核详情，请调用[查询资质列表](~~QuerySmsQualificationRecord~~)。
    /// - 受短信签名实名制报备要求影响，当前资质审核工单量增长快速，审核时间可能会延长，请耐心等待，预计2个工作日内完成（审核工作时间：周一至周日 9:00~21:00，法定节假日顺延）。特殊情况可能延长审核时间，请耐心等待。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn query_single_sms_qualification(
        &self,
        req: QuerySingleSmsQualification,
    ) -> impl std::future::Future<Output = crate::Result<QuerySingleSmsQualificationResponse>> + Send
    {
        self.call(req)
    }

    /// # 修改短信资质
    ///
    /// 如果您需要更新短信资质信息，可通过本接口提交修改请求，提交后将重新进入审核流程。
    ///
    /// - 审核中的资质不支持修改，请等待审核流程结束或在短信服务控制台[撤回申请](https://dysms.console.aliyun.com/domestic/text/qualification)后再修改。
    /// - 修改后的短信资质**需要重新审核**（包括已审核通过的资质），请根据[资质材料说明](~~2384377~~)上传符合规范的材料。
    /// - **不支持修改**资质命名、申请用途、统一社会信用代码。
    /// - 不支持批量修改短信资质，建议每次修改至少间隔5秒。
    ///
    /// # Error Codes
    /// - `QualificationNameNullError`: Qualification name cannot be empty.
    /// - `BusinessLicensePicsNullError`: Business license documents cannot be empty.
    /// - `OtherFileTypeError`: Invalid file format for other documents.
    /// - `QualificationNameAlreadyExist`: The qualification name already exists. Please modify and resubmit.
    /// - `QualificationNameNotMatchRegex`: Qualification names must be in Chinese, English, or alphanumeric combinations. Symbols or pure numbers are not supported.
    /// - `AdminIdcardTypeError`: Invalid administrator ID card type.
    /// - `BusinessLicensePicsFileError`: Invalid business license file format.
    /// - `BusinessLicenseTypeError`: Invalid business license type.
    /// - `LegalIdCardNoNullError`: Legal person's ID number cannot be empty.
    /// - `LegalPersonIdcardTypeError`: Invalid legal person ID card type.
    /// - `CompanyNameNullError`: Company name cannot be empty.
    /// - `CompanyTypeError`: Invalid company type.
    /// - `LegalIdCardNotMatchRegex`: Invalid legal person ID number format.
    /// - `LegalPersonNameNullError`: Legal person's name cannot be empty.
    /// - `OrganizationCodeNullError`: Unified Social Credit Code cannot be empty.
    /// - `AdminIdcardFrontFaceFileError`: Invalid format for administrator's ID card national emblem photo.
    /// - `AdminIdcardFrontFaceNullError`: Administrator's ID card national emblem photo cannot be empty.
    /// - `AdminIdcardPicsFileError`: Invalid format for administrator's ID card portrait photo.
    /// - `AdminIdcardPicsNullError`: Administrator's ID card portrait photo cannot be empty.
    /// - `AdminNameNullError`: Administrator's name cannot be empty.
    /// - `AdminIdcardExpdateNotMatchRegexError`: Invalid administrator ID card expiration time format.
    /// - `AdminIdcardNoNullError`: Administrator's ID number cannot be empty.
    /// - `AdminIdcardNotMatchRegex`: Invalid administrator ID number format.
    /// - `BusinessLicenseDateNotMatchRegexError`: Invalid business license expiration time format.
    /// - `LegalPersonIdcardEfftimeNotMatchRegexError`: Invalid legal person ID card expiration time format.
    /// - `AdminDateNotValid`: Current time is outside the administrator ID card validity period.
    /// - `BusinessLicenseDateNotValid`: Current time is outside the business license validity period.
    /// - `BusinessLicenseOssFileNotUploadError`: Business license file not uploaded.
    /// - `LegalDateNotValid`: Current time is outside the legal person ID card validity period.
    /// - `LegalFrontOssFileNotUploadError`: Legal person ID card national emblem photo not uploaded.
    /// - `AdminBackOssFileNotUploadError`: Administrator ID card portrait photo not uploaded.
    /// - `AdminFrontOssFileNotUploadError`: Administrator ID card national emblem photo not uploaded.
    /// - `CustNotExistError`: Customer's cloud communication information is invalid.
    /// - `GrayCustAccessError`: This customer is not authorized to use the OpenAPI. Please contact support for whitelisting.
    /// - `LegalBackOssFileNotUploadError`: Legal person ID card portrait photo not uploaded.
    /// - `OtherOssFileNotUploadError`: Other files not uploaded.
    /// - `CertifyCodeError`: SMS verification code is incorrect.
    /// - `NotEnterpriseCertifyCustCheckError`: Non-enterprise certified customers are not allowed to access.
    /// - `PhoneNoCertifyCodeNullError`: Phone number and verification code cannot be empty.
    /// - `SameQualificationGroupError`: A qualification with the same company and administrator information already exists.
    /// - `WorkOrderIdExpired`: Qualification details have changed. Please re-query the qualification list and resubmit.
    /// - `CompanyVerificationFailedCompanyStateInvalid`: Four Elements Verification Failed: Company is not in normal operation.
    /// - `CompanyVerificationFailedFourElementsError`: Four Elements Verification Failed: Authentication Failed.
    /// - `CompanyVerificationFailedMismatch`: Four Elements Verification Failed: Mismatch between Legal Representative and Company Information.
    /// - `CompanyVerificationFailedNoCompany`: Four Elements Verification Failed: Company Not Found.
    /// - `CompanyVerificationFailedNoLegalPerson`: Four Elements Verification Failed: Legal Representative Not Found.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn update_sms_qualification(
        &self,
        req: UpdateSmsQualification,
    ) -> impl std::future::Future<Output = crate::Result<UpdateSmsQualificationResponse>> + Send
    {
        self.call(req)
    }

    /// # 删除短信资质
    ///
    /// 若您不再使用某个短信资质或因其他原因需要删除时，调用此接口或在短信服务控制台删除短信资质。
    ///
    /// ><warning>资质删除后不能恢复，也无法在后续申请签名时选用，请谨慎操作。></warning>
    /// - 审核中的资质不支持修改或删除，您可以在短信服务[控制台](https://dysms.console.aliyun.com/domestic/text/qualification)撤回申请后操作。
    /// - 审核通过的资质若已被签名绑定则不支持删除。
    /// - 审核不通过的资质可通过[修改资质信息](~~UpdateSmsQualification~~)后直接重新发起审核。
    ///
    /// # Error Codes
    /// - `QualificationNotExist`: Qualification does not exist.
    /// - `BindSignDeleteFailed`: The qualification is bound to a signature and cannot be deleted temporarily.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn delete_sms_qualification(
        &self,
        req: DeleteSmsQualification,
    ) -> impl std::future::Future<Output = crate::Result<DeleteSmsQualificationResponse>> + Send
    {
        self.call(req)
    }

    /// # 获取手机验证码
    ///
    /// 申请短信资质时，需要验证管理员手机号，请通过本接口获取短信验证码。
    ///
    /// - 接收到手机验证码后，请传入[申请短信资质](~~SubmitSmsQualification~~)/[修改短信资质](~~UpdateSmsQualification~~)接口的`CertifyCode`参数中。
    /// - 您可以通过[ValidPhoneCode](~~ValidPhoneCode~~)接口校验短信验证码是否准确。
    /// - 本接口获取短信验证码有[流控限制](~~44335#section-0wh-xn6-0t7~~)，请勿频繁操作：针对同一个号码最多支持1条/分钟，5条/小时，10条/天。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn required_phone_code(
        &self,
        req: RequiredPhoneCode,
    ) -> impl std::future::Future<Output = crate::Result<RequiredPhoneCodeResponse>> + Send {
        self.call(req)
    }

    /// # 验证手机验证码
    ///
    /// 申请短信资质时，需要验证管理员手机号，本接口可对手机号及收到的验证码进行验证。
    ///
    /// - 请先调用[获取手机验证码](~~RequiredPhoneCode~~)接口，阿里云将发送短信验证码至您填写的手机号码。
    /// - 本接口不影响短信资质申请流程，仅供验证短信验证码使用。实际申请时，请在[申请短信资质](~~SubmitSmsQualification~~)接口中的`CertifyCode`参数传入验证码。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn valid_phone_code(
        &self,
        req: ValidPhoneCode,
    ) -> impl std::future::Future<Output = crate::Result<ValidPhoneCodeResponse>> + Send {
        self.call(req)
    }

    /// # 创建授权委托书
    ///
    /// 若申请的资质用途为他用或申请的签名涉及第三方权益，则必须获取第三方授权，并在申请前提前创建授权委托书。
    ///
    /// - 请您在使用前阅读[授权书规范](~~56741~~)，下载[授权委托书模板](https://help-static-aliyun-doc.aliyuncs.com/file-manage-files/zh-CN/20250414/bvpcmo/%E6%8E%88%E6%9D%83%E5%A7%94%E6%89%98%E4%B9%A6%E6%A8%A1%E7%89%88.doc)后，根据规范完成填写并盖章后上传。
    /// - 您创建的授权委托书可在后续申请短信资质/申请短信签名时使用，如果您的资质/签名涉及他用，则必须创建授权委托书并提交。
    /// - 创建授权委托书后，您可以通过[QuerySmsAuthorizationLetter](~~QuerySmsAuthorizationLetter~~)查询已创建的授权书详情；通过接口创建的授权书信息会同步在短信服务控制台。
    ///
    /// # Error Codes
    /// - `AuthorizationLetterNameRepeat`: The authorization letter name is duplicated.
    /// - `AuthorizationLetterDateNotMatchRegex`: The format of the authorization letter's effective and expiry date is incorrect.
    /// - `AuthorizationLetterDateNotValid`: The current time is not within the validity period of the authorization letter.
    /// - `AuthorizationLetterNameNotMatchRegex`: The authorization letter name cannot be empty and must consist of Chinese, English characters or a combination with numbers, symbols or purely numeric input are not supported.
    /// - `AuthorizationLetterNameOverLimit`: The authorization letter name exceeds the 100-character length limit.
    /// - `AuthorizationNotMatchRegex`: The authorizer name cannot be empty and currently does not support any symbols except middle dots, spaces, Chinese brackets, and English parentheses or purely numeric input.
    /// - `AuthorizationOssFileNotUploadError`: The authorization letter file has not been uploaded.
    /// - `AuthorizationOverLimit`: The authorizer exceeds the 1000-character length limit.
    /// - `ProxyAuthorizationOverLimit`: The authorized party exceeds the 1000-character length limit.
    /// - `ProxyAuthorizationNotMatchRegex`: The authorized party name currently does not support any symbols except middle dots, spaces, Chinese brackets, and English parentheses or purely numeric input.
    /// - `SignNumOverLimit`: The signature exceeds the limit of 100 entries.
    /// - `SignNotMatchRegex`: The signature length is limited to 2-12 characters and does not support some special characters.
    /// - `OrganizationCodeOverLimit`: The organization code is limited to 150 characters.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn create_sms_authorization_letter(
        &self,
        req: CreateSmsAuthorizationLetter,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsAuthorizationLetterResponse>> + Send
    {
        self.call(req)
    }

    /// # 查询授权委托书
    ///
    /// 查询已创建的授权委托书，可查看授权书审核状态、授权签名范围。
    ///
    /// - 支持全量查询或条件查询：
    ///   - **全量查询**：查询您当前帐户下所有授权委托书信息，无需传参。默认全量查询。
    ///   - **条件查询**：支持根据授权委托书ID、签名名称、授权委托书审核状态进行查询，传入您希望筛选的参数即可。
    ///
    /// - 审核时间：受短信签名实名制报备要求影响，当前资质审核工单量增长快速，审核时间可能会延长，请耐心等待，预计2个工作日内完成。短信签名及模板预计在审核提交后的2小时内完成审核，涉及政府企业相关，一般2个工作日内审核完成。如遇升级核验、审核任务较多、非工作时间，审核时间可能会延长，请耐心等待（审核工作时间：周一至周日 9:00~21:00，法定节假日顺延）。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn query_sms_authorization_letter(
        &self,
        req: QuerySmsAuthorizationLetter,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsAuthorizationLetterResponse>> + Send
    {
        self.call(req)
    }

    /// # 申请短信签名（新接口）
    ///
    /// 短信签名作为短信发送方的一种标识，发送短信前，您需要先申请签名和模板，系统会将已审核通过的短信签名添加到短信内容的开头，并与短信内容一起发送给接收方
    ///
    /// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
    ///
    /// - 个人认证用户，同一个阿里云账号一个自然日支持申请一个正式签名；企业认证用户目前无限制。个人用户与企业用户权益区别详情请参见[使用须知](~~55324~~)。
    ///
    /// - 请阅读[签名规范](~~108076~~)，了解短信签名审核规范的具体内容。
    /// - 通过接口申请的签名信息会同步在短信服务控制台。控制台相关操作，请参见[短信签名](~~108073~~)。
    ///
    /// - 提交签名申请后，您可以通过[GetSmsSign](~~2807429~~)接口查询签名审核状态和详情。也可以[配置回执消息](~~101508~~)，通过[SignSmsReport](~~120998~~)获取签名的审核状态消息。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.
    /// - `ParameterMismatch.ThirdParty`: The type of signature, whether for personal use or for a third party, should be consistent with the qualifications.
    /// - `SignName.Exists`: Sorry, this signature already exists and cannot be applied for again.
    /// - `InvalidQualification`: The qualification should be approved.
    /// - `InvalidSignName`: The signature cannot contain spaces, special
    ///     symbols, or all numbers.
    /// - `InvalidApplySceneContent`: For certain signature sources, the
    ///     applySceneContent should be an HTTP or HTTPS link.
    /// - `MissApplySceneContent`: In some signature sources, the applySceneContent is required.
    /// - `MissingSignName`: The signature name cannot be empty.
    /// - `SmsAuthorizationLetterNotMatch`: Please bind the available authorization letter whose the social credit code is same  to the the social credit code  of qualification.
    /// - `SmsAuthorizationLetterNotExist`: Authorization does not belong to the customer.
    /// - `SmsSignNotAuthorized`: the signature is not in the sign scope of the authorization letter.
    /// - `TrademarkNotExist`: The trademark does not exist.
    /// - `AppIcpRecordNotExist`: The APP-ICP record does not exist.
    /// - `QualificationNotFound`: Qualification does not exist.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn create_sms_sign(
        &self,
        req: CreateSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsSignResponse>> + Send {
        self.call(req)
    }

    /// # 查询签名详情（新接口）
    ///
    /// 申请签名后，通过此接口查询签名审核详情
    ///
    /// - 仅可查询**首次创建**的签名资料或者**最新审核通过**的资料。
    ///
    /// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
    ///
    /// - 审核时间：一般情况下，签名提交后，阿里云预计在 2 个小时内审核完成（审核工作时间：周一至周日 9:00~21:00，法定节假日顺延）。
    ///
    /// - 如果签名未通过审核，会返回审核失败的原因，请参考[短信审核失败的处理建议](~~65990~~)，调用[UpdateSmsSign](~~2807428~~)接口或在[签名管理](https://dysms.console.aliyun.com/domestic/text/sign)页面修改未审核通过的短信签名。
    ///
    /// - [QuerySmsSignList](~~QuerySmsSignList~~)接口可以查询您账号下的所有签名，包括签名审核状态、签名类型、签名名称等。
    ///
    /// - 本接口的单用户QPS限制为150次/秒。超过限制，API调用将会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.
    /// - `SignatureNotFound`: The signature does not exist.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn get_sms_sign(
        &self,
        req: GetSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<GetSmsSignResponse>> + Send {
        self.call(req)
    }

    /// # 查询签名列表详情
    ///
    /// 可以通过此接口可以查询您账号下的所有签名，方便您查看签名详情，包括签名审核状态、签名类型、签名名称等
    ///
    /// 本接口可以查询您当前账号下**首次创建**的签名资料或者**最新审核通过**的签名详情。如果您需要查询应用场景内容、申请时上传的文件资料信息等更多内容，可以调用[GetSmsSign](~~GetSmsSign~~)接口通过签名名称查询单个签名审核详情。
    ///
    /// # Methods
    /// - POST
    ///
    pub fn query_sms_sign_list(
        &self,
        req: QuerySmsSignList,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsSignListResponse>> + Send {
        self.call(req)
    }

    /// # 修改短信签名（新接口）
    ///
    /// 修改未通过审核和已经审核通过的签名，修改完成后自动提交审核，签名进入待审核状态
    ///
    /// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
    /// - 支持修改**未通过审核**和**已经审核通过**的签名，请参考[短信审核失败的处理建议](~~65990~~)，调用此接口修改后重新提交审核。
    /// - **未通过审核**的签名如需编辑名称，该接口不支持，您可以访问控制台页面进行修改。[短信服务签名控制台入口](https://dysms.console.aliyun.com/domestic/text/sign)。
    /// - 通过接口申请的签名信息会同步在短信服务控制台，在控制台对签名的相关操作，请参见[短信签名](~~108073~~)。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.
    /// - `ParameterMismatch.ThirdParty`: The type of signature, whether for personal use or for a third party, should be consistent with the qualifications.
    /// - `SignName.Exists`: Sorry, this signature already exists and cannot be applied for again.
    /// - `InvalidQualification`: The qualification should be approved.
    /// - `InvalidApplySceneContent`: For certain signature sources, the
    ///     applySceneContent should be an HTTP or HTTPS link.
    /// - `InvalidSignName`: The signature cannot contain spaces, special
    ///     symbols, or all numbers.
    /// - `MissApplySceneContent`: In some signature sources, the applySceneContent is required.
    /// - `MissingSignName`: The signature name cannot be empty.
    /// - `SmsAuthorizationLetterNotMatch`: Please bind the available authorization letter whose the social credit code is same  to the the social credit code  of qualification.
    /// - `SmsAuthorizationLetterNotExist`: Authorization does not belong to the customer.
    /// - `SmsSignNotAuthorized`: the signature is not in the sign scope of the authorization letter.
    /// - `TrademarkNotExist`: The trademark does not exist.
    /// - `AppIcpRecordNotExist`: The APP-ICP record does not exist.
    /// - `QualificationNotFound`: Qualification does not exist.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn update_sms_sign(
        &self,
        req: UpdateSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<UpdateSmsSignResponse>> + Send {
        self.call(req)
    }

    /// # 删除短信签名
    ///
    /// 如果您不再使用某个短信签名，需要删除签名时，调用此接口或在短信服务控制台删除短信签名。
    ///
    /// - 支持删除已撤回、审核失败或审核通过的签名，审核中的短信签名不支持删除。
    /// - 删除短信签名后不可恢复，且后续不可再使用该签名发送短信，请谨慎操作。
    /// - 通过接口删除签名的操作会在短信服务控制台同步，在控制台对模板的相关操作，请参见[短信签名](~~108073~~)。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn delete_sms_sign(
        &self,
        req: DeleteSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<DeleteSmsSignResponse>> + Send {
        self.call(req)
    }

    /// # 更换签名的资质和授权书（待下线）
    ///
    /// 更换签名的资质和授权书。
    ///
    /// # Error Codes
    /// - `QualificationNotExist`: Can't query qualification information.
    /// - `SmsAuthorizationLetterNotExist`: Authorization does not belong to the customer.
    /// - `SmsSignatureNotExist`: Signature does not exist.
    /// - `SmsQualificationRegisterFailed`: The registration of the current qualification fails. Please modify the qualification and re-bind the qualification before completing the signature registration process again.
    /// - `SmsQualificationNotPassed`: The qualification has not been approved and cannot be bound to the signature.
    /// - `SmsPassedAuthorizationLetterNotMatch`: Please bind audited authorization letter whose the social credit code is same  to the the social credit code  of qualification.
    /// - `QualificationNotComplete`: The qualification elements are incomplete.
    /// - `SmsSignNotAuthorized`: the signature is not in the sign scope of the authorization letter.
    /// - `SMS_STATUS_ILLEGAL`: When replacing the qualification and power of attorney of the signature, the signature status must be approved.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn change_signature_qualification(
        &self,
        req: ChangeSignatureQualification,
    ) -> impl std::future::Future<Output = crate::Result<ChangeSignatureQualificationResponse>> + Send
    {
        self.call(req)
    }

    /// # 申请短信签名（已下线）
    ///
    /// 该接口已下线。
    ///
    /// - 根据工信部规定与运营商[相关要求](~~2806975~~)，阿里云进行了签名相关API的功能改造，为提升您签名的审核效率和审核通过率，请您使用新接口[CreateSmsSign-申请短信签名](~~2807427~~)。
    ///
    /// - 个人用户同一个阿里云账号一个自然日支持申请一个签名；企业用户申请次数无限制。个人用户与企业用户权益区别详情请参见[使用须知](~~55324~~)。
    ///
    /// - 通过接口申请的签名信息会同步在短信服务控制台，在控制台对签名的相关操作，请参见[短信签名](~~108073~~)。
    ///
    /// - 提交签名申请后，您可以通过[QuerySmsSign](~~419283~~)接口查询签名审核状态和详情。也可以[配置回执消息](~~101508~~)，通过[SignSmsReport](~~120998~~)获取签名的审核状态消息。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为1次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn add_sms_sign(
        &self,
        req: AddSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<AddSmsSignResponse>> + Send {
        self.call(req)
    }

    /// # 修改短信签名（已下线）
    ///
    /// 该接口已下线。
    ///
    /// - 根据工信部规定与运营商[相关要求](~~2806975~~)，阿里云进行了签名相关API的功能改造。为提升您签名的审核效率和审核通过率，请您使用新接口[UpdateSmsSign-修改短信签名](~~2807428~~)。
    ///
    /// - 仅支持修改未通过审核的签名，请参考[短信审核失败的处理建议](~~65990~~)，调用此接口修改后自动提交审核，签名进入待审核状态。
    ///
    /// - 通过接口修改签名的操作会在短信服务控制台同步，在控制台对签名的相关操作，请参见[短信签名](~~108073~~)。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn modify_sms_sign(
        &self,
        req: ModifySmsSign,
    ) -> impl std::future::Future<Output = crate::Result<ModifySmsSignResponse>> + Send {
        self.call(req)
    }

    /// # 查询签名审核状态（待下线）
    ///
    /// 查询签名审核状态。
    ///
    /// - 根据工信部规定与运营商[相关要求](~~2806975~~)，阿里云进行了签名相关API的功能改造。请您使用新接口[GetSmsSign-查询签名详情](~~2807429~~)，新接口查询结果返回参数中将比原有接口返回更多的签名详情信息。
    ///
    /// - 审核时间：一般情况下，签名提交后，阿里云预计在2个小时内审核完成（审核工作时间：周一至周日9:00~21:00，法定节假日顺延），建议您尽量在18:00前提交申请。
    ///
    /// - 如果签名未通过审核，会返回审核失败的原因，请参考[短信审核失败的处理建议](~~65990~~)，调用[ModifySmsTemplate](~~419287~~)接口或在[签名管理](https://dysms.console.aliyun.com/domestic/text)页面修改短信签名。
    ///
    /// - 当前接口是通过签名名称查询单个签名的审核详情。[QuerySmsSignList](~~419288~~)接口可以查询您当前账号下所有签名的签名详情。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn query_sms_sign(
        &self,
        req: QuerySmsSign,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsSignResponse>> + Send {
        self.call(req)
    }

    /// # 创建商标实体
    ///
    /// 创建商标实体。使用场景是签名来源=商标时，需要上传商标信息。
    ///
    /// 商标应在国家知识产权局商标局-中国商标网中可查，且商标所有方与企业名称一致。
    ///
    /// # Error Codes
    /// - `TrademarkOssFileNotUploadError`: The trademark screenshot file is not uploaded.
    /// - `TrademarkLetterDateNotMatchRegex`: The format of the trademark's validity period is incorrect.
    /// - `TrademarkNameNotMatchRegex`: The trademark name cannot be empty.
    /// - `TrademarkNameOverLimit`: The trademark name exceeds the length limit.
    /// - `TrademarkRegistrationNumberNotMatchRegex`: The trademark registration number cannot be empty.
    /// - `TrademarkRegistrationNumberOverLimit`: The trademark registration number exceeds the length limit.
    /// - `TrademarkApplicantNameNotMatchRegex`: The trademark applicant cannot be empty.
    /// - `TrademarkApplicantNameOverLimit`: The trademark applicant exceeds the length limit.
    /// - `TrademarkDateNotValid`: The trademark's validity period is not within the valid range.
    /// - `TrademarkPicsFileError`: The format of the trademark screenshot file is incorrect.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn create_sms_trademark(
        &self,
        req: CreateSmsTrademark,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsTrademarkResponse>> + Send {
        self.call(req)
    }

    /// # 查询商标实体
    ///
    /// 查询商标实体详情信息。
    ///
    /// 传入商标id列表，返回商标详情。
    ///
    /// 如查签名接口（QuerySmsSignList/GetSmsSign）会查出商标id，然后使用此接口进一步查询详情。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn query_sms_trademark(
        &self,
        req: QuerySmsTrademark,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsTrademarkResponse>> + Send {
        self.call(req)
    }

    /// # 创建ICP备案实体
    ///
    /// 创建APP-ICP备案实体。使用场景是签名来源=APP时，需要上传ICP备案信息。
    ///
    /// 签名来源选择已上线APP，则需要上传ICP备案截图。
    ///
    /// # Error Codes
    /// - `AppIcpRecordOssFileNotUploadError`: The APP-ICP record screenshot file is not uploaded.
    /// - `AppIcpRecordDateNotMatchRegex`: The format of the APP-ICP record approval date is incorrect.
    /// - `AppPrincipalUnitNameNotMatchRegex`: The principal unit name of the APP cannot be empty.
    /// - `AppPrincipalUnitNameOverLimit`: The principal unit name of the APP exceeds the length limit.
    /// - `AppIcpLicenseNumberNotMatchRegex`: The ICP record/license number cannot be empty.
    /// - `AppIcpLicenseNumberOverLimit`: The ICP record/license number exceeds the length limit.
    /// - `AppDomainNotMatchRegex`: The APP app store link cannot be empty and must start with http:// or https://.
    /// - `AppDomainOverLimit`: The APP app store link exceeds the length limit.
    /// - `AppApprovalDateNotValid`: The validity period of the APP-ICP record is not within the valid range.
    /// - `AppIcpRecordPicsFileError`: The format of the APP-ICP record screenshot file is incorrect.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn create_sms_app_icp_record(
        &self,
        req: CreateSmsAppIcpRecord,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsAppIcpRecordResponse>> + Send
    {
        self.call(req)
    }

    /// # 查询ICP备案实体
    ///
    /// 查询ICP备案详情信息。
    ///
    /// 传入ICP备案id列表，返回ICP备案详情。
    ///
    /// 如查签名接口（QuerySmsSignList/GetSmsSign）会查出ICP备案id，然后使用此接口进一步查询详情。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn query_sms_app_icp_record(
        &self,
        req: QuerySmsAppIcpRecord,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsAppIcpRecordResponse>> + Send {
        self.call(req)
    }

    /// # 申请短信模板（新接口）
    ///
    /// 短信模板即接收方收到短信的详细内容，包括变量和模板内容。您可以根据业务需要，申请验证码、通知短信或推广短信，模板审核通过后才可以发送短信。
    ///
    /// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
    ///
    /// - 通过接口申请短信模板，建议每次申请至少间隔30秒。
    ///
    /// - 通过接口申请的模板信息会同步在短信服务控制台，在控制台对模板的相关操作，请参见[短信模板](~~108085~~)。
    ///
    /// - 提交模板申请后，您可以通过[GetSmsTemplate](~~2807433~~)接口查询模板审核状态和详情。也可以[配置回执消息](~~101508~~)，通过[TemplateSmsReport](~~120999~~)获取模板的审核状态消息。
    ///
    /// - 国内短信模板与国际/港澳台短信模板不通用（不能混用），请根据业务使用场景申请模板。
    ///
    /// - 仅支持企业认证用户申请推广短信和国际/港澳台消息，个人用户与企业用户权益区别详情请参见[使用须知](~~55324~~)。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.
    /// - `AssocSignUnapproved`: Associated signature must be approved.
    /// - `InvalidTemplateRule.Format`: The parameter TemplateRule format must be JSON.
    /// - `InvalidMoreData`: Specified parameter MoreData is not valid.
    /// - `TemplateVarLimitExceeded`: The verification code template only supports one variable.
    /// - `InvalidTemplateContent.Format`: Invalid template content format.
    /// - `InvalidTemplateRule`: The template variable format is non-standard. Please refer to the variable format specifications in the help documentation.
    /// - `ServiceNotOpened`: This product service is not opened.
    /// - `MissingTemplateName`: The template  name cannot be empty.
    /// - `SmsSignatureNotFound`: The associated SMS signature does not exist.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn create_sms_template(
        &self,
        req: CreateSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsTemplateResponse>> + Send {
        self.call(req)
    }

    /// # 查询模板审核详情（新接口）
    ///
    /// 申请模板后，通过此接口查询模板审核详情，可查看模板审核状态。
    ///
    /// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
    /// - 审核时间：一般情况下，模板提交后，阿里云预计在2个小时内审核完成（审核工作时间：周一至周日9:00~21:00，法定节假日顺延）。
    ///
    /// - 如果模板未通过审核，会返回审核失败的原因，请参考[短信审核失败的处理建议](~~65990~~)，调用[UpdateSmsTemplate](~~UpdateSmsTemplate~~)接口或在[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面修改短信模板。
    ///
    /// - 当前接口是通过模板Code查询单个模板的审核详情。[QuerySmsTemplateList](~~419288~~)接口可以查询您当前账号下所有模板的模板详情。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.
    /// - `TemplateNotFound`: The template does not exist.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn get_sms_template(
        &self,
        req: GetSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<GetSmsTemplateResponse>> + Send {
        self.call(req)
    }

    /// # 查询模板列表详情
    ///
    /// 可以通过此接口查询您账号下的所有模板，方便您查看模板详情，包括模板审核状态、模板类型、模板内容等。
    ///
    /// - 本接口用于查询您当前账号下所有模板的模板详情。如果您需要查询模板变量内容、申请时上传的文件资料信息等更多内容，可以调用[GetSmsTemplate](~~GetSmsTemplate~~)接口通过模板Code查询单个模板审核详情。
    /// - 您也可登录短信服务控制台[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页查看您当前账号下所有模板的模板详情。
    ///
    /// # Methods
    /// - POST
    ///
    pub fn query_sms_template_list(
        &self,
        req: QuerySmsTemplateList,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsTemplateListResponse>> + Send {
        self.call(req)
    }

    /// # 修改短信模板（新接口）
    ///
    /// 修改未通过审核的模板，调用本接口修改后将自动提交审核。
    ///
    /// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
    /// - 仅支持修改未通过审核的模板，请参考[短信审核失败的处理建议](~~65990~~)，调用此接口修改后重新提交审核。
    ///
    /// - 通过接口修改模板的操作会在短信服务控制台同步，在控制台对模板的相关操作，请参见[短信模板](~~108085~~)。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.
    /// - `AssocSignUnapproved`: Associated signature must be approved.
    /// - `InvalidTemplateRule`: The template variable format is non-standard. Please refer to the variable format specifications in the help documentation.
    /// - `InvalidTemplateRule.Format`: The parameter TemplateRule format must be JSON.
    /// - `InvalidMoreData`: Specified parameter MoreData is not valid.
    /// - `TemplateVarLimitExceeded`: The verification code template only supports one variable.
    /// - `InvalidTemplateContent.Format`: Invalid template content format.
    /// - `ServiceNotOpened`: This product service is not opened.
    /// - `MissingTemplateName`: The template  name cannot be empty.
    /// - `SmsSignatureNotFound`: The associated SMS signature does not exist.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn update_sms_template(
        &self,
        req: UpdateSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<UpdateSmsTemplateResponse>> + Send {
        self.call(req)
    }

    /// # 删除短信模板
    ///
    /// 如果您不再使用某个短信模板，需要删除模板时，调用此接口或在短信服务控制台删除短信模板。
    ///
    /// - 支持删除已撤回、审核失败或审核通过的模板，审核中的短信模板不支持删除。
    /// - 删除短信模板后不可恢复，且后续不可再使用该模板发送短信，请谨慎操作。
    /// - 通过接口删除模板的操作会在短信服务控制台同步，在控制台对模板的相关操作，请参见[短信模板](~~108085~~)。
    ///
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn delete_sms_template(
        &self,
        req: DeleteSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<DeleteSmsTemplateResponse>> + Send {
        self.call(req)
    }

    /// # 申请短信模板（已下线）
    ///
    /// 该接口已下线。
    ///
    /// - 根据工信部规定与运营商[相关要求](~~2806975~~)，阿里云进行了模板相关API的功能改造。为提升您模板的审核效率和审核通过率，请您使用新接口[CreateSmsTemplate-申请短信模板](~~2807431~~)。
    ///
    /// - 通过接口申请短信模板，一个自然日最多可以提交100次短信模板申请。建议每次申请至少间隔30秒。通过[控制台](https://dysms.console.aliyun.com/domestic/text/template)申请短信模板，提交次数无限制。
    ///
    /// - 通过接口申请的模板信息会同步在短信服务控制台，在控制台对模板的相关操作，请参见[短信模板](~~108085~~)。
    ///
    /// - 提交模板申请后，您可以通过[QuerySmsTemplate](~~419289~~)接口查询模板审核状态和详情。也可以[配置回执消息](~~101508~~)，通过[TemplateSmsReport](~~120999~~)获取模板的审核状态消息。
    ///
    /// - 国内短信模板与国际/港澳台短信模板不通用（不能混用），请根据业务使用场景申请模板。
    ///
    /// - 仅支持企业认证用户申请推广短信和国际/港澳台消息，个人用户与企业用户权益区别详情请参见[使用须知](~~55324~~)。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `TemplateParameterCountIllegal`: The verification code template only supports 1 verification code as a variable///
    /// # Methods
    /// - POST
    /// - GET
    ///
    #[deprecated]
    pub fn add_sms_template(
        &self,
        req: AddSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<AddSmsTemplateResponse>> + Send {
        self.call(req)
    }

    /// # 修改短信模板（已下线）
    ///
    /// 该接口已下线。
    ///
    /// - 根据工信部规定与运营商[相关要求](~~2806975~~)，阿里云进行了模板相关API的功能改造。为提升您模板的审核效率和审核通过率，请您使用新接口[UpdateSmsTemplate-修改短信模板](~~2807432~~)。
    ///
    /// - 仅支持修改未通过审核的模板，请参考[短信审核失败的处理建议](~~65990~~)，调用此接口修改后重新提交审核。
    ///
    /// - 通过接口修改模板的操作会在短信服务控制台同步，在控制台对模板的相关操作，请参见[短信模板](~~108085~~)。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `TemplateParameterCountIllegal`: The verification code template only supports 1 verification code as a variable///
    /// # Methods
    /// - POST
    /// - GET
    ///
    #[deprecated]
    pub fn modify_sms_template(
        &self,
        req: ModifySmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<ModifySmsTemplateResponse>> + Send {
        self.call(req)
    }

    /// # 查询模板审核状态（已下线）
    ///
    /// 该接口已下线。
    ///
    /// - 根据工信部规定与运营商[相关要求](~~2806975~~)，阿里云进行了模板相关API的功能改造。请您使用新接口[GetSmsTemplate-查询模板审核详情](~~2807433~~)，新接口查询结果返回参数中将比原有接口返回更多的模板详情信息。
    ///
    /// - 审核时间：一般情况下，模板提交后，阿里云预计在2个小时内审核完成（审核工作时间：周一至周日9:00~21:00，法定节假日顺延），建议您尽量在18:00前提交申请。
    ///
    /// - 如果模板未通过审核，会返回审核失败的原因，请参考[短信审核失败的处理建议](~~65990~~)，调用[ModifySmsTemplate](~~419287~~)接口或在[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面修改短信模板。
    ///
    /// - QuerySmsTemplate当前接口是通过模板Code查询单个模板的审核详情。[QuerySmsTemplateList](~~419288~~)接口可以查询您当前账号下所有模板的模板详情。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    #[deprecated]
    pub fn query_sms_template(
        &self,
        req: QuerySmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsTemplateResponse>> + Send {
        self.call(req)
    }

    /// # 发送短信
    ///
    /// 向指定的手机号码发送短信。
    ///
    /// 本接口主要用于向单个手机号发送短信，也支持向多个手机号（单次最多支持 1000 个手机号）发送相同签名、相同模板变量的短信，群发存在一定延迟。如果您需要向多个手机号发送不同签名、不同模板变量的短信，请使用[SendBatchSms](~~419274~~)接口（单次最多支持100个手机号）。
    ///
    /// ### 注意事项
    /// - 国内短信服务超时时间建议设置为≥1S；发生超时失败的情况时，建议查看回执状态后再判断是否重试。超时和重试的相关设置，请参见[超时机制](~~262079~~)、[重试机制](~~262080~~)。
    /// - 国内短信、国际短信及多媒体短信目前均不支持幂等的能力，请您做好幂等控制，防止因多次重试而导致的重复操作问题。
    /// - 发送短信为计费接口，国内短信按照运营商回执状态计费，调用 SendSms 提交成功但运营商回执失败时不计费。计费详情请参见[计费概述](~~44340~~)。
    ///
    /// ### QPS 限制
    /// 本接口的单用户 QPS 限制为 5000/秒。超过限制，API 调用将会被限流，请合理使用。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn send_sms(
        &self,
        req: SendSms,
    ) -> impl std::future::Future<Output = crate::Result<SendSmsResponse>> + Send {
        self.call(req)
    }

    /// # 批量发送短信
    ///
    /// 调用此接口可以给不同的手机号码，发送不同签名、同一个模板（可以是不同模板变量）的短信。
    ///
    /// ### 基本信息
    /// - 本接口主要用于向多个手机号发送短信，支持发送不同签名、同一模板、不同模板变量的短信，单次调用最多支持 100 个手机号。
    /// - [服务接入点](~~419270~~) (Endpoint)：全局接入点域名为`dysmsapi.aliyuncs.com`。请参见[服务接入点](~~419270~~) ，根据您的使用地域，选择对应的接入点地址。
    /// ### 快速调用
    /// - 推荐您通过 SDK 调用 API。 如果您需要了解如何使用 ，请参见[首次调用API](~~2841024~~)。
    /// - 如果您需要使用控制台发送短信，请参见[群发短信](~~108266~~)。
    /// -  如果您需要自定义封装API调用，请参见[V3版本请求体&签名机制](~~2593177~~)
    /// ### 注意事项
    /// - 国内短信服务超时时间建议设置为≥1S；发生超时失败的情况时，建议查看回执状态后再判断是否重试。超时和重试的相关设置，请参见[超时机制](~~262079~~)、[重试机制](~~262080~~)。
    /// - 国内短信、国际短信及多媒体短信目前均不支持幂等的能力，请您做好幂等控制，防止因多次重试而导致的重复操作问题。
    /// - 发送短信为计费接口，国内短信按照运营商回执状态计费，调用SendBatchSms提交成功但运营商回执失败的短信不计费，计费详情请参见[计费概述](~~44340~~)。
    /// ### QPS 限制
    /// 本接口的单用户 QPS 限制为 5000/秒。超过限制，API 调用将会被限流，请合理使用。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn send_batch_sms(
        &self,
        req: SendBatchSms,
    ) -> impl std::future::Future<Output = crate::Result<SendBatchSmsResponse>> + Send {
        self.call(req)
    }

    /// # 查询短信发送详情
    ///
    /// 查询单个号码的短信发送记录和发送状态等信息。
    ///
    /// - 本接口主要用于查询指定日期下，向某个手机号码发送短信的记录详情。您也可以传入发送流水号（BizId），查询该号码的指定发送记录。
    ///
    /// - 本接口仅支持查询单个手机号码发送详情。如果需要批量查看短信发送详情，您可以使用[QuerySendStatistics](~~419276~~)接口，查询短信发送统计详情；或登录[控制台发送记录查询](https://dysms.console.aliyun.com/record)页面，查询发送详情。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为5000/秒。超过限制，API调用将会被限流，请合理使用。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn query_send_details(
        &self,
        req: QuerySendDetails,
    ) -> impl std::future::Future<Output = crate::Result<QuerySendDetailsResponse>> + Send {
        self.call(req)
    }

    /// # 查询短信发送量统计
    ///
    /// 查询短信发送统计详情，包括短信发送时间、短信发送成功条数、接收回执条数等。
    ///
    /// - 如果选择的时间范围较长的话，可以分页查看。指定每页显示的短信详情数量和查看的页数，即可分页查看发送记录。
    ///
    /// - 您可以登录[短信服务控制台](https://dysms.console.aliyun.com/dysms.htm#/overview)，在**业务统计**-**发送记录**页面查询发送详情。
    ///
    /// # Methods
    /// - POST
    ///
    pub fn query_send_statistics(
        &self,
        req: QuerySendStatistics,
    ) -> impl std::future::Future<Output = crate::Result<QuerySendStatisticsResponse>> + Send {
        self.call(req)
    }

    /// # 获取OSS上传信息
    ///
    /// 获取卡片短信所属OSS资源配置信息，此配置信息将用于后续OSS文件上传操作。
    ///
    /// - 您在调用卡片短信相关API接口前，首先需要开通卡片短信功能，目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或联系[阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html)。
    ///
    /// - 卡片短信模板中使用的图片、视频等素材资源可上传到OSS文件系统保存。文件上传操作，请参见[OSS文件上传](~~437303~~)。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为300次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - GET
    /// - POST
    ///
    pub fn get_oss_info_for_card_template(
        &self,
        req: GetOSSInfoForCardTemplate,
    ) -> impl std::future::Future<Output = crate::Result<GetOSSInfoForCardTemplateResponse>> + Send
    {
        self.call(req)
    }

    /// # 获取媒体资源ID
    ///
    /// 将用户上传到卡片短信OSS存储的图片、视频转换成（生成）资源数据统一管理，并返回资源ID，用户可以对返回的资源ID自行管理。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为300次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - POST
    ///
    pub fn get_media_resource_id(
        &self,
        req: GetMediaResourceId,
    ) -> impl std::future::Future<Output = crate::Result<GetMediaResourceIdResponse>> + Send {
        self.call(req)
    }

    /// # 创建卡片短信模板
    ///
    /// 创建卡片短信模板。
    ///
    /// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或联系[阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html?spm=a2c4g.11186623.0.0.213273d4Xe6UEu#section-81n-72q-ybm)。
    ///
    /// - 保存卡片短信模板信息，提交手机厂商审核，并返回模板Code。
    /// - 使用CreateCardSmsTemplate创建卡片短信模板，若模板中包含厂商不支持的模板类型或事件时，将不向对应厂商提交审核。更多信息，请参见[厂商支持的模板说明](~~434611~~)。
    /// - 更多卡片短信模板的示例，请参见[卡片短信模板示例](~~435361~~)。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为300次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - POST
    ///
    pub fn create_card_sms_template(
        &self,
        req: CreateCardSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<CreateCardSmsTemplateResponse>> + Send
    {
        self.call(req)
    }

    /// # 查询卡片短信模板状态
    ///
    /// 查询卡片短信模板审核状态，返回手机厂商审核相关信息。
    ///
    /// - 未开通卡片短信业务的阿里云账号无法调用此API。
    /// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或[联系阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html)。
    /// - 您也可登录控制台[国内卡片短信](https://dysms.console.aliyun.com/domestic/card)页面，在模板管理页签内查询卡片短信模板的审核状态。
    /// ### QPS限制
    /// 本接口的单用户QPS限制为300次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - GET
    /// - POST
    ///
    pub fn query_card_sms_template(
        &self,
        req: QueryCardSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<QueryCardSmsTemplateResponse>> + Send {
        self.call(req)
    }

    /// # 检查手机号是否支持卡片短信（旧接口）
    ///
    /// 检查手机号码是否支持卡片短信。
    ///
    /// - 未开通卡片短信业务的阿里云账号无法调用此API。
    /// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或[联系阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html)。
    /// - 推荐使用新接口[QueryMobilesCardSupport](~~QueryMobilesCardSupport~~)查询手机号是否支持卡片短信。
    /// ### QPS限制
    /// 本接口的单用户QPS限制为2000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - GET
    /// - POST
    ///
    pub fn check_mobiles_card_support(
        &self,
        req: CheckMobilesCardSupport,
    ) -> impl std::future::Future<Output = crate::Result<CheckMobilesCardSupportResponse>> + Send
    {
        self.call(req)
    }

    /// # 查询手机号是否支持卡片短信（新接口）
    ///
    /// 查询手机号是否支持卡片短信。
    ///
    /// - 未开通卡片短信业务的阿里云账号无法调用此API。
    /// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或[联系阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html)。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - GET
    /// - POST
    ///
    pub fn query_mobiles_card_support(
        &self,
        req: QueryMobilesCardSupport,
    ) -> impl std::future::Future<Output = crate::Result<QueryMobilesCardSupportResponse>> + Send
    {
        self.call(req)
    }

    /// # 获取卡片短信短链
    ///
    /// 获取卡片短信短链。
    ///
    /// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或联系[阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html?spm=a2c4g.11186623.0.0.213273d4Xe6UEu#section-81n-72q-ybm)。
    ///
    /// ### QPS限制
    /// - 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - GET
    /// - POST
    ///
    pub fn get_card_sms_link(
        &self,
        req: GetCardSmsLink,
    ) -> impl std::future::Future<Output = crate::Result<GetCardSmsLinkResponse>> + Send {
        self.call(req)
    }

    /// # 查询单个号码的卡片短信发送记录和发送状态等信息
    ///
    /// 查询单个号码的卡片短信发送记录和发送状态等信息。
    ///
    /// # Error Codes
    /// - `InvalidParam.PhoneNumber`: Incorrect phone number format.
    /// - `InvalidParam.PageSize`: PageSize must be less than or equal to 50.
    /// - `InvalidParam.SendDate`: Only the last 30 days can be queried.
    /// - `InvalidParameter`: At most, only one parameter can be passed among bizCardId, bizSmsId, and bizDigitId.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn get_card_sms_details(
        &self,
        req: GetCardSmsDetails,
    ) -> impl std::future::Future<Output = crate::Result<GetCardSmsDetailsResponse>> + Send {
        self.call(req)
    }

    /// # 查询卡片短信指定模板的解析数据
    ///
    /// 查询卡片短信指定模板的解析数据，解析数据包括短信解析回执成功数、消息曝光次数和消息点击数等。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为300次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - GET
    /// - POST
    ///
    pub fn query_card_sms_template_report(
        &self,
        req: QueryCardSmsTemplateReport,
    ) -> impl std::future::Future<Output = crate::Result<QueryCardSmsTemplateReportResponse>> + Send
    {
        self.call(req)
    }

    /// # 发送卡片短信
    ///
    /// 发送卡片短信。
    ///
    /// - 发送卡片短信为计费接口，卡片短信发送失败或渲染失败时不计费，详情请参见[多媒体短信定价](~~437951~~)。
    /// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或联系[阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html?spm=a2c4g.11186623.0.0.213219fcSn2Ykj#section-81n-72q-ybm)。
    /// - 卡片短信超时时间建议设置为≥3S；发生超时失败的情况时，建议查看回执状态后再判断是否重试。同时建议您在调用此接口时，不要开启SDK重试逻辑，否则可能会造成多次发送的情况。超时和重试的相关设置，请参见[超时机制](~~262079~~)、[重试机制](~~262080~~)。
    /// - 国内短信、国际短信及多媒体短信目前均不支持幂等的能力，请您做好幂等控制，防止因多次重试而导致的重复操作问题。
    /// - 发送卡片短信前需添加卡片短信模板且模板审核通过。本接口在发送短信时，会校验号码是否支持卡片短信。如果该手机号不支持发送卡片短信，可在接口中设置是否接受数字短信和文本短信的回落，提升发送的触达率。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - GET
    /// - POST
    ///
    pub fn send_card_sms(
        &self,
        req: SendCardSms,
    ) -> impl std::future::Future<Output = crate::Result<SendCardSmsResponse>> + Send {
        self.call(req)
    }

    /// # 批量发送卡片短信
    ///
    /// 批量发送卡片短信。
    ///
    /// - 发送卡片短信为计费接口，卡片短信发送失败或渲染失败时不计费，详情请参见[多媒体短信定价](~~437951~~)。
    /// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或联系[阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html?spm=a2c4g.11186623.0.0.213219fcSn2Ykj#section-81n-72q-ybm)。
    /// - 卡片短信超时时间建议设置为≥3S；发生超时失败的情况时，建议查看回执状态后再判断是否重试。同时建议您在调用此接口时，不要开启SDK重试逻辑，否则可能会造成多次发送的情况。超时和重试的相关设置，请参见[超时机制](~~262079~~)、[重试机制](~~262080~~)。
    /// - 国内短信、国际短信及多媒体短信目前均不支持幂等的能力，请您做好幂等控制，防止因多次重试而导致的重复操作问题。
    /// - 发送卡片短信前需添加卡片短信模板且模板审核通过。本接口在发送短信时，会校验号码是否支持卡片短信。如果该手机号不支持发送卡片短信，可在接口中设置是否接受数字短信和文本短信的回落，提升发送的触达率。
    /// - 批量发送卡片短信，每个号码可以使用不同的签名，不同的回落。在一次请求中，最多可以向100个手机号码分别发送卡片短信。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - GET
    /// - POST
    ///
    pub fn send_batch_card_sms(
        &self,
        req: SendBatchCardSms,
    ) -> impl std::future::Future<Output = crate::Result<SendBatchCardSmsResponse>> + Send {
        self.call(req)
    }

    /// # 获取资质材料/授权书OSS配置信息
    ///
    /// 获取资质材料OSS资源配置信息，此配置信息将用于后续授权委托书、企业证件等资质文件的上传操作。
    ///
    /// - 您在申请资质/签名时，若用途为他用或涉及第三方，需要提供[授权委托书](~~56741~~)。
    /// - 请使用本接口获取OSS资源配置信息后，通过OSS上传相关资质材料。具体操作，可参见[通过OSS上传文件](~~2833114~~)。
    /// - 待上传的文件命名不支持包含中文和特殊符号。
    ///
    /// # Error Codes
    /// - `OssBiztypeNotSupportError`: Retrieving OSS configuration does not support this biz type.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn get_qualification_oss_info(
        &self,
        req: GetQualificationOssInfo,
    ) -> impl std::future::Future<Output = crate::Result<GetQualificationOssInfoResponse>> + Send
    {
        self.call(req)
    }

    /// # 获取签名/模板材料OSS配置信息
    ///
    /// 获取OSS资源配置信息，此配置信息将用于后续OSS文件上传操作。
    ///
    /// - 您在创建签名或模板时，可上传带有链接的登录页面、后台页面截图、软著、协议补充等资料。有助于审核人员了解您的业务详情。如果是多个资料，可拼成一个文件，支持png、jpg、jpeg、doc、docx、pdf格式。
    ///
    /// - 创建签名或模板所需的更多资料，可上传到OSS文件系统保存。文件上传操作，请参见[OSS上传文件](~~2833114~~)。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn get_oss_info_for_upload_file(
        &self,
        req: GetOSSInfoForUploadFile,
    ) -> impl std::future::Future<Output = crate::Result<GetOSSInfoForUploadFileResponse>> + Send
    {
        self.call(req)
    }

    /// # 获取OCR图片识别OSS信息
    ///
    /// 获取 OCR 的 OSS 信息。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn get_sms_ocr_oss_info(
        &self,
        req: GetSmsOcrOssInfo,
    ) -> impl std::future::Future<Output = crate::Result<GetSmsOcrOssInfoResponse>> + Send {
        self.call(req)
    }

    /// # 国内发国际短信转化反馈
    ///
    /// 将每一条消息ID(MessageId) 对应短信的接收情况反馈给阿里云国际短信平台。
    ///
    /// 指标说明：
    ///
    /// - OTP发送量：验证码发送量。
    ///
    /// - OTP转化量：验证码转换量。（用户成功获取验证码，并进行回传）
    ///
    /// 转化率=OTP转化量/OTP发送量。
    ///
    /// > 转化率反馈功能会对业务系统有一定的侵入性，为了防止调用转化率API的抖动影响业务逻辑，请考虑：  - 使用异步模式（例如：队列或事件驱动）调用API。  - 添加可降级的方案保护业务逻辑（例如：手动降级开工或者使用断路器自动降级）。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn sms_conversion_intl(
        &self,
        req: SmsConversionIntl,
    ) -> impl std::future::Future<Output = crate::Result<SmsConversionIntlResponse>> + Send {
        self.call(req)
    }

    /// # 国内发国际转化率数据接入API
    ///
    /// 将短信转化率统计数据反馈给阿里云短信平台。
    ///
    /// 指标说明：转化率=OTP 转化量/OTP 发送量。
    /// - OTP发送量：验证码发送量。
    /// - OTP转化量：验证码转换量。（用户成功获取验证码，并进行回传）
    /// >转化率反馈功能会对业务系统有一定的侵入性，为了防止调用转化率 API 的抖动影响业务逻辑，请考虑：
    /// >- 使用异步模式（例如：队列或事件驱动）调用 API。
    /// >- 添加可降级的方案保护业务逻辑（例如：手动降级开工或者使用断路器自动降级）。
    ///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn conversion_data_intl(
        &self,
        req: ConversionDataIntl,
    ) -> impl std::future::Future<Output = crate::Result<ConversionDataIntlResponse>> + Send {
        self.call(req)
    }

    /// # 创建短链
    ///
    /// 创建短链。
    ///
    /// ><notice>短信服务暂时不支持使用此接口。></notice>
    ///
    /// - 一个自然日最多可以创建3000个短链。
    /// - 短链接生成后，需要进行安全审核，审核时长一般为10分钟~2小时，安全审核通过之前，短链接无法直接访问。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn add_short_url(
        &self,
        req: AddShortUrl,
    ) -> impl std::future::Future<Output = crate::Result<AddShortUrlResponse>> + Send {
        self.call(req)
    }

    /// # 删除短链
    ///
    /// 删除短链，删除后短链将无法使用，无法还原为原链。
    ///
    /// ><notice>短信服务暂时不支持使用此接口。></notice>
    /// ### QPS限制
    /// 本接口的单用户QPS限制为100次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - POST
    /// - GET
    /// - DELETE
    ///
    pub fn delete_short_url(
        &self,
        req: DeleteShortUrl,
    ) -> impl std::future::Future<Output = crate::Result<DeleteShortUrlResponse>> + Send {
        self.call(req)
    }

    /// # 短链状态查询
    ///
    /// 查询短链状态，可判断短链是否可用。
    ///
    /// ><notice>短信服务暂时不支持使用此接口。></notice>
    /// ### QPS限制
    /// 本接口的单用户QPS限制为20次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn query_short_url(
        &self,
        req: QueryShortUrl,
    ) -> impl std::future::Future<Output = crate::Result<QueryShortUrlResponse>> + Send {
        self.call(req)
    }

    /// # 查询模板标签
    ///
    /// 标签是您为模板添加的标记，每个标签都由一对键值对（Key-Value）组成。您可以查询标签键值对绑定的模板Code，也可以查询某个模板已绑定的所有标签。
    ///
    /// 您可登录短信服务控制台[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面筛选标签键值对已绑定的短信模板，或单击操作列**详情**按钮查看当前模板已绑定的标签。
    /// ### QPS限制
    /// 本接口的单用户QPS限制为50次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn list_tag_resources(
        &self,
        req: ListTagResources,
    ) -> impl std::future::Future<Output = crate::Result<ListTagResourcesResponse>> + Send {
        self.call(req)
    }

    /// # 添加模板标签
    ///
    /// 标签可以标记资源，允许企业或个人将同类型的模板进行资源归类，便于搜索和资源聚合。调用本接口对短信模板进行标签绑定。
    ///
    /// - 每个模板最多可以绑定20个标签。
    /// - 同一个模板的标签键（Key）必须唯一，一个模板如果设置了同Key不同Value的两个标签，新值将覆盖旧值。
    /// - 此功能仅适用于中国站短信服务的国内文本短信。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为50次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn tag_resources(
        &self,
        req: TagResources,
    ) -> impl std::future::Future<Output = crate::Result<TagResourcesResponse>> + Send {
        self.call(req)
    }

    /// # 删除模板标签
    ///
    /// 标签可以标记资源，允许企业或个人将同类型的模板进行资源归类，便于搜索和资源聚合。如果模板不再适用于当前已绑定的标签，可以从模板中解绑标签。您可以删除单个标签，也可以批量删除标签。
    ///
    /// ### QPS限制
    /// 本接口的单用户QPS限制为50次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
    ///
    /// # Error Codes
    /// - `ForbiddenAction`: Access to the account is denied. Please contact the administrator.///
    /// # Methods
    /// - POST
    /// - GET
    ///
    pub fn untag_resources(
        &self,
        req: UntagResources,
    ) -> impl std::future::Future<Output = crate::Result<UntagResourcesResponse>> + Send {
        self.call(req)
    }
}

/// ><notice>
///
/// 短信资质材料具体要求请参见[资质材料说明](~~2384377~~)，要求可能随工信部与运营商要求实时调整，请以审核实际结果为准。
///
/// ></notice>
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SubmitSmsQualification {
    /// 资质名称，用于管理、区分您申请的多个资质，不会出现在短信内容中。名称不可与已有资质名称重复，仅支持中文、英文或与数字组合，不支持任何符号或纯数字输入，长度不超过100字符。
    qualification_name: String,
    /// 资质申请用途，取值：
    ///
    /// - **true**：**自用**，签名所属主体与本账号实名认证的主体一致。
    /// - **false**：**他用**，签名所属主体与本账号实名认证的主体不一致。
    use_by_self: bool,
    /// 企业类型，取值：
    ///
    /// - COMPANY：企业。
    ///
    /// - NON_PROFIT_ORGANIZATION：政府机关或事业单位。
    company_type: String,
    /// 企业营业证件信息。资质用途`UseBySelf`选择`false`他用时，此参数必填。
    #[setters(generate = true, strip_option)]
    business_license_pics: Option<Vec<SubmitSmsQualificationBusinessLicensePic>>,
    /// 企业名称。符号仅支持中点`·`、中文`【】（）`、英文`()`及`空格`，不可含其他符号或纯数字，长度不超过150字符。
    company_name: String,
    /// 社会统一信用代码，长度不超过150字符。
    organization_code: String,
    /// 营业证件有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    /// > 证照有效期为长期时，截止日期可填：2099-12-31。
    bussiness_license_exp_date: String,
    /// 法定代表人证件正面照片（身份证国徽面），仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    ///
    ///
    /// > 系统会使用您填写的法人姓名、证件号码进行校验；若校验不通过，则需要上传法定代表人身份证件照片。
    #[setters(generate = true, strip_option)]
    legal_person_id_card_front_side: Option<String>,
    /// 法定代表人证件反面照片（身份证人像面），仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    ///
    /// > 系统会使用您填写的法人姓名、证件号码进行校验；若校验不通过，则需要上传法定代表人身份证件照片。
    #[setters(generate = true, strip_option)]
    legal_person_id_card_back_side: Option<String>,
    /// 法定代表人姓名，长度不超过50字符。
    ///
    /// > - 若组织证件中无法定代表人信息，但存在负责人/首席代表等相关信息，请准备证件中对应负责人或首席代表的身份证件照片。
    /// > - 若组织证件中无法定代表人信息，且无任何负责人信息，请准备业务主要负责人的姓名、身份证件照片。
    legal_person_name: String,
    /// 法定代表人证件号码。
    legal_person_id_card_no: String,
    /// 法定代表人证件有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    /// > 证件有效期为长期时，截止日期可填：2099-12-31。
    legal_person_id_card_eff_time: String,
    /// 法定代表人证件类型。取值：
    ///
    /// - identityCard：身份证。
    /// - passport：护照。
    /// - homeReturnPermit：港澳居民来往内地通行证。
    /// - TaiwanCompatriotPermit：台湾居民来往大陆通行证。
    /// - residencePermit：港澳台居民居住证。
    /// - other：其他。
    legal_person_id_card_type: String,
    /// 管理员证件反面照片（身份证人像面），仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    ///
    /// ><notice>
    /// 证件的彩色原件无需盖章，若上传复印件/黑白照片，需要在复印件上加盖企业红章并拍照上传。
    ///
    /// ></notice>
    admin_id_card_pic: String,
    /// 管理员证件正面照片（身份证国徽面），仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符上传操作请参见通过[OSS上传文件](~~2833114~~)。
    ///
    /// ><notice>
    /// 证件的彩色原件无需盖章，若上传复印件/黑白照片，需要在复印件上加盖企业红章并拍照上传。
    ///
    /// ></notice>
    admin_id_card_front_face: String,
    /// 管理员姓名，长度不超过50字符。**在当前的[短信签名实名制](~~2873145~~)要求下，同一个管理员若申请多个不同的企业资质会导致报备失败，请确认一管一企以提升报备成功率。**
    ///
    /// > 管理员（又称经办人）指登录阿里云账号并管理短信业务的人员，一般是贵方管理此阿里云账号下资质、签名和模板，并进行短信发送的相关运营人员，且此人手机号可接收验证码。管理员不一定是此阿里云账号的管理员，管理员可以与企业法人为同一人。
    admin_name: String,
    /// 管理员证件号码。
    admin_id_card_no: String,
    /// 管理员证件类型。取值：
    ///
    /// - identityCard：身份证。
    /// - passport：护照。
    /// - homeReturnPermit：港澳居民来往内地通行证。
    /// - TaiwanCompatriotPermit：台湾居民来往大陆通行证。
    /// - residencePermit：港澳台居民居住证。
    /// - other：其他。
    admin_id_card_type: String,
    /// 管理员证件有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    /// > 证件有效期为长期时，截止日期可填：2099-12-31。
    admin_id_card_exp_date: String,
    /// 管理员手机号码，格式：+/+86/0086/86 或无任何前缀的手机号码，例如1390000****。
    admin_phone_no: String,
    /// 手机号验证码。请调用[RequiredPhoneCode](~~RequiredPhoneCode~~)接口并传入**管理员手机号码**后，在此填入接收到的短信验证码。
    ///
    /// > 您可以使用[ValidPhoneCode](~~ValidPhoneCode~~)自行校验短信验证码是否准确后再传入。
    certify_code: String,
    /// 更多资料，如果您还有其他证明或备注材料、照片等，可在此上传。
    #[setters(generate = true, strip_option)]
    other_files: Option<Vec<SubmitSmsQualificationOtherFile>>,
    /// 资质授权，是否同意与其他云通信产品（如国内语音、国内号码隐私保护）的资质共享。仅当您申请**自用资质**，且资质信息**与当前阿里云账号认证企业信息一致**时可被共享、复用；其他情况无效。取值：
    ///
    /// - true：同意，您的资质信息可在其他云通信产品的“资质认证环节”调用，免除重复认证环节。
    /// - false：不同意。
    whether_share: bool,
    /// 备注。如果您还有其他信息需要说明，或者需要给资质审核人员备注说明，可在此添加描述。
    #[setters(generate = true, strip_option)]
    remark: Option<String>,
}

impl sealed::Bound for SubmitSmsQualification {}

impl SubmitSmsQualification {
    pub fn new(
        qualification_name: impl Into<String>,
        use_by_self: impl Into<bool>,
        company_type: impl Into<String>,
        company_name: impl Into<String>,
        organization_code: impl Into<String>,
        bussiness_license_exp_date: impl Into<String>,
        legal_person_name: impl Into<String>,
        legal_person_id_card_no: impl Into<String>,
        legal_person_id_card_eff_time: impl Into<String>,
        legal_person_id_card_type: impl Into<String>,
        admin_id_card_pic: impl Into<String>,
        admin_id_card_front_face: impl Into<String>,
        admin_name: impl Into<String>,
        admin_id_card_no: impl Into<String>,
        admin_id_card_type: impl Into<String>,
        admin_id_card_exp_date: impl Into<String>,
        admin_phone_no: impl Into<String>,
        certify_code: impl Into<String>,
        whether_share: impl Into<bool>,
    ) -> Self {
        Self {
            qualification_name: qualification_name.into(),
            use_by_self: use_by_self.into(),
            company_type: company_type.into(),
            business_license_pics: None,
            company_name: company_name.into(),
            organization_code: organization_code.into(),
            bussiness_license_exp_date: bussiness_license_exp_date.into(),
            legal_person_id_card_front_side: None,
            legal_person_id_card_back_side: None,
            legal_person_name: legal_person_name.into(),
            legal_person_id_card_no: legal_person_id_card_no.into(),
            legal_person_id_card_eff_time: legal_person_id_card_eff_time.into(),
            legal_person_id_card_type: legal_person_id_card_type.into(),
            admin_id_card_pic: admin_id_card_pic.into(),
            admin_id_card_front_face: admin_id_card_front_face.into(),
            admin_name: admin_name.into(),
            admin_id_card_no: admin_id_card_no.into(),
            admin_id_card_type: admin_id_card_type.into(),
            admin_id_card_exp_date: admin_id_card_exp_date.into(),
            admin_phone_no: admin_phone_no.into(),
            certify_code: certify_code.into(),
            other_files: None,
            whether_share: whether_share.into(),
            remark: None,
        }
    }
}
impl crate::ToFormData for SubmitSmsQualification {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for SubmitSmsQualification {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SubmitSmsQualification";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<SubmitSmsQualificationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(24);
        params.push((
            "AdminIDCardExpDate".into(),
            (&self.admin_id_card_exp_date).into(),
        ));
        params.push((
            "AdminIDCardFrontFace".into(),
            (&self.admin_id_card_front_face).into(),
        ));
        params.push(("AdminIDCardNo".into(), (&self.admin_id_card_no).into()));
        params.push(("AdminIDCardPic".into(), (&self.admin_id_card_pic).into()));
        params.push(("AdminIDCardType".into(), (&self.admin_id_card_type).into()));
        params.push(("AdminName".into(), (&self.admin_name).into()));
        params.push(("AdminPhoneNo".into(), (&self.admin_phone_no).into()));

        if let Some(f) = &self.business_license_pics {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("BusinessLicensePics".into(), json.into()));
            }
        }
        params.push((
            "BussinessLicenseExpDate".into(),
            (&self.bussiness_license_exp_date).into(),
        ));
        params.push(("CertifyCode".into(), (&self.certify_code).into()));
        params.push(("CompanyName".into(), (&self.company_name).into()));
        params.push(("CompanyType".into(), (&self.company_type).into()));
        params.push((
            "LegalPersonIDCardNo".into(),
            (&self.legal_person_id_card_no).into(),
        ));
        params.push((
            "LegalPersonIDCardType".into(),
            (&self.legal_person_id_card_type).into(),
        ));

        if let Some(f) = &self.legal_person_id_card_back_side {
            params.push(("LegalPersonIdCardBackSide".into(), (f).into()));
        }
        params.push((
            "LegalPersonIdCardEffTime".into(),
            (&self.legal_person_id_card_eff_time).into(),
        ));

        if let Some(f) = &self.legal_person_id_card_front_side {
            params.push(("LegalPersonIdCardFrontSide".into(), (f).into()));
        }
        params.push(("LegalPersonName".into(), (&self.legal_person_name).into()));
        params.push(("OrganizationCode".into(), (&self.organization_code).into()));

        if let Some(f) = &self.other_files {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("OtherFiles".into(), json.into()));
            }
        }
        params.push((
            "QualificationName".into(),
            (&self.qualification_name).into(),
        ));

        if let Some(f) = &self.remark {
            params.push(("Remark".into(), (f).into()));
        }
        params.push(("UseBySelf".into(), (&self.use_by_self).into()));
        params.push(("WhetherShare".into(), (&self.whether_share).into()));

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
pub struct QuerySmsQualificationRecord {
    /// 资质名称。
    #[setters(generate = true, strip_option)]
    qualification_group_name: Option<String>,
    /// 企业名称。
    #[setters(generate = true, strip_option)]
    company_name: Option<String>,
    /// 审核状态。取值：
    ///
    /// - INIT：审核中。
    /// - NOT_PASS：审核不通过。
    /// - PASS：审核通过。
    /// - NOT_FINISH：资料待补充。
    /// - CANCEL：已撤回。
    #[setters(generate = true, strip_option)]
    state: Option<String>,
    /// 审核工单ID。
    #[setters(generate = true, strip_option)]
    work_order_id: Option<i64>,
    /// 法人姓名。
    #[setters(generate = true, strip_option)]
    legal_person_name: Option<String>,
    /// 资质申请用途，取值：
    ///
    /// - **true**：自用。
    /// - **false**：他用。
    #[setters(generate = true, strip_option)]
    use_by_self: Option<bool>,
    /// 当前页码。默认取值为 1。
    #[setters(generate = true, strip_option)]
    page_no: Option<i64>,
    /// 每页显示的数据条数。取值范围：**1~50**。
    #[setters(generate = true, strip_option)]
    page_size: Option<i64>,
}

impl sealed::Bound for QuerySmsQualificationRecord {}

impl QuerySmsQualificationRecord {
    pub fn new() -> Self {
        Self {
            qualification_group_name: None,
            company_name: None,
            state: None,
            work_order_id: None,
            legal_person_name: None,
            use_by_self: None,
            page_no: None,
            page_size: None,
        }
    }
}
impl crate::ToFormData for QuerySmsQualificationRecord {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySmsQualificationRecord {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySmsQualificationRecord";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySmsQualificationRecordResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(8);

        if let Some(f) = &self.company_name {
            params.push(("CompanyName".into(), (f).into()));
        }

        if let Some(f) = &self.legal_person_name {
            params.push(("LegalPersonName".into(), (f).into()));
        }

        if let Some(f) = &self.page_no {
            params.push(("PageNo".into(), (f).into()));
        }

        if let Some(f) = &self.page_size {
            params.push(("PageSize".into(), (f).into()));
        }

        if let Some(f) = &self.qualification_group_name {
            params.push(("QualificationGroupName".into(), (f).into()));
        }

        if let Some(f) = &self.state {
            params.push(("State".into(), (f).into()));
        }

        if let Some(f) = &self.use_by_self {
            params.push(("UseBySelf".into(), (f).into()));
        }

        if let Some(f) = &self.work_order_id {
            params.push(("WorkOrderId".into(), (f).into()));
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
pub struct QuerySingleSmsQualification {
    /// 资质ID，即您[申请资质](~~SubmitSmsQualification~~)返回的ID。您可以通过[查询资质列表](~~QuerySmsQualificationRecord~~)获取当前账号下的资质ID。
    qualification_group_id: i64,
    /// 审核工单ID，您可以通过[查询资质列表](~~QuerySmsQualificationRecord~~)获取当前账号下的资质及其对应审核工单ID。
    #[setters(generate = true, strip_option)]
    order_id: Option<i64>,
}

impl sealed::Bound for QuerySingleSmsQualification {}

impl QuerySingleSmsQualification {
    pub fn new(qualification_group_id: impl Into<i64>) -> Self {
        Self {
            qualification_group_id: qualification_group_id.into(),
            order_id: None,
        }
    }
}
impl crate::ToFormData for QuerySingleSmsQualification {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySingleSmsQualification {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySingleSmsQualification";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySingleSmsQualificationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

        if let Some(f) = &self.order_id {
            params.push(("OrderId".into(), (f).into()));
        }
        params.push((
            "QualificationGroupId".into(),
            (&self.qualification_group_id).into(),
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

/// ><notice>
///
/// 短信资质材料具体要求请参见[资质材料说明](~~2384377~~)，要求可能随工信部与运营商要求实时调整，请以审核实际结果为准。
///
/// ></notice>
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateSmsQualification {
    /// 资质ID，即您[申请短信资质](~~SubmitSmsQualification~~)返回的ID。您可以通过[查询资质列表](~~QuerySmsQualificationRecord~~)获取当前账号下的资质ID。
    qualification_group_id: i64,
    /// 审核工单ID。您可以通过[查询资质列表](~~QuerySmsQualificationRecord~~)获取当前账号下的资质及其对应审核工单ID。
    order_id: i64,
    /// 企业营业证件信息。待修改的资质用途为他用时，此参数必填。
    #[setters(generate = true, strip_option)]
    business_license_pics: Option<Vec<UpdateSmsQualificationBusinessLicensePic>>,
    /// 企业名称。符号仅支持中点`·`、中文`【】（）`、英文`()`及`空格`，不可含其他符号或纯数字，长度不超过150字符。
    #[setters(generate = true, strip_option)]
    company_name: Option<String>,
    /// 营业证件有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    /// > 证照有效期为长期时，截止日期可填：2099-12-31。
    #[setters(generate = true, strip_option)]
    bussiness_license_exp_date: Option<String>,
    /// 法定代表人证件正面照片（身份证国徽面），仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    /// > 系统会使用您填写的法人姓名、证件号码进行校验；若校验不通过，则需要上传法定代表人身份证件照片。
    #[setters(generate = true, strip_option)]
    legal_person_id_card_front_side: Option<String>,
    /// 法定代表人证件反面照片（身份证人像面），仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    /// > 系统会使用您填写的法人姓名、证件号码进行校验；若校验不通过，则需要上传法定代表人身份证件照片。
    #[setters(generate = true, strip_option)]
    legal_person_id_card_back_side: Option<String>,
    /// 法定代表人姓名。
    ///
    /// > - 若组织证件中无法定代表人信息，但存在负责人/首席代表等相关信息，请准备证件中对应负责人或首席代表的身份证件照片。
    /// > - 若组织证件中无法定代表人信息，且无任何负责人信息，请准备业务主要负责人的姓名、身份证件照片。
    #[setters(generate = true, strip_option)]
    legal_person_name: Option<String>,
    /// 法人证件号码。
    #[setters(generate = true, strip_option)]
    legal_person_id_card_no: Option<String>,
    /// 法人证件有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    /// > 证件有效期为长期时，截止日期可填：2099-12-31。
    #[setters(generate = true, strip_option)]
    legal_person_id_card_eff_time: Option<String>,
    /// 法人证件类型。取值：
    ///
    /// - identityCard：身份证。
    /// - passport：护照。
    /// - homeReturnPermit：港澳居民来往内地通行证。
    /// - TaiwanCompatriotPermit：台湾居民来往大陆通行证。
    /// - residencePermit：港澳台居民居住证。
    /// - other：其他。
    #[setters(generate = true, strip_option)]
    legal_person_id_card_type: Option<String>,
    /// 管理员证件反面照片（身份证人像面），仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    ///
    /// ><notice>
    ///  证件的彩色原件无需盖章，若上传复印件/黑白照片，需要在复印件上加盖企业红章并拍照上传。
    /// ></notice>
    #[setters(generate = true, strip_option)]
    admin_id_card_pic: Option<String>,
    /// 管理员证件正面照片（身份证国徽面），仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    ///
    /// ><notice>
    /// 证件的彩色原件无需盖章，若上传复印件/黑白照片，需要在复印件上加盖企业红章并拍照上传。
    /// ></notice>
    #[setters(generate = true, strip_option)]
    admin_id_card_front_face: Option<String>,
    /// 管理员姓名。
    ///
    /// > 管理员（又称经办人）指登录阿里云账号并管理短信业务的人员，一般是贵方管理此阿里云账号下资质、签名和模板，并进行短信发送的相关运营人员，且此人手机号可接收验证码。管理员不一定是此阿里云账号的管理员，管理员可以与企业法人为同一人。
    #[setters(generate = true, strip_option)]
    admin_name: Option<String>,
    /// 管理员证件号码。
    #[setters(generate = true, strip_option)]
    admin_id_card_no: Option<String>,
    /// 管理员证件有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    /// > 证件有效期为长期时，截止日期可填：2099-12-31。
    #[setters(generate = true, strip_option)]
    admin_id_card_exp_date: Option<String>,
    /// 管理员手机号码，格式：+/+86/0086/86 或无任何前缀的手机号码，例如1390000****。
    admin_phone_no: String,
    /// 手机号验证码。请调用[RequiredPhoneCode](~~RequiredPhoneCode~~)接口并传入**管理员手机号码**后，在此填入接收到的短信验证码。
    ///
    /// > 您可以使用[ValidPhoneCode](~~ValidPhoneCode~~)自行校验短信验证码是否准确后再传入。
    certify_code: String,
    /// 管理员证件类型。取值：
    ///
    /// - identityCard：身份证。
    /// - passport：护照。
    /// - homeReturnPermit：港澳居民来往内地通行证。
    /// - TaiwanCompatriotPermit：台湾居民来往大陆通行证。
    /// - residencePermit：港澳台居民居住证。
    /// - other：其他。
    #[setters(generate = true, strip_option)]
    admin_id_card_type: Option<String>,
    /// 更多资料，如果您还有其他证明或备注材料、照片等，可在此上传。
    #[setters(generate = true, strip_option)]
    other_files: Option<Vec<UpdateSmsQualificationOtherFile>>,
}

impl sealed::Bound for UpdateSmsQualification {}

impl UpdateSmsQualification {
    pub fn new(
        qualification_group_id: impl Into<i64>,
        order_id: impl Into<i64>,
        admin_phone_no: impl Into<String>,
        certify_code: impl Into<String>,
    ) -> Self {
        Self {
            qualification_group_id: qualification_group_id.into(),
            order_id: order_id.into(),
            business_license_pics: None,
            company_name: None,
            bussiness_license_exp_date: None,
            legal_person_id_card_front_side: None,
            legal_person_id_card_back_side: None,
            legal_person_name: None,
            legal_person_id_card_no: None,
            legal_person_id_card_eff_time: None,
            legal_person_id_card_type: None,
            admin_id_card_pic: None,
            admin_id_card_front_face: None,
            admin_name: None,
            admin_id_card_no: None,
            admin_id_card_exp_date: None,
            admin_phone_no: admin_phone_no.into(),
            certify_code: certify_code.into(),
            admin_id_card_type: None,
            other_files: None,
        }
    }
}
impl crate::ToFormData for UpdateSmsQualification {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for UpdateSmsQualification {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UpdateSmsQualification";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<UpdateSmsQualificationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(20);

        if let Some(f) = &self.admin_id_card_exp_date {
            params.push(("AdminIDCardExpDate".into(), (f).into()));
        }

        if let Some(f) = &self.admin_id_card_front_face {
            params.push(("AdminIDCardFrontFace".into(), (f).into()));
        }

        if let Some(f) = &self.admin_id_card_no {
            params.push(("AdminIDCardNo".into(), (f).into()));
        }

        if let Some(f) = &self.admin_id_card_pic {
            params.push(("AdminIDCardPic".into(), (f).into()));
        }

        if let Some(f) = &self.admin_id_card_type {
            params.push(("AdminIDCardType".into(), (f).into()));
        }

        if let Some(f) = &self.admin_name {
            params.push(("AdminName".into(), (f).into()));
        }
        params.push(("AdminPhoneNo".into(), (&self.admin_phone_no).into()));

        if let Some(f) = &self.business_license_pics {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("BusinessLicensePics".into(), json.into()));
            }
        }

        if let Some(f) = &self.bussiness_license_exp_date {
            params.push(("BussinessLicenseExpDate".into(), (f).into()));
        }
        params.push(("CertifyCode".into(), (&self.certify_code).into()));

        if let Some(f) = &self.company_name {
            params.push(("CompanyName".into(), (f).into()));
        }

        if let Some(f) = &self.legal_person_id_card_no {
            params.push(("LegalPersonIDCardNo".into(), (f).into()));
        }

        if let Some(f) = &self.legal_person_id_card_type {
            params.push(("LegalPersonIDCardType".into(), (f).into()));
        }

        if let Some(f) = &self.legal_person_id_card_back_side {
            params.push(("LegalPersonIdCardBackSide".into(), (f).into()));
        }

        if let Some(f) = &self.legal_person_id_card_eff_time {
            params.push(("LegalPersonIdCardEffTime".into(), (f).into()));
        }

        if let Some(f) = &self.legal_person_id_card_front_side {
            params.push(("LegalPersonIdCardFrontSide".into(), (f).into()));
        }

        if let Some(f) = &self.legal_person_name {
            params.push(("LegalPersonName".into(), (f).into()));
        }
        params.push(("OrderId".into(), (&self.order_id).into()));

        if let Some(f) = &self.other_files {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("OtherFiles".into(), json.into()));
            }
        }
        params.push((
            "QualificationGroupId".into(),
            (&self.qualification_group_id).into(),
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
pub struct DeleteSmsQualification {
    /// 资质ID，即您[申请资质](~~SubmitSmsQualification~~)返回的ID。您可以通过调用[查询资质列表](~~QuerySmsQualificationRecord~~)接口，或通过控制台国内消息[资质管理](https://dysms.console.aliyun.com/domestic/text/qualification)页面获取当前账号下的资质ID。
    qualification_group_id: i64,
    /// 审核工单ID。您可以通过调用[查询资质列表](~~QuerySmsQualificationRecord~~)接口，或通过控制台国内消息[资质管理](https://dysms.console.aliyun.com/domestic/text/qualification)页面获取当前账号下的资质及其对应审核工单ID。
    order_id: i64,
}

impl sealed::Bound for DeleteSmsQualification {}

impl DeleteSmsQualification {
    pub fn new(qualification_group_id: impl Into<i64>, order_id: impl Into<i64>) -> Self {
        Self {
            qualification_group_id: qualification_group_id.into(),
            order_id: order_id.into(),
        }
    }
}
impl crate::ToFormData for DeleteSmsQualification {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteSmsQualification {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DeleteSmsQualification";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<DeleteSmsQualificationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("OrderId".into(), (&self.order_id).into()));
        params.push((
            "QualificationGroupId".into(),
            (&self.qualification_group_id).into(),
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
pub struct RequiredPhoneCode {
    /// 接收验证码的管理员手机号。
    /// - 调用本接口**发送验证码的账号和提交资质的账号必须是同一个**，否则调用失败。
    /// - 号码格式：+/+86/0086/86 或无任何前缀的手机号码，例如 1390000****。
    phone_no: String,
}

impl sealed::Bound for RequiredPhoneCode {}

impl RequiredPhoneCode {
    pub fn new(phone_no: impl Into<String>) -> Self {
        Self {
            phone_no: phone_no.into(),
        }
    }
}
impl crate::ToFormData for RequiredPhoneCode {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for RequiredPhoneCode {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RequiredPhoneCode";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<RequiredPhoneCodeResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("PhoneNo".into(), (&self.phone_no).into()));

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
pub struct ValidPhoneCode {
    /// 手机号。
    phone_no: String,
    /// 验证码。
    certify_code: String,
}

impl sealed::Bound for ValidPhoneCode {}

impl ValidPhoneCode {
    pub fn new(phone_no: impl Into<String>, certify_code: impl Into<String>) -> Self {
        Self {
            phone_no: phone_no.into(),
            certify_code: certify_code.into(),
        }
    }
}
impl crate::ToFormData for ValidPhoneCode {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for ValidPhoneCode {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "ValidPhoneCode";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<ValidPhoneCodeResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("CertifyCode".into(), (&self.certify_code).into()));
        params.push(("PhoneNo".into(), (&self.phone_no).into()));

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
pub struct CreateSmsAuthorizationLetter {
    /// 委托授权书的fileKey。
    ///
    /// 1. 上传到OSS的授权委托书文件信息。请下载[授权委托书模板](https://help-static-aliyun-doc.aliyuncs.com/file-manage-files/zh-CN/20250414/bvpcmo/%E6%8E%88%E6%9D%83%E5%A7%94%E6%89%98%E4%B9%A6%E6%A8%A1%E7%89%88.doc)后，根据[规范](~~56741~~)完成填写并盖章后上传。文件上传要求：
    /// - 待上传的文件命名不可包含中文和特殊字符。
    /// - 仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。
    ///
    /// 2. fileKey的获取方式如下：[通过OSS上传文件](~~2833114~~)。
    authorization_letter_pic: String,
    /// 委托授权签名列表，签名数量不超过100个。
    ///
    /// > 建议您在授权书内将可能需要用到的签名一次性全部授权，避免后续申请签名时不在授权书签名范围内，导致审核不通过且需重新补充委托授权书。
    sign_list: Vec<String>,
    /// 授权委托书有效期。有效期格式：`YYYY-MM-DD~YYYY-MM-DD`。
    ///
    /// > 有效期限建议为1~3年。请设定一个合理的时间周期，避免有效期过长或过短。
    authorization_letter_exp_date: String,
    /// 委托授权方，即签名归属方。符号仅支持中点`·`、中文`【】（）`、英文`()`及空格，不可含其他符号或纯数字，长度不超过150字符。
    authorization: String,
    /// 委托授权方社会统一信用代码，长度不超过150字符。信用代码必须与签名绑定的资质信息中社会统一信用代码字段保持一致，否则创建签名失败。
    organization_code: String,
    /// 被委托授权方，即签名申请方。符号仅支持中点`·`、中文`【】（）`、英文`()`及空格，不可含其他符号或纯数字，长度不超过150字符。
    proxy_authorization: String,
    /// 授权委托书命名。命名不可与您其他授权书重复，仅支持中文、英文或与数字组合，不可含符号或纯数字，长度不超过100字符。
    authorization_letter_name: String,
}

impl sealed::Bound for CreateSmsAuthorizationLetter {}

impl CreateSmsAuthorizationLetter {
    pub fn new(
        authorization_letter_pic: impl Into<String>,
        sign_list: impl Into<Vec<String>>,
        authorization_letter_exp_date: impl Into<String>,
        authorization: impl Into<String>,
        organization_code: impl Into<String>,
        proxy_authorization: impl Into<String>,
        authorization_letter_name: impl Into<String>,
    ) -> Self {
        Self {
            authorization_letter_pic: authorization_letter_pic.into(),
            sign_list: sign_list.into(),
            authorization_letter_exp_date: authorization_letter_exp_date.into(),
            authorization: authorization.into(),
            organization_code: organization_code.into(),
            proxy_authorization: proxy_authorization.into(),
            authorization_letter_name: authorization_letter_name.into(),
        }
    }
}
impl crate::ToFormData for CreateSmsAuthorizationLetter {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CreateSmsAuthorizationLetter {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateSmsAuthorizationLetter";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<CreateSmsAuthorizationLetterResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(7);
        params.push(("Authorization".into(), (&self.authorization).into()));
        params.push((
            "AuthorizationLetterExpDate".into(),
            (&self.authorization_letter_exp_date).into(),
        ));
        params.push((
            "AuthorizationLetterName".into(),
            (&self.authorization_letter_name).into(),
        ));
        params.push((
            "AuthorizationLetterPic".into(),
            (&self.authorization_letter_pic).into(),
        ));
        params.push(("OrganizationCode".into(), (&self.organization_code).into()));
        params.push((
            "ProxyAuthorization".into(),
            (&self.proxy_authorization).into(),
        ));
        if let Ok(json) = serde_json::to_string(&self.sign_list) {
            params.push(("SignList".into(), json.into()));
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
pub struct QuerySmsAuthorizationLetter {
    /// 委托授权书ID列表。
    #[setters(generate = true, strip_option)]
    authorization_letter_id_list: Option<Vec<i64>>,
    /// 签名名称。若您创建授权书时，授权范围包括多个签名，则会返回包含该签名的授权书。
    #[setters(generate = true, strip_option)]
    sign_name: Option<String>,
    /// 委托授权方社会统一信用代码，长度不超过150个字符。
    #[setters(generate = true, strip_option)]
    organization_code: Option<String>,
    /// 委托授权书审核状态，与签名审核状态相关，取值：
    /// - **INT**：待审核。委托授权书已创建，当您提交签名申请后进入审核流程。
    /// - **PASSED**：审核通过。当您的委托授权签名范围中有签名审核通过时，委托授权书状态变为PASSED。
    #[setters(generate = true, strip_option)]
    state: Option<String>,
    /// 委托授权书可用状态，与授权书有效期相关，取值：
    ///
    /// - **VALID**：可用，授权书处于有效期内。
    /// - **INVALID**：不可用，授权书已过期。
    #[setters(generate = true, strip_option)]
    status: Option<String>,
}

impl sealed::Bound for QuerySmsAuthorizationLetter {}

impl QuerySmsAuthorizationLetter {
    pub fn new() -> Self {
        Self {
            authorization_letter_id_list: None,
            sign_name: None,
            organization_code: None,
            state: None,
            status: None,
        }
    }
}
impl crate::ToFormData for QuerySmsAuthorizationLetter {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySmsAuthorizationLetter {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySmsAuthorizationLetter";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySmsAuthorizationLetterResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);

        if let Some(f) = &self.authorization_letter_id_list {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("AuthorizationLetterIdList".into(), json.into()));
            }
        }

        if let Some(f) = &self.organization_code {
            params.push(("OrganizationCode".into(), (f).into()));
        }

        if let Some(f) = &self.sign_name {
            params.push(("SignName".into(), (f).into()));
        }

        if let Some(f) = &self.state {
            params.push(("State".into(), (f).into()));
        }

        if let Some(f) = &self.status {
            params.push(("Status".into(), (f).into()));
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
pub struct CreateSmsSign {
    /// 签名名称，签名名称请遵守[签名规范](~~108076#section-0p8-qn8-mmy~~)：
    ///
    /// - 长度限2~12字符，不支持包含“测试”、“test”等字样。
    ///
    /// - 不支持添加【】、()、[]等符号；不支持、，。空格等特殊字符。
    ///
    /// > - 签名名称区分大小写字母，如【Aliyun通信】和【aliyun通信】视为两个不同的签名。
    /// > - 当您的验证码签名和通用签名名称相同时，系统默认使用通用签名发送短信。
    sign_name: String,
    /// 短信签名场景说明，是签名审核的参考信息之一，长度不超过200个字符。
    /// >  - 您可以提供已上线业务的使用场景，并提供实际业务的网站链接、应用市场下载链接等。
    /// >  - 您可以提供短信完整示例，体现您的业务场景。
    /// >  - 您可以提供变量的传参内容，详细描述业务使用场景和选择这个变量属性的原因。
    /// >  - 若签名涉及政企机关单位，请备注政企机关单位的固定电话。
    ///
    /// 信息完善的申请说明会提高签名、模板的审核效率。如未按规范进行填写或不填写，可能会影响您签名审核的通过。
    #[setters(generate = true, strip_option)]
    remark: Option<String>,
    /// 签名类型。取值：
    ///
    /// - **0**：验证码。
    ///
    /// - **1**：通用（默认值）。
    ///
    /// 建议使用默认值：**通用**。
    #[setters(generate = true, strip_option)]
    sign_type: Option<i32>,
    /// 更多资料。补充上传业务证明文件或业务截图，有助于审核人员了解您的业务详情。请参见[签名申请材料](~~108076#section-xup-k46-yi4~~)，上传相关材料。
    #[setters(generate = true, strip_option)]
    more_data: Option<Vec<String>>,
    /// 已审核通过的资质ID。
    ///
    /// > - 在申请短信签名前，请先[申请资质](~~2539801~~)。
    /// > - 您可在[资质管理](https://dysms.console.aliyun.com/domestic/text/qualification)页面查看资质ID。
    qualification_id: i64,
    /// App应用商店链接。若签名来源为“已上线App”，即`SignSource`取值为2时，请填写http://或https://开头的应用商店链接，并确保App已经上线。
    #[setters(generate = true, strip_option)]
    apply_scene_content: Option<String>,
    /// 签名来源。取值：
    ///
    /// -  **0**：企事业单位的全称或简称。
    /// -  **2**：App应用的全称或简称。
    /// -  **5**：商标名的全称或简称。
    ///
    /// 签名来源的详细说明请参见[签名来源](~~108076#section-fow-bfu-wo9~~)。
    sign_source: i32,
    /// 签名用途。取值：
    ///
    /// - false：自用（默认值，签名为本账号实名认证的企业、网站、产品名等）。
    ///
    /// - true：他用（签名为非本账号实名认证的企业、网站、产品名等）。
    /// ><notice>签名为自用时，请选择自用资质ID；签名为他用时，请选择他用资质ID。></notice>
    #[setters(generate = true, strip_option)]
    third_party: Option<bool>,
    /// 委托授权书ID，当签名为他用时，委托授权书ID不可为空，否则签名审核不通过。委托授权书的社会统一信用代码必须与签名绑定的资质信息中社会统一信用代码字段保持一致，否则创建签名失败。
    #[setters(generate = true, strip_option)]
    authorization_letter_id: Option<i64>,
    /// 商标实体id。
    /// > - 当SignSource=5时，需要传商标实体id。
    /// > - 商标实体id可以通过调用[创建商标实体](~~CreateSmsTrademark~~)接口获取。
    #[setters(generate = true, strip_option)]
    trademark_id: Option<i64>,
    /// APP-ICP备案实体id。
    /// > - 当SignSource=2时，需要传备案实体id。
    /// > - 备案实体id可以通过调用[创建ICP备案实体](~~CreateSmsAppIcpRecord~~)接口获取。
    #[setters(generate = true, strip_option)]
    app_icp_record_id: Option<i64>,
}

impl sealed::Bound for CreateSmsSign {}

impl CreateSmsSign {
    pub fn new(
        sign_name: impl Into<String>,
        qualification_id: impl Into<i64>,
        sign_source: impl Into<i32>,
    ) -> Self {
        Self {
            sign_name: sign_name.into(),
            remark: None,
            sign_type: None,
            more_data: None,
            qualification_id: qualification_id.into(),
            apply_scene_content: None,
            sign_source: sign_source.into(),
            third_party: None,
            authorization_letter_id: None,
            trademark_id: None,
            app_icp_record_id: None,
        }
    }
}
impl crate::ToFormData for CreateSmsSign {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CreateSmsSign {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateSmsSign";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<CreateSmsSignResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(11);

        if let Some(f) = &self.app_icp_record_id {
            params.push(("AppIcpRecordId".into(), (f).into()));
        }

        if let Some(f) = &self.apply_scene_content {
            params.push(("ApplySceneContent".into(), (f).into()));
        }

        if let Some(f) = &self.authorization_letter_id {
            params.push(("AuthorizationLetterId".into(), (f).into()));
        }

        if let Some(f) = &self.more_data {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("MoreData".into(), json.into()));
            }
        }
        params.push(("QualificationId".into(), (&self.qualification_id).into()));

        if let Some(f) = &self.remark {
            params.push(("Remark".into(), (f).into()));
        }
        params.push(("SignName".into(), (&self.sign_name).into()));
        params.push(("SignSource".into(), (&self.sign_source).into()));

        if let Some(f) = &self.sign_type {
            params.push(("SignType".into(), (f).into()));
        }

        if let Some(f) = &self.third_party {
            params.push(("ThirdParty".into(), (f).into()));
        }

        if let Some(f) = &self.trademark_id {
            params.push(("TrademarkId".into(), (f).into()));
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
pub struct GetSmsSign {
    /// 签名名称。必须是本账号已申请的短信签名。
    ///
    /// - 调用[CreateSmsSign](~~2807427~~)接口后在返回参数中获取。
    /// - 在[签名管理](https://dysms.console.aliyun.com/domestic/text/sign)页面查看签名。
    sign_name: String,
}

impl sealed::Bound for GetSmsSign {}

impl GetSmsSign {
    pub fn new(sign_name: impl Into<String>) -> Self {
        Self {
            sign_name: sign_name.into(),
        }
    }
}
impl crate::ToFormData for GetSmsSign {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for GetSmsSign {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "GetSmsSign";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<GetSmsSignResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("SignName".into(), (&self.sign_name).into()));

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
pub struct QuerySmsSignList {
    /// 当前页码，默认取值为**1**。
    #[setters(generate = true, strip_option)]
    page_index: Option<i32>,
    /// 每页显示的签名个数。默认取值为**10**，取值范围：**1~50**。
    #[setters(generate = true, strip_option)]
    page_size: Option<i32>,
}

impl sealed::Bound for QuerySmsSignList {}

impl QuerySmsSignList {
    pub fn new() -> Self {
        Self {
            page_index: None,
            page_size: None,
        }
    }
}
impl crate::ToFormData for QuerySmsSignList {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySmsSignList {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySmsSignList";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySmsSignListResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

        if let Some(f) = &self.page_index {
            params.push(("PageIndex".into(), (f).into()));
        }

        if let Some(f) = &self.page_size {
            params.push(("PageSize".into(), (f).into()));
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
pub struct UpdateSmsSign {
    /// 未审核通过的短信签名。您可在控制台[国内消息>签名管理](https://dysms.console.aliyun.com/domestic/text/sign)页签进行查看未通过审核得短信签名。也可通过[QuerySmsSignList](~~QuerySmsSignList~~)接口查看未通过审核的短信签名。
    sign_name: String,
    /// 短信签名场景说明，是签名审核的参考信息之一，长度不超过200个字符。
    /// >  - 您可以提供已上线业务的使用场景，并提供实际业务的网站链接、应用市场下载链接等。
    /// >  - 您可以提供短信完整示例，体现您的业务场景。
    /// >  - 您可以提供变量的传参内容，详细描述业务使用场景和选择这个变量属性的原因。
    /// >  - 若签名涉及政企机关单位，请备注政企机关单位的固定电话。
    ///
    /// 信息完善的申请说明会提高签名、模板的审核效率。如未按规范进行填写或不填写，可能会影响您签名审核的通过。
    #[setters(generate = true, strip_option)]
    remark: Option<String>,
    /// 签名类型。取值：
    ///
    /// - **0**：验证码。
    /// - **1**：通用（默认值）。
    ///
    /// 建议使用默认值：**通用**。
    #[setters(generate = true, strip_option)]
    sign_type: Option<i32>,
    /// 更多资料。补充上传业务证明文件或业务截图，有助于审核人员了解您的业务详情。请参见[签名申请材料](~~108076#section-xup-k46-yi4~~)，上传相关材料。
    #[setters(generate = true, strip_option)]
    more_data: Option<Vec<String>>,
    /// 已审核通过的资质ID。
    ///
    /// > - 在申请短信签名前，请先[申请资质](https://help.aliyun.com/zh/sms/user-guide/new-qualification?spm=a2c4g.11186623.0.0.718d187bbkpMRK)。
    /// > - 您可在[资质管理](https://dysms.console.aliyun.com/domestic/text/qualification)页面查看资质ID。
    qualification_id: i64,
    /// App应用商店链接，若签名来源为“已上线App”，即`SignSource`取值为2时，请填写http://或https://开头的应用商店链接，并确保App已经上线。
    #[setters(generate = true, strip_option)]
    apply_scene_content: Option<String>,
    /// 签名来源。取值：
    ///
    /// -  **0**：企事业单位的全称或简称。
    /// -  **2**：App应用的全称或简称。
    /// -  **5**：商标名的全称或简称。
    ///
    /// 签名来源的详细说明请参见[签名来源](~~108076#section-fow-bfu-wo9~~)。
    sign_source: i32,
    /// 签名用途。取值：
    ///
    /// - false：自用（默认值，签名为本账号实名认证的企业、网站、产品名等）。
    ///
    /// - true：他用（签名为非本账号实名认证的企业、网站、产品名等）。
    /// ><notice>签名为自用时，请选择自用资质ID；签名为他用时，请选择他用资质ID。></notice>
    #[setters(generate = true, strip_option)]
    third_party: Option<bool>,
    /// 委托授权书ID，当签名为他用时，委托授权书ID不可为空，否则签名审核不通过。委托授权书的社会统一信用代码必须与签名绑定的资质信息中社会统一信用代码字段保持一致，否则创建签名失败。
    #[setters(generate = true, strip_option)]
    authorization_letter_id: Option<i64>,
    /// 商标实体id。
    /// > - 当SignSource=5时，需要传商标实体id。
    /// > - 商标实体id可以通过调用[创建商标实体](~~CreateSmsTrademark~~)接口获取。
    #[setters(generate = true, strip_option)]
    trademark_id: Option<i64>,
    /// APP-ICP备案实体id。
    /// > - 当SignSource=2时，需要传备案实体id。
    /// > - 备案实体id可以通过调用[创建ICP备案实体](~~CreateSmsAppIcpRecord~~)接口获取。
    #[setters(generate = true, strip_option)]
    app_icp_record_id: Option<i64>,
}

impl sealed::Bound for UpdateSmsSign {}

impl UpdateSmsSign {
    pub fn new(
        sign_name: impl Into<String>,
        qualification_id: impl Into<i64>,
        sign_source: impl Into<i32>,
    ) -> Self {
        Self {
            sign_name: sign_name.into(),
            remark: None,
            sign_type: None,
            more_data: None,
            qualification_id: qualification_id.into(),
            apply_scene_content: None,
            sign_source: sign_source.into(),
            third_party: None,
            authorization_letter_id: None,
            trademark_id: None,
            app_icp_record_id: None,
        }
    }
}
impl crate::ToFormData for UpdateSmsSign {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for UpdateSmsSign {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UpdateSmsSign";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<UpdateSmsSignResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(11);

        if let Some(f) = &self.app_icp_record_id {
            params.push(("AppIcpRecordId".into(), (f).into()));
        }

        if let Some(f) = &self.apply_scene_content {
            params.push(("ApplySceneContent".into(), (f).into()));
        }

        if let Some(f) = &self.authorization_letter_id {
            params.push(("AuthorizationLetterId".into(), (f).into()));
        }

        if let Some(f) = &self.more_data {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("MoreData".into(), json.into()));
            }
        }
        params.push(("QualificationId".into(), (&self.qualification_id).into()));

        if let Some(f) = &self.remark {
            params.push(("Remark".into(), (f).into()));
        }
        params.push(("SignName".into(), (&self.sign_name).into()));
        params.push(("SignSource".into(), (&self.sign_source).into()));

        if let Some(f) = &self.sign_type {
            params.push(("SignType".into(), (f).into()));
        }

        if let Some(f) = &self.third_party {
            params.push(("ThirdParty".into(), (f).into()));
        }

        if let Some(f) = &self.trademark_id {
            params.push(("TrademarkId".into(), (f).into()));
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

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteSmsSign {
    /// 短信签名。支持删除已撤回、审核失败或审核通过的签名，审核中的短信签名不支持删除。
    ///
    /// 您可以通过[QuerySmsSignList](~~419282~~)接口查询当前账号已申请的签名或在短信控制台[签名管理](https://dysms.console.aliyun.com/domestic/text/sign)页查看签名列表
    /// ><notice>删除短信签名后不可恢复，且后续不可再使用该签名发送短信，请谨慎操作。></notice>
    sign_name: String,
}

impl sealed::Bound for DeleteSmsSign {}

impl DeleteSmsSign {
    pub fn new(sign_name: impl Into<String>) -> Self {
        Self {
            sign_name: sign_name.into(),
        }
    }
}
impl crate::ToFormData for DeleteSmsSign {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteSmsSign {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DeleteSmsSign";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<DeleteSmsSignResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("SignName".into(), (&self.sign_name).into()));

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
pub struct ChangeSignatureQualification {
    /// 短信签名。
    signature_name: String,
    /// 资质ID，即您申请资质返回的ID。您可以通过控制台国内消息[资质管理](https://dysms.console.aliyun.com/domestic/text/qualification)页面获取当前账号下的资质ID。
    qualification_id: i64,
    /// 授权委托书ID。
    #[setters(generate = true, strip_option)]
    authorization_letter_id: Option<i64>,
}

impl sealed::Bound for ChangeSignatureQualification {}

impl ChangeSignatureQualification {
    pub fn new(signature_name: impl Into<String>, qualification_id: impl Into<i64>) -> Self {
        Self {
            signature_name: signature_name.into(),
            qualification_id: qualification_id.into(),
            authorization_letter_id: None,
        }
    }
}
impl crate::ToFormData for ChangeSignatureQualification {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for ChangeSignatureQualification {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "ChangeSignatureQualification";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<ChangeSignatureQualificationResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.authorization_letter_id {
            params.push(("AuthorizationLetterId".into(), (f).into()));
        }
        params.push(("QualificationId".into(), (&self.qualification_id).into()));
        params.push(("SignatureName".into(), (&self.signature_name).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AddSmsSign {
    /// 签名名称，签名名称请遵守[签名规范](~~108076#section-0p8-qn8-mmy~~)。
    ///
    /// > - 签名名称不区分大小写字母，如【Aliyun通信】和【aliyun通信】视为名称相同。
    /// > - 当您的验证码签名和通用签名名称相同时，系统默认使用通用签名发送短信。
    sign_name: String,
    /// 签名来源。取值：
    ///
    /// -  **0**：企事业单位的全称或简称。
    /// -  **1**：工信部备案网站的全称或简称。
    /// -  **2**：App应用的全称或简称。
    /// -  **3**：公众号或小程序的全称或简称。
    /// -  **4**：电商平台店铺名的全称或简称。
    /// -  **5**：商标名的全称或简称。
    ///
    /// 签名来源的详细说明请参见[签名来源](https://help.aliyun.com/zh/sms/user-guide/signature-specifications-1?spm=a2c4g.11186623.0.0.4f9710fder2gR7#section-xup-k46-yi4)。
    ///
    /// >此接口不支持申请签名来源是**测试或学习**和**线上试用**的签名，如果您需要申请这两种签名来源的签名，请前往[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/sign/add/qualification)申请。
    sign_source: i32,
    /// 短信签名场景说明，长度不超过200个字符。
    ///
    /// 签名审核的参考信息，完善申请说明有助于审核人员理解您的业务场景，提高审核效率。填写指导：
    ///
    /// - 您可以提供已上线业务的使用场景。
    ///
    /// - 您可以提供真实场景的短信示例，体现您的业务场景。
    ///
    /// - 您可以提供变量的传参内容，详细描述业务使用场景和选择这个变量属性的原因。
    ///
    /// - 您可以提供实际业务的网站链接、已备案的域名地址、应用市场下载链接等。
    ///
    /// - 登录场景，您可以提供测试账号和密码。
    remark: String,
    /// 签名文件列表。
    sign_file_list: Vec<AddSmsSignSignFileList>,
    /// 签名类型。
    ///
    /// - **0**：验证码
    /// - **1**：通用
    #[setters(generate = true, strip_option)]
    sign_type: Option<i32>,
}

impl sealed::Bound for AddSmsSign {}

impl AddSmsSign {
    pub fn new(
        sign_name: impl Into<String>,
        sign_source: impl Into<i32>,
        remark: impl Into<String>,
        sign_file_list: impl Into<Vec<AddSmsSignSignFileList>>,
    ) -> Self {
        Self {
            sign_name: sign_name.into(),
            sign_source: sign_source.into(),
            remark: remark.into(),
            sign_file_list: sign_file_list.into(),
            sign_type: None,
        }
    }
}
impl crate::ToFormData for AddSmsSign {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        crate::FlatSerialize::flat_serialize(&self.sign_file_list, "SignFileList", &mut params);

        params
    }
}

impl crate::Request for AddSmsSign {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "AddSmsSign";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<AddSmsSignResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(4);
        params.push(("Remark".into(), (&self.remark).into()));
        params.push(("SignName".into(), (&self.sign_name).into()));
        params.push(("SignSource".into(), (&self.sign_source).into()));

        if let Some(f) = &self.sign_type {
            params.push(("SignType".into(), (f).into()));
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

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ModifySmsSign {
    /// 签名名称。
    ///
    /// > 审核通过的签名支持修改，但不支持修改签名名称，修改后的签名需要重新通过审核后才能使用。签名审核未完成前，原签名也不能正常使用。
    sign_name: String,
    /// 签名来源。取值：
    ///
    /// - **0**：企事业单位的全称或简称。
    /// - **1**：工信部备案网站的全称或简称。
    /// - **2**：App应用的全称或简称。
    /// - **3**：公众号或小程序的全称或简称。
    /// - **4**：电商平台店铺名的全称或简称。
    /// - **5**：商标名的全称或简称。
    sign_source: i32,
    /// 短信签名申请说明，长度不超过200个字符。
    ///
    /// 签名审核的参考信息，完善申请说明有助于审核人员理解您的业务场景，提高审核效率。填写指导：
    ///
    /// - 您可以提供已上线业务的使用场景。
    /// - 您可以提供真实场景的短信示例，体现您的业务场景。
    /// - 您可以提供变量的传参内容，详细描述业务使用场景和选择这个变量属性的原因。
    /// - 您可以提供实际业务的网站链接、已备案的域名地址、应用市场下载链接等。
    /// - 登录场景，您可以提供测试账号和密码。
    remark: String,
    /// 签名文件列表。
    sign_file_list: Vec<ModifySmsSignSignFileList>,
    /// 签名类型。取值：
    ///
    /// - **0**：验证码
    ///
    /// - **1**：通用
    #[setters(generate = true, strip_option)]
    sign_type: Option<i32>,
}

impl sealed::Bound for ModifySmsSign {}

impl ModifySmsSign {
    pub fn new(
        sign_name: impl Into<String>,
        sign_source: impl Into<i32>,
        remark: impl Into<String>,
        sign_file_list: impl Into<Vec<ModifySmsSignSignFileList>>,
    ) -> Self {
        Self {
            sign_name: sign_name.into(),
            sign_source: sign_source.into(),
            remark: remark.into(),
            sign_file_list: sign_file_list.into(),
            sign_type: None,
        }
    }
}
impl crate::ToFormData for ModifySmsSign {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        crate::FlatSerialize::flat_serialize(&self.sign_file_list, "SignFileList", &mut params);

        params
    }
}

impl crate::Request for ModifySmsSign {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "ModifySmsSign";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<ModifySmsSignResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(4);
        params.push(("Remark".into(), (&self.remark).into()));
        params.push(("SignName".into(), (&self.sign_name).into()));
        params.push(("SignSource".into(), (&self.sign_source).into()));

        if let Some(f) = &self.sign_type {
            params.push(("SignType".into(), (f).into()));
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

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySmsSign {
    /// 签名名称。必须是本账号已申请的短信签名。
    sign_name: String,
}

impl sealed::Bound for QuerySmsSign {}

impl QuerySmsSign {
    pub fn new(sign_name: impl Into<String>) -> Self {
        Self {
            sign_name: sign_name.into(),
        }
    }
}
impl crate::ToFormData for QuerySmsSign {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySmsSign {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySmsSign";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySmsSignResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("SignName".into(), (&self.sign_name).into()));

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
pub struct CreateSmsTrademark {
    /// 商标详情截图fileKey。
    ///
    /// 1. 商标查询方法：
    /// - 登录中国商标网点击“商标网上查询”商标详情截图。
    /// - 接受使用说明，填入“申请/注册号”查询。
    /// - 点击“申请/注册号”查询详情。
    ///
    /// 2. 上传到OSS的商标文件信息。文件上传要求：
    /// - 待上传的文件命名不可包含中文和特殊字符。
    /// - 仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。
    /// - 截图中包含完整的网址;
    /// - 商标图片清晰，和“签名名称”完全一致；
    /// - “申请人名称”需和签名关联的企事业单位名称完全一致；
    /// - 商标状态为注册商标。
    ///
    /// 3. fileKey的获取方式如下：[通过OSS上传文件](~~2833114~~)。
    trademark_pic: String,
    /// 商标注册号，长度在15个字符内
    trademark_registration_number: String,
    /// 商标名称，长度在15个字符内
    trademark_name: String,
    /// 申请人名称，长度在50个字符内
    trademark_applicant_name: String,
    /// 专用权生失效日期。
    trademark_eff_exp_date: String,
}

impl sealed::Bound for CreateSmsTrademark {}

impl CreateSmsTrademark {
    pub fn new(
        trademark_pic: impl Into<String>,
        trademark_registration_number: impl Into<String>,
        trademark_name: impl Into<String>,
        trademark_applicant_name: impl Into<String>,
        trademark_eff_exp_date: impl Into<String>,
    ) -> Self {
        Self {
            trademark_pic: trademark_pic.into(),
            trademark_registration_number: trademark_registration_number.into(),
            trademark_name: trademark_name.into(),
            trademark_applicant_name: trademark_applicant_name.into(),
            trademark_eff_exp_date: trademark_eff_exp_date.into(),
        }
    }
}
impl crate::ToFormData for CreateSmsTrademark {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CreateSmsTrademark {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateSmsTrademark";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<CreateSmsTrademarkResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);
        params.push((
            "TrademarkApplicantName".into(),
            (&self.trademark_applicant_name).into(),
        ));
        params.push((
            "TrademarkEffExpDate".into(),
            (&self.trademark_eff_exp_date).into(),
        ));
        params.push(("TrademarkName".into(), (&self.trademark_name).into()));
        params.push(("TrademarkPic".into(), (&self.trademark_pic).into()));
        params.push((
            "TrademarkRegistrationNumber".into(),
            (&self.trademark_registration_number).into(),
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
pub struct QuerySmsTrademark {
    /// 商标实体id列表
    trademark_id_list: Vec<i64>,
}

impl sealed::Bound for QuerySmsTrademark {}

impl QuerySmsTrademark {
    pub fn new(trademark_id_list: impl Into<Vec<i64>>) -> Self {
        Self {
            trademark_id_list: trademark_id_list.into(),
        }
    }
}
impl crate::ToFormData for QuerySmsTrademark {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySmsTrademark {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySmsTrademark";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySmsTrademarkResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        if let Ok(json) = serde_json::to_string(&self.trademark_id_list) {
            params.push(("TrademarkIdList".into(), json.into()));
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
pub struct CreateSmsAppIcpRecord {
    /// APP-ICP备案详情截图fileKey。
    ///
    /// 1. ICP备案查询方法：
    /// 前往 [工信部服务平台](https://beian.miit.gov.cn/#/Integrated/recordQuery) 查询ICP备案 ，并进入详情页。
    ///
    /// 2. 上传到OSS的ICP 备案文件信息。文件上传要求：
    ///
    /// - 待上传的文件命名不可包含中文和特殊字符。
    /// - 仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。
    /// - 截图中包含完整的网址。
    /// - 搜索备案类型选择“APP”。
    /// - “主办单位名称”需和签名关联资质的企事业单位名称完全一致。
    /// - 有完整清晰的ICP备案/许可证号。
    /// - “服务名称”需和“签名名称”完全一致。
    ///
    /// 3. fileKey的获取方式如下：[通过OSS上传文件](~~2833114~~)。
    app_icp_record_pic: String,
    /// ICP备案：服务名称，长度在15个字符内
    app_service_name: String,
    /// ICP备案：主办单位名称，长度在50个字符内
    app_principal_unit_name: String,
    /// ICP备案/许可证号，长度在15个字符内
    app_icp_license_number: String,
    /// ICP备案审核通过日期
    app_approval_date: String,
    /// APP应用商店链接。
    ///
    /// >
    /// > - 必须以http://或https://开头。
    domain: String,
}

impl sealed::Bound for CreateSmsAppIcpRecord {}

impl CreateSmsAppIcpRecord {
    pub fn new(
        app_icp_record_pic: impl Into<String>,
        app_service_name: impl Into<String>,
        app_principal_unit_name: impl Into<String>,
        app_icp_license_number: impl Into<String>,
        app_approval_date: impl Into<String>,
        domain: impl Into<String>,
    ) -> Self {
        Self {
            app_icp_record_pic: app_icp_record_pic.into(),
            app_service_name: app_service_name.into(),
            app_principal_unit_name: app_principal_unit_name.into(),
            app_icp_license_number: app_icp_license_number.into(),
            app_approval_date: app_approval_date.into(),
            domain: domain.into(),
        }
    }
}
impl crate::ToFormData for CreateSmsAppIcpRecord {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CreateSmsAppIcpRecord {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateSmsAppIcpRecord";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<CreateSmsAppIcpRecordResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(6);
        params.push(("AppApprovalDate".into(), (&self.app_approval_date).into()));
        params.push((
            "AppIcpLicenseNumber".into(),
            (&self.app_icp_license_number).into(),
        ));
        params.push(("AppIcpRecordPic".into(), (&self.app_icp_record_pic).into()));
        params.push((
            "AppPrincipalUnitName".into(),
            (&self.app_principal_unit_name).into(),
        ));
        params.push(("AppServiceName".into(), (&self.app_service_name).into()));
        params.push(("Domain".into(), (&self.domain).into()));

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
pub struct QuerySmsAppIcpRecord {
    /// ICP备案实体id列表
    app_icp_record_id_list: Vec<i64>,
}

impl sealed::Bound for QuerySmsAppIcpRecord {}

impl QuerySmsAppIcpRecord {
    pub fn new(app_icp_record_id_list: impl Into<Vec<i64>>) -> Self {
        Self {
            app_icp_record_id_list: app_icp_record_id_list.into(),
        }
    }
}
impl crate::ToFormData for QuerySmsAppIcpRecord {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySmsAppIcpRecord {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySmsAppIcpRecord";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySmsAppIcpRecordResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        if let Ok(json) = serde_json::to_string(&self.app_icp_record_id_list) {
            params.push(("AppIcpRecordIdList".into(), json.into()));
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
pub struct CreateSmsTemplate {
    /// 模板名称，长度不超过30个字符。
    template_name: String,
    /// 模板内容，长度不超过500个字符。
    ///
    /// 模板内容和变量内容必须符合[短信模板规范](~~463161~~)，否则模板会审核失败。您也可以在[申请模板](https://dysms.console.aliyun.com/domestic/text/template/add)页面，查看常用模板示例。使用示例模板可提高审核效率和成功率。变量规范请参见[TemplateContent参数变量规范](~~2806243~~)。
    template_content: String,
    /// 请描述您使用短信的业务场景或提供业务场景的线上链接，并提供短信完整示例（填入变量内容），信息完整有助于提高模板审核通过率。如未按规范进行填写或不填写，可能会影响您模板审核的通过。
    #[setters(generate = true, strip_option)]
    remark: Option<String>,
    /// 短信类型。取值：
    ///
    /// - **0**：验证码。
    /// - **1**：短信通知。
    /// - **2**：推广短信。
    /// - **3**：国际/港澳台消息。
    ///
    /// > 仅支持企业认证用户申请推广短信和国际/港澳台消息。个人用户与企业用户权益区别详情请参见[使用须知](https://help.aliyun.com/zh/sms/user-guide/usage-notes?spm=a2c4g.11186623.0.0.67447f576NJnE8)。
    template_type: i32,
    /// 模板需要关联的签名名称。关联的短信签名必须为审核通过的签名。
    ///
    /// <notice>
    ///
    /// - 当TemplateType参数为**0**、**1**、**2**时，此参数必填。
    ///
    /// - 关联签名可以提升审核效率，此处关联的签名和短信发送时选择的签名无关。</notice>
    #[setters(generate = true, strip_option)]
    related_sign_name: Option<String>,
    /// 模板变量规则。变量规则的填写，请参见[示例文档](~~2806243~~)。
    #[setters(generate = true, strip_option)]
    template_rule: Option<String>,
    /// 更多资料，您可以补充上传业务证明文件或业务截图，有助于审核人员了解您的业务详情。如果您申请的短信模板为推广短信（即TemplateType为2），请上传用户授权证明材料，具体说明请参见[用户授权证明材料上传规范](~~312341~~)。
    #[setters(generate = true, strip_option)]
    more_data: Option<Vec<String>>,
    /// 业务场景。
    ///  - 模板关联的短信签名使用场景如果为“已上线APP”，`ApplySceneContent`必须为以`http://`开头或`https://`开头的App链接。
    ///  - 模板关联的短信签名使用场景如果为“已注册商标名”、“企事业单位名称”，`ApplySceneContent`必填。
    #[setters(generate = true, strip_option)]
    apply_scene_content: Option<String>,
    /// 国际/港澳台模板类型。当**TemplateType**参数为**3**时，国际/港澳台模板需要设置此参数，取值：
    /// - **0**：短信通知。
    /// - **1**：推广短信。
    /// - **2**：验证码。
    #[setters(generate = true, strip_option)]
    intl_type: Option<i32>,
    /// ><warning>
    /// 为管控短信内容安全，短信内容中包含“号码、链接”等引流信息，存在被运营商拦截导致发送失败的风险。建议尽可能在短信模板中避免包含相关信息，以降低短信发送失败风险。></warning>
    ///
    /// 引流信息列表JSON字符串。
    /// ><notice>JSON格式，传入前请转为字符串。></notice>
    ///
    /// ### 1. 字段说明：
    ///
    /// {
    ///     "trafficDrivingType":"引流类型",
    ///     "trafficDrivingContent":"引流内容",
    ///     "variableName":"变量名称",
    ///     "companyName":企事业单位名称,
    ///     "organizationCode":统一社会信用代码,
    ///     "icpNo":ICP备案/许可证号,
    ///     "icpPicOssKey":ICP备案截图OssKey,
    ///     "companyDifferentFromSignQuaReason":企事业单位名称与签名资质不同原因
    /// }
    ///
    /// ### 2. 注意事项：
    ///
    /// - 如果非变量则variableName不传。
    ///
    /// - 企事业单位名称与签名关联资质的企事业单位名称不同，则传入companyDifferentFromSignQuaReason。
    ///
    /// - 如果trfficDrivingType=DOMAIN，则传入所有参数。
    ///
    /// - 如果trafficDrivingType为其他，则传入trafficDrivingType、trafficDrivingContent、variableName（按需）、companyName、organizationCode、companyDifferentFromSignQuaReason（按需）
    ///
    /// ### 3. trafficDrivingType引流类型枚举值：
    /// ><warning>因监管要求，不支持手机号码。></warning>
    /// - DOMAIN：域名类型链接。
    /// - FIXED_PHONE：固定电话。
    /// - 400_PHONE：400开头电话。
    /// - 800_PHONE：800开头电话。
    /// - 95_PHONE：95开头电话。
    /// - 96_PHONE：96开头电话。
    /// - 1_PHONE：1开头，3~5位电话。
    /// - OTHER_PHONE：其他号码。
    #[setters(generate = true, strip_option)]
    traffic_driving: Option<String>,
}

impl sealed::Bound for CreateSmsTemplate {}

impl CreateSmsTemplate {
    pub fn new(
        template_name: impl Into<String>,
        template_content: impl Into<String>,
        template_type: impl Into<i32>,
    ) -> Self {
        Self {
            template_name: template_name.into(),
            template_content: template_content.into(),
            remark: None,
            template_type: template_type.into(),
            related_sign_name: None,
            template_rule: None,
            more_data: None,
            apply_scene_content: None,
            intl_type: None,
            traffic_driving: None,
        }
    }
}
impl crate::ToFormData for CreateSmsTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CreateSmsTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateSmsTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<CreateSmsTemplateResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(10);

        if let Some(f) = &self.apply_scene_content {
            params.push(("ApplySceneContent".into(), (f).into()));
        }

        if let Some(f) = &self.intl_type {
            params.push(("IntlType".into(), (f).into()));
        }

        if let Some(f) = &self.more_data {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("MoreData".into(), json.into()));
            }
        }

        if let Some(f) = &self.related_sign_name {
            params.push(("RelatedSignName".into(), (f).into()));
        }

        if let Some(f) = &self.remark {
            params.push(("Remark".into(), (f).into()));
        }
        params.push(("TemplateContent".into(), (&self.template_content).into()));
        params.push(("TemplateName".into(), (&self.template_name).into()));

        if let Some(f) = &self.template_rule {
            params.push(("TemplateRule".into(), (f).into()));
        }
        params.push(("TemplateType".into(), (&self.template_type).into()));

        if let Some(f) = &self.traffic_driving {
            params.push(("TrafficDriving".into(), (f).into()));
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
pub struct GetSmsTemplate {
    /// 短信模板Code。
    ///
    /// - 在[CreateSmsTemplate](~~2807431~~)接口的返回参数中获取短信模板Code。
    /// - 在[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面查看短信模板Code。
    template_code: String,
}

impl sealed::Bound for GetSmsTemplate {}

impl GetSmsTemplate {
    pub fn new(template_code: impl Into<String>) -> Self {
        Self {
            template_code: template_code.into(),
        }
    }
}
impl crate::ToFormData for GetSmsTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for GetSmsTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "GetSmsTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<GetSmsTemplateResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("TemplateCode".into(), (&self.template_code).into()));

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
pub struct QuerySmsTemplateList {
    /// 当前页码。默认取值为**1**。
    #[setters(generate = true, strip_option)]
    page_index: Option<i32>,
    /// 每页显示的模板个数。取值范围：**1~50**，默认取值为**10**。
    #[setters(generate = true, strip_option)]
    page_size: Option<i32>,
}

impl sealed::Bound for QuerySmsTemplateList {}

impl QuerySmsTemplateList {
    pub fn new() -> Self {
        Self {
            page_index: None,
            page_size: None,
        }
    }
}
impl crate::ToFormData for QuerySmsTemplateList {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySmsTemplateList {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySmsTemplateList";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySmsTemplateListResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

        if let Some(f) = &self.page_index {
            params.push(("PageIndex".into(), (f).into()));
        }

        if let Some(f) = &self.page_size {
            params.push(("PageSize".into(), (f).into()));
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
pub struct UpdateSmsTemplate {
    /// 模板名称，长度不超过30个字符。您可在控制台[国内消息>模板管理](https://dysms.console.aliyun.com/domestic/text/template)页签查看未通过审核的模板名称。也可通过[QuerySmsTemplateList](~~QuerySmsTemplateList~~)接口查看未通过审核的短信模板名称。
    template_name: String,
    /// 未通过审核的模板Code。您可在控制台[国内消息>模板管理](https://dysms.console.aliyun.com/domestic/text/template)页签查看未通过审核的模板Code。也可通过[QuerySmsTemplateList](~~QuerySmsTemplateList~~)接口查看未通过审核的模板Code。
    template_code: String,
    /// 模板内容，长度不超过500个字符。
    ///
    /// 模板内容和变量内容必须符合[短信模板规范](~~463161~~)，否则模板会审核失败。您也可以在[申请模板](https://dysms.console.aliyun.com/domestic/text/template/add)页面，查看常用模板示例。使用示例模板可提高审核效率和成功率。变量规范请参见[TemplateContent参数变量规范](~~2806243~~)。
    template_content: String,
    /// 请描述您使用短信的业务场景或提供业务场景的线上链接，并提供短信完整示例（填入变量内容），信息完整有助于提高模板审核通过率。如未按规范进行填写或不填写，可能会影响您模板审核的通过。
    #[setters(generate = true, strip_option)]
    remark: Option<String>,
    /// 短信类型。取值：
    ///
    /// - **0**：验证码。
    /// - **1**：短信通知。
    /// - **2**：推广短信。
    /// - **3**：国际/港澳台消息。
    ///
    /// > 仅支持企业认证用户申请推广短信和国际/港澳台消息。个人用户与企业用户权益区别详情请参见[使用须知](https://help.aliyun.com/zh/sms/user-guide/usage-notes?spm=a2c4g.11186623.0.0.67447f576NJnE8)。
    template_type: i32,
    /// 模板申请时关联的短信签名。
    #[setters(generate = true, strip_option)]
    related_sign_name: Option<String>,
    /// 模板变量规则。变量规则填写请参见[示例文档](~~2806243~~)。
    #[setters(generate = true, strip_option)]
    template_rule: Option<String>,
    /// 更多资料，您可以补充上传业务证明文件或业务截图，有助于审核人员了解您的业务详情。如果您申请的短信模板为推广短信（即TemplateType为2），请上传用户授权证明材料，具体说明请参见[用户授权证明材料上传规范](~~312341~~)。
    #[setters(generate = true, strip_option)]
    more_data: Option<Vec<String>>,
    /// 业务场景。
    ///  - 模板关联的短信签名使用场景如果为“已上线APP”，`ApplySceneContent`必须为以`http://`开头或`https://`开头的App链接。
    ///  - 模板关联的短信签名使用场景如果为“已注册商标名”、“企事业单位名称”，`ApplySceneContent`必填。
    #[setters(generate = true, strip_option)]
    apply_scene_content: Option<String>,
    /// 国际/港澳台模板类型。当**TemplateType**参数为**3**时，国际/港澳台模板需要设置此参数，取值：
    /// - **0**：短信通知。
    /// - **1**：推广短信。
    /// - **2**：验证码。
    #[setters(generate = true, strip_option)]
    intl_type: Option<i32>,
    /// ><warning>
    /// 为管控短信内容安全，短信内容中包含“号码、链接”等引流信息，存在被运营商拦截导致发送失败的风险。建议尽可能在短信模板中避免包含相关信息，以降低短信发送失败风险。></warning>
    ///
    /// 引流信息列表JSON字符串。
    /// ><notice>JSON格式，传入前请转为字符串。></notice>
    ///
    /// ### 1. 字段说明：
    ///
    /// {
    ///     "trafficDrivingType":"引流类型",
    ///     "trafficDrivingContent":"引流内容",
    ///     "variableName":"变量名称",
    ///     "companyName":企事业单位名称,
    ///     "organizationCode":统一社会信用代码,
    ///     "icpNo":ICP备案/许可证号,
    ///     "icpPicOssKey":ICP备案截图OssKey,
    ///     "companyDifferentFromSignQuaReason":企事业单位名称与签名资质不同原因
    /// }
    ///
    /// ### 2. 注意事项：
    ///
    /// - 如果非变量则variableName不传。
    ///
    /// - 企事业单位名称与签名关联资质的企事业单位名称不同，则传入companyDifferentFromSignQuaReason。
    ///
    /// - 如果trfficDrivingType=DOMAIN，则传入所有参数。
    ///
    /// - 如果trafficDrivingType为其他，则传入trafficDrivingType、trafficDrivingContent、variableName（按需）、companyName、organizationCode、companyDifferentFromSignQuaReason（按需）
    ///
    /// ### 3. trafficDrivingType引流类型枚举值：
    /// ><warning>因监管要求，不支持手机号码。></warning>
    ///
    /// - DOMAIN：域名类型链接。
    /// - FIXED_PHONE：固定电话。
    /// - 400_PHONE：400开头电话。
    /// - 800_PHONE：800开头电话。
    /// - 95_PHONE：95开头电话。
    /// - 96_PHONE：96开头电话。
    /// - 1_PHONE：1开头，3~5位电话。
    /// - OTHER_PHONE：其他号码。
    #[setters(generate = true, strip_option)]
    traffic_driving: Option<String>,
}

impl sealed::Bound for UpdateSmsTemplate {}

impl UpdateSmsTemplate {
    pub fn new(
        template_name: impl Into<String>,
        template_code: impl Into<String>,
        template_content: impl Into<String>,
        template_type: impl Into<i32>,
    ) -> Self {
        Self {
            template_name: template_name.into(),
            template_code: template_code.into(),
            template_content: template_content.into(),
            remark: None,
            template_type: template_type.into(),
            related_sign_name: None,
            template_rule: None,
            more_data: None,
            apply_scene_content: None,
            intl_type: None,
            traffic_driving: None,
        }
    }
}
impl crate::ToFormData for UpdateSmsTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for UpdateSmsTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UpdateSmsTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<UpdateSmsTemplateResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(11);

        if let Some(f) = &self.apply_scene_content {
            params.push(("ApplySceneContent".into(), (f).into()));
        }

        if let Some(f) = &self.intl_type {
            params.push(("IntlType".into(), (f).into()));
        }

        if let Some(f) = &self.more_data {
            if let Ok(json) = serde_json::to_string(f) {
                params.push(("MoreData".into(), json.into()));
            }
        }

        if let Some(f) = &self.related_sign_name {
            params.push(("RelatedSignName".into(), (f).into()));
        }

        if let Some(f) = &self.remark {
            params.push(("Remark".into(), (f).into()));
        }
        params.push(("TemplateCode".into(), (&self.template_code).into()));
        params.push(("TemplateContent".into(), (&self.template_content).into()));
        params.push(("TemplateName".into(), (&self.template_name).into()));

        if let Some(f) = &self.template_rule {
            params.push(("TemplateRule".into(), (f).into()));
        }
        params.push(("TemplateType".into(), (&self.template_type).into()));

        if let Some(f) = &self.traffic_driving {
            params.push(("TrafficDriving".into(), (f).into()));
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

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteSmsTemplate {
    /// 短信模板Code。支持删除已撤回、审核失败或审核通过的模板，审核中的短信模板不支持删除。
    ///
    /// - 您可通过[QuerySmsTemplateList](~~419288~~)接口获取短信模板Code。
    ///
    /// - 或在短信控制台[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面获取短信模板Code。
    /// ><notice>删除短信模板后不可恢复，且后续不可再使用该模板发送短信，请谨慎操作。></notice>
    template_code: String,
}

impl sealed::Bound for DeleteSmsTemplate {}

impl DeleteSmsTemplate {
    pub fn new(template_code: impl Into<String>) -> Self {
        Self {
            template_code: template_code.into(),
        }
    }
}
impl crate::ToFormData for DeleteSmsTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for DeleteSmsTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "DeleteSmsTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<DeleteSmsTemplateResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("TemplateCode".into(), (&self.template_code).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AddSmsTemplate {
    /// 短信类型。取值：
    ///
    /// - **0**：验证码。
    /// - **1**：短信通知。
    /// - **2**：推广短信。
    /// - **3**：国际/港澳台消息。
    ///
    /// > 仅支持企业认证用户申请推广短信和国际/港澳台消息。个人用户与企业用户权益区别详情请参见[使用须知](~~55324~~)。
    template_type: i32,
    /// 模板名称，长度不超过30个字符。
    template_name: String,
    /// 模板内容，长度不超过500个字符。
    ///
    /// 模板内容和变量内容必须符合[短信模板规范](~~463161~~)，否则模板会审核失败。您也可以在[申请模板](https://dysms.console.aliyun.com/domestic/text/template/add)页面，查看常用模板示例。使用示例模板可提高审核效率和成功率。变量规范请参见[TemplateContent参数变量规范](~~2806243~~)。
    template_content: String,
    /// 短信模板申请说明，长度不超过100个字符。
    ///
    /// 模板审核的参考信息，完善申请说明有助于审核人员理解您的业务场景，提高审核效率。填写指导：
    ///
    /// - 您可以提供已上线业务的使用场景。
    /// - 您可以提供真实场景的短信示例，体现您的业务场景。
    /// - 您可以提供变量的传参内容，详细描述业务使用场景和选择这个变量属性的原因。
    /// - 您可以提供实际业务的网站链接、已备案的域名地址、应用市场下载链接等。
    /// - 登录场景，您可以提供测试账号和密码。
    remark: String,
}

impl sealed::Bound for AddSmsTemplate {}

impl AddSmsTemplate {
    pub fn new(
        template_type: impl Into<i32>,
        template_name: impl Into<String>,
        template_content: impl Into<String>,
        remark: impl Into<String>,
    ) -> Self {
        Self {
            template_type: template_type.into(),
            template_name: template_name.into(),
            template_content: template_content.into(),
            remark: remark.into(),
        }
    }
}
impl crate::ToFormData for AddSmsTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for AddSmsTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "AddSmsTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<AddSmsTemplateResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(4);
        params.push(("Remark".into(), (&self.remark).into()));
        params.push(("TemplateContent".into(), (&self.template_content).into()));
        params.push(("TemplateName".into(), (&self.template_name).into()));
        params.push(("TemplateType".into(), (&self.template_type).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ModifySmsTemplate {
    /// 短信类型。取值：
    ///
    /// - **0**：验证码。
    /// - **1**：短信通知。
    /// - **2**：推广短信。
    /// - **3**：国际/港澳台消息。
    template_type: i32,
    /// 模板名称，长度限制为1~30个字符。
    template_name: String,
    /// 未通过审核的短信模板Code。
    ///
    /// - 通过[QuerySmsTemplateList](~~419288~~)接口获取未通过审核的短信模板Code。
    /// - 在[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面查看未通过审核的短信模板Code。
    template_code: String,
    /// 模板内容，长度不超过500个字符。
    ///
    /// 模板内容和变量内容必须符合[短信模板规范](~~463161~~)，否则模板会审核失败。您也可以在[申请模板](https://dysms.console.aliyun.com/domestic/text/template/add)页面，查看常用模板示例。使用示例模板可提高审核效率和成功率。变量规范请参见[TemplateContent参数变量规范](~~2806243~~)。
    template_content: String,
    /// 短信模板申请说明，长度不超过100个字符。
    ///
    /// 模板审核的参考信息，完善申请说明有助于审核人员理解您的业务场景，提高审核效率。填写指导：
    ///
    /// - 您可以提供已上线业务的使用场景。
    /// - 您可以提供真实场景的短信示例，体现您的业务场景。
    /// - 您可以提供变量的传参内容，详细描述业务使用场景和选择这个变量属性的原因。
    /// - 您可以提供实际业务的网站链接、已备案的域名地址、应用市场下载链接等。
    /// - 登录场景，您可以提供测试账号和密码。
    remark: String,
}

impl sealed::Bound for ModifySmsTemplate {}

impl ModifySmsTemplate {
    pub fn new(
        template_type: impl Into<i32>,
        template_name: impl Into<String>,
        template_code: impl Into<String>,
        template_content: impl Into<String>,
        remark: impl Into<String>,
    ) -> Self {
        Self {
            template_type: template_type.into(),
            template_name: template_name.into(),
            template_code: template_code.into(),
            template_content: template_content.into(),
            remark: remark.into(),
        }
    }
}
impl crate::ToFormData for ModifySmsTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for ModifySmsTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "ModifySmsTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<ModifySmsTemplateResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);
        params.push(("Remark".into(), (&self.remark).into()));
        params.push(("TemplateCode".into(), (&self.template_code).into()));
        params.push(("TemplateContent".into(), (&self.template_content).into()));
        params.push(("TemplateName".into(), (&self.template_name).into()));
        params.push(("TemplateType".into(), (&self.template_type).into()));

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
pub struct QuerySmsTemplate {
    /// 短信模板CODE。
    /// - 在[AddSmsTemplate](~~121208~~)接口的返回参数中获取短信模板Code。
    /// - 在[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面查看短信模板Code。
    template_code: String,
}

impl sealed::Bound for QuerySmsTemplate {}

impl QuerySmsTemplate {
    pub fn new(template_code: impl Into<String>) -> Self {
        Self {
            template_code: template_code.into(),
        }
    }
}
impl crate::ToFormData for QuerySmsTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySmsTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySmsTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySmsTemplateResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("TemplateCode".into(), (&self.template_code).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}

/// ## 请求示例
/// - 服务地址URL：dysmsapi.aliyuncs.com（中国站）
/// - 请求风格：RPC
/// - 请求方式：POST/GET （推荐使用POST）
/// - 公共请求头：[V3版本请求体公共请求头](~~2593177#sectiondiv-726-v1i-gel~~)
/// - 请求参数：见上方请求参数表格
///
/// ### SDK示例
/// 如果您需要了解如何使用SDK，请参见[首次调用API](~~2841024~~)。
/// ```ignore
/// // 构造请求对象
/// SendSmsRequest sendSmsRequest = new SendSmsRequest()
///             .setPhoneNumbers("1390000****")
///             .setSignName("阿里云")
///             .setTemplateCode("SMS_15305****")
///             // TemplateParam为序列化后的JSON字符串。其中\"表示转义后的双引号。
///             .setTemplateParam("{\"name\":\"张三\",\"number\":\"1390000****\"}");
///
/// // 发送API请求
/// SendSmsResponse sendSmsResponse = client.sendSms(sendSmsRequest);
/// ```
/// 您可以访问[OpenAPI门户](https://api.aliyun.com/api/Dysmsapi/2017-05-25/SendSms?tab=DEMO&lang=JAVA)，查看各语言SDK请求完整示例。
///
/// ### 自签名请求示例
/// 推荐您通过SDK调用API，SDK已经封装了签名等机制。
/// ```ignore
/// POST /?PhoneNumbers=123****4567&SignName=阿里云短信测试&TemplateCode=SMS_154950909&TemplateParam={"code":"1234"} HTTP/1.1
/// Host: dysmsapi.aliyuncs.com
/// Authorization: ACS3-HMAC-SHA256 Credential=YourAccessKeyId,SignedHeaders=host;x-acs-action;x-acs-content-sha256;x-acs-date;x-acs-signature-nonce;x-acs-version,Signature=06563a9e1b43f5dfe96b81********ceab24a1d853912eee15083a6f0f3283c0
/// x-acs-action: SendSms
/// x-acs-version: 2017-05-25
/// x-acs-signature-nonce: d410180a5abf7f********74aca91fc0
/// x-acs-date: 2024-12-02T06:53:09Z
/// x-acs-content-sha256: e3b0c44298fc1c149afb********b92427ae41e4649b934ca495991b7852b855
/// ```
/// 自定义封装API调用签名机制，请参见[V3版本签名机制示例](~~2593177#79cbd5a0c1gif~~)。
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SendSms {
    /// 接收短信的手机号码。手机号码格式：
    ///
    /// * 国内短信：+/+86/0086/86或无任何前缀的手机号码，例如1390000\*\*\*\*。
    /// * 国际/港澳台消息：国际区号+号码，例如852000012\*\*\*\*。
    /// * 接收测试短信的手机号：必须先在[控制台](https://dysms.console.aliyun.com/quickstart)绑定测试手机号后才可以发送。
    ///
    /// >支持向不同的手机号码发送短信，手机号码之间以半角逗号（,）分隔。上限为1000个手机号码。批量发送相对于单条发送，及时性稍有延迟。验证码类型的短信，建议单条发送。
    phone_numbers: String,
    /// 短信签名名称。
    ///
    /// 您可以通过[QuerySmsSignList](~~419282~~)接口查询当前账号已申请的签名或在[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/sign)查看签名列表，必须使用**审核通过**的签名发送短信。
    ///
    /// > 如果验证码签名和通用签名相同时，默认使用通用签名发送短信。
    sign_name: String,
    /// 短信模板Code。
    ///
    /// 您可以通过[QuerySmsTemplateList](~~419288~~)接口查询当前账号已申请的模板或在[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/template)查看模板列表，必须使用已**审核通过**的模板Code发送短信。
    template_code: String,
    /// 短信模板变量对应的实际值，请传入**JSON字符串**。当您选择的模板内容含有变量时，此参数必填。参数个数应与模板内变量个数一致。
    ///
    /// > - 如果JSON中需要带换行符，请参照标准的JSON协议处理。
    /// > - 模板变量规范，请参见[短信模板规范](~~463161~~)。
    #[setters(generate = true, strip_option)]
    template_param: Option<String>,
    /// 上行短信扩展码。上行短信指发送给通信服务提供商的短信，用于定制某种服务、完成查询，或是办理某种业务等，需要收费，按运营商普通短信资费进行扣费。
    /// > 扩展码是生成签名时系统自动默认生成的，不支持自行传入。无特殊需要可忽略此字段。
    #[setters(generate = true, strip_option)]
    sms_up_extend_code: Option<String>,
    /// 外部流水扩展字段。
    ///
    /// > 无特殊需要可忽略此字段。
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
}

impl sealed::Bound for SendSms {}

impl SendSms {
    pub fn new(
        phone_numbers: impl Into<String>,
        sign_name: impl Into<String>,
        template_code: impl Into<String>,
    ) -> Self {
        Self {
            phone_numbers: phone_numbers.into(),
            sign_name: sign_name.into(),
            template_code: template_code.into(),
            template_param: None,
            sms_up_extend_code: None,
            out_id: None,
        }
    }
}
impl crate::ToFormData for SendSms {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for SendSms {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SendSms";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<SendSmsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(6);

        if let Some(f) = &self.out_id {
            params.push(("OutId".into(), (f).into()));
        }
        params.push(("PhoneNumbers".into(), (&self.phone_numbers).into()));
        params.push(("SignName".into(), (&self.sign_name).into()));

        if let Some(f) = &self.sms_up_extend_code {
            params.push(("SmsUpExtendCode".into(), (f).into()));
        }
        params.push(("TemplateCode".into(), (&self.template_code).into()));

        if let Some(f) = &self.template_param {
            params.push(("TemplateParam".into(), (f).into()));
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
pub struct SendBatchSms {
    /// 接收短信的手机号码。手机号码格式：
    ///
    /// * 国内短信：+/+86/0086/86或无任何前缀的手机号码，例如1590000\*\*\*\*。
    /// * 国际/港澳台消息：国际区号+号码，例如852000012\*\*\*\*。
    ///
    /// > 验证码类型短信，建议使用[SendSms](~~419273~~)接口单条发送。
    phone_number_json: String,
    /// 短信签名名称，签名数量需与手机号码数量一致。
    ///
    /// 您可以通过[QuerySmsSignList](~~419282~~)接口查询当前账号已申请的签名或在[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/sign)查看已审核通过的签名，必须使用审核通过的签名发送短信。
    sign_name_json: String,
    /// 短信模板Code。国内短信模板和国际短信模板不可以混用。
    ///
    /// 您可以通过[QuerySmsTemplateList](~~419288~~)接口查询当前账号已申请的模板或在[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/template)查看模板列表，必须使用已审核通过的模板Code发送短信。
    template_code: String,
    /// 短信模板变量对应的实际值。当您选择的模板内含有变量时，此参数必填。
    ///
    /// > - 模板变量值的个数必须与手机号码、签名的个数相同、内容一一对应，表示向指定手机号码中发对应签名的短信，且短信模板中的变量参数替换为对应的值。
    /// > - 如果JSON中需要带换行符，请参照标准的JSON协议处理。
    #[setters(generate = true, strip_option)]
    template_param_json: Option<String>,
    /// 上行短信扩展码，JSON数组格式。
    ///
    /// > 无特殊需要可忽略此字段。
    #[setters(generate = true, strip_option)]
    sms_up_extend_code_json: Option<String>,
    /// 外部流水扩展字段，长度小于256的字符串。
    ///
    /// > 无特殊需要可忽略此字段。
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
}

impl sealed::Bound for SendBatchSms {}

impl SendBatchSms {
    pub fn new(
        phone_number_json: impl Into<String>,
        sign_name_json: impl Into<String>,
        template_code: impl Into<String>,
    ) -> Self {
        Self {
            phone_number_json: phone_number_json.into(),
            sign_name_json: sign_name_json.into(),
            template_code: template_code.into(),
            template_param_json: None,
            sms_up_extend_code_json: None,
            out_id: None,
        }
    }
}
impl crate::ToFormData for SendBatchSms {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(4);
        params.push(("PhoneNumberJson".into(), (&self.phone_number_json).into()));
        params.push(("SignNameJson".into(), (&self.sign_name_json).into()));

        if let Some(f) = &self.sms_up_extend_code_json {
            params.push(("SmsUpExtendCodeJson".into(), f.into()));
        }

        if let Some(f) = &self.template_param_json {
            params.push(("TemplateParamJson".into(), f.into()));
        }

        params
    }
}

impl crate::Request for SendBatchSms {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SendBatchSms";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<SendBatchSmsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);

        if let Some(f) = &self.out_id {
            params.push(("OutId".into(), (f).into()));
        }
        params.push(("TemplateCode".into(), (&self.template_code).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}

/// 您可以访问[OpenAPI](https://api.aliyun.com/api/Dysmsapi/2017-05-25/QuerySendDetails?tab=DEMO&lang=JAVA)门户，查看各语言SDK请求示例。
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySendDetails {
    /// 需要查询的手机号码。格式：
    ///
    /// * 国内短信：11位手机号码，例如1390000\*\*\*\*。
    /// * 国际/港澳台消息：国际区号+号码，例如8520000\*\*\*\*。
    ///
    /// > 只能传入一个手机号码。
    phone_number: String,
    /// 发送回执ID。即发送流水号，调用[SendSms](~~419273~~)或[SendBatchSms](~~419274~~)发送短信时，返回值中的BizId字段。
    ///
    /// > 只能传入一个Bizid的值，无法传入多个Bizid。
    #[setters(generate = true, strip_option)]
    biz_id: Option<String>,
    /// 短信发送日期，支持查询最近30天的记录。
    ///
    /// 格式：**yyyyMMdd**，例如20250601。
    send_date: String,
    /// 每页显示的短信记录数量，供分页查看发送记录使用。
    ///
    /// 取值范围为1~50。
    page_size: i64,
    /// 发送记录的当前页码，供分页查看发送记录使用。
    current_page: i64,
}

impl sealed::Bound for QuerySendDetails {}

impl QuerySendDetails {
    pub fn new(
        phone_number: impl Into<String>,
        send_date: impl Into<String>,
        page_size: impl Into<i64>,
        current_page: impl Into<i64>,
    ) -> Self {
        Self {
            phone_number: phone_number.into(),
            biz_id: None,
            send_date: send_date.into(),
            page_size: page_size.into(),
            current_page: current_page.into(),
        }
    }
}
impl crate::ToFormData for QuerySendDetails {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySendDetails {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySendDetails";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySendDetailsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);

        if let Some(f) = &self.biz_id {
            params.push(("BizId".into(), (f).into()));
        }
        params.push(("CurrentPage".into(), (&self.current_page).into()));
        params.push(("PageSize".into(), (&self.page_size).into()));
        params.push(("PhoneNumber".into(), (&self.phone_number).into()));
        params.push(("SendDate".into(), (&self.send_date).into()));

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
pub struct QuerySendStatistics {
    /// 短信发送范围。取值：
    ///
    /// - **1**：国内短信发送记录。
    ///
    /// - **2**：国际/港澳台短信发送记录。
    is_globe: i32,
    /// 开始日期，格式为yyyyMMdd。
    start_date: String,
    /// 结束日期，格式为yyyyMMdd。
    end_date: String,
    /// 当前页码。
    page_index: i32,
    /// 每页显示的条数。取值范围：**1~50**。
    page_size: i32,
    /// 模板类型。取值：
    ///
    /// - **0**：验证码。
    ///
    /// - **1**：通知短信。
    ///
    /// - **2**：推广短信。（仅支持企业客户）
    ///
    /// - **3**：国际/港澳台消息。（仅支持企业客户）
    ///
    /// - **7**：数字短信。
    #[setters(generate = true, strip_option)]
    template_type: Option<i32>,
    /// 签名名称。
    #[setters(generate = true, strip_option)]
    sign_name: Option<String>,
}

impl sealed::Bound for QuerySendStatistics {}

impl QuerySendStatistics {
    pub fn new(
        is_globe: impl Into<i32>,
        start_date: impl Into<String>,
        end_date: impl Into<String>,
        page_index: impl Into<i32>,
        page_size: impl Into<i32>,
    ) -> Self {
        Self {
            is_globe: is_globe.into(),
            start_date: start_date.into(),
            end_date: end_date.into(),
            page_index: page_index.into(),
            page_size: page_size.into(),
            template_type: None,
            sign_name: None,
        }
    }
}
impl crate::ToFormData for QuerySendStatistics {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QuerySendStatistics {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QuerySendStatistics";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QuerySendStatisticsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(7);
        params.push(("EndDate".into(), (&self.end_date).into()));
        params.push(("IsGlobe".into(), (&self.is_globe).into()));
        params.push(("PageIndex".into(), (&self.page_index).into()));
        params.push(("PageSize".into(), (&self.page_size).into()));

        if let Some(f) = &self.sign_name {
            params.push(("SignName".into(), (f).into()));
        }
        params.push(("StartDate".into(), (&self.start_date).into()));

        if let Some(f) = &self.template_type {
            params.push(("TemplateType".into(), (f).into()));
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

/// ### 请求参数说明
/// GetOSSInfoForCardTemplate接口没有入参，直接调用接口即可获取OSS上传信息。
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetOSSInfoForCardTemplate {}

impl sealed::Bound for GetOSSInfoForCardTemplate {}

impl GetOSSInfoForCardTemplate {
    pub fn new() -> Self {
        Self {}
    }
}
impl crate::ToFormData for GetOSSInfoForCardTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for GetOSSInfoForCardTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "GetOSSInfoForCardTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<GetOSSInfoForCardTemplateResponse>;

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
pub struct GetMediaResourceId {
    /// 资源类型。
    ///
    /// - **1**：文本
    /// - **2**：图片。小图片限制在100 KB以内，大图片限制在2 MB以内，图片要求清晰；图片格式支持JPG、JPEG、PNG。
    /// - **3**：音频
    /// - **4**：视频。视频格式支持MP4。
    /// >
    /// > - 资源类型为图片时**img_rate**必填。
    /// > - 1:1比例：oneToOne
    /// > - 16:9比例：sixteenToNine
    /// > - 3:1比例：threeToOne
    /// > - 48:65比例：fortyEightToSixtyFiv
    resource_type: i32,
    /// 获取的资源地址。
    oss_key: String,
    /// 文件大小，单位：Byte。
    file_size: i64,
    /// 扩展字段。
    ///
    /// > 资源类型为**图片**时必填。
    #[setters(generate = true, strip_option)]
    extend_info: Option<String>,
    /// 上传资源的描述。
    #[setters(generate = true, strip_option)]
    memo: Option<String>,
}

impl sealed::Bound for GetMediaResourceId {}

impl GetMediaResourceId {
    pub fn new(
        resource_type: impl Into<i32>,
        oss_key: impl Into<String>,
        file_size: impl Into<i64>,
    ) -> Self {
        Self {
            resource_type: resource_type.into(),
            oss_key: oss_key.into(),
            file_size: file_size.into(),
            extend_info: None,
            memo: None,
        }
    }
}
impl crate::ToFormData for GetMediaResourceId {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for GetMediaResourceId {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "GetMediaResourceId";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<GetMediaResourceIdResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);

        if let Some(f) = &self.extend_info {
            params.push(("ExtendInfo".into(), (f).into()));
        }
        params.push(("FileSize".into(), (&self.file_size).into()));

        if let Some(f) = &self.memo {
            params.push(("Memo".into(), (f).into()));
        }
        params.push(("OssKey".into(), (&self.oss_key).into()));
        params.push(("ResourceType".into(), (&self.resource_type).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateCardSmsTemplate {
    /// 卡片短信模板名称。
    template_name: String,
    /// 卡片短信的模板内容。
    ///
    /// > - Template、ExtendInfo、TemplateContent、TmpCard、Action等字段说明，请参见[卡片短信模板参数字段说明](~~434929~~)。
    /// > - 不同类型的卡片短信模板的内容结构不同，详情请参见[卡片短信模板示例](~~435361~~)。
    template: crate::OpenObject,
    /// 对上传模板的描述。
    #[setters(generate = true, strip_option)]
    memo: Option<String>,
    /// 模板提交的厂商。厂商类型取值：
    ///
    /// - **HuaWei**：表示华为厂商。
    /// - **XiaoMi**：表示小米厂商。
    /// - **OPPO**：表示OPPO厂商。
    /// - **VIVO**：表示VIVO厂商。
    /// - **MEIZU**：表示魅族厂商。
    /// - **HONOR**：表示荣耀厂商。
    ///
    /// > 该参数不填时，系统自动匹配模板支持的手机厂商。
    #[setters(generate = true, strip_option)]
    factorys: Option<String>,
}

impl sealed::Bound for CreateCardSmsTemplate {}

impl CreateCardSmsTemplate {
    pub fn new(template_name: impl Into<String>, template: impl Into<crate::OpenObject>) -> Self {
        Self {
            template_name: template_name.into(),
            template: template.into(),
            memo: None,
            factorys: None,
        }
    }
}
impl crate::ToFormData for CreateCardSmsTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CreateCardSmsTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CreateCardSmsTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<CreateCardSmsTemplateResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(4);

        if let Some(f) = &self.factorys {
            params.push(("Factorys".into(), (f).into()));
        }

        if let Some(f) = &self.memo {
            params.push(("Memo".into(), (f).into()));
        }
        if let Ok(json) = serde_json::to_string(&self.template) {
            params.push(("Template".into(), json.into()));
        }

        params.push(("TemplateName".into(), (&self.template_name).into()));

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
pub struct QueryCardSmsTemplate {
    /// 卡片短信模板Code。取值：
    ///
    /// - 调用[创建卡片短信模板](~~CreateCardSmsTemplate~~)接口后，返回参数**TemplateCode**即为新创建的卡片短信模板Code。
    ///
    /// - 您也可登录控制台[国内卡片短信](https://dysms.console.aliyun.com/domestic/card)页面，在**模板管理**页签查看卡片短信模板Code。
    template_code: String,
}

impl sealed::Bound for QueryCardSmsTemplate {}

impl QueryCardSmsTemplate {
    pub fn new(template_code: impl Into<String>) -> Self {
        Self {
            template_code: template_code.into(),
        }
    }
}
impl crate::ToFormData for QueryCardSmsTemplate {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QueryCardSmsTemplate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QueryCardSmsTemplate";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QueryCardSmsTemplateResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("TemplateCode".into(), (&self.template_code).into()));

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}

/// 完整请求参数示例：
///
/// ```ignore
/// {
///   "Mobiles": [
///     {
///       "#6#mobile": "137******00"
///     },
///     {
///       "#6#mobile": "130******00"
///     }
///   ],
///   "TemplateCode": "CARD_SMS_6***9"
/// }
/// ```
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CheckMobilesCardSupport {
    /// 卡片短信模板Code。
    /// 登录短信服务控制台[国内卡片短信](https://dysms.console.aliyun.com/domestic/card)页面，在**模板管理**页签查看卡片短信模板列表。
    ///
    /// >必须是已添加、并通过审核的卡片短信模板。
    template_code: String,
    /// 待查询的手机号列表。
    mobiles: Vec<crate::OpenObject>,
}

impl sealed::Bound for CheckMobilesCardSupport {}

impl CheckMobilesCardSupport {
    pub fn new(
        template_code: impl Into<String>,
        mobiles: impl Into<Vec<crate::OpenObject>>,
    ) -> Self {
        Self {
            template_code: template_code.into(),
            mobiles: mobiles.into(),
        }
    }
}
impl crate::ToFormData for CheckMobilesCardSupport {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for CheckMobilesCardSupport {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "CheckMobilesCardSupport";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<CheckMobilesCardSupportResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        crate::FlatSerialize::flat_serialize(&self.mobiles, "Mobiles", &mut params);

        params.push(("TemplateCode".into(), (&self.template_code).into()));

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
pub struct QueryMobilesCardSupport {
    /// 卡片短信模板Code。请在控制台选择**国内消息**>[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面查看。
    ///
    /// >必须是已添加、并通过审核的卡片短信模板。
    template_code: String,
    /// 手机号列表。
    mobiles: Vec<crate::OpenObject>,
    /// 手机号码加密方式。取值：
    /// - SHA1：SHA1加密方式。
    /// - NORMAL：不加密，明文传输。
    #[setters(generate = true, strip_option)]
    encrypt_type: Option<EncryptType>,
}

impl sealed::Bound for QueryMobilesCardSupport {}

impl QueryMobilesCardSupport {
    pub fn new(
        template_code: impl Into<String>,
        mobiles: impl Into<Vec<crate::OpenObject>>,
    ) -> Self {
        Self {
            template_code: template_code.into(),
            mobiles: mobiles.into(),
            encrypt_type: None,
        }
    }
}
impl crate::ToFormData for QueryMobilesCardSupport {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QueryMobilesCardSupport {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QueryMobilesCardSupport";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QueryMobilesCardSupportResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.encrypt_type {
            params.push(("EncryptType".into(), (f).into()));
        }
        if let Ok(json) = serde_json::to_string(&self.mobiles) {
            params.push(("Mobiles".into(), json.into()));
        }

        params.push(("TemplateCode".into(), (&self.template_code).into()));

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
pub struct GetCardSmsLink {
    /// 卡片短信模板Code。请在控制台[卡片短信>模板管理](https://dysms.console.aliyun.com/domestic/card)页面选择已通过审核的卡片短信模板Code。
    card_template_code: String,
    /// 外部流水扩展字段。
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
    /// 手机号码、用户ID或系统内部标识。
    ///
    /// > - 最多支持1万个手机号码。
    /// > - 用户也可自定义标识，长度不超过60个字符。
    /// > - OPPO厂商的模板一次最多申请10个。
    #[setters(generate = true, strip_option)]
    phone_number_json: Option<String>,
    /// 短信签名名称。
    /// 请在控制台[国内消息>签名管理](https://dysms.console.aliyun.com/domestic/text/sign)页签下**签名名称**一列查看。也可通过[QuerySmsSignList](https://help.aliyun.com/zh/sms/developer-reference/api-dysmsapi-2017-05-25-querysmssignlist?spm=a2c4g.11186623.help-menu-search-44282.d_0)接口查看的短信签名名称。
    ///
    /// >必须是已添加、并通过审核的短信签名；且短信签名的个数必须与手机号码的个数相同、内容一一对应。
    sign_name_json: String,
    /// 卡片短信动参。
    #[setters(generate = true, strip_option)]
    card_template_param_json: Option<String>,
    /// 卡片短信短链编码类型。取值：
    /// - 1：群发。
    /// - 2：个性化。
    #[setters(generate = true, strip_option)]
    card_code_type: Option<i32>,
    /// 卡片短信短链类型。取值：
    /// - 1：标准生成短码。
    /// - 2：自定义生成短码。
    ///
    /// > **CardLinkType**不填时，默认为标准生成短码。如需生成自定义短码，请联系阿里运营人员进行报备。
    #[setters(generate = true, strip_option)]
    card_link_type: Option<i32>,
    /// 发送账号分配的短链域名，需要提前报备。
    ///
    /// > - 当**CardLinkType**为**2**时，**Domain**参数必填。
    /// > - 当**Domain**参数不填时，则使用系统默认域名。长度不超过100个字符。
    #[setters(generate = true, strip_option)]
    domain: Option<String>,
    /// 客户自定义短码。长度为4~8位的数字或字母。
    ///
    /// > 当生成类型为自定义生成短码时必填。
    #[setters(generate = true, strip_option)]
    custom_short_code_json: Option<String>,
}

impl sealed::Bound for GetCardSmsLink {}

impl GetCardSmsLink {
    pub fn new(card_template_code: impl Into<String>, sign_name_json: impl Into<String>) -> Self {
        Self {
            card_template_code: card_template_code.into(),
            out_id: None,
            phone_number_json: None,
            sign_name_json: sign_name_json.into(),
            card_template_param_json: None,
            card_code_type: None,
            card_link_type: None,
            domain: None,
            custom_short_code_json: None,
        }
    }
}
impl crate::ToFormData for GetCardSmsLink {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for GetCardSmsLink {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "GetCardSmsLink";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<GetCardSmsLinkResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(9);

        if let Some(f) = &self.card_code_type {
            params.push(("CardCodeType".into(), (f).into()));
        }

        if let Some(f) = &self.card_link_type {
            params.push(("CardLinkType".into(), (f).into()));
        }
        params.push(("CardTemplateCode".into(), (&self.card_template_code).into()));

        if let Some(f) = &self.card_template_param_json {
            params.push(("CardTemplateParamJson".into(), (f).into()));
        }

        if let Some(f) = &self.custom_short_code_json {
            params.push(("CustomShortCodeJson".into(), (f).into()));
        }

        if let Some(f) = &self.domain {
            params.push(("Domain".into(), (f).into()));
        }

        if let Some(f) = &self.out_id {
            params.push(("OutId".into(), (f).into()));
        }

        if let Some(f) = &self.phone_number_json {
            params.push(("PhoneNumberJson".into(), (f).into()));
        }
        params.push(("SignNameJson".into(), (&self.sign_name_json).into()));

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
pub struct GetCardSmsDetails {
    /// 分页查看发送记录，指定发送记录的当前页码。
    #[setters(generate = true, strip_option)]
    current_page: Option<i64>,
    /// 分页查看发送记录，指定每页显示的卡片短信记录数量。
    ///
    /// 取值范围为1~50。
    #[setters(generate = true, strip_option)]
    page_size: Option<i64>,
    /// 卡片短信发送日期，支持查询最近30天的记录。
    ///
    /// 格式为yyyyMMdd，例如20240112。
    send_date: String,
    /// 接收短信的国内手机号码。格式：11位手机号码，例如1390000****。
    phone_number: String,
    /// 卡片短信发送ID。通过[SendCardSms](~~434120~~)接口或[SendBatchCardSms](~~434119~~)接口发送卡片短信时，获取返回值中的BizCardId字段。
    #[setters(generate = true, strip_option)]
    biz_card_id: Option<String>,
    /// 文本短信发送ID，调用[SendCardSms](~~434120~~)接口或[SendBatchCardSms](~~434119~~)接口发送卡片短信时，获取返回值中的BizSmsId字段。
    #[setters(generate = true, strip_option)]
    biz_sms_id: Option<String>,
    /// 数字短信发送ID，调用[SendCardSms](~~434120~~)接口或[SendBatchCardSms](~~434119~~)接口发送卡片短信时，获取返回值中的BizDigitalId字段。
    #[setters(generate = true, strip_option)]
    biz_digit_id: Option<String>,
}

impl sealed::Bound for GetCardSmsDetails {}

impl GetCardSmsDetails {
    pub fn new(send_date: impl Into<String>, phone_number: impl Into<String>) -> Self {
        Self {
            current_page: None,
            page_size: None,
            send_date: send_date.into(),
            phone_number: phone_number.into(),
            biz_card_id: None,
            biz_sms_id: None,
            biz_digit_id: None,
        }
    }
}
impl crate::ToFormData for GetCardSmsDetails {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for GetCardSmsDetails {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "GetCardSmsDetails";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<GetCardSmsDetailsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(7);

        if let Some(f) = &self.biz_card_id {
            params.push(("BizCardId".into(), (f).into()));
        }

        if let Some(f) = &self.biz_digit_id {
            params.push(("BizDigitId".into(), (f).into()));
        }

        if let Some(f) = &self.biz_sms_id {
            params.push(("BizSmsId".into(), (f).into()));
        }

        if let Some(f) = &self.current_page {
            params.push(("CurrentPage".into(), (f).into()));
        }

        if let Some(f) = &self.page_size {
            params.push(("PageSize".into(), (f).into()));
        }
        params.push(("PhoneNumber".into(), (&self.phone_number).into()));
        params.push(("SendDate".into(), (&self.send_date).into()));

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
pub struct QueryCardSmsTemplateReport {
    /// 卡片短信对象。
    template_codes: Vec<String>,
    /// 开始时间，格式为yyyy-MM-dd HH:mm:ss
    start_date: String,
    /// 结束时间，格式为yyyy-MM-dd HH:mm:ss
    end_date: String,
}

impl sealed::Bound for QueryCardSmsTemplateReport {}

impl QueryCardSmsTemplateReport {
    pub fn new(
        template_codes: impl Into<Vec<String>>,
        start_date: impl Into<String>,
        end_date: impl Into<String>,
    ) -> Self {
        Self {
            template_codes: template_codes.into(),
            start_date: start_date.into(),
            end_date: end_date.into(),
        }
    }
}
impl crate::ToFormData for QueryCardSmsTemplateReport {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for QueryCardSmsTemplateReport {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QueryCardSmsTemplateReport";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QueryCardSmsTemplateReportResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("EndDate".into(), (&self.end_date).into()));
        params.push(("StartDate".into(), (&self.start_date).into()));
        crate::FlatSerialize::flat_serialize(&self.template_codes, "TemplateCodes", &mut params);

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
pub struct SendCardSms {
    /// 卡片短信对象。
    card_objects: Vec<CardObject>,
    /// 短信签名名称。您可以通过[QuerySmsSignList](~~419282~~)接口查询当前账号已申请的签名或在[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/sign)查看签名列表。
    /// >必须是通过审核的短信签名。
    sign_name: String,
    /// 卡片短信模板Code。请在控制台**卡片短信**[模板管理](https://dysms.console.aliyun.com/domestic/card)页面选择已通过审核的卡片短信模板Code。
    card_template_code: String,
    /// 回落文本短信的模板Code。**FallbackType**选择**SMS**回落文本短信时，此参数必填。
    ///
    /// 您可以通过[QuerySmsTemplateList](~~419288~~)接口查询当前账号已申请的模板或在[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/template)查看模板列表。
    /// >必须是已添加、并通过审核的短信模板。
    #[setters(generate = true, strip_option)]
    sms_template_code: Option<String>,
    /// 上行短信扩展码。上行短信，指发送给通信服务提供商的短信，用于定制某种服务、完成查询，或是办理某种业务等，需要收费的，按运营商普通短信资费进行扣费。
    /// >无上述需求的用户请忽略此字段。
    #[setters(generate = true, strip_option)]
    sms_up_extend_code: Option<String>,
    /// 回落类型。取值：
    /// - **SMS**：不支持卡片短信的号码，回落文本短信。
    /// - **DIGITALSMS**：不支持卡片短信的号码，回落数字短信。
    /// - **NONE**：不需要回落。
    fallback_type: String,
    /// 回落数字短信的模板Code。**FallbackType**选择**DIGITALSMS**回落数字短信时，此参数必填。
    ///
    /// 您可在控制台**数字短信**[模板管理](https://dysms.console.aliyun.com/domestic/digit)页面查看数字短信模板列表。
    ///
    /// >必须是已添加、并通过审核的数字短信模板。
    #[setters(generate = true, strip_option)]
    digital_template_code: Option<String>,
    /// 预留给调用方使用的ID。
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
    /// 回落文本短信的模板变量对应的实际值。**SmsTemplateCode**回落的文本短信模板内含有变量时，此参数必填。
    ///
    /// >如果JSON中需要带换行符，请参照标准的JSON协议处理。
    #[setters(generate = true, strip_option)]
    sms_template_param: Option<String>,
    /// 回落数字短信的模板变量对应的实际值。**DigitalTemplateCode**回落的数字短信模板内含有变量时，此参数必填。
    ///
    /// >如果JSON中需要带换行符，请参照标准的JSON协议处理。
    #[setters(generate = true, strip_option)]
    digital_template_param: Option<String>,
    /// 自定义发送内容模板CODE。
    ///
    /// 自定义内容会按照选定的文本短信模板+卡片解析链接的形式下发到终端，您可以登录[短信服务控制台](https://dysms.console.aliyun.com/overview)，选择**国内消息**或**国际/港澳台短信**，在**模板管理**页签下查看**模板CODE**。
    ///
    /// > - 必须是已添加、并通过审核的模板CODE；且发送国际/港澳台消息时，请使用国际/港澳台短信模版。
    /// > - 例如：选择的文本短信模板内容为：您有一条消息待查收；卡片解析链接为：`1*.cn/2**d`。最终下发内容：`您有一条消息待查收1*.cn/2**d`。请在下发前做好测试和字数控制。
    #[setters(generate = true, strip_option)]
    template_code: Option<String>,
    /// 自定义发送内容模板变量对应的实际值。**TemplateCode**填入包含变量的短信模板时，此参数必填。
    ///
    /// >如果JSON中需要带换行符，请参照标准的JSON协议处理。
    #[setters(generate = true, strip_option)]
    template_param: Option<String>,
}

impl sealed::Bound for SendCardSms {}

impl SendCardSms {
    pub fn new(
        card_objects: impl Into<Vec<CardObject>>,
        sign_name: impl Into<String>,
        card_template_code: impl Into<String>,
        fallback_type: impl Into<String>,
    ) -> Self {
        Self {
            card_objects: card_objects.into(),
            sign_name: sign_name.into(),
            card_template_code: card_template_code.into(),
            sms_template_code: None,
            sms_up_extend_code: None,
            fallback_type: fallback_type.into(),
            digital_template_code: None,
            out_id: None,
            sms_template_param: None,
            digital_template_param: None,
            template_code: None,
            template_param: None,
        }
    }
}
impl crate::ToFormData for SendCardSms {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for SendCardSms {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SendCardSms";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<SendCardSmsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(12);
        crate::FlatSerialize::flat_serialize(&self.card_objects, "CardObjects", &mut params);

        params.push(("CardTemplateCode".into(), (&self.card_template_code).into()));

        if let Some(f) = &self.digital_template_code {
            params.push(("DigitalTemplateCode".into(), (f).into()));
        }

        if let Some(f) = &self.digital_template_param {
            params.push(("DigitalTemplateParam".into(), (f).into()));
        }
        params.push(("FallbackType".into(), (&self.fallback_type).into()));

        if let Some(f) = &self.out_id {
            params.push(("OutId".into(), (f).into()));
        }
        params.push(("SignName".into(), (&self.sign_name).into()));

        if let Some(f) = &self.sms_template_code {
            params.push(("SmsTemplateCode".into(), (f).into()));
        }

        if let Some(f) = &self.sms_template_param {
            params.push(("SmsTemplateParam".into(), (f).into()));
        }

        if let Some(f) = &self.sms_up_extend_code {
            params.push(("SmsUpExtendCode".into(), (f).into()));
        }

        if let Some(f) = &self.template_code {
            params.push(("TemplateCode".into(), (f).into()));
        }

        if let Some(f) = &self.template_param {
            params.push(("TemplateParam".into(), (f).into()));
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
pub struct SendBatchCardSms {
    /// 卡片短信模板Code。请在控制台**卡片短信**[模板管理](https://dysms.console.aliyun.com/domestic/card)页面选择**已通过审核**的卡片短信模板Code。
    card_template_code: String,
    /// 回落文本短信的模板Code。**FallbackType**选择**SMS**回落文本短信时，此参数必填。
    ///
    /// 您可以通过[QuerySmsTemplateList](~~419288~~)接口查询当前账号已申请的模板或在[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/template)查看模板列表。
    /// >必须是已添加、并通过审核的短信模板。
    #[setters(generate = true, strip_option)]
    sms_template_code: Option<String>,
    /// 回落类型。取值：
    /// - **SMS**：不支持卡片短信的号码，回落文本短信。
    /// - **DIGITALSMS**：不支持卡片短信的号码，回落数字短信。
    /// - **NONE**：不需要回落。
    fallback_type: String,
    /// 回落数字短信的模板Code。**FallbackType**选择**DIGITALSMS**回落数字短信时，此参数必填。
    ///
    /// 您可在控制台**国内数字短信**[模板管理](https://dysms.console.aliyun.com/domestic/digit)页面查看数字短信模板列表。
    /// >必须是已添加、并通过审核的数字短信模板。
    #[setters(generate = true, strip_option)]
    digital_template_code: Option<String>,
    /// 预留给调用方使用的ID。
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
    /// 接收短信的手机号码。
    phone_number_json: String,
    /// 短信签名名称。
    /// 您可以通过[QuerySmsSignList](~~419282~~)接口查询当前账号已申请的签名或在[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/sign)查看签名列表。
    /// >必须是已添加、并通过审核的短信签名；且短信签名的个数必须与手机号码的个数相同、内容一一对应。
    sign_name_json: String,
    /// 卡片短信模板参数对应的实际值。**CardTemplateCode**卡片短信模板内含有变量时，此参数必填。
    ///
    /// >如果JSON中需要带换行符，请参照标准的JSON协议处理。
    #[setters(generate = true, strip_option)]
    card_template_param_json: Option<String>,
    /// 文本短信模板参数对应的实际值。**SmsTemplateCode**回落的文本短信模板内含有变量时，此参数必填。
    ///
    /// >如果JSON中需要带换行符，请参照标准的JSON协议处理。
    #[setters(generate = true, strip_option)]
    sms_template_param_json: Option<String>,
    /// 数字短信模板参数对应的实际值。**DigitalTemplateCode**回落的数字短信模板内含有变量时，此参数必填。
    /// >如果JSON中需要带换行符，请参照标准的JSON协议处理。
    #[setters(generate = true, strip_option)]
    digital_template_param_json: Option<String>,
    /// 上行短信扩展码。
    #[setters(generate = true, strip_option)]
    sms_up_extend_code_json: Option<String>,
    /// 自定义发送内容模板CODE。
    ///
    /// 自定义内容会按照选定的文本短信模板+卡片解析链接的形式下发到终端，您可以登录[短信服务控制台](https://dysms.console.aliyun.com/overview)，选择**国内消息**或**国际/港澳台短信**，在**模板管理**页面查看**模板CODE**。
    ///
    /// > - 必须是已添加、并通过审核的短信模板；且发送国际/港澳台消息时，请使用国际/港澳台短信模板。
    /// > - 例如：选择的文本短信模板内容为：您有一条消息待查收；卡片解析链接为：`1*.cn/2**d`。最终下发内容：`您有一条消息待查收1*.cn/2**d`。请在下发前做好测试和字数控制。
    #[setters(generate = true, strip_option)]
    template_code: Option<String>,
    /// 自定义发送内容模板变量对应的实际值。**TemplateCode**填入包含变量的短信模板时，此参数必填。
    ///
    /// > - 如果JSON中需要带换行符，请参照标准的JSON协议处理；
    /// > - 模板变量值的个数必须与手机号码、签名的个数相同、内容一一对应，表示向指定手机号码中发对应签名的短信，且短信模板中的变量参数替换为对应的值。
    #[setters(generate = true, strip_option)]
    template_param_json: Option<String>,
}

impl sealed::Bound for SendBatchCardSms {}

impl SendBatchCardSms {
    pub fn new(
        card_template_code: impl Into<String>,
        fallback_type: impl Into<String>,
        phone_number_json: impl Into<String>,
        sign_name_json: impl Into<String>,
    ) -> Self {
        Self {
            card_template_code: card_template_code.into(),
            sms_template_code: None,
            fallback_type: fallback_type.into(),
            digital_template_code: None,
            out_id: None,
            phone_number_json: phone_number_json.into(),
            sign_name_json: sign_name_json.into(),
            card_template_param_json: None,
            sms_template_param_json: None,
            digital_template_param_json: None,
            sms_up_extend_code_json: None,
            template_code: None,
            template_param_json: None,
        }
    }
}
impl crate::ToFormData for SendBatchCardSms {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for SendBatchCardSms {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SendBatchCardSms";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<SendBatchCardSmsResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(13);
        params.push(("CardTemplateCode".into(), (&self.card_template_code).into()));

        if let Some(f) = &self.card_template_param_json {
            params.push(("CardTemplateParamJson".into(), (f).into()));
        }

        if let Some(f) = &self.digital_template_code {
            params.push(("DigitalTemplateCode".into(), (f).into()));
        }

        if let Some(f) = &self.digital_template_param_json {
            params.push(("DigitalTemplateParamJson".into(), (f).into()));
        }
        params.push(("FallbackType".into(), (&self.fallback_type).into()));

        if let Some(f) = &self.out_id {
            params.push(("OutId".into(), (f).into()));
        }
        params.push(("PhoneNumberJson".into(), (&self.phone_number_json).into()));
        params.push(("SignNameJson".into(), (&self.sign_name_json).into()));

        if let Some(f) = &self.sms_template_code {
            params.push(("SmsTemplateCode".into(), (f).into()));
        }

        if let Some(f) = &self.sms_template_param_json {
            params.push(("SmsTemplateParamJson".into(), (f).into()));
        }

        if let Some(f) = &self.sms_up_extend_code_json {
            params.push(("SmsUpExtendCodeJson".into(), (f).into()));
        }

        if let Some(f) = &self.template_code {
            params.push(("TemplateCode".into(), (f).into()));
        }

        if let Some(f) = &self.template_param_json {
            params.push(("TemplateParamJson".into(), (f).into()));
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
pub struct GetQualificationOssInfo {
    /// 业务类型。申请资质或创建授权委托书文件时请填写**dysms**。
    biz_type: String,
}

impl sealed::Bound for GetQualificationOssInfo {}

impl GetQualificationOssInfo {
    pub fn new(biz_type: impl Into<String>) -> Self {
        Self {
            biz_type: biz_type.into(),
        }
    }
}
impl crate::ToFormData for GetQualificationOssInfo {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for GetQualificationOssInfo {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "GetQualificationOssInfo";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<GetQualificationOssInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("BizType".into(), (&self.biz_type).into()));

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
pub struct GetOSSInfoForUploadFile {
    /// 业务类型，默认值为**fcMediaSms**。
    ///
    /// 创建签名、创建模板时，上传**更多资料**场景，该值为**fcMediaSms**。
    #[setters(generate = true, strip_option)]
    biz_type: Option<String>,
}

impl sealed::Bound for GetOSSInfoForUploadFile {}

impl GetOSSInfoForUploadFile {
    pub fn new() -> Self {
        Self { biz_type: None }
    }
}
impl crate::ToFormData for GetOSSInfoForUploadFile {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for GetOSSInfoForUploadFile {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "GetOSSInfoForUploadFile";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<GetOSSInfoForUploadFileResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.biz_type {
            params.push(("BizType".into(), (f).into()));
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
pub struct GetSmsOcrOssInfo {
    /// OCR任务类型。可选值：
    /// - ICP_DOMAIN（域名类型链接ICP备案截图）
    #[setters(generate = true, strip_option)]
    task_type: Option<String>,
}

impl sealed::Bound for GetSmsOcrOssInfo {}

impl GetSmsOcrOssInfo {
    pub fn new() -> Self {
        Self { task_type: None }
    }
}
impl crate::ToFormData for GetSmsOcrOssInfo {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for GetSmsOcrOssInfo {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "GetSmsOcrOssInfo";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<GetSmsOcrOssInfoResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);

        if let Some(f) = &self.task_type {
            params.push(("TaskType".into(), (f).into()));
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
pub struct SmsConversionIntl {
    /// 消息ID。
    message_id: String,
    /// 如果您的用户回复了您发送的消息，则设置为 true。否则，设置为 false。
    delivered: bool,
    /// 触达发送目标的时间戳。必须是Unix时间戳，毫秒级别长整型。
    ///
    /// - 如果不指定该字段：默认当前的时间戳。
    ///
    /// - 如果指定该字段：该时间戳必须大于发送时间并且小于当前时间戳。
    #[setters(generate = true, strip_option)]
    conversion_time: Option<i64>,
}

impl sealed::Bound for SmsConversionIntl {}

impl SmsConversionIntl {
    pub fn new(message_id: impl Into<String>, delivered: impl Into<bool>) -> Self {
        Self {
            message_id: message_id.into(),
            delivered: delivered.into(),
            conversion_time: None,
        }
    }
}
impl crate::ToFormData for SmsConversionIntl {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for SmsConversionIntl {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "SmsConversionIntl";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<SmsConversionIntlResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);

        if let Some(f) = &self.conversion_time {
            params.push(("ConversionTime".into(), (f).into()));
        }
        params.push(("Delivered".into(), (&self.delivered).into()));
        params.push(("MessageId".into(), (&self.message_id).into()));

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
pub struct ConversionDataIntl {
    /// 转化率观测的时间点。必须是Unix时间戳，毫秒级别长整型。
    ///
    /// >如果不指定该字段：默认当前的时间戳。
    #[setters(generate = true, strip_option)]
    report_time: Option<i64>,
    /// 转化率监控回报值。
    /// >该参数取值为double类型 ，区间是\[0,1]。
    conversion_rate: String,
}

impl sealed::Bound for ConversionDataIntl {}

impl ConversionDataIntl {
    pub fn new(conversion_rate: impl Into<String>) -> Self {
        Self {
            report_time: None,
            conversion_rate: conversion_rate.into(),
        }
    }
}
impl crate::ToFormData for ConversionDataIntl {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for ConversionDataIntl {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "ConversionDataIntl";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<ConversionDataIntlResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(2);
        params.push(("ConversionRate".into(), (&self.conversion_rate).into()));

        if let Some(f) = &self.report_time {
            params.push(("ReportTime".into(), (f).into()));
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

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AddShortUrl {
    /// 原始链接地址。不超过1000个字符。
    /// ><notice>短信服务暂时不支持使用此接口。></notice>
    source_url: String,
    /// 短链服务名称。不超过13个字符。
    short_url_name: String,
    /// 短链服务使用有效期。单位为天，有效期最长为90天。
    effective_days: String,
}

impl sealed::Bound for AddShortUrl {}

impl AddShortUrl {
    pub fn new(
        source_url: impl Into<String>,
        short_url_name: impl Into<String>,
        effective_days: impl Into<String>,
    ) -> Self {
        Self {
            source_url: source_url.into(),
            short_url_name: short_url_name.into(),
            effective_days: effective_days.into(),
        }
    }
}
impl crate::ToFormData for AddShortUrl {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(3);
        params.push(("EffectiveDays".into(), (&self.effective_days).into()));
        params.push(("ShortUrlName".into(), (&self.short_url_name).into()));
        params.push(("SourceUrl".into(), (&self.source_url).into()));

        params
    }
}

impl crate::Request for AddShortUrl {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "AddShortUrl";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<AddShortUrlResponse>;

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

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteShortUrl {
    /// 原始链接地址。不超过1000个字符。
    /// ><notice>短信服务暂时不支持使用此接口。></notice>
    source_url: String,
}

impl sealed::Bound for DeleteShortUrl {}

impl DeleteShortUrl {
    pub fn new(source_url: impl Into<String>) -> Self {
        Self {
            source_url: source_url.into(),
        }
    }
}
impl crate::ToFormData for DeleteShortUrl {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("SourceUrl".into(), (&self.source_url).into()));

        params
    }
}

impl crate::Request for DeleteShortUrl {
    const METHOD: http::Method = http::Method::DELETE;

    const ACTION: &'static str = "DeleteShortUrl";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<DeleteShortUrlResponse>;

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

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QueryShortUrl {
    /// 生成的短链服务地址。可通过[AddShortUrl](~~186774~~)接口获取。
    /// ><notice>短信服务暂时不支持使用此接口。></notice>
    short_url: String,
}

impl sealed::Bound for QueryShortUrl {}

impl QueryShortUrl {
    pub fn new(short_url: impl Into<String>) -> Self {
        Self {
            short_url: short_url.into(),
        }
    }
}
impl crate::ToFormData for QueryShortUrl {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(1);
        params.push(("ShortUrl".into(), (&self.short_url).into()));

        params
    }
}

impl crate::Request for QueryShortUrl {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "QueryShortUrl";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<QueryShortUrlResponse>;

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

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListTagResources {
    /// 资源类型，默认填写TEMPLATE。
    resource_type: String,
    /// 地域ID。默认取值：**cn-hangzhou**。
    region_id: String,
    /// 查询下一页标签的Token。
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    /// 每页显示条数。
    #[setters(generate = true, strip_option)]
    page_size: Option<i32>,
    /// 产品名。默认取值：**dysms**。
    #[setters(generate = true, strip_option)]
    prod_code: Option<String>,
    /// 标签列表。标签列表和**ResourceId**(短信模板Code)不能同时为空。最多可输入20个标签。
    #[setters(generate = true, strip_option)]
    tag: Option<Vec<ListTagResourcesTag>>,
    /// 短信模板Code。短信模板Code和标签列表**Tag**不能同时为空。
    #[setters(generate = true, strip_option)]
    resource_id: Option<Vec<String>>,
}

impl sealed::Bound for ListTagResources {}

impl ListTagResources {
    pub fn new(resource_type: impl Into<String>, region_id: impl Into<String>) -> Self {
        Self {
            resource_type: resource_type.into(),
            region_id: region_id.into(),
            next_token: None,
            page_size: None,
            prod_code: None,
            tag: None,
            resource_id: None,
        }
    }
}
impl crate::ToFormData for ListTagResources {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for ListTagResources {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "ListTagResources";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<ListTagResourcesResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(7);

        if let Some(f) = &self.next_token {
            params.push(("NextToken".into(), (f).into()));
        }

        if let Some(f) = &self.page_size {
            params.push(("PageSize".into(), (f).into()));
        }

        if let Some(f) = &self.prod_code {
            params.push(("ProdCode".into(), (f).into()));
        }
        params.push(("RegionId".into(), (&self.region_id).into()));

        if let Some(f) = &self.resource_id {
            crate::FlatSerialize::flat_serialize(f, "ResourceId", &mut params);
        }
        params.push(("ResourceType".into(), (&self.resource_type).into()));

        if let Some(f) = &self.tag {
            crate::FlatSerialize::flat_serialize(f, "Tag", &mut params);
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

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct TagResources {
    /// 资源类型，默认取值：**TEMPLATE**。
    resource_type: String,
    /// 地域ID，默认取值：**cn-hangzhou**。更多地域ID请参见[服务接入点](~~419270~~)。
    region_id: String,
    /// 产品名。默认取值：**dysms**。
    #[setters(generate = true, strip_option)]
    prod_code: Option<String>,
    /// 标签。单次添加的标签数量不超过20。
    tag: Vec<TagResourcesTag>,
    /// 短信模板Code。数量不能超过20条。
    #[setters(generate = true, strip_option)]
    resource_id: Option<Vec<String>>,
}

impl sealed::Bound for TagResources {}

impl TagResources {
    pub fn new(
        resource_type: impl Into<String>,
        region_id: impl Into<String>,
        tag: impl Into<Vec<TagResourcesTag>>,
    ) -> Self {
        Self {
            resource_type: resource_type.into(),
            region_id: region_id.into(),
            prod_code: None,
            tag: tag.into(),
            resource_id: None,
        }
    }
}
impl crate::ToFormData for TagResources {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for TagResources {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "TagResources";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<TagResourcesResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(5);

        if let Some(f) = &self.prod_code {
            params.push(("ProdCode".into(), (f).into()));
        }
        params.push(("RegionId".into(), (&self.region_id).into()));

        if let Some(f) = &self.resource_id {
            crate::FlatSerialize::flat_serialize(f, "ResourceId", &mut params);
        }
        params.push(("ResourceType".into(), (&self.resource_type).into()));
        crate::FlatSerialize::flat_serialize(&self.tag, "Tag", &mut params);

        params
    }

    fn to_headers(&self) -> Vec<(std::borrow::Cow<'static, str>, String)> {
        Default::default()
    }

    fn to_body(self) -> Self::Body {
        crate::Form(self)
    }
}

///  
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UntagResources {
    /// 资源类型，默认填写TEMPLATE。
    resource_type: String,
    /// 地域ID，默认填写cn-hangzhou。
    /// 更多地域ID请参见[服务接入点](~~419270~~)。
    region_id: String,
    /// 是否删除该模板下的所有标签。取值：
    ///
    /// - **true**：是。
    /// - **false**：否。
    #[setters(generate = true, strip_option)]
    all: Option<bool>,
    /// 产品名。默认取值：**dysms**。
    #[setters(generate = true, strip_option)]
    prod_code: Option<String>,
    /// 标签键。单次添加的标签数量不超过20。
    #[setters(generate = true, strip_option)]
    tag_key: Option<Vec<String>>,
    /// 短信模板Code。数量不能超过20条。
    #[setters(generate = true, strip_option)]
    resource_id: Option<Vec<String>>,
}

impl sealed::Bound for UntagResources {}

impl UntagResources {
    pub fn new(resource_type: impl Into<String>, region_id: impl Into<String>) -> Self {
        Self {
            resource_type: resource_type.into(),
            region_id: region_id.into(),
            all: None,
            prod_code: None,
            tag_key: None,
            resource_id: None,
        }
    }
}
impl crate::ToFormData for UntagResources {
    fn to_form_data(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        Default::default()
    }
}

impl crate::Request for UntagResources {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "UntagResources";

    type Body = crate::Form<Self>;

    type ResponseWrap = crate::JsonResponseWrap<UntagResourcesResponse>;

    fn to_query_params(&self) -> Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'_>)> {
        let mut params = Vec::with_capacity(6);

        if let Some(f) = &self.all {
            params.push(("All".into(), (f).into()));
        }

        if let Some(f) = &self.prod_code {
            params.push(("ProdCode".into(), (f).into()));
        }
        params.push(("RegionId".into(), (&self.region_id).into()));

        if let Some(f) = &self.resource_id {
            crate::FlatSerialize::flat_serialize(f, "ResourceId", &mut params);
        }
        params.push(("ResourceType".into(), (&self.resource_type).into()));

        if let Some(f) = &self.tag_key {
            crate::FlatSerialize::flat_serialize(f, "TagKey", &mut params);
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

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct SubmitSmsQualificationBusinessLicensePic {
    /// 营业证件类型。取值：
    ///
    /// - socialCreditLicense：社会信用代码证书。
    /// - businessLicense：企业营业执照。
    /// - signLegalLicense：事业单位法人证书。
    /// - otherLicense：其他。
    ///
    /// 选择一种上传即可，证件上需含有：企业名称、统一社会信用代码、证件有效期。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 营业证件图片，仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    ///
    ///
    /// ><notice>
    ///
    /// 证件的彩色原件无需盖章，若上传复印件/黑白照片，需要在复印件上加盖企业红章并拍照上传。
    /// ></notice>
    #[serde(rename = "LicensePic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_pic: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct SubmitSmsQualificationOtherFile {
    /// 更多资料文件，仅支持png、jpg、jpeg、doc、docx、pdf格式，文件不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    #[serde(rename = "LicensePic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_pic: Option<String>,
}

/// 单个资质审核详情。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DataList {
    /// 法人姓名。
    #[serde(rename = "LegalPersonName")]
    #[serde(default)]
    pub legal_person_name: String,
    /// 审核备注。
    #[serde(rename = "AuditRemark")]
    #[serde(default)]
    pub audit_remark: String,
    /// 企业名称。
    #[serde(rename = "CompanyName")]
    #[serde(default)]
    pub company_name: String,
    /// 审核工单ID。
    #[serde(rename = "WorkOrderId")]
    #[serde(default)]
    pub work_order_id: i64,
    /// 审核状态。取值：
    ///
    /// - INIT：审核中。
    /// - NOT_PASS：审核不通过。
    /// - PASS：审核通过。
    /// - NOT_FINISH：资料待补充。
    /// - CANCEL：已撤回。
    #[serde(rename = "StateName")]
    #[serde(default)]
    pub state_name: String,
    /// 审核时间。
    #[serde(rename = "AuditTime")]
    #[serde(default)]
    pub audit_time: String,
    /// 资质创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    pub create_date: String,
    /// 资质名称。
    #[serde(rename = "QualificationGroupName")]
    #[serde(default)]
    pub qualification_group_name: String,
    /// 资质ID。
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: i64,
    /// 资质申请用途，取值：
    ///
    /// - **true**：自用。
    /// - **false**：他用。
    #[serde(rename = "UseBySelf")]
    #[serde(default)]
    pub use_by_self: String,
}

/// 资质审核列表
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct QualificationRecordResponseData {
    /// 每页数据条数。
    #[serde(rename = "PageSize")]
    #[serde(default)]
    pub page_size: i64,
    /// 总条数。
    #[serde(rename = "Total")]
    #[serde(default)]
    pub total: i64,
    /// 当前页码。
    #[serde(rename = "PageNo")]
    #[serde(default)]
    pub page_no: i64,
    /// 满足过滤条件的数据列表。
    #[serde(rename = "List")]
    #[serde(default)]
    pub list: Vec<DataList>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DataBusinessLicensePic {
    /// 营业证件类型。取值：
    ///
    /// - socialCreditLicense：社会信用代码证书。
    /// - businessLicense：企业营业执照。
    /// - signLegalLicense：事业单位法人证书。
    /// - otherLicense：其他类型执照证书。
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    /// 营业证件文件路径参数。
    #[serde(rename = "LicensePic")]
    #[serde(default)]
    pub license_pic: String,
    /// 营业证件文件完整路径URL。
    #[serde(rename = "PicUrl")]
    #[serde(default)]
    pub pic_url: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DataOtherFile {
    /// 更多资料文件路径参数。
    #[serde(rename = "LicensePic")]
    #[serde(default)]
    pub license_pic: String,
    /// 更多资料文件完整路径URL。
    #[serde(rename = "PicUrl")]
    #[serde(default)]
    pub pic_url: String,
}

/// 单个资质详情。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct SmsQualificationResponseData {
    /// 管理员证件有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    #[serde(rename = "AdminIDCardExpDate")]
    #[serde(default)]
    pub admin_id_card_exp_date: String,
    /// 行业类型。
    #[serde(rename = "BusinessType")]
    #[serde(default)]
    pub business_type: String,
    /// 管理员证件正面（身份证国徽面）照片完整路径URL。
    #[serde(rename = "AdminIDCardFrontFace")]
    #[serde(default)]
    pub admin_id_card_front_face: String,
    /// 备注。
    #[serde(rename = "Remark")]
    #[serde(default)]
    pub remark: String,
    /// 法人姓名。
    #[serde(rename = "LegalPersonName")]
    #[serde(default)]
    pub legal_person_name: String,
    /// 企业名称。
    #[serde(rename = "CompanyName")]
    #[serde(default)]
    pub company_name: String,
    /// 法人证件号码。
    #[serde(rename = "LegalPersonIDCardNo")]
    #[serde(default)]
    pub legal_person_id_card_no: String,
    /// 企业营业证件信息。
    #[serde(rename = "BusinessLicensePics")]
    #[serde(default)]
    pub business_license_pics: Vec<DataBusinessLicensePic>,
    /// 管理员证件号码。
    #[serde(rename = "AdminIDCardNo")]
    #[serde(default)]
    pub admin_id_card_no: String,
    /// 社会统一信用代码。
    #[serde(rename = "OrganizationCode")]
    #[serde(default)]
    pub organization_code: String,
    /// 法人证件有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    #[serde(rename = "LegalPersonIdCardEffTime")]
    #[serde(default)]
    pub legal_person_id_card_eff_time: String,
    /// 管理员手机号码。
    #[serde(rename = "AdminPhoneNo")]
    #[serde(default)]
    pub admin_phone_no: String,
    /// 资质名称。
    #[serde(rename = "QualificationName")]
    #[serde(default)]
    pub qualification_name: String,
    /// 管理员姓名。
    #[serde(rename = "AdminName")]
    #[serde(default)]
    pub admin_name: String,
    /// 企业类型，取值：
    ///
    /// - COMPANY：企业。
    /// - NON_PROFIT_ORGANIZATION：政府机关或事业单位。
    #[serde(rename = "CompanyType")]
    #[serde(default)]
    pub company_type: String,
    /// 管理员证件类型。取值：
    ///
    /// - identityCard：身份证。
    /// - passport：护照。
    /// - homeReturnPermit：港澳居民来往内地通行证。
    /// - TaiwanCompatriotPermit：台湾居民来往大陆通行证。
    /// - residencePermit：港澳台居民居住证。
    /// - other：其他。
    #[serde(rename = "AdminIDCardType")]
    #[serde(default)]
    pub admin_id_card_type: String,
    /// 资质申请用途，取值：
    ///
    /// - **true**：自用。
    /// - **false**：他用。
    #[serde(rename = "UseBySelf")]
    #[serde(default)]
    pub use_by_self: bool,
    /// 营业证照有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    #[serde(rename = "EffTimeStr")]
    #[serde(default)]
    pub eff_time_str: String,
    /// 资质ID。
    #[serde(rename = "QualificationGroupId")]
    #[serde(default)]
    pub qualification_group_id: i64,
    /// 资质授权，是否同意与其他云通信产品（如国内语音、国内号码隐私保护）的资质共享。仅当您申请**自用资质**，且资质信息**与当前阿里云账号认证企业信息一致**时可被共享、复用；其他情况无效。取值：
    ///
    /// - true：同意，您的资质信息可在其他云通信产品的“资质认证环节”调用，免除重复认证环节。
    /// - false：不同意。
    #[serde(rename = "WhetherShare")]
    #[serde(default)]
    pub whether_share: bool,
    /// 审核工单ID。
    #[serde(rename = "WorkOrderId")]
    #[serde(default)]
    pub work_order_id: i64,
    /// 审核状态。取值：
    ///
    /// - INT：审核中。
    /// - FAILED：审核不通过。
    /// - PASSED：审核通过。
    /// - NOT_FINISH：资料待补充。
    /// - CANCELED：已撤回。
    ///
    /// > 本接口不会返回审核备注，如需要查询审核备注（`AuditRemark`）请使用[查询资质列表](~~QuerySmsQualificationRecord~~)。
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
    /// 更多资料。
    #[serde(rename = "OtherFiles")]
    #[serde(default)]
    pub other_files: Vec<DataOtherFile>,
    /// 法人证件类型。取值：
    ///
    /// - identityCard：身份证。
    /// - passport：护照。
    /// - homeReturnPermit：港澳居民来往内地通行证。
    /// - TaiwanCompatriotPermit：台湾居民来往大陆通行证。
    /// - residencePermit：港澳台居民居住证。
    /// - other：其他。
    #[serde(rename = "LegalPersonIDCardType")]
    #[serde(default)]
    pub legal_person_id_card_type: String,
    /// 管理员证件反面（身份证人像面）图片完整路径URL。
    #[serde(rename = "AdminIDCardPic")]
    #[serde(default)]
    pub admin_id_card_pic: String,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UpdateSmsQualificationBusinessLicensePic {
    /// 营业证件类型。取值：
    ///
    /// - socialCreditLicense：社会信用代码证书。
    /// - businessLicense：企业营业执照。
    /// - signLegalLicense：事业单位法人证书。
    /// - otherLicense：其他。
    ///
    /// 选择一种上传即可，证件上需含有：企业名称、统一社会信用代码、证件有效期。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 营业证件图片，仅支持jpg、png、gif、jpeg格式的图片，图片不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    ///
    /// ><notice>
    /// 证件的彩色原件无需盖章，若上传复印件/黑白照片，需要在复印件上加盖企业红章并拍照上传。
    /// ></notice>
    #[serde(rename = "LicensePic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_pic: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UpdateSmsQualificationOtherFile {
    /// 更多资料文件，仅支持png、jpg、jpeg、doc、docx、pdf格式，文件不大于5MB。请填写上传到OSS的文件路径参数，待上传的文件命名不可包含中文和特殊字符，上传操作请参见通过[OSS上传文件](~~2833114~~)。
    #[serde(rename = "LicensePic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_pic: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct LetterResponseData {
    /// 委托授权方。
    #[serde(rename = "Authorization")]
    #[serde(default)]
    pub authorization: String,
    /// 委托授权书可用状态，与授权书有效期相关，取值：
    ///
    /// - **VALID**：可用，授权书处于有效期内。
    /// - **INVALID**：不可用，授权书已过期。
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    /// 委托授权签名范围。若有多个签名，签名之间使用顿号`、`分隔。
    #[serde(rename = "SignScope")]
    #[serde(default)]
    pub sign_scope: String,
    /// 委托授权书状态，与签名审核状态相关，取值：
    /// - **INT**：待审核。委托授权书已创建，当您提交签名申请后进入审核流程。
    /// - **PASSED**：审核通过。当您的委托授权签名范围中有签名审核通过时，委托授权书状态变为PASSED。
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
    /// 委托授权书有效期。有效期格式：YYYY-MM-DD~YYYY-MM-DD。
    #[serde(rename = "AuthorizationLetterExpDate")]
    #[serde(default)]
    pub authorization_letter_exp_date: String,
    /// 委托授权书文件地址。
    #[serde(rename = "AuthorizationLetterPic")]
    #[serde(default)]
    pub authorization_letter_pic: String,
    /// 委托授权方社会统一信用代码。
    #[serde(rename = "OrganizationCode")]
    #[serde(default)]
    pub organization_code: String,
    /// 被委托授权方。
    #[serde(rename = "ProxyAuthorization")]
    #[serde(default)]
    pub proxy_authorization: String,
    /// 委托授权书ID。
    #[serde(rename = "AuthorizationLetterId")]
    #[serde(default)]
    pub authorization_letter_id: i64,
    /// 委托授权书命名。
    #[serde(rename = "AuthorizationLetterName")]
    #[serde(default)]
    pub authorization_letter_name: String,
}

/// 审核信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct SignResponseAuditInfo {
    /// 审批未通过的原因。
    #[serde(rename = "RejectInfo")]
    #[serde(default)]
    pub reject_info: String,
    /// 审核时间。
    #[serde(rename = "AuditDate")]
    #[serde(default)]
    pub audit_date: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct StatusReason {
    /// 报备状态原因码。取值：
    /// - **UNBINDING_QUA**：签名未关联资质；
    /// - **BINDING_INCOMPLETE_QUA**：关联资质信息不全；
    /// - **NON_REGISTER**：未发起报备；
    /// - **REGISTERING**：签名报备中；
    /// - **DETECTING**：未发起探测或探测中；
    /// - **DETECT_SUCCESS**：报备成功；
    /// - **QUALIFICATION_ERROR**：资质原因；
    /// - **SIGNATURE_ERROR**：签名原因；
    /// - **SIGNATURE_QUALIFICATION_ERROR**：签名与资质关系不符；
    /// - **ONE_CODE_MULTIPLE_SIGN**：扩展码原因；
    /// - **OTHERS_ERROR**：其他原因；
    /// - **REGISTER_TIMEOUT**：报备超时；
    /// - **NO_SEND_RECORD**：签名超过6个月无发送记录；
    /// - **EXT_CODE_RECYCLE**：扩展码收回。
    /// - **SUBPORT_RECYCLE**：子端口被运营商治理。
    #[serde(rename = "ReasonCode")]
    #[serde(default)]
    pub reason_code: String,
    /// 原因说明列表。可能返回0个或者多个原因说明，返回原因码不一定会返回原因说明。
    #[serde(rename = "ReasonDescList")]
    #[serde(default)]
    pub reason_desc_list: Vec<String>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DetailList {
    /// 报备状态。取值：
    ///
    /// - **0**：报备失败，原因可能为资质信息与工信注册信息不一致或运营商侧无法支持等。建议您登录[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/sign)查看具体失败原因，并依据提示进行操作；
    /// - **1**：已报备待验证，当前至少有一个子端口号运营商已返回报备通过，建议您少量多次向不同运营商手机号发送验证码、通知短信进行验证；
    /// - **2**：报备失效，签名超过 6 个月无发送记录时，报备结果失效。如您需要重新启用该签名，请在[短信服务控制台](https://dysms.console.aliyun.com/domestic/text/sign)重新发起报备；
    /// - **3**：报备成功，当前至少有一个子端口号运营商已返回报备通过，经验证短信发送成功率符合预期，建议您持续关注发送成功率；
    /// - **-1**：报备中，当前尚未收到运营商反馈的报备结果，建议您等待签名报备状态变更为“已报备待验证”后再批量发送，当前可少量多次尝试使用该签名发送，观察短信发送效果；
    /// - **-2**：未报备，原因可能为当前签名未关联实名资质或关联资质信息不全，建议您修改当前资质或编辑签名绑定其他资质以重新发起报备。
    ///
    /// 建议您单击查看[更多签名实名制报备内容及建议操作](~~2873145~~)。
    #[serde(rename = "RegisterStatus")]
    #[serde(default)]
    pub register_status: i32,
    /// 运营商类型。取值：
    /// - **mobile**：中国移动；
    /// - **unicom**：中国联通；
    /// - **telecom**：中国电信。
    #[serde(rename = "OperatorCode")]
    #[serde(default)]
    pub operator_code: String,
    /// 运营商反馈时间，格式为yyyy-MM-dd HH:mm:ss。
    #[serde(rename = "OperatorCompleteTime")]
    #[serde(default)]
    pub operator_complete_time: String,
    /// 报备状态原因列表。
    #[serde(rename = "RegisterStatusReasons")]
    #[serde(default)]
    pub register_status_reasons: Vec<StatusReason>,
}

/// 审核备注。
///
/// - 如果审核状态为**审核通过**或**审核中**，参数Reason显示为“无审核备注”。
///
/// - 如果审核状态为**审核未通过**，参数Reason显示审核的具体原因。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct SignListItemReason {
    /// 审批未通过的备注信息。
    #[serde(rename = "RejectSubInfo")]
    #[serde(default)]
    pub reject_sub_info: String,
    /// 审批未通过的时间，格式为yyyy-MM-dd HH:mm:ss。
    #[serde(rename = "RejectDate")]
    #[serde(default)]
    pub reject_date: String,
    /// 审批未通过的原因。
    #[serde(rename = "RejectInfo")]
    #[serde(default)]
    pub reject_info: String,
}

/// 结果列表。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct SignList {
    /// 签名名称。
    #[serde(rename = "SignName")]
    #[serde(default)]
    pub sign_name: String,
    /// 签名审批状态。取值：
    ///
    /// - **AUDIT_STATE_INIT**：审核中。
    ///
    /// - **AUDIT_STATE_PASS**：审核通过。
    ///
    /// - **AUDIT_STATE_NOT_PASS**：审核未通过，请在返回参数Reason中查看审核未通过原因。
    ///
    /// - **AUDIT_STATE_CANCEL**：取消审核。
    #[serde(rename = "AuditStatus")]
    #[serde(default)]
    pub audit_status: String,
    /// 短信签名的创建日期和时间，格式为yyyy-MM-dd HH:mm:ss。
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    pub create_date: String,
    /// 审核备注。
    ///
    /// - 如果审核状态为**审核通过**或**审核中**，参数Reason显示为“无审核备注”。
    ///
    /// - 如果审核状态为**审核未通过**，参数Reason显示审核的具体原因。
    #[serde(rename = "Reason")]
    #[serde(default)]
    pub reason: SignListItemReason,
    /// 签名场景类型。取值：
    ///
    /// - 验证码类型。
    ///
    /// - 通用类型。
    #[serde(rename = "BusinessType")]
    #[serde(default)]
    pub business_type: String,
    /// 工单号。
    ///
    /// 审核人员查询审核时会用到此参数。您需要审核加急时需要提供此工单号。
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
    /// 委托授权书ID。
    #[serde(rename = "AuthorizationLetterId")]
    #[serde(default)]
    pub authorization_letter_id: i64,
    /// 委托授权书审核状态。取值：
    /// - true：审核通过。
    /// - false：审核未通过（包含审核通过外的其他所有状态）。
    #[serde(rename = "authorizationLetterAuditPass")]
    #[serde(default)]
    pub authorization_letter_audit_pass: bool,
    /// 商标实体id。
    #[serde(rename = "TrademarkId")]
    #[serde(default)]
    pub trademark_id: i64,
    /// APP-ICP备案实体id。
    #[serde(rename = "AppIcpRecordId")]
    #[serde(default)]
    pub app_icp_record_id: i64,
}

/// 返回数据结构。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct SignatureQualificationResponseData {
    /// 本数据无返回，可忽略。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: crate::OpenObject,
    /// 状态码的描述。
    #[serde(rename = "ErrMessage")]
    #[serde(default)]
    pub err_message: String,
    /// 调用接口是否成功。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 请求状态码。
    #[serde(rename = "ErrCode")]
    #[serde(default)]
    pub err_code: String,
}

/// 签名数据。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AddSmsSignSignFileList {
    /// 签名的资质证明文件经base64编码后的字符串。图片不超过2 MB。个别场景下，申请签名需要上传证明文件。
    ///
    /// 详细规范，请参见[上传文件规范](~~463316~~)。
    #[serde(rename = "FileContents")]
    pub file_contents: String,
    /// 签名的证明文件格式，支持上传多张图片。当前支持JPG、PNG、GIF或JPEG格式的图片。个别场景下，申请签名需要上传证明文件。
    ///
    /// > 如果签名用途为他用或个人认证用户的自用签名来源为企事业单位名时，还需上传证明文件和委托授权书，详情请参见[证明文件](~~108076~~)和[授权委托书](~~56741~~)。
    #[serde(rename = "FileSuffix")]
    pub file_suffix: String,
}
impl crate::FlatSerialize for AddSmsSignSignFileList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.file_contents,
            &format!("{}.FileContents", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.file_suffix,
            &format!("{}.FileSuffix", name),
            params,
        );
    }
}

/// 签名文件列表。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ModifySmsSignSignFileList {
    /// 签名的纸质证明文件经base64编码后的字符串。图片不超过2 MB。
    ///
    /// 个别场景下，申请签名需要上传证明文件。详细说明，请参见[短信签名规范](~~108076~~)。
    #[serde(rename = "FileContents")]
    pub file_contents: String,
    /// 签名的证明文件格式，支持上传多张图片。当前支持JPG、PNG、GIF或JPEG格式的图片。
    ///
    /// 个别场景下，申请签名需要上传证明文件。详细说明，请参见[短信签名规范](~~108076~~)。
    /// > 如果签名用途为他用或个人认证用户的自用签名来源为企事业单位名时，还需上传证明文件和委托授权书，详情请参见[证明文件](~~108076~~)和[授权委托书](~~56741~~)。
    #[serde(rename = "FileSuffix")]
    pub file_suffix: String,
}
impl crate::FlatSerialize for ModifySmsSignSignFileList {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.file_contents,
            &format!("{}.FileContents", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.file_suffix,
            &format!("{}.FileSuffix", name),
            params,
        );
    }
}

/// 商标详情。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct TrademarkResponseData {
    /// 商标截图url地址
    #[serde(rename = "TrademarkPicUrl")]
    #[serde(default)]
    pub trademark_pic_url: String,
    /// 商标注册号
    #[serde(rename = "TrademarkRegistrationNumber")]
    #[serde(default)]
    pub trademark_registration_number: String,
    /// 商标名称
    #[serde(rename = "TrademarkName")]
    #[serde(default)]
    pub trademark_name: String,
    /// 商标截图oss返回的图片fileKey
    #[serde(rename = "TrademarkPic")]
    #[serde(default)]
    pub trademark_pic: String,
    /// 专用权生失效日期
    #[serde(rename = "TrademarkEffExpDate")]
    #[serde(default)]
    pub trademark_eff_exp_date: String,
    /// 商标材料id
    #[serde(rename = "TrademarkId")]
    #[serde(default)]
    pub trademark_id: i64,
    /// 申请人名称
    #[serde(rename = "TrademarkApplicantName")]
    #[serde(default)]
    pub trademark_applicant_name: String,
}

/// APP-ICP备案实体详情。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct IcpRecordResponseData {
    /// APP-ICP备案材料id
    #[serde(rename = "AppIcpRecordId")]
    #[serde(default)]
    pub app_icp_record_id: i64,
    /// APP-ICP备案截图oss返回的图片fileKey
    #[serde(rename = "AppIcpRecordPic")]
    #[serde(default)]
    pub app_icp_record_pic: String,
    /// APP-ICP备案截图url地址
    #[serde(rename = "AppIcpRecordPicUrl")]
    #[serde(default)]
    pub app_icp_record_pic_url: String,
    /// 主办单位名称
    #[serde(rename = "AppPrincipalUnitName")]
    #[serde(default)]
    pub app_principal_unit_name: String,
    /// ICP备案/许可证号
    #[serde(rename = "AppIcpLicenseNumber")]
    #[serde(default)]
    pub app_icp_license_number: String,
    /// APP应用商店链接
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    /// 审核通过日期
    #[serde(rename = "AppApprovalDate")]
    #[serde(default)]
    pub app_approval_date: String,
    /// APP服务名称
    #[serde(rename = "AppServiceName")]
    #[serde(default)]
    pub app_service_name: String,
}

/// 审核信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct TemplateResponseAuditInfo {
    /// 审核未通过的原因。
    #[serde(rename = "RejectInfo")]
    #[serde(default)]
    pub reject_info: String,
    /// 审核时间。
    #[serde(rename = "AuditDate")]
    #[serde(default)]
    pub audit_date: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ResponseFileUrlList {
    /// 文件资料信息。兼容旧接口创建的信息。
    #[serde(rename = "FileUrl")]
    #[serde(default)]
    pub file_url: Vec<String>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DataFileUrlList {
    /// 更多资料信息，补充上传业务证明文件或业务截图文件列表。
    #[serde(rename = "MoreDataFileUrl")]
    #[serde(default)]
    pub more_data_file_url: Vec<String>,
}

/// 审核返回值。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct TemplateListItemReason {
    /// 审核未通过的时间，格式为yyyy-MM-dd HH:mm:ss。
    #[serde(rename = "RejectDate")]
    #[serde(default)]
    pub reject_date: String,
    /// 审核未通过的原因。
    #[serde(rename = "RejectInfo")]
    #[serde(default)]
    pub reject_info: String,
    /// 审核未通过的详细说明。
    #[serde(rename = "RejectSubInfo")]
    #[serde(default)]
    pub reject_sub_info: String,
}

/// 单个模板详情。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct TemplateList {
    /// 短信模板Code。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
    /// 短信模板名称。
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    /// 模板类型（对外使用）。返回值：
    ///
    /// - **0**：验证码短信。
    /// - **1**：通知短信。
    /// - **2**：推广短信。
    /// - **3**：国际/港澳台短信。
    #[serde(rename = "OuterTemplateType")]
    #[serde(default)]
    pub outer_template_type: i32,
    /// 模板审核状态。返回值：
    ///
    /// - **AUDIT_STATE_INIT**：审核中。
    /// - **AUDIT_STATE_PASS**：通过审核。
    /// - **AUDIT_STATE_NOT_PASS**：未通过审核，请在返回参数Reason中查看审核未通过原因。
    /// - **AUDIT_SATE_CANCEL**：取消审核。
    #[serde(rename = "AuditStatus")]
    #[serde(default)]
    pub audit_status: String,
    /// 模板内容。
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    pub template_content: String,
    /// 创建模板的时间，格式为yyyy-MM-dd HH:mm:ss。
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    pub create_date: String,
    /// 审核返回值。
    #[serde(rename = "Reason")]
    #[serde(default)]
    pub reason: TemplateListItemReason,
    /// 工单号。
    ///
    /// 审核人员查询审核时会用到此参数。您需要审核加急时需要提供此工单号。
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
    /// 模板类型。返回值：
    ///
    /// - **0**：短信通知。
    /// - **1**：推广短信。
    /// - **2**：验证码短信。
    /// - **6**：国际/港澳台短信。
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    pub template_type: i32,
    /// 关联签名名称。
    #[serde(rename = "SignatureName")]
    #[serde(default)]
    pub signature_name: String,
    /// 引流信息列表JSON字符串。
    /// ><notice>JSON格式，传入前请转为字符串。></notice>
    ///
    /// ### 1. 字段说明：
    ///
    /// {
    ///     "trafficDrivingType":"引流类型",
    ///     "trafficDrivingContent":"引流内容",
    ///     "variableName":"变量名称",
    ///     "companyName":企事业单位名称,
    ///     "organizationCode":统一社会信用代码,
    ///     "icpNo":ICP备案/许可证号,
    ///     "icpPicOssKey":ICP备案截图OssKey,
    ///     "companyDifferentFromSignQuaReason":企事业单位名称与签名资质不同原因
    /// }
    ///
    /// ### 2. 注意事项：
    ///
    /// - 如果非变量则variableName不传。
    ///
    /// - 企事业单位名称与签名关联资质的企事业单位名称不同，则传入companyDifferentFromSignQuaReason。
    ///
    /// - 如果trfficDrivingType=DOMAIN，则传入所有参数。
    ///
    /// - 如果trafficDrivingType为其他，则传入trafficDrivingType、trafficDrivingContent、variableName（按需）、companyName、organizationCode、companyDifferentFromSignQuaReason（按需）
    ///
    /// ### 3. trafficDrivingType引流类型枚举值：
    ///
    /// - DOMAIN：域名类型链接。
    /// - FIXED_PHONE：固定电话。
    /// - 400_PHONE：400开头电话。
    /// - 800_PHONE：800开头电话。
    /// - 95_PHONE：95开头电话。
    /// - 96_PHONE：96开头电话。
    /// - 1_PHONE：1开头，3~5位电话。
    /// - OTHER_PHONE：其他号码。
    #[serde(rename = "TrafficDriving")]
    #[serde(default)]
    pub traffic_driving: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DetailDto {
    /// 运营商短信状态码。
    ///
    /// -  短信发送成功：DELIVERED。
    /// -  短信发送失败：失败错误码请参见[错误码](~~101347~~)。
    #[serde(rename = "ErrCode")]
    #[serde(default)]
    pub err_code: String,
    /// 短信模板Code。
    ///
    /// > 若选择`[测试专用]阿里云通信`和`[测试专用]阿里云通信测试模板`发送的测试短信，本接口将不会返回TemplateCode字段。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
    /// 外部流水扩展字段。
    ///
    /// > 若发送短信时未传入OutId，本接口将不会返回OutId字段。
    #[serde(rename = "OutId")]
    #[serde(default)]
    pub out_id: String,
    /// 短信接收日期和时间。
    #[serde(rename = "ReceiveDate")]
    #[serde(default)]
    pub receive_date: String,
    /// 短信发送日期和时间。
    #[serde(rename = "SendDate")]
    #[serde(default)]
    pub send_date: String,
    /// 接收短信的手机号码。
    #[serde(rename = "PhoneNum")]
    #[serde(default)]
    pub phone_num: String,
    /// 短信内容。
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    /// 短信发送状态，包括：
    ///
    /// - **1**：等待回执。
    /// - **2**：发送失败。
    /// - **3**：发送成功。
    #[serde(rename = "SendStatus")]
    #[serde(default)]
    pub send_status: i64,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DetailDTOs {
    /// 短信发送明细。
    #[serde(rename = "SmsSendDetailDTO")]
    #[serde(default)]
    pub sms_send_detail_dto: Vec<DetailDto>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct TargetList {
    /// 发送成功的短信条数。
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    pub total_count: i64,
    /// 接收到回执成功的短信条数。
    #[serde(rename = "RespondedSuccessCount")]
    #[serde(default)]
    pub responded_success_count: i64,
    /// 接收到回执失败的短信条数。
    #[serde(rename = "RespondedFailCount")]
    #[serde(default)]
    pub responded_fail_count: i64,
    /// 未收到回执的短信条数。
    #[serde(rename = "NoRespondedCount")]
    #[serde(default)]
    pub no_responded_count: i64,
    /// 短信发送日期，格式为yyyyMMdd。
    #[serde(rename = "SendDate")]
    #[serde(default)]
    pub send_date: String,
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct StatisticsResponseData {
    /// 返回数据的总条数。
    #[serde(rename = "TotalSize")]
    #[serde(default)]
    pub total_size: i64,
    /// 返回数据列表。
    #[serde(rename = "TargetList")]
    #[serde(default)]
    pub target_list: Vec<TargetList>,
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ResponseData {
    /// 短信签名。
    #[serde(rename = "Signature")]
    #[serde(default)]
    pub signature: String,
    /// 访问地址。
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    /// 签名策略。
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    /// 超时时间戳。单位：秒。
    #[serde(rename = "ExpireTime")]
    #[serde(default)]
    pub expire_time: String,
    /// 阿里云账号ID。
    #[serde(rename = "AliUid")]
    #[serde(default)]
    pub ali_uid: String,
    /// 签名使用的AccessKey ID。
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
    /// 策略路径。
    #[serde(rename = "StartPath")]
    #[serde(default)]
    pub start_path: String,
    /// OSS文件保存桶名称。
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct IdResponseData {
    /// 资源ID。
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: i64,
    /// 资源下载地址。
    #[serde(rename = "ResUrlDownload")]
    #[serde(default)]
    pub res_url_download: String,
}

/// 返回对象。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct CreateCardSmsTemplateResponseData {
    /// 卡片短信模板Code。您可以在控制台**卡片短信**[模板管理](https://dysms.console.aliyun.com/domestic/card)页面查看**模板Code**。
    ///
    /// >必须是已添加、并通过审核的卡片短信模板。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct QueryCardSmsTemplateResponseData {
    /// 返回模板列表。
    #[serde(rename = "Templates")]
    #[serde(default)]
    pub templates: Vec<crate::OpenObject>,
}

/// 查询数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct CheckMobilesCardSupportResponseDataQueryResult {
    /// 手机号码。
    #[serde(rename = "mobile")]
    #[serde(default)]
    pub mobile: String,
    /// 是否支持卡片短信能力。取值：
    ///
    /// - **true**：支持卡片短信。
    ///
    /// - **false**：不支持卡片短信。
    #[serde(rename = "support")]
    #[serde(default)]
    pub support: bool,
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct CheckMobilesCardSupportResponseData {
    /// 查询结果列表。
    #[serde(rename = "queryResult")]
    #[serde(default)]
    pub query_result: Vec<CheckMobilesCardSupportResponseDataQueryResult>,
}

/// 查询值。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct QueryMobilesCardSupportResponseDataQueryResult {
    /// 查询的手机号码。
    #[serde(rename = "Mobile")]
    #[serde(default)]
    pub mobile: String,
    /// 是否支持卡片短信。取值：
    ///
    /// - **true**：支持。
    /// - **false**：不支持。
    #[serde(rename = "Support")]
    #[serde(default)]
    pub support: bool,
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct QueryMobilesCardSupportResponseData {
    /// 查询值。
    #[serde(rename = "QueryResult")]
    #[serde(default)]
    pub query_result: Vec<QueryMobilesCardSupportResponseDataQueryResult>,
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct LinkResponseData {
    /// 卡片短信模板审核状态。取值：
    /// - **0**：审核中。
    /// - **1**：审核通过。
    /// - **2**：审核不通过。
    ///
    /// > 未审核通过的短信走回落流程。
    #[serde(rename = "CardTmpState")]
    #[serde(default)]
    pub card_tmp_state: i32,
    /// 不支持卡片短信的手机号。
    #[serde(rename = "NotMediaMobiles")]
    #[serde(default)]
    pub not_media_mobiles: String,
    /// 支持卡片短信的手机号码。
    #[serde(rename = "CardPhoneNumbers")]
    #[serde(default)]
    pub card_phone_numbers: String,
    /// 卡片短信短链。
    #[serde(rename = "CardSmsLinks")]
    #[serde(default)]
    pub card_sms_links: String,
    /// 用于申请卡片短信短链的短信签名，在发送时签名、接收号码、卡片短信短链要一一对应。
    #[serde(rename = "CardSignNames")]
    #[serde(default)]
    pub card_sign_names: String,
}

/// 卡片短信发送记录。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DTORecord {
    /// 模板code。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
    /// 渲染时间。
    #[serde(rename = "RenderDate")]
    #[serde(default)]
    pub render_date: String,
    /// 接收时间。
    #[serde(rename = "ReceiveDate")]
    #[serde(default)]
    pub receive_date: String,
    /// 解析状态。取值：
    ///
    /// - 0：未解析；
    /// - 1：解析成功；
    /// - 3：未解析。
    #[serde(rename = "RenderStatus")]
    #[serde(default)]
    pub render_status: i64,
    /// 接收短信类型。
    #[serde(rename = "ReceiveType")]
    #[serde(default)]
    pub receive_type: String,
    /// 发送状态。取值：
    ///
    /// - 1：发送中；
    /// - 2：发送失败；
    /// - 3：发送成功；
    /// - 4：寻址失败
    #[serde(rename = "SendStatus")]
    #[serde(default)]
    pub send_status: i64,
    /// 客户传输outId。
    #[serde(rename = "OutId")]
    #[serde(default)]
    pub out_id: String,
    /// 接收短信手机号。
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    pub phone_number: String,
    /// 短信内容。只有文本短信有值。
    #[serde(rename = "SmsContent")]
    #[serde(default)]
    pub sms_content: String,
    /// 短信发送时间。
    #[serde(rename = "SendDate")]
    #[serde(default)]
    pub send_date: String,
    /// 发送错误码。
    #[serde(rename = "ErrCode")]
    #[serde(default)]
    pub err_code: String,
}

/// 卡片短信发送结果。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct DetailDTO {
    /// 总量。
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    pub total_count: i64,
    /// 页数。
    #[serde(rename = "PageSize")]
    #[serde(default)]
    pub page_size: i64,
    /// 页码。
    #[serde(rename = "CurrentPage")]
    #[serde(default)]
    pub current_page: i64,
    /// 卡片短信发送记录列表。
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<DTORecord>,
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ReportResponseData {
    /// 返回数据列表。
    #[serde(rename = "model")]
    #[serde(default)]
    pub model: Vec<crate::OpenObject>,
}

/// 卡片短信对象。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CardObject {
    /// 渲染失败后跳转链接。
    #[serde(rename = "customUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_url: Option<String>,
    /// 动态参数。动参变量不需要${}
    #[serde(rename = "dyncParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dync_params: Option<String>,
    /// 接收卡片短信的手机号码。
    #[serde(rename = "mobile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
}
impl crate::FlatSerialize for CardObject {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.custom_url,
            &format!("{}.customUrl", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.dync_params,
            &format!("{}.dyncParams", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.mobile, &format!("{}.mobile", name), params);
    }
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct SendCardSmsResponseData {
    /// 接收卡片短信的手机号。
    #[serde(rename = "MediaMobiles")]
    #[serde(default)]
    pub media_mobiles: String,
    /// 卡片短信发送ID。
    #[serde(rename = "BizCardId")]
    #[serde(default)]
    pub biz_card_id: String,
    /// 数字短信发送ID。
    #[serde(rename = "BizDigitalId")]
    #[serde(default)]
    pub biz_digital_id: String,
    /// 卡片短信模板审核状态。取值：
    /// - **0**：审核中。
    /// - **1**：审核通过。
    /// - **2**：审核不通过。
    /// >  审核不通过的短信可通过**FallbackType**字段设置回落流程。
    #[serde(rename = "CardTmpState")]
    #[serde(default)]
    pub card_tmp_state: i32,
    /// 回落的手机号。
    #[serde(rename = "NotMediaMobiles")]
    #[serde(default)]
    pub not_media_mobiles: String,
    /// 文本短信发送ID。
    #[serde(rename = "BizSmsId")]
    #[serde(default)]
    pub biz_sms_id: String,
}

/// 返回数据。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct BatchCardSmsResponseData {
    /// 接收卡片短信的手机号。
    #[serde(rename = "MediaMobiles")]
    #[serde(default)]
    pub media_mobiles: String,
    /// 卡片短信发送ID。
    #[serde(rename = "BizCardId")]
    #[serde(default)]
    pub biz_card_id: String,
    /// 数字短信发送ID。
    #[serde(rename = "BizDigitalId")]
    #[serde(default)]
    pub biz_digital_id: String,
    /// 卡片短信模板审核状态。取值：
    /// - **0**：审核中。
    /// - **1**：审核通过。
    /// - **2**：审核不通过。
    /// > 审核不通过的短信可通过**FallbackType**字段设置回落流程。
    #[serde(rename = "CardTmpState")]
    #[serde(default)]
    pub card_tmp_state: i32,
    /// 回落的手机号。
    #[serde(rename = "NotMediaMobiles")]
    #[serde(default)]
    pub not_media_mobiles: String,
    /// 文本短信发送ID。
    #[serde(rename = "BizSmsId")]
    #[serde(default)]
    pub biz_sms_id: String,
}

/// OSS配置信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct InfoResponseData {
    /// 签名策略。
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    /// 过期时间戳，单位：秒。
    #[serde(rename = "Expire")]
    #[serde(default)]
    pub expire: i64,
    /// 策略路径。
    #[serde(rename = "StartPath")]
    #[serde(default)]
    pub start_path: String,
    /// 签名使用的 AccessKey ID。
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
    /// 根据AccessKey Secret和Policy计算出的签名信息。调用OSS API时，OSS验证该签名信息，从而确认请求的合法性。
    #[serde(rename = "Signature")]
    #[serde(default)]
    pub signature: String,
    /// Host 地址。
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
}

/// 返回结果。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ResponseModel {
    /// 签名策略。
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    /// 策略路径。
    #[serde(rename = "StartPath")]
    #[serde(default)]
    pub start_path: String,
    /// 签名使用的AccessKey ID。
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
    /// 根据**AccessKey Secret**和**Policy**计算出的签名信息。调用OSS API时，OSS验证该签名信息，从而确认Post请求的合法性。
    #[serde(rename = "Signature")]
    #[serde(default)]
    pub signature: String,
    /// Host地址。
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    /// 到期时间。
    #[serde(rename = "ExpireTime")]
    #[serde(default)]
    pub expire_time: String,
}

/// OSS配置信息。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct InfoResponseModel {
    /// 签名策略。
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    /// 策略路径。
    #[serde(rename = "StartPath")]
    #[serde(default)]
    pub start_path: String,
    /// bucket名称。
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    /// 签名使用的 AccessKey ID。
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
    /// 根据 AccessKey Secret 和 Policy 计算出的签名信息。调用 OSS API 时，OSS 验证该签名信息，从而确认请求的合法性。
    #[serde(rename = "Signature")]
    #[serde(default)]
    pub signature: String,
    /// Host 地址。
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    /// 过期时间戳，单位：秒。
    #[serde(rename = "ExpireTime")]
    #[serde(default)]
    pub expire_time: String,
}

/// 短链详情。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct AddShortUrlResponseData {
    /// 原始链接地址。
    #[serde(rename = "SourceUrl")]
    #[serde(default)]
    pub source_url: String,
    /// 短链服务使用失效时间。
    ///
    /// > **ExpireDate**值为整点时间。
    #[serde(rename = "ExpireDate")]
    #[serde(default)]
    pub expire_date: String,
    /// 生成的短链服务地址。
    #[serde(rename = "ShortUrl")]
    #[serde(default)]
    pub short_url: String,
}

/// 短链详情。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct QueryShortUrlResponseData {
    /// 短链使用的UV数据。
    #[serde(rename = "UniqueVisitorCount")]
    #[serde(default)]
    pub unique_visitor_count: String,
    /// 原始链接地址。
    #[serde(rename = "SourceUrl")]
    #[serde(default)]
    pub source_url: String,
    /// 短链状态。取值：
    ///
    /// - **expired**：失效。
    /// - **effective**：有效。
    /// - **audit**：审核中。
    /// - **reject**：审核拒绝。
    #[serde(rename = "ShortUrlStatus")]
    #[serde(default)]
    pub short_url_status: String,
    /// 短链使用的PV数据。
    #[serde(rename = "PageViewCount")]
    #[serde(default)]
    pub page_view_count: String,
    /// 短链失效时间。
    #[serde(rename = "ExpireDate")]
    #[serde(default)]
    pub expire_date: String,
    /// 短链服务名称。
    #[serde(rename = "ShortUrlName")]
    #[serde(default)]
    pub short_url_name: String,
    /// 短链创建时间。
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    pub create_date: String,
    /// 生成的短链服务地址。
    #[serde(rename = "ShortUrl")]
    #[serde(default)]
    pub short_url: String,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ListTagResourcesTag {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl crate::FlatSerialize for ListTagResourcesTag {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(std::borrow::Cow<'static, str>, crate::QueryValue<'a>)>,
    ) {
        crate::FlatSerialize::flat_serialize(&self.key, &format!("{}.Key", name), params);
        crate::FlatSerialize::flat_serialize(&self.value, &format!("{}.Value", name), params);
    }
}

/// 标签资源。
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct TagResource {
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(default)]
    pub tag_value: String,
    /// 短信模板Code。
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub struct ResponseTagResources {
    /// 标签资源。
    #[serde(rename = "TagResource")]
    #[serde(default)]
    pub tag_resource: Vec<TagResource>,
}

/// 标签。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct TagResourcesTag {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl crate::FlatSerialize for TagResourcesTag {
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
pub enum EncryptType {
    #[serde(rename = "SHA1")]
    Sha1,
    #[serde(rename = "NORMAL")]
    Normal,
}

impl Default for EncryptType {
    fn default() -> Self {
        Self::Sha1
    }
}

impl EncryptType {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sha1 => "SHA1",
            Self::Normal => "NORMAL",
        }
    }
}

impl std::fmt::Display for EncryptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> From<&'a EncryptType> for crate::QueryValue<'a> {
    fn from(value: &'a EncryptType) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

/// - 在发起申请前，请您阅读[资质材料说明](~~2384377~~)并准备相关资质材料。
/// - 目前仅**企业认证**用户可使用API申请短信资质。若您当前阿里云账号为个人认证，请通过短信服务[控制台](https://dysms.console.aliyun.com/domestic/text/qualification/add)申请资质，或[升级为企业认证](~~37178~~)。[查看我的帐户认证类型](https://myaccount.console.aliyun.com/cert-info)
/// - 不支持批量申请短信资质，建议每次申请至少间隔5秒。
#[derive(Debug, Default, serde::Deserialize)]
pub struct SubmitSmsQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 资质ID。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 支持全量查询或条件查询：
///   - **全量查询**：查询您当前帐户下所有短信资质，无需传参。默认全量查询。
///   - **条件查询**：支持根据资质名称、企业名称、法人姓名、审核状态、审核工单ID、资质用途进行查询，传入您希望筛选的参数即可。
///
/// - 本接口用于查询资质及其审核信息，如果需要查询单个资质的具体信息（企业信息、法人信息、管理员信息），请调用[查询单个资质详情](~~QuerySingleSmsQualification~~)接口。
///
/// - 受短信签名实名制报备要求影响，当前资质审核工单量增长快速，审核时间可能会延长，请耐心等待，预计2个工作日内完成（审核工作时间：周一至周日 9:00~21:00，法定节假日顺延）。特殊情况可能延长审核时间，请耐心等待。
/// - 如果资质未通过审核，审核备注`AuditRemark`会返回审核失败的原因，请参考[审核失败的处理建议](~~2384377#a96cc318b94x1~~)，调用[修改短信资质](~~UpdateSmsQualification~~)接口或在控制台[资质管理](https://dysms.console.aliyun.com/domestic/text/qualification)页面修改资质信息后，重新发起审核。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySmsQualificationRecordResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 资质审核列表
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: QualificationRecordResponseData,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 本接口查询单个资质的详情（企业信息、法人信息、管理员信息）。
/// - 如果需要查询您当前账号下所有资质信息，或需要查询审核详情，请调用[查询资质列表](~~QuerySmsQualificationRecord~~)。
/// - 受短信签名实名制报备要求影响，当前资质审核工单量增长快速，审核时间可能会延长，请耐心等待，预计2个工作日内完成（审核工作时间：周一至周日 9:00~21:00，法定节假日顺延）。特殊情况可能延长审核时间，请耐心等待。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySingleSmsQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 单个资质详情。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: SmsQualificationResponseData,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 审核中的资质不支持修改，请等待审核流程结束或在短信服务控制台[撤回申请](https://dysms.console.aliyun.com/domestic/text/qualification)后再修改。
/// - 修改后的短信资质**需要重新审核**（包括已审核通过的资质），请根据[资质材料说明](~~2384377~~)上传符合规范的材料。
/// - **不支持修改**资质命名、申请用途、统一社会信用代码。
/// - 不支持批量修改短信资质，建议每次修改至少间隔5秒。
#[derive(Debug, Default, serde::Deserialize)]
pub struct UpdateSmsQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 资质ID。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// ><warning>资质删除后不能恢复，也无法在后续申请签名时选用，请谨慎操作。></warning>
/// - 审核中的资质不支持修改或删除，您可以在短信服务[控制台](https://dysms.console.aliyun.com/domestic/text/qualification)撤回申请后操作。
/// - 审核通过的资质若已被签名绑定则不支持删除。
/// - 审核不通过的资质可通过[修改资质信息](~~UpdateSmsQualification~~)后直接重新发起审核。
#[derive(Debug, Default, serde::Deserialize)]
pub struct DeleteSmsQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 是否删除成功。取值：
    ///
    /// - **true**：成功。
    /// - **false**：失败。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: bool,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 接收到手机验证码后，请传入[申请短信资质](~~SubmitSmsQualification~~)/[修改短信资质](~~UpdateSmsQualification~~)接口的`CertifyCode`参数中。
/// - 您可以通过[ValidPhoneCode](~~ValidPhoneCode~~)接口校验短信验证码是否准确。
/// - 本接口获取短信验证码有[流控限制](~~44335#section-0wh-xn6-0t7~~)，请勿频繁操作：针对同一个号码最多支持1条/分钟，5条/小时，10条/天。
#[derive(Debug, Default, serde::Deserialize)]
pub struct RequiredPhoneCodeResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 本数据无返回，可忽略。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 请先调用[获取手机验证码](~~RequiredPhoneCode~~)接口，阿里云将发送短信验证码至您填写的手机号码。
/// - 本接口不影响短信资质申请流程，仅供验证短信验证码使用。实际申请时，请在[申请短信资质](~~SubmitSmsQualification~~)接口中的`CertifyCode`参数传入验证码。
#[derive(Debug, Default, serde::Deserialize)]
pub struct ValidPhoneCodeResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 是否验证成功。取值：
    ///
    /// - **true**：验证成功。
    /// - **false**：验证失败。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: bool,
    /// 调用接口是否成功。取值：
    ///
    /// - true：调用成功。
    /// - false：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 请您在使用前阅读[授权书规范](~~56741~~)，下载[授权委托书模板](https://help-static-aliyun-doc.aliyuncs.com/file-manage-files/zh-CN/20250414/bvpcmo/%E6%8E%88%E6%9D%83%E5%A7%94%E6%89%98%E4%B9%A6%E6%A8%A1%E7%89%88.doc)后，根据规范完成填写并盖章后上传。
/// - 您创建的授权委托书可在后续申请短信资质/申请短信签名时使用，如果您的资质/签名涉及他用，则必须创建授权委托书并提交。
/// - 创建授权委托书后，您可以通过[QuerySmsAuthorizationLetter](~~QuerySmsAuthorizationLetter~~)查询已创建的授权书详情；通过接口创建的授权书信息会同步在短信服务控制台。
#[derive(Debug, Default, serde::Deserialize)]
pub struct CreateSmsAuthorizationLetterResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 委托授权书ID。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 支持全量查询或条件查询：
///   - **全量查询**：查询您当前帐户下所有授权委托书信息，无需传参。默认全量查询。
///   - **条件查询**：支持根据授权委托书ID、签名名称、授权委托书审核状态进行查询，传入您希望筛选的参数即可。
///
/// - 审核时间：受短信签名实名制报备要求影响，当前资质审核工单量增长快速，审核时间可能会延长，请耐心等待，预计2个工作日内完成。短信签名及模板预计在审核提交后的2小时内完成审核，涉及政府企业相关，一般2个工作日内审核完成。如遇升级核验、审核任务较多、非工作时间，审核时间可能会延长，请耐心等待（审核工作时间：周一至周日 9:00~21:00，法定节假日顺延）。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySmsAuthorizationLetterResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 委托授权书信息。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: Vec<LetterResponseData>,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
///
/// - 个人认证用户，同一个阿里云账号一个自然日支持申请一个正式签名；企业认证用户目前无限制。个人用户与企业用户权益区别详情请参见[使用须知](~~55324~~)。
///
/// - 请阅读[签名规范](~~108076~~)，了解短信签名审核规范的具体内容。
/// - 通过接口申请的签名信息会同步在短信服务控制台。控制台相关操作，请参见[短信签名](~~108073~~)。
///
/// - 提交签名申请后，您可以通过[GetSmsSign](~~2807429~~)接口查询签名审核状态和详情。也可以[配置回执消息](~~101508~~)，通过[SignSmsReport](~~120998~~)获取签名的审核状态消息。
#[derive(Debug, Default, serde::Deserialize)]
pub struct CreateSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 签名名称。
    #[serde(rename = "SignName")]
    #[serde(default)]
    pub sign_name: String,
    /// 工单号。
    ///
    /// 审核人员查询审核时会用到此参数。您需要审核加急时需要提供此工单号。
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
}
/// - 仅可查询**首次创建**的签名资料或者**最新审核通过**的资料。
///
/// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
///
/// - 审核时间：一般情况下，签名提交后，阿里云预计在 2 个小时内审核完成（审核工作时间：周一至周日 9:00~21:00，法定节假日顺延）。
///
/// - 如果签名未通过审核，会返回审核失败的原因，请参考[短信审核失败的处理建议](~~65990~~)，调用[UpdateSmsSign](~~2807428~~)接口或在[签名管理](https://dysms.console.aliyun.com/domestic/text/sign)页面修改未审核通过的短信签名。
///
/// - [QuerySmsSignList](~~QuerySmsSignList~~)接口可以查询您账号下的所有签名，包括签名审核状态、签名类型、签名名称等。
///
/// - 本接口的单用户QPS限制为150次/秒。超过限制，API调用将会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct GetSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 短信签名名称。
    #[serde(rename = "SignName")]
    #[serde(default)]
    pub sign_name: String,
    /// 签名审核状态。取值：
    ///
    /// - **0**：审核中。
    /// - **1**：审核通过。
    /// - **2**：审核失败，请在返回参数`AuditInfo.RejectInfo`中查看审核失败原因。
    /// - **10**：取消审核。
    #[serde(rename = "SignStatus")]
    #[serde(default)]
    pub sign_status: i64,
    /// 短信签名的创建日期和时间。
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    pub create_date: String,
    /// 工单号。
    ///
    /// 审核人员查询审核时会用到此参数。您需要审核加急时需要提供此工单号。
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
    /// 资质ID。申请签名时关联的资质ID。
    #[serde(rename = "QualificationId")]
    #[serde(default)]
    pub qualification_id: i64,
    /// 短信签名场景说明，长度不超过200个字符。
    #[serde(rename = "Remark")]
    #[serde(default)]
    pub remark: String,
    /// 审核信息。
    #[serde(rename = "AuditInfo")]
    #[serde(default)]
    pub audit_info: SignResponseAuditInfo,
    /// 更多资料信息，补充上传业务证明文件或业务截图文件列表。
    #[serde(rename = "FileUrlList")]
    #[serde(default)]
    pub file_url_list: Vec<String>,
    /// 短信签名Code。
    #[serde(rename = "SignCode")]
    #[serde(default)]
    pub sign_code: String,
    /// 签名标识。取值：
    ///
    /// - 2：用户自定义创建签名。
    /// - 3：系统赠送签名。
    /// - 4：测试签名。
    /// - 5：试用签名。
    #[serde(rename = "SignTag")]
    #[serde(default)]
    pub sign_tag: String,
    /// 应用场景内容。
    #[serde(rename = "ApplyScene")]
    #[serde(default)]
    pub apply_scene: String,
    /// 签名为自用或他用。
    ///
    /// - false：自用（默认值）。
    ///
    /// - true：他用。
    #[serde(rename = "ThirdParty")]
    #[serde(default)]
    pub third_party: bool,
    /// 签名使用场景。
    #[serde(rename = "SignUsage")]
    #[serde(default)]
    pub sign_usage: String,
    /// **已废弃，请使用`SignIspRegisterDetailList`查看各运营商实名报备结果。**
    ///
    /// 签名实名制报备结果。取值：
    /// - 0：报备失败。
    /// - 1：报备成功。
    /// - 2：报备失效。
    /// - -1：无状态。
    ///
    /// 建议您单击查看[更多签名实名制报备内容及建议操作](~~2873145~~)。
    #[serde(rename = "RegisterResult")]
    #[serde(default)]
    pub register_result: i32,
    /// 委托授权书ID。
    #[serde(rename = "AuthorizationLetterId")]
    #[serde(default)]
    pub authorization_letter_id: i64,
    /// 委托授权书审核状态。取值：
    /// - true：审核通过。
    /// - false：审核未通过（包含审核通过外的其他所有状态）。
    #[serde(rename = "AuthorizationLetterAuditPass")]
    #[serde(default)]
    pub authorization_letter_audit_pass: bool,
    /// 运营商报备状态列表。获取此参数返回数据需要[更新SDK](https://api.aliyun.com/api-tools/sdk/Dysmsapi?version=2017-05-25&language=java-tea&tab=primer-doc)至4.1.2版本或以上。
    #[serde(rename = "SignIspRegisterDetailList")]
    #[serde(default)]
    pub sign_isp_register_detail_list: Vec<DetailList>,
    /// 商标实体id。
    #[serde(rename = "TrademarkId")]
    #[serde(default)]
    pub trademark_id: i64,
    /// APP-ICP备案实体id。
    #[serde(rename = "AppIcpRecordId")]
    #[serde(default)]
    pub app_icp_record_id: i64,
}
/// 本接口可以查询您当前账号下**首次创建**的签名资料或者**最新审核通过**的签名详情。如果您需要查询应用场景内容、申请时上传的文件资料信息等更多内容，可以调用[GetSmsSign](~~GetSmsSign~~)接口通过签名名称查询单个签名审核详情。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySmsSignListResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 结果列表。
    #[serde(rename = "SmsSignList")]
    #[serde(default)]
    pub sms_sign_list: Vec<SignList>,
    /// 签名总数。
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    pub total_count: i64,
    /// 当前页码。默认取值为**1**。
    #[serde(rename = "CurrentPage")]
    #[serde(default)]
    pub current_page: i32,
    /// 每页显示的签名个数。默认取值为**10**，取值范围：**1~50**。
    #[serde(rename = "PageSize")]
    #[serde(default)]
    pub page_size: i32,
}
/// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
/// - 支持修改**未通过审核**和**已经审核通过**的签名，请参考[短信审核失败的处理建议](~~65990~~)，调用此接口修改后重新提交审核。
/// - **未通过审核**的签名如需编辑名称，该接口不支持，您可以访问控制台页面进行修改。[短信服务签名控制台入口](https://dysms.console.aliyun.com/domestic/text/sign)。
/// - 通过接口申请的签名信息会同步在短信服务控制台，在控制台对签名的相关操作，请参见[短信签名](~~108073~~)。
#[derive(Debug, Default, serde::Deserialize)]
pub struct UpdateSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 已修改的签名名称。
    #[serde(rename = "SignName")]
    #[serde(default)]
    pub sign_name: String,
    /// 工单号。
    ///
    /// 审核人员查询审核时会用到此参数。您需要审核加急时需要提供此工单号。
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct DeleteSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 已删除的签名名称。
    #[serde(rename = "SignName")]
    #[serde(default)]
    pub sign_name: String,
}
#[derive(Debug, Default, serde::Deserialize)]
pub struct ChangeSignatureQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 返回数据结构。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: SignatureQualificationResponseData,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct AddSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 签名名称。
    #[serde(rename = "SignName")]
    #[serde(default)]
    pub sign_name: String,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct ModifySmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 签名名称。
    #[serde(rename = "SignName")]
    #[serde(default)]
    pub sign_name: String,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 签名审核状态。取值：
    ///
    /// - **0**：审核中。
    /// - **1**：审核通过。
    /// - **2**：审核失败，请在返回参数Reason中查看审核失败原因。
    /// - **10**：取消审核。
    #[serde(rename = "SignStatus")]
    #[serde(default)]
    pub sign_status: i32,
    /// 短信签名的创建日期和时间。
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    pub create_date: String,
    /// 审核备注。
    ///
    /// - 如果审核状态为**审核通过**或**审核中**，参数Reason显示为“无审核备注”。
    /// - 如果审核状态为**审核未通过**，参数Reason显示审核的具体原因。
    #[serde(rename = "Reason")]
    #[serde(default)]
    pub reason: String,
    /// 短信签名。
    #[serde(rename = "SignName")]
    #[serde(default)]
    pub sign_name: String,
}
/// 商标应在国家知识产权局商标局-中国商标网中可查，且商标所有方与企业名称一致。
#[derive(Debug, Default, serde::Deserialize)]
pub struct CreateSmsTrademarkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 商标id
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    /// 接口调用是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// 传入商标id列表，返回商标详情。
///
/// 如查签名接口（QuerySmsSignList/GetSmsSign）会查出商标id，然后使用此接口进一步查询详情。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySmsTrademarkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 商标详情列表。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: Vec<TrademarkResponseData>,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// 签名来源选择已上线APP，则需要上传ICP备案截图。
#[derive(Debug, Default, serde::Deserialize)]
pub struct CreateSmsAppIcpRecordResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// APP-ICP备案实体ID。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**
    ///
    /// - **false**
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// 传入ICP备案id列表，返回ICP备案详情。
///
/// 如查签名接口（QuerySmsSignList/GetSmsSign）会查出ICP备案id，然后使用此接口进一步查询详情。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySmsAppIcpRecordResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// APP-ICP备案实体详情列表。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: Vec<IcpRecordResponseData>,
    /// 接口调用是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
///
/// - 通过接口申请短信模板，建议每次申请至少间隔30秒。
///
/// - 通过接口申请的模板信息会同步在短信服务控制台，在控制台对模板的相关操作，请参见[短信模板](~~108085~~)。
///
/// - 提交模板申请后，您可以通过[GetSmsTemplate](~~2807433~~)接口查询模板审核状态和详情。也可以[配置回执消息](~~101508~~)，通过[TemplateSmsReport](~~120999~~)获取模板的审核状态消息。
///
/// - 国内短信模板与国际/港澳台短信模板不通用（不能混用），请根据业务使用场景申请模板。
///
/// - 仅支持企业认证用户申请推广短信和国际/港澳台消息，个人用户与企业用户权益区别详情请参见[使用须知](~~55324~~)。
#[derive(Debug, Default, serde::Deserialize)]
pub struct CreateSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID。是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 短信模板名称。
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    /// 短信模板Code。
    ///
    /// 提交模板申请后，您可以使用短信模板Code，通过[GetSmsTemplate](~~2807433~~)接口查询模板审核详情。也可以[配置回执消息](~~101508~~)，通过[TemplateSmsReport](~~120999~~)获取模板的审核状态消息。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
    /// 工单号。
    ///
    /// 审核人员查询审核时会用到此参数。您需要审核加急时需要提供此工单号。
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
}
/// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
/// - 审核时间：一般情况下，模板提交后，阿里云预计在2个小时内审核完成（审核工作时间：周一至周日9:00~21:00，法定节假日顺延）。
///
/// - 如果模板未通过审核，会返回审核失败的原因，请参考[短信审核失败的处理建议](~~65990~~)，调用[UpdateSmsTemplate](~~UpdateSmsTemplate~~)接口或在[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面修改短信模板。
///
/// - 当前接口是通过模板Code查询单个模板的审核详情。[QuerySmsTemplateList](~~419288~~)接口可以查询您当前账号下所有模板的模板详情。
#[derive(Debug, Default, serde::Deserialize)]
pub struct GetSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 短信模板名称。
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    /// 短信模板Code。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
    /// 短信模板内容。
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    pub template_content: String,
    /// 短信类型。取值：
    ///
    /// - **0**：验证码。
    /// - **1**：短信通知。
    /// - **2**：推广短信。
    /// - **3**：国际/港澳台消息。
    ///
    /// > 仅支持企业认证用户申请推广短信和国际/港澳台消息。个人用户与企业用户权益区别详情请参见[使用须知](https://help.aliyun.com/zh/sms/user-guide/usage-notes?spm=a2c4g.11186623.0.0.67447f576NJnE8)。
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    pub template_type: String,
    /// 模板审核状态。返回值：
    ///
    /// - **0**：审核中。
    /// - **1**：通过审核。
    /// - **2**：未通过审核，会返回审核失败的原因，请参考[短信审核失败的处理建议](https://help.aliyun.com/zh/sms/user-guide/causes-of-application-failures-and-suggestions?spm=a2c4g.11186623.0.0.41fd339f3bPSCQ)，调用[UpdateSmsTemplate](https://help.aliyun.com/zh/sms/developer-reference/api-dysmsapi-2017-05-25-updatesmstemplate?spm)接口或在[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面修改短信模板。
    /// - **10**：取消审核。
    #[serde(rename = "TemplateStatus")]
    #[serde(default)]
    pub template_status: String,
    /// 申请模板时，关联的短信签名。
    #[serde(rename = "RelatedSignName")]
    #[serde(default)]
    pub related_sign_name: String,
    /// 模板标识。取值：
    ///
    /// - 2：用户自定义创建模板。
    ///
    /// - 3：系统赠送模板。
    ///
    /// - 4：测试模板。
    #[serde(rename = "TemplateTag")]
    #[serde(default)]
    pub template_tag: i32,
    /// 工单号。
    ///
    /// 审核人员查询审核时会用到此参数。您需要审核加急时需要提供此工单号。
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
    /// 模板变量规则。
    ///
    /// 模板变量规则详情，请参见[示例文档](https://help.aliyun.com/zh/sms/templaterule-template-variable-parameter-filling-example)。
    #[serde(rename = "VariableAttribute")]
    #[serde(default)]
    pub variable_attribute: String,
    /// 短信模板申请说明，是模板审核的参考信息之一。
    #[serde(rename = "Remark")]
    #[serde(default)]
    pub remark: String,
    /// 创建短信模板的时间。
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    pub create_date: String,
    /// 审核信息。
    #[serde(rename = "AuditInfo")]
    #[serde(default)]
    pub audit_info: TemplateResponseAuditInfo,
    #[serde(rename = "FileUrlList")]
    #[serde(default)]
    pub file_url_list: ResponseFileUrlList,
    #[serde(rename = "MoreDataFileUrlList")]
    #[serde(default)]
    pub more_data_file_url_list: DataFileUrlList,
    /// 国际/港澳台模板类型。当**TemplateType**参数返回值为**3**时，此参数取值：
    /// - **0**：短信通知。
    /// - **1**：推广短信。
    /// - **2**：验证码。
    #[serde(rename = "IntlType")]
    #[serde(default)]
    pub intl_type: i32,
    /// 应用场景内容。
    #[serde(rename = "ApplyScene")]
    #[serde(default)]
    pub apply_scene: String,
    /// 各运营商审核状态，仅数字短信会返回该参数。
    ///
    ///
    /// key代表运营商。取值：
    ///
    /// - MOBILE_VENDOR：中国移动。
    ///
    /// - TELECOM_VENDOR：中国电信。
    ///
    /// - UNICOM_VENDOR：中国联通。
    ///
    ///  value代表审核状态。取值：
    ///
    /// - 0：审核中。
    ///
    /// - 1：通过。
    ///
    ///  - 2：不通过。
    ///
    ///  - 15：已失效。
    #[serde(rename = "VendorAuditStatus")]
    #[serde(default)]
    pub vendor_audit_status: crate::OpenObject,
}
/// - 本接口用于查询您当前账号下所有模板的模板详情。如果您需要查询模板变量内容、申请时上传的文件资料信息等更多内容，可以调用[GetSmsTemplate](~~GetSmsTemplate~~)接口通过模板Code查询单个模板审核详情。
/// - 您也可登录短信服务控制台[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页查看您当前账号下所有模板的模板详情。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySmsTemplateListResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 结果列表。
    #[serde(rename = "SmsTemplateList")]
    #[serde(default)]
    pub sms_template_list: Vec<TemplateList>,
    /// 本次查询到的模板总数。
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    pub total_count: i64,
    /// 当前页码。默认取值为**1**。
    #[serde(rename = "CurrentPage")]
    #[serde(default)]
    pub current_page: i32,
    /// 每页显示的模板个数。取值范围：**1~50**。
    #[serde(rename = "PageSize")]
    #[serde(default)]
    pub page_size: i32,
}
/// - 新接口和原接口变更的公告详情请参见[关于短信服务更新签名&模板接口的公告](~~2806975~~)。
/// - 仅支持修改未通过审核的模板，请参考[短信审核失败的处理建议](~~65990~~)，调用此接口修改后重新提交审核。
///
/// - 通过接口修改模板的操作会在短信服务控制台同步，在控制台对模板的相关操作，请参见[短信模板](~~108085~~)。
///
/// ### QPS限制
/// 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct UpdateSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 短信模板名称。
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    /// 短信模板Code。
    ///
    /// 提交模板修改后，您可以使用短信模板Code，通过[GetSmsTemplate](~~GetSmsTemplate~~)接口查询模板审核详情。也可以[配置回执消息](https://help.aliyun.com/zh/sms/developer-reference/configure-delivery-receipts-1)，通过[TemplateSmsReport](~~120999~~)获取模板的审核状态消息。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
    /// 工单号。
    ///
    /// 审核人员查询审核时会用到此参数。您需要审核加急时需要提供此工单号。
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct DeleteSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 已删除的模板Code。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct AddSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 短信模板Code。
    ///
    /// 提交模板申请后，您可以使用短信模板Code，通过[QuerySmsTemplate](~~419289~~)接口查询模板审核详情。也可以[配置回执消息](~~101508~~)，通过[TemplateSmsReport](~~120999~~)获取模板的审核状态消息。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct ModifySmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 已修改的模板Code。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
}
/// - 根据工信部规定与运营商[相关要求](~~2806975~~)，阿里云进行了模板相关API的功能改造。请您使用新接口[GetSmsTemplate-查询模板审核详情](~~2807433~~)，新接口查询结果返回参数中将比原有接口返回更多的模板详情信息。
///
/// - 审核时间：一般情况下，模板提交后，阿里云预计在2个小时内审核完成（审核工作时间：周一至周日9:00~21:00，法定节假日顺延），建议您尽量在18:00前提交申请。
///
/// - 如果模板未通过审核，会返回审核失败的原因，请参考[短信审核失败的处理建议](~~65990~~)，调用[ModifySmsTemplate](~~419287~~)接口或在[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面修改短信模板。
///
/// - QuerySmsTemplate当前接口是通过模板Code查询单个模板的审核详情。[QuerySmsTemplateList](~~419288~~)接口可以查询您当前账号下所有模板的模板详情。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 短信模板内容。
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    pub template_content: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 短信模板Code。
    #[serde(rename = "TemplateCode")]
    #[serde(default)]
    pub template_code: String,
    /// 模板审核状态。返回值：
    ///
    /// - **0**：审核中。
    /// - **1**：通过审核。
    /// - **2**：未通过审核，会返回审核失败的原因，请参考[短信审核失败的处理建议](~~65990~~)，调用[ModifySmsTemplate](~~419287~~)接口或在[模板管理](https://dysms.console.aliyun.com/domestic/text/template)页面修改短信模板。
    /// - **10**：取消审核。
    #[serde(rename = "TemplateStatus")]
    #[serde(default)]
    pub template_status: i32,
    /// 短信类型。返回值：
    ///
    /// - **0**：验证码。
    /// - **1**：短信通知。
    /// - **2**：推广短信。
    /// - **3**：国际/港澳台消息。
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    pub template_type: i32,
    /// 短信模板名称。
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    /// 创建短信模板的时间。
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    pub create_date: String,
    /// 模板审核备注。
    ///
    /// - 如果审核状态为**通过审核**或**审核中**，Reason返回“无审批备注”。
    /// - 如果审核状态为**未通过审核**，Reason返回未审核通过的具体原因。
    #[serde(rename = "Reason")]
    #[serde(default)]
    pub reason: String,
}
/// 本接口主要用于向单个手机号发送短信，也支持向多个手机号（单次最多支持 1000 个手机号）发送相同签名、相同模板变量的短信，群发存在一定延迟。如果您需要向多个手机号发送不同签名、不同模板变量的短信，请使用[SendBatchSms](~~419274~~)接口（单次最多支持100个手机号）。
///
/// ### 注意事项
/// - 国内短信服务超时时间建议设置为≥1S；发生超时失败的情况时，建议查看回执状态后再判断是否重试。超时和重试的相关设置，请参见[超时机制](~~262079~~)、[重试机制](~~262080~~)。
/// - 国内短信、国际短信及多媒体短信目前均不支持幂等的能力，请您做好幂等控制，防止因多次重试而导致的重复操作问题。
/// - 发送短信为计费接口，国内短信按照运营商回执状态计费，调用 SendSms 提交成功但运营商回执失败时不计费。计费详情请参见[计费概述](~~44340~~)。
///
/// ### QPS 限制
/// 本接口的单用户 QPS 限制为 5000/秒。超过限制，API 调用将会被限流，请合理使用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct SendSmsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 发送回执ID。
    ///
    /// 可根据发送回执ID在接口[QuerySendDetails](~~QuerySendDetails~~)中查询具体的发送状态。
    #[serde(rename = "BizId")]
    #[serde(default)]
    pub biz_id: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
}
/// ### 基本信息
/// - 本接口主要用于向多个手机号发送短信，支持发送不同签名、同一模板、不同模板变量的短信，单次调用最多支持 100 个手机号。
/// - [服务接入点](~~419270~~) (Endpoint)：全局接入点域名为`dysmsapi.aliyuncs.com`。请参见[服务接入点](~~419270~~) ，根据您的使用地域，选择对应的接入点地址。
/// ### 快速调用
/// - 推荐您通过 SDK 调用 API。 如果您需要了解如何使用 ，请参见[首次调用API](~~2841024~~)。
/// - 如果您需要使用控制台发送短信，请参见[群发短信](~~108266~~)。
/// -  如果您需要自定义封装API调用，请参见[V3版本请求体&签名机制](~~2593177~~)
/// ### 注意事项
/// - 国内短信服务超时时间建议设置为≥1S；发生超时失败的情况时，建议查看回执状态后再判断是否重试。超时和重试的相关设置，请参见[超时机制](~~262079~~)、[重试机制](~~262080~~)。
/// - 国内短信、国际短信及多媒体短信目前均不支持幂等的能力，请您做好幂等控制，防止因多次重试而导致的重复操作问题。
/// - 发送短信为计费接口，国内短信按照运营商回执状态计费，调用SendBatchSms提交成功但运营商回执失败的短信不计费，计费详情请参见[计费概述](~~44340~~)。
/// ### QPS 限制
/// 本接口的单用户 QPS 限制为 5000/秒。超过限制，API 调用将会被限流，请合理使用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct SendBatchSmsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 发送回执ID。
    ///
    /// - 根据该ID在接口[QuerySendDetails](~~102352~~)中查询具体的发送状态。
    /// - 登录[短信服务控制台](https://dysms.console.aliyun.com/dysms.htm#/overview)，在**业务统计**-**发送记录查询**页面查看发送详情。
    #[serde(rename = "BizId")]
    #[serde(default)]
    pub biz_id: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
}
/// - 本接口主要用于查询指定日期下，向某个手机号码发送短信的记录详情。您也可以传入发送流水号（BizId），查询该号码的指定发送记录。
///
/// - 本接口仅支持查询单个手机号码发送详情。如果需要批量查看短信发送详情，您可以使用[QuerySendStatistics](~~419276~~)接口，查询短信发送统计详情；或登录[控制台发送记录查询](https://dysms.console.aliyun.com/record)页面，查询发送详情。
///
/// ### QPS限制
/// 本接口的单用户QPS限制为5000/秒。超过限制，API调用将会被限流，请合理使用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySendDetailsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 短信发送总条数。
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    pub total_count: i64,
    #[serde(rename = "SmsSendDetailDTOs")]
    #[serde(default)]
    pub sms_send_detail_dtos: DetailDTOs,
}
/// - 如果选择的时间范围较长的话，可以分页查看。指定每页显示的短信详情数量和查看的页数，即可分页查看发送记录。
///
/// - 您可以登录[短信服务控制台](https://dysms.console.aliyun.com/dysms.htm#/overview)，在**业务统计**-**发送记录**页面查询发送详情。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QuerySendStatisticsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: StatisticsResponseData,
}
/// - 您在调用卡片短信相关API接口前，首先需要开通卡片短信功能，目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或联系[阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html)。
///
/// - 卡片短信模板中使用的图片、视频等素材资源可上传到OSS文件系统保存。文件上传操作，请参见[OSS文件上传](~~437303~~)。
///
/// ### QPS限制
/// 本接口的单用户QPS限制为300次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct GetOSSInfoForCardTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: ResponseData,
}
/// ### QPS限制
/// 本接口的单用户QPS限制为300次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct GetMediaResourceIdResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: IdResponseData,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct CreateCardSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回对象。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: CreateCardSmsTemplateResponseData,
}
/// - 未开通卡片短信业务的阿里云账号无法调用此API。
/// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或[联系阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html)。
/// - 您也可登录控制台[国内卡片短信](https://dysms.console.aliyun.com/domestic/card)页面，在模板管理页签内查询卡片短信模板的审核状态。
/// ### QPS限制
/// 本接口的单用户QPS限制为300次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QueryCardSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 接口调用是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: QueryCardSmsTemplateResponseData,
}
/// - 未开通卡片短信业务的阿里云账号无法调用此API。
/// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或[联系阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html)。
/// - 推荐使用新接口[QueryMobilesCardSupport](~~QueryMobilesCardSupport~~)查询手机号是否支持卡片短信。
/// ### QPS限制
/// 本接口的单用户QPS限制为2000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct CheckMobilesCardSupportResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 接口调用是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: CheckMobilesCardSupportResponseData,
}
/// - 未开通卡片短信业务的阿里云账号无法调用此API。
/// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或[联系阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html)。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QueryMobilesCardSupportResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 阿里云为该请求生成的唯一标识符。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: QueryMobilesCardSupportResponseData,
}
/// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或联系[阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html?spm=a2c4g.11186623.0.0.213273d4Xe6UEu#section-81n-72q-ybm)。
///
/// ### QPS限制
/// - 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct GetCardSmsLinkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 接口调用是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: LinkResponseData,
}
#[derive(Debug, Default, serde::Deserialize)]
pub struct GetCardSmsDetailsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息；只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 卡片短信发送结果。
    #[serde(rename = "CardSendDetailDTO")]
    #[serde(default)]
    pub card_send_detail_dto: DetailDTO,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// ### QPS限制
/// 本接口的单用户QPS限制为300次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct QueryCardSmsTemplateReportResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: ReportResponseData,
}
/// - 发送卡片短信为计费接口，卡片短信发送失败或渲染失败时不计费，详情请参见[多媒体短信定价](~~437951~~)。
/// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或联系[阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html?spm=a2c4g.11186623.0.0.213219fcSn2Ykj#section-81n-72q-ybm)。
/// - 卡片短信超时时间建议设置为≥3S；发生超时失败的情况时，建议查看回执状态后再判断是否重试。同时建议您在调用此接口时，不要开启SDK重试逻辑，否则可能会造成多次发送的情况。超时和重试的相关设置，请参见[超时机制](~~262079~~)、[重试机制](~~262080~~)。
/// - 国内短信、国际短信及多媒体短信目前均不支持幂等的能力，请您做好幂等控制，防止因多次重试而导致的重复操作问题。
/// - 发送卡片短信前需添加卡片短信模板且模板审核通过。本接口在发送短信时，会校验号码是否支持卡片短信。如果该手机号不支持发送卡片短信，可在接口中设置是否接受数字短信和文本短信的回落，提升发送的触达率。
///
/// ### QPS限制
/// 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct SendCardSmsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: SendCardSmsResponseData,
}
/// - 发送卡片短信为计费接口，卡片短信发送失败或渲染失败时不计费，详情请参见[多媒体短信定价](~~437951~~)。
/// - 目前卡片短信在内部邀约阶段，请联系您的阿里云商务经理申请开通或联系[阿里云售前咨询](https://help.aliyun.com/document_detail/464625.html?spm=a2c4g.11186623.0.0.213219fcSn2Ykj#section-81n-72q-ybm)。
/// - 卡片短信超时时间建议设置为≥3S；发生超时失败的情况时，建议查看回执状态后再判断是否重试。同时建议您在调用此接口时，不要开启SDK重试逻辑，否则可能会造成多次发送的情况。超时和重试的相关设置，请参见[超时机制](~~262079~~)、[重试机制](~~262080~~)。
/// - 国内短信、国际短信及多媒体短信目前均不支持幂等的能力，请您做好幂等控制，防止因多次重试而导致的重复操作问题。
/// - 发送卡片短信前需添加卡片短信模板且模板审核通过。本接口在发送短信时，会校验号码是否支持卡片短信。如果该手机号不支持发送卡片短信，可在接口中设置是否接受数字短信和文本短信的回落，提升发送的触达率。
/// - 批量发送卡片短信，每个号码可以使用不同的签名，不同的回落。在一次请求中，最多可以向100个手机号码分别发送卡片短信。
///
/// ### QPS限制
/// 本接口的单用户QPS限制为1000次/秒。超过限制，API调用会被限流，这可能会影响您的业务，请合理调用。
#[derive(Debug, Default, serde::Deserialize)]
pub struct SendBatchCardSmsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
    /// 返回数据。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: BatchCardSmsResponseData,
}
/// - 您在申请资质/签名时，若用途为他用或涉及第三方，需要提供[授权委托书](~~56741~~)。
/// - 请使用本接口获取OSS资源配置信息后，通过OSS上传相关资质材料。具体操作，可参见[通过OSS上传文件](~~2833114~~)。
/// - 待上传的文件命名不支持包含中文和特殊符号。
#[derive(Debug, Default, serde::Deserialize)]
pub struct GetQualificationOssInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有RAM校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// OSS配置信息。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: InfoResponseData,
    /// 调用接口是否成功。取值：
    ///
    /// - **true**：调用成功。
    ///
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// - 您在创建签名或模板时，可上传带有链接的登录页面、后台页面截图、软著、协议补充等资料。有助于审核人员了解您的业务详情。如果是多个资料，可拼成一个文件，支持png、jpg、jpeg、doc、docx、pdf格式。
///
/// - 创建签名或模板所需的更多资料，可上传到OSS文件系统保存。文件上传操作，请参见[OSS上传文件](~~2833114~~)。
#[derive(Debug, Default, serde::Deserialize)]
pub struct GetOSSInfoForUploadFileResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 返回结果。
    #[serde(rename = "Model")]
    #[serde(default)]
    pub model: ResponseModel,
    /// 接口调用是否成功。取值：
    ///
    /// - **true**：调用成功。
    /// - **false**：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
#[derive(Debug, Default, serde::Deserialize)]
pub struct GetSmsOcrOssInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 访问被拒绝详细信息，只有 RAM 校验失败才会返回此字段。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(default)]
    pub access_denied_detail: String,
    /// 本次调用请求的 ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// OSS配置信息。
    #[serde(rename = "Model")]
    #[serde(default)]
    pub model: InfoResponseModel,
    /// 调用接口是否成功。取值：
    ///
    /// - true：调用成功。
    ///
    /// - false：调用失败。
    #[serde(rename = "Success")]
    #[serde(default)]
    pub success: bool,
}
/// 指标说明：
///
/// - OTP发送量：验证码发送量。
///
/// - OTP转化量：验证码转换量。（用户成功获取验证码，并进行回传）
///
/// 转化率=OTP转化量/OTP发送量。
///
/// > 转化率反馈功能会对业务系统有一定的侵入性，为了防止调用转化率API的抖动影响业务逻辑，请考虑：  - 使用异步模式（例如：队列或事件驱动）调用API。  - 添加可降级的方案保护业务逻辑（例如：手动降级开工或者使用断路器自动降级）。
#[derive(Debug, Default, serde::Deserialize)]
pub struct SmsConversionIntlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
}
/// 指标说明：转化率=OTP 转化量/OTP 发送量。
/// - OTP发送量：验证码发送量。
/// - OTP转化量：验证码转换量。（用户成功获取验证码，并进行回传）
/// >转化率反馈功能会对业务系统有一定的侵入性，为了防止调用转化率 API 的抖动影响业务逻辑，请考虑：
/// >- 使用异步模式（例如：队列或事件驱动）调用 API。
/// >- 添加可降级的方案保护业务逻辑（例如：手动降级开工或者使用断路器自动降级）。
#[derive(Debug, Default, serde::Deserialize)]
pub struct ConversionDataIntlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct AddShortUrlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 短链详情。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: AddShortUrlResponseData,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct DeleteShortUrlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct QueryShortUrlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    /// 短链详情。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: QueryShortUrlResponseData,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct ListTagResourcesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 查询下一页标签的Token。
    #[serde(rename = "NextToken")]
    #[serde(default)]
    pub next_token: String,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(rename = "TagResources")]
    #[serde(default)]
    pub tag_resources: ResponseTagResources,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct TagResourcesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 添加标签的执行结果。取值：
    ///
    /// - **true**：成功。
    /// - **false**：失败。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
}
///  
#[derive(Debug, Default, serde::Deserialize)]
pub struct UntagResourcesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    /// 删除标签的执行结果。取值：
    ///
    /// - **true**：成功。
    ///
    /// - **false**：失败。
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
}

crate::impl_to_code_message!(
    SubmitSmsQualificationResponse,
    QuerySmsQualificationRecordResponse,
    QuerySingleSmsQualificationResponse,
    UpdateSmsQualificationResponse,
    DeleteSmsQualificationResponse,
    RequiredPhoneCodeResponse,
    ValidPhoneCodeResponse,
    CreateSmsAuthorizationLetterResponse,
    QuerySmsAuthorizationLetterResponse,
    CreateSmsSignResponse,
    GetSmsSignResponse,
    QuerySmsSignListResponse,
    UpdateSmsSignResponse,
    DeleteSmsSignResponse,
    ChangeSignatureQualificationResponse,
    AddSmsSignResponse,
    ModifySmsSignResponse,
    QuerySmsSignResponse,
    CreateSmsTrademarkResponse,
    QuerySmsTrademarkResponse,
    CreateSmsAppIcpRecordResponse,
    QuerySmsAppIcpRecordResponse,
    CreateSmsTemplateResponse,
    GetSmsTemplateResponse,
    QuerySmsTemplateListResponse,
    UpdateSmsTemplateResponse,
    DeleteSmsTemplateResponse,
    AddSmsTemplateResponse,
    ModifySmsTemplateResponse,
    QuerySmsTemplateResponse,
    SendSmsResponse,
    SendBatchSmsResponse,
    QuerySendDetailsResponse,
    QuerySendStatisticsResponse,
    GetOSSInfoForCardTemplateResponse,
    GetMediaResourceIdResponse,
    CreateCardSmsTemplateResponse,
    QueryCardSmsTemplateResponse,
    CheckMobilesCardSupportResponse,
    QueryMobilesCardSupportResponse,
    GetCardSmsLinkResponse,
    GetCardSmsDetailsResponse,
    QueryCardSmsTemplateReportResponse,
    SendCardSmsResponse,
    SendBatchCardSmsResponse,
    GetQualificationOssInfoResponse,
    GetOSSInfoForUploadFileResponse,
    GetSmsOcrOssInfoResponse,
    SmsConversionIntlResponse,
    ConversionDataIntlResponse,
    AddShortUrlResponse,
    DeleteShortUrlResponse,
    QueryShortUrlResponse,
    ListTagResourcesResponse,
    TagResourcesResponse,
    UntagResourcesResponse
);

use std::collections::HashMap;

impl SendSms {
    pub fn with_numbers(
        phone_numbers: impl IntoIterator<Item = String>,
        sign_name: impl Into<String>,
        template_code: impl Into<String>,
        template_params: HashMap<String, String>,
    ) -> crate::Result<Self> {
        let phone_numbers: Vec<String> = phone_numbers.into_iter().collect();
        Ok(Self::new(phone_numbers.join(","), sign_name, template_code)
            .template_param(serde_json::to_string(&template_params)?))
    }
}
