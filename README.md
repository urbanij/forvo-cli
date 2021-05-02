# forvo-cli

Play [Forvo](https://forvo.com/) pronunciations straight from your terminal.




## Authors

- [@urbanij](https://www.github.com/urbanij)

  
## Badges

![](https://img.shields.io/crates/d/forvo-cli)
## Demo

`forvo-cli -v -w <some_word>`


[![asciicast](https://asciinema.org/a/q48AMFx9rr00kzQDBHxnDQtel.svg)](https://asciinema.org/a/q48AMFx9rr00kzQDBHxnDQtel)
## Documentation

[Documentation](https://docs.rs/forvo-cli/0.1.2/forvolib/)

  
## Installation 

Install forvo-cli with cargo

```bash 
cargo install forvo-cli
```
    
## Usage/Examples

Run basic demo example:

```bash
cargo run --example hello
```

## Notes

The audio is currently auto-played only on macOS. On the other platforms they are just downloaded into the `/tmp` folder.