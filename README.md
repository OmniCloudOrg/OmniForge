# OmniForge

OmniForge is the dedicated build engine for OmniCloud, specializing in building VM Helix images, container images, and developer-created applications ready for deployment to OmniCloud environments.

## Overview

OmniForge provides a comprehensive build pipeline for all OmniCloud deployable assets. It operates as a distributed build system where multiple OmniForge instances can be deployed across an OmniCloud environment, pulling jobs from a central queue and reporting build status back to the OmniCloud Orchestrator cluster.

### Key Features

- **OmniCloud Integration**: Seamlessly builds all assets for deployment to OmniCloud
- **Multi-Asset Building**: Creates VM Helix images, container images, and compiled applications 
- **Distributed Build Capacity**: Deploy multiple OmniForge instances to scale build capacity
- **Centralized Job Queuing**: Pulls jobs from OmniCloud's central build queue
- **Orchestrator Reporting**: Reports build status directly to OmniCloud Orchestrator cluster
- **Development Environment Generation**: Creates consistent development environments for OmniCloud application creation

## Architecture

### Build Pipeline Components

- **VM Helix Image Builder**: Creates optimized virtual machine images for OmniCloud deployment
- **Container Image Factory**: Builds and validates container images for microservice deployments
- **Application Compiler**: Compiles developer applications with appropriate runtime dependencies
- **OmniCloud Connector**: Interfaces with OmniCloud Orchestrator for job management

### Deployment Modes

#### Distributed Build Farm
1. **Multiple Instances**: Deploy numerous OmniForge engines across OmniCloud for increased capacity
2. **Load Balancing**: Automatic job distribution based on instance availability and capacity
3. **Fail-over Protection**: Jobs automatically reassigned if a build instance fails

#### Development Mode
1. **Local Development**: Simulates OmniCloud environment locally for developers
2. **Pre-deployment Validation**: Verifies applications before submission to OmniCloud
3. **Configuration Testing**: Tests deployment configurations before production use

### Job Queue System

The OmniForge job queue system integrates with OmniCloud Orchestrator:
- Job priority management and scheduling
- Build result reporting and logging
- Resource allocation optimization
- Failure recovery and retry mechanisms

## Getting Started

### Prerequisites

- Node.js and NPM
- Docker
- Dev Containers CLI

### Installation

1. Clone the repository:
```bash
git clone https://github.com/omnicloudorg/omniforge
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

### Build Engine Configuration

OmniForge must be configured to connect to your OmniCloud Orchestrator cluster:

```bash
omni connect --orchestrator https://orchestrator.omnicloud.example.com
```

This registers the OmniForge instance with the OmniCloud Orchestrator and enables it to receive build jobs.

### Container and VM Image Generation

OmniForge creates specialized build outputs for different OmniCloud deployment scenarios:

```bash
# Generate a VM Helix image for OmniCloud deployment
omni build vm-helix --config configs/vm-spec.json

# Generate a container image for microservice deployment
omni build container --config configs/container-spec.json
```

#### Development Environment Generation

```bash
omni show devcontainer config
```

This command:
1. Creates a development environment configured for OmniCloud application development
2. Installs OmniCloud SDK and development tools
3. Configures local testing capabilities for OmniCloud deployments

## Configuration

### OmniCloud Orchestrator Connection

Configure connection to the OmniCloud Orchestrator in `omniforge.yaml`:

```yaml
orchestrator:
  url: https://orchestrator.omnicloud.example.com
  api_key: ${OMNI_API_KEY}
  build_capacity: 4  # Number of concurrent builds
  job_types:
    - vm-helix
    - container
    - application
```

### Build Configuration

VM Helix build configuration example:

```json
{
  "name": "web-server-vm",
  "base_image": "omnicloud-ubuntu-22.04",
  "resources": {
    "cpu": 2,
    "memory": "4GB",
    "storage": "40GB"
  },
  "network": {
    "interfaces": 1,
    "public": true
  },
  "packages": [
    "nginx",
    "certbot"
  ]
}
```

## Development

### Project Structure

```
omniforge/
├── src/
│   ├── builders/          # Build engines for different asset types
│   ├── queue/             # Job queue management
│   ├── orchestrator/      # OmniCloud Orchestrator client
│   └── main.rs            # Main application
├── .devcontainer/         # Development container configs
└── docs/                  # Documentation
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
- Issue Tracker: [https://github.com/omnicloudorg/projects](https://github.com/orgs/OmniCloudOrg/projects/1/)
- Community Forum: [https://github.com/omnicloudorg/discussions](https://github.com/omnicloudorg/discussions)

## Acknowledgments

- The OmniCloud Team
- Dev Containers project
- All contributors

---

Built with ❤️ by the OmniCloud & OmniForge Community
