use clap::{Arg, Command};
use rand::prelude::Distribution;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Travis Aucoin")
        .about("head in rust")
        .arg(
            Arg::new("files")
                .default_value("-")
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .default_value("10")
                .help("number of lines to display"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .allow_negative_numbers(false)
                .help("number of bytes to display"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        lines: matches
            .get_one::<String>("lines")
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        bytes: matches.get_one::<String>("bytes").unwrap().as_str(),
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
