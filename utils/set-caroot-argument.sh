#!/bin/bash -e
appname=$1
appver=$2
cafile=$3

if [ ! -f "$cafile" ] || [ -z "$appname" ] || [ -z "$appver" ] ; then
	echo "Usage: $0 <appname> <appversion> <path to rootCA.pem file>"
	exit 1
fi

if [ -z "$EDGEGAP_API_KEY" ] ; then
	echo "Ensure your EDGEGAP_API_KEY is set"
	exit 2
fi

body="{\"arguments\": \"--ca_contents '$(cat "$cafile" | tr -d '\n')'\"}"

#echo "Setting body to: $body"
url="https://api.edgegap.com/v1/app/$appname/version/$appver"
#echo "url=$url"
echo "🔧 Sending PATCH command to $url"
curl -X PATCH "$url" -H "Content-Type: application/json" -H "Authorization: $EDGEGAP_API_KEY" -d "$body" -o -
