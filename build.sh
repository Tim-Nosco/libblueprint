#!/bin/bash

docker build -t bp .

export dir="/src/test/hfuzz"
docker run -it --rm -e "HFUZZ_INPUT=$dir/corpus/hfuzz_decode" \
-u `id -u`:`id -g` -w $dir --privileged \
-v $PWD/libblueprint:/src bp
