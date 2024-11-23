use super::common::get_platform;
use std::io::{self, Write};
use std::process::Command;

pub fn install_docker_platform() -> io::Result<()> {
    let platform = get_platform();

    match platform.as_str() {
        "windows" => {
            println!("Installing Docker Desktop for Windows...");
            Command::new("winget")
                .args(["install", "Docker.DockerDesktop"])
                .status()?;
        }
        "darwin" => {
            println!("Installing Docker Desktop for Mac...");
            Command::new("brew")
                .args(["install", "--cask", "docker"])
                .status()?;
        }
        "linux" => {
            let curl_output = Command::new("curl")
                .args(["-fsSL", "https://download.docker.com/linux/ubuntu/gpg"])
                .output()?;

            let mut gpg = Command::new("gpg")
                .args([
                    "--dearmor",
                    "-o",
                    "/usr/share/keyrings/docker-archive-keyring.gpg",
                ])
                .stdin(std::process::Stdio::piped())
                .spawn()?;

            gpg.stdin.as_mut().unwrap().write_all(&curl_output.stdout)?;
            gpg.wait()?;

            let release = Command::new("lsb_release").arg("-cs").output()?;
            let codename = String::from_utf8_lossy(&release.stdout).trim().to_string();

            let repo = format!(
                "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] \
                https://download.docker.com/linux/ubuntu {} stable",
                codename
            );

            std::fs::write("/etc/apt/sources.list.d/docker.list", repo)?;

            Command::new("apt").args(["update"]).status()?;

            Command::new("apt")
                .args([
                    "install",
                    "-y",
                    "docker-ce",
                    "docker-ce-cli",
                    "containerd.io",
                ])
                .status()?;
        }
        _ => return Err(io::Error::new(io::ErrorKind::Other, "Unsupported platform")),
    }

    Ok(())
}
