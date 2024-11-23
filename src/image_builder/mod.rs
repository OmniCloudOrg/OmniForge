// main.rs
mod ensure;
mod image_gen;

use anyhow::Context;
use anyhow::Result;
use ensure::common;
use ensure::ensure_devcontainers_cli;
use ensure::ensure_docker;
use ensure::ensure_npm;
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use anyhow::anyhow;
#[derive(Debug, Serialize, Deserialize)]
pub struct DevContainer {
    pub name: String,
    pub image: String,
    pub features: HashMap<String, Option<FeatureData>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeatureData {
    pub version: Option<String>,
}

const DOCKER_REGISTRY: &str = "localhost:5000";

pub fn build_devcontainer(devcontainer_path: &Path) -> Result<String> {
    println!("Final path: {}", devcontainer_path.display());

    // Read and verify the devcontainer.json content
    let content = fs
        ::read_to_string(devcontainer_path)
        .context("failed to read the path to the dev container")?;
    let config: Value = serde_json
        ::from_str(&content)
        .context("Failed to serialize 'App/.devcontainer/devcontainer.json'")?;

    // Generate image name from devcontainer.json configuration
    let image_name = generate_image_name(&config, devcontainer_path).context(
        "Failed to get image name"
    )?;

    // Get the workspace folder (two levels up from devcontainer.json)
    let workspace_folder = devcontainer_path
        .parent() // Gets .devcontainer folder
        .and_then(|p| p.parent()) // Gets the workspace folder (App)
        .ok_or_else(||
            io::Error::new(io::ErrorKind::Other, "Failed to determine workspace folder")
        )?;

    println!("Path {}", workspace_folder.display());

    // Use workspace folder path for the CLI command
    let output = Command::new("devcontainer")
        .args([
            "build",
            "--workspace-folder",
            workspace_folder.to_str().unwrap(), // Pass the workspace folder, not the devcontainer.json path
            "--image-name",
            &image_name,
        ])
        .output()?;

    if !output.status.success() {
        return Err(
            io::Error::new(io::ErrorKind::Other, String::from_utf8_lossy(&output.stderr)).into()
        );
    }

    // Tag the image for the local Docker registry
    let tagged_image = format!("{}/{}", DOCKER_REGISTRY, image_name);
    let tag_output = Command::new("docker").args(["tag", &image_name, &tagged_image]).output()?;

    if !tag_output.status.success() {
        return Err(
            io::Error::new(io::ErrorKind::Other, String::from_utf8_lossy(&tag_output.stderr)).into()
        );
    }

    // Push the image to the local Docker registry
    let push_output = Command::new("docker").args(["push", &tagged_image]).output()?;

    if !push_output.status.success() {
        return Err(
            io::Error
                ::new(io::ErrorKind::Other, String::from_utf8_lossy(&push_output.stderr))
                .into()
        );
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
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "Failed to determine container name from path")
            })?;
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
        .map(|c| {
            match c {
                'a'..='z' | '0'..='9' | '.' | '_' | '-' => c,
                _ => '-',
            }
        })
        .collect()
}

// Example usage in main:
pub fn scan_and_build(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("Failed to locate application build path"));
    }
    image_gen::gen_devcontainer(path.to_str().unwrap());
    let status = ensure::ensure_installations().context("Failed to enture installation")?;
    println!("Installation status: {:?}", status);

    println!("Building devcontainer image...");

    let dev_ctr_json_str = format!("{}/.devcontainer/devcontainer.json", path.to_str().unwrap());
    let dev_ctr_json_path: &Path = Path::new(&dev_ctr_json_str);

    match build_devcontainer(dev_ctr_json_path) {
        Ok(image) => println!("Built container image: {}", image),
        Err(e) => eprintln!("Failed to build container: {}", e),
    }

    Ok(())
}
