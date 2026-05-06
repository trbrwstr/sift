#!/usr/bin/env bash

set -e

REPO="yourname/sift"
VERSION=${VERSION:-latest}

echo "Installing sift..."

if [[ "$VERSION" == "latest" ]]; then
  URL="https://github.com/$REPO/releases/latest/download/sift"
else
  URL="https://github.com/$REPO/releases/download/$VERSION/sift"
fi

OS="$(uname -s)"

case "$OS" in
  Linux)
    URL="$URL-linux"
    ;;
  Darwin)
    URL="$URL-macos"
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

curl -L "$URL" -o /usr/local/bin/sift
chmod +x /usr/local/bin/sift

echo "sift installed successfully."
