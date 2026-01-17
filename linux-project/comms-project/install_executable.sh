#!/bin/bash

# Stop script if any command fails
set -e

# --- Configuration ---
BINARY_NAME="comms-project"
TARGET_ARCH="arm-unknown-linux-gnueabihf"
TARGET_HOST="zachs-pi-zero.local"
TARGET_USER="zwarres"
TARGET_DIR="/home/zwarres/Documents/Projects/AnemometerComms"

# The local path where Cargo outputs the binary
LOCAL_BIN_PATH="./target/${TARGET_ARCH}/release/${BINARY_NAME}"


# --- Execution ---
echo "🔨 Building ${BINARY_NAME} for ${TARGET_ARCH}..."
cross build --target ${TARGET_ARCH} --release

echo "🚀 Deploying to ${TARGET_HOST}..."
scp ${LOCAL_BIN_PATH} ${TARGET_USER}@${TARGET_HOST}:${TARGET_DIR}

echo "✅ Done! Binary is at: ${TARGET_DIR}/${BINARY_NAME}"