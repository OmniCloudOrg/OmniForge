use std::process::Command;
use std::io;

pub fn get_platform() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "darwin".to_string()
    } else {
        "linux".to_string()
    }
}

#[derive(Debug)]
pub struct InstallationStatus {
    pub node: bool,
    pub npm: bool,
    pub docker: bool,
    pub devcontainers: bool,
}
