#!/bin/bash
# This sets env vars like you would see if you booted up a server on edgegap infra.
# example values taken from edgegap docs

export ARBITRIUM_REQUEST_ID="f0000000000f"
export ARBITRIUM_DELETE_URL="https://api.edgegap.com/v1/self/stop/9f511e17/660"
export ARBITRIUM_DELETE_TOKEN="7df4cd933df87084b34ae80d8abde293"
export ARBITRIUM_DEPLOYMENT_LOCATION='{
  "city": "Montreal",
  "country": "Canada",
  "continent": "North America",
  "administrative_division": "Quebec",
  "timezone": "Eastern Time",
  "latitude": 45.513707,
  "longitude": -73.619073
}'
export ARBITRIUM_CONTEXT_URL="https://api.edgegap.com/v1/context/9170f5211e17/17"
export ARBITRIUM_CONTEXT_TOKEN="dfaf50b9333b9ee07b22ed247e4a17e6"
export ARBITRIUM_PUBLIC_IP="162.254.141.66"
export ARBITRIUM_PORTS_MAPPING='{
  "ports": {
    "5000": {
      "name": "game_port",
      "internal": 5000,
      "external": 31500,
      "protocol": "HTTP"
    }
  }
}'

# Verify that the environment variables are set
echo "Environment variables set:"
echo " ARBITRIUM_REQUEST_ID=$ARBITRIUM_REQUEST_ID"
echo " ARBITRIUM_DELETE_URL=$ARBITRIUM_DELETE_URL"
echo " ARBITRIUM_DELETE_TOKEN=$ARBITRIUM_DELETE_TOKEN"
echo " ARBITRIUM_DEPLOYMENT_LOCATION=$ARBITRIUM_DEPLOYMENT_LOCATION"
echo " ARBITRIUM_CONTEXT_URL=$ARBITRIUM_CONTEXT_URL"
echo " ARBITRIUM_CONTEXT_TOKEN=$ARBITRIUM_CONTEXT_TOKEN"
echo " ARBITRIUM_PUBLIC_IP=$ARBITRIUM_PUBLIC_IP"
echo " ARBITRIUM_PORTS_MAPPING=$ARBITRIUM_PORTS_MAPPING"

