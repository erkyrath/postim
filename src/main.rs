#![allow(dead_code)]
#![allow(unused_imports)]

use gumdrop::Options;

mod run;
mod script;
mod exec;
mod pixel;
mod img;

#[derive(Options, Debug)]
pub struct AppOptions {
    #[options(free)]
    script: Vec<String>,

    #[options(help = "print help message")]
    help: bool,

    #[options(long="in", help = "input file")]
    infiles: Vec<String>,

    #[options(long="out", help = "output file")]
    outfiles: Vec<String>,
}

fn main() {
    let opts = AppOptions::parse_args_default_or_exit();

    if opts.script.len() == 0 {
        println!("Usage: postim [OPTIONS] script");
        std::process::exit(1);
    }

    run::run(&opts).unwrap_or_else(|err| {
        println!("Error: {err}");
    });
}

