#!/bin/bash
sudo apt update
sudo apt install -y ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/debian/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

sudo tee /etc/apt/sources.list.d/docker.sources <<EOF
Types: deb
URIs: https://download.docker.com/linux/debian
Suites: $(. /etc/os-release && echo "$VERSION_CODENAME")
Components: stable
Architectures: $(dpkg --print-architecture)
Signed-By: /etc/apt/keyrings/docker.asc
EOF

sudo apt update

sudo apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

docker pull ghcr.io/CLOUD-DREAM-TEAM/vm-runner:latest

docker run -d -p 7000:80 --name vm-runner \
  -e RUST_LOG="${RUST_LOG:-debug}" \
  -e VM_ID="${VM_ID}" \
  -e ORCHESTRATOR_URL="${ORCHESTRATOR_URL}" \
  -e LOGS_PORT="${LOGS_PORT}" \
  -e VM_REPORT_INTERVAL="${VM_REPORT_INTERVAL}" \
  ghcr.io/CLOUD-DREAM-TEAM/vm-runner:latest
