# Cliprs - A clipboard Manager written in Rust

Provided is a Clipboard Manager service as well as a Front End GUI using GTK-4

This manager currently only works on systems running X11/xorg as their display and is primarily targeted towards Linux systems.

## Usage
You can easily build and install this application by running the install bash script:
```bash
cd ~
cd cliprs
git clone https://github.com/Adonca2203/cliprs.git
chmod -x install.sh
. ./install.sh
```

### Set up an User Level Service
There is a basic service file included in the repo. The install script can also automatically set this up if you answer
yes when prompted

If you wish to perform this step manuall you can follow these steps:
1. If not present, create the following directories `~/.config/systemd/user`
2. Create a file inside of `~/.config/systemd/user` called `cliprs.service`
3. Paste the following into it, replacing `ExecStart` with your full path
```
[Service]
Type=forking
ExecStart=/full/path/to/target/release/cliprs
Restart=always
RestartSec=1

[Unit]
Description=Clipboard Manager
StartLimitIntervalSec=0

[Install]
WantedBy=default.target
```
4. Run the following commands to enable (this makes it so it runs automatically on log in) and start it
```bash
systemctl --user enable cliprs.service
systemctl --user start cliprs.service
```

Once done, the service will automatically start up on user login to x and start logging your clipboard

### Set up GUI/GTK App
In order to view your clipboard history through a basic GUI you will need to build and run the binary in cliprs-gtk4.

It is recommended that you set it up so that you can run this easily via a hotkey for easiest access.

I currently use [sxhkd](https://github.com/baskerville/sxhkd) and have provided a basic user service file which can be added
to the same directory and using the same method as shown above.


# Known Limitations
1. Only able to log text (support for images/video maybe?)
2. Only able to log x11 (Planned support for Windows/MacOs/Wayland?)
