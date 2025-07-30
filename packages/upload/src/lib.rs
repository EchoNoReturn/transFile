use crate::common::UploadStrategy;

mod aliyun;
pub mod common;

pub async fn upload_image(
    path: &str,
    config: common::upload_types::UploadConfig,
) -> common::upload_types::UploadResult {
    let aliyun_oss_client = aliyun::AliOssClient::new(
        config.access_key.clone().as_str(),
        config.secret_key.clone().as_str(),
        config
            .region
            .unwrap_or_else(|| "oss-cn-shanghai".to_string())
            .as_str(),
        config.domain.clone().as_str(),
        config.bucket_name.clone().as_str(),
    );

    let prefix = if config.prefix.is_some() {
        config.prefix.unwrap()
    } else {
        String::new()
    };

    match aliyun_oss_client.upload(path, &prefix).await {
        Ok(url) => common::upload_types::UploadResult {
            success: true,
            url: url.clone(),
            error: None,
            file_name: path.to_string(),
            // 从上传后的 url 中获取上传后的文件路径
            uploaded_path: url
                .split_once("aliyuncs.com/")
                .map(|(_, after_domain)| {
                    let path = after_domain.trim_start_matches('/');
                    path.to_string()
                })
                .unwrap_or_else(|| String::new()),
        },
        Err(e) => common::upload_types::UploadResult {
            success: false,
            url: String::new(),
            error: Some(e.to_string()),
            file_name: path.to_string(),
            uploaded_path: String::new(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use std::env;
    use tokio;

    fn get_test_image_path() -> String {
        let args: Vec<String> = env::args().collect();

        for (i, arg) in args.iter().enumerate() {
            if arg == "--test-image" && i + 1 < args.len() {
                return args[i + 1].clone();
            }
        }

        env::var("TEST_IMAGE_PATH").unwrap_or_else(|_| {
            // TODO : 使用默认测试图片路径
            "/path/to/test/image.png".to_string()
        })
    }

    #[tokio::test]
    async fn test_upload_image() {
        // dotenv::dotenv().ok();
        dotenv::from_filename(".env.test.local").ok();

        let test_image_path = get_test_image_path();
        println!("📁 使用测试图片: {}", test_image_path);

        // 验证文件存在
        if !std::path::Path::new(&test_image_path).exists() {
            println!("⚠️  测试图片不存在: {}", test_image_path);
            println!("💡 使用方法:");
            println!(
                "   cargo test test_upload_image -- --test-image /path/to/image.png --nocapture"
            );
            println!("   或设置环境变量: TEST_IMAGE_PATH=/path/to/image.png cargo test");
            return;
        }

        let domain = env::var("ALIYUN_OSS_DOMAIN")
            .unwrap_or_else(|_| "oss-cn-shanghai.aliyuncs.com".to_string());
        let access_key = env::var("ALIYUN_ACCESS_KEY").expect("请设置环境变量 ALIYUN_ACCESS_KEY");
        let secret_key = env::var("ALIYUN_SECRET_KEY").expect("请设置环境变量 ALIYUN_SECRET_KEY");
        let bucket_name =
            env::var("ALIYUN_BUCKET_NAME").unwrap_or_else(|_| "test-bucket".to_string());
        let region = env::var("ALIYUN_REGION").unwrap_or_else(|_| "oss-cn-shanghai".to_string());
        let prefix = env::var("ALIYUN_PREFIX").unwrap_or_else(|_| "test/".to_string());

        let config =
            common::upload_types::UploadConfig::new(domain, access_key, secret_key, bucket_name)
                .with_region(region)
                .with_prefix(prefix);
        let result = upload_image(test_image_path.as_str(), config).await;
        assert!(result.success, "err: {}", result.error.unwrap_or_default());
        println!("上传成功: {}", result.url);
        // 这里可以添加更多的断言或逻辑
    }
}
