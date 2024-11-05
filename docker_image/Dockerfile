FROM rust:1.63-buster

# Copy the maps directory to /filler/maps
COPY ./maps /filler/maps

# Copy the linux_robots directory to /filler/linux_robots
COPY ./linux_robots /filler/linux_robots

# Copy the linux_game_engine to /filler/linux_game_engine
COPY ./linux_game_engine /filler/linux_game_engine

# Set the working directory to /filler
WORKDIR /filler/

# Set the entry point to /bin/bash
# This will allow us to run the container and play the game inside it
# by choosing the map and the players in command:
# ./game_engine -f maps/map01 -p1 target/release/filler -p2 robots/bender 
ENTRYPOINT /bin/bash
