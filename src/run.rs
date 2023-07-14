use std::error::Error;

use crate::AppOptions;

use crate::script::Script;
use crate::script::parse::load_script;
use crate::exec::ExecContext;

pub fn run(opts: &AppOptions) -> Result<(), Box<dyn Error>> {

    let scripts: Vec<Script> = opts.script
        .iter()
        .map(|filename| load_script(&filename))
        .collect::<Result<Vec<_>, _>>()?;

    let mut ctx = ExecContext::new();

    ctx.loadargs(&opts.infiles)?;

    for script in &scripts {
        ctx.execute(&script)?;
    }

    println!("stack: {:?}", &ctx.stack());
    
    Ok(())
}
