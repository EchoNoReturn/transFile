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
            url,
            err_msg: String::new(),
        },
        Err(e) => common::upload_types::UploadResult {
            success: false,
            url: String::new(),
            err_msg: e.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use dotenv;
    use std::env;

    #[tokio::test]
    async fn test_upload_image() {
        // dotenv::dotenv().ok();
        dotenv::from_filename(".env.test.local").ok();

        let domain = env::var("ALIYUN_OSS_DOMAIN")
            .unwrap_or_else(|_| "oss-cn-shanghai.aliyuncs.com".to_string());
        let access_key = env::var("ALIYUN_ACCESS_KEY")
            .expect("请设置环境变量 ALIYUN_ACCESS_KEY");
        let secret_key = env::var("ALIYUN_SECRET_KEY")
            .expect("请设置环境变量 ALIYUN_SECRET_KEY");
        let bucket_name = env::var("ALIYUN_BUCKET_NAME")
            .unwrap_or_else(|_| "test-bucket".to_string());
        let region = env::var("ALIYUN_REGION")
            .unwrap_or_else(|_| "oss-cn-shanghai".to_string());
        let prefix = env::var("ALIYUN_PREFIX")
            .unwrap_or_else(|_| "test/".to_string());

        let config = common::upload_types::UploadConfig::new(
            domain,
            access_key,
            secret_key,
            bucket_name,
        )
        .with_region(region)
        .with_prefix(prefix);
        let result = upload_image("/Users/jgl/CodeSpace/rustSpace/transfile/output/sss.png", config).await;
        assert!(result.success, "err: {}", result.err_msg);
        println!("上传成功: {}", result.url);
        // 这里可以添加更多的断言或逻辑
    }
}
//             "yoyoj-dev".to_string(),
//         )
//         .with_region("oss-cn-shanghai".to_string())
//         .with_prefix("test/".to_string());
//         let result = upload_image("/Users/jgl/CodeSpace/rustSpace/transfile/output/sss.png", config).await;
//         assert!(result.success, "err: {}", result.err_msg);
//         println!("上传成功: {}", result.url);
//         // 这里可以添加更多的断言或逻辑
//     }
// }
