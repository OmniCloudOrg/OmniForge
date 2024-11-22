//-------------------------------------------------------------------------
// 
// 
//-------------------------------------------------------------------------

use anyhow::Result;
use image_builder::DevContainer;
use std::fs;
mod image_builder;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let file = fs::read_to_string(".devcontainer/devcontainer.json")?;
    let container: DevContainer = serde_json::from_str(&file)?;
    println!("{:#?}",container);
    let start = Instant::now();
    image_builder::main();
    let duration = start.elapsed();
    println!("Took {:?} to build image ", duration);
  
    Ok(())

}