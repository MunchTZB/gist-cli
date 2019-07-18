extern crate clap;
extern crate reqwest;
use clap::{Arg, App};

fn main() {
    let matches = App::new("gist-cli")
                        .version("0.0.1")
                        .author("MunchTZB <dev.tzb@gmail.com>")
                        .about("a github gist cli tool.")
                        .arg(Arg::with_name("INPUT_FILE")
                             .help("Set the target file")
                             .required(true)
                             .index(1))
                        .get_matches();

    let input_file = matches.value_of("INPUT_FILE").unwrap();
    let input_file = std::path::PathBuf::from(input_file);
    let content = std::fs::read_to_string(input_file).unwrap();
    let resp = reqwest::get("http://www.baidu.com").unwrap().text().unwrap();

    println!("{}", resp);
}
