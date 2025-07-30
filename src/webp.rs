mod png;

pub struct WebpArgs {
    pub input_path: String,
    pub output_path: String,
    pub target_formate: String, // 目前仅支持 png
    pub quality: u8,            // WebP 质量，范围 1-100
}

pub fn build() -> clap::Command {
    clap::Command::new("webp")
        .about("WebP 图片处理，转换为 png 格式")
        .arg(
            clap::Arg::new("input")
                .index(1)
                .help("输入 WebP 文件路径")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("output")
                .index(2)
                .help("输出文件路径")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("target")
                .short('t')
                .long("to")
                .help("目标格式 (目前仅支持 png)")
                .default_value("png")
                .value_parser(["png"]),
        )
        .arg(
            clap::Arg::new("quality")
                .short('q')
                .long("quality")
                .help("WebP 质量，范围 1-100。默认值: 100")
                .default_value("100")
                .value_parser(clap::value_parser!(u32)),
        )
}

pub fn execute(matches: clap::ArgMatches) -> anyhow::Result<()> {
    let webp_args = WebpArgs {
        input_path: matches.get_one::<String>("input").unwrap().clone(),
        output_path: matches.get_one::<String>("output").unwrap().clone(),
        target_formate: matches.get_one::<String>("target").unwrap().clone(),
        quality: *matches.get_one::<u32>("quality").unwrap_or(&100) as u8,
    };

    // 确保质量值在1-100范围内
    let quality = webp_args.quality.clamp(1, 100) as u8;

    match webp_args.target_formate.as_str() {
        "png" => {
            png::webp_to_png(&webp_args.input_path, &webp_args.output_path, quality)?;
        }
        _ => {
            return Err(anyhow::anyhow!(
                "不支持的目标格式: {}",
                webp_args.target_formate
            ));
        }
    }

    Ok(())
}
