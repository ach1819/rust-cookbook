use clap::{Arg, Command};
fn main() {
    argument_parsing();
}

fn argument_parsing() {
    let m = Command::new("My cookbook program")
        .version("0.1.0")
        .author("ACH")
        .about("Mastering rust")
        .arg(Arg::new("file").short('f').long("file").help("A cool file"))
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .help("Five less than youp favorite number"),
        )
        .get_matches();

    let default_file = String::from("input.txt");
    let my_file = m.get_one::<String>("file").unwrap_or(&default_file);
    println!("The file passed is: {}", my_file);

    let num = m.get_one::<String>("num");
    match num {
        None => println!("No idea what your favorite number is."),
        Some(n) => match n.parse::<i32>() {
            Ok(n_parsed) => println!("Your favorite number must be {}.", n_parsed + 5),
            Err(_) => println!("That's not a number! {}", n),
        },
    }
}
