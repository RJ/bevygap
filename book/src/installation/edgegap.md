# Edgegap Setup

Sign up for a free account at [Edgegap](https://edgegap.com).

Free accounts are limited to a single "deployment" (in Edgegap vernacular, a deployment is a single instance of your gameserver running on their infrastructure), which is fine for testing and development.

From the Edgegap dashboard, find your API key.

Put it in `edgegap.env`, like this:

**edgegap.env:**
```
EDGEGAP_API_KEY="token asjhgaskjdhasd-kjhasd-asd-asd-asd"
```





### Deploying a gameserver to Edgegap

Fork the [bevygap-spaceships](https://github.com/RJ/bevygap-spaceships/) repository into your own github account. 
This is the Lightyear "spaceships" example game, modified to run as a headless server and client that expects to connect using Webtransport (and is thus WASM-compatible).

To build the gameserver docker image with Github Actions, you must tell github some of your credentials.

Visit the "Container Registry" page on the Edgegap dashboard, to see the credentials you need to push
docker images to Edgegap's container registry.

On Github, go to the `Settings / Secrets and variables / Actions` page of your newly forked bevygap-spaceships repo, and add these secrets:

  | Secret Name             | Example value                                   |
  | ----------------------- | ----------------------------------------------- |
  | EDGEGAP_IMAGE_NAME      | metabrew-kfk5fha5fsct/bevygap-spaceships-server |
  | EDGEGAP_DOCKER_REGISTRY | registry.edgegap.com                            |
  | EDGEGAP_DOCKER_USERNAME | robot$metabrew-kfk5fha5fsct+client-push         |
  | EDGEGAP_DOCKER_PASSWORD | ....password from edgegap dashboard...          |

### Building the gameserver

To trigger the github action that builds the gameserver docker image and publishes it to Edgegap's container registry, either commit a new git tag, or visit the "Actions" page of the repo, select the "Build server" action, and click "Run workflow". Enter a version like `v1.0.0`.

At the time of writing, this was taking between 10 and 15 minutes to complete. This time can probably be reduced with some docker/github/caching trickery.

Once it completes, check the Edgegap container registry page to see that the image was published.

### Make an Edgegap Application

Now we configure an Edgegap Application.

In the Edgegap dashboard, under "Applications", create a new application called `bevygap-spaceships`. Set it to use the Edgegap container registry, select the image you published (eg: `youruser-abc123/bevygap-spaceships-server`), and choose the tag you used when building the server, eg `1.0.0`.

Other settings:

| Setting            | Value      |
| ------------------ | ---------- |
| Session Type       | Seat       |
| Session Sockets    | 6          |
| Empty Time to Live | 10 minutes |
| Auto Deploy        | Enabled    |

### Add a Port Mapping

You must tell Edgegap what port(s) your gameserver listens on. These will be mapped to a different external port for clients to use when connecting.

Add a port with these settings:

| Setting       | Value  |
| ------------- | ------ |
| Protocol      | UDP    |
| Port          | 6420   |
| Name          | server |
| Verifications | true   |
| TLS Upgrade   | false  |

Port 6420 is the port that `bevygap-spaceships` listens on. (ie: `0.0.0.0:6420`)

### Add Environment Variables

Set the env vars that bevygap needs to connect to NATS, and the lightyear key:

| Name                 | Value                     | Notes                      |
| -------------------- | ------------------------- | -------------------------- |
| NATS_USER            | gameserver                | from your nats-server.conf |
| NATS_PASSWORD        | gameserver                | from your nats-server.conf |
| NATS_HOST            | 1.2.3.4                   | Your NATS server public IP |
| LIGHYEAR_PRIVATE_KEY | [1, 2, 3, 4, 5, 6, ... 1] | From the game source       |


#### Providing the rootCA.pem file to the gameserver container

If you're using a self-signed certificate for your NATS server, you need to provide the CA root certificate to the gameserver, otherwise it won't be able to verify the NATS server's certificate.

Setting `NATS_CA` to the path to the rootCA.pem file works, but we didn't include the .pem file in the docker image.

Setting `NATS_CA_CONTENTS` to the contents of the rootCA.pem file would work, except Edgegap limits ENVs to 255 bytes (I've asked them to increase this limit!).

##### Slight Hack..
To work around this, `bevygap_server_plugin` looks for a `--ca_contents 'XXXXX'` flag on startup, and if found, will write the contents to a temporary file and pass that as `NATS_CA` for you. The Edgegap dashboard doesn't support setting docker arguments for server startup, but does support it via the API.

Use the `set-caroot-argument.sh` script to set the flag via the API:

```bash
$ ./utils/set-caroot-argument.sh 
Usage: ./utils/set-caroot-argument.sh <appname> <appversion> <path to rootCA.pem file>

$ ./utils/set-caroot-argument.sh "bevygap-spaceships" "1" "/Users/rj/Library/Application Support/mkcert/rootCA.pem"
🔧 Sending PATCH command to https://api.edgegap.com/v1/app/bevygap-spaceships/version/1

{"success":true,"version":{"arguments":"--ca_contents '-----BEGIN CERTIFICATE-----MIIE2zCCA0Og..snip...'...}}

✅ OK. Deployments of bevygap-spaceships at version 1 will have --ca_contents '<...contents...>' passed as arguments.
```

You need to do this for each version. During devlopment, I've been reusing the version and simply bumping the container tag associated with that application version. Don't be tempted to rely on a 'latest' docker tag though, Edgegap's caching doesn't like that. Make sure to specify a new version each time.

When adapting this process for your own gameserver, you could ship your rootCA.pem in the server's docker container,
in which case just set `NATS_CA=/path/to/rootCA.pem` as an Environment Variable in the Edgegap dashboard.

Alternatively, set up LetsEncrypt, get a trusted cert for your NATS server domain, and you won't need to provide the root CA file at all.

<small>
It's also possible to set up Edgegap "pull profiles" to yoink files from a configured S3 bucket on boot, but that is out of scope here..
</small>

### Deploying a gameserver

Once we have the server image published and the application version configured correctly, deployments should work.

Typically the matchmaker triggers a deployment for you, but to test, we can deploy manually, and verify the server starts up OK.

> **Why not watch NATS traffic for this bit?**
> <br>Subscribe to the everything wildcard in nats from your local machine.
> <br> A server starting up will publish something:
> <br>TODO: lookup the actual NATS topic i'm using instead of '>'
>```bash
>nats sub '>'
>```

Go to "Deployments" in the Edgegap dashboard, "Create Deployment", select our application and version, and click "Deploy".

The list of IP addresses are placeholders, and edgegap will use the geolocation of those IPs to figure out where in the world to run the server. Normally those are player IPs.

Once it starts, have a look at the "Container Logs" tab.  All being well, it will have connected to NATS and be waiting for clients to connect.

The "Deployment Details" also shows you the port mapping, so you can see the external port that maps to the internal 6420 port. You won't be able to connect a client to it just yet – the next step is the matchmaker service that issues valid Lightyear connect tokens.

Use the "Terminate Deployment" button to stop the server. The next time it starts up should be in response to a matchmaker request.


