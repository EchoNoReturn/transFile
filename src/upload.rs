use crate::conf;
use clap::{Arg, ArgMatches};
use upload as upload_util;

pub fn command_builder() -> clap::Command {
    clap::Command::new("upload")
        .about("上传文件到图床服务")
        .arg(
            Arg::new("inputs")
                .help("输入文件路径(支持多张图片上传)")
                .required(true)
                .num_args(1..) // 支持多张图片上传
                .value_parser(clap::value_parser!(String)),
        )
}

pub async fn execute(matches: ArgMatches) {
    // 读取配置文件
    let config = conf::load_config(&conf::CONFIG_FILE).unwrap_or_else(|_| {
        panic!(
            "加载配置失败，请检查配置文件路径和格式，或确保配置文件存在于{:?}",
            conf::CONFIG_FILE
        )
    });

    let config = config.upload.parse_to_upload_config();

    let input_paths: Vec<&String> = matches
        .get_many::<String>("inputs")
        .unwrap_or_default()
        .collect();

    let mut results: Vec<upload_util::common::upload_types::UploadResult> = Vec::new();
    for input_path in input_paths {
        let result = upload_util::upload_image(input_path, config.clone()).await;
        results.push(result);
    }

    println!("Upload Success: ");
    for result in &results {
        if result.success {
            println!("{}", result.url);
        } else {
            println!("上传失败: {}", result.error.as_deref().unwrap_or("未知错误"));
        }
    }
}
