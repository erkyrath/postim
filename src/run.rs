use std::error::Error;

use crate::AppOptions;

use crate::script::parse::load_script;

pub fn run(opts: &AppOptions) -> Result<(), Box<dyn Error>> {
    for filename in &opts.script {
    	load_script(&filename)?;
    }
    
    Ok(())
}
