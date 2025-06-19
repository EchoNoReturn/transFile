# transFile

一个简单易用的图像格式转换工具，支持PNG、JPEG、WebP等格式之间的相互转换。

## 功能特点

- 支持多种图像格式：PNG、JPEG、ICNS、ICO、WebP等
- 支持调整输出质量（对JPEG、WebP）
- 支持生成多种尺寸的ICNS和ICO图标
- 命令行操作简单直观
- 批量处理能力（支持生成多种尺寸或多种质量级别的输出）

## 安装

### 直接下载

从 [GitHub Release页面](https://github.com/EchoNoReturn/transFile/releases/latest) 下载适合您平台的预编译可执行文件。

1. 访问 https://github.com/EchoNoReturn/transFile/releases/latest
2. 下载对应您操作系统的可执行文件
3. 赋予执行权限（如需要）：`chmod +x transFile`
4. 移动到系统路径（可选）：`sudo mv transFile /usr/local/bin/`

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/EchoNoReturn/transFile.git
cd transfile

# 编译
cargo build --release

# 运行
cargo run -- <命令> [参数]
```

## 使用说明

transFile提供了三个主要子命令：`png`、`jpeg`和`webp`，分别用于处理不同格式的源图像。

### PNG 子命令

用于将PNG格式转换为其他格式，包括ICNS、ICO、JPEG和WebP。

#### 基本语法

```bash
transFile png -t <目标格式> [可选参数] <输入文件路径> <输出文件路径>
```

#### 参数说明

- `-t, --to`：指定目标格式，必需。支持的格式有：`icns`、`ico`、`jpeg`、`webp`、`bmp`
- `-s, --size`：指定输出图像尺寸，仅用于ICNS和部分格式。可以用逗号分隔多个值，例如：`16,32,64`
  - 默认值：`16,32,64,128,256`
  - ICNS支持的尺寸：16, 32, 64, 128, 256, 512, 1024
  - ICO自动支持常用尺寸，此参数对ICO无效
- `-q, --quality`：指定JPEG或WebP的质量，范围1-100。可以用逗号分隔多个值，例如：`75,85,95`
  - 默认值：`100`（最高质量）

#### 示例

1. **将PNG转换为ICNS：**
   ```bash
   # 生成默认尺寸的ICNS图标
   transFile png -t icns input.png output.icns
   
   # 生成指定尺寸的ICNS图标
   transFile png -t icns -s 16,32,128,256 input.png output.icns
   
   # 输出到文件夹（会自动生成带尺寸后缀的文件）
   transFile png -t icns -s 128,256,512 input.png output/
   ```

2. **将PNG转换为ICO：**
   ```bash
   # 生成ICO图标（自动包含多个标准尺寸）
   transFile png -t ico input.png output.ico
   ```

3. **将PNG转换为JPEG：**
   ```bash
   # 使用最高质量
   transFile png -t jpeg input.png output.jpg
   
   # 指定单一质量
   transFile png -t jpeg -q 85 input.png output.jpg
   
   # 生成多个不同质量的JPEG（会自动添加质量后缀）
   transFile png -t jpeg -q 50,75,90 input.png output/
   ```

4. **将PNG转换为WebP：**
   ```bash
   # 使用最高质量
   transFile png -t webp input.png output.webp
   
   # 指定质量
   transFile png -t webp -q 80 input.png output.webp
   ```

### JPEG 子命令

用于将JPEG格式转换为PNG、WebP或BMP等格式。

#### 基本语法

```bash
transFile jpeg [可选参数] <输入文件路径> <输出文件路径>
```

#### 参数说明

- `-t, --to`：指定目标格式，默认为`png`。支持的格式有：`png`、`webp`、`bmp`

> 注：0.0.2 版本仅支持 png 作为目标格式。其他格式的支持将在未来逐步实现。

#### 示例

1. **将JPEG转换为PNG：**
   ```bash
   # 默认转为PNG
   transFile jpeg input.jpg output.png
   
   # 显式指定PNG格式
   transFile jpeg -t png input.jpg output.png
   ```

### WebP 子命令

用于将WebP格式转换为其他格式（目前仅支持PNG）。

#### 基本语法

```bash
transFile webp [可选参数] <输入文件路径> <输出文件路径>
```

#### 参数说明

- `-t, --to`：指定目标格式，默认为`png`
- `-q, --quality`：指定生成的PNG质量，范围1-100，默认为100

#### 示例

1. **将WebP转换为PNG：**
   ```bash
   # 默认转为PNG
   transFile webp input.webp output.png
   
   # 显式指定PNG格式和质量
   transFile webp -t png -q 90 input.webp output.png
   ```

## 提示

- 当输出路径以`/`结尾时，会被视为目录，并使用自动生成的文件名
- 生成多个尺寸或质量时，输出文件名会自动添加相应的后缀
- ICNS文件会生成每个尺寸独立的文件，例如`icon-16.icns`、`icon-32.icns`等
- JPEG质量输出会生成带质量标记的文件，例如`image-q75.jpg`、`image-q90.jpg`等

## 许可证

[LICENSE](LICENSE)
