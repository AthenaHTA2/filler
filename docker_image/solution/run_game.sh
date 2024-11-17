#!/bin/sh

# Change to the solution directory
cd /filler/solution

# Build the project
cargo build --release

# Run the game engine
../linux_game_engine -f ../maps/map00 -p1 target/release/filler -p2 ../linux_robots/bender