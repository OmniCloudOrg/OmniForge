pub mod image_gen;
pub mod deployment;
pub mod autoscaler;
pub mod api;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    deployment::main().context("Failed to start deployment")?;
    let file_types = image_gen::try_compile("/users/chance")?;
    Ok(())
}