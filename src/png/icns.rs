use anyhow::{Context, Result};
use icns::IconFamily;
use image::DynamicImage;
use std::{
    fs::{File, create_dir_all},
    io::{BufReader, BufWriter},
    path::Path,
};

pub fn png_to_icns(input_path: &str, output_path: &str, sizes: Vec<u32>) -> Result<()> {
    // 这里可以使用 icns crate 来处理 PNG 到 ICNS 的转换
    let file = File::open(&input_path).context(format!("无法打开输入文件: {}", input_path))?;
    let mut file_reader = BufReader::new(file);
    let img = image::load(&mut file_reader, image::ImageFormat::Png)
        .context(format!("无法加载 PNG 图像: {}", input_path))?;

    // 创建 ICNS 图标族
    let mut icon_family = IconFamily::new();

    // 添加指定大小的图标
    // 如果没有指定大小，则使用默认的 16, 32, 64, 128, 256, 512, 1024
    let default_sizes = vec![16, 32, 64, 128, 256, 512, 1024];
    let mut output_paths: Vec<String> = vec![];
    let sizes = if sizes.is_empty() {
        default_sizes
    } else {
        sizes
    };
    for size in sizes {
        if size == 0 {
            continue; // 跳过无效的大小
        }
        add_icon_size(&mut icon_family, &img, size)?;
        // 构建输出路径
        // 去除 output_path 的后缀
        let output_path_obj = Path::new(output_path);
        let (parent, base_output) = if output_path.ends_with('/') {
            (output_path_obj, "icon".into())
        } else {
            (
                output_path_obj.parent().unwrap_or_else(|| Path::new("")),
                match output_path_obj.file_stem() {
                    Some(stem) => stem.to_string_lossy(),
                    None => output_path.into(),
                },
            )
        };
        let output_file = parent.join(format!("{}-{}.icns", base_output, size));
        output_paths.push(output_file.to_string_lossy().into_owned());
    }

    // 确保输出目录存在
    if let Some(parent) = Path::new(output_path).parent() {
        create_dir_all(parent)?;
    }

    // 写入 ICNS 文件
    let output_file = File::create(&output_path)
        .context(format!("Could not create output file: {}", output_path))?;
    let mut writer = BufWriter::new(output_file);
    icon_family
        .write(&mut writer)
        .context("Failed to write ICNS file")?;
    Ok(())
}

fn add_icon_size(icon_family: &mut IconFamily, img: &DynamicImage, size: u32) -> Result<()> {
    // 调整图像大小
    let resized = img.resize_exact(size, size, image::imageops::Lanczos3);

    // 将图像添加到图标族
    let rgba_image = resized.to_rgba8();
    let icns_image = icns::Image::from_data(
        icns::PixelFormat::RGBA,
        size,
        size,
        rgba_image.as_raw().clone(),
    )?;
    icon_family
        .add_icon(&icns_image)
        .context(format!("Failed to add {}x{} icon", size, size))?;

    Ok(())
}
