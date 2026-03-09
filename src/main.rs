use clap::Command;

mod commands;

fn main() {
    let matches: clap::ArgMatches = Command::new("miyu")
        .subcommand(Command::new("scroll"))
        .get_matches();

    match matches.subcommand() {
        Some(("scroll", _)) => {
            commands::scroll::run();
        }
        _ => {
            println!("Unknown command. Please try a again")
        }
    }
}
