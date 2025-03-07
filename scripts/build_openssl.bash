#!/usr/bin/env bash
set -euo pipefail

sudo apt-get update
sudo apt-get install -y musl-tools libssl-dev
sudo ln -s /usr/include/x86_64-linux-gnu/asm /usr/include/x86_64-linux-musl/asm
sudo ln -s /usr/include/asm-generic /usr/include/x86_64-linux-musl/asm-generic
sudo ln -s /usr/include/linux /usr/include/x86_64-linux-musl/linux

mkdir musl_build
MUSL_OPENSSL_BUILD_DIR=$(readlink -f musl_build)
readonly MUSL_OPENSSL_BUILD_DIR

wget https://github.com/openssl/openssl/archive/OpenSSL_1_1_1f.tar.gz
tar zxvf OpenSSL_1_1_1f.tar.gz
cd openssl-OpenSSL_1_1_1f/

CC="musl-gcc -fPIE -pie" ./Configure no-shared no-async --prefix="${MUSL_OPENSSL_BUILD_DIR}" --openssldir="${MUSL_OPENSSL_BUILD_DIR}/ssl" linux-x86_64
make depend
make -j"$(nproc)"
make install