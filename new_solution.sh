#!/bin/bash
echo -n "Enter the YEAR-## of the problem: "
read solution_name
echo -n "Enter the template to use (default: rust): "
read template

if [ -z "$template" ]; then
    template="rust"
fi

target_dir=$solution_name/$template

mkdir -p $target_dir

cp -rT templates/$template-template $target_dir

if [ "$template" == "rust" ]; then
    sed -i "s/rust/$solution_name/g" $target_dir/Cargo.toml
fi
