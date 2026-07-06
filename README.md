# Server Status Applet for COSMIC DE

A minimal panel applet that pings a server every 30 seconds and shows a status

![Applet Demo](demo.gif)

## Installation

# Clone and build
git clone https://github.com/yourusername/cosmic-server-status-applet.git
cd cosmic-server-status-applet
cargo build --release

# Install
sudo cp target/release/server-status-applet /usr/lib/cosmic/applets/

or use `just install` if available

Start the thing via COSMIC Settings → Desktop → Panel → Add applet.
