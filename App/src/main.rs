// main.rs
use crate::ui::PremiumUI;
use clap::{Arg, Command};
use console::style;

mod ui;
mod models;
mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ui = PremiumUI::new();
    ui.display_welcome()?;

    let cli = Command::new("omni")
        .about(format!("{}", style("Modern Development Environment CLI").cyan().bold()))
        .subcommand(
            Command::new("up")
                .about(format!("{}", style("Deploy application components").green()))
                .arg(
                    Arg::new("environment")
                        .long("env")
                        .help(&format!("Target environment {}", style("[dev/staging/prod]").yellow()))
                        .required(false)
                )
        )
        .subcommand(
            Command::new("push")
                .about(format!("{}", style("Push images to container registry").green()))
                .arg(
                    Arg::new("tag")
                        .long("tag")
                        .help(&format!("Image tag {}", style("[latest]").yellow()))
                        .required(false)
                )
        )
        .subcommand(
            Command::new("scale")
                .about(format!("{}", style("Scale application components").green()))
                .arg(
                    Arg::new("component")
                        .long("component")
                        .help(&format!("Component to scale {}", style("[frontend/backend/database]").yellow()))
                        .required(false)
                )
                .arg(
                    Arg::new("replicas")
                        .long("replicas")
                        .help(&format!("Number of replicas {}", style("[1-10]").yellow()))
                        .required(false)
                )
        )
        .subcommand(
            Command::new("logs")
                .about(format!("{}", style("View application logs").green()))
                .arg(
                    Arg::new("component")
                        .long("component")
                        .help("Component to view logs for")
                        .required(false)
                )
                .arg(
                    Arg::new("tail")
                        .long("tail")
                        .help("Number of lines to show")
                        .default_value("100")
                )
        )
        .subcommand(
            Command::new("status")
                .about(format!("{}", style("Check application status").green()))
        )
        .subcommand(
            Command::new("rollback")
                .about(format!("{}", style("Rollback to previous version").green()))
                .arg(
                    Arg::new("version")
                        .long("version")
                        .help("Version to rollback to")
                        .required(false)
                )
        )
        .subcommand(
            Command::new("config")
                .about(format!("{}", style("Manage application configuration").green()))
                .subcommand(
                    Command::new("view")
                        .about("View current configuration")
                )
                .subcommand(
                    Command::new("edit")
                        .about("Edit configuration")
                )
                .subcommand(
                    Command::new("reset")
                        .about("Reset configuration to defaults")
                )
        )
        .get_matches();

    match cli.subcommand() {
        Some(("up", _)) => ui.deploy_interactive().await?,
        Some(("push", _)) => ui.push_interactive().await?,
        Some(("scale", _)) => ui.scale_interactive().await?,
        Some(("logs", _)) => ui.logs_interactive().await?,
        Some(("status", _)) => ui.status_interactive().await?,
        Some(("rollback", _)) => ui.rollback_interactive().await?,
        Some(("config", subcommand)) => match subcommand.subcommand() {
            Some(("view", _)) => ui.config_view().await?,
            Some(("edit", _)) => ui.config_edit().await?,
            Some(("reset", _)) => ui.config_reset().await?,
            _ => ui.config_view().await?,
        },
        _ => {
            println!("\n{}", style("AVAILABLE COMMANDS:").magenta().bold());
            println!("  {} {}", style("up").cyan(), style("Deploy your application").dim());
            println!("  {} {}", style("push").cyan(), style("Push images to registry").dim());
            println!("  {} {}", style("scale").cyan(), style("Scale application components").dim());
            println!("  {} {}", style("logs").cyan(), style("View application logs").dim());
            println!("  {} {}", style("status").cyan(), style("Check application status").dim());
            println!("  {} {}", style("rollback").cyan(), style("Rollback to previous version").dim());
            println!("  {} {}", style("config").cyan(), style("Manage application configuration").dim());
            println!("\n{}", style("Use --help with any command for more information.").yellow());
        }
    }

    Ok(())
}