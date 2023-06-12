#!/bin/bash

cd "$(dirname "$0")"

docker compose --project-name hrms-asgn up -d
mkdir -p "${HOME}/lcms-asgn/bin"

cp "./docker-rust" ${HOME}/lcms-asgn/bin/
chmod 755 "${HOME}/lcms-asgn/bin/docker-rust"
echo "export PATH=${HOME}/lcms-asgn/bin:${PATH}" >> ~/.bash_profile

