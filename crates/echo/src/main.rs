use clap::{App, Arg};

fn main() {
    let cli = App::new("echo")
        .version("0.1.0")
        .author("Mohammad ali Ali panah")
        .about("echo written in rust")
        .arg(
            Arg::new("text")
                .help("Text to echo")
                .min_values(1)
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print a newline at the end of the output")
                .takes_value(false),
        )
        .get_matches();

    let text: Vec<_> = cli.values_of("text").unwrap().collect();
    let omit_newline = cli.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
