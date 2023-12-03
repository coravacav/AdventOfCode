#!/bin/bash
echo -n "Enter the YEAR-## of the problem: "
read solution_name
echo -n "Enter the template to use (default: rust): "
read template

if [ -z "$template" ]; then
    template="rust"
fi

target_dir=$solution_name/$template

# abort if target directory already exists
if [ -d "$target_dir" ]; then
    echo -n "Directory $target_dir already exists"
    exit 1
fi

mkdir -p $target_dir

cp -rT templates/$template-template $target_dir

if [ "$template" == "rust" ]; then
    # replace the first occurence of rust
    sed -i "0,/rust/s/rust/$solution_name/" $target_dir/Cargo.toml
fi
