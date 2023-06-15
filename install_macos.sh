#!/bin/zsh

cd "$(dirname "$0")"

docker compose -f './corems-cli/docker-compose.yml' up -d
docker pull deweycw/corems-cli

mkdir -p "${HOME}/corems-cli/bin"

cp -r "./corems-cli/corems-cli" ${HOME}/corems-cli/bin/
cp -r "./corems-cli/docker-compose.yml" ${HOME}/corems-cli/bin/
chmod 755 "${HOME}/corems-cli/bin/"
echo "export PATH=${HOME}/corems-cli/bin/:${PATH}" >> "${HOME}/.bashrc"
echo "export PATH=${HOME}/corems-cli/bin/:${PATH}" >> "${HOME}/.bash_profile"
echo "export PATH=${HOME}/corems-cli/bin/:${PATH}" >> "${HOME}/.zprofile"
echo "export PATH=${HOME}/corems-cli/bin/:${PATH}" >> "${HOME}/.zprofile"