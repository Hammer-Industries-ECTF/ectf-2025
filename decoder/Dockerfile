FROM ubuntu:24.04

LABEL version="1.0.0"

WORKDIR /root

# Install Requisite Packages
RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    make \
    wget \
    curl \
    binutils-arm-none-eabi \
    build-essential

# Install MSDK
RUN apt-get install -y unzip && \
    wget https://github.com/analogdevicesinc/msdk/archive/refs/tags/v2024_02.zip && \
    unzip v2024_02.zip && \
    rm -f v2024_02.zip

ENV MAXIM_PATH=/root/msdk-2024_02

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . "$HOME/.cargo/env"

RUN ~/.cargo/bin/rustup target add thumbv7em-none-eabihf

WORKDIR /decoder

ENTRYPOINT [ "bash", "-c", "/root/.cargo/bin/cargo build --release && mkdir -p build && arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/release/decoder ./build/max78000.bin && cp build/max78000.bin /out" ]

# CMD [ "bash", "-c", "exec bash" ]

# Sample run command
# docker run -v ./decoder:/decoder -v ./global.secrets:/global.secrets:ro -v ./deadbeef_build:/out -e DECODER_ID=0xdeadbeef build-decoder