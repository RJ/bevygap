## Bevygap NATS Setup

Now that `nats-cli` can connect to your NATS server, and we know it's working, let's ensure that the bevygap code can connect too.

### Bevygap Required Environment Variables

`bevygap_matchmaker`, `bevygap_httpd`,and the gameservers (via `bevygap_server_plugin`) need to connect to NATS.

The NATS connection code in `bevygap_shared` depends on the following environment variables to set up the NATS connection.

| Variable         | Required | Description                                                                                   |
| ---------------- | -------- | --------------------------------------------------------------------------------------------- |
| NATS_HOST        | Yes      | NATS server address<br><small>eg: `nats.example.com:4222` or `1.2.3.4`</small>                |
| NATS_USER        | Yes      | Username for NATS authentication                                                              |
| NATS_PASSWORD    | Yes      | Password for NATS authentication                                                              |
| NATS_CA          | No       | Path to CA root certificate for self-signed certs<br><small>eg: `/path/to/rootCA.pem`</small> |
| NATS_CA_CONTENTS | No       | Contents of the CA file<br><small>gets written to tmp file and used as NATS_CA</small>        |


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

The `bevygap_shared` crate has an example (non-bevy) program that connects to NATS and prints a success message then exits.
This will test that your environment variables are set correctly for bevygap:

```bash
$ cargo run -p bevygap_shared --example nats
     ...compiling...
     Running `target/debug/examples/nats`
2024-11-04T09:49:23.764924Z  INFO bevygap_shared: NATS: setting up, client name: bevygap_nats_test    
2024-11-04T09:49:23.765494Z  INFO bevygap_shared: NATS: TLS is enabled    
2024-11-04T09:49:23.765498Z  INFO bevygap_shared: NATS: connecting as 'matchmaker' to 1.2.3.4    
2024-11-04T09:49:23.765512Z  INFO bevygap_shared: NATS: using self-signed CA: /Users/rj/Library/Application Support/mkcert/rootCA.pem    
2024-11-04T09:49:23.777111Z  INFO bevygap_shared: 🟢 NATS: connected OK    
2024-11-04T09:49:23.777121Z  INFO async_nats: event: connected
NATS connected OK!

```

If you made it this far, you've got a working NATS setup. Now on to the fun stuff.


