pub mod upload_types {
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UploadConfig {
        /// 必选，指定上传域名
        pub domain: String,
        pub access_key: String,
        pub secret_key: String,
        pub bucket_name: String,
        /// 可选，指定上传区域，适配阿里云
        pub region: Option<String>,
        /// 可选，指定上传超时时间，默认为 60 秒
        pub timeout: Option<Duration>,
        /// 可选，上传文件前缀. 如果不指定，则不添加前缀
        /// 例如: "user_uploads/"
        /// 如果指定为 "user_uploads/", 则上传的文件会以 "user_uploads/" 开头
        /// 例如: "user_uploads/myfile.png"
        /// 如果不指定，则上传的文件会直接以文件名为前缀
        /// 例如: "myfile.png"
        pub prefix: Option<String>, // 可选，上传文件前缀
    }

    impl UploadConfig {
        pub fn new(
            domain: String,
            access_key: String,
            secret_key: String,
            bucket_name: String,
        ) -> Self {
            Self {
                domain,
                access_key,
                secret_key,
                bucket_name,
                region: None,
                timeout: Some(Duration::from_secs(60)),
                prefix: None,
            }
        }

        pub fn with_region(mut self, region: String) -> Self {
            self.region = Some(region);
            self
        }

        pub fn with_timeout(mut self, timeout: Duration) -> Self {
            self.timeout = Some(timeout);
            self
        }

        pub fn with_prefix(mut self, prefix: String) -> Self {
            self.prefix = Some(prefix);
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UploadResult {
        pub success: bool,
        pub url: String,
        pub error: Option<String>,
        pub file_name: String,
        pub uploaded_path: String,
    }
}

// 定义策略接口
#[async_trait::async_trait]
pub trait UploadStrategy {
    async fn upload(
        &self,
        file_path: &str,
        prefix: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
    fn get_provider_name(&self) -> &str;
    fn get_read_url(&self, key: &str) -> String;
}
