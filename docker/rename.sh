#!/bin/bash

# Check if a parameter is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <os/architecture>"
    exit 1
fi

# Extract the os and architecture from the parameter
IFS='/' read -r os arch <<< "$1"

# Navigate to the directory containing the files
cd /usr/local/bin

# Loop through each file that matches the pattern quicklook-*-$arch
for file in quicklook-*-$arch; do
    # Use parameter expansion to create the new file name
    newname="${file%-$arch}"

    # Rename the file by appending the os
    mv "$file" "${newname}-${os}"
done
