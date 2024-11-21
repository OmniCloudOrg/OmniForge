# Omniforge Instance Deployment

The Omniforge instance is designed to deploy directors that manage virtual machines (VMs) on bare metal servers. This setup ensures efficient utilization of hardware resources and provides a robust environment for running various applications.

## Deployment Architecture

### Mode 1: VM-Based Deployment

1. **Directors**: The directors are responsible for managing the lifecycle of VMs on the bare metal servers. They handle tasks such as provisioning, scaling, and decommissioning of VMs.

2. **Virtual Machines (VMs)**: Each VM is equipped with agents that manage the containers running within them. These agents ensure that the containers are running smoothly and handle tasks such as starting, stopping, and monitoring container health.

3. **Containers**: The containers host the applications and services. Each container includes a monitoring tool called **OmniSentry**. OmniSentry provides real-time metrics and alerts for the applications running inside the containers, ensuring high availability and performance.

### Mode 2: Bare Metal Deployment

1. **Out-of-Band (OOB) Management Controllers**: Omniforge interacts with OOB management controllers to install operating systems (OS) on bare metal servers. This allows for direct deployment of containers without the need for VMs.

2. **Containers**: In this mode, containers are deployed directly on the bare metal servers. Each container includes the **OmniSentry** monitoring tool, ensuring real-time metrics and alerts for high availability and performance.

3. **Non-Managed Mode**: Alternatively, Omniforge can operate in a non-managed mode where it does not install the OS. Instead, it assumes that an OS is already set up on the bare metal servers and proceeds to deploy containers directly.

## Monitoring with OmniSentry

OmniSentry is a comprehensive monitoring tool integrated into each container. It offers the following features:

- **Real-time Metrics**: OmniSentry collects and displays real-time data on CPU usage, memory consumption, and network activity.
- **Alerts and Notifications**: Configurable alerts notify administrators of any issues, allowing for quick resolution.
- **API**: A robust API allowing for the user-friendly gathering of metrics on running applications.