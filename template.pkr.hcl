packer {
    required_plugins {
    virtualbox = {
      source  = "github.com/hashicorp/virtualbox"
      version = "~> 1"
    }
  }
}

variable "vm_name" {
  type    = string
  default = "ubuntu-base"
}

variable "cpus" {
  type    = string
  default = "2"
}

variable "memory" {
  type    = string
  default = "4096"
}

variable "disk_size" {
  type    = string
  default = "40960"
}

variable "headless" {
  type    = bool
  default = true
}

locals {
  iso_url      = "https://releases.ubuntu.com/jammy/ubuntu-22.04.5-live-server-amd64.iso"
  iso_checksum = "sha256:9bc6028870aef3f74f4e16b900008179e78b130e6b0b9a140635434a46aa98b0"
  
  // Base autoinstall configuration embedded as heredoc
  user_data = <<EOF
#cloud-config
autoinstall:
  version: 1
  locale: en_US
  keyboard:
    layout: us
  network:
    network:
      version: 2
      ethernets:
        enp0s3:
          dhcp4: true
  identity:
    hostname: ubuntu-server
    username: ubuntu
    # Password is 'ubuntu'
    password: "$6$rounds=4096$NFl9k7AwRX6UhF62$GV9a05.ytTkapUGvMwGsKFkxdbk3vO3nEd4cWsyPxNjjYFdTGHORdYYLmEYIJLPB7zI0rldfC4IiKvI/TLnDX."
  ssh:
    install-server: true
    allow-pw: true
  storage:
    layout:
      name: direct
  packages:
    - openssh-server
    - sudo
    - curl
    - wget
    - vim
    - net-tools
    - docker.io
  user-data:
    disable_root: false
  late-commands:
    - echo 'ubuntu ALL=(ALL) NOPASSWD:ALL' > /target/etc/sudoers.d/ubuntu
    - chmod 440 /target/etc/sudoers.d/ubuntu
    - "sed -i 's/^#*\\(PermitRootLogin\\).*/\\1 yes/' /target/etc/ssh/sshd_config"
    - "sed -i 's/^#*\\(PasswordAuthentication\\).*/\\1 yes/' /target/etc/ssh/sshd_config"
EOF
}

source "virtualbox-iso" "ubuntu" {
  guest_os_type    = "Ubuntu_64"
  vm_name          = var.vm_name
  cpus             = var.cpus
  memory           = var.memory
  disk_size        = var.disk_size
  headless         = var.headless
  
  iso_url          = local.iso_url
  iso_checksum     = local.iso_checksum
  
  ssh_username     = "ubuntu"
  ssh_password     = "ubuntu"
  ssh_timeout      = "30m"
  ssh_host_port_min = 2222
  ssh_host_port_max = 2222
  host_port_min    = 2222
  host_port_max    = 2222
  
  shutdown_command = "echo 'ubuntu' | sudo -S shutdown -P now"
  
  boot_command = [
    "<esc><wait>",
    "f2<wait>",
    "c<wait>",
    "linux /casper/vmlinuz --- autoinstall ds='nocloud-net;seedfrom=http://{{.HTTPIP}}:{{.HTTPPort}}/'<enter><wait>",
    "initrd /casper/initrd<enter><wait>",
    "boot<enter><wait>"
  ]
  
  vboxmanage = [
    ["modifyvm", "{{.Name}}", "--rtcuseutc", "on"],
    ["modifyvm", "{{.Name}}", "--graphicscontroller", "vmsvga"],
    ["modifyvm", "{{.Name}}", "--boot1", "dvd"],
    ["modifyvm", "{{.Name}}", "--boot2", "disk"],
    ["modifyvm", "{{.Name}}", "--nat-localhostreachable1", "on"],
    ["modifyvm", "{{.Name}}", "--natpf1", "guestssh,tcp,,2222,,22"]
  ]
  
  http_content = {
    "/user-data" = local.user_data
    "/meta-data" = ""
  }
}

build {
  sources = ["source.virtualbox-iso.ubuntu"]
  
  provisioner "shell" {
    inline = [
      "echo 'ubuntu' | sudo -S apt-get update",
      "echo 'ubuntu' | sudo -S apt-get upgrade -y",
      "echo 'ubuntu' | sudo -S apt-get clean",
      "echo 'ubuntu' | sudo -S apt-get autoremove -y",
      
      # Clear logs and temporary files
      "sudo rm -f /var/log/audit/audit.log",
      "sudo truncate -s 0 /var/log/wtmp",
      "sudo truncate -s 0 /var/log/lastlog",
      
      # Remove SSH host keys
      "sudo rm -f /etc/ssh/ssh_host_*",
      
      # Clear machine ID
      "sudo truncate -s 0 /etc/machine-id",
      "sudo rm -f /var/lib/dbus/machine-id",
      
      # Clear shell history
      "sudo rm -f /root/.bash_history",
      "rm -f ~/.bash_history",
      
      # Clean temporary files
      "sudo rm -rf /tmp/*",
      "sudo rm -rf /var/tmp/*"
    ]
  }
}