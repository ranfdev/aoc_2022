# How to use
```
# install nix
# open an environment with all the needed tools
nix develop
# fetch the input file, create the binary file for the day from src/template.rs
cargo run --bin fetch_challenge 03
# Edit the file src/bin/d03.rs to solve the challenge.

# Get the output with 
cargo run --bin src/bin/d03.rs < inputs/03.txt
```
