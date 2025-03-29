#!/bin/bash

if [ "$(id -u)" -ne 0 ]; then
    echo "You must run this script as root or with sudo."
    exit 1
fi

# Check if git is installed
if ! command -v git &> /dev/null; then
    echo "git is not installed. Please install git and try again."
    exit 1
fi

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "cargo is not installed. Please install Rust (which includes cargo) and try again."
    exit 1
fi

# Create IPM directory
mkdir -p /opt/the-infinitys/ipm
# Setup IPM directory structure
mkdir /opt/the-infinitys/ipm/source
mkdir /opt/the-infinitys/ipm/bin
mkdir /opt/the-infinitys/ipm/usr
mkdir /opt/the-infinitys/ipm/share
mkdir /opt/the-infinitys/ipm/share/packages
mkdir /opt/the-infinitys/ipm/share/packages/cache
mkdir /opt/the-infinitys/ipm/share/packages/installed

cd /opt/the-infinitys/ipm
git clone https://github.com/The-Infinitys/app.linux.ipm.git source
cd source
cd ipm-core
cargo build --release
cp target/release/ipm /opt/the-infinitys/ipm/bin
# Add IPM to PATH
IFS=':' read -ra PATH_DIRS <<< "$PATH"
for dir in "${PATH_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        ln -sf /opt/the-infinitys/ipm/bin/ipm "$dir/ipm"
    fi
done
echo "IPM has been installed successfully."
echo "You can now use the 'ipm' command to manage your packages."
