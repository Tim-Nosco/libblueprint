#!/bin/bash

docker build -t bp .
docker run -it --rm --privileged bp $@
