#!/bin/bash

### Couldn't get progenitor to work
# cargo install cargo-progenitor

npm install @openapitools/openapi-generator-cli;

npx @openapitools/openapi-generator-cli generate -i immich-openapi-specs.json -g rust -o immich_client;

cd immich_client;

cargo build;
#EOF
