use std::error::Error;

use crate::AppOptions;

pub fn run(opts: &AppOptions) -> Result<(), Box<dyn Error>> {
    println!("### {} in, {} out", opts.infiles.len(), opts.outfiles.len());
    
    Ok(())
}
