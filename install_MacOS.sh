#!/bin/bash

mkdir -p "${HOME}/lcms-asgn/bin"
cd "$(dirname "$0")"
ls .
cp "./docker-rust" ${HOME}/lcms-asgn/bin/
chmod 755 "${HOME}/lcms-asgn/bin/docker-rust"

echo "export PATH=${HOME}/lcms-asgn/bin:${PATH}" >> ~/.bash_profile

