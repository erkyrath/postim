use std::error::Error;

use crate::AppOptions;

use crate::script::Script;
use crate::script::parse::load_script;

pub fn run(opts: &AppOptions) -> Result<(), Box<dyn Error>> {
    let mut scripts: Vec<Script> = Vec::new();
    
    for filename in &opts.script {
    	let script = load_script(&filename)?;
	scripts.push(script);
    }
    
    Ok(())
}
