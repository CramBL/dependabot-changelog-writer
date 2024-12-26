FROM ubuntu:24.10

# Install build dependencies
RUN apt-get update && apt-get install -y \
    musl-tools \
    libssl-dev \
    curl \
    build-essential \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

# Create necessary symlinks for musl
RUN ln -s /usr/include/x86_64-linux-gnu/asm /usr/include/x86_64-linux-musl/asm && \
    ln -s /usr/include/asm-generic /usr/include/x86_64-linux-musl/asm-generic && \
    ln -s /usr/include/linux /usr/include/x86_64-linux-musl/linux

# Build OpenSSL for musl
WORKDIR /build
RUN curl -sSL https://github.com/openssl/openssl/archive/OpenSSL_1_1_1f.tar.gz | tar xz && \
    cd openssl-OpenSSL_1_1_1f/ && \
    CC="musl-gcc -fPIE -pie" ./Configure no-shared no-async --prefix=/usr/local/musl --openssldir=/usr/local/musl/ssl linux-x86_64 && \
    make depend && \
    make -j$(nproc) && \
    make install

# Set environment variables for linking
ENV PKG_CONFIG_PATH=/usr/local/musl/lib/pkgconfig
ENV OPENSSL_DIR=/usr/local/musl
ENV OPENSSL_INCLUDE_DIR=/usr/local/musl/include
ENV OPENSSL_LIB_DIR=/usr/local/musl/lib

# Create a directory for the Rust project
WORKDIR /app

# Copy the Rust project files
COPY . .

# Build the project for musl target
CMD ["cargo", "build", "--target", "x86_64-unknown-linux-musl", "--release"]