#!/bin/bash

target_name=zachs-pi-zero
target_username=zwarres
target_install_directory=/home/zwarres/Documents/Projects/AnemometerComms
build_output=./target/arm-unknown-linux-gnueabihf/release/comms-project


cross build --target arm-unknown-linux-gnueabihf --release
scp ${build_output} ${target_username}@${target_name}.local:${target_install_directory}