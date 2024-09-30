#!/bin/bash
# run_gen_types.sh

set -e  # Exit immediately if a command exits with a non-zero status

# Function to extract the project name from Cargo.toml
extract_project_name() {
    local cargo_toml="$1"
    sed -n 's/^name = "\(.*\)"/\1/p' "$cargo_toml"
}

# Find all gen-types-*.rs files in the workspace
find_gen_types_files() {
    find . -name "gen-types-*.rs" -type f
}

# Function to find the nearest Cargo.toml file in parent directories
find_cargo_toml() {
    local dir="$1"
    while [[ "$dir" != "/" ]]; do
        if [[ -f "$dir/Cargo.toml" ]]; then
            echo "$dir/Cargo.toml"
            return
        fi
        dir=$(dirname "$dir")
    done
    echo "Cargo.toml not found for $1"
}

# Iterate over each gen-types-*.rs file
while IFS= read -r gen_types_file; do
    # Get the directory containing the gen-types-*.rs file
    project_dir=$(dirname "$gen_types_file")

    # Find the nearest Cargo.toml file in parent directories
    cargo_toml=$(find_cargo_toml "$project_dir")

    # Extract the project name from Cargo.toml
    project_name=$(extract_project_name "$cargo_toml")

    # Get the binary name from the gen-types-*.rs file
    gen_types_binary=$(basename "$gen_types_file" .rs)

    # Run cargo update for the project
    echo "Running cargo update for project: $project_name"
    cargo update

    # Run the gen-types binary for the project
    echo "Running gen-types for project: $project_name"
    cargo run --bin "$gen_types_binary"

done < <(find_gen_types_files)
