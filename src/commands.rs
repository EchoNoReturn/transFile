use clap::{ArgMatches, Command};

pub fn build_command() -> Command {
    Command::new("transFile")
    .subcommand(crate::png::build())
}

pub fn execute(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("png", sub_matches)) => {
            let args = crate::png::PngArgs {
                target_format: sub_matches.get_one::<String>("target").unwrap().clone(),
                size: *sub_matches.get_one::<u32>("size").unwrap(),
                input_path: sub_matches.get_one::<String>("input").unwrap().clone(),
                output_path: sub_matches.get_one::<String>("output").unwrap().clone(),
            };
            if let Err(e) = crate::png::execute(args) {
                eprintln!("Error executing PNG command: {}", e);
            }
        },
        _ => eprintln!("Unknown command or missing subcommand"),
    }
}