use std::{fs::File, io::BufReader};

use anyhow::Context;

pub fn jpeg_to_png(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    // 加载 JPEG 图像
    let file = File::open(input_path)
        .context(format!("无法打开输入文件: {}", input_path))?;
    let reader = BufReader::new(file);
    let img = image::load(reader, image::ImageFormat::Jpeg)
        .context("无法加载 JPEG 图像")?;
      
    // 确保输出目录存在
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        std::fs::create_dir_all(parent)
            .context(format!("无法创建输出目录: {}", parent.display()))?;
    }

    // 保存为 PNG
    let output_file = File::create(output_path)
        .context(format!("无法创建输出文件: {}", output_path))?;
    img.write_to(&mut std::io::BufWriter::new(output_file), image::ImageOutputFormat::Png)
        .context("写入 PNG 文件失败")?;

    Ok(())
}