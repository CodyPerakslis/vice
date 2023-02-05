use clap::{Command, Arg};
mod crud;

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
        .subcommand(
            Command::new("add")
                .about("Add a new vice")
                .arg(
                    Arg::new("name")
                        .required(true)
                )
        )
}


fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", add_matches)) => {
            let name = add_matches.get_one::<String>("name").unwrap();
            let new_vice = crud::Vice {
                name: name.to_string(),
            };
            println!("Adding new vice: {}", name);
            crud::write_file(new_vice).unwrap();
        },
        Some(("list", _)) => {
            println!("Listing all vices");
            let vice = crud::read_file().unwrap();
            println!("Current vice: {}", vice.name);
        },
        _ => unreachable!(),
    }
}
