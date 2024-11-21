//-------------------------------------------------------------------------
// 
// 
//-------------------------------------------------------------------------

use anyhow::Result;
use image_builder::DevContainer;
use std::fs;
mod image_builder;

#[tokio::main]
async fn main() -> Result<()> {
    let file = fs::read_to_string(".devcontainer/devcontainer.json")?;
    let container: DevContainer = serde_json::from_str(&file)?;
    println!("{:#?}",container);
    image_builder::main();
    
  
    Ok(())

}