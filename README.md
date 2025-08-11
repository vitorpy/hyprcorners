Hyprcorners is a utility inspired by the similar Gnome extension. It allows you to trigger hyprland dispatchers when the cursor reaches a screen corner.

# Installation
You can download binaries for Linux from the releases page or install via cargo:
`cargo install hyprcorners`

# Configuration
The app will create a default config file in ~/.config/hypr directory called hyprcorners.toml. 

## Global Configuration
- `timeout` (defaults to 50ms): How often the app checks cursor position
- `sticky_timeout` (optional): If set, allows repeated triggering without leaving the corner (recommended: 300-500ms)

## Hot Corner Configuration

Hyprcorners now supports **per-monitor hot corners**! You can configure corners globally or for specific monitors.

### Global Configuration
Use the `[global]` section to define hot corners that apply to all monitors without specific configuration:

```toml
[global]
[global.top_right]
radius = 10
dispatcher = "workspace"
args = "e+1"

[global.bottom_left]
radius = 10
dispatcher = "workspace"
args = "e-1"
```

### Per-Monitor Configuration
Configure hot corners for specific monitors using their names (as shown by `hyprctl monitors`):

```toml
[monitors.DP-1]
[monitors.DP-1.top_left]
radius = 15
dispatcher = "exec"
args = "kitty"

[monitors.HDMI-A-1]
[monitors.HDMI-A-1.top_right]
radius = 20
dispatcher = "killactive"
args = ""
```

### Corner Properties
Each corner can have the following properties:
- `radius` (defaults to 10 pixels): Size of the hot corner area
- `dispatcher` (string, defaults to "workspace"): Hyprland dispatcher to trigger
- `args` (string, defaults to ""): Arguments for the dispatcher

### Example Configuration
See `hyprcorners.example.toml` for a complete example with multiple monitors.
