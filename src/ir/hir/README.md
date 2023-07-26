# HIR - High-level Intermediate Representation

HIR is an acronym for **H**igh-level **I**ntermediate **R**epresentation. It serves as a compiler-friendly Abstract Syntax Tree (AST) that simplifies indirections, performs advanced type analysis, and handles constant resolution.

## Table of Contents

- [HIR - High-level Intermediate Representation](#hir---high-level-intermediate-representation)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Features](#features)
  - [Contributing](#contributing)
    - [License](#license)

## Introduction

HIR is a key component of LUNIR, serving as the bridge between higher-level source code and lower-level Intermediate Representations (MIR and IL). It fills the gap between the complexity of the source code and the optimizations carried out at lower levels.

HIR makes it possible for the compiler to do multiple analyses and transformations more efficiently by providing an abstract and organized representation of the source code. To help generate optimal code, it simplifies difficult expressions, resolves constant values, and does extensive type analysis.

## Features

- **Simplified Indirections:** HIR simplifies complex expressions by reducing indirections, making it easier for the compiler to perform optimizations.

- **Advanced Type Analysis:** HIR performs in-depth type analysis, allowing the compiler to make more informed decisions during code generation.

- **Constant Resolution:** HIR handles the resolution of constant values, optimizing their usage in the resulting code.

## Contributing

Contributions to HIR and the entire LUNIR project are highly welcome! If you find bugs, have suggestions for improvements, or want to add new features, please feel free to open issues or submit pull requests.

Before contributing, please read the [contribution guidelines](/CONTRIBUTING.md) to ensure a smooth collaboration process.

If you're looking to contribute check out the *[roadmap](hir/ROADMAP.md)* for a nice list of needed bug-fixes and features to work on.

### License

LUNIR is an open-source project released under the MIT License. See the [LICENSE](/LICENSE) file for more details. By contributing to this project, you agree to abide by the terms of the license and to respect the open-source community guidelines.
