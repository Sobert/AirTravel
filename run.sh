#!/bin/bash

PORT=$1

if [ -z "$PORT" ]
then
    echo "$0 <port>"
else
    ID=$(docker ps -f "name=air-travel-api" --format "{{.ID}}")
    ! [ -z "$ID" ] && echo "Killing previous instannce $ID" && docker kill $ID
    echo "build and run local/air-travel..."
    if O=$(docker build -t local/air-travel . && docker run -d --rm -ti -p $PORT:8080 --name air-travel-api local/air-travel)
    then
        docker ps -f "name=local/air-travel" --format "Container {{.ID}} with name {{.Names}} on ports {{.Ports}}"
        echo -e "API running on http://localhost:$PORT"
    fi

fi
