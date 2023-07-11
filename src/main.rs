#![allow(dead_code)]

use std::error::Error;
use gumdrop::Options;

#[derive(Options, Debug)]
struct AppOptions {
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

    run(&opts).unwrap_or_else(|err| {
        println!("Error: {err}");
    });
}

fn run(opts: &AppOptions) -> Result<(), Box<dyn Error>> {
    println!("### {} in, {} out", opts.infiles.len(), opts.outfiles.len());
    
    Ok(())
}
