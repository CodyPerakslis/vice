use clap::{Command};

fn cli() -> Command {
    Command::new("vice")
        .version("0.1.0")
        .author("Cody Perakslis")
        .about("A program to ween off vice over the course of a lifetime")
        .subcommand_required(true)
        .subcommand(
            Command::new("list")
                .about("List all vices")
        )
}


fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("list", _)) => {
            println!("Listing all vices");
        },
        _ => unreachable!(),
    }
}
