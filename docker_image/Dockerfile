# FROM rust:1.63-buster
# Using 1.75.0-buster because of error: 
FROM rust:1.75-buster
# package `bumpalo v3.16.0` cannot be built 
# because it requires rustc 1.73.0 or newer, 
# while the currently active rustc version is 1.63.0
#FROM rust:1.73.0-buster
# install the necessary dependencies
# libxdo-dev is required for the xdo crate
# and remove the apt cache
RUN apt-get update && apt-get install -y \
    libxdo-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the maps directory to /filler/maps
COPY ./maps /filler/maps

# Copy the linux_robots directory to /filler/linux_robots
COPY ./linux_robots /filler/linux_robots

# Copy the solution directory to /filler/solution
#COPY ./solution /filler/solution

# Copy the Cargo.toml file to /filler/Cargo.toml
#COPY ./Cargo.toml /filler/Cargo.toml

# Copy the linux_game_engine to /filler/linux_game_engine
COPY ./linux_game_engine /filler/linux_game_engine

# Set the working directory to /filler
WORKDIR /filler/

# Set the entry point to /bin/bash
# This will allow us to run the container and play the game inside it
# by choosing the map and the players in command:
# 
ENTRYPOINT ["/bin/bash", "-c"]
#ENTRYPOINT ["cargo run", "./linux_game_engine", "-f", "maps/map01", "-p1", "linux_robots/bender", "-p2", "target/release/filler"]
#ENTRYPOINT ["cargo", "run", "./linux_game_engine", "-f", "maps/map01", "-p1", "linux_robots/bender", "-p2", "target/release/filler"]
#ENTRYPOINT ["cargo", "run", "./linux_game_engine", "-f", "maps/map01", "-p1", "linux_robots/bender", "-p2", "linux_robots/h2_d2", "RUST_BACKTRACE=1"]