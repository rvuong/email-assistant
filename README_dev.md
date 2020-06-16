# README (dev)

This documentation aims to set up a working dev environment.

## Build the Docker image

```bash
docker-compose -f docker-compose_dev.yml build
```

## Environment variables

Copy & paste the file `.env.dist` into `.env`, so that it can be automatically used by Docker compose.

Then, edit and change the appropriate variable values.

eg.:

```
# .env

# ...

# The debug mode can be change to true.
# It changes specific actions, for example 
# maintaining email as Unread after being processed.
DEBUG=true

# ...
```

## Build & run the app

```bash
# At this stage, .env vars are transmitted to the container
docker-compose -f docker-compose_dev.yml run --service-ports app /bin/bash

# Direct build & run in the container
# env vars can be overridden:
DEBUG=true RUST_LOG=debug cargo run

# Build only
cargo build
```

Done!

Back to the main [README.md](README.md)
