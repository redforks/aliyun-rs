#[derive(Clone, Copy)]
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
pub struct Connection(crate::common::Connection);

impl Connection {
    pub fn new(endpoint: Endpoint, app_key_secret: crate::v3::AccessKeySecret) -> Self {
        Self(crate::common::Connection::new(
            app_key_secret,
            "1.0",
            endpoint.into(),
        ))
    }

    fn call<R: crate::Request + sealed::Bound>(
        &self,
        req: R,
    ) -> impl std::future::Future<Output = crate::Result<<R as crate::Request>::Response>> + Send
    {
        self.0.call(req)
    }

    pub fn submit_sms_qualification(
        &self,
        req: SubmitSmsQualification,
    ) -> impl std::future::Future<Output = crate::Result<SubmitSmsQualificationResponse>> + Send
    {
        self.call(req)
    }

    pub fn query_sms_qualification_record(
        &self,
        req: QuerySmsQualificationRecord,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsQualificationRecordResponse>> + Send
    {
        self.call(req)
    }

    pub fn query_single_sms_qualification(
        &self,
        req: QuerySingleSmsQualification,
    ) -> impl std::future::Future<Output = crate::Result<QuerySingleSmsQualificationResponse>> + Send
    {
        self.call(req)
    }

    pub fn update_sms_qualification(
        &self,
        req: UpdateSmsQualification,
    ) -> impl std::future::Future<Output = crate::Result<UpdateSmsQualificationResponse>> + Send
    {
        self.call(req)
    }

    pub fn delete_sms_qualification(
        &self,
        req: DeleteSmsQualification,
    ) -> impl std::future::Future<Output = crate::Result<DeleteSmsQualificationResponse>> + Send
    {
        self.call(req)
    }

    pub fn required_phone_code(
        &self,
        req: RequiredPhoneCode,
    ) -> impl std::future::Future<Output = crate::Result<RequiredPhoneCodeResponse>> + Send {
        self.call(req)
    }

    pub fn valid_phone_code(
        &self,
        req: ValidPhoneCode,
    ) -> impl std::future::Future<Output = crate::Result<ValidPhoneCodeResponse>> + Send {
        self.call(req)
    }

    pub fn create_sms_authorization_letter(
        &self,
        req: CreateSmsAuthorizationLetter,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsAuthorizationLetterResponse>> + Send
    {
        self.call(req)
    }

    pub fn query_sms_authorization_letter(
        &self,
        req: QuerySmsAuthorizationLetter,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsAuthorizationLetterResponse>> + Send
    {
        self.call(req)
    }

    pub fn create_sms_sign(
        &self,
        req: CreateSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsSignResponse>> + Send {
        self.call(req)
    }

    pub fn get_sms_sign(
        &self,
        req: GetSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<GetSmsSignResponse>> + Send {
        self.call(req)
    }

    pub fn query_sms_sign_list(
        &self,
        req: QuerySmsSignList,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsSignListResponse>> + Send {
        async {
            todo!(r##"Only HttpMethod::Get supported"##);
        }
    }

    pub fn update_sms_sign(
        &self,
        req: UpdateSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<UpdateSmsSignResponse>> + Send {
        self.call(req)
    }

    pub fn delete_sms_sign(
        &self,
        req: DeleteSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<DeleteSmsSignResponse>> + Send {
        self.call(req)
    }

    pub fn change_signature_qualification(
        &self,
        req: ChangeSignatureQualification,
    ) -> impl std::future::Future<Output = crate::Result<ChangeSignatureQualificationResponse>> + Send
    {
        self.call(req)
    }

    pub fn add_sms_sign(
        &self,
        req: AddSmsSign,
    ) -> impl std::future::Future<Output = crate::Result<AddSmsSignResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'SignFileList': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'SignFileList': Unsupported ParameterIn variant: FormData. Only Query parameters are supported."##
            );
        }
    }

    pub fn modify_sms_sign(
        &self,
        req: ModifySmsSign,
    ) -> impl std::future::Future<Output = crate::Result<ModifySmsSignResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'SignFileList': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'SignFileList': Unsupported ParameterIn variant: FormData. Only Query parameters are supported."##
            );
        }
    }

    pub fn query_sms_sign(
        &self,
        req: QuerySmsSign,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsSignResponse>> + Send {
        self.call(req)
    }

    pub fn create_sms_trademark(
        &self,
        req: CreateSmsTrademark,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsTrademarkResponse>> + Send {
        self.call(req)
    }

    pub fn query_sms_trademark(
        &self,
        req: QuerySmsTrademark,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsTrademarkResponse>> + Send {
        self.call(req)
    }

    pub fn create_sms_app_icp_record(
        &self,
        req: CreateSmsAppIcpRecord,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsAppIcpRecordResponse>> + Send
    {
        self.call(req)
    }

    pub fn query_sms_app_icp_record(
        &self,
        req: QuerySmsAppIcpRecord,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsAppIcpRecordResponse>> + Send {
        self.call(req)
    }

    pub fn create_sms_template(
        &self,
        req: CreateSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<CreateSmsTemplateResponse>> + Send {
        self.call(req)
    }

    pub fn get_sms_template(
        &self,
        req: GetSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<GetSmsTemplateResponse>> + Send {
        self.call(req)
    }

    pub fn query_sms_template_list(
        &self,
        req: QuerySmsTemplateList,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsTemplateListResponse>> + Send {
        async {
            todo!(r##"Only HttpMethod::Get supported"##);
        }
    }

    pub fn update_sms_template(
        &self,
        req: UpdateSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<UpdateSmsTemplateResponse>> + Send {
        self.call(req)
    }

    pub fn delete_sms_template(
        &self,
        req: DeleteSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<DeleteSmsTemplateResponse>> + Send {
        self.call(req)
    }

    pub fn add_sms_template(
        &self,
        req: AddSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<AddSmsTemplateResponse>> + Send {
        self.call(req)
    }

    pub fn modify_sms_template(
        &self,
        req: ModifySmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<ModifySmsTemplateResponse>> + Send {
        self.call(req)
    }

    pub fn query_sms_template(
        &self,
        req: QuerySmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<QuerySmsTemplateResponse>> + Send {
        self.call(req)
    }

    pub fn send_sms(
        &self,
        req: SendSms,
    ) -> impl std::future::Future<Output = crate::Result<SendSmsResponse>> + Send {
        self.call(req)
    }

    pub fn send_batch_sms(
        &self,
        req: SendBatchSms,
    ) -> impl std::future::Future<Output = crate::Result<SendBatchSmsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'PhoneNumberJson': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'SignNameJson': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'TemplateParamJson': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'SmsUpExtendCodeJson': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'PhoneNumberJson': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'SignNameJson': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'TemplateParamJson': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'SmsUpExtendCodeJson': Unsupported ParameterIn variant: FormData. Only Query parameters are supported."##
            );
        }
    }

    pub fn query_send_details(
        &self,
        req: QuerySendDetails,
    ) -> impl std::future::Future<Output = crate::Result<QuerySendDetailsResponse>> + Send {
        self.call(req)
    }

    pub fn query_send_statistics(
        &self,
        req: QuerySendStatistics,
    ) -> impl std::future::Future<Output = crate::Result<QuerySendStatisticsResponse>> + Send {
        async {
            todo!(r##"Only HttpMethod::Get supported"##);
        }
    }

    pub fn get_oss_info_for_card_template(
        &self,
        req: GetOSSInfoForCardTemplate,
    ) -> impl std::future::Future<Output = crate::Result<GetOSSInfoForCardTemplateResponse>> + Send
    {
        async {
            todo!(
                r##"Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn get_media_resource_id(
        &self,
        req: GetMediaResourceId,
    ) -> impl std::future::Future<Output = crate::Result<GetMediaResourceIdResponse>> + Send {
        async {
            todo!(
                r##"Only HttpMethod::Get supported
Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn create_card_sms_template(
        &self,
        req: CreateCardSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<CreateCardSmsTemplateResponse>> + Send
    {
        async {
            todo!(
                r##"Only HttpMethod::Get supported
Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn query_card_sms_template(
        &self,
        req: QueryCardSmsTemplate,
    ) -> impl std::future::Future<Output = crate::Result<QueryCardSmsTemplateResponse>> + Send {
        async {
            todo!(
                r##"Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn check_mobiles_card_support(
        &self,
        req: CheckMobilesCardSupport,
    ) -> impl std::future::Future<Output = crate::Result<CheckMobilesCardSupportResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'Mobiles': Unsupported ParameterStyle variant: Flat. Only Json style is supported.
Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn query_mobiles_card_support(
        &self,
        req: QueryMobilesCardSupport,
    ) -> impl std::future::Future<Output = crate::Result<QueryMobilesCardSupportResponse>> + Send
    {
        async {
            todo!(
                r##"Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn get_card_sms_link(
        &self,
        req: GetCardSmsLink,
    ) -> impl std::future::Future<Output = crate::Result<GetCardSmsLinkResponse>> + Send {
        async {
            todo!(
                r##"Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn get_card_sms_details(
        &self,
        req: GetCardSmsDetails,
    ) -> impl std::future::Future<Output = crate::Result<GetCardSmsDetailsResponse>> + Send {
        self.call(req)
    }

    pub fn query_card_sms_template_report(
        &self,
        req: QueryCardSmsTemplateReport,
    ) -> impl std::future::Future<Output = crate::Result<QueryCardSmsTemplateReportResponse>> + Send
    {
        async {
            todo!(
                r##"Parameter 'TemplateCodes': Unsupported ParameterStyle variant: Flat. Only Json style is supported.
Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn send_card_sms(
        &self,
        req: SendCardSms,
    ) -> impl std::future::Future<Output = crate::Result<SendCardSmsResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'CardObjects': Unsupported ParameterStyle variant: Flat. Only Json style is supported.
Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn send_batch_card_sms(
        &self,
        req: SendBatchCardSms,
    ) -> impl std::future::Future<Output = crate::Result<SendBatchCardSmsResponse>> + Send {
        async {
            todo!(
                r##"Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn get_qualification_oss_info(
        &self,
        req: GetQualificationOssInfo,
    ) -> impl std::future::Future<Output = crate::Result<GetQualificationOssInfoResponse>> + Send
    {
        self.call(req)
    }

    pub fn get_oss_info_for_upload_file(
        &self,
        req: GetOSSInfoForUploadFile,
    ) -> impl std::future::Future<Output = crate::Result<GetOSSInfoForUploadFileResponse>> + Send
    {
        self.call(req)
    }

    pub fn get_sms_ocr_oss_info(
        &self,
        req: GetSmsOcrOssInfo,
    ) -> impl std::future::Future<Output = crate::Result<GetSmsOcrOssInfoResponse>> + Send {
        self.call(req)
    }

    pub fn sms_conversion_intl(
        &self,
        req: SmsConversionIntl,
    ) -> impl std::future::Future<Output = crate::Result<SmsConversionIntlResponse>> + Send {
        self.call(req)
    }

    pub fn conversion_data_intl(
        &self,
        req: ConversionDataIntl,
    ) -> impl std::future::Future<Output = crate::Result<ConversionDataIntlResponse>> + Send {
        self.call(req)
    }

    pub fn add_short_url(
        &self,
        req: AddShortUrl,
    ) -> impl std::future::Future<Output = crate::Result<AddShortUrlResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'SourceUrl': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'ShortUrlName': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'EffectiveDays': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'SourceUrl': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'ShortUrlName': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'EffectiveDays': Unsupported ParameterIn variant: FormData. Only Query parameters are supported."##
            );
        }
    }

    pub fn delete_short_url(
        &self,
        req: DeleteShortUrl,
    ) -> impl std::future::Future<Output = crate::Result<DeleteShortUrlResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'SourceUrl': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'SourceUrl': Unsupported ParameterIn variant: FormData. Only Query parameters are supported."##
            );
        }
    }

    pub fn query_short_url(
        &self,
        req: QueryShortUrl,
    ) -> impl std::future::Future<Output = crate::Result<QueryShortUrlResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'ShortUrl': Unsupported ParameterIn variant: FormData. Only Query parameters are supported.
Parameter 'ShortUrl': Unsupported ParameterIn variant: FormData. Only Query parameters are supported."##
            );
        }
    }

    pub fn list_tag_resources(
        &self,
        req: ListTagResources,
    ) -> impl std::future::Future<Output = crate::Result<ListTagResourcesResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'Tag': Unsupported ParameterStyle variant: RepeatList. Only Json style is supported.
Parameter 'ResourceId': Unsupported ParameterStyle variant: RepeatList. Only Json style is supported.
Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn tag_resources(
        &self,
        req: TagResources,
    ) -> impl std::future::Future<Output = crate::Result<TagResourcesResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'Tag': Unsupported ParameterStyle variant: RepeatList. Only Json style is supported.
Parameter 'ResourceId': Unsupported ParameterStyle variant: RepeatList. Only Json style is supported.
Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }

    pub fn untag_resources(
        &self,
        req: UntagResources,
    ) -> impl std::future::Future<Output = crate::Result<UntagResourcesResponse>> + Send {
        async {
            todo!(
                r##"Parameter 'TagKey': Unsupported ParameterStyle variant: RepeatList. Only Json style is supported.
Parameter 'ResourceId': Unsupported ParameterStyle variant: RepeatList. Only Json style is supported.
Response struct error: Response must contain 'Message' field for CodeMessage"##
            );
        }
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SubmitSmsQualification {
    qualification_name: String,
    use_by_self: bool,
    company_type: String,
    #[setters(generate = true, strip_option)]
    business_license_pics: Option<Vec<SubmitSmsQualificationBusinessLicensePicsItem>>,
    company_name: String,
    organization_code: String,
    bussiness_license_exp_date: String,
    #[setters(generate = true, strip_option)]
    legal_person_id_card_front_side: Option<String>,
    #[setters(generate = true, strip_option)]
    legal_person_id_card_back_side: Option<String>,
    legal_person_name: String,
    legal_person_id_card_no: String,
    legal_person_id_card_eff_time: String,
    legal_person_id_card_type: String,
    admin_id_card_pic: String,
    admin_id_card_front_face: String,
    admin_name: String,
    admin_id_card_no: String,
    admin_id_card_type: String,
    admin_id_card_exp_date: String,
    admin_phone_no: String,
    certify_code: String,
    #[setters(generate = true, strip_option)]
    other_files: Option<Vec<SubmitSmsQualificationOtherFilesItem>>,
    whether_share: bool,
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

impl crate::Request for SubmitSmsQualification {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "SubmitSmsQualification";

    type Body = ();

    type Response = SubmitSmsQualificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("QualificationName", (&self.qualification_name).into());
        params.insert("UseBySelf", (&self.use_by_self).into());
        params.insert("CompanyType", (&self.company_type).into());
        params.insert("CompanyName", (&self.company_name).into());
        params.insert("OrganizationCode", (&self.organization_code).into());
        params.insert(
            "BussinessLicenseExpDate",
            (&self.bussiness_license_exp_date).into(),
        );
        params.insert("LegalPersonName", (&self.legal_person_name).into());
        params.insert(
            "LegalPersonIDCardNo",
            (&self.legal_person_id_card_no).into(),
        );
        params.insert(
            "LegalPersonIdCardEffTime",
            (&self.legal_person_id_card_eff_time).into(),
        );
        params.insert(
            "LegalPersonIDCardType",
            (&self.legal_person_id_card_type).into(),
        );
        params.insert("AdminIDCardPic", (&self.admin_id_card_pic).into());
        params.insert(
            "AdminIDCardFrontFace",
            (&self.admin_id_card_front_face).into(),
        );
        params.insert("AdminName", (&self.admin_name).into());
        params.insert("AdminIDCardNo", (&self.admin_id_card_no).into());
        params.insert("AdminIDCardType", (&self.admin_id_card_type).into());
        params.insert("AdminIDCardExpDate", (&self.admin_id_card_exp_date).into());
        params.insert("AdminPhoneNo", (&self.admin_phone_no).into());
        params.insert("CertifyCode", (&self.certify_code).into());
        params.insert("WhetherShare", (&self.whether_share).into());

        if let Some(f) = &self.business_license_pics {
            params.insert("BusinessLicensePics", serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.legal_person_id_card_front_side {
            params.insert("LegalPersonIdCardFrontSide", f.into());
        }

        if let Some(f) = &self.legal_person_id_card_back_side {
            params.insert("LegalPersonIdCardBackSide", f.into());
        }

        if let Some(f) = &self.other_files {
            params.insert("OtherFiles", serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.remark {
            params.insert("Remark", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySmsQualificationRecord {
    #[setters(generate = true, strip_option)]
    qualification_group_name: Option<String>,
    #[setters(generate = true, strip_option)]
    company_name: Option<String>,
    #[setters(generate = true, strip_option)]
    state: Option<String>,
    #[setters(generate = true, strip_option)]
    work_order_id: Option<i64>,
    #[setters(generate = true, strip_option)]
    legal_person_name: Option<String>,
    #[setters(generate = true, strip_option)]
    use_by_self: Option<bool>,
    #[setters(generate = true, strip_option)]
    page_no: Option<i64>,
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

impl crate::Request for QuerySmsQualificationRecord {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySmsQualificationRecord";

    type Body = ();

    type Response = QuerySmsQualificationRecordResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.qualification_group_name {
            params.insert("QualificationGroupName", f.into());
        }

        if let Some(f) = &self.company_name {
            params.insert("CompanyName", f.into());
        }

        if let Some(f) = &self.state {
            params.insert("State", f.into());
        }

        if let Some(f) = &self.work_order_id {
            params.insert("WorkOrderId", f.into());
        }

        if let Some(f) = &self.legal_person_name {
            params.insert("LegalPersonName", f.into());
        }

        if let Some(f) = &self.use_by_self {
            params.insert("UseBySelf", f.into());
        }

        if let Some(f) = &self.page_no {
            params.insert("PageNo", f.into());
        }

        if let Some(f) = &self.page_size {
            params.insert("PageSize", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySingleSmsQualification {
    qualification_group_id: i64,
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

impl crate::Request for QuerySingleSmsQualification {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySingleSmsQualification";

    type Body = ();

    type Response = QuerySingleSmsQualificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "QualificationGroupId",
            (&self.qualification_group_id).into(),
        );

        if let Some(f) = &self.order_id {
            params.insert("OrderId", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateSmsQualification {
    qualification_group_id: i64,
    order_id: i64,
    #[setters(generate = true, strip_option)]
    business_license_pics: Option<Vec<UpdateSmsQualificationBusinessLicensePicsItem>>,
    #[setters(generate = true, strip_option)]
    company_name: Option<String>,
    #[setters(generate = true, strip_option)]
    bussiness_license_exp_date: Option<String>,
    #[setters(generate = true, strip_option)]
    legal_person_id_card_front_side: Option<String>,
    #[setters(generate = true, strip_option)]
    legal_person_id_card_back_side: Option<String>,
    #[setters(generate = true, strip_option)]
    legal_person_name: Option<String>,
    #[setters(generate = true, strip_option)]
    legal_person_id_card_no: Option<String>,
    #[setters(generate = true, strip_option)]
    legal_person_id_card_eff_time: Option<String>,
    #[setters(generate = true, strip_option)]
    legal_person_id_card_type: Option<String>,
    #[setters(generate = true, strip_option)]
    admin_id_card_pic: Option<String>,
    #[setters(generate = true, strip_option)]
    admin_id_card_front_face: Option<String>,
    #[setters(generate = true, strip_option)]
    admin_name: Option<String>,
    #[setters(generate = true, strip_option)]
    admin_id_card_no: Option<String>,
    #[setters(generate = true, strip_option)]
    admin_id_card_exp_date: Option<String>,
    admin_phone_no: String,
    certify_code: String,
    #[setters(generate = true, strip_option)]
    admin_id_card_type: Option<String>,
    #[setters(generate = true, strip_option)]
    other_files: Option<Vec<UpdateSmsQualificationOtherFilesItem>>,
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

impl crate::Request for UpdateSmsQualification {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "UpdateSmsQualification";

    type Body = ();

    type Response = UpdateSmsQualificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "QualificationGroupId",
            (&self.qualification_group_id).into(),
        );
        params.insert("OrderId", (&self.order_id).into());
        params.insert("AdminPhoneNo", (&self.admin_phone_no).into());
        params.insert("CertifyCode", (&self.certify_code).into());

        if let Some(f) = &self.business_license_pics {
            params.insert("BusinessLicensePics", serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.company_name {
            params.insert("CompanyName", f.into());
        }

        if let Some(f) = &self.bussiness_license_exp_date {
            params.insert("BussinessLicenseExpDate", f.into());
        }

        if let Some(f) = &self.legal_person_id_card_front_side {
            params.insert("LegalPersonIdCardFrontSide", f.into());
        }

        if let Some(f) = &self.legal_person_id_card_back_side {
            params.insert("LegalPersonIdCardBackSide", f.into());
        }

        if let Some(f) = &self.legal_person_name {
            params.insert("LegalPersonName", f.into());
        }

        if let Some(f) = &self.legal_person_id_card_no {
            params.insert("LegalPersonIDCardNo", f.into());
        }

        if let Some(f) = &self.legal_person_id_card_eff_time {
            params.insert("LegalPersonIdCardEffTime", f.into());
        }

        if let Some(f) = &self.legal_person_id_card_type {
            params.insert("LegalPersonIDCardType", f.into());
        }

        if let Some(f) = &self.admin_id_card_pic {
            params.insert("AdminIDCardPic", f.into());
        }

        if let Some(f) = &self.admin_id_card_front_face {
            params.insert("AdminIDCardFrontFace", f.into());
        }

        if let Some(f) = &self.admin_name {
            params.insert("AdminName", f.into());
        }

        if let Some(f) = &self.admin_id_card_no {
            params.insert("AdminIDCardNo", f.into());
        }

        if let Some(f) = &self.admin_id_card_exp_date {
            params.insert("AdminIDCardExpDate", f.into());
        }

        if let Some(f) = &self.admin_id_card_type {
            params.insert("AdminIDCardType", f.into());
        }

        if let Some(f) = &self.other_files {
            params.insert("OtherFiles", serde_json::to_string(f)?.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteSmsQualification {
    qualification_group_id: i64,
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

impl crate::Request for DeleteSmsQualification {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteSmsQualification";

    type Body = ();

    type Response = DeleteSmsQualificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "QualificationGroupId",
            (&self.qualification_group_id).into(),
        );
        params.insert("OrderId", (&self.order_id).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RequiredPhoneCode {
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

impl crate::Request for RequiredPhoneCode {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "RequiredPhoneCode";

    type Body = ();

    type Response = RequiredPhoneCodeResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("PhoneNo", (&self.phone_no).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ValidPhoneCode {
    phone_no: String,
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

impl crate::Request for ValidPhoneCode {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ValidPhoneCode";

    type Body = ();

    type Response = ValidPhoneCodeResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("PhoneNo", (&self.phone_no).into());
        params.insert("CertifyCode", (&self.certify_code).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateSmsAuthorizationLetter {
    authorization_letter_pic: String,
    sign_list: Vec<String>,
    authorization_letter_exp_date: String,
    authorization: String,
    organization_code: String,
    proxy_authorization: String,
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

impl crate::Request for CreateSmsAuthorizationLetter {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CreateSmsAuthorizationLetter";

    type Body = ();

    type Response = CreateSmsAuthorizationLetterResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "AuthorizationLetterPic",
            (&self.authorization_letter_pic).into(),
        );
        params.insert("SignList", serde_json::to_string(&self.sign_list)?.into());
        params.insert(
            "AuthorizationLetterExpDate",
            (&self.authorization_letter_exp_date).into(),
        );
        params.insert("Authorization", (&self.authorization).into());
        params.insert("OrganizationCode", (&self.organization_code).into());
        params.insert("ProxyAuthorization", (&self.proxy_authorization).into());
        params.insert(
            "AuthorizationLetterName",
            (&self.authorization_letter_name).into(),
        );

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySmsAuthorizationLetter {
    #[setters(generate = true, strip_option)]
    authorization_letter_id_list: Option<Vec<i64>>,
    #[setters(generate = true, strip_option)]
    sign_name: Option<String>,
    #[setters(generate = true, strip_option)]
    organization_code: Option<String>,
    #[setters(generate = true, strip_option)]
    state: Option<String>,
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

impl crate::Request for QuerySmsAuthorizationLetter {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySmsAuthorizationLetter";

    type Body = ();

    type Response = QuerySmsAuthorizationLetterResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.authorization_letter_id_list {
            params.insert(
                "AuthorizationLetterIdList",
                serde_json::to_string(f)?.into(),
            );
        }

        if let Some(f) = &self.sign_name {
            params.insert("SignName", f.into());
        }

        if let Some(f) = &self.organization_code {
            params.insert("OrganizationCode", f.into());
        }

        if let Some(f) = &self.state {
            params.insert("State", f.into());
        }

        if let Some(f) = &self.status {
            params.insert("Status", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateSmsSign {
    sign_name: String,
    #[setters(generate = true, strip_option)]
    remark: Option<String>,
    #[setters(generate = true, strip_option)]
    sign_type: Option<i32>,
    #[setters(generate = true, strip_option)]
    more_data: Option<Vec<String>>,
    qualification_id: i64,
    #[setters(generate = true, strip_option)]
    apply_scene_content: Option<String>,
    sign_source: i32,
    #[setters(generate = true, strip_option)]
    third_party: Option<bool>,
    #[setters(generate = true, strip_option)]
    authorization_letter_id: Option<i64>,
    #[setters(generate = true, strip_option)]
    trademark_id: Option<i64>,
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

impl crate::Request for CreateSmsSign {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CreateSmsSign";

    type Body = ();

    type Response = CreateSmsSignResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SignName", (&self.sign_name).into());
        params.insert("QualificationId", (&self.qualification_id).into());
        params.insert("SignSource", (&self.sign_source).into());

        if let Some(f) = &self.remark {
            params.insert("Remark", f.into());
        }

        if let Some(f) = &self.sign_type {
            params.insert("SignType", f.into());
        }

        if let Some(f) = &self.more_data {
            params.insert("MoreData", serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.apply_scene_content {
            params.insert("ApplySceneContent", f.into());
        }

        if let Some(f) = &self.third_party {
            params.insert("ThirdParty", f.into());
        }

        if let Some(f) = &self.authorization_letter_id {
            params.insert("AuthorizationLetterId", f.into());
        }

        if let Some(f) = &self.trademark_id {
            params.insert("TrademarkId", f.into());
        }

        if let Some(f) = &self.app_icp_record_id {
            params.insert("AppIcpRecordId", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetSmsSign {
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

impl crate::Request for GetSmsSign {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetSmsSign";

    type Body = ();

    type Response = GetSmsSignResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SignName", (&self.sign_name).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySmsSignList {
    #[setters(generate = true, strip_option)]
    page_index: Option<i32>,
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

impl crate::Request for QuerySmsSignList {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySmsSignList";

    type Body = ();

    type Response = QuerySmsSignListResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.page_index {
            params.insert("PageIndex", f.into());
        }

        if let Some(f) = &self.page_size {
            params.insert("PageSize", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateSmsSign {
    sign_name: String,
    #[setters(generate = true, strip_option)]
    remark: Option<String>,
    #[setters(generate = true, strip_option)]
    sign_type: Option<i32>,
    #[setters(generate = true, strip_option)]
    more_data: Option<Vec<String>>,
    qualification_id: i64,
    #[setters(generate = true, strip_option)]
    apply_scene_content: Option<String>,
    sign_source: i32,
    #[setters(generate = true, strip_option)]
    third_party: Option<bool>,
    #[setters(generate = true, strip_option)]
    authorization_letter_id: Option<i64>,
    #[setters(generate = true, strip_option)]
    trademark_id: Option<i64>,
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

impl crate::Request for UpdateSmsSign {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "UpdateSmsSign";

    type Body = ();

    type Response = UpdateSmsSignResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SignName", (&self.sign_name).into());
        params.insert("QualificationId", (&self.qualification_id).into());
        params.insert("SignSource", (&self.sign_source).into());

        if let Some(f) = &self.remark {
            params.insert("Remark", f.into());
        }

        if let Some(f) = &self.sign_type {
            params.insert("SignType", f.into());
        }

        if let Some(f) = &self.more_data {
            params.insert("MoreData", serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.apply_scene_content {
            params.insert("ApplySceneContent", f.into());
        }

        if let Some(f) = &self.third_party {
            params.insert("ThirdParty", f.into());
        }

        if let Some(f) = &self.authorization_letter_id {
            params.insert("AuthorizationLetterId", f.into());
        }

        if let Some(f) = &self.trademark_id {
            params.insert("TrademarkId", f.into());
        }

        if let Some(f) = &self.app_icp_record_id {
            params.insert("AppIcpRecordId", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteSmsSign {
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

impl crate::Request for DeleteSmsSign {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteSmsSign";

    type Body = ();

    type Response = DeleteSmsSignResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SignName", (&self.sign_name).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ChangeSignatureQualification {
    signature_name: String,
    qualification_id: i64,
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

impl crate::Request for ChangeSignatureQualification {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ChangeSignatureQualification";

    type Body = ();

    type Response = ChangeSignatureQualificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SignatureName", (&self.signature_name).into());
        params.insert("QualificationId", (&self.qualification_id).into());

        if let Some(f) = &self.authorization_letter_id {
            params.insert("AuthorizationLetterId", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AddSmsSign {
    sign_name: String,
    sign_source: i32,
    remark: String,
    #[setters(generate = true, strip_option)]
    sign_type: Option<i32>,
}

impl sealed::Bound for AddSmsSign {}

impl AddSmsSign {
    pub fn new(
        sign_name: impl Into<String>,
        sign_source: impl Into<i32>,
        remark: impl Into<String>,
    ) -> Self {
        Self {
            sign_name: sign_name.into(),
            sign_source: sign_source.into(),
            remark: remark.into(),
            sign_type: None,
        }
    }
}

impl crate::Request for AddSmsSign {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "AddSmsSign";

    type Body = ();

    type Response = AddSmsSignResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SignName", (&self.sign_name).into());
        params.insert("SignSource", (&self.sign_source).into());
        params.insert("Remark", (&self.remark).into());

        if let Some(f) = &self.sign_type {
            params.insert("SignType", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ModifySmsSign {
    sign_name: String,
    sign_source: i32,
    remark: String,
    #[setters(generate = true, strip_option)]
    sign_type: Option<i32>,
}

impl sealed::Bound for ModifySmsSign {}

impl ModifySmsSign {
    pub fn new(
        sign_name: impl Into<String>,
        sign_source: impl Into<i32>,
        remark: impl Into<String>,
    ) -> Self {
        Self {
            sign_name: sign_name.into(),
            sign_source: sign_source.into(),
            remark: remark.into(),
            sign_type: None,
        }
    }
}

impl crate::Request for ModifySmsSign {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ModifySmsSign";

    type Body = ();

    type Response = ModifySmsSignResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SignName", (&self.sign_name).into());
        params.insert("SignSource", (&self.sign_source).into());
        params.insert("Remark", (&self.remark).into());

        if let Some(f) = &self.sign_type {
            params.insert("SignType", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySmsSign {
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

impl crate::Request for QuerySmsSign {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySmsSign";

    type Body = ();

    type Response = QuerySmsSignResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SignName", (&self.sign_name).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateSmsTrademark {
    trademark_pic: String,
    trademark_registration_number: String,
    trademark_name: String,
    trademark_applicant_name: String,
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

impl crate::Request for CreateSmsTrademark {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CreateSmsTrademark";

    type Body = ();

    type Response = CreateSmsTrademarkResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TrademarkPic", (&self.trademark_pic).into());
        params.insert(
            "TrademarkRegistrationNumber",
            (&self.trademark_registration_number).into(),
        );
        params.insert("TrademarkName", (&self.trademark_name).into());
        params.insert(
            "TrademarkApplicantName",
            (&self.trademark_applicant_name).into(),
        );
        params.insert("TrademarkEffExpDate", (&self.trademark_eff_exp_date).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySmsTrademark {
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

impl crate::Request for QuerySmsTrademark {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySmsTrademark";

    type Body = ();

    type Response = QuerySmsTrademarkResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "TrademarkIdList",
            serde_json::to_string(&self.trademark_id_list)?.into(),
        );

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateSmsAppIcpRecord {
    app_icp_record_pic: String,
    app_service_name: String,
    app_principal_unit_name: String,
    app_icp_license_number: String,
    app_approval_date: String,
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

impl crate::Request for CreateSmsAppIcpRecord {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CreateSmsAppIcpRecord";

    type Body = ();

    type Response = CreateSmsAppIcpRecordResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("AppIcpRecordPic", (&self.app_icp_record_pic).into());
        params.insert("AppServiceName", (&self.app_service_name).into());
        params.insert(
            "AppPrincipalUnitName",
            (&self.app_principal_unit_name).into(),
        );
        params.insert("AppIcpLicenseNumber", (&self.app_icp_license_number).into());
        params.insert("AppApprovalDate", (&self.app_approval_date).into());
        params.insert("Domain", (&self.domain).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySmsAppIcpRecord {
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

impl crate::Request for QuerySmsAppIcpRecord {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySmsAppIcpRecord";

    type Body = ();

    type Response = QuerySmsAppIcpRecordResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert(
            "AppIcpRecordIdList",
            serde_json::to_string(&self.app_icp_record_id_list)?.into(),
        );

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateSmsTemplate {
    template_name: String,
    template_content: String,
    #[setters(generate = true, strip_option)]
    remark: Option<String>,
    template_type: i32,
    #[setters(generate = true, strip_option)]
    related_sign_name: Option<String>,
    #[setters(generate = true, strip_option)]
    template_rule: Option<String>,
    #[setters(generate = true, strip_option)]
    more_data: Option<Vec<String>>,
    #[setters(generate = true, strip_option)]
    apply_scene_content: Option<String>,
    #[setters(generate = true, strip_option)]
    intl_type: Option<i32>,
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

impl crate::Request for CreateSmsTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CreateSmsTemplate";

    type Body = ();

    type Response = CreateSmsTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateName", (&self.template_name).into());
        params.insert("TemplateContent", (&self.template_content).into());
        params.insert("TemplateType", (&self.template_type).into());

        if let Some(f) = &self.remark {
            params.insert("Remark", f.into());
        }

        if let Some(f) = &self.related_sign_name {
            params.insert("RelatedSignName", f.into());
        }

        if let Some(f) = &self.template_rule {
            params.insert("TemplateRule", f.into());
        }

        if let Some(f) = &self.more_data {
            params.insert("MoreData", serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.apply_scene_content {
            params.insert("ApplySceneContent", f.into());
        }

        if let Some(f) = &self.intl_type {
            params.insert("IntlType", f.into());
        }

        if let Some(f) = &self.traffic_driving {
            params.insert("TrafficDriving", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetSmsTemplate {
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

impl crate::Request for GetSmsTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetSmsTemplate";

    type Body = ();

    type Response = GetSmsTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateCode", (&self.template_code).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySmsTemplateList {
    #[setters(generate = true, strip_option)]
    page_index: Option<i32>,
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

impl crate::Request for QuerySmsTemplateList {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySmsTemplateList";

    type Body = ();

    type Response = QuerySmsTemplateListResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.page_index {
            params.insert("PageIndex", f.into());
        }

        if let Some(f) = &self.page_size {
            params.insert("PageSize", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UpdateSmsTemplate {
    template_name: String,
    template_code: String,
    template_content: String,
    #[setters(generate = true, strip_option)]
    remark: Option<String>,
    template_type: i32,
    #[setters(generate = true, strip_option)]
    related_sign_name: Option<String>,
    #[setters(generate = true, strip_option)]
    template_rule: Option<String>,
    #[setters(generate = true, strip_option)]
    more_data: Option<Vec<String>>,
    #[setters(generate = true, strip_option)]
    apply_scene_content: Option<String>,
    #[setters(generate = true, strip_option)]
    intl_type: Option<i32>,
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

impl crate::Request for UpdateSmsTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "UpdateSmsTemplate";

    type Body = ();

    type Response = UpdateSmsTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateName", (&self.template_name).into());
        params.insert("TemplateCode", (&self.template_code).into());
        params.insert("TemplateContent", (&self.template_content).into());
        params.insert("TemplateType", (&self.template_type).into());

        if let Some(f) = &self.remark {
            params.insert("Remark", f.into());
        }

        if let Some(f) = &self.related_sign_name {
            params.insert("RelatedSignName", f.into());
        }

        if let Some(f) = &self.template_rule {
            params.insert("TemplateRule", f.into());
        }

        if let Some(f) = &self.more_data {
            params.insert("MoreData", serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.apply_scene_content {
            params.insert("ApplySceneContent", f.into());
        }

        if let Some(f) = &self.intl_type {
            params.insert("IntlType", f.into());
        }

        if let Some(f) = &self.traffic_driving {
            params.insert("TrafficDriving", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteSmsTemplate {
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

impl crate::Request for DeleteSmsTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteSmsTemplate";

    type Body = ();

    type Response = DeleteSmsTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateCode", (&self.template_code).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AddSmsTemplate {
    template_type: i32,
    template_name: String,
    template_content: String,
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

impl crate::Request for AddSmsTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "AddSmsTemplate";

    type Body = ();

    type Response = AddSmsTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateType", (&self.template_type).into());
        params.insert("TemplateName", (&self.template_name).into());
        params.insert("TemplateContent", (&self.template_content).into());
        params.insert("Remark", (&self.remark).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ModifySmsTemplate {
    template_type: i32,
    template_name: String,
    template_code: String,
    template_content: String,
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

impl crate::Request for ModifySmsTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ModifySmsTemplate";

    type Body = ();

    type Response = ModifySmsTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateType", (&self.template_type).into());
        params.insert("TemplateName", (&self.template_name).into());
        params.insert("TemplateCode", (&self.template_code).into());
        params.insert("TemplateContent", (&self.template_content).into());
        params.insert("Remark", (&self.remark).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySmsTemplate {
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

impl crate::Request for QuerySmsTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySmsTemplate";

    type Body = ();

    type Response = QuerySmsTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateCode", (&self.template_code).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SendSms {
    phone_numbers: String,
    sign_name: String,
    template_code: String,
    #[setters(generate = true, strip_option)]
    template_param: Option<String>,
    #[setters(generate = true, strip_option)]
    sms_up_extend_code: Option<String>,
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

impl crate::Request for SendSms {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "SendSms";

    type Body = ();

    type Response = SendSmsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("PhoneNumbers", (&self.phone_numbers).into());
        params.insert("SignName", (&self.sign_name).into());
        params.insert("TemplateCode", (&self.template_code).into());

        if let Some(f) = &self.template_param {
            params.insert("TemplateParam", f.into());
        }

        if let Some(f) = &self.sms_up_extend_code {
            params.insert("SmsUpExtendCode", f.into());
        }

        if let Some(f) = &self.out_id {
            params.insert("OutId", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SendBatchSms {
    template_code: String,
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
}

impl sealed::Bound for SendBatchSms {}

impl SendBatchSms {
    pub fn new(template_code: impl Into<String>) -> Self {
        Self {
            template_code: template_code.into(),
            out_id: None,
        }
    }
}

impl crate::Request for SendBatchSms {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "SendBatchSms";

    type Body = ();

    type Response = SendBatchSmsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateCode", (&self.template_code).into());

        if let Some(f) = &self.out_id {
            params.insert("OutId", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySendDetails {
    phone_number: String,
    #[setters(generate = true, strip_option)]
    biz_id: Option<String>,
    send_date: String,
    page_size: i64,
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

impl crate::Request for QuerySendDetails {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySendDetails";

    type Body = ();

    type Response = QuerySendDetailsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("PhoneNumber", (&self.phone_number).into());
        params.insert("SendDate", (&self.send_date).into());
        params.insert("PageSize", (&self.page_size).into());
        params.insert("CurrentPage", (&self.current_page).into());

        if let Some(f) = &self.biz_id {
            params.insert("BizId", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QuerySendStatistics {
    is_globe: i32,
    start_date: String,
    end_date: String,
    page_index: i32,
    page_size: i32,
    #[setters(generate = true, strip_option)]
    template_type: Option<i32>,
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

impl crate::Request for QuerySendStatistics {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySendStatistics";

    type Body = ();

    type Response = QuerySendStatisticsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("IsGlobe", (&self.is_globe).into());
        params.insert("StartDate", (&self.start_date).into());
        params.insert("EndDate", (&self.end_date).into());
        params.insert("PageIndex", (&self.page_index).into());
        params.insert("PageSize", (&self.page_size).into());

        if let Some(f) = &self.template_type {
            params.insert("TemplateType", f.into());
        }

        if let Some(f) = &self.sign_name {
            params.insert("SignName", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetOSSInfoForCardTemplate {}

impl sealed::Bound for GetOSSInfoForCardTemplate {}

impl GetOSSInfoForCardTemplate {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for GetOSSInfoForCardTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetOSSInfoForCardTemplate";

    type Body = ();

    type Response = GetOSSInfoForCardTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetMediaResourceId {
    resource_type: i32,
    oss_key: String,
    file_size: i64,
    #[setters(generate = true, strip_option)]
    extend_info: Option<String>,
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

impl crate::Request for GetMediaResourceId {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetMediaResourceId";

    type Body = ();

    type Response = GetMediaResourceIdResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("ResourceType", (&self.resource_type).into());
        params.insert("OssKey", (&self.oss_key).into());
        params.insert("FileSize", (&self.file_size).into());

        if let Some(f) = &self.extend_info {
            params.insert("ExtendInfo", f.into());
        }

        if let Some(f) = &self.memo {
            params.insert("Memo", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CreateCardSmsTemplate {
    template_name: String,
    template: CreateCardSmsTemplateTemplate,
    #[setters(generate = true, strip_option)]
    memo: Option<String>,
    #[setters(generate = true, strip_option)]
    factorys: Option<String>,
}

impl sealed::Bound for CreateCardSmsTemplate {}

impl CreateCardSmsTemplate {
    pub fn new(
        template_name: impl Into<String>,
        template: impl Into<CreateCardSmsTemplateTemplate>,
    ) -> Self {
        Self {
            template_name: template_name.into(),
            template: template.into(),
            memo: None,
            factorys: None,
        }
    }
}

impl crate::Request for CreateCardSmsTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CreateCardSmsTemplate";

    type Body = ();

    type Response = CreateCardSmsTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateName", (&self.template_name).into());
        params.insert("Template", serde_json::to_string(&self.template)?.into());

        if let Some(f) = &self.memo {
            params.insert("Memo", f.into());
        }

        if let Some(f) = &self.factorys {
            params.insert("Factorys", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QueryCardSmsTemplate {
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

impl crate::Request for QueryCardSmsTemplate {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QueryCardSmsTemplate";

    type Body = ();

    type Response = QueryCardSmsTemplateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateCode", (&self.template_code).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct CheckMobilesCardSupport {
    template_code: String,
}

impl sealed::Bound for CheckMobilesCardSupport {}

impl CheckMobilesCardSupport {
    pub fn new(template_code: impl Into<String>) -> Self {
        Self {
            template_code: template_code.into(),
        }
    }
}

impl crate::Request for CheckMobilesCardSupport {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "CheckMobilesCardSupport";

    type Body = ();

    type Response = CheckMobilesCardSupportResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateCode", (&self.template_code).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QueryMobilesCardSupport {
    template_code: String,
    mobiles: Vec<QueryMobilesCardSupportMobilesItem>,
    #[setters(generate = true, strip_option)]
    encrypt_type: Option<String>,
}

impl sealed::Bound for QueryMobilesCardSupport {}

impl QueryMobilesCardSupport {
    pub fn new(
        template_code: impl Into<String>,
        mobiles: impl Into<Vec<QueryMobilesCardSupportMobilesItem>>,
    ) -> Self {
        Self {
            template_code: template_code.into(),
            mobiles: mobiles.into(),
            encrypt_type: None,
        }
    }
}

impl crate::Request for QueryMobilesCardSupport {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QueryMobilesCardSupport";

    type Body = ();

    type Response = QueryMobilesCardSupportResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("TemplateCode", (&self.template_code).into());
        params.insert("Mobiles", serde_json::to_string(&self.mobiles)?.into());

        if let Some(f) = &self.encrypt_type {
            params.insert("EncryptType", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetCardSmsLink {
    card_template_code: String,
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
    #[setters(generate = true, strip_option)]
    phone_number_json: Option<String>,
    sign_name_json: String,
    #[setters(generate = true, strip_option)]
    card_template_param_json: Option<String>,
    #[setters(generate = true, strip_option)]
    card_code_type: Option<i32>,
    #[setters(generate = true, strip_option)]
    card_link_type: Option<i32>,
    #[setters(generate = true, strip_option)]
    domain: Option<String>,
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

impl crate::Request for GetCardSmsLink {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetCardSmsLink";

    type Body = ();

    type Response = GetCardSmsLinkResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("CardTemplateCode", (&self.card_template_code).into());
        params.insert("SignNameJson", (&self.sign_name_json).into());

        if let Some(f) = &self.out_id {
            params.insert("OutId", f.into());
        }

        if let Some(f) = &self.phone_number_json {
            params.insert("PhoneNumberJson", f.into());
        }

        if let Some(f) = &self.card_template_param_json {
            params.insert("CardTemplateParamJson", f.into());
        }

        if let Some(f) = &self.card_code_type {
            params.insert("CardCodeType", f.into());
        }

        if let Some(f) = &self.card_link_type {
            params.insert("CardLinkType", f.into());
        }

        if let Some(f) = &self.domain {
            params.insert("Domain", f.into());
        }

        if let Some(f) = &self.custom_short_code_json {
            params.insert("CustomShortCodeJson", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetCardSmsDetails {
    #[setters(generate = true, strip_option)]
    current_page: Option<i64>,
    #[setters(generate = true, strip_option)]
    page_size: Option<i64>,
    send_date: String,
    phone_number: String,
    #[setters(generate = true, strip_option)]
    biz_card_id: Option<String>,
    #[setters(generate = true, strip_option)]
    biz_sms_id: Option<String>,
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

impl crate::Request for GetCardSmsDetails {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetCardSmsDetails";

    type Body = ();

    type Response = GetCardSmsDetailsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SendDate", (&self.send_date).into());
        params.insert("PhoneNumber", (&self.phone_number).into());

        if let Some(f) = &self.current_page {
            params.insert("CurrentPage", f.into());
        }

        if let Some(f) = &self.page_size {
            params.insert("PageSize", f.into());
        }

        if let Some(f) = &self.biz_card_id {
            params.insert("BizCardId", f.into());
        }

        if let Some(f) = &self.biz_sms_id {
            params.insert("BizSmsId", f.into());
        }

        if let Some(f) = &self.biz_digit_id {
            params.insert("BizDigitId", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QueryCardSmsTemplateReport {
    start_date: String,
    end_date: String,
}

impl sealed::Bound for QueryCardSmsTemplateReport {}

impl QueryCardSmsTemplateReport {
    pub fn new(start_date: impl Into<String>, end_date: impl Into<String>) -> Self {
        Self {
            start_date: start_date.into(),
            end_date: end_date.into(),
        }
    }
}

impl crate::Request for QueryCardSmsTemplateReport {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QueryCardSmsTemplateReport";

    type Body = ();

    type Response = QueryCardSmsTemplateReportResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("StartDate", (&self.start_date).into());
        params.insert("EndDate", (&self.end_date).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SendCardSms {
    sign_name: String,
    card_template_code: String,
    #[setters(generate = true, strip_option)]
    sms_template_code: Option<String>,
    #[setters(generate = true, strip_option)]
    sms_up_extend_code: Option<String>,
    fallback_type: String,
    #[setters(generate = true, strip_option)]
    digital_template_code: Option<String>,
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
    #[setters(generate = true, strip_option)]
    sms_template_param: Option<String>,
    #[setters(generate = true, strip_option)]
    digital_template_param: Option<String>,
    #[setters(generate = true, strip_option)]
    template_code: Option<String>,
    #[setters(generate = true, strip_option)]
    template_param: Option<String>,
}

impl sealed::Bound for SendCardSms {}

impl SendCardSms {
    pub fn new(
        sign_name: impl Into<String>,
        card_template_code: impl Into<String>,
        fallback_type: impl Into<String>,
    ) -> Self {
        Self {
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

impl crate::Request for SendCardSms {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "SendCardSms";

    type Body = ();

    type Response = SendCardSmsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("SignName", (&self.sign_name).into());
        params.insert("CardTemplateCode", (&self.card_template_code).into());
        params.insert("FallbackType", (&self.fallback_type).into());

        if let Some(f) = &self.sms_template_code {
            params.insert("SmsTemplateCode", f.into());
        }

        if let Some(f) = &self.sms_up_extend_code {
            params.insert("SmsUpExtendCode", f.into());
        }

        if let Some(f) = &self.digital_template_code {
            params.insert("DigitalTemplateCode", f.into());
        }

        if let Some(f) = &self.out_id {
            params.insert("OutId", f.into());
        }

        if let Some(f) = &self.sms_template_param {
            params.insert("SmsTemplateParam", f.into());
        }

        if let Some(f) = &self.digital_template_param {
            params.insert("DigitalTemplateParam", f.into());
        }

        if let Some(f) = &self.template_code {
            params.insert("TemplateCode", f.into());
        }

        if let Some(f) = &self.template_param {
            params.insert("TemplateParam", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SendBatchCardSms {
    card_template_code: String,
    #[setters(generate = true, strip_option)]
    sms_template_code: Option<String>,
    fallback_type: String,
    #[setters(generate = true, strip_option)]
    digital_template_code: Option<String>,
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
    phone_number_json: String,
    sign_name_json: String,
    #[setters(generate = true, strip_option)]
    card_template_param_json: Option<String>,
    #[setters(generate = true, strip_option)]
    sms_template_param_json: Option<String>,
    #[setters(generate = true, strip_option)]
    digital_template_param_json: Option<String>,
    #[setters(generate = true, strip_option)]
    sms_up_extend_code_json: Option<String>,
    #[setters(generate = true, strip_option)]
    template_code: Option<String>,
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

impl crate::Request for SendBatchCardSms {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "SendBatchCardSms";

    type Body = ();

    type Response = SendBatchCardSmsResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("CardTemplateCode", (&self.card_template_code).into());
        params.insert("FallbackType", (&self.fallback_type).into());
        params.insert("PhoneNumberJson", (&self.phone_number_json).into());
        params.insert("SignNameJson", (&self.sign_name_json).into());

        if let Some(f) = &self.sms_template_code {
            params.insert("SmsTemplateCode", f.into());
        }

        if let Some(f) = &self.digital_template_code {
            params.insert("DigitalTemplateCode", f.into());
        }

        if let Some(f) = &self.out_id {
            params.insert("OutId", f.into());
        }

        if let Some(f) = &self.card_template_param_json {
            params.insert("CardTemplateParamJson", f.into());
        }

        if let Some(f) = &self.sms_template_param_json {
            params.insert("SmsTemplateParamJson", f.into());
        }

        if let Some(f) = &self.digital_template_param_json {
            params.insert("DigitalTemplateParamJson", f.into());
        }

        if let Some(f) = &self.sms_up_extend_code_json {
            params.insert("SmsUpExtendCodeJson", f.into());
        }

        if let Some(f) = &self.template_code {
            params.insert("TemplateCode", f.into());
        }

        if let Some(f) = &self.template_param_json {
            params.insert("TemplateParamJson", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetQualificationOssInfo {
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

impl crate::Request for GetQualificationOssInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetQualificationOssInfo";

    type Body = ();

    type Response = GetQualificationOssInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("BizType", (&self.biz_type).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetOSSInfoForUploadFile {
    #[setters(generate = true, strip_option)]
    biz_type: Option<String>,
}

impl sealed::Bound for GetOSSInfoForUploadFile {}

impl GetOSSInfoForUploadFile {
    pub fn new() -> Self {
        Self { biz_type: None }
    }
}

impl crate::Request for GetOSSInfoForUploadFile {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetOSSInfoForUploadFile";

    type Body = ();

    type Response = GetOSSInfoForUploadFileResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.biz_type {
            params.insert("BizType", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct GetSmsOcrOssInfo {
    #[setters(generate = true, strip_option)]
    task_type: Option<String>,
}

impl sealed::Bound for GetSmsOcrOssInfo {}

impl GetSmsOcrOssInfo {
    pub fn new() -> Self {
        Self { task_type: None }
    }
}

impl crate::Request for GetSmsOcrOssInfo {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "GetSmsOcrOssInfo";

    type Body = ();

    type Response = GetSmsOcrOssInfoResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.task_type {
            params.insert("TaskType", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SmsConversionIntl {
    message_id: String,
    delivered: bool,
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

impl crate::Request for SmsConversionIntl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "SmsConversionIntl";

    type Body = ();

    type Response = SmsConversionIntlResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("MessageId", (&self.message_id).into());
        params.insert("Delivered", (&self.delivered).into());

        if let Some(f) = &self.conversion_time {
            params.insert("ConversionTime", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ConversionDataIntl {
    #[setters(generate = true, strip_option)]
    report_time: Option<i64>,
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

impl crate::Request for ConversionDataIntl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ConversionDataIntl";

    type Body = ();

    type Response = ConversionDataIntlResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("ConversionRate", (&self.conversion_rate).into());

        if let Some(f) = &self.report_time {
            params.insert("ReportTime", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct AddShortUrl {}

impl sealed::Bound for AddShortUrl {}

impl AddShortUrl {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for AddShortUrl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "AddShortUrl";

    type Body = ();

    type Response = AddShortUrlResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct DeleteShortUrl {}

impl sealed::Bound for DeleteShortUrl {}

impl DeleteShortUrl {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for DeleteShortUrl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "DeleteShortUrl";

    type Body = ();

    type Response = DeleteShortUrlResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct QueryShortUrl {}

impl sealed::Bound for QueryShortUrl {}

impl QueryShortUrl {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Request for QueryShortUrl {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QueryShortUrl";

    type Body = ();

    type Response = QueryShortUrlResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct ListTagResources {
    resource_type: String,
    region_id: String,
    #[setters(generate = true, strip_option)]
    next_token: Option<String>,
    #[setters(generate = true, strip_option)]
    page_size: Option<i32>,
    #[setters(generate = true, strip_option)]
    prod_code: Option<String>,
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
        }
    }
}

impl crate::Request for ListTagResources {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "ListTagResources";

    type Body = ();

    type Response = ListTagResourcesResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("ResourceType", (&self.resource_type).into());
        params.insert("RegionId", (&self.region_id).into());

        if let Some(f) = &self.next_token {
            params.insert("NextToken", f.into());
        }

        if let Some(f) = &self.page_size {
            params.insert("PageSize", f.into());
        }

        if let Some(f) = &self.prod_code {
            params.insert("ProdCode", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct TagResources {
    resource_type: String,
    region_id: String,
    #[setters(generate = true, strip_option)]
    prod_code: Option<String>,
}

impl sealed::Bound for TagResources {}

impl TagResources {
    pub fn new(resource_type: impl Into<String>, region_id: impl Into<String>) -> Self {
        Self {
            resource_type: resource_type.into(),
            region_id: region_id.into(),
            prod_code: None,
        }
    }
}

impl crate::Request for TagResources {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "TagResources";

    type Body = ();

    type Response = TagResourcesResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("ResourceType", (&self.resource_type).into());
        params.insert("RegionId", (&self.region_id).into());

        if let Some(f) = &self.prod_code {
            params.insert("ProdCode", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct UntagResources {
    resource_type: String,
    region_id: String,
    #[setters(generate = true, strip_option)]
    all: Option<bool>,
    #[setters(generate = true, strip_option)]
    prod_code: Option<String>,
}

impl sealed::Bound for UntagResources {}

impl UntagResources {
    pub fn new(resource_type: impl Into<String>, region_id: impl Into<String>) -> Self {
        Self {
            resource_type: resource_type.into(),
            region_id: region_id.into(),
            all: None,
            prod_code: None,
        }
    }
}

impl crate::Request for UntagResources {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "UntagResources";

    type Body = ();

    type Response = UntagResourcesResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<&'static str, crate::QueryValue<'_>>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("ResourceType", (&self.resource_type).into());
        params.insert("RegionId", (&self.region_id).into());

        if let Some(f) = &self.all {
            params.insert("All", f.into());
        }

        if let Some(f) = &self.prod_code {
            params.insert("ProdCode", f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubmitSmsQualificationBusinessLicensePicsItem {
    pub license_pic: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubmitSmsQualificationOtherFilesItem {
    pub license_pic: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateSmsQualificationBusinessLicensePicsItem {
    pub license_pic: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateSmsQualificationOtherFilesItem {
    pub license_pic: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCardSmsTemplateTemplate {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryMobilesCardSupportMobilesItem {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsQualificationRecordResponseDataListItem {
    pub audit_remark: String,
    pub audit_time: String,
    pub company_name: String,
    pub create_date: String,
    pub group_id: i64,
    pub legal_person_name: String,
    pub qualification_group_name: String,
    pub state_name: String,
    pub use_by_self: String,
    pub work_order_id: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsQualificationRecordResponseData {
    pub list: Vec<QuerySmsQualificationRecordResponseDataListItem>,
    pub page_no: i64,
    pub page_size: i64,
    pub total: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySingleSmsQualificationResponseDataOtherFilesItem {
    pub license_pic: String,
    pub pic_url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySingleSmsQualificationResponseDataBusinessLicensePicsItem {
    pub license_pic: String,
    pub pic_url: String,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySingleSmsQualificationResponseData {
    pub admin_id_card_exp_date: String,
    pub admin_id_card_front_face: String,
    pub admin_id_card_no: String,
    pub admin_id_card_pic: String,
    pub admin_id_card_type: String,
    pub admin_name: String,
    pub admin_phone_no: String,
    pub business_license_pics: Vec<QuerySingleSmsQualificationResponseDataBusinessLicensePicsItem>,
    pub business_type: String,
    pub company_name: String,
    pub company_type: String,
    pub eff_time_str: String,
    pub legal_person_id_card_no: String,
    pub legal_person_id_card_type: String,
    pub legal_person_id_card_eff_time: String,
    pub legal_person_name: String,
    pub organization_code: String,
    pub other_files: Vec<QuerySingleSmsQualificationResponseDataOtherFilesItem>,
    pub qualification_group_id: i64,
    pub qualification_name: String,
    pub remark: String,
    pub state: String,
    pub use_by_self: bool,
    pub whether_share: bool,
    pub work_order_id: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsAuthorizationLetterResponseDataItem {
    pub authorization: String,
    pub authorization_letter_exp_date: String,
    pub authorization_letter_id: i64,
    pub authorization_letter_name: String,
    pub authorization_letter_pic: String,
    pub organization_code: String,
    pub proxy_authorization: String,
    pub sign_scope: String,
    pub state: String,
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsSignResponseAuditInfo {
    pub audit_date: String,
    pub reject_info: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsSignResponseSignIspRegisterDetailListItemRegisterStatusReasonsItem {
    pub reason_code: String,
    pub reason_desc_list: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsSignResponseSignIspRegisterDetailListItem {
    pub operator_code: String,
    pub operator_complete_time: String,
    pub register_status: i32,
    pub register_status_reasons:
        Vec<GetSmsSignResponseSignIspRegisterDetailListItemRegisterStatusReasonsItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsSignListResponseSmsSignListItemReason {
    pub reject_date: String,
    pub reject_info: String,
    pub reject_sub_info: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsSignListResponseSmsSignListItem {
    pub app_icp_record_id: i64,
    pub audit_status: String,
    pub authorization_letter_id: i64,
    pub business_type: String,
    pub create_date: String,
    pub order_id: String,
    pub reason: QuerySmsSignListResponseSmsSignListItemReason,
    pub sign_name: String,
    pub trademark_id: i64,
    pub authorization_letter_audit_pass: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeSignatureQualificationResponseDataData {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeSignatureQualificationResponseData {
    pub data: ChangeSignatureQualificationResponseDataData,
    pub err_code: String,
    pub err_message: String,
    pub success: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsTrademarkResponseDataItem {
    pub trademark_applicant_name: String,
    pub trademark_eff_exp_date: String,
    pub trademark_id: i64,
    pub trademark_name: String,
    pub trademark_pic: String,
    pub trademark_pic_url: String,
    pub trademark_registration_number: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsAppIcpRecordResponseDataItem {
    pub app_approval_date: String,
    pub app_icp_license_number: String,
    pub app_icp_record_id: i64,
    pub app_icp_record_pic: String,
    pub app_icp_record_pic_url: String,
    pub app_principal_unit_name: String,
    pub app_service_name: String,
    pub domain: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsTemplateResponseFileUrlList {
    pub file_url: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsTemplateResponseMoreDataFileUrlList {
    pub more_data_file_url: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsTemplateResponseAuditInfo {
    pub audit_date: String,
    pub reject_info: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsTemplateResponseVendorAuditStatus {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsTemplateListResponseSmsTemplateListItemReason {
    pub reject_date: String,
    pub reject_info: String,
    pub reject_sub_info: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsTemplateListResponseSmsTemplateListItem {
    pub audit_status: String,
    pub create_date: String,
    pub order_id: String,
    pub outer_template_type: i32,
    pub reason: QuerySmsTemplateListResponseSmsTemplateListItemReason,
    pub signature_name: String,
    pub template_code: String,
    pub template_content: String,
    pub template_name: String,
    pub template_type: i32,
    pub traffic_driving: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySendDetailsResponseSmsSendDetailDtOsSmsSendDetailDtoItem {
    pub content: String,
    pub err_code: String,
    pub out_id: String,
    pub phone_num: String,
    pub receive_date: String,
    pub send_date: String,
    pub send_status: i64,
    pub template_code: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySendDetailsResponseSmsSendDetailDtOs {
    pub sms_send_detail_dto: Vec<QuerySendDetailsResponseSmsSendDetailDtOsSmsSendDetailDtoItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySendStatisticsResponseDataTargetListItem {
    pub no_responded_count: i64,
    pub responded_fail_count: i64,
    pub responded_success_count: i64,
    pub send_date: String,
    pub total_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySendStatisticsResponseData {
    pub target_list: Vec<QuerySendStatisticsResponseDataTargetListItem>,
    pub total_size: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCardSmsDetailsResponseCardSendDetailDtoRecordsItem {
    pub err_code: String,
    pub out_id: String,
    pub phone_number: String,
    pub receive_date: String,
    pub receive_type: String,
    pub render_date: String,
    pub render_status: i64,
    pub send_date: String,
    pub send_status: i64,
    pub sms_content: String,
    pub template_code: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCardSmsDetailsResponseCardSendDetailDto {
    pub current_page: i64,
    pub page_size: i64,
    pub records: Vec<GetCardSmsDetailsResponseCardSendDetailDtoRecordsItem>,
    pub total_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetQualificationOssInfoResponseData {
    pub access_key_id: String,
    pub expire: i64,
    pub host: String,
    pub policy: String,
    pub signature: String,
    pub start_path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetOssInfoForUploadFileResponseModel {
    pub access_key_id: String,
    pub expire_time: String,
    pub host: String,
    pub policy: String,
    pub signature: String,
    pub start_path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsOcrOssInfoResponseModel {
    pub access_key_id: String,
    pub bucket: String,
    pub expire_time: String,
    pub host: String,
    pub policy: String,
    pub signature: String,
    pub start_path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddShortUrlResponseData {
    pub expire_date: String,
    pub short_url: String,
    pub source_url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryShortUrlResponseData {
    pub create_date: String,
    pub expire_date: String,
    pub page_view_count: String,
    pub short_url: String,
    pub short_url_name: String,
    pub short_url_status: String,
    pub source_url: String,
    pub unique_visitor_count: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubmitSmsQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: String,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsQualificationRecordResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: QuerySmsQualificationRecordResponseData,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySingleSmsQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: QuerySingleSmsQualificationResponseData,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateSmsQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: String,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSmsQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: bool,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequiredPhoneCodeResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: String,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValidPhoneCodeResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: bool,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSmsAuthorizationLetterResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: String,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsAuthorizationLetterResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: Vec<QuerySmsAuthorizationLetterResponseDataItem>,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub order_id: String,
    pub request_id: String,
    pub sign_name: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub app_icp_record_id: i64,
    pub apply_scene: String,
    pub audit_info: GetSmsSignResponseAuditInfo,
    pub authorization_letter_audit_pass: bool,
    pub authorization_letter_id: i64,
    pub create_date: String,
    pub file_url_list: Vec<String>,
    pub order_id: String,
    pub qualification_id: i64,
    pub register_result: i32,
    pub remark: String,
    pub request_id: String,
    pub sign_code: String,
    pub sign_isp_register_detail_list: Vec<GetSmsSignResponseSignIspRegisterDetailListItem>,
    pub sign_name: String,
    pub sign_status: i64,
    pub sign_tag: String,
    pub sign_usage: String,
    pub third_party: bool,
    pub trademark_id: i64,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsSignListResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub current_page: i32,
    pub page_size: i32,
    pub request_id: String,
    pub sms_sign_list: Vec<QuerySmsSignListResponseSmsSignListItem>,
    pub total_count: i64,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub order_id: String,
    pub request_id: String,
    pub sign_name: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
    pub sign_name: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeSignatureQualificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: ChangeSignatureQualificationResponseData,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddSmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
    pub sign_name: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifySmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
    pub sign_name: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsSignResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub create_date: String,
    pub reason: String,
    pub request_id: String,
    pub sign_name: String,
    pub sign_status: i32,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSmsTrademarkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: String,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsTrademarkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: Vec<QuerySmsTrademarkResponseDataItem>,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSmsAppIcpRecordResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: String,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsAppIcpRecordResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: Vec<QuerySmsAppIcpRecordResponseDataItem>,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub order_id: String,
    pub request_id: String,
    pub template_code: String,
    pub template_name: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub apply_scene: String,
    pub audit_info: GetSmsTemplateResponseAuditInfo,
    pub create_date: String,
    pub file_url_list: GetSmsTemplateResponseFileUrlList,
    pub intl_type: i32,
    pub more_data_file_url_list: GetSmsTemplateResponseMoreDataFileUrlList,
    pub order_id: String,
    pub related_sign_name: String,
    pub remark: String,
    pub request_id: String,
    pub template_code: String,
    pub template_content: String,
    pub template_name: String,
    pub template_status: String,
    pub template_tag: i32,
    pub template_type: String,
    pub variable_attribute: String,
    pub vendor_audit_status: GetSmsTemplateResponseVendorAuditStatus,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsTemplateListResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub current_page: i32,
    pub page_size: i32,
    pub request_id: String,
    pub sms_template_list: Vec<QuerySmsTemplateListResponseSmsTemplateListItem>,
    pub total_count: i64,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub order_id: String,
    pub request_id: String,
    pub template_code: String,
    pub template_name: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
    pub template_code: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
    pub template_code: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifySmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
    pub template_code: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub create_date: String,
    pub reason: String,
    pub request_id: String,
    pub template_code: String,
    pub template_content: String,
    pub template_name: String,
    pub template_status: i32,
    pub template_type: i32,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendSmsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub biz_id: String,
    pub request_id: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendBatchSmsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub biz_id: String,
    pub request_id: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySendDetailsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
    pub sms_send_detail_dt_os: QuerySendDetailsResponseSmsSendDetailDtOs,
    pub total_count: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySendStatisticsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub data: QuerySendStatisticsResponseData,
    pub request_id: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetOSSInfoForCardTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetMediaResourceIdResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCardSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryCardSmsTemplateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CheckMobilesCardSupportResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryMobilesCardSupportResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCardSmsLinkResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCardSmsDetailsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub card_send_detail_dto: GetCardSmsDetailsResponseCardSendDetailDto,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryCardSmsTemplateReportResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendCardSmsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendBatchCardSmsResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetQualificationOssInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub data: GetQualificationOssInfoResponseData,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetOSSInfoForUploadFileResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub model: GetOssInfoForUploadFileResponseModel,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSmsOcrOssInfoResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub access_denied_detail: String,
    pub model: GetSmsOcrOssInfoResponseModel,
    pub request_id: String,
    pub success: bool,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SmsConversionIntlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConversionDataIntlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddShortUrlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub data: AddShortUrlResponseData,
    pub request_id: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteShortUrlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub request_id: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryShortUrlResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    pub data: QueryShortUrlResponseData,
    pub request_id: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTagResourcesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagResourcesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UntagResourcesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
}
