## Filler docker image instructions

- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

### Notes

- `Terminator` is a very strong robot so it's optional to beat him.

# How to run filler inside Docker

- Download & turn on the docker application on your local device

- Open VSC terminal within the filler folder and type either

```shell
bash ./run_filler.sh
``` 
or
```shell
./run_filler.sh
``` 
and when prompted press
```shell
y
``` 

Alternatively, the step-by-step commands are:

- In the terminal, run the command to create the image:
```shell
docker build -t filler .
```
- run the container and name it fillercontainer
```shell
docker run --name fillercontainer -v "$(pwd)/solution":/filler/solution -it filler
```
- Inside the Docker container, compile your AI:
```shell
cd /filler/solution
cargo build --release
```
- Run the game with your AI:
```bash
./game_engine -f maps/map01 -p1 target/release/filler -p2 robots/bender
```

To exit Docker in VSC terminal type: 
```shell
exit
```

#### Note to self: This is a draft Dockerfile
```Docker
FROM golang:1.18
RUN mkdir /forum
WORKDIR /forum
# Download necessary Go modules
COPY go.mod ./
COPY go.sum ./
# download all packages in mod file
RUN go mod download
# upload the entire 'forum' application
ADD . /forum
RUN go mod tidy
RUN cd /forum
# Next build a static application binary named 'binaryForum'
RUN go build -o binaryForum
# The port that connects to docker daemon
EXPOSE 8080
LABEL version="1.0"
LABEL description="Project forum Created by Sonal, Nathan, Kingsley, Helena"
# Tell Docker to execute the 'binaryForum' command when this image is used to start a container.
ENTRYPOINT [ "/forum/binaryForum" ]
```