#!/bin/bash

APPLICATION=$1
VERSION=$2

sed -i -z "s/name = \"${APPLICATION}\"\nversion = \"[0-9]*\.[0-9]*\.[0-9]*\"/name = \"${APPLICATION}\"\nversion = \"${VERSION}\"/" Cargo.toml
