# base image
FROM alpine:3.17

# install dependency tools
RUN apk add --no-cache net-tools iptables iproute2 wget

# working directory
WORKDIR /app

COPY mount /app/mount
COPY assets /app/assets

# entry point
RUN chmod +x /app/assets/entrypoint.sh
ENTRYPOINT /app/assets/entrypoint.sh
