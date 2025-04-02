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

echo "Start Uninstalling..."


for dir in $(echo $PATH | tr ':' '\n'); do
  if [[ -d "$dir" ]]; then
    rm "$dir/ipm"
    echo "Removed ipm-binary link from $dir"
  fi
done

echo "Uninstallation Complete!"
