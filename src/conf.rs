use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{Read};
use std::path::PathBuf;
use upload::common::upload_types;

/// 获取配置文件路径
pub fn get_config_file_path() -> PathBuf {
    let home_dir = dirs::home_dir()
        .expect("无法获取用户主目录");
    
    home_dir.join(".transFile").join("config.toml")
}

/// 配置文件路径的字符串表示
pub fn get_config_file_str() -> String {
    get_config_file_path()
        .to_string_lossy()
        .to_string()
}

// 为了向后兼容，保留常量（但使用函数获取）
lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG_FILE: String = get_config_file_str();
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub upload: UploadConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadConfig {
    pub access_key: String,
    pub secret_key: String,
    pub region: Option<String>,
    pub domain: String,
    pub bucket_name: String,
    pub prefix: Option<String>,
}

impl UploadConfig {
    /// 将当前配置转换为 upload 包需要的上传配置
    pub fn parse_to_upload_config(&self) -> upload_types::UploadConfig {
        upload_types::UploadConfig::new(
            self.domain.clone(),
            self.access_key.clone(),
            self.secret_key.clone(),
            self.bucket_name.clone(),
        )
        .with_region(self.region.clone().unwrap_or_default())
        .with_prefix(self.prefix.clone().unwrap_or_default())
    }
}

/// 从配置文件加载配置
pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
