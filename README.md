[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/lunir-project/lunir/blob/main/LICENSE
[build-badge]: https://github.com/lunir-project/lunir/actions/workflows/rust.yml/badge.svg
[build-url]: https://github.com/lunir-project/lunir/actions/workflows/rust.yml

# LUNIR • [![MIT licensed][mit-badge]][mit-url] [![Build][build-badge]][build-url]

LUNIR (pronounced `/ˈluː.nɚ/`) is a pun on "lunar," and an acronym for **L**ua **Un**iversal **I**ntermediate **R**epresentation.

## Table of Contents

- [LUNIR](#lunir---)
  - [Table of Contents](#table-of-contents)
  - [IR](#ir)
    - [HIR](/src/ir/hir)
    - [MIR](/src/ir/mir)
    - [IL](/src/ir/il)
  - [Questions](#questions)
  - [MSRV](#msrv)
  - [License](#license)

## IR

LUNIR employs a three-stage IR, where the HIR is at the top, the MIR is in the middle, and the IL is at the bottom.

For more information on the IR, please look at the READMEs for each stage *below*.

### [HIR](/src/ir/hir)

### [MIR](/src/ir/mir)

### [IL](/src/ir/il)

## Questions

Please use *[Github Discussions](../../discussions)* if you have any questions about LUNIR.

For frequently asked questions, please see our *[FAQ](/FAQ.md)*.

## MSRV

The Minimum Supported Rust Version for this crate is **1.58.1**.

## License

LUNIR is an open-source project released under the MIT License. See the [LICENSE](/LICENSE) file for more details.
