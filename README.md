# Advent of Code 2024
## Setup
To download inputs, add your session cookie to ~/.config/adventofcode.session. The AoC integration is made through https://crates.io/crates/aoc-client.  
The inputs will be downloaded into input.txt, don't commit this file, as per AoC:s wishes.
## Create new day template
Use included makefile with:  
`make day=1`  
This will setup a new template for the day specified. It will also add two empty example files,
add example inputs to these files(do not commit them to repo...)
## Using the template
Examples are run as tests, run them through cargo:  
`cargo test part1` or `cargo test part2`, or run both with `cargo test`  
To run with the real input, simply use:  
`cargo run`  
in the folder of the day you wish to run.
