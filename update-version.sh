#!/bin/bash

# remove me when https://github.com/rust-lang/cargo/pull/7649 lands in stable
VERSION=$1

echo "Setting Cargo.toml's version to '${VERSION}', it will not be committed, but it will ensure the version of the build is correct, should we choose to release it."

sed -i -z "s/\"0.0.0-development\"/\"${VERSION}\"/" Cargo.toml
