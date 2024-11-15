#!/bin/sh

# Remove any existing container named fillercontainer
sudo docker rm -f fillercontainer

# Prune the system to remove all unused containers, networks, images, and optionally, volumes
sudo docker system prune -a -f

# Build the Docker image
sudo docker build -t filler .

# Run the Docker container and name it fillercontainer
sudo docker run --name fillercontainer -v "$(pwd)/solution":/filler/solution -it filler /bin/bash -c "
    cd /filler/solution &&
    cargo build --release &&
    ./linux_game_engine -f maps/map01 -p1 target/release/filler -p2 robots/bender 
"

# List Docker images
sudo docker image ls

# List all Docker containers
sudo docker ps -a

# Open a bash shell in the running container
sudo docker exec -it fillercontainer /bin/bash

# List files in the current directory
ls -l