use clap::{Command, Arg};

fn main() {
    let matches = Command::new("Rget")
        .version("0.1.0")
        .author("Kalashnikov Vladislav")
        .about("wget clone written in Rust")
        .arg(
            Arg::new("URL")
                .required(true)
                .index(1)
                .help("url to download"),
        )
        .get_matches();

    let url = matches.get_one::<String>("URL").unwrap();
    println!("{}", url);
}