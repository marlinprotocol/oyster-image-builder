# Oyster Image Builder

## Configuration

```json
{
    "caddy": {
        "url": "",
        "caddyfile": ""
    },
    "params": {
        "arch": ""
    },
    "service_commands": [
        {
            "name": "",
            "command": "",
            "build_commands": [
                ""
            ],
            "ports": [],
            "env": {}
        },
        ...
    ]
}
```

Oyster image builders provides default options like setting up a caddy server to serve https websites. 

### Caddy

Caddy is a web server that can serve static websites or act as reverse proxy to services and helps establish TLS connections to the client. We support caddy natively so that you can easily setup a website/service with TLS.

* `url`: By default base caddy with no plugins is installed if url is "". If you want to use a different version of caddy probably with plugins, you can specify the url here. You can find the download links at https://caddyserver.com/download.
* `caddyfile`: Caddyfile is the configuration file for caddy. You can find more information about caddyfile at https://caddyserver.com/docs/caddyfile. If caddyfile is "", caddy setup is skipped, otherwise caddy is setup with the caddyfile provided. The path to the caddyfile is relative to the mount folder in the volume specified.

### Volume

The mount folder which will contain all the assets needed to build the image has to be 