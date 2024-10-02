#!/bin/sh
wget https://api.edgegap.com/swagger.json -O swagger.json
docker run \
	--rm -v $PWD:/local openapitools/openapi-generator-cli generate \
	-i ./swagger.json -g rust -o ./edgegap-client/
rm swagger.json
