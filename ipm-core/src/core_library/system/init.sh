#!/bin/bash

echo "Initializing system..."

# Function to install local dependencies
install_local() {
    echo "Local installation..."
}

# Function to install global dependencies
install_global() {
    echo "Global installation..."
}

# Check if the script is run as superuser
if [ "$(id -u)" -ne 0 ]; then
    install_local
else
    install_global
fi
