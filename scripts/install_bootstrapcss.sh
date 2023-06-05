#!/bin/bash

# Install Bootstrap CSS
echo "Installing Bootstrap CSS..."
cd $CARGO_MANIFEST_DIR/style
curl -o "bootstrap.min.css" https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css
cd -
echo "Bootstrap CSS installed"
