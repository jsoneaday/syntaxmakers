#!/bin/bash

rm -rf dbdata
docker compose down --rmi 'all' --remove-orphans
docker compose up -d --build