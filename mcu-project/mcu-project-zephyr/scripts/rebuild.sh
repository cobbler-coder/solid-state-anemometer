#!/bin/bash
# usage: ./rebuild.sh [app_name]

APP_NAME="app" # Default to 'my-app' if no arg provided
BOARD="nucleo_l476rg"

echo "♻️  Rebuilding $APP_NAME for $BOARD (Pristine)..."

west build -b $BOARD $APP_NAME --pristine

if [ $? -eq 0 ]; then
    echo "✅ Build Successful!"
else
    echo "❌ Build Failed"
    exit 1
fi