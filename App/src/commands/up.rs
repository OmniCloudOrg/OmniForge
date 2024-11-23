use crate::models::ComponentStatus;
use crate::ui::PremiumUI;
use anyhow::{Context, Result};
use console::style;
use dialoguer::{Confirm, Input, Select};
use flate2::write::GzEncoder;
use flate2::Compression;
use reqwest::multipart::{Form, Part};
use std::{fs::File, io::Write, path::Path};
use std::{thread, time::Duration};
use tabled::Table;
use tar::Builder;
use tokio::fs;

impl PremiumUI {
    pub async fn deploy_interactive(&self) -> Result<()> {
        // Get project path
        let project_path: String = Input::with_theme(&self.theme)
            .with_prompt("Enter project path")
            .default(".".into())
            .interact_text()?;

        // Validate project path
        if !Path::new(&project_path).exists() {
            println!("{}", style("Error: Project path does not exist.").red());
            return Ok(());
        }

        // Environment selection
        let environments = vec!["Development", "Staging", "Production"];
        let env_selection = Select::with_theme(&self.theme)
            .with_prompt("Select deployment environment")
            .items(&environments)
            .default(0)
            .interact()?;

        // Production confirmation
        if environments[env_selection] == "Production" {
            let confirm = Confirm::with_theme(&self.theme)
                .with_prompt("⚠️  You're deploying to production. Are you sure?")
                .default(false)
                .interact()?;
            if !confirm {
                println!("{}", style("Deployment cancelled.").yellow());
                return Ok(());
            }
        }

        println!("\n{}", style("🚀 Initializing deployment...").cyan().bold());

        // Create tarball
        let tarball_path = self.create_tarball(&project_path).await
            .context("Failed to create tarball")?;

        // Upload tarball
        self.upload_tarball(&tarball_path, environments[env_selection]).await
            .context("Failed to upload tarball")?;

        // Clean up tarball
        fs::remove_file(&tarball_path).await
            .context("Failed to clean up tarball")?;

        // Clean up tarball
        fs::remove_file(&tarball_path).await
            .context("Failed to clean up tarball")?;

        let steps = [
            ("Analyzing project", 20),
            ("Building containers", 40),
            ("Pushing to registry", 30),
            ("Configuring services", 25),
            ("Starting components", 35),
        ];

        for (step, duration) in steps.iter() {
            let pb = self.create_progress_bar(*duration, step);
            for i in 0..*duration {
                pb.inc(1);
                thread::sleep(Duration::from_millis(100));

                match i {
                    5 => pb.set_message(format!("{} (scanning dependencies)", step)),
                    15 => pb.set_message(format!("{} (optimizing)", step)),
                    25 => pb.set_message(format!("{} (finalizing)", step)),
                    _ => {}
                }
            }
            pb.finish_with_message(format!("{} ✓", step));
        }

        let status_table = Table::new(vec![
            ComponentStatus {
                name: "Web Frontend".into(),
                status: "Running".into(),
                replicas: "3/3".into(),
                cpu: "150m".into(),
                memory: "256Mi".into(),
            },
            ComponentStatus {
                name: "API Backend".into(),
                status: "Running".into(),
                replicas: "2/2".into(),
                cpu: "200m".into(),
                memory: "512Mi".into(),
            },
            ComponentStatus {
                name: "Database".into(),
                status: "Running".into(),
                replicas: "1/1".into(),
                cpu: "500m".into(),
                memory: "1Gi".into(),
            },
        ])
        .to_string();

        println!("\n{}", style("📊 Deployment Status").cyan().bold());
        println!("{}", status_table);
        println!("\n{}", style("🌍 Application Endpoints").cyan().bold());
        println!("Frontend: {}", style("https://app.example.com").green());
        println!("API:      {}", style("https://api.example.com").green());
        println!("Metrics:  {}", style("https://metrics.example.com").green());
        println!("\n{}", style("✨ Deployment completed successfully!").green().bold());
        println!("{}", style("Run 'omni status' to monitor your deployment.").dim());
        Ok(())
    }

    async fn create_tarball(&self, project_path: &str) -> Result<String> {
        // Get the absolute path and resolve "." to the actual directory name
        let absolute_path = fs::canonicalize(project_path)
            .await
            .context("Failed to resolve project path")?;
        
        // Get the directory name - use the last component of the path
        let project_name = absolute_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_else(|| {
                // Fallback to using the last component of the path if we're at root
                absolute_path
                    .components()
                    .last()
                    .and_then(|comp| comp.as_os_str().to_str())
                    .unwrap_or("project")
            });
    
        // Create tarball filename
        let tar_gz_path = format!("{}.tar.gz", project_name);
    
        // Create the tarball file
        let tar_gz = File::create(&tar_gz_path)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(enc);
    
        // Add the directory contents to the tarball
        tar.append_dir_all(".", project_path)
            .context("Failed to add directory contents to tarball")?;
    
        // Finish creating the tarball
        tar.into_inner()
            .context("Failed to finish tarball creation")?
            .finish()
            .context("Failed to finish compression")?;
    
        Ok(tar_gz_path)
    }

    async fn upload_tarball(&self, tarball_path: &str, environment: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let api_url = match environment {
            "Production" => "http://localhost:3030/upload",
            "Staging" => "https://staging-api.example.com/v1/deploy",
            _ => "http://localhost:3030/upload",
        };

        let file_content = fs::read(tarball_path).await?;
        let part = Part::bytes(file_content)
            .file_name(tarball_path.to_string())
            .mime_str("application/gzip")?;

        let form = Form::new()
            .part("file", part)
            .text("environment", environment.to_string());

        let pb = self.create_progress_bar(100, "Uploading project");
        
        let response = client
            .post(api_url)
//            .bearer_auth(&self.config.api_token)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            pb.abandon_with_message("Upload failed!");
            anyhow::bail!("Failed to upload tarball: {}", response.status());
        }

        pb.finish_with_message("Upload completed successfully ✓");
        Ok(())
    }
}