FROM rust:latest

# Install Trunk
RUN cargo install trunk

# Create a working directory
WORKDIR /app

# Copy local files to the container
COPY . .

# Build the project using Trunk
RUN trunk build --release

CMD ["trunk", "serve"]

CMD ["-port","8080","-https-promote", "-enable-logging"]
