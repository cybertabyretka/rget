use clap::{Command, Arg};
use indicatif::{ProgressBar, ProgressStyle};

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

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = if quiet_mode {
        ProgressBar::hidden()
    } else {
        match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        }
    };

    bar.set_message(msg.to_string());

    if let Some(_) = length {
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/.blue}] {bytes}/{total_bytes} eta: {eta}")
                .unwrap()
                .progress_chars("=> "),
        );
    } else {
        bar.set_style(ProgressStyle::default_spinner());
    }

    bar
}