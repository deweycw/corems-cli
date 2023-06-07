FROM debian:bullseye

RUN apt update && apt upgrade -y
RUN apt install clang build-essential pkg-config autoconf libtool python3-dev libhdf5-dev libnetcdf-dev -y
RUN apt install wget -y
RUN apt install zlib1g-dev libncurses5-dev libgdbm-dev libnss3-dev libssl-dev libsqlite3-dev libreadline-dev libffi-dev curl libbz2-dev -y
RUN apt install gdb lldb vim -y
RUN apt install git -y

WORKDIR /CoreMS