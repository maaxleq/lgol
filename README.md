# Lgol

Lgol is a simple program that simulates the [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life). It is written in Rust and uses [Termion](https://github.com/redox-os/termion) for rendering the grid in the terminal.

# Building and running

## Prerequisite

You will need the [Rust toolchain](https://www.rust-lang.org/tools/install) in order to build the project.

## Fast run

To run the project quickly, enter the following command:

    cargo run

This will compile the project without optimization and run it.

## Building

To build the project, enter the following command:

    cargo build --release

This will compile the project. The executable binary will be found in **target/release**.

# Usage

    lgol [help] [ARGUMENTS]
    
Entering **help** as an argument will display a help message and the simulation will not launch.

Here is a list of arguments that can be passed to the program to customize the simulation:

    width=[number]		Set the width of the grid, default is 20
    height=[number]		Set the height of the grid, default is 20
    delay=[number]		Set the interval in milliseconds between each iteration, default is 500
    alive=[char]		Set the character used for alive cells, default is O
    dead=[char]			Set the character used for dead cells, default is '
