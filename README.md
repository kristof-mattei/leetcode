# Rust seed application

This is a framework for building Rust applications

It supports:

- Building multi-platform images
- Reusing images when merging in PRs to preserve provenance
    - Support for tags like `pr-${PR_NUMBER}-latest` (last build on PR), `edge` (last build on `main`), `pr-${SHA_MAIN_HEAD}-${SHA_PR_HEAD}` (uniquely identifying the merge result of a PR)
- Container attestation
- Crate publishing
- Release publishing
    - Crate publishing to crates.io
    - Container retagging to `:latest`

## TODO

- [ ] Remove old containers when the new one gets build for a PR?<br />
      Or rely on a general weekly untagged cleanup?
- [ ] Remove PR containers when PR closed<br />

## License

MIT, see [LICENSE](./LICENSE)

`SPDX-License-Identifier: MIT`
