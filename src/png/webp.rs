use anyhow::Context;
use image::ImageFormat;
use std::{fs::File, io::BufReader};

pub fn png_to_webp(input_path: &str, output_path: &str, qualities: &[u32]) -> anyhow::Result<()> {
    // 加载 PNG 图像
    let file = File::open(input_path).context(format!("无法打开输入文件: {}", input_path))?;
    let reader = BufReader::new(file);
    let img = image::load(reader, ImageFormat::Png).context("无法加载 PNG 图像")?;

    for &quality in qualities {
        let quality = quality.clamp(1, 100) as u8; // 确保质量值在1-100范围内并转为u8
        let output_file_name = super::jpg::format_output_path(output_path, quality);
        
        // 确保输出目录存在
        if let Some(parent) = std::path::Path::new(&output_file_name).parent() {
            std::fs::create_dir_all(parent)
                .context(format!("无法创建输出目录: {}", parent.display()))?;
        }

        // 创建 WebP 编码器
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();
        let encoder = webp::Encoder::from_rgba(rgba_img.as_raw(), width, height);
        let webp_data = encoder.encode(quality as f32);
        
        // 保存 WebP 数据到文件
        std::fs::write(&output_file_name, &*webp_data)
            .context(format!("无法写入输出文件: {}", output_file_name))?;
    }
    Ok(())
}
