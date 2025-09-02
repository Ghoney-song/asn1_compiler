# Project Overview

This project is a Rust-based compiler for ASN.1 specifications, named `rasn-compiler`. It is designed as a parser combinator that can parse ASN.1 specifications and generate bindings for various backends. The primary supported backends are Rust (for the `rasn` crate) and TypeScript (for JER-encoded data).

The project is structured as a Rust workspace containing the following packages:

-   `rasn-compiler`: The core compiler library.
-   `rasn-compiler-derive`: Proc-macros for the compiler.
-   `rasn-compiler-tests`: Integration and system tests.
-   `internal-macros`: Internal macros used by the compiler.

The compiler can be used as a library in `build.rs` scripts to generate bindings during the build process, or as a standalone CLI application. It also provides WebAssembly bindings for compiling ASN.1 in a web environment.

## Building and Running

The project is a standard Rust workspace. The following commands can be used for building and testing:

*   **Build:**
    ```bash
    cargo build
    ```
*   **Run tests:**
    ```bash
    cargo test
    ```
*   **Run the CLI:**
    The CLI can be run using the following command, after which you can run `./rasn_compiler_cli -h` for usage info:
    ```bash
    cargo run --features cli
    ```

## Development Conventions

The code follows standard Rust conventions. The compiler is designed to be extensible, allowing for the creation of custom backends by implementing the `Backend` trait. The project has a suite of tests in the `rasn-compiler-tests` package, which includes various ASN.1 specifications to ensure correctness.

## Current Goal

The current objective is to use `pdcp-sim` to parse standard ASN.1 files. The errors encountered during this process will be resolved by modifying the `rasn-compiler`. The ultimate goal is to achieve error-free parsing, thereby improving the stability and accuracy of the `rasn-compiler`.