#!/bin/bash

# configuration
SPECS_DIR="docs"
BASE_URL="https://raw.githubusercontent.com/FIS2425"

mkdir -p "$SPECS_DIR"

# Just list services one per line
services=(
    appointment
    authorization
    history
    patient
    payment
    staff
    workshift
    logger
)

for svc in "${services[@]}"; do
    echo "fetching $svc"
    curl -o "$SPECS_DIR/$svc-openapi.yaml" \
         -L "$BASE_URL/$svc-svc/refs/heads/main/openapi.yaml" \
         -f -s && echo "✓ $svc" || echo "✗ failed: $svc"
done

date > "$SPECS_DIR/.last_updated"
