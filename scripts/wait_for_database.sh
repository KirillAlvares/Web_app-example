#!/bin/bash
cd ..
sudo docker-compose up -d

until pg_isready -h localhost -p 5432 -U username
do
    echo "Waiting for postgres"
    sleep 2;
done
    echo "docker is now running"
    sudo docker-compose down