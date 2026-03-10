use clap::Command;

mod commands;
mod services;

fn main() {
    let matches: clap::ArgMatches = Command::new("miyu")
        .subcommand(Command::new("scroll"))
        .subcommand(Command::new("sleep"))
        .get_matches();

    match matches.subcommand() {
        Some(("scroll", _)) => {
            commands::scroll::run();
        }

        Some(("sleep", _)) => {
            commands::sleep::run();
        }
        _ => {
            println!("Unknown command. Please try a again")
        }
    }
}
