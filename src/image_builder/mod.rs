// main.rs
mod ensure;
mod image_gen;

use std::collections::HashMap;
use ensure::common;
use ensure::ensure_npm;
use ensure::ensure_docker;
use ensure::ensure_devcontainers_cli;
use std::io;
use std::path::Path;
use std::process::Command;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug,Serialize,Deserialize)]
pub struct DevContainer {
    pub name: String,
    pub image: String,
    pub features: HashMap<String,Option<FeatureData>>,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct FeatureData {
    pub version: Option<String>
}

const DOCKER_REGISTRY: &str = "localhost:5000";

pub fn build_devcontainer(devcontainer_path: &Path) -> io::Result<String> {
    let content = fs::read_to_string(devcontainer_path)?;
    let config: Value = serde_json::from_str(&content)?;
    
    // Generate image name from devcontainer.json configuration
    let image_name = generate_image_name(&config, devcontainer_path)?;
    
    // Build using devcontainer CLI
    let output = Command::new("devcontainer")
        .args([
            "build",
            "--workspace-folder",
            devcontainer_path.parent().unwrap().to_str().unwrap(),
            "--image-name",
            &image_name
        ])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Tag the image for the local Docker registry
    let tagged_image = format!("{}/{}", DOCKER_REGISTRY, image_name);
    let tag_output = Command::new("docker")
        .args(["tag", &image_name, &tagged_image])
        .output()?;

    if !tag_output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&tag_output.stderr)
        ));
    }

    // Push the image to the local Docker registry
    let push_output = Command::new("docker")
        .args(["push", &tagged_image])
        .output()?;

    if !push_output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&push_output.stderr)
        ));
    }

    Ok(tagged_image)
}

fn generate_image_name(config: &Value, devcontainer_path: &Path) -> io::Result<String> {
    // Try to get name from devcontainer.json configuration
    let name = if let Some(name) = config.get("name").and_then(|n| n.as_str()) {
        // Sanitize the name to be docker-compatible
        sanitize_docker_name(name)
    } else {
        // Fallback to parent directory name if no name in config
        let dir_name = devcontainer_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::Other,
                "Failed to determine container name from path"
            ))?;
        sanitize_docker_name(dir_name)
    };

    // Get optional version from config
    let version = config
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("latest");

    Ok(format!("{}-devcontainer:{}", name, version))
}

fn sanitize_docker_name(name: &str) -> String {
    // Docker image names must be lowercase and can only contain:
    // lowercase letters, digits, dots, underscores, or hyphens
    name.to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' | '.' | '_' | '-' => c,
            _ => '-'
        })
        .collect()
}

// Example usage in main:
pub fn main() -> io::Result<()> {
    image_gen::gen_devcontainer();
    let status = ensure::ensure_installations()?;
    println!("Installation status: {:?}", status);
    

    match build_devcontainer(Path::new("./devcontainer2.json")) {
        Ok(image) => println!("Built container image: {}", image),
        Err(e) => eprintln!("Failed to build container: {}", e)
    }
    
    Ok(())
}