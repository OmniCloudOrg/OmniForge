use anyhow::{Context, Result};
use async_trait::async_trait;
use russh::client;
use russh::*;
use russh_keys::load_secret_key;
use serde::Deserialize;
use std::{
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::io::AsyncWriteExt;
use tokio::net::ToSocketAddrs;

#[derive(Debug, Deserialize)]
struct Host {
    name: String,
    address: String,
    port: u16,
    username: String,
    password: Option<String>,
    use_key: bool,
    key_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    hosts: Vec<Host>,
}

struct Client {}

#[async_trait]
impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &russh_keys::key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct Session {
    session: client::Handle<Client>,
}

impl Session {
    async fn connect<P: AsRef<Path>, A: ToSocketAddrs>(
        config: Arc<client::Config>,
        addrs: A,
        username: String,
        key_path: Option<P>,
        password: Option<String>,
    ) -> Result<Self> {
        let sh = Client {};
        let mut session = client::connect(config, addrs, sh).await?;

        if let Some(key_path) = key_path {
            let key_pair = load_secret_key(key_path, None)?;
            let auth_res = session
                .authenticate_publickey(username, Arc::new(key_pair))
                .await?;

            if !auth_res {
                anyhow::bail!("Public key authentication failed");
            }
        } else if let Some(password) = password {
            let auth_res = session.authenticate_password(username, password).await?;
            if !auth_res {
                anyhow::bail!("Password authentication failed");
            }
        } else {
            anyhow::bail!("No authentication method provided");
        }

        Ok(Self { session })
    }

    async fn execute_command(&mut self, command: &str) -> Result<u32> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut code = None;
        let mut stdout = tokio::io::stdout();

        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                ChannelMsg::Data { ref data } => {
                    stdout.write_all(data).await?;
                    stdout.flush().await?;
                }
                ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                }
                _ => {}
            }
        }
        Ok(code.expect("program did not exit cleanly"))
    }

    async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}

fn read_config<P: AsRef<Path>>(config_path: P) -> Result<ConfigFile> {
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let config: ConfigFile = serde_json5::from_reader(reader)?;
    Ok(config)
}

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

#[tokio::main]
pub async fn main() -> Result<()> {
    // Parse command line arguments
    let args = parse_args()?;
    let config = read_config(args.config).context("failed to read configuration")?;

    const MASTER_RATIO: f32 = 0.2;
    const WORKER_RATIO: f32 = 0.8;

    let total_hosts = config.hosts.len();
    let num_masters = (total_hosts as f32 * MASTER_RATIO).ceil() as usize;
    let num_workers = (total_hosts as f32 * WORKER_RATIO).floor() as usize;

    let mut masters = Vec::new();
    let mut workers = Vec::new();

    for (i, host) in config.hosts.iter().enumerate() {
        if i < num_masters {
            masters.push(host);
        } else if i < num_masters + num_workers {
            workers.push(host);
        }
    }

    println!("Masters: {:?}", masters);
    println!("Workers: {:?}", workers);

    for host in config.hosts {
        println!("Connecting to {}:{}", host.address, host.port);

        let client_config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };

        let mut ssh = Session::connect(
            Arc::new(client_config),
            (host.address, host.port),
            host.username,
            host.key_path.as_ref(),
            host.password,
        )
        .await?;

        // Execute a command to test the connection
        let cmd = ssh
            .execute_command("ls -l /team/lab/HomeLab/")
            .await
            .context("failed to execute shell command")?;

        println!("Dir: {}", cmd);

        println!("Connected to {}", host.name);

        if let Some(ref cmd) = args.command {
            let exit_code = ssh.execute_command(cmd).await?;
            println!("Exit code for {}: {}", host.name, exit_code);
        }

        ssh.close().await?;
    }

    Ok(())
}