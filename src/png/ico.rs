use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::Context;

use ico::{IconDir, IconDirEntry, IconImage};

pub fn png_to_ico(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    // 加载 PNG 图像
    let file = File::open(input_path).context(format!("无法打开输入文件: {}", input_path))?;
    let mut reader = BufReader::new(file);
    let img = image::load(&mut reader, image::ImageFormat::Png).context("无法加载 PNG 图像")?;

    // 创建 ICO 图标目录
    let mut icon_dir = IconDir::new(ico::ResourceType::Icon);

    // 默认 ICO 支持的尺寸
    let sizes = vec![16, 24, 32, 48, 64, 128, 256];

    // 为每个尺寸添加图标
    for size in sizes {
        if size == 0 || size > 256 {
            continue; // ICO 格式通常限制在 256x256
        }

        // 调整图像大小
        let resized = img.resize_exact(size, size, image::imageops::Lanczos3);
        let rgba_image = resized.to_rgba8();

        // 创建 ICO 图像
        let icon_image = IconImage::from_rgba_data(size as u32, size as u32, rgba_image.into_raw());

        // 添加到图标目录
        icon_dir.add_entry(IconDirEntry::encode(&icon_image)?);
    }

    // 确保输出目录存在
    if let Some(parent) = Path::new(output_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 写入 ICO 文件
    let output_file =
        File::create(output_path).context(format!("无法创建输出文件: {}", output_path))?;
    let mut writer = std::io::BufWriter::new(output_file);
    icon_dir.write(&mut writer).context("写入 ICO 文件失败")?;
    
    Ok(())
}
