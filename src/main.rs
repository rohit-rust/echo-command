use clap::{App, Arg};

fn main() {
    let matches = App::new("echoc")
        .version("1.0.0")
        .author("rohit br <rohitbr@proton.me>")
        .about("echo in rust from scratch")
        .arg(
            Arg::with_name("Text")
                .takes_value(true)
                .required(false)
                .help("Takes Text As Input")
                .min_values(1),
        )
        .arg(
            Arg::with_name("ignore_newline")
                .takes_value(false)
                .required(false)
                .help(" do not print new line")
                .short("n"),
        )
        .get_matches();

    if matches.args.len() == 0 {
        println!("Echoc is ready")
    } else {
        let input_text = matches.value_of_lossy("Text").unwrap();
        let is_newline = matches.is_present("ignore_newline");
        print!("{} {}", input_text, if is_newline { " " } else { "\n" });
    }
}
