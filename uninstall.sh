#!/bin/sh

set -e
set -u

cd /usr/bin

if [ -f "ld.lld_bak" ]; then
    if [ -f "ld.lld" ]; then
        sudo rm ld.lld
    fi
    sudo mv ld.lld_bak ld.lld
    echo "Succesfully uninstalled"
else
    echo "Was not installed"
fi
