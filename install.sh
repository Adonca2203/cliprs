#!/bin/bash

set -e

echo "Building service package..."
cd ./cliprs.service && cargo build --release && cd ..

echo "------"

echo "Building gtk app..."
cd ./cliprs-gtk4 && cargo build --release && cd ..

INSTALL_DIR=/usr/lib/cliprs
USER_SYSTEM_DIR=/home/$(whoami)/.config/systemd/user/

echo "Creating service directory"
sudo mkdir -p $INSTALL_DIR

echo "Directory created"

echo "Copying binaries"
yes | sudo cp -rf ./cliprs.service/target/release/cliprs $INSTALL_DIR
yes | sudo cp -rf ./cliprs-gtk4/target/release/cliprs-gtk $INSTALL_DIR

read -p "Would you like the service to auto start on login? [yes/no]" RESP

if [ $RESP = "y" ] || [ $RESP = "yes" ] || [ -z $RESP ]
then
  mkdir -p $USER_SYSTEM_DIR
  yes | cp -rf ./example-cliprs.service $USER_SYSTEM_DIR/cliprs.service
  
  systemctl --user stop cliprs.service
  systemctl --user disable cliprs.service
  systemctl --user daemon-reload
  systemctl --user enable cliprs.service
  systemctl --user start --no-block cliprs.service
else
  echo "Done installing"
fi
