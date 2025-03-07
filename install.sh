#!/bin/bash

REPO="https://github.com/u8array/zebro"
VERSION="v0.1.0"
FILE="zebro-$VERSION-linux.tar.gz"

if [[ "$OSTYPE" == "darwin"* ]]; then
  FILE="zebro-$VERSION-macOs.zip"
fi

REPO_URL="$REPO/releases/download/$VERSION/$FILE"

if command -v wget &> /dev/null; then
  wget "$REPO_URL"
else
  curl -L -O "$REPO_URL"
fi

if [[ "$OSTYPE" == "darwin"* ]]; then
  unzip "$FILE"
else
  tar -xzf "$FILE"
fi

sudo mv zebro /usr/local/bin/

rm "$FILE"