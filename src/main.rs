extern crate clap;
extern crate reqwest;
extern crate dirs;
use clap::{Arg, App};
use std::collections::HashMap;
use reqwest::Client;

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

    let mut rc_path = std::path::PathBuf::from(dirs::home_dir().unwrap());
    rc_path.push(".gist-cli");

    let rc = std::fs::read_to_string(rc_path).unwrap();
    let rc = rc.trim();

    let input_file = matches.value_of("INPUT_FILE").unwrap();
    let input_file = std::path::PathBuf::from(input_file);
    let content = std::fs::read_to_string(input_file).unwrap();


    let clients = Client::new();
    let mut resp = clients.get("https://api.github.com/users/codertocat")
                        .header("Authorization", format!("token {token}", token = rc))
                        .send().unwrap();
    for (key, value) in resp.headers().iter() {
        println!("{:?}: {:?}", key, value);
    }

    println!("{:#?}", resp);
}
