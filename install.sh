#!/bin/bash

REPO="https://github.com/u8array/zebro"
VERSION="v0.1.0"
FILE="zebro-$VERSION-linux.tar.gz"

if [[ "$OSTYPE" == "darwin"* ]]; then
  FILE="zebro-$VERSION-macOs.zip"
fi

if command -v wget &> /dev/null; then
  wget "$REPO/releases/download/$VERSION/$FILE"
else
  curl -L -O "$REPO/releases/download/$VERSION/$FILE"
fi

if [[ "$OSTYPE" == "darwin"* ]]; then
  unzip "$FILE"
else
  tar -xzf "$FILE"
fi

sudo mv zebro /usr/local/bin/

rm "$FILE"