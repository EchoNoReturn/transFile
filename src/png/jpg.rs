use std::{fs::File, io::BufReader, path::Path};
use anyhow::Context;
use image::{ImageFormat, ImageOutputFormat};

/// 将 PNG 转换为 JPEG
/// 
/// # 参数
/// * `input_path` - 输入 PNG 文件路径
/// * `output_path` - 输出 JPEG 文件路径
/// * `qualities` - JPEG 质量列表，范围 1-100，默认 100
/// 
/// # 返回值
/// * `anyhow::Result<()>` - 操作结果
pub fn png_to_jpeg(input_path: &str, output_path: &str, qualities: &[u32]) -> anyhow::Result<()> {
    // 加载 PNG 图像
    let file = File::open(input_path)
        .context(format!("无法打开输入文件: {}", input_path))?;
    let reader = BufReader::new(file);
    let img = image::load(reader, ImageFormat::Png)
        .context("无法加载 PNG 图像")?;

    // 如果没有指定质量或只有一个质量值
    if qualities.is_empty() || qualities.len() == 1 {
        // 确保输出目录存在
        if let Some(parent) = Path::new(output_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // 使用默认或指定的质量值
        let quality = qualities.first().copied().unwrap_or(100);
        let quality = quality.clamp(1, 100) as u8; // 确保在合理范围内并转为u8
        
        // 转换并保存为 JPEG
        let output_file = File::create(output_path)
            .context(format!("无法创建输出文件: {}", output_path))?;
        img.write_to(&mut std::io::BufWriter::new(output_file), ImageOutputFormat::Jpeg(quality))
            .context("写入 JPEG 文件失败")?;
            
        return Ok(());
    }

    // 处理每个指定质量值
    for &quality in qualities {
        if quality == 0 || quality > 100 {
            println!("警告: 忽略无效的质量值 {}, 应在1-100范围内", quality);
            continue;
        }
        
        let quality = quality.clamp(1, 100) as u8; // 确保质量值在1-100范围内并转为u8

        // 构建输出路径，添加质量后缀
        let output_file_name = format_output_path(output_path, quality, "jpg");

        // 确保输出目录存在
        if let Some(parent) = Path::new(&output_file_name).parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // 转换并保存为 JPEG
        let output_file = File::create(&output_file_name)
            .context(format!("无法创建输出文件: {}", output_file_name))?;
        img.write_to(
            &mut std::io::BufWriter::new(output_file), 
            ImageOutputFormat::Jpeg(quality)
        ).context("写入 JPEG 文件失败")?;
    }
    
    Ok(())
}

// 格式化输出路径，处理质量后缀
pub fn format_output_path(output_path: &str, quality: u8, suffix: &str) -> String {
    // 检查输出路径是否以 / 结尾
    if output_path.ends_with('/') {
        return format!("{}image-q{}.jpg", output_path, quality);
    }
    
    // 获取基本路径（不含扩展名）和目录
    let path = Path::new(output_path);
    let stem = path.file_stem()
        .map_or("image", |s| s.to_str().unwrap_or("image"));
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    
    // 构建新路径 - 添加质量后缀
    parent.join(format!("{}-q{}.{}", stem, quality, suffix))
        .to_string_lossy()
        .into_owned()
}