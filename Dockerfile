# base image
FROM marlinorg/nitro-cli

# working directory
WORKDIR /app

# add files
COPY enclave-builder /app/builder/enclave-builder
RUN chmod +x /app/builder/enclave-builder
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x entrypoint.sh

# entry point
ENTRYPOINT [ "/app/entrypoint.sh" ]