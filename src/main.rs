//-----------------------------------------------------------------------------
// OmniForge - A Rust-based Free and open-source application deployment and
// lifecycle management platform built for the modern era or cloud native apps.
//-----------------------------------------------------------------------------


mod image_builder;
pub mod api;

#[tokio::main]
async fn main()  {
    api::listen().await;
}
