FROM ubuntu:24.04

ARG OPENSSL_VERSION=3.4.0

RUN apt-get update && apt-get install -y \
    musl-tools \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add x86_64-unknown-linux-musl

# Create necessary symlinks for musl
RUN ln -s /usr/include/x86_64-linux-gnu/asm /usr/include/x86_64-linux-musl/asm && \
    ln -s /usr/include/asm-generic /usr/include/x86_64-linux-musl/asm-generic && \
    ln -s /usr/include/linux /usr/include/x86_64-linux-musl/linux

# Build OpenSSL for musl
WORKDIR /build

RUN curl -sSL https://github.com/openssl/openssl/releases/download/openssl-${OPENSSL_VERSION}/openssl-${OPENSSL_VERSION}.tar.gz | tar xz && \
    cd openssl-${OPENSSL_VERSION}/ && \
    CC="musl-gcc -fPIE -pie" ./Configure no-shared --prefix=/usr/local/musl --openssldir=/usr/local/musl/ssl linux-x86_64 && \
    make depend && \
    make -j$(nproc) && \
    make install

ENV OPENSSL_DIR=/usr/local/musl

# Create a directory for the Rust project
WORKDIR /app

# Copy the Rust project files
COPY . .

# Build the project for musl target
CMD ["cargo", "build","--color=always", "--target", "x86_64-unknown-linux-musl", "--release"]