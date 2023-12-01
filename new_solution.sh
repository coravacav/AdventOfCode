#!/bin/bash
echo -n "Enter the YEAR-## of the problem: "
read solution_name
echo -n "Enter the template to use (default: rust): "
read template

if [ -z "$template" ]; then
    template="rust"
fi

mkdir -p $solution_name/$template

cp -rT $template-template $solution_name/$template
cp -rT template-files $solution_name/$template

if [ "$template" == "rust" ]; then
    sed -i "s/rust/$solution_name/g" $solution_name/$template/Cargo.toml
fi
