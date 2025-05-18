#!/usr/bin/env bash

# sane defaults
compiler="gcc"
# static linking
flags="-Clink-self-contained=yes -Clinker=rust-lld"

# mind the space between the [ and "
if [[ "$BUILDPLATFORM" != "$TARGETPLATFORM" ]]; then
    case $TARGET in
        x86_64-unknown-linux-musl)
            compiler="i686-linux-gnu-gcc"
            ;;
        aarch64-unknown-linux-musl)
            compiler="aarch64-linux-gnu-gcc"
            ;;
        *)
            echo "INVALID CONFIGURATION"
            exit 1
            ;;
    esac
fi

# replace - with _ in the Rust target
target_lower=${TARGET//-/_}
cc_var=CC_${target_lower}
declare -x "${cc_var}=${compiler}"

RUSTFLAGS=$flags cargo $@
