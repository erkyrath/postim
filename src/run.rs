use std::error::Error;

use crate::AppOptions;

use crate::args::parse_args;
use crate::args::Argument;
use crate::script::Script;
use crate::exec::ExecContext;

pub fn run(opts: &AppOptions) -> Result<(), Box<dyn Error>> {

    let args = parse_args(&opts.args)?;
    
    let mut ctx = ExecContext::new();

    for arg in args {
        match arg {
            Argument::ScriptArg(script) => { ctx.execute_script(&script)?; },
            Argument::ImageArg(img) => { ctx.push_img(img); },
        }
    }

    ctx.unloadargs(&opts.outfiles)?;

    if ctx.stack().len() != 0 {
        println!("stack: {:?}", &ctx.stack());
    }
    
    Ok(())
}
