FROM rustlang/rust:nightly

RUN apt update && apt install -y build-essential binutils-dev libunwind-dev libblocksruntime-dev

CMD cargo install honggfuzz && bash