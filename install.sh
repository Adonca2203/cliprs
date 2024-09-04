#!/bin/bash

set -e

echo "Building service package..."
cd ./cliprs.service && cargo build --release && cd ..

echo "------"

echo "Building gtk app..."
cd ./cliprs-gtk4 && cargo build --release && cd ..

INSTALL_DIR=/usr/lib/cliprs
USER_SYSTEM_DIR=/home/$(whoami)/.config/systemd/user/
AUTOSTART_DIR=/home/$(whoami)/.config/autostart/

echo "Creating service directory"
sudo mkdir -p $INSTALL_DIR

echo "Directory created"

echo "Copying binaries"
yes | sudo cp -rf ./cliprs.service/target/release/cliprs $INSTALL_DIR
yes | sudo cp -rf ./cliprs-gtk4/target/release/cliprs-gtk $INSTALL_DIR

read -p "Would you like the service to auto start on login? [yes/no] " RESP

if [ $RESP = "y" ] || [ $RESP = "yes" ] || [ -z $RESP ]
then
  mkdir -p $USER_SYSTEM_DIR
  mkdir -p $AUTOSTART_DIR
  yes | cp -rf ./example-cliprs.service $USER_SYSTEM_DIR/cliprs.service
  yes | cp -rf ./example-cliprs.desktop $AUTOSTART_DIR/cliprs.desktop
  
  systemctl --user stop cliprs.service
  systemctl --user daemon-reload
  systemctl --user start --no-block cliprs.service
else
  echo "Done installing"
fi
