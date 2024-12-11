//-----------------------------------------------------------------------------
// OmniForge - A Rust-based Free and open-source application deployment and
// lifecycle management platform built for the modern era or cloud native apps.
//-----------------------------------------------------------------------------
use hyper::client;

mod image_builder;
mod autoscalar;
pub mod api;
pub mod interfaces;

#[tokio::main]
async fn main()  {
    let client = std::sync::Arc::new(interfaces::agent::VmClient::new("http://100.107.144.110:8081"));

    let guest_id = "guest123";
    let memory_mb = 2048;
    let os_type = "linux_64";
    let resource_pool = "pool1";
    let datastore = "datastore1";
    let vm_name = "vm_director";
    let cpu_count = 2;

    println!("{:?}", client);

    let mut handles = vec![];

    for i in 0..1 {
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
            if let Err(e) = client.start_vm(vm_name_clone.clone()).await {
                println!("Failed to start VM {}: {:?}", vm_name_clone.clone(), e);
            }
        });

        handles.push((vm_name, handle));
    }
    api::listen().await;
}
