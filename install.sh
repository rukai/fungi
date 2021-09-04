#!/bin/sh

set -e
set -u

./uninstall.sh

cargo build --release

CRATE=$PWD
cd /usr/bin

sudo mv ld.lld ld.lld_bak
sudo cp $CRATE/target/release/fungi ld.lld

echo "Succesfully installed"
