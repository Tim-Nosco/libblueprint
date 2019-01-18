#!/bin/bash

docker build -t bp .

docker run -it --rm -e "HFUZZ_INPUT=/corpus/$1" --privileged \
-v $PWD/libblueprint/test/hfuzz/corpus:/corpus bp $1
