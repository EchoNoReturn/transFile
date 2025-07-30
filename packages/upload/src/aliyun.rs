use aliyun_oss_rust_sdk::error::OssError;
use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use async_trait::async_trait;

pub struct AliOssClient {
    endpoint: String,
    bucket: String,
    oss_client: OSS,
    request_builder: RequestBuilder,
}

impl AliOssClient {
    pub fn new(
        access_key_id: &str,
        access_key_secret: &str,
        region: &str,
        endpoint: &str,
        bucket: &str,
    ) -> Self {
        let domain = if !endpoint.is_empty() {
            endpoint.to_string()
        } else {
            format!("{}.aliyuncs.com", region)
        };
        let oss_client = OSS::new(
            access_key_id.to_string(),
            access_key_secret.to_string(),
            domain,
            bucket.to_string(),
        );
        let request_builder = RequestBuilder::new();
        Self {
            endpoint: endpoint.to_string(),
            bucket: bucket.to_string(),
            oss_client,
            request_builder,
        }
    }

    pub async fn upload_file(&self, key: &str, file_path: &str) -> Result<String, OssError> {
        // 判断文件是否存在
        if !std::path::Path::new(file_path).exists() {
            return Err(OssError::Err(format!("文件不存在: {}", file_path)));
        }

        // 读取文件的 mime 类型
        let mime_type = mime_guess::from_path(file_path)
            .first()
            .map(|mime| mime.to_string())
            .unwrap_or("application/octet-stream".to_string());

        let build = self
            .request_builder
            .clone()
            .with_content_type(mime_type.to_string());

        self.oss_client
            .clone()
            .put_object_from_file(key, file_path, build)
            .await
            .map_err(|e| OssError::Err(format!("上传失败: {:?}", e)))?;

        Ok("https://".to_string() + &self.bucket + "." + &self.endpoint + "/" + key)
    }
}

#[async_trait]
impl crate::common::UploadStrategy for AliOssClient {
    async fn upload(
        &self,
        file_path: &str,
        prefix: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 通过路径指向的文件名作为 key
        let key = std::path::Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Box::new(OssError::Err("无法获取文件名".to_string())))?;
        
        // 在 key 的前面添加上年月日的标记
        let key = format!("{}{}", chrono::Utc::now().format("%Y-%m-%d/"), key);

        // 如果有前缀，则添加前缀
        let key = if !prefix.is_empty() {
            format!("{}{}", prefix, key)
        } else {
            key.to_string()
        };

        self.upload_file(key.as_str(), file_path)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    fn get_provider_name(&self) -> &str {
        "aliyun"
    }

    fn get_read_url(&self, key: &str) -> String {
        format!("https://{}.{}/{}", self.bucket, self.endpoint, key)
    }
}
