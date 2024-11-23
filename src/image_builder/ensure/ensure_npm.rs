use super::common::get_platform;
use std::io;
use std::process::Command;

pub fn install_node_platform() -> io::Result<()> {
    let platform = get_platform();

    match platform.as_str() {
        "windows" => {
            println!("Installing Node.js using winget...");
            Command::new("winget")
                .args(["install", "OpenJS.NodeJS"])
                .status()?;
        }
        "darwin" => {
            println!("Installing Node.js using Homebrew...");
            Command::new("brew").args(["install", "node"]).status()?;
        }
        "linux" => {
            println!("Installing Node.js using package manager...");
            let apt_result = Command::new("apt")
                .args(["install", "-y", "nodejs", "npm"])
                .status();

            if apt_result.is_err() {
                let dnf_result = Command::new("dnf")
                    .args(["install", "-y", "nodejs", "npm"])
                    .status();

                if dnf_result.is_err() {
                    Command::new("pacman")
                        .args(["-S", "--noconfirm", "nodejs", "npm"])
                        .status()?;
                }
            }
        }
        _ => return Err(io::Error::new(io::ErrorKind::Other, "Unsupported platform")),
    }

    Ok(())
}
