#![allow(dead_code)]

mod read_config;
mod scroller;
mod run_error;

use clap::{Command, Arg};
use read_config::read_config;
use run_error::RunError;

use std::fs::read_to_string;
use std::process::exit;

fn run() -> Result<(), RunError> {
    let args = Command::new("lux")
        .bin_name("lux")
        .about("Console utility for outputing colored text")
        .version("0.1.0")
        .author("Davud Magomedov")
        .arg(
            Arg::new("file")
                .required(true)
        )
        .get_matches();
    let file_name: &str = args.get_one::<String>("file").unwrap();
    let file_content: String = match read_to_string(file_name) {
        Ok(cnt) => cnt,
        Err(e) => return Err(RunError::File(e.to_string()))
    };
    let config = match read_config(
        match file_name.split('.').skip(1).next() {
            Some(n) => n,
            None => {
                println!("{}", file_content);
                return Ok(())
            }
        }
    ) {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("{}", file_content);
            return Ok(())
        }
    };
    let colorize_content = match luxlib::colorize(
        &file_content,
        config.main_changes(),
        config.special_funcs()
    ) {
        Ok(n) => n,
        Err(e) => return Err(RunError::Colorize(e))
    };

    println!(
        "{}",
        colorize_content
    );

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    };
}
