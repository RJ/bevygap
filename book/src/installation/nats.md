## NATS - what, why, and how.

There has to be a way to communicate between the matchmaker and the gameservers. I chose NATS, because
it's both a message bus and a key-value store, both of which are very handy for this kind of thing.

It also allows you to run the matchmaker and other components on your local dev machine if you like, connect them to the production NATS server, and test using real gameservers on edgegap. This saves the hassle of doing a full deploy during the development cycle.

In fact, this tutorial starts off with deploying a public NATS server, but running all the other components locally.
The NATS server has to be public because the gameservers (running on Edgegap) need to connect to it. Later on, we'll cover deploying the matchmaking stuff in containers on your server.

### Creating a self-signed TLS certificate for your NATS server

We'll be running NATS server using docker, and you'll need a TLS certificate for it.
The [NATS docs on TLS](https://docs.nats.io/running-a-nats-service/configuration/securing_nats/tls) are extensive, but here are the edited highlights:

### Making certifcates

Install [mkcert](https://github.com/FiloSottile/mkcert)

Find the directory where `mkcert` puts its CA certificate. On my mac, this looks like:

```bash
$ mkcert -CAROOT
/Users/rj/Library/Application Support/mkcert

# Check for the rootCA.pem file:
$ ls "/Users/rj/Library/Application Support/mkcert"
rootCA-key.pem  rootCA.pem # <-- this one
```

Now generate a certificate for your NATS server:

```bash
$ mkcert -cert-file nats-server-cert.pem -key-file nats-server-key.pem localhost ::1 nats
```

Make a `nats-config` directory, and copy the server certificate and key into it:

```bash
$ mkdir nats-config
$ mv nats-server-cert.pem nats-server-key.pem ./nats-config/
```

When we run the NATS server in docker, we'll ensure this `nats-config` directory is available at `/config` inside the container. With this in mind, we create a `nats-server.conf` file that references the server certificate, and creates some username/password pairs.

Don't forget to create real passwords before exposing this to the internet.

Create `nats-config/nats-server.conf`:
```
listen: 0.0.0.0:4222

authorization: {
    users: [
        {user: "matchmaker", password: "matchmaker"},
        {user: "matchmaker_httpd", password: "matchmaker_httpd"},
        {user: "gameserver", password: "gameserver"},
        {user: "webhook",    password: "webhook"},
    ]
}

tls {
  cert_file: "/config/nats-server-cert.pem"
  key_file:  "/config/nats-server-key.pem"
}

jetstream {
    # storage directory will be mapped for you by docker:
    store_dir: /data
    # 100MB = high but sane limits, which we don't expect to hit:
    max_memory_store: 104857600
    max_file_store: 104857600
}
```

For a NATS client to connect, they will need the CA cert which signed the NATS server certificate, ie the `rootCA.pem` file – to verify the server's certificate.

We'll ship this file in the gameserver docker image, and make it available to the matchmaker and other services that establish NATS connections.

> **Self-signed vs Trusted CAs**
> This can all be avoided if you use a certificate authority that is already trusted, such as LetsEncrypt. In my deployment, my NATS server reuses a certificate generated for my domain name by Traefik, which is already trusted by browsers and the NATS clients. No difference in 'how secure', and this tutorial will assume we use self-signed for now.


## Deploying the NATS server

Now you need to run a nats-server docker container on a machine that is publically accessible on the internet. Hopefully you have a cheap linux server you can use. Mine runs ubuntu. Doesn't matter, as long as you can install docker on it. The gameserver won't run here, just the nats, and matchmaking stuff (pretty lightweight).

```bash
# Connect to your remote server (which has a public IP address)
$ ssh myserver
```

Make a new directory to work in:
```bash
$ mkdir nats-bits
$ cd nats-bits
```
Create a `docker-compose.yaml` file with the following contents:
```
version: "3.5"
services:
  nats:
    ports:
      - "4222:4222"
    image: nats:2.10.21
    restart: unless-stopped
    command: "--config /config/nats-server.conf"
    volumes:
      - ./nats-config:/config
      - ./nats-data:/data
```

Note how the `volumes:` section maps your config directory to `/config`. An empty `nats-data` directory will be created for you automatically by docker.

Into your `nats-bits` directory on the remote server, copy your `nats-config` directory you made earlier.

Verify all the files are in the right place:

```bash
rj@myserver:~/nats-bits $ find .
.
./docker-compose.yaml
./nats-config
./nats-config/nats-server.conf
./nats-config/nats-server-key.pem
./nats-config/nats-server-cert.pem
```

Now you can start the server:
```bash
# NB !!! older docker installs use "docker-compose", newer use "docker compose"
#        so if one doesn't work, try the other.
#
# this starts up in detached mode (in the background)
$ docker compose up -d

# Check logs, Control-C to exit:
$ docker compose logs -f nats
```

You should now be able to connect to your NATS server from your local machine using the public IP address. Docker usually manages to manipulate the firewall for you, to make it work. Let's test that.



Install the [nats-cli](https://docs.nats.io/using-nats/nats-tools/nats_cli) tool, to allow you to examine the NATS bus and key-values while your system is running.

Create a [nats context to use with `nats-cli`](https://docs.nats.io/using-nats/nats-tools/nats_cli). Again, NATS docs are good, edited highlights follow:

```bash
# Change the server IP/hostname, the ca path, and the user/password to match your setup.
nats context save \
  --server="nats://1.2.3.4" \
  --description="My NATS server" \
  --user="matchmaker" \
  --password="matchmaker" \
  --tlsca="/Users/rj/Library/Application Support/mkcert/rootCA.pem" \
  --select \
  bevygap
```

Now you can use the `nats` command to examine the NATS bus and key-values.

```bash
$ nats server check connection
OK Connection OK:connected to nats://nats.example.com:4222 in 135.609292ms OK:rtt time 25.149083ms OK:round trip took 0.025140s | connect_time=0.1356s;0.5000;1.0000 rtt=0.0251s;0.5000;1.0000 request_time=0.0251s;0.5000;1.0000

```
Congratulations! You've now got a working NATS server, which you can connect to over the internet.

I suggest you try the [NATS pub/sub walkthrough](https://docs.nats.io/nats-concepts/core-nats/pubsub/pubsub_walkthrough) to get a feel for how the nats-cli tool works.

Next, we configure Bevygap so it can connect to your NATS server.