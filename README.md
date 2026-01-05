# COSMIC Server Status Applet

A minimal COSMIC panel applet that pings a server every 30 seconds and shows status

## Installation

```bash
# Clone and build
git clone https://github.com/yourusername/cosmic-server-status-applet.git
cd cosmic-server-status-applet
cargo build --release

# Install
sudo cp target/release/server-status-applet /usr/lib/cosmic/applets/
# Or use `just install` if available

# Restart panel
pkill cosmic-panel

Then add it via COSMIC Settings → Desktop → Panel → + Add applet.
=======
# cosmic-server-applet
shows if my server is up
