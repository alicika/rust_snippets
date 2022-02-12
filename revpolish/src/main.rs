use env;
use clap::{App, Arg};

fn main() {
    let matches = App::new("My RPN program")
        .version("0.1.0")
        .author("Me")
        .about("sample RPN calculator")
        .arg(
            Arg::new("formula_file")
                .about("Formula written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}