use clap::{ArgMatches, Command};

pub fn build_command() -> Command {
    Command::new("transFile")
        .subcommand(crate::png::build())
        .subcommand(crate::webp::build())
        .subcommand(crate::jpeg::build())
}

pub fn execute(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("png", sub_matches)) => {
            if let Err(e) = crate::png::execute(sub_matches.clone()) {
                eprintln!("png 命令执行出错: {}", e);
            }
        },
        Some(("webp", sub_matches)) => {
            if let Err(e) = crate::webp::execute(sub_matches.clone()) {
                eprintln!("webp 命令执行出错: {}", e);
            }
        },
        Some(("jpeg", sub_matches)) => {
            if let Err(e) = crate::jpeg::execute(sub_matches.clone()) {
                eprintln!("jpeg 命令执行出错: {}", e);
            }
        },
        _ => eprintln!("未知的命令"),
    }
}
