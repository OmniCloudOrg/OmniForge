
struct Args {
    config: PathBuf,
    command: Option<String>,
}

fn parse_args() -> Result<Args> {
    let args: Vec<String> = env::args().collect();
    let mut config = PathBuf::from("config.json");
    let mut command = None;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--config" => {
                i += 1;
                if i < args.len() {
                    config = PathBuf::from(&args[i]);
                } else {
                    anyhow::bail!("Missing value for config argument");
                }
            }
            "-x" | "--command" => {
                i += 1;
                if i < args.len() {
                    command = Some(args[i].clone());
                } else {
                    anyhow::bail!("Missing value for command argument");
                }
            }
            _ => {
                if command.is_none() {
                    command = Some(args[i].clone());
                }
            }
        }
        i += 1;
    }

    Ok(Args { config, command })
}
