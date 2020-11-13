#!/bin/bash
VERSION=`cargo read-manifest | jq .version | sed -r 's/\"//g'`

cargo build --release
docker build ./ -t "romf:$VERSION"