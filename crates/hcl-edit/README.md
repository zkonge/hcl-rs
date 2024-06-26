# hcl-edit

[![Build Status](https://github.com/martinohmann/hcl-rs/workflows/ci/badge.svg)](https://github.com/martinohmann/hcl-rs/actions?query=workflow%3Aci)
[![crates.io](https://img.shields.io/crates/v/hcl-edit)](https://crates.io/crates/hcl-edit)
[![docs.rs](https://img.shields.io/docsrs/hcl-edit)](https://docs.rs/hcl-edit)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Parse and modify HCL documents while preserving whitespace and comments. This
crate is to HCL what [`toml_edit`](https://docs.rs/toml_edit) is to TOML. In
fact, its API is heavily inspired by `toml_edit`.

## Notice

The documentation as well as usage examples are scarce. There's still a lot of
planned functionality missing and some APIs might even be cumbersome to use at
the moment.

This will improve over time as this project evolves.

**Expect breaking changes at any time** until the biggest issues are fleshed out.

## HCL document traversal

The [`visit`](https://docs.rs/hcl-edit/latest/hcl_edit/visit/index.html) module
allows traversal of language items within a HCL document. Mutable document
traversal is supported via the
[`visit_mut`](https://docs.rs/hcl-edit/latest/hcl_edit/visit_mut/index.html)
module.

See the respective module's documentation for more.

## Contributing

Contributions are welcome! Please read
[`CONTRIBUTING.md`](https://github.com/martinohmann/hcl-rs/blob/main/CONTRIBUTING.md)
before creating a PR.

## License

The source code of hcl-edit is licensed under either of [Apache License, Version
2.0](https://github.com/martinohmann/hcl-rs/blob/main/LICENSE-APACHE) or [MIT
license](https://github.com/martinohmann/hcl-rs/blob/main/LICENSE-MIT) at your
option.
