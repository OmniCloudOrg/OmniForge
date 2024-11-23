use std::io;
use std::process::Command;

pub fn install_devcontainers() -> io::Result<()> {
    Command::new("npm")
        .args(["install", "-g", "@devcontainers/cli"])
        .status()?;

    Ok(())
}
