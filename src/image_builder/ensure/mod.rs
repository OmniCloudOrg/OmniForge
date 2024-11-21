use super::common::InstallationStatus;
use super::ensure_npm::install_node_platform;
use super::ensure_docker::install_docker_platform;
use super::ensure_devcontainers_cli::install_devcontainers;
use std::process::Command;
use std::io;

pub mod common;
pub mod ensure_npm;
pub mod ensure_docker;
pub mod ensure_devcontainers_cli;

pub fn ensure_installations() -> io::Result<InstallationStatus> {
    let mut status = InstallationStatus {
        node: false,
        npm: false,
        docker: false,
        devcontainers: false,
    };

    // Check Node.js
    match Command::new("node").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("Node.js is installed: {}", String::from_utf8_lossy(&output.stdout));
            status.node = true;
        },
        _ => {
            println!("Installing Node.js...");
            install_node_platform()?;
            status.node = true;
        }
    }

    // Check NPM
    match Command::new("npm").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("NPM is installed: {}", String::from_utf8_lossy(&output.stdout));
            status.npm = true;
        },
        _ => {
            println!("NPM not found. Attempting to install NPM...");
            Command::new("npm").arg("install").arg("-g").arg("npm").status()?;
            match Command::new("npm").arg("--version").output() {
                Ok(output) if output.status.success() => {
                    println!("NPM is installed: {}", String::from_utf8_lossy(&output.stdout));
                    status.npm = true;
                },
                _ => {
                    return Err(io::Error::new(io::ErrorKind::NotFound, "NPM installation failed"));
                }
            }
        }
    }

    // Check Docker
    match Command::new("docker").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("Docker is installed: {}", String::from_utf8_lossy(&output.stdout));
            status.docker = true;
        },
        _ => {
            println!("Installing Docker...");
            install_docker_platform()?;
            status.docker = true;
        }
    }

    // Check Dev Containers CLI
    match Command::new("devcontainer").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("Dev Containers CLI is installed: {}", String::from_utf8_lossy(&output.stdout));
            status.devcontainers = true;
        },
        _ => {
            println!("Installing Dev Containers CLI...");
            install_devcontainers()?;
            status.devcontainers = true;
        }
    }

    Ok(status)
}