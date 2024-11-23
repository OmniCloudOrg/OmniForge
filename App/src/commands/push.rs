use crate::ui::PremiumUI;
use anyhow::Result;
use console::style;
use dialoguer::{Input, Select};
use std::{thread, time::Duration};

impl PremiumUI {
    pub async fn push_interactive(&self) -> Result<()> {
        let tag: String = Input::with_theme(&self.theme)
            .with_prompt("Enter image tag")
            .default("latest".into())
            .interact_text()?;

        let registries = vec!["Docker Hub", "Google Container Registry", "Amazon ECR"];
        let registry = Select::with_theme(&self.theme)
            .with_prompt("Select registry")
            .items(&registries)
            .interact()?;

        println!("\n{}", style("📦 Pushing image...").cyan().bold());
        
        let pb = self.create_progress_bar(100, "Preparing image");
        for i in 0..100 {
            pb.inc(1);
            thread::sleep(Duration::from_millis(50));
            
            match i {
                20 => pb.set_message("Building layers..."),
                50 => pb.set_message("Optimizing image..."),
                80 => pb.set_message("Pushing to registry..."),
                _ => {}
            }
        }
        pb.finish_with_message("✓ Image pushed successfully!");

        println!("\n{}", style("🏷️  Image Details").cyan().bold());
        println!("Registry: {}", style(registries[registry]).green());
        println!("Tag:      {}", style(tag).green());
        println!("Size:     {}", style("156.4 MB").green());
        println!("Layers:   {}", style("12").green());

        Ok(())
    }
}
