#!/bin/bash

REPO="https://github.com/u8array/zebro"
VERSION="v0.1.0"

wget "$REPO/releases/download/$VERSION/zebro-$VERSION-linux.tar.gz"

tar -xzf "zebro-$VERSION-linux.tar.gz"

sudo mv zebro /usr/local/bin/

rm "zebro-$VERSION-linux.tar.gz"