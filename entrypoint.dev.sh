#!/bin/bash
set -e

echo "Running database migrations..."
diesel migration run

echo "Starting hxckr-core..."
exec hxckr-core