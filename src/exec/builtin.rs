use crate::exec::ExecContext;
use crate::exec::except::ExecError;

impl ExecContext {
    pub fn execute_builtin(&mut self, tok: &str) -> Result<(), ExecError> {
        let msg = format!("name not known: {:?}", tok);
        Err(ExecError::new(&msg))
    }
}
