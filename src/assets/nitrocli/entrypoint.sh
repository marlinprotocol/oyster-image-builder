#!/bin/sh

dockerd &

sleep 10

/app/builder/enclave-builder --config /app/mount/config.json

docker image build -t enclave:latest .
mkdir -p /app/mount/enclave
nitro-cli build-enclave --docker-uri enclave:latest --output-file /app/mount/enclave/enclave.eif