use crate::conf;
use clap::{Arg, ArgMatches};
use upload as upload_util;

pub fn command_builder() -> clap::Command {
    clap::Command::new("upload")
        .about("上传文件到图床服务")
        .arg(
            Arg::new("input")
                .index(1)
                .help("输入文件路径")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
}

pub async fn execute(matches: ArgMatches) {
    // 读取配置文件
    let config = conf::load_config(conf::CONFIG_FILE).expect(
        format!(
            "加载配置失败，请检查配置文件路径和格式，或确保配置文件存在于{}",
            conf::CONFIG_FILE
        )
        .as_str(),
    );

    let config = config.upload.parse_to_upload_config();
    let input_path = matches.get_one::<String>("input").unwrap();
    let result = upload_util::upload_image(&input_path, config).await;
    if result.success {
        println!("上传成功，文件 URL: {}", result.url);
    } else {
        eprintln!("上传失败: {}", result.err_msg);
    }
}
