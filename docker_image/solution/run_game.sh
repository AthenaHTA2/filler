#!/bin/sh

# Change to the solution directory
cd /filler/solution

# Build the project
cargo build --release

# Run the game engine
#../linux_game_engine -f ../maps/map00 -p1 ../linux_robots/bender -p2 target/release/filler
#../linux_game_engine -f ../maps/map00 -p1 ../linux_robots/h2_d2 -p2 ../linux_robots/bender -r
../linux_game_engine -f ../maps/map00 -p1 target/release/filler -p2 ../linux_robots/bender RUST_BACKTRACE=1
#../linux_game_engine -f ../maps/map00 -p1 target/release/filler -p2 ../linux_robots/bender -r