use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

mod icns;
mod ico;
mod jpg;
mod webp;

// 定义 PNG 子命令的参数结构
pub struct PngArgs {
    pub target_format: String,
    pub size: Vec<u32>,
    pub quality: Vec<u32>,   // JPEG 质量参数
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
                .help("目标格式 (icns, ico, jpeg, webp, etc.) \n icns 如果使用多个尺寸会分别输出 \n ICO 文件可以包含多个尺寸的图标, ICO 格式通常支持的尺寸有限，常用的是 16, 32, 48, 64, 128, 256 \n")
                .required(true)
                .value_parser(["icns", "ico", "jpeg", "webp", "bmp"]),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .help("目标图标大小。ico、jpeg 此参数无效。ico 默认适配了 16、24、32、48、64、128、256  \n ICNS 仅支持 16, 32, 64, 128, 256, 512, 1024。\n 可以用逗号分隔多个 (例如: 16,32,64,128,256,512,1024)\n")
                .value_delimiter(',')
                .default_value("16,32,64,128,256")
                .value_parser(clap::value_parser!(u32))
                .num_args(1)  // 只接受一个参数，可以包含多个逗号分隔的值
        )
        .arg(
            Arg::new("quality")
                .short('q')
                .long("quality")
                .help("JPEG 质量，范围 1-100。可以用逗号分隔多个值，将为每个质量值生成一个文件。\n默认值: 100")
                .value_delimiter(',')
                .default_value("100")
                .value_parser(clap::value_parser!(u32))
                .num_args(1)
        )
        .arg(
            Arg::new("input")
                .index(1)
                .help("输入文件路径")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("output")
                .index(2)
                .help("输出文件路径")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
}

// 执行 png 命令
pub fn execute(matches: ArgMatches) -> Result<()> {
    let args = PngArgs {
        target_format: matches.get_one::<String>("target").unwrap().clone(),
        size: matches
            .get_many::<u32>("size")
            .map_or(vec![16, 32, 64, 128, 256, 512, 1024], |s| {
                s.copied().collect()
            }),
        quality: matches
            .get_many::<u32>("quality")
            .map_or(vec![100], |q| {
                q.copied().collect()
            }),
        input_path: matches.get_one::<String>("input").unwrap().clone(),
        output_path: matches.get_one::<String>("output").unwrap().clone(),
    };
    match args.target_format.as_str() {
        "icns" => {
            icns::png_to_icns(&args.input_path, &args.output_path, args.size)?;
        }
        "ico" => {
            ico::png_to_ico(&args.input_path, &args.output_path)?;
        }
        "jpeg" => {
            jpg::png_to_jpeg(&args.input_path, &args.output_path, &args.quality)?;
        }
        "webp" => {
            webp::png_to_webp(&args.input_path, &args.output_path, &args.quality)?;
        },
        _ => return Err(anyhow::anyhow!("不支持的目标格式: {}", args.target_format)),
    }
    Ok(())
}
