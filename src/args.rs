
use crate::img::Img;
use crate::img::ppmio;
use crate::script::Script;
use crate::script::parse::load_script_file;
use crate::script::parse::load_script_text;
use crate::exec::except::ExecError;

pub enum Argument {
    ScriptArg(Script),
    ImageArg(Img<f32>),
}

pub fn parse_args(argls: &[String]) -> Result<Vec<Argument>, ExecError> {
    let mut args: Vec<Argument> = Vec::new();
    
    for arg in argls {
        if arg.ends_with(".ppm") {
            let u8img = ppmio::img_read(&arg)?;
            args.push(Argument::ImageArg(u8img.as_f32()));
        }
        else if arg.ends_with(".imp") {
            let script = load_script_file(&arg)?;
            args.push(Argument::ScriptArg(script));
        }
        else {
            let script = load_script_text(&arg)?;
            args.push(Argument::ScriptArg(script));
        }
    }

    Ok(args)
}
