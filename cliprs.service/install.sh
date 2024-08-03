#!/bin/bash

set -e

echo "Building package..."
cargo build --release

INSTALL_DIR=/usr/lib/cliprs
USER_SYSTEM_DIR=/home/$(whoami)/.config/systemd/user/

echo "Creating service directory"
sudo mkdir -p $INSTALL_DIR

echo "Directory created"

echo "Copying service binary"
yes | sudo cp -rf ./target/release/cliprs $INSTALL_DIR

read -p "Would you like the service to auto start on login? [yes/no]" RESP
RESP=${auto_start:-yes}

RESP=$RESP | tr '[:upper:]' '[:lower:]'

if [ $RESP = "y" ] || [ $RESP = "yes" ] || [ -z $RESP ]
then
  mkdir -p $USER_SYSTEM_DIR
  yes | cp -rf ../example-cliprs.service $USER_SYSTEM_DIR/cliprs.service
  
  systemctl --user stop cliprs.service
  systemctl --user disable cliprs.service
  systemctl --user daemon-reload
  systemctl --user enable cliprs.service
  systemctl --user start --no-block cliprs.service
else
  echo "Done installing"
fi
