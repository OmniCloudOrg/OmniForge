use crate::ui::PremiumUI;
use anyhow::Result;
use console::style;
use dialoguer::{Select, Confirm};
use std::{thread, time::Duration};

impl PremiumUI {
    pub async fn rollback_interactive(&self) -> Result<()> {
        let versions = vec![
            "v1.2.3 (Current)",
            "v1.2.2 (2 days ago)",
            "v1.2.1 (5 days ago)",
            "v1.2.0 (1 week ago)"
        ];
        
        let version = Select::with_theme(&self.theme)
            .with_prompt("Select version to rollback to")
            .items(&versions)
            .default(0)
            .interact()?;
            
        if version == 0 {
            println!("{}", style("Cannot rollback to current version.").yellow());
            return Ok(());
        }

        let confirm = Confirm::with_theme(&self.theme)
            .with_prompt(&format!("⚠️  Are you sure you want to rollback to {}?", versions[version]))
            .default(false)
            .interact()?;

        if !confirm {
            println!("{}", style("Rollback cancelled.").yellow());
            return Ok(());
        }

        println!("\n{}", style("🔄 Initiating rollback...").cyan().bold());
        
        let pb = self.create_progress_bar(100, "Preparing rollback");
        for i in 0..100 {
            pb.inc(1);
            thread::sleep(Duration::from_millis(50));
            
            match i {
                20 => pb.set_message("Stopping current version..."),
                40 => pb.set_message("Loading previous version..."),
                60 => pb.set_message("Updating configuration..."),
                80 => pb.set_message("Starting services..."),
                _ => {}
            }
        }
        pb.finish_with_message("✓ Rollback completed successfully!");

        println!("\n{}", style("Current System Version").cyan().bold());
        println!("Version:    {}", style(versions[version]).green());
        println!("Deployed:   {}", style("Just now").green());
        println!("Status:     {}", style("Healthy").green());

        Ok(())
    }
}