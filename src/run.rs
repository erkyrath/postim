use std::error::Error;

use crate::AppOptions;

use crate::script::Script;
use crate::script::parse::load_script;

pub fn run(opts: &AppOptions) -> Result<(), Box<dyn Error>> {

    let iter = opts.script.iter().map(|filename| load_script(&filename));
    let scripts: Vec<Script> = iter.collect::<Result<Vec<_>, _>>()?;
    
    println!("### loaded {} scripts", scripts.len());
    
    Ok(())
}
