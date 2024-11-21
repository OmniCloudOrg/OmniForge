use std::process::Command;
use std::io;

pub fn install_devcontainers() -> io::Result<()> {
    Command::new("npm")
        .args(["install", "-g", "@devcontainers/cli"])
        .status()?;
    
    Ok(())
}