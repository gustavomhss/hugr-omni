# The image of the self-hosted Linux runner (.gitlab-ci.yml, scripts/ci.mjs): Rust 1.98 with the Windows and musl
# targets, Node 22 and Python 3, so a job starts building at once.
#   docker build -t omni-ci-linux:1 -f scripts/ci.Dockerfile scripts
FROM rust:1.96
RUN apt-get update -qq && apt-get install -y -qq --no-install-recommends python3 python3-pip xz-utils binutils \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://nodejs.org/dist/v22.20.0/node-v22.20.0-linux-x64.tar.xz | tar -xJ -C /usr/local --strip-components=1
RUN rustup toolchain install 1.98.0 --profile minimal -c rustfmt,clippy -t x86_64-pc-windows-msvc,x86_64-unknown-linux-musl \
    && rustup default 1.98.0 && rustup toolchain uninstall 1.96
