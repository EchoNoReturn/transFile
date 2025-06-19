mod commands;
mod png;
mod webp;
mod jpeg;

fn main() {
    let matches = commands::build_command()
        .version("0.0.2")
        .author("YOYOJ")
        .about("一个简单易用的文件转换工具")
        .get_matches();
    commands::execute(matches);
}
