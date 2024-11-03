## Bevygap NATS Setup

Now that `nats-cli` can connect to your NATS server, and we know it's working, let's ensure that the bevygap code can connect too.

### Bevygap Required Environment Variables

`bevygap_matchmaker`, `bevygap_httpd`,and the gameservers (via `bevygap_server_plugin`) need to connect to NATS.

The NATS connection code in `bevygap_shared` depends on the following environment variables to set up the NATS connection.

| Variable      | Required | Description                                                                             |
| ------------- | -------- | --------------------------------------------------------------------------------------- |
| NATS_HOST     | Yes      | NATS server address, eg: `nats.example.com:4222` or `1.2.3.4`                           |
| NATS_USER     | Yes      | Username for NATS authentication                                                        |
| NATS_PASSWORD | Yes      | Password for NATS authentication                                                        |
| NATS_CA       | No       | Path to CA certificate for self-signed TLS cert verification, eg: `/path/to/rootCA.pem` |


### Create nats.env file

Back on your local machine, in the bevygap directory, copy `nats.env.example` to `nats.env`,
and edit it with your server's IP address, nats user, nats password, and path to CA certificate.

**nats.env:**
```
NATS_USER=matchmaker
NATS_PASSWORD=matchmaker
NATS_HOST=1.2.3.4
NATS_CA="/Users/rj/Library/Application Support/mkcert/rootCA.pem"
```

Our `docker-compose.yaml` file will apply these environment variables to containers we run, but we 
also want to set them in our shell, before we run (eg) the bevygap matchmaker service using `cargo run`.

```bash
# Setting environment variables in bash, on linux/mac
export NATS_USER=....
export NATS_PASSWORD=....
# Bash trick to set them from the .env file:
set -a && . ./nats.env && set +a
```

```
# How do you do this in windows? something like this maybe:
setx NATS_USER "matchmaker"
```

Verify your environment variables are set:
```bash
$ echo $NATS_USER
matchmaker # <-- your nats username should be printed here
```

#### The final test

The `bevygap_server_plugin` crate has an example (non-bevy) program that connects to NATS and prints a success message then exits.
This will test that your environment variables are set correctly for bevygap:

```bash
$ cargo run -p bevygap_server_plugin --example nats
NATS connected OK!
```

If you made it this far, you've got a working NATS setup. Now on to the fun stuff.


