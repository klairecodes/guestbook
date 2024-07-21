#!/bin/bash
# This script quickly starts a very basic PostgreSQL container using Podman.

if ! command -v <podman> &> /dev/null
then
    echo "<podman> could not be found"
    exit 1
fi


podman run --name postgres -e POSTGRES_USER=username -e POSTGRES_PASSWORD=password -p 5432:5432 -v /var/lib/data -d postgres
