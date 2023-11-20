#!/bin/bash

# Check if a parameter is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <os/architecture>"
    exit 1
fi

arch_cut=$(echo "$1" | cut -d'/' -f2)
cd /usr/local/bin

for file in quicklook-*-$arch_cut; do
    newname="${file%-$arch_cut}"
    mv "$file" "${newname}"
done

rm quicklook-*-*
