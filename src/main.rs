mod commands;
mod png;
mod webp;
mod jpeg;

fn main() {
    let matches = commands::build_command()
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
    commands::execute(matches);
}
