#!/bin/bash

docker build -t bp .

export dir="/src/test/hfuzz"
docker run -it --rm -e "HFUZZ_INPUT=$dir/corpus/$1" \
-u `id -u`:`id -g` -w $dir --privileged -e "harness=$1" \
-v $PWD/libblueprint:/src bp
