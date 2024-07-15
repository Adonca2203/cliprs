# Cliprs - A clipboard Manager written in Rust

This logs your clipboard history to a txt file in /tmp and uses a gtk-4 based front-end to display it

The logging part is done and will be working on the front-end part
Currently only works for x11 but have plans of adding other OSs and maybe Wayland.

## Usage
```bash
cd ~
git clone https://github.com/Adonca2203/cliprs.git
cd cliprs/cliprs.service
cargo build --release
./target/release/cliprs
```

### Set up an User Level Service
You will want to create a user systemd service so that this runs automatically on user login.
To do so you can:
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
4. Run the following commands to enable and run it
```bash
systemctl --user enable cliprs.service
systemctl --user start cliprs.service
```

Once done, the service will automatically start up on user login to x and start logging your clipboard

# Known Limitations
1. Only able to log text (support for images/video maybe?)
2. Only able to log x11 (Planned support for Windows/MacOs/Wayland
