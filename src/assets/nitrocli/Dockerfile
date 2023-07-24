# base image
FROM marlinorg/nitro-cli

# working directory
WORKDIR /app

COPY enclave-builder /app/builder/enclave-builder

# add files
COPY assets /app/assets
RUN mv assets/Dockerfile /app/Dockerfile
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x entrypoint.sh

# entry point
ENTRYPOINT [ "/app/entrypoint.sh" ]