# syntax=docker/dockerfile:1
FROM debian:bookworm AS firmware-builder

# FIRMWARE_GIT should be a git repo with the firmware source code
ARG FIRMWARE_GIT="https://github.com/flipperdevices/flipperzero-firmware.git"
ARG BRANCH="0.102.3"

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    ca-certificates \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
ADD --keep-git-dir=true "${FIRMWARE_GIT}#${BRANCH}" firmware

WORKDIR /app/firmware
RUN ./fbt

####################
# bindgen
FROM rust:1.85.0-bookworm AS builder

ARG CLANG_URL="https://github.com/llvm/llvm-project/releases/download/llvmorg-17.0.6/clang+llvm-17.0.6-x86_64-linux-gnu-ubuntu-22.04.tar.xz"

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    ca-certificates \
    libudev-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt

# We don't use debian's libclang. For details see https://github.com/flipperzero-rs/flipperzero/pull/70#discussion_r1199723419
RUN mkdir --parents /lib/clang
RUN curl --location "${CLANG_URL}" | tar --extract --xz --directory=/lib/clang --strip-components=1
ENV LIBCLANG_PATH="/lib/clang/lib"

COPY --from=firmware-builder /app/firmware/toolchain/ /app/firmware/toolchain/
COPY --from=firmware-builder /app/firmware/build/f7-firmware-D/sdk_headers/ /app/firmware/build/f7-firmware-D/sdk_headers/

WORKDIR /app/flipperzero-rs/

# Cache cargo dependencies
WORKDIR /app/flipperzero-rs/tools
COPY ./tools/Cargo.toml ./tools/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build

# Now copy the rest of the files
WORKDIR /app/flipperzero-rs/tools
COPY ./tools .
RUN cargo build --bin=generate-bindings
RUN cargo run --bin=generate-bindings /app/firmware/build/f7-firmware-D/sdk_headers

####################
FROM scratch
COPY --from=builder /app/flipperzero-rs/tools/bindings.rs /
# Formal entrypoint to simplify the usage of `docker container create`
ENTRYPOINT ["/"]
