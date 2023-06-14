#!/bin/zsh

cd "$(dirname "$0")"

docker compose -f './corems-cli/docker-compose.yml' up -d
docker pull deweycw/corems-cli

mkdir -p "${HOME}/hrms-asgn/bin"

cp -r "./corems-cli" ${HOME}/hrms-asgn/bin/
chmod 755 "${HOME}/hrms-asgn/bin/corems-cli/hrms-asgn"
echo "export PATH=${HOME}/hrms-asgn/bin/corems-cli:${PATH}" >> "${HOME}/.bashrc"
echo "export PATH=${HOME}/hrms-asgn/bin/corems-cli:${PATH}" >> "${HOME}/.bash_profile"
echo "export PATH=${HOME}/hrms-asgn/bin/corems-cli:${PATH}" >> "${HOME}/.zprofile"