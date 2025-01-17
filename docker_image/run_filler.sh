#!/bin/sh

# Remove any existing container named fillercontainer
sudo docker rm -f fillercontainer

# Prune the system to remove all unused containers, networks, images, and optionally, volumes
sudo docker system prune -a -f

# Build the Docker image
sudo docker build -t filler .

# Run the Docker container and name it fillercontainer
sudo docker run --name fillercontainer -v "$(pwd)/solution":/filler/solution -it filler /filler/solution/run_game.sh
    #cd /filler/solution && \
    #cargo build --release && \
    #./linux_game_engine -f maps/map01 -p1 target/release/filler -p2 linux_robots/bender
#sudo docker run --name fillercontainer -v "$(pwd)/solution":/filler/solution -it filler
#sudo docker run --name fillercontainer -v "$(pwd)/solution":/filler/solution -it filler /bin/bash -c "
#sudo docker run --name fillercontainer -v "$(pwd)/solution":/filler/solution -it filler /bin/bash -c " 
#   cd /filler/solution
#   cargo build --release
#   ./linux_game_engine -f maps/map00 -p1 linux_robots/bender -p2 linux_robots/h2_d2 RUST_BACKTRACE=1
#"

#./linux_game_engine -f maps/map00 -p1 linux_robots/bender -p2 solution/target/release/filler RUST_BACKTRACE=1
#./linux_game_engine -f maps/map00 -p1 solution/target/release/filler -p2 linux_robots/bender RUST_BACKTRACE=1
# List Docker images
#sudo docker image ls

# List all Docker containers
#sudo docker ps -a

# Open a bash shell in the running container
#sudo docker exec -it fillercontainer /bin/bash

# List files in the current directory
#ls -l