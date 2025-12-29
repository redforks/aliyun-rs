#![allow(unused_mut)]

#[derive(Clone, Copy)]
pub enum Endpoint {
    CnHangzhou,
}

impl From<Endpoint> for &'static str {
    fn from(ep: Endpoint) -> Self {
        match ep {
            Endpoint::CnHangzhou => "ocr-api.cn-hangzhou.aliyuncs.com",
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
            "2021-07-07",
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
            "2021-07-07",
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
    /// # OCR统一识别
    ///
    /// OCR统一识别接口支持识别多种图片类型，包括通用文字、个人卡证、发票等。您只需要通过Type参数指定图片类型，无须更换接口。
    ///
    /// #### 如何使用本接口
    /// | 步骤 | 概述                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    /// | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | 1    | 开通[OCR 统一识别](https://common-buy.aliyun.com/?commodityCode=ocr_unity_public_cn)服务。 开通此API后会赠送免费额度，可使用免费额度测试。                                                                                                                                                                                                                                                                                                                                      |
    /// | 2    | 购买[OCR 共享资源包](https://common-buy.aliyun.com/?spm=5176.23043878.0.0.5f3d1287hpm7ZT&commodityCode=ocr_share_dp_cn)。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。                                                                                                                                           |
    /// | 3    | 可以参照[调试页面](https://api.aliyun.com/api/ocr-api/2021-07-07/RecognizeAllText?sdkStyle=dara)提供的代码示例完成 API 接入开发。接入完成后，调用 API 获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对 RAM 账号进行授权。创建 RAM 用户的具体操作，请参考：[创建 RAM 用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为 RAM 用户授权。](https://help.aliyun.com/document_detail/116146.html) |
    ///
    /// #### 重要提示
    ///
    /// | 类型     | 概述                                                                                                                                                                      |
    /// | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | 图片格式 | <ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、PDF。</li></ul>                                        |
    /// | 图片尺寸 | <ul> <li> 图片长宽需要大于 15 像素，小于 8192 像素。</li> <li>长宽比需要小于 50。</li> <li>如需达到较好识别效果，建议长宽均大于 500px。</li> </ul>                        |
    /// | 图片大小 | <ul> <li> 图片二进制文件不能超过 10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于 1.5M 图片进行识别，且通过传图片 URL 的方式调用接口。</li> </ul>                |
    /// | 其他提示 | <ul> <li>请保证整张图片内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul> |
    ///
    /// # Error Codes
    /// - `InvalidCountry`: Specified parameter Country is not valid.
    /// - `invalidInputParameter`: %s
    ///
    /// # Extra Info
    /// #### 您可以参考下面的示例调用统一API接口
    /// * 通过图片URL请求接口Java示例
    /// ```ignore
    /// package demo;
    /// import com.aliyun.ocr_api20210707.Client;
    /// import com.aliyun.ocr_api20210707.models.*;
    /// import com.aliyun.tea.TeaException;
    /// import com.aliyun.teaopenapi.models.Config;
    /// import com.google.gson.Gson;
    /// public class Demo {
    ///     public static void main(String[] args) throws Exception {
    ///         final String endpoint = "ocr-api.cn-hangzhou.aliyuncs.com";
    ///         final String accessKeyID = "您的AccessKeyID";
    ///         final String accessKeySecret = "您的AccessKeySecret";
    ///         final String imageUrl = "https://example.png";
    ///         final Config config = new Config().setEndpoint(endpoint).setAccessKeyId(accessKeyID).setAccessKeySecret(accessKeySecret);
    ///         Client client = new Client(config);
    ///         RecognizeAllTextRequest request = new RecognizeAllTextRequest()
    ///                 .setType("Advanced")        // 指定Type（此参数为必填参数）
    ///                 .setUrl(imageUrl)           // 图片url
    ///                 .setOutputOricoord(true);   // 设置返回原图坐标。您可以设置更多二级参数。
    ///         // 您也可以在 request 中指定更多三级参数。例如对于 Type=Advanced，可以指定 OutputCharInfo=true（输出单字信息）
    ///         RecognizeAllTextRequest.RecognizeAllTextRequestAdvancedConfig advancedConfig = new RecognizeAllTextRequest.RecognizeAllTextRequestAdvancedConfig()
    ///             .setOutputCharInfo(true);
    ///         request.setAdvancedConfig(advancedConfig);
    ///         try {
    ///             RecognizeAllTextResponse response = client.recognizeAllText(request);
    ///             System.out.println(new Gson().toJson(response.getBody().getData().toMap()));
    ///         } catch (TeaException e) {
    ///             System.out.println(e.getStatusCode());
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// * 通过上传本地图片请求接口Java示例
    /// ```ignore
    /// package demo;
    /// import com.aliyun.ocr_api20210707.Client;
    /// import com.aliyun.ocr_api20210707.models.*;
    /// import com.aliyun.tea.TeaException;
    /// import com.aliyun.teaopenapi.models.Config;
    /// import com.google.gson.Gson;
    /// public class Demo {
    ///     public static void main(String[] args) throws Exception {
    ///         final String endpoint = "ocr-api.cn-hangzhou.aliyuncs.com";
    ///         final String accessKeyID = "您的AccessKeyID";
    ///         final String accessKeySecret = "您的AccessKeySecret";
    ///         final Config config = new Config().setEndpoint(endpoint).setAccessKeyId(accessKeyID).setAccessKeySecret(accessKeySecret);
    ///         final String localImageFileName = "~/example.png"; // 本地图片路径
    ///         Client client = new Client(config);
    ///         try (InputStream imageStream = new FileInputStream(localImageFileName)) {
    ///             RecognizeAllTextRequest request = new RecognizeAllTextRequest()
    ///                     .setType("Advanced")        // 指定Type（此参数为必填参数）
    ///                     .setBody(imageStream)       // 指定本地图片路径
    ///                     .setOutputOricoord(true);   // 设置返回原图坐标。您可以设置更多二级参数。
    ///             // 您也可以在 request 中指定更多参数。例如对于 Type=Advanced，可以指定 OutputCharInfo=true（输出单字信息）
    ///             RecognizeAllTextRequest.RecognizeAllTextRequestAdvancedConfig advancedConfig = new RecognizeAllTextRequest.RecognizeAllTextRequestAdvancedConfig()
    ///                     .setOutputCharInfo(true);
    ///             request.setAdvancedConfig(advancedConfig);
    ///             RecognizeAllTextResponse response = client.recognizeAllText(request);
    ///             System.out.println(new Gson().toJson(response.getBody().getData().toMap()));
    ///         } catch (TeaException e) {
    ///             System.out.println(e.getStatusCode());
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_all_text(
        &self,
        req: RecognizeAllText,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeAllTextResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 通用票证抽取
    ///
    /// 通用票证抽取结合读光OCR和通义千问大模型的能力，针对OCR不支持的长尾票据，提供关键KV信息抽取，例如名称、地址、开票日期等关键字段结构化识别输出。
    ///
    /// #### 如何使用本接口
    ///
    /// | 步骤 | 概述                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    /// | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | 1    | 开通 [OCR 统一识别](https://common-buy.aliyun.com/?commodityCode=ocr_unity_public_cn) 服务。本接口在公测阶段，是免费接口，开通后即可调用。
    /// | 3    | 可以参照[调试页面](https://api.aliyun.com/api/ocr-api/2021-07-07/RecognizeGeneralStructure?sdkStyle=dara) 提供的代码示例完成 API 接入开发。接入完成后，调用 API 获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对 RAM 账号进行授权。创建 RAM 用户的具体操作，请参考：[创建 RAM 用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为 RAM 用户授权。](https://help.aliyun.com/document_detail/116146.html) |
    ///
    /// #### 重要提示
    ///
    /// | 类型     | 概述                                                                                                                                                                      |
    /// | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | 图片格式 | <ul> <li>本接口支持：PNG、JPG、JPEG、PDF、BMP、GIF、TIFF、WebP。</li></ul>                                        |
    /// | 图片尺寸 | <ul> <li> 图片长宽需要大于 15 像素，小于 8192 像素。</li> <li>长宽比需要小于 50。</li> <li>如需达到较好识别效果，建议长宽均大于 500px。</li> </ul>                        |
    /// | 图片大小 | <ul> <li> 图片二进制文件不能超过 10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于 1.5M 图片进行识别，且通过传图片 URL 的方式调用接口。</li> </ul>                |
    /// | 其他提示 | <ul> <li>请保证整张图片内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li><li> PDF类型文件仅识别第一页。 </li> </ul> |
    /// ---
    ///
    /// # Error Codes
    /// - `DataInspectionFailed`: Input or output data may contain inappropriate content.
    /// - `ExceededKeyNumber`: Too many keys, please try again with fewer keys.
    /// - `LLMTimeout`: Large language model timeout, please try again with fewer keys.
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_general_structure(
        &self,
        req: RecognizeGeneralStructure,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeGeneralStructureResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 全文识别高精版
    ///
    /// 支持多格式版面、复杂文档背景和光照环境的精准识别，可实现印章擦除后识别，支持低置信度过滤、图案检测等高阶功能。
    ///
    /// #### 本接口适用场景
    /// * 阿里云全文识别高精版，是阿里云官方自研OCR文字识别产品，智能识别图片所包含的全部字段，集表格识别、旋转识别、生僻字识别等多功能为一体，提供高性价比的多场景文字识别体验。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01JW5Amf1TfpKdxvNhB_!!6000000002410-2-tps-1105-549.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多功能集结|集表格识别、旋转识别、生僻字识别等多功能为一体。|
    /// |抗干扰|支持多格式版面、复杂文档背景和光照环境的精准识别。|
    /// |自动排异|对有印章、手印的文档，可实现印章擦除后识别。|
    /// |高阶能力|支持覆盖文字编辑、低置信度过滤、图案检测。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [通用文字识别](https://common-buy.aliyun.com?commodityCode=ocr_general_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=universal&subtype=general#intro)免费体验本功能识别效果。|
    /// |2|购买[全文识别高精版资源包](https://common-buy.aliyun.com/?commodityCode=ocr_general_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_general_dp_cn_20211103172431_0719%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeAdvanced?lang=JAVA&sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_advanced(
        &self,
        req: RecognizeAdvanced,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeAdvancedResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 通用手写体识别
    ///
    /// 支持中文手写体、英文手写体、数字手写体等各种复杂场景的手写文字识别。
    ///
    /// #### 本接口适用场景
    /// * 阿里云通用手写体识别，是阿里云官方自研OCR文字识别产品，适用于获取手写体书面形式的文字场景，适用于各类手写笔记、板书等。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1xvaLcggP7K4jSZFqXXamhVXa-1600-920.jpg" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多文字形式|支持中文手写体、英文手写体、数字手写体。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [通用文字识别](https://common-buy.aliyun.com?commodityCode=ocr_general_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=universal&subtype=shouxie#intro)免费体验本功能识别效果。|
    /// |2|购买[通用手写体识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_general_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_general_dp_cn_20211103172431_0249%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeHandwriting?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [云市场手写体识别。](https://market.aliyun.com/apimarket/detail/cmapi00040832) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_handwriting(
        &self,
        req: RecognizeHandwriting,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeHandwritingResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 电商图片文字识别
    ///
    /// 针对电商商品宣传图片、社区贴吧图片、网络UGC图片等网络场景下图片字符快速精准识别。
    ///
    /// #### 本接口适用场景
    /// * 阿里云电商图片文字识别，是阿里云官方自研OCR文字识别产品，支持电商商品宣传图片、社区贴吧图片、网络UGC图片识别，针对电商海量图片内容核查就场景进行特定优化，只输出文字块内容及坐标，极大提升识别效率。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i4/O1CN01beY6FP20nVIAEwIiL_!!6000000006894-0-tps-850-443.jpg" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |多网络场景|电商商品宣传图片、社区贴吧图片、网络UGC图片等网络场景识别文字。|
    /// |适用场合|适用于违规广告识别、信息审核管理和网络安全治理等场景。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [通用文字识别](https://common-buy.aliyun.com?commodityCode=ocr_general_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=universal&subtype=ecommerce#intro)免费体验本功能识别效果。|
    /// |2|购买[电商图片文字识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_general_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_general_dp_cn_20211103172431_0503%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeBasic?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_basic(
        &self,
        req: RecognizeBasic,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeBasicResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 通用文字识别
    ///
    /// 适用于非结构化文字识别，支持返回文字内容和位置坐标信息。
    ///
    /// #### 本接口适用场景
    /// * 阿里云通用文字识别，是阿里云官方自研OCR文字识别产品，适用于各类常见文档图片或文档扫描件中的文字信息按照文档原有的格式智能识别文字并结构化输出识别结果。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i3/O1CN01g9tMm71eQDRRu7U3C_!!6000000003865-2-tps-899-243.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |全字段识别|结构化识别图片上所包含的全字段，并返回JSON。|
    /// |图像增强|默认支持图像增强，包括图像畸变自动矫正、模糊图片自动增强等能力。|
    /// |高精度高性能|超高精度及性能；识别准确率位于行业前列，识别速度显著高于国内其他OCR云服务。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [通用文字识别](https://common-buy.aliyun.com?commodityCode=ocr_general_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=universal&subtype=general_text#intro)免费体验本功能识别效果。|
    /// |2|购买[通用文字识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_general_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_general_dp_cn_20211103172431_0908%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeGeneral?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_general(
        &self,
        req: RecognizeGeneral,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeGeneralResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 表格识别
    ///
    /// 支持对有线表格、条纹表格、无线表格进行有效识别。
    ///
    /// #### 本接口适用场景
    /// * 阿里云表格识别，是阿里云官方自研OCR文字识别产品，支持对多种表格格式（有线表格、条纹表格、无线表格）进行智能文字识别并结构化输出识别结果。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://help-static-aliyun-doc.aliyuncs.com/assets/img/zh-CN/6884068261/p303185.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多格式|支持有线表格、条纹表格、无线表格、手写表格识别。|
    /// |全字段识别|智能识别图片上的表格所包含的全部字段。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |置信度对比|对低置信度文字进行标红处理，便于二次确认。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [通用文字识别](https://common-buy.aliyun.com?commodityCode=ocr_general_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=universal&subtype=table#intro)免费体验本功能识别效果。|
    /// |2|购买[表格识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_general_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_general_dp_cn_20211103172431_0075%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeTableOcr?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// 注：PDF文件格式的表格解析请点击[表格智能解析](https://help.aliyun.com/document_detail/450742.html)快速了解
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_table_ocr(
        &self,
        req: RecognizeTableOcr,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeTableOcrResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 防疫健康码识别
    ///
    /// 包括全国主要省市健康码，支持健康码图片的姓名、日期、时间、颜色、备注信息等主要字段的识别结果输出。
    ///
    /// #### 本接口适用场景
    /// * 阿里云防疫健康码识别，是阿里云官方自研OCR文字识别产品，适用于获取健康码上的健康码颜色、姓名、日期等关键信息的场景。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://help-static-aliyun-doc.aliyuncs.com/assets/img/zh-CN/7365590561/p433785.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |全字段识别|智能识别营业执照上所包含的全部字段。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [通用文字识别](https://common-buy.aliyun.com?commodityCode=ocr_general_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=universal&subtype=health_code#intro)免费体验本功能识别效果。|
    /// |2|购买[防疫健康码识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_general_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_general_dp_cn_20220419111546_0636%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeHealthCode?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|。
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_health_code(
        &self,
        req: RecognizeHealthCode,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeHealthCodeResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 文档结构化识别
    ///
    /// 对文档信息进行结构化识别，并提供元素平铺和层级树两种视角的版面信息输出。能够将文档中的文字元素（单字、文字块、行等）和相应的版面格式（标题、段落、表格）抽离并按顺序输出。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_document_structure(
        &self,
        req: RecognizeDocumentStructure,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeDocumentStructureResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 身份证识别
    ///
    /// 支持二代身份证正反面，包括姓名、性别、民族、地址、出生日期、身份证号、签发机关、有效期限等字段的结构化识别。支持身份证质量检测，是否翻拍，是否是复印件，完整度评分，整体质量分数、篡改指数及人脸位置检测。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云身份证文字识别，是阿里云官方自研OCR文字识别产品，用于对中国大陆身份证（含临时身份证）正反面图片进行智能文字识别并结构化输出识别结果。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i3/O1CN01VMB4xL1kWWl9GqGNt_!!6000000004691-0-tps-1071-532.jpg" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多类型覆盖|支持自动区分正反面、支持少数民族版式识别、临时身份证识别、生僻字识别、反光实拍、劣质图像识别。<img width=1000/>|
    /// |风险检测|支持证件风险检测预警能力，包括智能判断图片完整度、复印件检测、翻拍检测、质量分等。|
    /// |人像检测|支持图像检测功能，可定位身份证中的人像图案并返回坐标。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=standard&subtype=idcard#intro)免费体验本功能识别效果。|
    /// |2|购买[身份证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20211018150333_0014%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](~~295347~~)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeIdcard?lang=SWIFT&sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](~~93720~~)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](~~116146~~)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |国家与语言| <ul> <li>本接口只支持中国大陆身份证。</li></ul> |
    /// |其他提示|<ul> <li>请保证整张身份证内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [身份证混贴识别。](https://market.aliyun.com/products/57124001/cmapi00042846.html?#sku=yuncode3684600001) </li> <li> [国际身份证识别。](~~455939~~) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_idcard(
        &self,
        req: RecognizeIdcard,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeIdcardResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 国际护照识别
    ///
    /// 可对美国、法国、英国、日本、韩国等世界多个主要国家和地区护照提供识别服务，支持字段包括国籍、护照号码、出生日期、姓名等。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云国际护照识别，是阿里云官方自研OCR文字识别产品，适用于出入境审查、国内外身份核验等各种需要提取护照信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i4/O1CN01A0sPpE1ZzPvVTa6QV_!!6000000003265-2-tps-2482-1193.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多国护照|对美国、法国、英国、日本、韩国等多国和地区护照提供识别服务。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=standard&subtype=passport#intro)免费体验本功能识别效果。|
    /// |2|购买[国际护照识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20211018150333_0624%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizePassport?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |相关能力|<ul> <li> [云市场护照识别。](https://market.aliyun.com/products/57124001/cmapi016682.html?spm=a2c4g.11186623.0.0.47e98a21paGIxa&innerSource=search#sku=yuncode1068200007) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_passport(
        &self,
        req: RecognizePassport,
    ) -> impl std::future::Future<Output = crate::Result<RecognizePassportResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 户口本识别
    ///
    /// 可结构化识别户口常住人口登记卡页面及户主页的内容，有效识别户口本上的相关户籍证明信息。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云户口本识别，是阿里云官方自研OCR文字识别产品，可用于识别户口本户主页的户主姓名、住址、户号等字段。也适用于识别户口本常住人口页的出生日期、出生地、姓名、民族等信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i2/O1CN01XgQQf11PBoxYZP19J_!!6000000001803-2-tps-2458-1318.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=standard&subtype=household#intro)免费体验本功能识别效果。|
    /// |2|购买 [个人证照识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20211018150333_0555%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeHousehold?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_household(
        &self,
        req: RecognizeHousehold,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeHouseholdResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 不动产权证识别
    ///
    /// 可准确识别不动产证中的各项关键信息，包括户主信息、房屋地址、面积大小、土地权利类型等，适用于全国各地的不同房产证识别。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云不动产权证识别，是阿里云官方自研OCR文字识别产品，适用于识别不动产权证和房产证上的关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1Nk0DOpP7gK0jSZFjXXc5aXXa-1600-920.jpg" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |适用范围广|适用于全国各地的不同不动产权证和房产证识别。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=standard&subtype=estate_cert#intro)免费体验本功能识别效果。|
    /// |2|购买[不动产权证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20211018150333_0807%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeEstateCertification?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |相关能力|<ul> <li> [云市场不动产权证识别。](https://market.aliyun.com/products/57124001/cmapi032590.html?spm=a2c4g.11186623.0.0.53898a21nnCeEE#sku=yuncode2659000001) </li>  </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_estate_certification(
        &self,
        req: RecognizeEstateCertification,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeEstateCertificationResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 银行卡识别
    ///
    /// 可精准识别各类银行卡中的银行卡卡号和有效期，且支持横卡、竖卡及银行卡任意角度偏斜情况的识别与提取，支持中国内地大多数银行，以及各种位数、凸字卡面、平面卡面等的识别。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云银行卡识别，是阿里云官方自研OCR文字识别产品，适用于获取银行卡上的卡号、日期、银行名称等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1gbfaN7L0gK0jSZFAXXcA9pXa-1600-800.jpg" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多银行|支持中国银行、中国工商银行、交通银行、邮政银行等多家银行。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |多卡面类型|支持各种位数、凸字卡面、平面卡面的识别。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=standard&subtype=bank_card#intro)免费体验本功能识别效果。|
    /// |2|购买 [银行卡识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20211018150333_0139%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeBankCard?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |相关能力|<ul> <li> [云市场银行卡识别。](https://market.aliyun.com/products/57124001/cmapi016870.html?spm=a2c4g.11186623.0.0.47e98a21uyjeUi&innerSource=search#sku=yuncode1087000000) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_bank_card(
        &self,
        req: RecognizeBankCard,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeBankCardResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 出生证明识别
    ///
    /// 可准确识别出生证明中的各项关键信息，包括出生日期、出生体重、出生地点等。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云出生证明识别，是阿里云官方自研OCR文字识别产品，适用于识别出生证明所包含的新生儿姓名、性别、出生日期、出生地点等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i2/O1CN01NGKqgA1DoxKux5pBP_!!6000000000264-0-tps-1046-705.jpg" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图片格式|支持PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=standard&subtype=birth_certification#intro)免费体验本功能识别效果。|
    /// |2|购买[出生证明识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20211018150333_0645%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeBirthCertification?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li></ul>|
    /// |相关能力|<ul> <li> [云市场出生证明识别。](https://market.aliyun.com/products/57124001/cmapi00043620.html?spm=a2c4g.11186623.0.0.47e98a21Sz0eAq#sku=yuncode3762000001) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_birth_certification(
        &self,
        req: RecognizeBirthCertification,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeBirthCertificationResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 中国护照识别
    ///
    /// 支持中国人民共和国护照的结构化内容检测识别功能，支持中国内地、中国香港、中国澳门和中国台湾地区的护照识别，识别内容包括出生地、出生日期、国籍、性别、护照号码、有效期至、签发国、签发地等字段。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_chinese_passport(
        &self,
        req: RecognizeChinesePassport,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeChinesePassportResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 来往大陆（内地）通行证识别
    ///
    /// 可准确识别通行证中的各项关键信息，包括姓名、出生日期、证件号码等。包括港澳居民来往大陆通行证以及台湾居民来往大陆通行证。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云来往大陆通行证识别，精准识别通行证中所包含的中英文姓名、出生日期、有效期限、签发地点、证件号码等信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例（仅支持正面识别）
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i2/O1CN01VpucoK1PtmovU859J_!!6000000001899-0-tps-928-626.jpg" width="50%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=standard&subtype=mainland_card#intro)免费体验本功能识别效果。|
    /// |2|购买 [个人证照识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20211222165053_0134%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeExitEntryPermitToMainland?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |识别范围|<ul> <li> 本接口只支持正面识别，背面不支持。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_exit_entry_permit_to_mainland(
        &self,
        req: RecognizeExitEntryPermitToMainland,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeExitEntryPermitToMainlandResponse>>
    + Send {
        self.call(req)
    }

    ///
    /// # 往来港澳台通行证识别
    ///
    /// 支持通行证中的各项关键信息，姓名、出生日期、证件号码等字段的准确识别。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_exit_entry_permit_to_hk(
        &self,
        req: RecognizeExitEntryPermitToHK,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeExitEntryPermitToHKResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 中国香港身份证识别
    ///
    /// 支持香港永久性居民身份证和香港居民身份证两种类型的证件识别，已支持全字段识别，包括中文姓名（如有）、英文姓名、中文姓名电码（如有）、出生日期、性别、符号标记、身份证号码等。
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。|
    /// |2|购买[中国香港身份证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20230323152059_0554%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeHKIdcard?sdkStyle=dara&tab=DEMO&lang=JAVA)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |国家与语言| <ul> <li>本接口只支持中国香港身份证。</li></ul> |
    /// |其他提示|<ul> <li>请保证整张身份证内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [国际身份证识别。](https://help.aliyun.com/document_detail/455939.html) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_hk_idcard(
        &self,
        req: RecognizeHKIdcard,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeHKIdcardResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 社保卡识别
    ///
    /// 支持全字段识别，包括标题、姓名、社会保障号码、社会保障卡号、银行账号、发卡日期等。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云社保卡识别，是阿里云官方自研OCR文字识别产品，适用于识别社会保障卡中所包含的标题、姓名、社保卡号码、卡号、发卡日期等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01lTV8Qu1jeU1ycPA30_!!6000000004573-2-tps-820-272.png" width="60%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达97%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=standard&subtype=social_security_card#intro)免费体验本功能识别效果。|
    /// |2|购买[个人证照识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20220507144415_0128%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeSocialSecurityCardVersionII?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。</li> </ul> |
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_social_security_card_version_ii(
        &self,
        req: RecognizeSocialSecurityCardVersionII,
    ) -> impl std::future::Future<
        Output = crate::Result<RecognizeSocialSecurityCardVersionIIResponse>,
    > + Send {
        self.call(req)
    }

    ///
    /// # 国际身份证识别
    ///
    /// 可对国外身份证件进行结构化识别，目前支持越南、韩国、印度、孟加拉居民身份证，可识别字段包括姓名、出生日期、证件号码等。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云国际身份证识别，是阿里云官方自研OCR文字识别产品，适用于出入境审查、国内外身份核验等各种需要提取身份证信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i2/O1CN01DvhDz81Y8uxtjp3ER_!!6000000003015-0-tps-1071-532.jpg" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多国身份证|对越南、韩国、印度、孟加拉居民身份证提供识别服务|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [个人证照识别](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=standard&subtype=idcard#intro)免费体验本功能识别效果。|
    /// |2|购买[国际身份证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_personalcard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_personalcard_dp_cn_20220829103503_0482%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeInternationalIdcard?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、OFD、PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>请保证整张身份证内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// # Error Codes
    /// - `illegalCountryName`: the country name is not supported.
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_international_idcard(
        &self,
        req: RecognizeInternationalIdcard,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeInternationalIdcardResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 混贴发票识别
    ///
    /// 支持各类票据的发票代码、价税合计、合计金额、购买方识别号、开票日期等关键字段结构化识别输出。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云混贴发票识别，是阿里云官方自研OCR文字识别产品，适用于获取多种发票集合在一个页面的场景，需要获取多种发票上的关键信息。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i4/O1CN018t7r7W1mCx440fhmU_!!6000000004919-2-tps-1052-594.png" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |全字段识别|智能识别混贴发票上所包含的全部字段。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=multi_invoice#intro)免费体验本功能识别效果。|
    /// |2|购买[混贴发票识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20211103182712_0880%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeMixedInvoices?lang=PHP&sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、PDF、OFD。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> URL长度不能超过2048。 </li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |当前支持的发票类型|增值税发票、火车票、机票行程单、出租车票、定额发票、增值税发票卷票、机动车销售发票、网约车行程单、二手车销售统一发票、通用机打发票、客运车船票、过路过桥费发票、社会保障卡、税收完税证明、支付详情页、电商详情页、酒店流水、非税收入发票。|
    /// |相关能力|<ul> <li> [云市场混贴票据识别。](https://market.aliyun.com/products/57124001/cmapi00034969.html?spm=a2c4g.11186623.0.0.6dcb4dcdy2b5CR#sku=yuncode2896900002) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_mixed_invoices(
        &self,
        req: RecognizeMixedInvoices,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeMixedInvoicesResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 增值税发票识别
    ///
    /// 支持增值税专用发票、增值税普通发票、增值税电子发票识别，支持包括发票代码、发票号码、开票日期、发票金额、发票税额、检验码、购买方税号、销售方税号、发票详情等关键字段结构化识别输出。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云增值税发票识别，是阿里云官方自研OCR文字识别产品，适用于识别增值税发票上所包含的价税合计、发票代码、发票号码等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1D2NXpIKfxu4jSZPfXXb3dXXa-2060-1200.jpg" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=invoice#intro)免费体验本功能识别效果。|
    /// |2|购买[增值税发票识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20211103182712_0783%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeInvoice?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// ------
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、PDF、OFD。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |相关能力|<ul> <li> [云市场增值税发票识别。](https://market.aliyun.com/products/57124001/cmapi027758.html?spm=a2c4g.11186623.0.0.1ff64dcdDsX9s8#sku=yuncode2175800000) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_invoice(
        &self,
        req: RecognizeInvoice,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeInvoiceResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 机动车销售统一发票识别
    ///
    /// 支持包括发票代码、开票号码、开票日期、发票金额、增值税税额、合格证号、购买方名称、购买方身份证号/代码等关键字段结构化识别输出。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云机动车销售发票识别，是阿里云官方自研OCR文字识别产品，适用于识别购车发票上的发票金额、购买方名称、车辆类型、厂牌型号、销售方名称等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1k5AyamslXu8jSZFuXXXg7FXa-2060-1000.jpg" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图片格式|支持PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=car_invoice#intro)免费体验本功能识别效果。|
    /// |2|购买[机动车销售发票识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20211103182712_0430%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeCarInvoice?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// ------
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |相关能力|<ul> <li> [云市场机动车销售发票识别。](https://market.aliyun.com/products/57124001/cmapi029811.html?spm=a2c4g.11186623.0.0.6dcb4dcdaoX2WN#sku=yuncode2381100001) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_car_invoice(
        &self,
        req: RecognizeCarInvoice,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeCarInvoiceResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 定额发票识别
    ///
    /// 支持包括发票号码、发票代码、发票金额等关键字段结构化识别输出。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_quota_invoice(
        &self,
        req: RecognizeQuotaInvoice,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeQuotaInvoiceResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 航空行程单识别
    ///
    /// 支持包括旅客姓名、身份证号码、电子客票号码、填开日期、填开单位等字段结构化识别输出。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云航空行程单识别，是阿里云官方自研OCR文字识别产品，适用于识别航空行程单所包含的乘机人姓名、身份证号、电子客票号码、验证码、填开日期、销售单位代号、承运人、填开单位、票价、税费、燃油附加费等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 电子发票（航空运输电子客票行程单）暂不支持。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01B5q2Z321hNcDMA9zN_!!6000000007016-2-tps-825-318.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=air_itinerary#intro)免费体验本功能识别效果。|
    /// |2|购买 [票据凭证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20211103182712_0981%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeAirItinerary?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、OFD、PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li>  </ul>|
    /// |相关能力|<ul> <li> [云市场航空行程单识别。](https://market.aliyun.com/products/57124001/cmapi00035385.html?#sku=yuncode2938500001) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_air_itinerary(
        &self,
        req: RecognizeAirItinerary,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeAirItineraryResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 火车票识别
    ///
    /// 支持包括票号、出发站、到达站、开车时间、票价、座位类型、旅客信息、座位号、车次等字段结构化识别输出。
    /// 2024.12.31更新后，支持电子火车票，增加返回以下新字段：电子客票号、购买方名称、购买方统一信用代码、标题、开票日期、备注。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云火车票识别，是阿里云官方自研OCR文字识别产品，适用于识别火车票上车次、座位号、旅客信息、座位类型、票价等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1n9tZccVl614jSZKPXXaGjpXa-1600-800.jpg" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图片格式|支持PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、OFD、PDF。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=train_ticket#intro)免费体验本功能识别效果。|
    /// |2|购买[火车票识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20211103182712_0135%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeTrainInvoice?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// ------
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、OFD、PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |相关能力|<ul> <li> [云市场火车票识别。](https://market.aliyun.com/products/57124001/cmapi020096.html?spm=a2c4g.11186623.0.0.6dcb4dcdaoX2WN&innerSource=search#sku=yuncode1409600000) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_train_invoice(
        &self,
        req: RecognizeTrainInvoice,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeTrainInvoiceResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 出租车发票识别
    ///
    /// 支持包括发票代码、发票号码、日期、发票金额等关键字段结构化识别输出。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云出租车发票识别，是阿里云官方自研OCR文字识别产品，适用于识别出租车发票所包含的发票代码、发票号码、金额、里程等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1.OicXebviK0jSZFNXXaApXXa-364-982.jpg" width="30%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达97%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=taxi_ticket#intro)免费体验本功能识别效果。|
    /// |2|购买[票据凭证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20211103182712_0996%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeTaxiInvoice?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li>  </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_taxi_invoice(
        &self,
        req: RecognizeTaxiInvoice,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeTaxiInvoiceResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 增值税发票卷票识别
    ///
    /// 支持对卷票上包括发票代码、发票号码、开票日期、发票金额、校验码、大写金额、销售方税号、购买方税号等关键字段结构化识别输出。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_roll_ticket(
        &self,
        req: RecognizeRollTicket,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeRollTicketResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 银行承兑汇票识别
    ///
    /// 支持包括出票日期、票据号码、出票人信息、收票人信息、承兑人信息、票据金额等关键字段结构化识别输出。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云银行承兑汇票识别，是阿里云官方自研OCR文字识别产品，适用于识别银行承兑汇票上的出票日期、到期日期、票据状态、票据号码、出票人信息、售票人信息、承兑人信息等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例（仅支持正面识别，暂不支持背面识别）
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN017mjVfp1P2CY21py2u_!!6000000001782-2-tps-2602-1888.png" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图片格式|支持PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=bank_acceptance#intro)免费体验本功能识别效果。|
    /// |2|购买[银行承兑汇票识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20220506145928_0399%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeBankAcceptance?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// ------
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |相关能力|<ul> <li> [云市场银行承兑汇票识别。](https://market.aliyun.com/products/57000002/cmapi00040502.html?spm=a2c4g.11186623.0.0.6dcb4dcdaoX2WN&innerSource=search_%E9%93%B6%E6%89%BF#sku=yuncode3450200001) </li> </ul>|
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_bank_acceptance(
        &self,
        req: RecognizeBankAcceptance,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeBankAcceptanceResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 客运车船票识别
    ///
    /// 支持包括标题、发票号码、出发车站、到达车站、日期、金额等关键字段结构化识别输出。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_bus_ship_ticket(
        &self,
        req: RecognizeBusShipTicket,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeBusShipTicketResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 非税收入发票识别
    ///
    /// 支持包括票据代码、交款人、票据号码、合计金额、收款单位等关键字段结构化识别输出。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云非税收入发票识别，是阿里云官方自研OCR文字识别产品，适用于识别非税收入发票所包含的票据号码、标题、开票日期、合计金额、收款人等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i4/O1CN01jraEpU29O9qPIWKaT_!!6000000008057-0-tps-2977-1800.jpg" width="40%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达97%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=nontax_invoice#intro)免费体验本功能识别效果。|
    /// |2|购买[车辆物流识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20220303112214_0050%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeNonTaxInvoice?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。</li> </ul> |
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_non_tax_invoice(
        &self,
        req: RecognizeNonTaxInvoice,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeNonTaxInvoiceResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 通用机打发票识别
    ///
    /// 支持包括发票代码、发票号码、销售方名称、销售方识别号、购买方名称、购买方识别号、合计金额等关键字段结构化识别输出。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_common_printed_invoice(
        &self,
        req: RecognizeCommonPrintedInvoice,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeCommonPrintedInvoiceResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 酒店流水识别
    ///
    /// 支持包括房号、入住日期、离店日期、消费总计、付款总计、消费详单等关键字段结构化识别输出。
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_hotel_consume(
        &self,
        req: RecognizeHotelConsume,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeHotelConsumeResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 支付详情页识别
    ///
    /// 支持包括收款方名称、合计金额、付款方式、商品说明、支付时间等关键字段结构化识别输出。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云支付详情页识别，是阿里云官方自研OCR文字识别产品，适用于识别支付详情页所包含的收款方名称、合计金额、付款方式、商品说明、支付时间等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i2/O1CN01pP14mQ1F2WjPhgXev_!!6000000000429-2-tps-821-313.png" width="50%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=nontax_invoice#intro)免费体验本功能识别效果。|
    /// |2|购买[票据凭证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20220303112214_0069%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizePaymentRecord?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。</li> </ul> |
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_payment_record(
        &self,
        req: RecognizePaymentRecord,
    ) -> impl std::future::Future<Output = crate::Result<RecognizePaymentRecordResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 电商订单页识别
    ///
    /// 支持包括订单编号、收货信息、交易金额、店铺名称、商品详单等关键字段结构化识别输出。
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_purchase_record(
        &self,
        req: RecognizePurchaseRecord,
    ) -> impl std::future::Future<Output = crate::Result<RecognizePurchaseRecordResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 网约车行程单识别
    ///
    /// 支持网约车行程单全部字段的识别，包括：服务商、申请日期、行程开始时间、行程结束时间、行程人手机号、总金额等字段。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_ride_hailing_itinerary(
        &self,
        req: RecognizeRideHailingItinerary,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeRideHailingItineraryResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 购物小票识别
    ///
    /// 支持包括开票方名称、开票日期、联系电话、地址、合计（实际）金额等关键字段结构化识别输出。
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_shopping_receipt(
        &self,
        req: RecognizeShoppingReceipt,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeShoppingReceiptResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 社会保障卡识别
    ///
    /// 支持全字段识别，包括标题、姓名、社会保障号码、社会保障卡号、银行账号、发卡日期等。
    ///
    /// 此接口不再更新，不支持新用户接入。请使用新版接口：[社保卡识别](https://help.aliyun.com/document_detail/442264.html)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_social_security_card(
        &self,
        req: RecognizeSocialSecurityCard,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeSocialSecurityCardResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 过路过桥费发票识别
    ///
    /// 支持包括发票代码、发票号码、金额、日期、车型、出口、入口等关键字段结构化识别输出。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_toll_invoice(
        &self,
        req: RecognizeTollInvoice,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeTollInvoiceResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 税收完税证明识别
    ///
    /// 支持包括税务机关、纳税人识别号、纳税人名称、合计金额、填票人、完税详单等关键字段的结构化识别输出。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云税收完税证明识别，是阿里云官方自研OCR文字识别产品，适用于识别非税收入证明所包含的税务机关、纳税人识别号、纳税人名称、合计金额、填票人、完税详单等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i4/O1CN01hmLCcX1JV9xJF1joS_!!6000000001033-2-tps-757-472.png" width="50%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=bill&subtype=tax_clearance_certificate#intro)免费体验本功能识别效果。|
    /// |2|购买[票据凭证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_invoice_dp_cn_20211222173940_0304%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeTaxClearanceCertificate?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。</li> </ul> |
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_tax_clearance_certificate(
        &self,
        req: RecognizeTaxClearanceCertificate,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeTaxClearanceCertificateResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 二手车统一销售发票识别
    ///
    /// 支持包括发票代码、发票号码、开票日期、发票金额、购买方名称、购买方身份证号等关键字段结构化识别输出。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_used_car_invoice(
        &self,
        req: RecognizeUsedCarInvoice,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeUsedCarInvoiceResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 营业执照识别
    ///
    /// 可快速精准的识别企业营业执照中的统一社会信用代码、公司名称、地址、主体类型、法定代表人、注册资金、组成形式、成立日期、营业期限和经营范围等关键有效字段。支持营业执照、民办非企业登记证书、社会团体法人登记证书、事业单位法人证书。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云营业执照识别，是阿里云官方自研OCR文字识别产品，适用于识别营业执照上的公司名称、地址、主体类型、法定代表人、注册资金、组成形式、成立日期等关键信息的场景。
    ///   * 泛营业执照包含民办非企业登记证书、社会团体法人登记证书、事业单位法人证书。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01bAhaqS1WxdaE0QfbB_!!6000000002855-0-tps-875-391.jpg" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |全字段识别|智能识别营业执照上所包含的全部字段。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|企事业名称、法人代表等文字信息准确率超过95%，营业执照注册号等数字信息准确率超过98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [企业资质识别](https://common-buy.aliyun.com/?commodityCode=ocr_enterprisecard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=assets&subtype=blicense#intro)免费体验本功能识别效果。|
    /// |2|购买[营业执照识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_enterprisecard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_enterprisecard_dp_cn_20211103184836_0975%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeBusinessLicense?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>请保证整张营业执照内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [云市场营业执照识别。](https://market.aliyun.com/products/57124001/cmapi013592.html?spm=5176.730005.result.41.7fc03524S3wFYv&innerSource=search_%E8%90%A5%E4%B8%9A%E6%89%A7%E7%85%A7#sku=yuncode759200000) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_business_license(
        &self,
        req: RecognizeBusinessLicense,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeBusinessLicenseResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 银行开户许可证识别
    ///
    /// 可快速精准的识别银行开户许可证中的账号、法定代表人、开户银行、核准号、企业名称、编号等关键信息。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云银行开户许可证识别，是阿里云官方自研OCR文字识别产品，适用于识别银行开户许可证所包含的账号、核准号、企业名称、法人姓名以及开户行等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01h572VA1PARjgZ1TyV_!!6000000001800-2-tps-819-316.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票据凭证识别](https://common-buy.aliyun.com/?commodityCode=ocr_invoice_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=assets&subtype=bank_account_permit#intro)免费体验本功能识别效果。|
    /// |2|购买[银行开户许可证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_enterprisecard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_enterprisecard_dp_cn_20211103184836_0059%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeBankAccountLicense?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul><li>接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。</li></ul>|
    /// |相关能力|<ul> <li> [云市场银行开户许可证识别。](https://market.aliyun.com/products/57124001/cmapi00042885.html?#sku=yuncode3688500001) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_bank_account_license(
        &self,
        req: RecognizeBankAccountLicense,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeBankAccountLicenseResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 商标注册证识别
    ///
    /// 可快速精准的识别商标注册证中所包含的商标名称、注册人、注册人地址以及有效期限、核定服务项目等关键有效字段信息。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_trade_mark_certification(
        &self,
        req: RecognizeTradeMarkCertification,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeTradeMarkCertificationResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 食品生产许可证识别
    ///
    /// 可快速精准的识别食品生产许可证所包含经营者名称、社会信用代码、法定代表人姓名、地址、经营场所、经营项目、有效期、许可证编号等关键字段信息。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云食品生产许可证识别，是阿里云官方自研OCR文字识别产品。适用于识别食品生产许可证社会信用代码、发证机关、生产地址、签发日期等信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1YaMhXKT2gK0jSZFvXXXnFXXa-1414-1000.png" width="50%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [企业资质识别](https://common-buy.aliyun.com/?commodityCode=ocr_enterprisecard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=assets&subtype=food_plicense#intro)免费体验本功能识别效果。|
    /// |2|购买 [企业资质识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_enterprisecard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_enterprisecard_dp_cn_20211103184836_0461%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeFoodProduceLicense?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_food_produce_license(
        &self,
        req: RecognizeFoodProduceLicense,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeFoodProduceLicenseResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 食品经营许可证识别
    ///
    /// 可快速精准的识别食品经营许可证所包含生产者名称、社会信用代码、法定代表人姓名、地址、生产场所、食品类别、有效期、许可证编号等关键字段信息。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云食品经营许可证识别，是阿里云官方自研OCR文字识别产品，适用于识别食品经营许可证上的经营者名称、法定代表人名称、社会信用代码等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///    * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1Rwa_N1H2gK0jSZJnXXaT1FXa-2060-800.jpg" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图片格式|支持PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [企业资质识别](https://common-buy.aliyun.com/?commodityCode=ocr_enterprisecard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=assets&subtype=food_blicense#intro)免费体验本功能识别效果。|
    /// |2|购买[食品经营许可证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_enterprisecard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_enterprisecard_dp_cn_20211103184836_0564%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeFoodManageLicense?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |相关能力|<ul> <li> [云市场食品经营许可证识别。](https://market.aliyun.com/products/57124001/cmapi033384.html?spm=a2c4g.11186623.0.0.43f6525aYt7UN6#sku=yuncode2738400001) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_food_manage_license(
        &self,
        req: RecognizeFoodManageLicense,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeFoodManageLicenseResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 医疗器械经营许可证识别
    ///
    /// 可快速精准的识别医疗器械经营许可证所包含许可证编号、企业名称、注册地址、法定代表人、企业负责人、质量管理人、仓库地址、经营范围、许可期限、发证日期等关键字段信息。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_medical_device_manage_license(
        &self,
        req: RecognizeMedicalDeviceManageLicense,
    ) -> impl std::future::Future<
        Output = crate::Result<RecognizeMedicalDeviceManageLicenseResponse>,
    > + Send {
        self.call(req)
    }

    ///
    /// # 医疗器械生产许可证识别
    ///
    /// 可快速精准的识别医疗器械生产许可证所包含许可证编号、法定代表人、企业名称、注册地址、生产地址、生产范围、企业负责人、有效期限等关键字段信息。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_medical_device_produce_license(
        &self,
        req: RecognizeMedicalDeviceProduceLicense,
    ) -> impl std::future::Future<
        Output = crate::Result<RecognizeMedicalDeviceProduceLicenseResponse>,
    > + Send {
        self.call(req)
    }

    ///
    /// # 第二类医疗器械经营备案凭证识别
    ///
    /// 可快速精准的识别第二类医疗器械经营备案凭证所包含备案编号、企业名称、住所、经营场所、库房地址、经营方式、法定代表人、企业负责人、经营范围、许可期限、备案日期等关键字段信息。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云第二类医疗器械经营备案凭证识别，是阿里云官方自研OCR文字识别产品。适用于识别第二类医疗器械经营备案凭证备案编号、企业名称、经营方式、法定代表人、经营范围等信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i4/O1CN01UQj0kH1u8qOmEehui_!!6000000005993-0-tps-2066-802.jpg" width="45%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [企业资质识别](https://common-buy.aliyun.com/?commodityCode=ocr_enterprisecard_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=assets&subtype=c2_medical_device_op_record#intro)免费体验本功能识别效果。|
    /// |2|购买 [企业资质识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_enterprisecard_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_enterprisecard_dp_cn_20211103184836_0131%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeCtwoMedicalDeviceManageLicense?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_ctwo_medical_device_manage_license(
        &self,
        req: RecognizeCtwoMedicalDeviceManageLicense,
    ) -> impl std::future::Future<
        Output = crate::Result<RecognizeCtwoMedicalDeviceManageLicenseResponse>,
    > + Send {
        self.call(req)
    }

    ///
    /// # 化妆品生产许可证识别
    ///
    /// 支持关键字段识别，包括证照名称、企业名称、社会信用代码、住址、法定代表人、许可证编号等。
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_cosmetic_produce_license(
        &self,
        req: RecognizeCosmeticProduceLicense,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeCosmeticProduceLicenseResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 国际企业执照识别
    ///
    /// 支持韩国、印度营业执照类型，提供包括证件类型、公司名称、注册号、法人姓名、签发日期等关键字段的识别能力。
    ///
    /// # Error Codes
    /// - `illegalCountryName`: the country name is not supported.
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_international_business_license(
        &self,
        req: RecognizeInternationalBusinessLicense,
    ) -> impl std::future::Future<
        Output = crate::Result<RecognizeInternationalBusinessLicenseResponse>,
    > + Send {
        self.call(req)
    }

    ///
    /// # 行驶证识别
    ///
    /// 支持对行驶证正页、副页关键字段的自动定位和识别，同时也支持对正副页在同一张图片的场景进行自动分割与结构化识别。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云行驶证识别，是阿里云官方自研OCR文字识别产品，精准定位和识别行驶证正、副页所包含的关键信息，支持正副页在同一张图片的场景进行自动分割与结构化识别。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i2/O1CN01ZihUIy1bCzJiSNPrk_!!6000000003430-0-tps-1323-828.jpg" width="70%"></p>
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i2/O1CN017IZ97Q1TdzkT25B2F_!!6000000002406-0-tps-989-517.jpg" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |行驶证混贴|支持对正副页在同一张图片的场景进行自动分割与结构化识别。|
    /// |高精度识别|总体准确率达93%以上。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [车辆物流识别](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=logistics&subtype=vehicle_license#intro)免费体验本功能识别效果。|
    /// |2|购买[行驶证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_logistics_dp_cn_20211103160032_0739%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeVehicleLicense?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |国家与语言| <ul> <li>本接口只支持中国行驶证。</li></ul> |
    /// |其他提示|<ul> <li>请保证整张行驶证内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [行驶证识别。](https://market.aliyun.com/products/57124001/cmapi011791.html?spm=5176.730005.result.13.291d3524fc1E2j&innerSource=search_%E8%A1%8C%E9%A9%B6%E8%AF%81#sku=yuncode579100000) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_vehicle_license(
        &self,
        req: RecognizeVehicleLicense,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeVehicleLicenseResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 驾驶证识别
    ///
    /// 支持对驾驶证上的姓名、证号、国籍、住址、初次领证日期、准驾类型、有效期等字段进行结构化提取。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云驾驶证识别，是阿里云官方自研OCR文字识别产品，适用于获取驾驶证上的姓名、证号、国籍、住址、准驾类型、初次领证日期、有效期等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i4/O1CN016MD6rM1Clv8MfB6Uz_!!6000000000122-0-tps-787-431.jpg" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |全字段识别|智能识别营业执照上所包含的全部字段。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体准确率和召回率达95%以上。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [车辆物流识别](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=logistics&subtype=driving_license#intro)免费体验本功能识别效果。|
    /// |2|购买[驾驶证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_logistics_dp_cn_20211103160032_0001%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeDrivingLicense?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |国家与语言| <ul> <li>本接口只支持中国驾驶证。</li></ul> |
    /// |其他提示|<ul> <li>请保证整张驾驶证内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [云市场驾驶证识别。](https://market.aliyun.com/products/57002002/cmapi010402.html?spm=5176.730005.result.29.34d635246xDdY0&innerSource=search_%E9%A9%BE%E9%A9%B6%E8%AF%81) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_driving_license(
        &self,
        req: RecognizeDrivingLicense,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeDrivingLicenseResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 电子面单识别
    ///
    /// 支持识别面单上所有关键字段。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云电子面单识别，是阿里云官方自研OCR文字识别产品，适用于自动提取面单上的手机号进行拨打收件人号码或发短信，减少快递员拨号时间；可快速定位面单上的所需信息，提升快递转运效率。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://help-static-aliyun-doc.aliyuncs.com/assets/img/zh-CN/7133931661/p480011.png" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |全字段识别|智能识别快递运单上所包含的全部字段。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|识别准确率可达93%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [车辆物流识别](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=logistics&subtype=waybill#intro)免费体验本功能识别效果。|
    /// |2|购买[电子面单识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_logistics_dp_cn_20211103160032_0823%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeWaybill?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li> 接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。 </li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li>  </ul>|
    /// |相关能力|<ul> <li> [云市场电子面单识别。](https://market.aliyun.com/products/57124001/cmapi00043511.html?spm=a2c4g.11186623.0.0.4efc4288F9Ffm7#sku=yuncode3751100001) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_waybill(
        &self,
        req: RecognizeWaybill,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeWaybillResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 车牌识别
    ///
    /// 可有效识别车辆车牌信息，支持机动车车牌、摩托车车牌以及临时车牌。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云车牌识别，是阿里云官方自研OCR文字识别产品，可有效识别车辆车牌信息，支持多车牌以及多类车型检测识别。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01ornmBX24I8d09nFQV_!!6000000007367-0-tps-661-264.jpg" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多地区车牌识别|浙、苏、赣、黑、鄂、川、甘、陕、吉、辽、闽、皖等。|
    /// |多车型识别|大型汽车、小型汽车、新能源车、挂车、临时车牌、警车、军车、使领馆车、教练车、港澳车。|
    /// |使用场景|广泛应用于车辆安防检控、车辆出入识别等场景。|
    /// |高精度识别|总体准确率达93%以上。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [车辆物流识别](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=logistics&subtype=driving_license#intro)免费体验本功能识别效果。|
    /// |2|购买[车牌识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_logistics_dp_cn_20211103160032_0692%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeCarNumber?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>请保证整张车牌内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [云市场车牌识别。](https://market.aliyun.com/products/57124001/cmapi020094.html?spm=5176.730005.result.17.517535242WwEyb&innerSource=search_%E8%BD%A6%E7%89%8C%E8%AF%86%E5%88%AB#sku=yuncode1409400000) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_car_number(
        &self,
        req: RecognizeCarNumber,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeCarNumberResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 车辆vin码识别
    ///
    /// 支持识别车辆VIN码。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云VIN码识别，是阿里云官方自研OCR文字识别产品，适用于识别车辆上的VIN码，用于进行车辆质检检查、车辆登记的等场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1.NXUdLzO3e4jSZFxXXaP_FXa-1600-800.jpg" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图片格式|PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [车辆物流识别](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=logistics&subtype=car_vin#intro)免费体验本功能识别效果。|
    /// |2|购买[车辆vin码识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_logistics_dp_cn_20211103160032_0057%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeCarVinCode?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>请保证整张车牌内容及其边缘包含在图像内。 </li> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [云市场车辆vin码识别。](https://market.aliyun.com/products/57124001/cmapi023049.html?spm=a2c4g.11186623.0.0.3ecb4288iZinrC#sku=yuncode1704900000) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_car_vin_code(
        &self,
        req: RecognizeCarVinCode,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeCarVinCodeResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 机动车注册登记证识别
    ///
    /// 可快速精准的识别机动车注册证所包含证件类别、条形编码、登记机关、登记日期、机动车登记编号等关键字段信息。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云机动车注册登记证识别，是阿里云官方自研OCR文字识别产品，适用于识别机动车注册证所包含的证件类型、编号、机动车所有人、登记机关、登记日期等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i3/O1CN01Bi3iwu1EWT0KLLzgX_!!6000000000359-2-tps-635-368.png" width="70%"></p>
    ///
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [车辆物流识别](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=logistics&subtype=vehicle_register#intro)免费体验本功能识别效果。|
    /// |2|购买[车辆物流识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_logistics_dp_cn_20211222171406_0783%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeVehicleRegistration?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li> 本能力会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [云市场机动车注册登记证识别。](https://market.aliyun.com/products/57124001/cmapi00038697.html?#sku=yuncode3269700001) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_vehicle_registration(
        &self,
        req: RecognizeVehicleRegistration,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeVehicleRegistrationResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 车辆合格证识别
    ///
    /// 支持车辆型号、车辆识别代号、底盘型号、发动机型号等字段进行结构化提取。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云车辆合格证识别，是阿里云官方自研OCR文字识别产品，适用于识别车辆合格证所包含的车辆型号、车辆识别代号、地盘型号、发动机型号等关键信息的场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01myFZQ91pMyaGJpRZn_!!6000000005347-2-tps-562-316.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达97%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [车辆物流识别](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=logistics&subtype=vehicle_certificate#intro)免费体验本功能识别效果。|
    /// |2|购买[车辆合格证识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_logistics_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_logistics_dp_cn_20211222171406_0433%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeVehicleCertification?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li>  </ul>|
    /// |相关能力|<ul> <li> [云市场车辆合格证识别。](https://market.aliyun.com/products/57124001/cmapi00049687.html?#sku=yuncode4368700002) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_vehicle_certification(
        &self,
        req: RecognizeVehicleCertification,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeVehicleCertificationResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 印刷体数学公式识别
    ///
    /// 支持印刷体的数学公式识别。
    ///
    /// #### 本接口适用场景
    ///   * 阿里云公式识别，是阿里云官方自研OCR文字识别产品，适用于题目录入、智能批改、作业批改等应用场景。
    ///   * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    ///   * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01tW1cGY1U1LxDXWmtJ_!!6000000002457-2-tps-641-318.png" width="60%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [教育场景识别](https://common-buy.aliyun.com/?commodityCode=ocr_education_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=edu&subtype=math_rec#intro)免费体验本功能识别效果。|
    /// |2|购买[教育场景识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_education_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_education_dp_cn_20211103164555_0495%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.html)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeEduFormula?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。</li> </ul> |
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_edu_formula(
        &self,
        req: RecognizeEduFormula,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeEduFormulaResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 口算判题
    ///
    /// 可以识别小学数学口算题目并给出题目判断结果。可支持整数的加减乘除四则运算、整数的混合运算、大小比较、最大数最小数等。
    ///
    /// #### 本接口适用场景
    /// * 阿里云口算判题识别，是阿里云官方自研OCR文字识别产品，适用于整数的加减乘除四则运算、整数的混合运算、大小比较、最大数最小数等的场景。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01JwIWUI1UyR34OnMHv_!!6000000002586-2-tps-636-316.png" width="50%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达97%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [教育场景识别](https://common-buy.aliyun.com/?commodityCode=ocr_education_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=edu&subtype=kousuan#intro)免费体验本功能识别效果。|
    /// |2|购买[教育场景识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_education_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_education_dp_cn_20211103164555_0373%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeEduOralCalculation?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [云市场口算判题。](https://market.aliyun.com/products/57124001/cmapi00043293.html?#sku=yuncode3729300001) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_edu_oral_calculation(
        &self,
        req: RecognizeEduOralCalculation,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeEduOralCalculationResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 整页试卷识别
    ///
    /// 支持K12全学科扫描场景的整页内容文字识别。接口支持印刷体文本及公式的OCR识别和坐标返回，此外，接口还可对题目中的配图位置进行检测并返回坐标位置。
    ///
    /// #### 本接口适用场景
    /// * 阿里云整页试卷识别，是阿里云官方自研OCR文字识别产品，适用于对练习册、教辅、教材等内容进行整页识别与题目检索场景。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i4/O1CN016bI8WV1TWfQ4ocurU_!!6000000002390-2-tps-739-450.png" width="50%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达97%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [教育场景识别](https://common-buy.aliyun.com/?commodityCode=ocr_education_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=edu&subtype=paper_ocr#intro)免费体验本功能识别效果。|
    /// |2|购买[教育场景识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_education_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_education_dp_cn_20211103164555_0789%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeEduPaperOcr?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_edu_paper_ocr(
        &self,
        req: RecognizeEduPaperOcr,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeEduPaperOcrResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 试卷切题识别
    ///
    /// 支持各学科的教辅试卷的结构化电子录入，将试卷中的题目进行自动化切分和结构化打标，并进行对应题目、题干、选项、答案等内容的结构化输出。
    ///
    /// #### 本接口适用场景
    /// * 阿里云试卷切题识别，是阿里云官方自研OCR文字识别产品，适用于识别整页练习册、试卷或教辅中的题目的场景，适用于教育材料内容的数字化生产与题库录入。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i1/O1CN01DOZA301QjXXwGP8uJ_!!6000000002012-2-tps-1030-942.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |服务|自动切题，并识别其中所包含的文字内容和坐标位置。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |适用范围广|支持K12全学科、多版式扫描版印刷体的整页切题场景。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [教育场景识别](https://common-buy.aliyun.com/?commodityCode=ocr_education_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=edu&subtype=paper_cut#intro)免费体验本功能识别效果。|
    /// |2|购买[试卷切题识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_education_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_education_dp_cn_20211103164555_0194%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeEduPaperCut?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> URL长度不能超过2048。 </li>  <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul> <li> [云市场扫描版试卷切题识别。](https://market.aliyun.com/products/57124001/cmapi00042623.html?#sku=yuncode3662300001) </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_edu_paper_cut(
        &self,
        req: RecognizeEduPaperCut,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeEduPaperCutResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 题目识别
    ///
    /// 可对题目进行有效识别。通过对题目的元素进行打标，提升题目的识别效果。
    ///
    /// #### 本接口适用场景
    /// * 阿里云题目识别，是阿里云官方自研OCR文字识别产品，适用于扫描、拍照场景的单题题目识别，适用于智能批改等场景的题目内容识别。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1KESJj639YK4jSZPcXXXrUFXa-1030-400.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |多文本格式|支持印刷体文本以及公式的OCR识别。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |返回坐标|可实现对题目中的配图位置进行检测并返回坐标位置。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [教育场景识别](https://common-buy.aliyun.com/?commodityCode=ocr_education_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=edu&subtype=question_ocr#intro)免费体验本功能识别效果。|
    /// |2|购买[题目识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_education_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_education_dp_cn_20211103164555_0111%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeEduQuestionOcr?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_edu_question_ocr(
        &self,
        req: RecognizeEduQuestionOcr,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeEduQuestionOcrResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 精细版结构化切题
    ///
    /// 支持多学科教辅试卷的结构化识别，将整页练习册、试卷或教辅中的题目进行自动切题，并识别出其中的文字内容和坐标位置。
    ///
    /// #### 本接口适用场景
    /// * 阿里云精细版结构化切题，是阿里云官方自研OCR文字识别产品，适用于整页练习册、试卷或教辅种的题目场景。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i3/O1CN01XxrLu71rjXK95i1lW_!!6000000005667-2-tps-1147-626.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |智能识别|自动切题，并识别其中的全部字段和坐标位置。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [教育场景识别](https://common-buy.aliyun.com/?commodityCode=ocr_education_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=edu&subtype=paper_structured#intro)免费体验本功能识别效果。|
    /// |2|购买[精细版结构化切题资源包](https://common-buy.aliyun.com/?commodityCode=ocr_education_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_education_dp_cn_20211103164555_0978%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeEduPaperStructed?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> URL长度不能超过2048。 </li>  <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_edu_paper_structed(
        &self,
        req: RecognizeEduPaperStructed,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeEduPaperStructedResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 通用多语言识别
    ///
    /// 支持国际主流几大语系的自动语言分类判定并返回对应语言的文字信息。
    ///
    /// #### 本接口适用场景
    /// * 阿里云通用多语言证识别，是阿里云官方自研OCR文字识别产品，适用于国际化所需的各类图文识别与信息翻译场景。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i4/O1CN01tVz6Eh1eY0Lb3pUGZ_!!6000000003882-2-tps-640-368.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [通用文字识别](https://common-buy.aliyun.com?commodityCode=ocr_general_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=i18n&subtype=languages#intro)免费体验本功能识别效果。|
    /// |2|购买[小语种识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_multilanguage_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_multilanguage_dp_cn_20211103180438_0408%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeMultiLanguage?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> <li> 图片尺寸过小，会影响识别精度。图片内单字大小在10-50px内时，识别效果较好。 </li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    /// |相关能力|<ul><li>[云市场通用多语言识别。](https://market.aliyun.com/products/57124001/cmapi00040847.html?#sku=yuncode3484700001)</li></ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_multi_language(
        &self,
        req: RecognizeMultiLanguage,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeMultiLanguageResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 英语作文识别
    ///
    /// 针对全英文图片文档场景下英文印刷体字符高效检测和识别，具备英文专项识别和英文分词功能，支持旋转、表格、文字坐标等多项基础功能。
    ///
    /// #### 本接口适用场景
    /// * 阿里云英语专项识别，是阿里云官方自研OCR文字识别产品，适用于全英文图片、文档场景下的英文印刷体字符的高效检测和识别。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/tfs/TB1K2a4NVY7gK0jSZKzXXaikpXa-2060-800.jpg" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |功能|具备英文专项识别和英文分词功能。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |多卡面类型|支持各种位数、凸字卡面、平面卡面的识别。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [小语种识别](https://common-buy.aliyun.com/?commodityCode=ocr_multilanguage_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=i18n&subtype=eng#intro)免费体验本功能识别效果。|
    /// |2|购买[英语作文识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_multilanguage_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_multilanguage_dp_cn_20211103180438_0108%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeEnglish?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_english(
        &self,
        req: RecognizeEnglish,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeEnglishResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 泰语识别
    ///
    /// 针对泰语图片文档场景下泰文印刷体高效检测和识别，支持旋转、表格、文字坐标等多项基础功能。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_thai(
        &self,
        req: RecognizeThai,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeThaiResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 日语识别
    ///
    /// 针对全日文图片文档场景下日文印刷体高效检测和识别，支持旋转、表格、文字坐标等多项基础功能。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_janpanese(
        &self,
        req: RecognizeJanpanese,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeJanpaneseResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 韩语识别
    ///
    /// 针对韩语图片文档场景下韩文印刷体高效检测和识别，支持旋转、表格、文字坐标等多项基础功能。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_korean(
        &self,
        req: RecognizeKorean,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeKoreanResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 拉丁语识别
    ///
    /// 针对拉丁语系的图片文档场景下印刷体高效检测和识别，支持旋转、表格、文字坐标等多项基础功能。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_latin(
        &self,
        req: RecognizeLatin,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeLatinResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 俄语识别
    ///
    /// 针对图片文档场景下俄文印刷体高效检测和识别，支持旋转、表格、文字坐标等多项基础功能。
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_russian(
        &self,
        req: RecognizeRussian,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeRussianResponse>> + Send {
        self.call(req)
    }

    ///
    /// # 核酸检测报告识别
    ///
    /// 支持对全国各地区不同版式的核酸检测记录中姓名、证件号码、采样日期、采样时间、检测机构、检测结果等6个关键字段的结构化结果输出。
    ///
    /// #### 本接口适用场景
    /// * 阿里云核酸检测报告识别，是阿里云官方自研OCR文字识别产品，适用于识别核酸检测报告上的姓名、证件号码、采样时间、检测结果等关键信息的场景。
    /// * 阿里云OCR产品基于阿里巴巴达摩院强大的AI技术及海量数据，历经多年沉淀打磨，具有服务稳定、操作简易、实时性高、能力全面等几大优势。
    /// * 本接口图片示例
    /// <p style="text-align:center"><img src="https://img.alicdn.com/imgextra/i2/O1CN01qWUm4s1kF7eX52tJy_!!6000000004653-2-tps-1921-831.png" width="70%"></p>
    ///
    /// #### 本接口核心能力
    ///   
    /// |分类 |概述|
    /// |---|---------|
    /// |图片格式|支持PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。|
    /// |图像增强|默认支持图像增强，包括图像自动旋转、畸变自动矫正、模糊图片自动增强等能力。|
    /// |多类型覆盖|支持模糊、光照不均、透视畸变、任意背景等低质量图像识别。|
    /// |高精度识别|总体识别准确率可达98%。|
    ///
    /// #### 如何使用本接口
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [医疗场景识别](https://common-buy.aliyun.com/?commodityCode=ocr_medical_public_cn) 服务。开通服务前后，您可以通过[体验馆](https://duguang.aliyun.com/experience?type=medicalCare&subtype=covid_test_report#intro)免费体验本功能识别效果。|
    /// |2|购买[核酸检测报告识别资源包](https://common-buy.aliyun.com/?commodityCode=ocr_medical_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_medical_dp_cn_20220506154332_0030%22,%22flowout_spec%22:%22500%22%7D)。本API会赠送免费额度，可使用免费额度测试。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/RecognizeCovidTestReport?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// #### 重要提示
    /// |类型|概述|
    /// |----|-------------------|
    /// |图片格式|<ul> <li>本接口支持：PNG、JPG、JPEG、BMP、GIF、TIFF、WebP。暂不支持PDF格式。</li></ul>|
    /// |图片尺寸|<ul> <li> 图片长宽需要大于15像素，小于8192像素。</li> <li>长宽比需要小于50。</li> <li>如需达到较好识别效果，建议长宽均大于500px。</li> </ul>|
    /// |图片大小|<ul> <li> 图片二进制文件不能超过10MB。</li> <li> 图片过大会影响接口响应速度，建议使用小于1.5M图片进行识别，且通过传图片URL的方式调用接口。</li> </ul>|
    /// |其他提示|<ul> <li>接口响应速度和图片中的文字数量有关，如果图片中文字数量越多，接口响应可能越慢。</li> <li> 接口会自动处理反光、扭曲等干扰信息，但会影响精度。请尽量选择清晰度高、无反光、无扭曲的图片。 </li> </ul>|
    ///
    /// # Extra Info
    /// SDK调用
    /// 通过SDK调用此接口的示例请参考[开发者中心](https://next.api.aliyun.com/api-tools/sdk/ocr-api?version=2021-07-07&language=java-tea)
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn recognize_covid_test_report(
        &self,
        req: RecognizeCovidTestReport,
    ) -> impl std::future::Future<Output = crate::Result<RecognizeCovidTestReportResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 营业执照核验
    ///
    /// 营业执照三要素核验支持通过输入营业执照的统一信用社会代码（工商注册号）、企业名称、法人姓名做一致性验证。
    ///
    /// #### 如何开通/购买本服务
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票证核验](https://common-buy.aliyun.com/?spm=5176.28059030.0.0.68c21287dXK0hR&commodityCode=ocr_cardverification_public_cn) 服务。开通后您可享50次免费额度。|
    /// |2|购买[营业执照核验资源包](https://common-buy.aliyun.com/?commodityCode=ocr_cardverification_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_cardverification_dp_cn_20220826134917_0698%22,%22flowout_spec%22:%221000%22%7D%E3%80%91)。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.htm?spm=a2c4g.11186623.0.0.22878a21R9tkuV)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/VerifyBusinessLicense?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// # Error Codes
    /// - `ServiceUnavailable`: The request has failed due to a temporary failure of the server
    ///
    /// # Extra Info
    /// * <span style="font-size:larger;"> <b> 返回结果示例 </b></span>
    /// ```ignore
    /// // 三要素核验一致
    /// {
    ///   "data": "{\"code\":\"0\",\"data\":true,\"message\":\"法人姓名、社会信用代码一致\"}",
    ///   "requestId": "E5CDCE98-EC32-16AC-B851-EDFC4C9B7F2D"
    /// }
    ///
    ///
    /// // 公司不存在
    /// {
    ///   "data": "{\"code\":\"20001\",\"data\":false,\"message\":\"此公司在数据库中不存在！\"}",
    ///   "requestId": "E5CDCE98-EC32-16AC-B851-EDFC4C9B7F2D"
    /// }
    ///
    ///
    /// // 法人名字不一致
    /// {
    ///   "data": "{\"code\":\"20002\",\"data\":false,\"message\":\"输入的法人名字和公司法人名字不一致\"}",
    ///   "requestId": "E5CDCE98-EC32-16AC-B851-EDFC4C9B7F2D"
    /// }
    ///
    ///
    /// // 三要素都不一致
    /// {
    ///   "data": "{\"code\":\"20003\",\"data\":false,\"message\":\"传入的注册号与工商注册号和统一社会信用代码都不一致\"}",
    ///   "requestId": "E5CDCE98-EC32-16AC-B851-EDFC4C9B7F2D"
    /// }
    ///
    /// ```
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn verify_business_license(
        &self,
        req: VerifyBusinessLicense,
    ) -> impl std::future::Future<Output = crate::Result<VerifyBusinessLicenseResponse>> + Send
    {
        self.call(req)
    }

    ///
    /// # 发票核验
    ///
    /// 发票核验接口支持包括：增值税专用发票、增值税普通发票（折叠票）、增值税普通发票（卷票）、增值税电子普通发票（含收费公路通行费增值税电子普通发票）、机动车销售统一发票、二手车销售统一发票多种类型发票核验。您可以通过输入发票的关键验证字段，返回真实的票面信息，包括发票类型、发票代码、发票号码、作废标志、开票日期、购方税号及其他发票信息等。当天开具发票当日可查验（T+0）。注意：可能有几小时到十几小时的延迟。
    ///
    /// #### 如何开通/购买本服务
    ///
    /// |步骤|概述|
    /// |--|---|
    /// |1|开通 [票证核验](https://common-buy.aliyun.com/?spm=5176.28059030.0.0.68c21287dXK0hR&commodityCode=ocr_cardverification_public_cn) 服务。开通后您可享50次免费额度。|
    /// |2|购买[发票核验资源包](https://common-buy.aliyun.com/?commodityCode=ocr_cardverification_dp_cn&request=%7B%22ord_time%22:%221:Year%22,%22order_num%22:1,%22pack%22:%22ocr_cardverification_dp_cn_20220830140650_0595%22,%22flowout_spec%22:%221000%22%7D)。您也可以不购买资源包，系统会通过“[按量付费](https://help.aliyun.com/document_detail/295347.htm?spm=a2c4g.11186623.0.0.22878a21R9tkuV)”方式按实际调用量自动扣款。|
    /// |3|可以参照[调试页面](https://next.api.aliyun.com/api/ocr-api/2021-07-07/VerifyVATInvoice?sdkStyle=dara)提供的代码示例完成API接入开发。接入完成后，调用API获取识别结果。如果使用子账号调用接口，需要阿里云账号（主账号）对RAM账号进行授权。创建RAM用户的具体操作，请参考：[创建RAM用户。](https://help.aliyun.com/document_detail/93720.html)文字识别服务提供一种系统授权策略，即**AliyunOCRFullAccess**。具体授权操作，请参见[在用户页面为RAM用户授权。](https://help.aliyun.com/document_detail/116146.html)|
    ///
    /// # Error Codes
    /// - `ServiceUnavailable`: The request has failed due to a temporary failure of the server
    ///
    /// # Methods
    /// - Get
    /// - Post
    ///
    pub fn verify_vat_invoice(
        &self,
        req: VerifyVATInvoice,
    ) -> impl std::future::Future<Output = crate::Result<VerifyVATInvoiceResponse>> + Send {
        self.call(req)
    }
}

/// * 本接口请求参数可分为三级，一级入参是**必传**的基础参数，例如图片链接、图片类型。二级参数可以控制识别内容输出，例如是否返回坐标等。
/// 三级参数和特定的图片类型相关，用于控制是否输出特定信息，例如是否输出身份证的质量检测分数。注意，只有**Type**是必传参数，其余参数可以根据需要设置。
/// #### 图片类型（Type）支持的请求参数补充说明
/// | Type        | 类型描述       | 支持的参数|
/// | ----------- | ------ | --- |
/// | Advanced    | 通用文字识别高精版 | <ul> <li>OutputFigure（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false）</li> <li>AdvancedConfig（**通用识别高精版**专有参数，默认：空）</li></ul> |
/// | General     | 通用文字识别基础版 | <ul> <li>OutputStamp（默认：false）</li> </li></ul> |
/// | Commerce    | 电商图片文字   | <ul> <li>OutputStamp（默认：false）</li> </li></ul>|   
/// | HandWriting | 手写文字  | <ul> <li>OutputFigure（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li></ul>|   
/// | MultiLang | 多语言文字   | <ul> <li>OutputFigure（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li>MultiLanConfig（**多语言通用类型**专有参数，默认：空）</li> </ul>|   
/// | Table | 表格   | <ul> <li>OutputFigure（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li>TableConfig（**表格类型**专有参数，默认：空）</li> </ul>|   
/// | IdCard | 身份证 | <ul> <li>OutputFigure（默认：false）</li> <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：**true**） </li>  <li>IdCardConfig（**身份证**专有参数，默认：空）</li> </ul>|   
/// | BankCard | 银行卡  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | InternationalPassport | 国际护照  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：**true**） </li> </ul>|   
/// | ChinesePassport | 中国护照  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | SocialSecurityCard | 社保卡  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | PermitToHK_MO_TW | 往来港澳台通行证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | PermitToMainland | 来往中国大陆（内地）通行证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | HouseholdHead | 户口本首页 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | HouseholdResident | 户口本常住人口页 |  <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | EstateCertification | 不动产权证 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：true）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | BirthCertification | 出生证明 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | HKIdCard | 中国香港身份证 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：**true**） </li> </ul>|   
/// | InternationalIdCard | 国际身份证 |  <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> InternationalIdCardConfig（**国际身份证**专有参数，默认：空） </li> </ul>|   
/// | Stamp | 公章 | <ul> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> </ul>|   
/// | MixedInvoice | 混贴票证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li>PageNo（默认：1） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | Invoice | 增值税发票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li>PageNo（默认：1） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | CarInvoice | 机动车销售统一发票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | QuotaInvoice | 定额发票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li>PageNo（默认：1） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | AirItinerary | 航空行程单  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li>PageNo（默认：1） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | TrainTicket | 火车票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li>PageNo（默认：1） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | TollInvoice | 过路过桥费发票 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | RollTicket | 增值税发票卷票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li>PageNo（默认：1） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | BankAcceptance | 银行承兑汇票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | BusShipTicket | 客运车船票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | NonTaxInvoice | 非税收入发票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | CommonPrintedInvoice | 通用机打发票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li>PageNo（默认：1） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | HotelConsume | 酒店流水  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | PaymentRecord | 支付详情页  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | PurchaseRecord | 电商订单页  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | RideHailingItinerary | 网约车行程单  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | ShoppingReceipt | 购物小票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | TaxClearanceCertificate | 税收完税证明  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | UsedCarInvoice | 二手车销售统一发票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | VehicleLicense | 行驶证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | DrivingLicense | 驾驶证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | VehicleRegistration | 机动车登记证 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | VehicleCertification | 车辆合格证 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | LicensePlateNumber | 车牌 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | CarVinCode | 车辆vin码 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | BusinessLicense | 营业执照  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | InternationalBusinessLicense | 国际企业执照  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> <li> OutputKVExcel（默认：false）</li> <li> InternationalBusinessLicenseConfig （**国际企业执照**专有参数，默认：空）</li> </ul>|   
/// | MedicalDeviceManageLicense | 医疗器械经营许可证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | MedicalDeviceProduceLicense | 医疗器械生产许可证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | CosmeticProduceLicense | 化妆品生产许可证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|   
/// | QrCode | 二维码  | |
/// | BarCode | 条形码 | |
/// | TaxiInvoice | 出租车发票  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|
/// | TrademarkCertificate | 商标注册证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|
/// | FoodProduceLicense | 食品生产许可证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|
/// | FoodManagementLicense | 食品经营许可证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：**true**）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|
/// | ClassIIMedicalDeviceManageLicense | 第二类医疗器械经营备案凭证  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|
/// | WayBill | 电子面单  | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|
/// | BankAccountPermit | 银行开户许可证 | <ul> <li>OutputFigure（默认：false）</li>  <li>OutputQrCode（默认：false）</li> <li>OutputBarCode（默认：false）</li> <li>OutputStamp（默认：false）</li> <li>OutputCoordinate（默认：空）</li> <li>OutputOricoord（默认：false） </li> <li> OutputKVExcel（默认：false）</li> </ul>|
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeAllText {
    /// - 本字段和 body 字段二选一，不可同时透传或同时为空。
    /// - 图片链接（长度不超过2048字节，不支持 base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// - 本字段和 URL 字段二选一，不可同时透传或同时为空。
    /// - 图片二进制文件，最大 10MB。
    /// - 使用 HTTP 方式调用，把图片二进制文件放到 HTTP body 中上传即可。
    /// - 使用 SDK 的方式调用，把图片放到 SDK 的 body 中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 图片类型。**必选**参数，且为**单选**。
    /// * 支持的图片类型请参考 **请求参数补充说明**。
    /// * 请注意，对于票据卡证类图片，当图片真实类型和入参指定的**Type**不一致时，会导致识别失败。
    r#type: Type,
    /// - 是否需要图案检测功能。如果开启，会返回**FigureInfo**字段（详见返回参数说明）。
    /// - true：需要；false：不需要。
    /// - 默认值：不同图片类型（**Type**）的默认值不同，详见**请求参数补充说明**。
    /// - 支持识别的图案类型如下：
    ///     - blicense_title：营业执照标题
    ///     - national_emblem：国徽
    ///     - face：人脸图案
    ///     - finger_print：指纹
    ///     - signature：签名区域
    /// - **请注意**：开启此参数后，会增加接口的响应时间，请在需要识别图案时开启此参数。
    #[setters(generate = true, strip_option)]
    output_figure: Option<bool>,
    /// - 是否需要二维码检测功能。开启后会返回**QrCodeInfo**字段（详见返回参数说明）。
    /// - true：需要；false：不需要。
    /// - 默认值：false。
    /// - **请注意**：开启此参数后，会增加接口的响应时间，请在需要识别二维码时开启此参数。
    #[setters(generate = true, strip_option)]
    output_qrcode: Option<bool>,
    /// - 是否需要条形码检测功能。开启后会返回**BarCodeInfo**字段（详见返回参数说明）。
    /// - true：需要；false：不需要。
    /// - 默认值：false。
    /// - **请注意**：开启此参数后，会增加接口的响应时间，请在需要识别条形码时开启此参数。
    #[setters(generate = true, strip_option)]
    output_bar_code: Option<bool>,
    /// - 是否需要印章检测功能。开启后会返回**StampInfo**字段（详见返回参数说明）。
    /// - true：需要；false：不需要。
    /// - 默认值：false。
    /// - **请注意**：开启此参数后，会增加接口的响应时间，请在需要识别印章时开启此参数。
    #[setters(generate = true, strip_option)]
    output_stamp: Option<bool>,
    /// - 返回坐标格式（**points**、**rectangle**）。
    /// - points：四点坐标；rectangle：旋转矩形。
    /// - 默认不需要传此参数，不返回文字坐标。
    #[setters(generate = true, strip_option)]
    output_coordinate: Option<String>,
    /// - 是否需要返回原图坐标信息。 系统会自动对图片做处理（比如自动旋转、图片校正等），您可以设置返回的坐标口径，是“原图坐标”或“算法处理后图片坐标”。
    /// - true：需要；false：不需要。
    /// - 默认值：不同图片类型（**Type**）的默认值不同，详见**请求参数补充说明**。
    /// - **请注意**：仅当**OutputCoordinate**不为空时，设置此参数才有意义。
    #[setters(generate = true, strip_option)]
    output_oricoord: Option<bool>,
    /// - 是否需要把识别出的结构化信息转成 Excel 文件链接（默认不需要）。
    /// - true：需要；false：不需要。
    /// - 文件链接有效期为一小时。
    /// - **注意**：开启此参数后，会增加接口的响应时间，请在需要时开启。
    #[setters(generate = true, strip_option)]
    output_kv_excel: Option<bool>,
    /// 当图片类型为混贴票证/增值税发票/定额发票航空行程单/火车票增值税发票卷票/通用机打发票时（即Type=MixedInvoice/Invoice/QuotaInvoice/AirItinerary/TrainTicket/RollTicket/CommonPrintedInvoice），可通过本字段设置可选功能。
    /// - 指定识别的 PDF/OFD 页码；例如：PageNo=6，则识别 PDF/OFD 的第六页。
    /// - 如果不传此参数，或传值大于 PDF/OFD 总页数，则识别 PDF/OFD 的第一页。
    /// - 默认识别第一页。
    #[setters(generate = true, strip_option)]
    page_no: Option<i32>,
    /// * 当图片类型为通用文字识别高精版时（**Type=Advanced**），可通过本字段设置可选功能。
    #[setters(generate = true, strip_option)]
    advanced_config: Option<AdvancedConfig>,
    /// * 当图片类型为身份证时（**Type=IdCard**），可通过本字段设置可选功能。
    #[setters(generate = true, strip_option)]
    id_card_config: Option<TextIdCardConfig>,
    /// * 当图片类型为国际身份证时（Type=**InternationalIdCard**），可通过本字段设置可选功能。
    #[setters(generate = true, strip_option)]
    international_id_card_config: Option<InternationalIdCardConfig>,
    /// * 当图片类型为国际企业执照时（Type=**InternationalBusinessLicense**），可通过本字段设置可选功能。
    #[setters(generate = true, strip_option)]
    international_business_license_config: Option<LicenseConfig>,
    /// * 当图片类型为通用多语言文字时（Type=**MultiLang**），可通过本字段设置可选功能。
    #[setters(generate = true, strip_option)]
    multi_lan_config: Option<LanConfig>,
    /// * 当图片类型为表格时（Type=**Table**），可通过本字段设置可选功能。
    #[setters(generate = true, strip_option)]
    table_config: Option<TableConfig>,
}

impl sealed::Bound for RecognizeAllText {}

impl RecognizeAllText {
    pub fn new(r#type: impl Into<Type>) -> Self {
        Self {
            url: None,
            body: None,
            r#type: r#type.into(),
            output_figure: None,
            output_qrcode: None,
            output_bar_code: None,
            output_stamp: None,
            output_coordinate: None,
            output_oricoord: None,
            output_kv_excel: None,
            page_no: None,
            advanced_config: None,
            id_card_config: None,
            international_id_card_config: None,
            international_business_license_config: None,
            multi_lan_config: None,
            table_config: None,
        }
    }
}

impl crate::Request for RecognizeAllText {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeAllText";

    type Body = crate::OctetStream;

    type Response = RecognizeAllTextResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("Type".into(), (&self.r#type).into());

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_figure {
            params.insert("OutputFigure".into(), f.into());
        }

        if let Some(f) = &self.output_qrcode {
            params.insert("OutputQrcode".into(), f.into());
        }

        if let Some(f) = &self.output_bar_code {
            params.insert("OutputBarCode".into(), f.into());
        }

        if let Some(f) = &self.output_stamp {
            params.insert("OutputStamp".into(), f.into());
        }

        if let Some(f) = &self.output_coordinate {
            params.insert("OutputCoordinate".into(), f.into());
        }

        if let Some(f) = &self.output_oricoord {
            params.insert("OutputOricoord".into(), f.into());
        }

        if let Some(f) = &self.output_kv_excel {
            params.insert("OutputKVExcel".into(), f.into());
        }

        if let Some(f) = &self.page_no {
            params.insert("PageNo".into(), f.into());
        }

        if let Some(f) = &self.advanced_config {
            params.insert("AdvancedConfig".into(), serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.id_card_config {
            params.insert("IdCardConfig".into(), serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.international_id_card_config {
            params.insert(
                "InternationalIdCardConfig".into(),
                serde_json::to_string(f)?.into(),
            );
        }

        if let Some(f) = &self.international_business_license_config {
            params.insert(
                "InternationalBusinessLicenseConfig".into(),
                serde_json::to_string(f)?.into(),
            );
        }

        if let Some(f) = &self.multi_lan_config {
            params.insert("MultiLanConfig".into(), serde_json::to_string(f)?.into());
        }

        if let Some(f) = &self.table_config {
            params.insert("TableConfig".into(), serde_json::to_string(f)?.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// #### 请求注意事项
/// 1. 此接口 **30** 秒超时。
/// 2. 如果通过OCR SDK调用接口，SDK默认的 **socketTimeout** 为 **10** 秒。请注意通过适当增加 **RuntimeOptions** 中 **socketTimeout** 参数的值。
/// 3. 如果传入的Key数量过多，可能会返回 **LLMTimeout** 错误码。建议减少Key的数量后重试。
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeGeneralStructure {
    /// * 本字段和 body 字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超过2048字节，不支持 base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和 URL 字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大 10MB。
    /// * 使用 HTTP 方式调用，把图片二进制文件放到 HTTP body 中上传即可。
    /// * 使用 SDK 的方式调用，把图片放到 SDK 的 body 中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 需要抽取的所有Key（字符串数组）。
    /// * 默认值为**空**，表示由大模型自动判断需要抽取的Key。
    /// * Key的上限数量为**30**（包含30）。
    /// * 建议调用接口时传此参数，减小接口耗时。**请注意**：接口响应时间和Key的数量呈正相关关系。
    #[setters(generate = true, strip_option)]
    keys: Option<Vec<String>>,
}

impl sealed::Bound for RecognizeGeneralStructure {}

impl RecognizeGeneralStructure {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            keys: None,
        }
    }
}

impl crate::Request for RecognizeGeneralStructure {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeGeneralStructure";

    type Body = crate::OctetStream;

    type Response = RecognizeGeneralStructureResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.keys {
            crate::SimpleSerialize::simple_serialize(f, "Keys", &mut params);
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeAdvanced {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否输出单字识别结果，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_char_info: Option<bool>,
    /// * 是否需要自动旋转功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// * 是否输出表格识别结果，包含单元格信息，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
    /// * 是否按顺序输出文字块，默认为false。
    /// * false表示从左往右，从上到下的顺序；true表示从上到下，从左往右的顺序。
    #[setters(generate = true, strip_option)]
    need_sort_page: Option<bool>,
    /// * 是否需要图案检测功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_figure: Option<bool>,
    /// * 是否需要去除印章功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    no_stamp: Option<bool>,
    /// * 是否需要分段功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    paragraph: Option<bool>,
    /// * 是否需要成行返回功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    row: Option<bool>,
}

impl sealed::Bound for RecognizeAdvanced {}

impl RecognizeAdvanced {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_char_info: None,
            need_rotate: None,
            output_table: None,
            need_sort_page: None,
            output_figure: None,
            no_stamp: None,
            paragraph: None,
            row: None,
        }
    }
}

impl crate::Request for RecognizeAdvanced {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeAdvanced";

    type Body = crate::OctetStream;

    type Response = RecognizeAdvancedResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_char_info {
            params.insert("OutputCharInfo".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        if let Some(f) = &self.need_sort_page {
            params.insert("NeedSortPage".into(), f.into());
        }

        if let Some(f) = &self.output_figure {
            params.insert("OutputFigure".into(), f.into());
        }

        if let Some(f) = &self.no_stamp {
            params.insert("NoStamp".into(), f.into());
        }

        if let Some(f) = &self.paragraph {
            params.insert("Paragraph".into(), f.into());
        }

        if let Some(f) = &self.row {
            params.insert("Row".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeHandwriting {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否输出单字识别结果，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_char_info: Option<bool>,
    /// * 是否需要自动旋转功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// * 是否输出表格识别结果，包含单元格信息，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
    /// * 是否按顺序输出文字块，默认为false。
    /// * false表示从左往右，从上到下的顺序；true表示从上到下，从左往右的顺序。
    #[setters(generate = true, strip_option)]
    need_sort_page: Option<bool>,
    /// * 是否需要分段功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    paragraph: Option<bool>,
}

impl sealed::Bound for RecognizeHandwriting {}

impl RecognizeHandwriting {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_char_info: None,
            need_rotate: None,
            output_table: None,
            need_sort_page: None,
            paragraph: None,
        }
    }
}

impl crate::Request for RecognizeHandwriting {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeHandwriting";

    type Body = crate::OctetStream;

    type Response = RecognizeHandwritingResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_char_info {
            params.insert("OutputCharInfo".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        if let Some(f) = &self.need_sort_page {
            params.insert("NeedSortPage".into(), f.into());
        }

        if let Some(f) = &self.paragraph {
            params.insert("Paragraph".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeBasic {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否需要自动旋转功能，默认需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
}

impl sealed::Bound for RecognizeBasic {}

impl RecognizeBasic {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            need_rotate: None,
        }
    }
}

impl crate::Request for RecognizeBasic {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeBasic";

    type Body = crate::OctetStream;

    type Response = RecognizeBasicResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeGeneral {
    /// * 本字段和body字段二选一，不可同时透传或同时为空
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeGeneral {}

impl RecognizeGeneral {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeGeneral {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeGeneral";

    type Body = crate::OctetStream;

    type Response = RecognizeGeneralResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeTableOcr {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否需要自动旋转功能，默认需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// * 是否无线条或者只有横线没有竖线,默认有线条。
    /// * true：无线条；false：有线条。
    #[setters(generate = true, strip_option)]
    line_less: Option<bool>,
    /// * 是否跳过检测，默认为false。
    /// * true：跳过检查；false：不跳过检查。
    #[setters(generate = true, strip_option)]
    skip_detection: Option<bool>,
    /// * 是否是手写表格，默认不是。
    /// * true：是手写表格；false：不是手写表格。
    /// * 注意：该字段是字符串类型。
    #[setters(generate = true, strip_option)]
    is_hand_writing: Option<Writing>,
}

impl sealed::Bound for RecognizeTableOcr {}

impl RecognizeTableOcr {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            need_rotate: None,
            line_less: None,
            skip_detection: None,
            is_hand_writing: None,
        }
    }
}

impl crate::Request for RecognizeTableOcr {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeTableOcr";

    type Body = crate::OctetStream;

    type Response = RecognizeTableOcrResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.line_less {
            params.insert("LineLess".into(), f.into());
        }

        if let Some(f) = &self.skip_detection {
            params.insert("SkipDetection".into(), f.into());
        }

        if let Some(f) = &self.is_hand_writing {
            params.insert("IsHandWriting".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeHealthCode {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeHealthCode {}

impl RecognizeHealthCode {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeHealthCode {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeHealthCode";

    type Body = crate::OctetStream;

    type Response = RecognizeHealthCodeResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
/// * PDF文件格式的文档结构化解析请点击[文档智能解析](https://docmind.console.aliyun.com/file/docAnalysis)快速了解
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeDocumentStructure {
    /// 图片链接（长度不超2048字节，不支持 base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。
    /// 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。
    /// 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 是否需要自动旋转功能，返回角度信息。默认不需要。true：需要 false：不需要。
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// 是否输出单字识别结果，默认不需要。true：需要 false：不需要。
    /// 当UseNewStyleOutput=true时，此参数不生效。
    #[setters(generate = true, strip_option)]
    output_char_info: Option<bool>,
    /// 是否输出表格识别结果，包含单元格信息。默认不需要。true：需要 false：不需要。
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
    /// 是否按顺序输出文字块，默认不需要。true：需要 false：不需要。false表示从左往右，从上到下的顺序；true表示从上到下，从左往右的顺序。
    /// 当UseNewStyleOutput=true时，此参数不生效。
    #[setters(generate = true, strip_option)]
    need_sort_page: Option<bool>,
    /// 是否需要分页功能，默认不需要。 true：需要 false：不需要。
    /// 当UseNewStyleOutput=true时，此参数不生效。
    #[setters(generate = true, strip_option)]
    page: Option<bool>,
    /// 是否需要去除印章功能，默认不需要。true：需要 false：不需要
    #[setters(generate = true, strip_option)]
    no_stamp: Option<bool>,
    /// 是否需要分段功能，默认不需要。true：需要 false：不需要。
    /// 当UseNewStyleOutput=true时，此参数不生效。
    #[setters(generate = true, strip_option)]
    paragraph: Option<bool>,
    /// 是否需要成行返回功能，默认不需要。true：需要 false：不需要。
    /// 当UseNewStyleOutput=true时，此参数不生效。
    #[setters(generate = true, strip_option)]
    row: Option<bool>,
    /// 是否返回新版格式输出，默认为false
    #[setters(generate = true, strip_option)]
    use_new_style_output: Option<bool>,
}

impl sealed::Bound for RecognizeDocumentStructure {}

impl RecognizeDocumentStructure {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            need_rotate: None,
            output_char_info: None,
            output_table: None,
            need_sort_page: None,
            page: None,
            no_stamp: None,
            paragraph: None,
            row: None,
            use_new_style_output: None,
        }
    }
}

impl crate::Request for RecognizeDocumentStructure {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeDocumentStructure";

    type Body = crate::OctetStream;

    type Response = RecognizeDocumentStructureResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_char_info {
            params.insert("OutputCharInfo".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        if let Some(f) = &self.need_sort_page {
            params.insert("NeedSortPage".into(), f.into());
        }

        if let Some(f) = &self.page {
            params.insert("Page".into(), f.into());
        }

        if let Some(f) = &self.no_stamp {
            params.insert("NoStamp".into(), f.into());
        }

        if let Some(f) = &self.paragraph {
            params.insert("Paragraph".into(), f.into());
        }

        if let Some(f) = &self.row {
            params.insert("Row".into(), f.into());
        }

        if let Some(f) = &self.use_new_style_output {
            params.insert("UseNewStyleOutput".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeIdcard {
    /// * 本字段和body字段二选一，不可同时传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否需要图案检测功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_figure: Option<bool>,
    /// * 是否需要身份证质量检测功能，默认不需要。
    /// * 身份证质量检测功能包含：是否翻拍，是否是复印件，完整度评分，整体质量分数、篡改指数。
    /// * 注意：如果需要设置此参数，请使用最新版本SDK。如果不需要设置此参数，您无需更新SDK。
    #[setters(generate = true, strip_option)]
    output_quality_info: Option<bool>,
    #[setters(generate = true, strip_option)]
    llm_rec: Option<bool>,
}

impl sealed::Bound for RecognizeIdcard {}

impl RecognizeIdcard {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_figure: None,
            output_quality_info: None,
            llm_rec: None,
        }
    }
}

impl crate::Request for RecognizeIdcard {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeIdcard";

    type Body = crate::OctetStream;

    type Response = RecognizeIdcardResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_figure {
            params.insert("OutputFigure".into(), f.into());
        }

        if let Some(f) = &self.output_quality_info {
            params.insert("OutputQualityInfo".into(), f.into());
        }

        if let Some(f) = &self.llm_rec {
            params.insert("Llm_rec".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// #### 请求注意事项
/// 1. 此接口 **10** 秒超时。
/// 2. 如果通过OCR SDK调用接口，SDK默认的 **socketTimeout** 为 **10** 秒。请注意适当增加 **RuntimeOptions** 中 **socketTimeout** 参数的值。
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizePassport {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizePassport {}

impl RecognizePassport {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizePassport {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizePassport";

    type Body = crate::OctetStream;

    type Response = RecognizePassportResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeHousehold {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否是户口本常住人口页，默认为否。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    is_resident_page: Option<bool>,
}

impl sealed::Bound for RecognizeHousehold {}

impl RecognizeHousehold {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            is_resident_page: None,
        }
    }
}

impl crate::Request for RecognizeHousehold {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeHousehold";

    type Body = crate::OctetStream;

    type Response = RecognizeHouseholdResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.is_resident_page {
            params.insert("IsResidentPage".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeEstateCertification {
    /// * 本字段和BODY字段二选一，不可同时透传或同时为空。
    ///   * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    ///   * 图片二进制文件，最大10MB。
    ///   * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    ///   * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeEstateCertification {}

impl RecognizeEstateCertification {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeEstateCertification {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeEstateCertification";

    type Body = crate::OctetStream;

    type Response = RecognizeEstateCertificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeBankCard {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeBankCard {}

impl RecognizeBankCard {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeBankCard {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeBankCard";

    type Body = crate::OctetStream;

    type Response = RecognizeBankCardResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeBirthCertification {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeBirthCertification {}

impl RecognizeBirthCertification {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeBirthCertification {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeBirthCertification";

    type Body = crate::OctetStream;

    type Response = RecognizeBirthCertificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeChinesePassport {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 是否需要图案检测功能，默认需要
    #[setters(generate = true, strip_option)]
    output_figure: Option<bool>,
}

impl sealed::Bound for RecognizeChinesePassport {}

impl RecognizeChinesePassport {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_figure: None,
        }
    }
}

impl crate::Request for RecognizeChinesePassport {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeChinesePassport";

    type Body = crate::OctetStream;

    type Response = RecognizeChinesePassportResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_figure {
            params.insert("OutputFigure".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeExitEntryPermitToMainland {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否需要图案检测功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_figure: Option<bool>,
}

impl sealed::Bound for RecognizeExitEntryPermitToMainland {}

impl RecognizeExitEntryPermitToMainland {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_figure: None,
        }
    }
}

impl crate::Request for RecognizeExitEntryPermitToMainland {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeExitEntryPermitToMainland";

    type Body = crate::OctetStream;

    type Response = RecognizeExitEntryPermitToMainlandResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_figure {
            params.insert("OutputFigure".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeExitEntryPermitToHK {
    /// 图片链接（长度不超 2048字节，不支持 base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 图案坐标信息输出，针对结构化，如身份证人脸头像
    #[setters(generate = true, strip_option)]
    output_figure: Option<bool>,
}

impl sealed::Bound for RecognizeExitEntryPermitToHK {}

impl RecognizeExitEntryPermitToHK {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_figure: None,
        }
    }
}

impl crate::Request for RecognizeExitEntryPermitToHK {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeExitEntryPermitToHK";

    type Body = crate::OctetStream;

    type Response = RecognizeExitEntryPermitToHKResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_figure {
            params.insert("OutputFigure".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeHKIdcard {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeHKIdcard {}

impl RecognizeHKIdcard {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeHKIdcard {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeHKIdcard";

    type Body = crate::OctetStream;

    type Response = RecognizeHKIdcardResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeSocialSecurityCardVersionII {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeSocialSecurityCardVersionII {}

impl RecognizeSocialSecurityCardVersionII {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeSocialSecurityCardVersionII {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeSocialSecurityCardVersionII";

    type Body = crate::OctetStream;

    type Response = RecognizeSocialSecurityCardVersionIIResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeInternationalIdcard {
    /// * 本字段和BODY字段二选一，不可同时透传或同时为空。
    ///   * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    ///   * 图片二进制文件，最大10MB。
    ///   * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    ///   * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 国家名称。
    ///   * 如：India，Vietnam，Korea，Bangladesh。
    country: IdcardCountry,
}

impl sealed::Bound for RecognizeInternationalIdcard {}

impl RecognizeInternationalIdcard {
    pub fn new(country: impl Into<IdcardCountry>) -> Self {
        Self {
            url: None,
            body: None,
            country: country.into(),
        }
    }
}

impl crate::Request for RecognizeInternationalIdcard {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeInternationalIdcard";

    type Body = crate::OctetStream;

    type Response = RecognizeInternationalIdcardResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("Country".into(), (&self.country).into());

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeMixedInvoices {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 待识别的PDF/OFD页码。
    /// * 如果字段为空，或大于PDF/OFD总页数，则识别第一页。
    /// * 使用SDK设置此字段，请更新SDK版本。
    #[setters(generate = true, strip_option)]
    page_no: Option<i32>,
    /// * 是否合并PDF的**前几页**，并返回合并页的识别结果（最大支持返回**前4页**识别结果）。例如上传的PDF有**3页**，且设置此参数为**true**，返回前**3页**所有识别结果。
    /// * 默认为**false**。
    /// * 如果上传的PDF总页数大于**4页**，且设置此参数为**true**，则只识别**前4页**（此参数不支持选择任意页码号）。
    /// * 如果设置了 **PageNo**，同时 **MergePdfPages** 设置为**true**，则 **MergePdfPages** 不生效，会识别 **PageNo** 指定的PDF页面。
    #[setters(generate = true, strip_option)]
    merge_pdf_pages: Option<bool>,
}

impl sealed::Bound for RecognizeMixedInvoices {}

impl RecognizeMixedInvoices {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            page_no: None,
            merge_pdf_pages: None,
        }
    }
}

impl crate::Request for RecognizeMixedInvoices {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeMixedInvoices";

    type Body = crate::OctetStream;

    type Response = RecognizeMixedInvoicesResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.page_no {
            params.insert("PageNo".into(), f.into());
        }

        if let Some(f) = &self.merge_pdf_pages {
            params.insert("MergePdfPages".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeInvoice {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 指定识别的PDF/OFD页码；例如：pageNo=6，识别PDF/OFD的第六页。
    /// * 如果该参数为空，或传值大于PDF/OFD总页数，则识别PDF/OFD的第一页。
    /// * 如果使用SDK设置此参数，请更新SDK版本，该参数在SDK版本1.1.16开始支持。
    #[setters(generate = true, strip_option)]
    page_no: Option<i32>,
}

impl sealed::Bound for RecognizeInvoice {}

impl RecognizeInvoice {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            page_no: None,
        }
    }
}

impl crate::Request for RecognizeInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeInvoice";

    type Body = crate::OctetStream;

    type Response = RecognizeInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.page_no {
            params.insert("PageNo".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeCarInvoice {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeCarInvoice {}

impl RecognizeCarInvoice {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeCarInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeCarInvoice";

    type Body = crate::OctetStream;

    type Response = RecognizeCarInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、PDF、OFD
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeQuotaInvoice {
    /// 图片链接（长度不超2048字节，不支持 base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeQuotaInvoice {}

impl RecognizeQuotaInvoice {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeQuotaInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeQuotaInvoice";

    type Body = crate::OctetStream;

    type Response = RecognizeQuotaInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeAirItinerary {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeAirItinerary {}

impl RecognizeAirItinerary {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeAirItinerary {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeAirItinerary";

    type Body = crate::OctetStream;

    type Response = RecognizeAirItineraryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeTrainInvoice {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeTrainInvoice {}

impl RecognizeTrainInvoice {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeTrainInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeTrainInvoice";

    type Body = crate::OctetStream;

    type Response = RecognizeTrainInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeTaxiInvoice {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeTaxiInvoice {}

impl RecognizeTaxiInvoice {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeTaxiInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeTaxiInvoice";

    type Body = crate::OctetStream;

    type Response = RecognizeTaxiInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、PDF、OFD
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeRollTicket {
    /// 图片链接（长度不超2048字节，不支持 base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeRollTicket {}

impl RecognizeRollTicket {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeRollTicket {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeRollTicket";

    type Body = crate::OctetStream;

    type Response = RecognizeRollTicketResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeBankAcceptance {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    ///   * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeBankAcceptance {}

impl RecognizeBankAcceptance {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeBankAcceptance {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeBankAcceptance";

    type Body = crate::OctetStream;

    type Response = RecognizeBankAcceptanceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeBusShipTicket {
    /// 图片链接（长度不超2048字节，不支持 base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeBusShipTicket {}

impl RecognizeBusShipTicket {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeBusShipTicket {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeBusShipTicket";

    type Body = crate::OctetStream;

    type Response = RecognizeBusShipTicketResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeNonTaxInvoice {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeNonTaxInvoice {}

impl RecognizeNonTaxInvoice {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeNonTaxInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeNonTaxInvoice";

    type Body = crate::OctetStream;

    type Response = RecognizeNonTaxInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、OFD、PDF
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeCommonPrintedInvoice {
    /// 图片链接（长度不超2048字节，不支持 base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeCommonPrintedInvoice {}

impl RecognizeCommonPrintedInvoice {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeCommonPrintedInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeCommonPrintedInvoice";

    type Body = crate::OctetStream;

    type Response = RecognizeCommonPrintedInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeHotelConsume {
    /// 图片链接（长度不超2048字节，不支持 base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeHotelConsume {}

impl RecognizeHotelConsume {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeHotelConsume {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeHotelConsume";

    type Body = crate::OctetStream;

    type Response = RecognizeHotelConsumeResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizePaymentRecord {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizePaymentRecord {}

impl RecognizePaymentRecord {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizePaymentRecord {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizePaymentRecord";

    type Body = crate::OctetStream;

    type Response = RecognizePaymentRecordResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizePurchaseRecord {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否需要识别多条订单，默认不需要。
    /// * true：需要；false：不需要。
    /// * 如果需要使用此参数，请更新SDK到1.1.14或更高版本。
    /// * 注意：如果此参数设置为true，返回结果字段会变化。
    #[setters(generate = true, strip_option)]
    output_multi_orders: Option<bool>,
}

impl sealed::Bound for RecognizePurchaseRecord {}

impl RecognizePurchaseRecord {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_multi_orders: None,
        }
    }
}

impl crate::Request for RecognizePurchaseRecord {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizePurchaseRecord";

    type Body = crate::OctetStream;

    type Response = RecognizePurchaseRecordResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_multi_orders {
            params.insert("OutputMultiOrders".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeRideHailingItinerary {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeRideHailingItinerary {}

impl RecognizeRideHailingItinerary {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeRideHailingItinerary {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeRideHailingItinerary";

    type Body = crate::OctetStream;

    type Response = RecognizeRideHailingItineraryResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeShoppingReceipt {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeShoppingReceipt {}

impl RecognizeShoppingReceipt {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeShoppingReceipt {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeShoppingReceipt";

    type Body = crate::OctetStream;

    type Response = RecognizeShoppingReceiptResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeSocialSecurityCard {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeSocialSecurityCard {}

impl RecognizeSocialSecurityCard {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeSocialSecurityCard {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeSocialSecurityCard";

    type Body = crate::OctetStream;

    type Response = RecognizeSocialSecurityCardResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeTollInvoice {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeTollInvoice {}

impl RecognizeTollInvoice {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeTollInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeTollInvoice";

    type Body = crate::OctetStream;

    type Response = RecognizeTollInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeTaxClearanceCertificate {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeTaxClearanceCertificate {}

impl RecognizeTaxClearanceCertificate {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeTaxClearanceCertificate {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeTaxClearanceCertificate";

    type Body = crate::OctetStream;

    type Response = RecognizeTaxClearanceCertificateResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、PDF、OFD。
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeUsedCarInvoice {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeUsedCarInvoice {}

impl RecognizeUsedCarInvoice {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeUsedCarInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeUsedCarInvoice";

    type Body = crate::OctetStream;

    type Response = RecognizeUsedCarInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeBusinessLicense {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeBusinessLicense {}

impl RecognizeBusinessLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeBusinessLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeBusinessLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeBusinessLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeBankAccountLicense {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeBankAccountLicense {}

impl RecognizeBankAccountLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeBankAccountLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeBankAccountLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeBankAccountLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeTradeMarkCertification {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeTradeMarkCertification {}

impl RecognizeTradeMarkCertification {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeTradeMarkCertification {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeTradeMarkCertification";

    type Body = crate::OctetStream;

    type Response = RecognizeTradeMarkCertificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeFoodProduceLicense {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeFoodProduceLicense {}

impl RecognizeFoodProduceLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeFoodProduceLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeFoodProduceLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeFoodProduceLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeFoodManageLicense {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeFoodManageLicense {}

impl RecognizeFoodManageLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeFoodManageLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeFoodManageLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeFoodManageLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeMedicalDeviceManageLicense {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeMedicalDeviceManageLicense {}

impl RecognizeMedicalDeviceManageLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeMedicalDeviceManageLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeMedicalDeviceManageLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeMedicalDeviceManageLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeMedicalDeviceProduceLicense {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeMedicalDeviceProduceLicense {}

impl RecognizeMedicalDeviceProduceLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeMedicalDeviceProduceLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeMedicalDeviceProduceLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeMedicalDeviceProduceLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeCtwoMedicalDeviceManageLicense {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeCtwoMedicalDeviceManageLicense {}

impl RecognizeCtwoMedicalDeviceManageLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeCtwoMedicalDeviceManageLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeCtwoMedicalDeviceManageLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeCtwoMedicalDeviceManageLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeCosmeticProduceLicense {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制字节流，最大10MB
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeCosmeticProduceLicense {}

impl RecognizeCosmeticProduceLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeCosmeticProduceLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeCosmeticProduceLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeCosmeticProduceLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP、OFD、PDF
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeInternationalBusinessLicense {
    /// 图片/PDF 链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片/PDF二进制字节流，最大10M
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 国家名称
    country: LicenseCountry,
}

impl sealed::Bound for RecognizeInternationalBusinessLicense {}

impl RecognizeInternationalBusinessLicense {
    pub fn new(country: impl Into<LicenseCountry>) -> Self {
        Self {
            url: None,
            body: None,
            country: country.into(),
        }
    }
}

impl crate::Request for RecognizeInternationalBusinessLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeInternationalBusinessLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeInternationalBusinessLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("Country".into(), (&self.country).into());

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeVehicleLicense {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeVehicleLicense {}

impl RecognizeVehicleLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeVehicleLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeVehicleLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeVehicleLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeDrivingLicense {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeDrivingLicense {}

impl RecognizeDrivingLicense {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeDrivingLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeDrivingLicense";

    type Body = crate::OctetStream;

    type Response = RecognizeDrivingLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeWaybill {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeWaybill {}

impl RecognizeWaybill {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeWaybill {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeWaybill";

    type Body = crate::OctetStream;

    type Response = RecognizeWaybillResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeCarNumber {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeCarNumber {}

impl RecognizeCarNumber {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeCarNumber {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeCarNumber";

    type Body = crate::OctetStream;

    type Response = RecognizeCarNumberResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeCarVinCode {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeCarVinCode {}

impl RecognizeCarVinCode {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeCarVinCode {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeCarVinCode";

    type Body = crate::OctetStream;

    type Response = RecognizeCarVinCodeResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeVehicleRegistration {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeVehicleRegistration {}

impl RecognizeVehicleRegistration {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeVehicleRegistration {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeVehicleRegistration";

    type Body = crate::OctetStream;

    type Response = RecognizeVehicleRegistrationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeVehicleCertification {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeVehicleCertification {}

impl RecognizeVehicleCertification {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeVehicleCertification {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeVehicleCertification";

    type Body = crate::OctetStream;

    type Response = RecognizeVehicleCertificationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeEduFormula {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeEduFormula {}

impl RecognizeEduFormula {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeEduFormula {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeEduFormula";

    type Body = crate::OctetStream;

    type Response = RecognizeEduFormulaResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeEduOralCalculation {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
}

impl sealed::Bound for RecognizeEduOralCalculation {}

impl RecognizeEduOralCalculation {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
        }
    }
}

impl crate::Request for RecognizeEduOralCalculation {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeEduOralCalculation";

    type Body = crate::OctetStream;

    type Response = RecognizeEduOralCalculationResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeEduPaperOcr {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 图片类型。
    /// * scan：扫描图， photo：实拍图。
    image_type: String,
    /// * 年级学科。
    /// * default:默认, Math:数学, PrimarySchool_Math:小学数学, JHighSchool_Math: 初中数学, Chinese:语文, PrimarySchool_Chinese:小学语文, JHighSchool_Chinese:初中语文, English:英语, PrimarySchool_English:小学英语, JHighSchool_English:初中英语, Physics:物理, JHighSchool_Physics:初中物理, Chemistry: 化学, JHighSchool_Chemistry:初中化学, Biology:生物, JHighSchool_Biology:初中生物, History:历史, JHighSchool_History:初中历史, Geography:地理, JHighSchool_Geography:初中地理, Politics:政治, JHighSchool_Politics:初中政治。
    #[setters(generate = true, strip_option)]
    subject: Option<String>,
    /// * 是否输出原图坐标信息（如果图片被做过旋转，图片校正等处理），默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_oricoord: Option<bool>,
}

impl sealed::Bound for RecognizeEduPaperOcr {}

impl RecognizeEduPaperOcr {
    pub fn new(image_type: impl Into<String>) -> Self {
        Self {
            url: None,
            body: None,
            image_type: image_type.into(),
            subject: None,
            output_oricoord: None,
        }
    }
}

impl crate::Request for RecognizeEduPaperOcr {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeEduPaperOcr";

    type Body = crate::OctetStream;

    type Response = RecognizeEduPaperOcrResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("ImageType".into(), (&self.image_type).into());

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.subject {
            params.insert("Subject".into(), f.into());
        }

        if let Some(f) = &self.output_oricoord {
            params.insert("OutputOricoord".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeEduPaperCut {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 切题类型。
    /// * question：题目， answer：答案。
    cut_type: String,
    /// * 图片类型。
    /// * scan：扫描图， photo：实拍图。
    image_type: String,
    /// * 年级学科。
    /// * default:默认, Math:数学, PrimarySchool_Math:小学数学, JHighSchool_Math: 初中数学, Chinese:语文, PrimarySchool_Chinese:小学语文, JHighSchool_Chinese:初中语文, English:英语, PrimarySchool_English:小学英语, JHighSchool_English:初中英语, Physics:物理, JHighSchool_Physics:初中物理, Chemistry: 化学, JHighSchool_Chemistry:初中化学, Biology:生物, JHighSchool_Biology:初中生物, History:历史, JHighSchool_History:初中历史, Geography:地理, JHighSchool_Geography:初中地理, Politics:政治, JHighSchool_Politics:初中政治。
    #[setters(generate = true, strip_option)]
    subject: Option<String>,
    /// * 是否输出原图坐标信息（如果图片被做过旋转，图片校正等处理），默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_oricoord: Option<bool>,
}

impl sealed::Bound for RecognizeEduPaperCut {}

impl RecognizeEduPaperCut {
    pub fn new(cut_type: impl Into<String>, image_type: impl Into<String>) -> Self {
        Self {
            url: None,
            body: None,
            cut_type: cut_type.into(),
            image_type: image_type.into(),
            subject: None,
            output_oricoord: None,
        }
    }
}

impl crate::Request for RecognizeEduPaperCut {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeEduPaperCut";

    type Body = crate::OctetStream;

    type Response = RecognizeEduPaperCutResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("CutType".into(), (&self.cut_type).into());
        params.insert("ImageType".into(), (&self.image_type).into());

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.subject {
            params.insert("Subject".into(), f.into());
        }

        if let Some(f) = &self.output_oricoord {
            params.insert("OutputOricoord".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeEduQuestionOcr {
    /// * 本字段和BODY字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否需要自动旋转功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
}

impl sealed::Bound for RecognizeEduQuestionOcr {}

impl RecognizeEduQuestionOcr {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            need_rotate: None,
        }
    }
}

impl crate::Request for RecognizeEduQuestionOcr {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeEduQuestionOcr";

    type Body = crate::OctetStream;

    type Response = RecognizeEduQuestionOcrResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeEduPaperStructed {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 年级学科。
    /// * default:默认, Math:数学, PrimarySchool_Math:小学数学, JHighSchool_Math: 初中数学, Chinese:语文, PrimarySchool_Chinese:小学语文, JHighSchool_Chinese:初中语文, English:英语, PrimarySchool_English:小学英语, JHighSchool_English:初中英语, Physics:物理, JHighSchool_Physics:初中物理, Chemistry: 化学, JHighSchool_Chemistry:初中化学, Biology:生物, JHighSchool_Biology:初中生物, History:历史, JHighSchool_History:初中历史, Geography:地理, JHighSchool_Geography:初中地理, Politics:政治, JHighSchool_Politics:初中政治。
    #[setters(generate = true, strip_option)]
    subject: Option<String>,
    /// * 是否需要自动旋转功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// * 是否输出原图坐标信息（如果图片被做过旋转，图片校正等处理），默认不需要。
    /// * 如需输出原图坐标，建议同时将NeedRotate参数设置为true。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_oricoord: Option<bool>,
}

impl sealed::Bound for RecognizeEduPaperStructed {}

impl RecognizeEduPaperStructed {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            subject: None,
            need_rotate: None,
            output_oricoord: None,
        }
    }
}

impl crate::Request for RecognizeEduPaperStructed {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeEduPaperStructed";

    type Body = crate::OctetStream;

    type Response = RecognizeEduPaperStructedResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.subject {
            params.insert("Subject".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_oricoord {
            params.insert("OutputOricoord".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeMultiLanguage {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 支持语言列表。
    languages: Vec<String>,
    /// * 是否输出单字识别结果，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_char_info: Option<bool>,
    /// * 是否需要自动旋转功能，默认需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// * 是否输出表格识别结果，包含单元格信息，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
    /// * 是否按顺序输出文字块，默认为false。
    /// * false表示从左往右，从上到下的顺序；true表示从上到下，从左往右的顺序。
    #[setters(generate = true, strip_option)]
    need_sort_page: Option<bool>,
}

impl sealed::Bound for RecognizeMultiLanguage {}

impl RecognizeMultiLanguage {
    pub fn new(languages: impl Into<Vec<String>>) -> Self {
        Self {
            url: None,
            body: None,
            languages: languages.into(),
            output_char_info: None,
            need_rotate: None,
            output_table: None,
            need_sort_page: None,
        }
    }
}

impl crate::Request for RecognizeMultiLanguage {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeMultiLanguage";

    type Body = crate::OctetStream;

    type Response = RecognizeMultiLanguageResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        crate::SimpleSerialize::simple_serialize(&self.languages, "Languages", &mut params);

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_char_info {
            params.insert("OutputCharInfo".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        if let Some(f) = &self.need_sort_page {
            params.insert("NeedSortPage".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeEnglish {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 是否需要自动旋转功能，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// * 是否输出表格识别结果，包含单元格信息，默认不需要。
    /// * true：需要；false：不需要。
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
}

impl sealed::Bound for RecognizeEnglish {}

impl RecognizeEnglish {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            need_rotate: None,
            output_table: None,
        }
    }
}

impl crate::Request for RecognizeEnglish {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeEnglish";

    type Body = crate::OctetStream;

    type Response = RecognizeEnglishResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeThai {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 是否输出单字识别结果
    #[setters(generate = true, strip_option)]
    output_char_info: Option<bool>,
    /// 是否需要自动旋转功能（结构化检测、混贴场景、教育相关场景会自动做旋转，无需设置），返回角度信息
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// 是否输出表格识别结果，包含单元格信息
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
}

impl sealed::Bound for RecognizeThai {}

impl RecognizeThai {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_char_info: None,
            need_rotate: None,
            output_table: None,
        }
    }
}

impl crate::Request for RecognizeThai {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeThai";

    type Body = crate::OctetStream;

    type Response = RecognizeThaiResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_char_info {
            params.insert("OutputCharInfo".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeJanpanese {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 是否输出单字识别结果
    #[setters(generate = true, strip_option)]
    output_char_info: Option<bool>,
    /// 是否需要自动旋转功能（结构化检测、混贴场景、教育相关场景会自动做旋转，无需设置），返回角度信息
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// 是否输出表格识别结果，包含单元格信息
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
}

impl sealed::Bound for RecognizeJanpanese {}

impl RecognizeJanpanese {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_char_info: None,
            need_rotate: None,
            output_table: None,
        }
    }
}

impl crate::Request for RecognizeJanpanese {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeJanpanese";

    type Body = crate::OctetStream;

    type Response = RecognizeJanpaneseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_char_info {
            params.insert("OutputCharInfo".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeKorean {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 是否输出单字识别结果
    #[setters(generate = true, strip_option)]
    output_char_info: Option<bool>,
    /// 是否需要自动旋转功能（结构化检测、混贴场景、教育相关场景会自动做旋转，无需设置），返回角度信息
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// 是否输出表格识别结果，包含单元格信息
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
}

impl sealed::Bound for RecognizeKorean {}

impl RecognizeKorean {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_char_info: None,
            need_rotate: None,
            output_table: None,
        }
    }
}

impl crate::Request for RecognizeKorean {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeKorean";

    type Body = crate::OctetStream;

    type Response = RecognizeKoreanResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_char_info {
            params.insert("OutputCharInfo".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeLatin {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 是否输出单字识别结果
    #[setters(generate = true, strip_option)]
    output_char_info: Option<bool>,
    /// 是否需要自动旋转功能（结构化检测、混贴场景、教育相关场景会自动做旋转，无需设置），返回角度信息
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// 是否输出表格识别结果，包含单元格信息
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
}

impl sealed::Bound for RecognizeLatin {}

impl RecognizeLatin {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_char_info: None,
            need_rotate: None,
            output_table: None,
        }
    }
}

impl crate::Request for RecognizeLatin {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeLatin";

    type Body = crate::OctetStream;

    type Response = RecognizeLatinResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_char_info {
            params.insert("OutputCharInfo".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}

/// ### 支持的图片格式
/// * PNG、JPG、JPEG、BMP、GIF、TIFF、WebP
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeRussian {
    /// 图片链接（长度不超2048字节，不支持base64）
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// 图片二进制文件，最大10MB，与URL二选一。 使用HTTP方式调用，把图片二进制文件放到HTTP body 中上传即可。 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// 是否输出单字识别结果
    #[setters(generate = true, strip_option)]
    output_char_info: Option<bool>,
    /// 是否需要自动旋转功能（结构化检测、混贴场景、教育相关场景会自动做旋转，无需设置），返回角度信息
    #[setters(generate = true, strip_option)]
    need_rotate: Option<bool>,
    /// 是否输出表格识别结果，包含单元格信息
    #[setters(generate = true, strip_option)]
    output_table: Option<bool>,
}

impl sealed::Bound for RecognizeRussian {}

impl RecognizeRussian {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            output_char_info: None,
            need_rotate: None,
            output_table: None,
        }
    }
}

impl crate::Request for RecognizeRussian {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeRussian";

    type Body = crate::OctetStream;

    type Response = RecognizeRussianResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.output_char_info {
            params.insert("OutputCharInfo".into(), f.into());
        }

        if let Some(f) = &self.need_rotate {
            params.insert("NeedRotate".into(), f.into());
        }

        if let Some(f) = &self.output_table {
            params.insert("OutputTable".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct RecognizeCovidTestReport {
    /// * 本字段和body字段二选一，不可同时透传或同时为空。
    /// * 图片链接（长度不超2048字节，不支持base64）。
    #[setters(generate = true, strip_option)]
    url: Option<String>,
    /// * 本字段和URL字段二选一，不可同时透传或同时为空。
    /// * 图片二进制文件，最大10MB。
    /// * 使用HTTP方式调用，把图片二进制文件放到HTTP body中上传即可。
    /// * 使用SDK的方式调用，把图片放到SDK的body中即可。
    #[setters(generate = true, strip_option)]
    body: Option<Vec<u8>>,
    /// * 当一张图有多个子图时，是否要返回多个识别结果,默认不需要。
    /// * true：返回所有子图识别结果；false：返回检测日期最新的一个结果。
    #[setters(generate = true, strip_option)]
    multiple_result: Option<bool>,
}

impl sealed::Bound for RecognizeCovidTestReport {}

impl RecognizeCovidTestReport {
    pub fn new() -> Self {
        Self {
            url: None,
            body: None,
            multiple_result: None,
        }
    }
}

impl crate::Request for RecognizeCovidTestReport {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "RecognizeCovidTestReport";

    type Body = crate::OctetStream;

    type Response = RecognizeCovidTestReportResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();

        if let Some(f) = &self.url {
            params.insert("Url".into(), f.into());
        }

        if let Some(f) = &self.multiple_result {
            params.insert("MultipleResult".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::OctetStream(self.body.unwrap_or_default()))
    }
}
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct VerifyBusinessLicense {
    /// 企业注册号或统一社会信用代码
    credit_code: String,
    /// 企业名称
    company_name: String,
    /// 企业法人姓名
    legal_person: String,
}

impl sealed::Bound for VerifyBusinessLicense {}

impl VerifyBusinessLicense {
    pub fn new(
        credit_code: impl Into<String>,
        company_name: impl Into<String>,
        legal_person: impl Into<String>,
    ) -> Self {
        Self {
            credit_code: credit_code.into(),
            company_name: company_name.into(),
            legal_person: legal_person.into(),
        }
    }
}
impl crate::ToFormData for VerifyBusinessLicense {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for VerifyBusinessLicense {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "VerifyBusinessLicense";

    type Body = crate::Form<Self>;

    type Response = VerifyBusinessLicenseResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("CreditCode".into(), (&self.credit_code).into());
        params.insert("CompanyName".into(), (&self.company_name).into());
        params.insert("LegalPerson".into(), (&self.legal_person).into());

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}

/// * <span style="font-size:larger;"> <b> 发票类型代码说明 </b></span>
///
/// |发票类型代码|发票类型说明|
/// |------|-------|
/// |01| 增值税专用发票 |
/// |02| 货运运输业增值税专用发票 |
/// |03| 机动车销售统一发票 |
/// |04| 增值税普通发票 |
/// |10| 增值税普通发票（电子）|
/// |11| 增值税普通发票（卷式） |
/// |14| 增值税普通发票（通行费） |
/// |15| 二手车销售统一发票 |
/// |20| 增值税电子专用发票 |
/// |31| 数电发票（增值税专用发票）|
/// |32| 数电发票（增值税普通发票）|
/// |51| 电子发票（铁路电子客票）|
/// |61| 电子发票（航空运输电子客票行程单）|
/// |83| 电子发票（机动车销售统一发票）|
/// |84| 电子发票（二手车销售统一发票）|
/// |85| 数电发票（纸质专用发票）|
/// |86| 数电发票（纸质普通发票）|
/// |87| 数电纸质发票（机动车销售统一发票）|
/// |88| 数电纸质发票（二手车销售统一发票）|
#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct VerifyVATInvoice {
    /// 发票代码。数电发票（发票类型代码为31，32，51，61，83，84）时可为空（发票类型代码见**发票类型代码说明**）。
    #[setters(generate = true, strip_option)]
    invoice_code: Option<String>,
    /// 发票号码。
    invoice_no: String,
    /// 开票日期（日期格式为：YYYYMMDD）。
    invoice_date: String,
    /// 发票金额。发票类型代码为 01，03，15，20，31，32 ，51，61，85，83，84时必填：为 01，03，20 ,85时填写发票**不含税金额**；为 15 ,84时填写发票**车价合计**；为 31，32 ，51，61，83时填写**含税金额**；为区块链发票（InvoiceKind=1）时填写 **不含税金额**。
    /// 其它类型可为空（详见**发票类型代码说明**）。
    #[setters(generate = true, strip_option)]
    invoice_sum: Option<String>,
    /// 校验码，取**后6位**。发票类型代码为 04，10，11，14，86 时必填，发票类型代码为 86 时，填写密码区数电票号码后六位，为区块链发票（InvoiceKind=1）时必填，其他发票种类可为空（详见**发票类型代码说明**）。
    #[setters(generate = true, strip_option)]
    verify_code: Option<String>,
    /// 发票类型。用来区分是否为 **区块链发票**。
    /// * InvoiceKind=0 或不填，表示 **非区块链发票**。
    /// * InvoiceKind=1，表示 **区块链发票**。注意，如果核验区块链发票，则 **InvoiceCode**，**InvoiceNumber**，**InvoiceDate**，**InvoiceSum**，**VerifyCode** 这5个入参均为必传参数。
    #[setters(generate = true, strip_option)]
    invoice_kind: Option<i32>,
}

impl sealed::Bound for VerifyVATInvoice {}

impl VerifyVATInvoice {
    pub fn new(invoice_no: impl Into<String>, invoice_date: impl Into<String>) -> Self {
        Self {
            invoice_code: None,
            invoice_no: invoice_no.into(),
            invoice_date: invoice_date.into(),
            invoice_sum: None,
            verify_code: None,
            invoice_kind: None,
        }
    }
}
impl crate::ToFormData for VerifyVATInvoice {
    fn to_form_data(
        &self,
    ) -> std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>> {
        let mut params = std::collections::BTreeMap::new();

        params
    }
}

impl crate::Request for VerifyVATInvoice {
    const METHOD: http::Method = http::Method::POST;

    const ACTION: &'static str = "VerifyVATInvoice";

    type Body = crate::Form<Self>;

    type Response = VerifyVATInvoiceResponse;

    fn to_query_params(
        &self,
    ) -> crate::Result<
        std::collections::BTreeMap<std::borrow::Cow<'static, str>, crate::QueryValue<'_>>,
    > {
        let mut params = std::collections::BTreeMap::new();
        params.insert("InvoiceNo".into(), (&self.invoice_no).into());
        params.insert("InvoiceDate".into(), (&self.invoice_date).into());

        if let Some(f) = &self.invoice_code {
            params.insert("InvoiceCode".into(), f.into());
        }

        if let Some(f) = &self.invoice_sum {
            params.insert("InvoiceSum".into(), f.into());
        }

        if let Some(f) = &self.verify_code {
            params.insert("VerifyCode".into(), f.into());
        }

        if let Some(f) = &self.invoice_kind {
            params.insert("InvoiceKind".into(), f.into());
        }

        Ok(params)
    }

    fn to_body(self) -> crate::Result<Self::Body> {
        Ok(crate::Form(self))
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdvancedConfig {
    #[serde(rename = "IsHandWritingTable")]
    pub is_hand_writing_table: bool,
    #[serde(rename = "IsLineLessTable")]
    pub is_line_less_table: bool,
    #[serde(rename = "OutputCharInfo")]
    pub output_char_info: bool,
    #[serde(rename = "OutputParagraph")]
    pub output_paragraph: bool,
    #[serde(rename = "OutputRow")]
    pub output_row: bool,
    #[serde(rename = "OutputTable")]
    pub output_table: bool,
    #[serde(rename = "OutputTableExcel")]
    pub output_table_excel: bool,
    #[serde(rename = "OutputTableHtml")]
    pub output_table_html: bool,
}

impl crate::FlatSerialize for AdvancedConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.is_hand_writing_table,
            &format!("{}.IsHandWritingTable", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_line_less_table,
            &format!("{}.IsLineLessTable", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_char_info,
            &format!("{}.OutputCharInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_paragraph,
            &format!("{}.OutputParagraph", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_row,
            &format!("{}.OutputRow", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_table,
            &format!("{}.OutputTable", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_table_excel,
            &format!("{}.OutputTableExcel", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_table_html,
            &format!("{}.OutputTableHtml", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BarCodeDetail {
    #[serde(rename = "BarCodeAngle")]
    pub bar_code_angle: i32,
    #[serde(rename = "BarCodePoints")]
    pub bar_code_points: Vec<BarCodePoint>,
    #[serde(rename = "BarCodeRect")]
    pub bar_code_rect: BarCodeRect,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "Type")]
    pub r#type: String,
}

impl crate::FlatSerialize for BarCodeDetail {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bar_code_angle,
            &format!("{}.BarCodeAngle", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bar_code_points,
            &format!("{}.BarCodePoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bar_code_rect,
            &format!("{}.BarCodeRect", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.data, &format!("{}.Data", name), params);
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BarCodeInfo {
    #[serde(rename = "BarCodeCount")]
    pub bar_code_count: i32,
    #[serde(rename = "BarCodeDetails")]
    pub bar_code_details: Vec<BarCodeDetail>,
}

impl crate::FlatSerialize for BarCodeInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.bar_code_count,
            &format!("{}.BarCodeCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.bar_code_details,
            &format!("{}.BarCodeDetails", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BarCodePoint {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
}

impl crate::FlatSerialize for BarCodePoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.x, &format!("{}.X", name), params);
        crate::FlatSerialize::flat_serialize(&self.y, &format!("{}.Y", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BarCodeRect {
    #[serde(rename = "CenterX")]
    pub center_x: i32,
    #[serde(rename = "CenterY")]
    pub center_y: i32,
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "Width")]
    pub width: i32,
}

impl crate::FlatSerialize for BarCodeRect {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.center_x, &format!("{}.CenterX", name), params);
        crate::FlatSerialize::flat_serialize(&self.center_y, &format!("{}.CenterY", name), params);
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BlockDetail {
    #[serde(rename = "BlockAngle")]
    pub block_angle: i32,
    #[serde(rename = "BlockConfidence")]
    pub block_confidence: i32,
    #[serde(rename = "BlockContent")]
    pub block_content: String,
    #[serde(rename = "BlockId")]
    pub block_id: i32,
    #[serde(rename = "BlockPoints")]
    pub block_points: Vec<BlockPoint>,
    #[serde(rename = "BlockRect")]
    pub block_rect: BlockRect,
    #[serde(rename = "CharInfos")]
    pub char_infos: Vec<CharInfo>,
}

impl crate::FlatSerialize for BlockDetail {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.block_angle,
            &format!("{}.BlockAngle", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.block_confidence,
            &format!("{}.BlockConfidence", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.block_content,
            &format!("{}.BlockContent", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.block_id, &format!("{}.BlockId", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.block_points,
            &format!("{}.BlockPoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.block_rect,
            &format!("{}.BlockRect", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.char_infos,
            &format!("{}.CharInfos", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BlockInfo {
    #[serde(rename = "BlockCount")]
    pub block_count: i32,
    #[serde(rename = "BlockDetails")]
    pub block_details: Vec<BlockDetail>,
}

impl crate::FlatSerialize for BlockInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.block_count,
            &format!("{}.BlockCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.block_details,
            &format!("{}.BlockDetails", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BlockPoint {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
}

impl crate::FlatSerialize for BlockPoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.x, &format!("{}.X", name), params);
        crate::FlatSerialize::flat_serialize(&self.y, &format!("{}.Y", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BlockRect {
    #[serde(rename = "CenterX")]
    pub center_x: i32,
    #[serde(rename = "CenterY")]
    pub center_y: i32,
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "Width")]
    pub width: i32,
}

impl crate::FlatSerialize for BlockRect {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.center_x, &format!("{}.CenterX", name), params);
        crate::FlatSerialize::flat_serialize(&self.center_y, &format!("{}.CenterY", name), params);
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CellDetail {
    #[serde(rename = "BlockList")]
    pub block_list: Vec<i32>,
    #[serde(rename = "CellAngle")]
    pub cell_angle: i32,
    #[serde(rename = "CellContent")]
    pub cell_content: String,
    #[serde(rename = "CellId")]
    pub cell_id: i32,
    #[serde(rename = "CellPoints")]
    pub cell_points: Vec<CellPoint>,
    #[serde(rename = "CellRect")]
    pub cell_rect: CellRect,
    #[serde(rename = "ColumnEnd")]
    pub column_end: i32,
    #[serde(rename = "ColumnStart")]
    pub column_start: i32,
    #[serde(rename = "RowEnd")]
    pub row_end: i32,
    #[serde(rename = "RowStart")]
    pub row_start: i32,
}

impl crate::FlatSerialize for CellDetail {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.block_list,
            &format!("{}.BlockList", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cell_angle,
            &format!("{}.CellAngle", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cell_content,
            &format!("{}.CellContent", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.cell_id, &format!("{}.CellId", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.cell_points,
            &format!("{}.CellPoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cell_rect,
            &format!("{}.CellRect", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.column_end,
            &format!("{}.ColumnEnd", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.column_start,
            &format!("{}.ColumnStart", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.row_end, &format!("{}.RowEnd", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.row_start,
            &format!("{}.RowStart", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CellPoint {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
}

impl crate::FlatSerialize for CellPoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.x, &format!("{}.X", name), params);
        crate::FlatSerialize::flat_serialize(&self.y, &format!("{}.Y", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CellRect {
    #[serde(rename = "CenterX")]
    pub center_x: i32,
    #[serde(rename = "CenterY")]
    pub center_y: i32,
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "Width")]
    pub width: i32,
}

impl crate::FlatSerialize for CellRect {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.center_x, &format!("{}.CenterX", name), params);
        crate::FlatSerialize::flat_serialize(&self.center_y, &format!("{}.CenterY", name), params);
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CharInfo {
    #[serde(rename = "CharConfidence")]
    pub char_confidence: i32,
    #[serde(rename = "CharContent")]
    pub char_content: String,
    #[serde(rename = "CharId")]
    pub char_id: i32,
    #[serde(rename = "CharPoints")]
    pub char_points: Vec<CharPoint>,
    #[serde(rename = "CharRect")]
    pub char_rect: CharRect,
}

impl crate::FlatSerialize for CharInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.char_confidence,
            &format!("{}.CharConfidence", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.char_content,
            &format!("{}.CharContent", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.char_id, &format!("{}.CharId", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.char_points,
            &format!("{}.CharPoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.char_rect,
            &format!("{}.CharRect", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CharPoint {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
}

impl crate::FlatSerialize for CharPoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.x, &format!("{}.X", name), params);
        crate::FlatSerialize::flat_serialize(&self.y, &format!("{}.Y", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CharRect {
    #[serde(rename = "CenterX")]
    pub center_x: i32,
    #[serde(rename = "CenterY")]
    pub center_y: i32,
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "Width")]
    pub width: i32,
}

impl crate::FlatSerialize for CharRect {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.center_x, &format!("{}.CenterX", name), params);
        crate::FlatSerialize::flat_serialize(&self.center_y, &format!("{}.CenterY", name), params);
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FigureInfo {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, crate::Value>,
}

impl crate::FlatSerialize for FigureInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.extra, name, params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Footer {
    #[serde(rename = "BlockId")]
    pub block_id: i32,
    #[serde(rename = "Contents")]
    pub contents: Vec<String>,
}

impl crate::FlatSerialize for Footer {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.block_id, &format!("{}.BlockId", name), params);
        crate::FlatSerialize::flat_serialize(&self.contents, &format!("{}.Contents", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Header {
    #[serde(rename = "BlockId")]
    pub block_id: i32,
    #[serde(rename = "Contents")]
    pub contents: Vec<String>,
}

impl crate::FlatSerialize for Header {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.block_id, &format!("{}.BlockId", name), params);
        crate::FlatSerialize::flat_serialize(&self.contents, &format!("{}.Contents", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ImagePoint {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
}

impl crate::FlatSerialize for ImagePoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.x, &format!("{}.X", name), params);
        crate::FlatSerialize::flat_serialize(&self.y, &format!("{}.Y", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ImageRect {
    #[serde(rename = "CenterX")]
    pub center_x: i32,
    #[serde(rename = "CenterY")]
    pub center_y: i32,
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "Width")]
    pub width: i32,
}

impl crate::FlatSerialize for ImageRect {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.center_x, &format!("{}.CenterX", name), params);
        crate::FlatSerialize::flat_serialize(&self.center_y, &format!("{}.CenterY", name), params);
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InternationalIdCardConfig {
    #[serde(rename = "Country")]
    pub country: ConfigCountry,
}

impl crate::FlatSerialize for InternationalIdCardConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.country, &format!("{}.Country", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ItemData {
    #[serde(rename = "AntiFakeCode")]
    pub anti_fake_code: String,
    #[serde(rename = "CompanyId")]
    pub company_id: String,
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
    #[serde(rename = "OrganizationNameEng")]
    pub organization_name_eng: String,
    #[serde(rename = "OtherText")]
    pub other_text: String,
    #[serde(rename = "TaxpayerId")]
    pub taxpayer_id: String,
    #[serde(rename = "TopText")]
    pub top_text: String,
}

impl crate::FlatSerialize for ItemData {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.anti_fake_code,
            &format!("{}.AntiFakeCode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.company_id,
            &format!("{}.CompanyId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.organization_name,
            &format!("{}.OrganizationName", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.organization_name_eng,
            &format!("{}.OrganizationNameEng", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.other_text,
            &format!("{}.OtherText", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.taxpayer_id,
            &format!("{}.TaxpayerId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.top_text, &format!("{}.TopText", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct KvDetail {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, crate::Value>,
}

impl crate::FlatSerialize for KvDetail {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.extra, name, params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LanConfig {
    #[serde(rename = "Languages")]
    pub languages: String,
}

impl crate::FlatSerialize for LanConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.languages,
            &format!("{}.Languages", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LicenseConfig {
    #[serde(rename = "Country")]
    pub country: String,
}

impl crate::FlatSerialize for LicenseConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.country, &format!("{}.Country", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ParagraphDetail {
    #[serde(rename = "BlockList")]
    pub block_list: Vec<i32>,
    #[serde(rename = "ParagraphContent")]
    pub paragraph_content: String,
    #[serde(rename = "ParagraphId")]
    pub paragraph_id: i32,
}

impl crate::FlatSerialize for ParagraphDetail {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.block_list,
            &format!("{}.BlockList", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.paragraph_content,
            &format!("{}.ParagraphContent", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.paragraph_id,
            &format!("{}.ParagraphId", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ParagraphInfo {
    #[serde(rename = "ParagraphCount")]
    pub paragraph_count: i32,
    #[serde(rename = "ParagraphDetails")]
    pub paragraph_details: Vec<ParagraphDetail>,
}

impl crate::FlatSerialize for ParagraphInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.paragraph_count,
            &format!("{}.ParagraphCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.paragraph_details,
            &format!("{}.ParagraphDetails", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QrCodeDetail {
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "QrCodeAngle")]
    pub qr_code_angle: i32,
    #[serde(rename = "QrCodePoints")]
    pub qr_code_points: Vec<QrCodePoint>,
    #[serde(rename = "QrCodeRect")]
    pub qr_code_rect: QrCodeRect,
}

impl crate::FlatSerialize for QrCodeDetail {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.data, &format!("{}.Data", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.qr_code_angle,
            &format!("{}.QrCodeAngle", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qr_code_points,
            &format!("{}.QrCodePoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qr_code_rect,
            &format!("{}.QrCodeRect", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QrCodeInfo {
    #[serde(rename = "QrCodeCount")]
    pub qr_code_count: i32,
    #[serde(rename = "QrCodeDetails")]
    pub qr_code_details: Vec<QrCodeDetail>,
}

impl crate::FlatSerialize for QrCodeInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.qr_code_count,
            &format!("{}.QrCodeCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qr_code_details,
            &format!("{}.QrCodeDetails", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QrCodePoint {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
}

impl crate::FlatSerialize for QrCodePoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.x, &format!("{}.X", name), params);
        crate::FlatSerialize::flat_serialize(&self.y, &format!("{}.Y", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QrCodeRect {
    #[serde(rename = "CenterX")]
    pub center_x: i32,
    #[serde(rename = "CenterY")]
    pub center_y: i32,
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "Width")]
    pub width: i32,
}

impl crate::FlatSerialize for QrCodeRect {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.center_x, &format!("{}.CenterX", name), params);
        crate::FlatSerialize::flat_serialize(&self.center_y, &format!("{}.CenterY", name), params);
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QualityInfo {
    #[serde(rename = "CompletenessScore")]
    pub completeness_score: f32,
    #[serde(rename = "IsCopy")]
    pub is_copy: bool,
    #[serde(rename = "IsReshoot")]
    pub is_reshoot: bool,
    #[serde(rename = "QualityScore")]
    pub quality_score: f32,
    #[serde(rename = "TamperScore")]
    pub tamper_score: f32,
}

impl crate::FlatSerialize for QualityInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.completeness_score,
            &format!("{}.CompletenessScore", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.is_copy, &format!("{}.IsCopy", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.is_reshoot,
            &format!("{}.IsReshoot", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.quality_score,
            &format!("{}.QualityScore", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.tamper_score,
            &format!("{}.TamperScore", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RowDetail {
    #[serde(rename = "BlockList")]
    pub block_list: Vec<i32>,
    #[serde(rename = "RowContent")]
    pub row_content: String,
    #[serde(rename = "RowId")]
    pub row_id: i32,
}

impl crate::FlatSerialize for RowDetail {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.block_list,
            &format!("{}.BlockList", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.row_content,
            &format!("{}.RowContent", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.row_id, &format!("{}.RowId", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RowInfo {
    #[serde(rename = "RowCount")]
    pub row_count: i32,
    #[serde(rename = "RowDetails")]
    pub row_details: Vec<RowDetail>,
}

impl crate::FlatSerialize for RowInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.row_count,
            &format!("{}.RowCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.row_details,
            &format!("{}.RowDetails", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StampDetail {
    #[serde(rename = "Data")]
    pub data: ItemData,
    #[serde(rename = "StampAngle")]
    pub stamp_angle: i32,
    #[serde(rename = "StampPoints")]
    pub stamp_points: Vec<StampPoint>,
    #[serde(rename = "StampRect")]
    pub stamp_rect: StampRect,
}

impl crate::FlatSerialize for StampDetail {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.data, &format!("{}.Data", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.stamp_angle,
            &format!("{}.StampAngle", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.stamp_points,
            &format!("{}.StampPoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.stamp_rect,
            &format!("{}.StampRect", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StampInfo {
    #[serde(rename = "StampCount")]
    pub stamp_count: i32,
    #[serde(rename = "StampDetails")]
    pub stamp_details: Vec<StampDetail>,
}

impl crate::FlatSerialize for StampInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.stamp_count,
            &format!("{}.StampCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.stamp_details,
            &format!("{}.StampDetails", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StampPoint {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
}

impl crate::FlatSerialize for StampPoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.x, &format!("{}.X", name), params);
        crate::FlatSerialize::flat_serialize(&self.y, &format!("{}.Y", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StampRect {
    #[serde(rename = "CenterX")]
    pub center_x: i32,
    #[serde(rename = "CenterY")]
    pub center_y: i32,
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "Width")]
    pub width: i32,
}

impl crate::FlatSerialize for StampRect {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.center_x, &format!("{}.CenterX", name), params);
        crate::FlatSerialize::flat_serialize(&self.center_y, &format!("{}.CenterY", name), params);
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StructureResponseData {
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "SubImageCount")]
    pub sub_image_count: i32,
    #[serde(rename = "SubImages")]
    pub sub_images: Vec<StructureResponseDataSubImage>,
    #[serde(rename = "Width")]
    pub width: i32,
}

impl crate::FlatSerialize for StructureResponseData {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.sub_image_count,
            &format!("{}.SubImageCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.sub_images,
            &format!("{}.SubImages", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StructureResponseDataSubImage {
    #[serde(rename = "Angle")]
    pub angle: i32,
    #[serde(rename = "KvInfo")]
    pub kv_info: StructureResponseDataSubImagesItemKvInfo,
    #[serde(rename = "SubImageId")]
    pub sub_image_id: i32,
}

impl crate::FlatSerialize for StructureResponseDataSubImage {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.angle, &format!("{}.Angle", name), params);
        crate::FlatSerialize::flat_serialize(&self.kv_info, &format!("{}.KvInfo", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.sub_image_id,
            &format!("{}.SubImageId", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StructureResponseDataSubImagesItemKvInfo {
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "KvCount")]
    pub kv_count: i32,
}

impl crate::FlatSerialize for StructureResponseDataSubImagesItemKvInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.data, &format!("{}.Data", name), params);
        crate::FlatSerialize::flat_serialize(&self.kv_count, &format!("{}.KvCount", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TableConfig {
    #[serde(rename = "IsHandWritingTable")]
    pub is_hand_writing_table: bool,
    #[serde(rename = "IsLineLessTable")]
    pub is_line_less_table: bool,
    #[serde(rename = "OutputTableExcel")]
    pub output_table_excel: bool,
    #[serde(rename = "OutputTableHtml")]
    pub output_table_html: bool,
}

impl crate::FlatSerialize for TableConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.is_hand_writing_table,
            &format!("{}.IsHandWritingTable", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.is_line_less_table,
            &format!("{}.IsLineLessTable", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_table_excel,
            &format!("{}.OutputTableExcel", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.output_table_html,
            &format!("{}.OutputTableHtml", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TableDetail {
    #[serde(rename = "CellCount")]
    pub cell_count: i32,
    #[serde(rename = "CellDetails")]
    pub cell_details: Vec<CellDetail>,
    #[serde(rename = "ColumnCount")]
    pub column_count: i32,
    #[serde(rename = "Footer")]
    pub footer: Footer,
    #[serde(rename = "Header")]
    pub header: Header,
    #[serde(rename = "RowCount")]
    pub row_count: i32,
    #[serde(rename = "TableId")]
    pub table_id: i32,
    #[serde(rename = "TablePoints")]
    pub table_points: Vec<TablePoint>,
    #[serde(rename = "TableRect")]
    pub table_rect: TableRect,
}

impl crate::FlatSerialize for TableDetail {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.cell_count,
            &format!("{}.CellCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.cell_details,
            &format!("{}.CellDetails", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.column_count,
            &format!("{}.ColumnCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.footer, &format!("{}.Footer", name), params);
        crate::FlatSerialize::flat_serialize(&self.header, &format!("{}.Header", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.row_count,
            &format!("{}.RowCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.table_id, &format!("{}.TableId", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.table_points,
            &format!("{}.TablePoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.table_rect,
            &format!("{}.TableRect", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TableInfo {
    #[serde(rename = "TableCount")]
    pub table_count: i32,
    #[serde(rename = "TableDetails")]
    pub table_details: Vec<TableDetail>,
    #[serde(rename = "TableExcel")]
    pub table_excel: String,
    #[serde(rename = "TableHtml")]
    pub table_html: String,
}

impl crate::FlatSerialize for TableInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.table_count,
            &format!("{}.TableCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.table_details,
            &format!("{}.TableDetails", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.table_excel,
            &format!("{}.TableExcel", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.table_html,
            &format!("{}.TableHtml", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TablePoint {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
}

impl crate::FlatSerialize for TablePoint {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.x, &format!("{}.X", name), params);
        crate::FlatSerialize::flat_serialize(&self.y, &format!("{}.Y", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TableRect {
    #[serde(rename = "CenterX")]
    pub center_x: i32,
    #[serde(rename = "CenterY")]
    pub center_y: i32,
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "Width")]
    pub width: i32,
}

impl crate::FlatSerialize for TableRect {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.center_x, &format!("{}.CenterX", name), params);
        crate::FlatSerialize::flat_serialize(&self.center_y, &format!("{}.CenterY", name), params);
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextIdCardConfig {
    #[serde(rename = "Llm_rec")]
    pub llm_rec: bool,
    #[serde(rename = "OutputIdCardQuality")]
    pub output_id_card_quality: bool,
}

impl crate::FlatSerialize for TextIdCardConfig {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.llm_rec, &format!("{}.Llm_rec", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.output_id_card_quality,
            &format!("{}.OutputIdCardQuality", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextResponseData {
    #[serde(rename = "AlgoServer")]
    pub algo_server: Vec<String>,
    #[serde(rename = "AlgoVersion")]
    pub algo_version: String,
    #[serde(rename = "Content")]
    pub content: String,
    #[serde(rename = "DebugInfo")]
    pub debug_info: String,
    #[serde(rename = "Height")]
    pub height: i32,
    #[serde(rename = "IsMixedMode")]
    pub is_mixed_mode: bool,
    #[serde(rename = "KvExcelUrl")]
    pub kv_excel_url: String,
    #[serde(rename = "PageNo")]
    pub page_no: i32,
    #[serde(rename = "SubImageCount")]
    pub sub_image_count: i32,
    #[serde(rename = "SubImages")]
    pub sub_images: Vec<TextResponseDataSubImage>,
    #[serde(rename = "Width")]
    pub width: i32,
    #[serde(rename = "XmlResult")]
    pub xml_result: String,
}

impl crate::FlatSerialize for TextResponseData {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(
            &self.algo_server,
            &format!("{}.AlgoServer", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.algo_version,
            &format!("{}.AlgoVersion", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.content, &format!("{}.Content", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.debug_info,
            &format!("{}.DebugInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.height, &format!("{}.Height", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.is_mixed_mode,
            &format!("{}.IsMixedMode", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.kv_excel_url,
            &format!("{}.KvExcelUrl", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.page_no, &format!("{}.PageNo", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.sub_image_count,
            &format!("{}.SubImageCount", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.sub_images,
            &format!("{}.SubImages", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.width, &format!("{}.Width", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.xml_result,
            &format!("{}.XmlResult", name),
            params,
        );
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextResponseDataSubImage {
    #[serde(rename = "Angle")]
    pub angle: i32,
    #[serde(rename = "BarCodeInfo")]
    pub bar_code_info: BarCodeInfo,
    #[serde(rename = "BlockInfo")]
    pub block_info: BlockInfo,
    #[serde(rename = "FigureInfo")]
    pub figure_info: FigureInfo,
    #[serde(rename = "KvInfo")]
    pub kv_info: TextResponseDataSubImagesItemKvInfo,
    #[serde(rename = "ParagraphInfo")]
    pub paragraph_info: ParagraphInfo,
    #[serde(rename = "QrCodeInfo")]
    pub qr_code_info: QrCodeInfo,
    #[serde(rename = "QualityInfo")]
    pub quality_info: QualityInfo,
    #[serde(rename = "RowInfo")]
    pub row_info: RowInfo,
    #[serde(rename = "StampInfo")]
    pub stamp_info: StampInfo,
    #[serde(rename = "SubImageId")]
    pub sub_image_id: i32,
    #[serde(rename = "SubImagePoints")]
    pub sub_image_points: Vec<ImagePoint>,
    #[serde(rename = "SubImageRect")]
    pub sub_image_rect: ImageRect,
    #[serde(rename = "TableInfo")]
    pub table_info: TableInfo,
    #[serde(rename = "Type")]
    pub r#type: String,
}

impl crate::FlatSerialize for TextResponseDataSubImage {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.angle, &format!("{}.Angle", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.bar_code_info,
            &format!("{}.BarCodeInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.block_info,
            &format!("{}.BlockInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.figure_info,
            &format!("{}.FigureInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.kv_info, &format!("{}.KvInfo", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.paragraph_info,
            &format!("{}.ParagraphInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.qr_code_info,
            &format!("{}.QrCodeInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.quality_info,
            &format!("{}.QualityInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.row_info, &format!("{}.RowInfo", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.stamp_info,
            &format!("{}.StampInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.sub_image_id,
            &format!("{}.SubImageId", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.sub_image_points,
            &format!("{}.SubImagePoints", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.sub_image_rect,
            &format!("{}.SubImageRect", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(
            &self.table_info,
            &format!("{}.TableInfo", name),
            params,
        );
        crate::FlatSerialize::flat_serialize(&self.r#type, &format!("{}.Type", name), params);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextResponseDataSubImagesItemKvInfo {
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "KvCount")]
    pub kv_count: i32,
    #[serde(rename = "KvDetails")]
    pub kv_details: KvDetail,
}

impl crate::FlatSerialize for TextResponseDataSubImagesItemKvInfo {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        crate::FlatSerialize::flat_serialize(&self.data, &format!("{}.Data", name), params);
        crate::FlatSerialize::flat_serialize(&self.kv_count, &format!("{}.KvCount", name), params);
        crate::FlatSerialize::flat_serialize(
            &self.kv_details,
            &format!("{}.KvDetails", name),
            params,
        );
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConfigCountry {
    #[serde(rename = "India")]
    India,
    #[serde(rename = "Korea")]
    Korea,
    #[serde(rename = "Vietnam")]
    Vietnam,
    #[serde(rename = "Bangladesh")]
    Bangladesh,
}

impl Default for ConfigCountry {
    fn default() -> Self {
        Self::India
    }
}

impl ConfigCountry {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::India => "India",
            Self::Korea => "Korea",
            Self::Vietnam => "Vietnam",
            Self::Bangladesh => "Bangladesh",
        }
    }
}

impl<'a> From<&'a ConfigCountry> for crate::QueryValue<'a> {
    fn from(value: &'a ConfigCountry) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for ConfigCountry {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        params.insert(name.to_string().into(), self.into());
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IdcardCountry {
    /// India
    #[serde(rename = "India")]
    India,
    /// Vietnam
    #[serde(rename = "Vietnam")]
    Vietnam,
    /// Korea
    #[serde(rename = "Korea")]
    Korea,
    /// Bangladesh
    #[serde(rename = "Bangladesh")]
    Bangladesh,
}

impl Default for IdcardCountry {
    fn default() -> Self {
        Self::India
    }
}

impl IdcardCountry {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::India => "India",
            Self::Vietnam => "Vietnam",
            Self::Korea => "Korea",
            Self::Bangladesh => "Bangladesh",
        }
    }
}

impl<'a> From<&'a IdcardCountry> for crate::QueryValue<'a> {
    fn from(value: &'a IdcardCountry) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for IdcardCountry {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        params.insert(name.to_string().into(), self.into());
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LicenseCountry {
    #[serde(rename = "India")]
    India,
    #[serde(rename = "Korea")]
    Korea,
}

impl Default for LicenseCountry {
    fn default() -> Self {
        Self::India
    }
}

impl LicenseCountry {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::India => "India",
            Self::Korea => "Korea",
        }
    }
}

impl<'a> From<&'a LicenseCountry> for crate::QueryValue<'a> {
    fn from(value: &'a LicenseCountry) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for LicenseCountry {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        params.insert(name.to_string().into(), self.into());
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Type {
    #[serde(rename = "Advanced")]
    Advanced,
    #[serde(rename = "General")]
    General,
    #[serde(rename = "HandWriting")]
    HandWriting,
    #[serde(rename = "Commerce")]
    Commerce,
    #[serde(rename = "MultiLang")]
    MultiLang,
    #[serde(rename = "IdCard")]
    IdCard,
    #[serde(rename = "InternationalPassport")]
    InternationalPassport,
    #[serde(rename = "HouseholdHead")]
    HouseholdHead,
    #[serde(rename = "HouseholdResident")]
    HouseholdResident,
    #[serde(rename = "EstateCertification")]
    EstateCertification,
    #[serde(rename = "BankCard")]
    BankCard,
    #[serde(rename = "BirthCertification")]
    BirthCertification,
    #[serde(rename = "ChinesePassport")]
    ChinesePassport,
    #[serde(rename = "PermitToHK_MO_TW")]
    PermitToHkMoTw,
    #[serde(rename = "PermitToMainland")]
    PermitToMainland,
    #[serde(rename = "HKIdCard")]
    HkIdCard,
    #[serde(rename = "SocialSecurityCard")]
    SocialSecurityCard,
    #[serde(rename = "InternationalIdCard")]
    InternationalIdCard,
    #[serde(rename = "Stamp")]
    Stamp,
    #[serde(rename = "Invoice")]
    Invoice,
    #[serde(rename = "CarInvoice")]
    CarInvoice,
    #[serde(rename = "QuotaInvoice")]
    QuotaInvoice,
    #[serde(rename = "AirItinerary")]
    AirItinerary,
    #[serde(rename = "TrainTicket")]
    TrainTicket,
    #[serde(rename = "TaxiInvoice")]
    TaxiInvoice,
    #[serde(rename = "RollTicket")]
    RollTicket,
    #[serde(rename = "BankAcceptance")]
    BankAcceptance,
    #[serde(rename = "BusShipTicket")]
    BusShipTicket,
    #[serde(rename = "NonTaxInvoice")]
    NonTaxInvoice,
    #[serde(rename = "CommonPrintedInvoice")]
    CommonPrintedInvoice,
    #[serde(rename = "HotelConsume")]
    HotelConsume,
    #[serde(rename = "PaymentRecord")]
    PaymentRecord,
    #[serde(rename = "PurchaseRecord")]
    PurchaseRecord,
    #[serde(rename = "RideHailingItinerary")]
    RideHailingItinerary,
    #[serde(rename = "ShoppingReceipt")]
    ShoppingReceipt,
    #[serde(rename = "TollInvoice")]
    TollInvoice,
    #[serde(rename = "TaxClearanceCertificate")]
    TaxClearanceCertificate,
    #[serde(rename = "UsedCarInvoice")]
    UsedCarInvoice,
    #[serde(rename = "VehicleLicense")]
    VehicleLicense,
    #[serde(rename = "DrivingLicense")]
    DrivingLicense,
    #[serde(rename = "LicensePlateNumber")]
    LicensePlateNumber,
    #[serde(rename = "MixedInvoice")]
    MixedInvoice,
    #[serde(rename = "BusinessLicense")]
    BusinessLicense,
    #[serde(rename = "CarVinCode")]
    CarVinCode,
    #[serde(rename = "InternationalBusinessLicense")]
    InternationalBusinessLicense,
    #[serde(rename = "WayBill")]
    WayBill,
    #[serde(rename = "FoodProduceLicense")]
    FoodProduceLicense,
    #[serde(rename = "FoodManagementLicense")]
    FoodManagementLicense,
    #[serde(rename = "MedicalDeviceManageLicense")]
    MedicalDeviceManageLicense,
    #[serde(rename = "MedicalDeviceProduceLicense")]
    MedicalDeviceProduceLicense,
    #[serde(rename = "ClassIIMedicalDeviceManageLicense")]
    ClassIiMedicalDeviceManageLicense,
    #[serde(rename = "CosmeticProduceLicense")]
    CosmeticProduceLicense,
    #[serde(rename = "VehicleRegistration")]
    VehicleRegistration,
    #[serde(rename = "VehicleCertification")]
    VehicleCertification,
    #[serde(rename = "QrCode")]
    QrCode,
    #[serde(rename = "BarCode")]
    BarCode,
    #[serde(rename = "BankAccountPermit")]
    BankAccountPermit,
    #[serde(rename = "Table")]
    Table,
    #[serde(rename = "TrademarkCertificate")]
    TrademarkCertificate,
}

impl Default for Type {
    fn default() -> Self {
        Self::Advanced
    }
}

impl Type {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Advanced => "Advanced",
            Self::General => "General",
            Self::HandWriting => "HandWriting",
            Self::Commerce => "Commerce",
            Self::MultiLang => "MultiLang",
            Self::IdCard => "IdCard",
            Self::InternationalPassport => "InternationalPassport",
            Self::HouseholdHead => "HouseholdHead",
            Self::HouseholdResident => "HouseholdResident",
            Self::EstateCertification => "EstateCertification",
            Self::BankCard => "BankCard",
            Self::BirthCertification => "BirthCertification",
            Self::ChinesePassport => "ChinesePassport",
            Self::PermitToHkMoTw => "PermitToHK_MO_TW",
            Self::PermitToMainland => "PermitToMainland",
            Self::HkIdCard => "HKIdCard",
            Self::SocialSecurityCard => "SocialSecurityCard",
            Self::InternationalIdCard => "InternationalIdCard",
            Self::Stamp => "Stamp",
            Self::Invoice => "Invoice",
            Self::CarInvoice => "CarInvoice",
            Self::QuotaInvoice => "QuotaInvoice",
            Self::AirItinerary => "AirItinerary",
            Self::TrainTicket => "TrainTicket",
            Self::TaxiInvoice => "TaxiInvoice",
            Self::RollTicket => "RollTicket",
            Self::BankAcceptance => "BankAcceptance",
            Self::BusShipTicket => "BusShipTicket",
            Self::NonTaxInvoice => "NonTaxInvoice",
            Self::CommonPrintedInvoice => "CommonPrintedInvoice",
            Self::HotelConsume => "HotelConsume",
            Self::PaymentRecord => "PaymentRecord",
            Self::PurchaseRecord => "PurchaseRecord",
            Self::RideHailingItinerary => "RideHailingItinerary",
            Self::ShoppingReceipt => "ShoppingReceipt",
            Self::TollInvoice => "TollInvoice",
            Self::TaxClearanceCertificate => "TaxClearanceCertificate",
            Self::UsedCarInvoice => "UsedCarInvoice",
            Self::VehicleLicense => "VehicleLicense",
            Self::DrivingLicense => "DrivingLicense",
            Self::LicensePlateNumber => "LicensePlateNumber",
            Self::MixedInvoice => "MixedInvoice",
            Self::BusinessLicense => "BusinessLicense",
            Self::CarVinCode => "CarVinCode",
            Self::InternationalBusinessLicense => "InternationalBusinessLicense",
            Self::WayBill => "WayBill",
            Self::FoodProduceLicense => "FoodProduceLicense",
            Self::FoodManagementLicense => "FoodManagementLicense",
            Self::MedicalDeviceManageLicense => "MedicalDeviceManageLicense",
            Self::MedicalDeviceProduceLicense => "MedicalDeviceProduceLicense",
            Self::ClassIiMedicalDeviceManageLicense => "ClassIIMedicalDeviceManageLicense",
            Self::CosmeticProduceLicense => "CosmeticProduceLicense",
            Self::VehicleRegistration => "VehicleRegistration",
            Self::VehicleCertification => "VehicleCertification",
            Self::QrCode => "QrCode",
            Self::BarCode => "BarCode",
            Self::BankAccountPermit => "BankAccountPermit",
            Self::Table => "Table",
            Self::TrademarkCertificate => "TrademarkCertificate",
        }
    }
}

impl<'a> From<&'a Type> for crate::QueryValue<'a> {
    fn from(value: &'a Type) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for Type {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        params.insert(name.to_string().into(), self.into());
    }
}

/// Enum type marshalled as String
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Writing {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for Writing {
    fn default() -> Self {
        Self::True
    }
}

impl Writing {
    /// Returns the string value of this enum variant as used in the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
        }
    }
}

impl<'a> From<&'a Writing> for crate::QueryValue<'a> {
    fn from(value: &'a Writing) -> Self {
        crate::QueryValue::from(value.as_str())
    }
}

impl crate::FlatSerialize for Writing {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut std::collections::BTreeMap<
            std::borrow::Cow<'static, str>,
            crate::QueryValue<'a>,
        >,
    ) {
        params.insert(name.to_string().into(), self.into());
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeAllTextResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: TextResponseData,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeAllTextResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeGeneralStructureResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: StructureResponseData,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeGeneralStructureResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeAdvancedResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeAdvancedResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeHandwritingResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeHandwritingResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeBasicResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeBasicResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeGeneralResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeGeneralResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeTableOcrResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeTableOcrResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeHealthCodeResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeHealthCodeResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeDocumentStructureResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeDocumentStructureResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeIdcardResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeIdcardResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizePassportResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizePassportResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeHouseholdResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeHouseholdResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeEstateCertificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeEstateCertificationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeBankCardResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeBankCardResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeBirthCertificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeBirthCertificationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeChinesePassportResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeChinesePassportResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeExitEntryPermitToMainlandResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeExitEntryPermitToMainlandResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeExitEntryPermitToHKResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeExitEntryPermitToHKResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeHKIdcardResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeHKIdcardResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeSocialSecurityCardVersionIIResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeSocialSecurityCardVersionIIResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeInternationalIdcardResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeInternationalIdcardResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeMixedInvoicesResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeMixedInvoicesResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeCarInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeCarInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeQuotaInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeQuotaInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeAirItineraryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeAirItineraryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeTrainInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeTrainInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeTaxiInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeTaxiInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeRollTicketResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeRollTicketResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeBankAcceptanceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeBankAcceptanceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeBusShipTicketResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeBusShipTicketResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeNonTaxInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeNonTaxInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeCommonPrintedInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeCommonPrintedInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeHotelConsumeResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeHotelConsumeResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizePaymentRecordResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizePaymentRecordResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizePurchaseRecordResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizePurchaseRecordResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeRideHailingItineraryResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeRideHailingItineraryResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeShoppingReceiptResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeShoppingReceiptResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeSocialSecurityCardResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeSocialSecurityCardResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeTollInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeTollInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeTaxClearanceCertificateResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeTaxClearanceCertificateResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeUsedCarInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeUsedCarInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeBusinessLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeBusinessLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeBankAccountLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeBankAccountLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeTradeMarkCertificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeTradeMarkCertificationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeFoodProduceLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeFoodProduceLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeFoodManageLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeFoodManageLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeMedicalDeviceManageLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeMedicalDeviceManageLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeMedicalDeviceProduceLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeMedicalDeviceProduceLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeCtwoMedicalDeviceManageLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeCtwoMedicalDeviceManageLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeCosmeticProduceLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeCosmeticProduceLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeInternationalBusinessLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeInternationalBusinessLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeVehicleLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeVehicleLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeDrivingLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeDrivingLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeWaybillResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeWaybillResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeCarNumberResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeCarNumberResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeCarVinCodeResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeCarVinCodeResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeVehicleRegistrationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeVehicleRegistrationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeVehicleCertificationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeVehicleCertificationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeEduFormulaResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeEduFormulaResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeEduOralCalculationResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeEduOralCalculationResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeEduPaperOcrResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeEduPaperOcrResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeEduPaperCutResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeEduPaperCutResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeEduQuestionOcrResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeEduQuestionOcrResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeEduPaperStructedResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeEduPaperStructedResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeMultiLanguageResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeMultiLanguageResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeEnglishResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeEnglishResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeThaiResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeThaiResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeJanpaneseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeJanpaneseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeKoreanResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeKoreanResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeLatinResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeLatinResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeRussianResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeRussianResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct RecognizeCovidTestReportResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for RecognizeCovidTestReportResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct VerifyBusinessLicenseResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for VerifyBusinessLicenseResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct VerifyVATInvoiceResponse {
    #[serde(flatten)]
    pub code_message: crate::CodeMessage,
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl AsRef<crate::CodeMessage> for VerifyVATInvoiceResponse {
    fn as_ref(&self) -> &crate::CodeMessage {
        &self.code_message
    }
}
