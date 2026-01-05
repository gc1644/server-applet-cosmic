# COSMIC Server Status Applet

A minimal COSMIC panel applet that pings a server every 30 seconds and shows a status

![Applet Demo](demo.gif)

## Installation

# Clone and build
git clone https://github.com/yourusername/cosmic-server-status-applet.git
cd cosmic-server-status-applet
cargo build --release

# Install
sudo cp target/release/server-status-applet /usr/lib/cosmic/applets/
# Or use `just install` if available

# Restart panel
pkill cosmic-panel

Start the thing via COSMIC Settings → Desktop → Panel → + Add applet.
