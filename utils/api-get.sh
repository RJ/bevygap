#!/bin/bash -ex
curl -X GET "https://api.edgegap.com$1" \
     -H "Content-Type: application/json" \
     -H "Authorization: $EDGEGAP_API_KEY" \
     -o -
