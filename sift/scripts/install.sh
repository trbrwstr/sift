#!/usr/bin/env bash

set -e

REPO="yourname/logforge"
VERSION=${VERSION:-latest}

echo "Installing LogForge..."

if [[ "$VERSION" == "latest" ]]; then
  URL="https://github.com/$REPO/releases/latest/download/logforge"
else
  URL="https://github.com/$REPO/releases/download/$VERSION/logforge"
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

curl -L "$URL" -o /usr/local/bin/logforge
chmod +x /usr/local/bin/logforge

echo "LogForge installed successfully."