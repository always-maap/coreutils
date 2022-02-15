use clap::{App, Arg};
use std::{
    error::Error,
    io::{BufRead, BufReader},
};

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> AppResult<Config> {
    let cli = App::new("cat")
        .version("0.1.0")
        .author("Mohammad ali Ali panah")
        .about("cat written in rust")
        .arg(
            Arg::new("files")
                .multiple_values(true)
                .default_value("-")
                .help("files to read"),
        )
        .arg(
            Arg::new("number-lines")
                .short('n')
                .long("number")
                .help("number lines")
                .takes_value(false)
                .conflicts_with("number-nonblank"),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .help("number nonblank lines")
                .takes_value(false),
        )
        .get_matches();

    let files: Vec<_> = cli
        .values_of("files")
        .unwrap()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.to_string())
        .collect();

    Ok(Config {
        files,
        number_lines: cli.is_present("number-lines"),
        number_nonblank_lines: cli.is_present("number-nonblank"),
    })
}

pub fn run(config: Config) -> AppResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("failed to open {}: {}", filename, e),
            Ok(file) => {
                let mut last_num = 0;

                for (line_number, line) in file.lines().enumerate() {
                    let line_value = line?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_number + 1, line_value);
                    } else if config.number_nonblank_lines {
                        if !line_value.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line_value);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", &line_value);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filesname: &str) -> AppResult<Box<dyn BufRead>> {
    match filesname {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(filesname)?))),
    }
}
