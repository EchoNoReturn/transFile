use anyhow::Context;
use image::imageops::FilterType;

/// 一键使用 png 生成 apple APP 图标
/// # 参数
/// * `input_path` - 输入 PNG 文件路径
/// * `output_path` - 输出目录路径，图标将保存到该目录下
/// # 返回值
/// * `anyhow::Result<()>` - 操作结果
pub fn png_to_app_icon(
    input_path: &str,
    output_path: &str,
    filter_type: FilterType
) -> anyhow::Result<()> {
    // 加载 PNG 图像
    let file =
        std::fs::File::open(input_path).context(format!("无法打开输入文件: {}", input_path))?;
    let reader = std::io::BufReader::new(file);
    let img = image::load(reader, image::ImageFormat::Png).context("无法加载 PNG 图像")?;

    // 确保输出目录存在
    let output_dir = std::path::Path::new(output_path);
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)
            .context(format!("无法创建输出目录: {}", output_path))?;
    }
    let output_path = output_dir.join("icon");

    // 输出为不同尺寸的图标
    // 一次输出所有 apple 平台可能用到的所有尺寸
    let sizes = [
        20, 29, 40, 60, 76, 83, 1024, // iOS 和 iPad
        16, 32, 64, 128, 256, // macOS
        24, 48, // watchOS
        30, // tvOS
    ];
    for &size in &sizes {
        let resized_img = img.resize_exact(size, size, filter_type);
        let output_file_path = format!("{}-{}.png", output_path.to_string_lossy(), size);
        let output_file = std::fs::File::create(&output_file_path)
            .context(format!("无法创建输出文件: {}", output_file_path))?;
        resized_img
            .write_to(
                &mut std::io::BufWriter::new(output_file),
                image::ImageOutputFormat::Png,
            )
            .context(format!("写入 PNG 文件失败: {}", output_file_path))?;
    }

    Ok(())
}

pub fn get_filter_type(filter: &str) -> FilterType {
    match filter {
        "Nearest" => FilterType::Nearest,
        "Triangle" => FilterType::Triangle,
        "CatmullRom" => FilterType::CatmullRom,
        "Gaussian" => FilterType::Gaussian,
        "Lanczos3" => FilterType::Lanczos3,
        _ => FilterType::Lanczos3, // 默认使用 Lanczos3
    }
}