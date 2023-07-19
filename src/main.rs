#![allow(dead_code)]
#![allow(unused_imports)]

use gumdrop::Options;

mod run;
mod args;
mod script;
mod exec;
mod img;

#[derive(Options, Debug)]
pub struct AppOptions {
    #[options(free)]
    args: Vec<String>,

    #[options(help = "print help message")]
    help: bool,

    #[options(long="out", help = "output file")]
    outfiles: Vec<String>,
}

fn main() {
    let opts = AppOptions::parse_args_default_or_exit();

    run::run(&opts).unwrap_or_else(|err| {
        println!("Error: {err}");
    });
}

