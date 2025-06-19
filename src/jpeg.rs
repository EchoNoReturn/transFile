mod png;

pub fn build() -> clap::Command {
  clap::Command::new("jpeg")
    .about("JPEG 图片处理 转换为其他格式")
    .arg(
      clap::Arg::new("input")
        .index(1)
        .help("输入 JPEG 文件路径")
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
        .value_parser(["png", "webp", "bmp"]),
    )
}

pub fn execute(matches: clap::ArgMatches) -> anyhow::Result<()> {
    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();

    match matches.get_one::<String>("target").unwrap().as_str() {
        "png" => png::jpeg_to_png(input_path, output_path)?,
        _ => return Err(anyhow::anyhow!("不支持的目标格式")),
    }

    Ok(())
}
