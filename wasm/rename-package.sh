#!/bin/bash

# Update the "name" field in pkg/package.json files

# Define the old and new names
OLD_NAME='"name": "refilelabs-image"'
NEW_NAME='"name": "@refilelabs/image"'

# Find all pkg/package.json files and update the "name" field
find . -path '*/pkg/package.json' -type f | while read -r file; do
    if grep -q "$OLD_NAME" "$file"; then
        sed -i.bak "s|$OLD_NAME|$NEW_NAME|" "$file" && echo "Updated $file"
        rm "$file.bak" # Remove backup file created by sed
    else
        echo "No match found in $file, skipping."
    fi
done

echo "Renaming completed."
