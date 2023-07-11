# rain.metadata

## Specs

MetadataV1 spec - https://github.com/rainprotocol/specs/blob/main/metadata-v1.md

## Test
### Start Docker container
- docker-compose -f subgraph/docker/docker-compose.yaml up -d

### Run Forge Script
- cargo run --manifest-path metboard-cli/Cargo.toml test
