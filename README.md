# Oyster Image Builder

Oyster image builder is a tool which can be used to build enclave images where enclave networking, attestation services and other such essential default services are setup.

## Configuration

```json
{
    "caddy": {
        "url": "",
        "caddyfile": ""
    },
    "params": {
        "ARCH": ""
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

The configuration to build the enclave image has to be provided in the above format. Oyster image builder provides default options like setting up a caddy server to serve https websites. 

Any assets which are necessary to build the enclave can be exposed to the builder by mounting them on `/app/mount` while running the docker image. The various options in the configuration are explained in detail below

### Caddy

Caddy is a web server that can serve static websites or act as reverse proxy to services and helps establish TLS connections to the client. We support caddy natively so that you can easily setup a website/service with TLS.

* `url`: By default base caddy with no plugins is installed if url is "". If you want to use a different version of caddy probably with plugins, you can specify the url here. You can find the download links at https://caddyserver.com/download.
* `caddyfile`: Caddyfile is the configuration file for caddy. You can find more information about caddyfile at https://caddyserver.com/docs/caddyfile. If caddyfile is "", caddy setup is skipped, otherwise caddy is setup with the caddyfile provided. The path to the caddyfile is relative to the mount folder in the volume specified.

### params

Params are any global parameters that can be passed while building the image. These can be used to parameterize the `build_commands` and `command` specified in the `service_commands` in the config. Examples of such parametrization can be found in [prebuilt](src/config/prebuilt/) configurations.

#### ARCH

ARCH is a special parameter used to specify the architecture to build the image for. This can be used as parameter while specifying the `service_commands` as done [here](src/config/prebuilt/base.json). Currently `amd64` and `arm64` are supported values for ARCH.

### Service commands

Service commands section is used to specify how to setup services or setup environments. Supervisor is internally used to setup and run services during the enclave execution.

#### name

Name used to setup the supervisor. This is used as identifier for the services.

#### command

Command added to supervisor to run on startup by supervisor. The variables specified in `params` section can be used in `command` section by accessing the variable using `{ParamName}`. This command is run when the supervisor startsup to setup various services during runtime of the enclave.

#### build_commands

Build command are an array of commands used to setup the environment for the `command` to run. The variables specified in `params` section can be used in `Build_command` section by accessing the variable using `{ParamName}`. Build commands are added the Dockerfile used to create the enclave image. So these commands are run during build of the enclave rather than during runtime.

#### ports

Ports are the array of ports to the setup for the service to access. This sets up the networking on the enclave to ensure that the services can communicate through the ports.

#### env

Env are the list of environmental variables to setup in the enclave. The variables specified here are added to the Dockerfile as ENV variables while building the enclave image.

## Need more help

In case of any questions or issues with the above. Please feel free to reach out by joining the discord server https://discord.gg/GSYCSq3myq.