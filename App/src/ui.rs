use console::{style, Term};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use spinners::{Spinner, Spinners};
use dialoguer::theme::ColorfulTheme;
use anyhow::Result;

const LOGO: &str = r#"
   ____                  _ ______                    
  / __ \____ ___  ____  (_) ____/___  _________ ____ 
 / / / / __ `__ \/ __ \/ / /_  / __ \/ ___/ __ `/ _ \
/ /_/ / / / / / / / / / / __/ / /_/ / /  / /_/ /  __/
\____/_/ /_/ /_/_/ /_/_/_/    \____/_/   \__, /\___/ 
                                        /____/       
"#;

pub struct PremiumUI {
    pub term: Term,
    pub multi_progress: MultiProgress,
    pub theme: ColorfulTheme,
}

impl PremiumUI {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
            multi_progress: MultiProgress::new(),
            theme: ColorfulTheme::default(),
        }
    }

    pub fn display_welcome(&self) -> Result<()> {
        self.term.clear_screen()?;
        println!("{}", style(LOGO).cyan().bold());
        println!("{}", style("Welcome to Omniforge - Modern Development Environment").cyan().bold());
        println!("{}\n", style("Version 1.0.0").dim());
        Ok(())
    }

    pub fn create_spinner(&self, message: &str) -> Spinner {
        Spinner::with_timer(Spinners::Dots12, message.into())
    }

    pub fn create_progress_bar(&self, len: u64, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(len));
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("=>-"));
        pb.set_message(message.to_string());
        pb
    }
}