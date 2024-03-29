<a href="https://github.com/urbanij/forvo-cli/actions"><img src="https://github.com/urbanij/forvo-cli/actions/workflows/rust.yml/badge.svg"></a>
<a href="https://crates.io/crates/forvo-cli"><img src="https://img.shields.io/crates/d/forvo-cli"></a>

# forvo-cli

Play [Forvo](https://forvo.com/) pronunciations straight from your terminal.

## Installation 

Install forvo-cli with cargo<sup>1</sup>

```bash 
cargo install forvo-cli
```
    
<!-- 
## Usage/Examples

Run basic demo example:

```bash
cargo run --example hello
``` 
-->

## Usage

```bash
forvo-cli -w <word> [-v]
```
e.g.:
```bash
forvo-cli -w torvalds -v
```



<!-- [![asciicast](https://asciinema.org/a/q48AMFx9rr00kzQDBHxnDQtel.svg)](https://asciinema.org/a/q48AMFx9rr00kzQDBHxnDQtel) -->

## Documentation

[Documentation](https://docs.rs/forvo-cli/0.1.2/forvolib/)

  

## Authors

- [@urbanij](https://www.github.com/urbanij)

## Notes

The audio is currently auto-played only on macOS. On other platforms they are just downloaded inside the `/tmp` folder.

---
<sup>1</sup> Requires the Rust toolchain (fairly lightweight) https://www.rust-lang.org/tools/install