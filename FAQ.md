# Frequently Asked Questions (*FAQ*)

## *What can LUNIR do?*

LUNIR is mainly an IR geared towards compilation, but its design was specifically made to have it go both ways.

LUNIR can be used for the following:

- Compilers
- Decompilers
- Transpilers
- Formatters
- Linters
- Obfuscators
- Deobfuscators
- Optimizers

## *Where can LUNIR be applied?*

Compilation speed with LUNIR will unavoidably be slower than with other compilers due to the high level of complexity in its architecture, it should not be employed in performance critical application.

LUNIR is an environment-aware code analysis tool that may be used with an IDE or linter to do high-quality analysis and type inference.

LUNIR is also quite useful for translating between various Lua source/bytecode versions. You can also utilize the IR to develop your own language translates into Lua at either the source or bytecode levels.

## *Can I use LUNIR comercially?*

Yes, LUNIR adheres to the MIT license, which allows for unrestricted commercial usage; the only requirement is that LUNIR be given attribution whenever it is utilized.

## *How can I contribute to LUNIR?*

Thank you for considering contribution, you may check our *[Contribution Guide](/CONTRIBUTING.md)* for details on how to contribute.

## *I found a bug, where do I report it?*

You can create an *[issue](../../issues)* detailing what the problem is and how to reproduce it.

If you're unsure whether something is a problem, create a *[topic on Github Discussions](../../discussions)* outlining your concerns.

## *What platforms are supported?*

LUNIR is written entirely in *[Rust](https://rust-lang.org/)*, it supports many targets and LUNIR's codebase does not use any platform specific code.

If Rust's standard library is supported, then so is LUNIR. You can see a detailed list of supported targets at Rust's documentation: *[Supported platforms](https://doc.rust-lang.org/stable/rustc/platform-support.html)*.
