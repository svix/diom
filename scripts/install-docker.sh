#!/bin/bash

# This script is for installing Docker on arm64 Github actions runners that don't have it

sudo apt-get -yq update
sudo apt-get -yq install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# shellcheck disable=SC1091
RELEASE="$(. /etc/os-release && echo "$VERSION_CODENAME")"

echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu $RELEASE stable" |
    sudo tee /etc/apt/sources.list.d/docker.list >/dev/null
sudo apt-get update -yq
sudo apt-get -yq install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

sudo usermod -aG docker "$USER"
sudo apt-get -yq install acl
sudo setfacl --modify "user:$USER:rw" /var/run/docker.sock
