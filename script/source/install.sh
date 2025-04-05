#!/bin/bash

if [[ $EUID -ne 0 ]]; then
  echo "This script must be run as root. Please use sudo." >&2
  exit 1
fi
script_dir=$(dirname "$(readlink -f "$0")")
target_dir="/opt/ipm"
if [[ "$script_dir" != $target_dir ]]; then
  echo "The script must be located in /opt/ipm to proceed." >&2
  exit 1
fi

echo "Start Installing..."

for dir in $(echo $PATH | tr ':' '\n'); do
  if [[ -d "$dir" ]]; then
    ln -sf $target_dir/bin/ipm "$dir/ipm"
    echo "Linked ipm-binary to $dir/ipm"
  fi
done

echo "Installation Complete!"
