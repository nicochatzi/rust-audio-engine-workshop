#!/bin/sh

set -e

check_software_is_installed()
{
    echo "--- $1"
    echo "Minimum required version : $2"

    if ! command -v $1 &> /dev/null
    then
        echo "Missing requirement: $1"
        exit
    else
        echo "Installed :"
        $1 --version
        echo
    fi
}

check_requirements()
{
    echo "=> Checking requirements..."
    check_requirement   git     2.x
    check_requirement   cmake   3.15
    check_requirement   node    14.x
    check_requirement   npm     6.x
    check_requirement   rustc   1.57
    check_requirement   cargo   1.57
    check_requirement   python  3.x
    check_requirement   poetry  1.x
    echo "Required software found!"
    echo
}

install_cargo_extensions()
{
    echo "=> Installing cargo extensions"
    cargo install cargo-watch
    cargo install cargo-edit
    cargo install wasm-pack
}

clone_and_checkout_template()
{
    echo "=> Grabbing template"
    git clone https://github.com/nicochatzi/rust-audio-engine-workshop
    cd rust-audio-engine-workshop && git checkout SOME_HASH
}

check_requirements
install_cargo_extensions
clone_and_checkout_template