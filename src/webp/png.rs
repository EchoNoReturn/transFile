use anyhow::Context;

pub fn webp_to_png(input_path: &str, output_path: &str, quality: u8) -> anyhow::Result<()> {
    // 加载 WebP 图像
    let file = std::fs::File::open(input_path)
        .context(format!("无法打开输入文件: {}", input_path))?;
    let reader = std::io::BufReader::new(file);
    let img = image::load(reader, image::ImageFormat::WebP)
        .context("无法加载 WebP 图像")?;
    // 确保输出目录存在
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        std::fs::create_dir_all(parent)
            .context(format!("无法创建输出目录: {}", parent.display()))?;
    }

    let png_data = webp::Encoder::from_rgba(img.to_rgba8().as_raw(), img.width(), img.height())
        .encode(quality as f32);

    // 保存为 PNG
    std::fs::write(&output_path, &*png_data)
        .context(format!("无法写入输出文件: {}", output_path))?;
    Ok(())
}