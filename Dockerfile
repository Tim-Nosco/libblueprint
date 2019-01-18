FROM rust

RUN apt update && apt install -y build-essential binutils-dev libunwind-dev libblocksruntime-dev

WORKDIR /src
COPY libblueprint .

RUN cargo install honggfuzz

WORKDIR /src/test/hfuzz
ENTRYPOINT ["cargo","hfuzz","run","hfuzz_decode"]
