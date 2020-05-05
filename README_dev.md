# README (dev)

This documentation aims to set up a working RUST/Rocket project environment.

## Build the Docker image

```bash
docker-compose -f docker-compose_dev.yml build
```

## Build & run the app

```bash
docker-compose -f docker-compose_dev.yml run --service-ports app /bin/bash

cargo build
cargo run
```

Done!
