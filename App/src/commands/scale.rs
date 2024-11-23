use crate::models::ComponentStatus;
use crate::ui::PremiumUI;
use anyhow::Result;
use console::style;
use dialoguer::{Input, Select};
use std::{thread, time::Duration};
use tabled::Table;

impl PremiumUI {
    pub async fn scale_interactive(&self) -> Result<()> {
        let components = vec!["Web Frontend", "API Backend", "Database"];
        let component = Select::with_theme(&self.theme)
            .with_prompt("Select component to scale")
            .items(&components)
            .interact()?;

        let replicas: u32 = Input::with_theme(&self.theme)
            .with_prompt("Enter number of replicas")
            .validate_with(|input: &String| -> Result<(), &str> {
                match input.parse::<u32>() {
                    Ok(n) if n > 0 && n <= 10 => Ok(()),
                    _ => Err("Please enter a number between 1 and 10")
                }
            })
            .interact_text()?
            .parse()?;

        let mut spinner = self.create_spinner("Scaling component...");
        thread::sleep(Duration::from_secs(2));
        spinner.stop_with_message("✓ Scaling completed successfully!".to_string());

        println!("\n{}", style("📊 Updated Component Status").cyan().bold());
        let status = Table::new(vec![ComponentStatus {
            name: components[component].into(),
            status: "Running".into(),
            replicas: format!("{}/{}", replicas, replicas),
            cpu: format!("{}m", replicas * 150),
            memory: format!("{}Mi", replicas * 256),
        }]).to_string();
        println!("{}", status);

        Ok(())
    }
}
