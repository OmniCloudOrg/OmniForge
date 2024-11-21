# OmniForge

OmniForge is a universal deployment platform for the modern cloud native era, providing comprehensive container and VM orchestration alongside development environment management.

## Overview

OmniForge combines powerful containerization capabilities with VM orchestration and development environment management. It operates in multiple modes to support various deployment scenarios, from development containers to full production orchestration.

### Key Features

- **Universal Deployment**: Deploy anywhere - public cloud, private infrastructure, or hybrid environments
- **Development Container Management**: Automatic detection and configuration of development environments
- **VM Orchestration**: Manages virtual machines on bare metal servers through directors
- **Container Orchestration**: Handles container lifecycle within VMs or directly on bare metal
- **Multi-Mode Operation**: Supports both VM-based and bare metal deployments
- **Integrated Monitoring**: Built-in OmniSentry monitoring for containers and applications

## Architecture

### Development Environment Management

- Automatically detects project requirements by scanning source code
- Generates appropriate development container configurations
- Manages installation of required tools and dependencies
- Supports a wide range of programming languages and frameworks

### Deployment Modes

#### Mode 1: VM-Based Deployment
1. **Directors**: Manage VM lifecycle on bare metal servers
2. **Virtual Machines**: Run container agents for orchestration
3. **Containers**: Host applications with integrated OmniSentry monitoring

#### Mode 2: Bare Metal Deployment
1. **OOB Management**: Direct OS installation and container deployment
2. **Container Management**: Direct container orchestration on bare metal
3. **Non-Managed Mode**: Works with pre-configured systems

### Monitoring System

OmniSentry provides comprehensive monitoring capabilities:
- Real-time metrics collection
- Configurable alerts and notifications
- API for metrics gathering
- Performance monitoring and optimization

## Getting Started

### Prerequisites

- Node.js and NPM
- Docker
- Dev Containers CLI

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/omniforge.git
cd omniforge
```

2. Run the installation script:
```bash
cargo run
```

This will automatically:
- Check for required dependencies
- Install missing components
- Configure the development environment

### Container Generation and Usage

OmniForge's container generation process creates specialized environments for both development and production use cases. When generating a build container, the system creates a comprehensive development environment that includes the full compiler toolchain, development dependencies, and testing frameworks. This container is designed to provide developers with all the tools they need for efficient development work.

For production deployments, OmniForge generates a separate runtime container. This container is stripped down to include only the components necessary to run the application in production. The runtime container undergoes automatic optimization for size and security, removing unnecessary development tools and potential security vulnerabilities.

To create these containers, OmniForge provides simple commands that handle the complexity of container generation:

```bash
# Generate a development container with full build toolchain
omniforge gen-devcontainer --type build

# Generate a minimal runtime container for production
omniforge gen-devcontainer --type runtime
```

These commands trigger OmniForge's analysis and generation systems, which automatically determine the appropriate configurations based on your project's requirements. The system handles all aspects of container creation, from dependency resolution to security hardening, requiring minimal manual intervention.

#### Development Container Generation

```bash
omniforge gen-devcontainer
```

This command:
1. Scans your project directory
2. Identifies required development tools
3. Generates appropriate container configurations

#### Deployment Management

```bash
omniforge deploy --mode [vm|baremetal]
```

Options:
- `--mode vm`: Deploy using VM-based orchestration
- `--mode baremetal`: Deploy directly to bare metal
- `--config path/to/config.json`: Specify custom configuration

## Configuration

### Development Containers

Configuration is managed through `.devcontainer/devcontainer.json`:

```json
{
  "name": "Project Name",
  "image": "ubuntu:latest",
  "features": {
    "ghcr.io/devcontainers/features/node:1": {},
    "ghcr.io/devcontainers/features/python:1": {}
  }
}
```

### Deployment Configuration

VM and container orchestration is configured through `config.json`:

```json
{
  "hosts": [
    {
      "name": "host1",
      "address": "192.168.1.100",
      "port": 22,
      "username": "admin",
      "use_key": true,
      "key_path": "/path/to/key"
    }
  ]
}
```

## Development

### Project Structure

```
omniforge/
├── src/
│   ├── ensure/           # Dependency management
│   ├── image_gen/        # Container image generation
│   ├── scanner/          # Project analysis
│   └── main.rs          # Main application
├── .devcontainer/       # Development container configs
└── docs/               # Documentation
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

[License Type] - see LICENSE.md for details

## Support

- Documentation: [Link to docs]
- Issue Tracker: [Link to issues]
- Community Forum: [Link to forum]

## Acknowledgments

- The Rust community
- Dev Containers project
- All contributors

---

Built with ❤️ by the OmniForge Community