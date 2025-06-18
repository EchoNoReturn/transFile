use clap::{Arg, Command};
use anyhow::{Result};

mod icns;

// 定义 PNG 子命令的参数结构
pub struct PngArgs {
    pub target_format: String,
    pub size: u32,
    pub input_path: String,
    pub output_path: String,
}

pub fn build() -> Command {
    Command::new("png")
        .about("png 图片处理")
        .arg(
            Arg::new("target")
                .short('t')
                .long("to")
                .help("目标格式 (icns, jpg, webp, etc.)")
                .required(true)
                .value_parser(["icns", "jpeg", "webp", "bmp"]),
        )
        .arg(
          Arg::new("size")
          .short('s')
          .long("size")
          .help("目标图标大小 (例如: 16, 32, 64, 128, 256, 512, 1024)")
          .required(true)
          .value_parser(clap::value_parser!(u32))
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("输入文件路径")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
          Arg::new("output")
            .short('o')
            .long("output")
            .help("输出文件路径")
            .required(true)
            .value_parser(clap::value_parser!(String)),
        )
}

// 执行 png 命令
pub fn execute(args: PngArgs) -> Result<()> {
    match args.target_format.as_str() {
        "icns" => {
          icns::png_to_icns(&args.input_path, &args.output_path, &args.size)?;
          println!("转换为 ICNS 格式");
        },
        "jpeg" => println!("转换为 JPEG 格式"),
        "webp" => println!("转换为 WEBP 格式"),
        "bmp" => println!("转换为 BMP 格式"),
        _ => return Err(anyhow::anyhow!("不支持的目标格式: {}", args.target_format)),
    }
    Ok(())
}