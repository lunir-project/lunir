[apache-badge]: https://img.shields.io/github/license/lunir-project/lunir.svg?style=flat-square&color=00007F
[apache-url]: /LICENSE-APACHE

[rust-vers-badge]: https://img.shields.io/badge/Rust-1.58.1+-B7410E
[rust-vers-url]: https://releases.rs/docs/1.58.1

[build-badge]: https://github.com/lunir-project/lunir/actions/workflows/rust.yml/badge.svg
[build-url]: https://github.com/lunir-project/lunir/actions/workflows/rust.yml

<div align="center">

  # LUNIR

  <h2>Lua Universal Intermediate Representation</h2>
  <br />
  
  [![Apache licensed][apache-badge]][apache-url] [![Build][build-badge]][build-url] [![Rust Version][rust-vers-badge]][rust-vers-url]
  <br />
</div>


<div align="center">

LUNIR (pronounced `/ˈluː.nɚ/`) is a pun on "lunar," and an acronym for **L**ua **Un**iversal **I**ntermediate **R**epresentation,<br>capable of targeting Lua 5.*x* and Luau.
</div>

<details open="open">
<summary>Table of Contents</summary>
  
- [LUNIR](#lunir)
  - [IR](#ir)
    - [HIR](/src/ir/hir)
    - [MIR](/src/ir/mir)
    - [IL](/src/ir/il)
  - [Questions](#questions)
  - [MSRV](#msrv)
  - [License](#license)
    
</details>

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

LUNIR is an open-source project released under the following licenses:

- [MIT License](/LICENSE-MIT)
- [Apache License 2.0](/LICENSE-APACHE)
