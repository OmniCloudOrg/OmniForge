use libomni::cpi::vm::CpiCommandType;
use reqwest::{Client, Error as ReqwestError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] ReqwestError),
    #[error("Server returned error: {0}")]
    ServerError(String),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct ContainerClient {
    client: Client,
    base_url: String,
}

impl ContainerClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn create_vm(
        &self,
        guest_id: impl Into<String>,
        memory_mb: i32,
        os_type: impl Into<String>,
        resource_pool: impl Into<String>,
        datastore: impl Into<String>,
        vm_name: impl Into<String>,
        cpu_count: i32,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::CreateVM {
            guest_id: guest_id.into(),
            memory_mb,
            os_type: os_type.into(),
            resource_pool: resource_pool.into(),
            datastore: datastore.into(),
            vm_name: vm_name.into(),
            cpu_count,
        };
        self.execute_command("/vms/create", command).await
    }

    pub async fn start_vm(&self, vm_name: impl Into<String>) -> Result<String, ClientError> {
        let command = CpiCommandType::StartVM {
            vm_name: vm_name.into(),
        };
        self.execute_command("/vms/start", command).await
    }

    pub async fn delete_vm(&self, vm_name: impl Into<String>) -> Result<String, ClientError> {
        let command = CpiCommandType::DeleteVM {
            vm_name: vm_name.into(),
        };
        self.execute_command("/vms/delete", command).await
    }

    pub async fn has_vm(&self, vm_name: impl Into<String>) -> Result<String, ClientError> {
        let command = CpiCommandType::HasVM {
            vm_name: vm_name.into(),
        };
        self.execute_command("/vms/has", command).await
    }

    pub async fn configure_networks(
        &self,
        vm_name: impl Into<String>,
        network_index: i32,
        network_type: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::ConfigureNetworks {
            vm_name: vm_name.into(),
            network_index,
            network_type: network_type.into(),
        };
        self.execute_command("/vms/configure_networks", command)
            .await
    }

    pub async fn create_disk(
        &self,
        size_mb: i32,
        disk_path: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::CreateDisk {
            size_mb,
            disk_path: disk_path.into(),
        };
        self.execute_command("/vms/create_disk", command).await
    }

    pub async fn attach_disk(
        &self,
        vm_name: impl Into<String>,
        controller_name: impl Into<String>,
        port: i32,
        disk_path: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::AttachDisk {
            vm_name: vm_name.into(),
            controller_name: controller_name.into(),
            port,
            disk_path: disk_path.into(),
        };
        self.execute_command("/vms/attach_disk", command).await
    }

    pub async fn delete_disk(
        &self,
        vm_name: impl Into<String>,
        disk_path: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::DeleteDisk {
            vm_name: vm_name.into(),
            disk_path: disk_path.into(),
        };
        self.execute_command("/vms/delete_disk", command).await
    }

    pub async fn detach_disk(
        &self,
        vm_name: impl Into<String>,
        controller_name: impl Into<String>,
        port: i32,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::DetachDisk {
            vm_name: vm_name.into(),
            controller_name: controller_name.into(),
            port,
        };
        self.execute_command("/vms/detach_disk", command).await
    }

    pub async fn has_disk(
        &self,
        vm_name: impl Into<String>,
        disk_path: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::HasDisk {
            vm_name: vm_name.into(),
            disk_path: disk_path.into(),
        };
        self.execute_command("/vms/has_disk", command).await
    }

    pub async fn set_metadata(
        &self,
        vm_name: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::SetVMMetadata {
            vm_name: vm_name.into(),
            key: key.into(),
            value: value.into(),
        };
        self.execute_command("/vms/set_metadata", command).await
    }

    pub async fn create_snapshot(
        &self,
        vm_name: impl Into<String>,
        snapshot_name: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::CreateSnapshot {
            vm_name: vm_name.into(),
            snapshot_name: snapshot_name.into(),
        };
        self.execute_command("/vms/create_snapshot", command).await
    }

    pub async fn delete_snapshot(
        &self,
        vm_name: impl Into<String>,
        snapshot_name: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::DeleteSnapshot {
            vm_name: vm_name.into(),
            snapshot_name: snapshot_name.into(),
        };
        self.execute_command("/vms/delete_snapshot", command).await
    }

    pub async fn has_snapshot(
        &self,
        vm_name: impl Into<String>,
        snapshot_name: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::HasSnapshot {
            vm_name: vm_name.into(),
            snapshot_name: snapshot_name.into(),
        };
        self.execute_command("/vms/has_snapshot", command).await
    }

    pub async fn get_disks(&self, vm_name: impl Into<String>) -> Result<String, ClientError> {
        let command = CpiCommandType::GetDisks {
            vm_name: vm_name.into(),
        };
        self.execute_command("/vms/get_disks", command).await
    }

    pub async fn get_vm(&self, vm_name: impl Into<String>) -> Result<String, ClientError> {
        let command = CpiCommandType::GetVM {
            vm_name: vm_name.into(),
        };
        self.execute_command("/vms/get", command).await
    }

    pub async fn reboot_vm(&self, vm_name: impl Into<String>) -> Result<String, ClientError> {
        let command = CpiCommandType::RebootVM {
            vm_name: vm_name.into(),
        };
        self.execute_command("/vms/reboot", command).await
    }

    pub async fn snapshot_disk(
        &self,
        disk_path: impl Into<String>,
        snapshot_name: impl Into<String>,
    ) -> Result<String, ClientError> {
        let command = CpiCommandType::SnapshotDisk {
            disk_path: disk_path.into(),
            snapshot_name: snapshot_name.into(),
        };
        self.execute_command("/vms/snapshot_disk", command).await
    }

    pub async fn get_snapshots(&self, vm_name: impl Into<String>) -> Result<String, ClientError> {
        let command = CpiCommandType::GetSnapshots {
            vm_name: vm_name.into(),
        };
        self.execute_command("/vms/get_snapshots", command).await
    }

    async fn execute_command(
        &self,
        endpoint: &str,
        command: CpiCommandType,
    ) -> Result<String, ClientError> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self.client.post(url).json(&command).send().await?;

        if !response.status().is_success() {
            return Err(ClientError::ServerError(
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown server error".to_string()),
            ));
        }

        Ok(response.text().await?)
    }
}
