//-----------------------------------------------------------------------------
// OmniForge - A Rust-based Free and open-source application deployment and
// lifecycle management platform built for the modern era or cloud native apps.
//-----------------------------------------------------------------------------

use anyhow::Result;
mod image_builder;
use std::time::Instant;
pub mod api;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    image_builder::main()?;
    let duration = start.elapsed();
    println!("Took {:?} to build image ", duration);

    api::listen().await;
    Ok(())
}
