//-----------------------------------------------------------------------------
// OmniForge - A Rust-based Free and open-source application deployment and
// lifecycle management platform built for the modern era or cloud native apps.
//-----------------------------------------------------------------------------
use hyper::client;
use packer_rs::Packer;
use packer_rs::BuildOptionsBuilder;
use anyhow::Result;

mod image_builder;
mod autoscalar;
pub mod api;
pub mod interfaces;

#[tokio::main]
async fn main() -> Result<()>  {
    runtime().await?;
    Ok(())
}

async fn runtime() -> Result<()> {
    // Create a new Packer instance
    let packer = Packer::new().expect("Failed to create Packer instance");
    
    // Set up some build options
    let options = BuildOptionsBuilder::default()
    .debug(true)
//    .vars(vec![("region".to_string(), "us-west-2".to_string())])
    .build();

    // Build your template
    packer.build("template.pkr.hcl", &options.expect("Failed to build options")).expect("Failed to build");

    
    let mut handles = vec![];
    let client = std::  sync::Arc::new(interfaces::agent::ContainerClient::new("http://localhost:8081"));
    println!("{:?}", client);

    let guest_id = "guest123";
    let memory_mb = 2048;
    let os_type = "linux_64";
    let resource_pool = "pool1";
    let datastore = "datastore1";
    let vm_name: &str = "vm_director";
    let cpu_count = 2;

    for i in 0..0 {
        let vm_name = format!("vm_director{}", i);
        let client = client.clone();
        let vm_name_clone = vm_name.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = client.create_vm(guest_id, memory_mb, os_type, resource_pool, datastore, &vm_name_clone, cpu_count).await {
                println!("Failed to create VM {}: {:?}", vm_name_clone, e);
            }
            if let Err(e) = client.attach_disk(vm_name_clone.clone(), "sata", 1, "C:\\Users\\Chance\\Downloads\\debian-12.8.0-amd64-netinst.iso").await {
                println!("Failed to attach disk to VM {}: {:?}", vm_name_clone, e);
            }
            //if let Err(e) = client.start_vm(vm_name_clone.clone()).await {
            //    println!("Failed to start VM {}: {:?}", vm_name_clone.clone(), e);
            //}
        });
        handles.push((vm_name, handle));
    }
    api::listen().await;
    Ok(())
}