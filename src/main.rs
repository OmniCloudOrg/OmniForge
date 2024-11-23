//-----------------------------------------------------------------------------
// OmniForge - A Rust-based Free and open-source application deployment
// and lifecycle management platform built for the modern era or cloud
// native apps.
//-----------------------------------------------------------------------------

use anyhow::Result;
use image_builder::DevContainer;
use std::fs;
mod image_builder;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    image_builder::main()?;
    let duration = start.elapsed();
    println!("Took {:?} to build image ", duration);

    Ok(())
}