#!/bin/bash
set -e
set -o pipefail
apt-get update
apt-get install openssh-server
systemctl start sshd
adduser gravitle
