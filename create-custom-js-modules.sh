#!/bin/bash

# Set source and destination directories
SRC_DIR="frontend/ui/tauri-plugin-apis"
DEST_DIR="frontend/tests/commands-tests/custom-modules"
TEMPLATE_DIR="custom-js-modules-files"

# Check if template files exist
if [ ! -f "$TEMPLATE_DIR/rollup.config.mjs" ] || [ ! -f "$TEMPLATE_DIR/tsconfig.json" ] || [ ! -f "$TEMPLATE_DIR/package.json" ]; then
    echo "Error: Template files are missing in $TEMPLATE_DIR"
    exit 1
fi

# Read common file contents
ROLLUP_CONFIG=$(cat "$TEMPLATE_DIR/rollup.config.mjs")
TSCONFIG=$(cat "$TEMPLATE_DIR/tsconfig.json")
PACKAGE_JSON=$(cat "$TEMPLATE_DIR/package.json")

# Function to create package
create_package() {
    local plugin_dir=$1
    local package_name=$(basename "$plugin_dir")
    local dest_path="$DEST_DIR/$package_name"

    echo "Creating package: $package_name"

    # Create destination directory if it doesn't exist
    mkdir -p "$dest_path"

    # Copy bindings
    cp -r "$plugin_dir/bindings" "$dest_path/"

    # Create rollup.config.mjs
    echo "$ROLLUP_CONFIG" > "$dest_path/rollup.config.mjs"

    # Create tsconfig.json
    echo "$TSCONFIG" > "$dest_path/tsconfig.json"

    # Create package.json with updated name
    echo "$PACKAGE_JSON" | sed "s/\"name\": \".*\"/\"name\": \"$package_name\"/" > "$dest_path/package.json"

    # Run npm install and build
    (cd "$dest_path" && npm install && npm run build)
}

# Loop through all directories in the source directory
for plugin_dir in "$SRC_DIR"/*; do
    if [ -d "$plugin_dir" ]; then
        create_package "$plugin_dir"
    fi
done

echo "All packages have been generated and built successfully."