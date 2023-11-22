// execute.rs
use anyhow::Result;

pub trait Execute {
    fn execute(&self) -> Result<()>;
}
