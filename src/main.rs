mod commands;
mod png;

fn main() {
    let matches = commands::build_command()
        .version("0.0.1")
        .author("YOYOJ")
        .about("一个简单易用的文件处理工具")
        .get_matches();
    commands::execute(matches);
}
