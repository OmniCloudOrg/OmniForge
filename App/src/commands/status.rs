use crate::ui::PremiumUI;
use crate::models::ComponentStatus;
use anyhow::Result;
use console::style;
use std::{thread, time::Duration};
use tabled::Table;

impl PremiumUI {
    pub async fn status_interactive(&self) -> Result<()> {
        let mut spinner = self.create_spinner("Fetching application status...");
        thread::sleep(Duration::from_secs(1));
        
        let status = vec![
            ComponentStatus {
                name: "Web Frontend".into(),
                status: "Healthy".into(),
                replicas: "3/3".into(),
                cpu: "65%".into(),
                memory: "78%".into(),
            },
            ComponentStatus {
                name: "API Backend".into(),
                status: "Healthy".into(),
                replicas: "2/2".into(),
                cpu: "45%".into(),
                memory: "52%".into(),
            },
            ComponentStatus {
                name: "Database".into(),
                status: "Healthy".into(),
                replicas: "1/1".into(),
                cpu: "35%".into(),
                memory: "60%".into(),
            },
        ];

        spinner.stop();
        
        println!("\n{}", style("📊 System Status").cyan().bold());
        println!("{}", Table::new(status).to_string());
        
        println!("\n{}", style("🔍 System Metrics").cyan().bold());
        println!("Uptime:        {}", style("15d 7h 23m").green());
        println!("Response Time: {}", style("145ms").green());
        println!("Error Rate:    {}", style("0.02%").green());
        println!("CPU Usage:     {}", style("48.3%").green());
        println!("Memory Usage:  {}", style("63.5%").green());

        Ok(())
    }
}