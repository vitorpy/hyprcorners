Hyprcorners is a utility inspired by the similar Gnome extension. It allows you to trigger hyprland dispatchers when the cursor reaches a screen corner.

# Installation
You can download binaries for Linux from the releases page or install via cargo:
`cargo install hyprcorners`

# Configuration
The app will create some default config file in ~/.config/hypr directory called hyprcorners.toml. 
Keys:
- timeout (defaults to 50ms). This specifies how often will the app wake app and check your cursor position
- screen_width (defualts to 1920 pixels)
- screen_height (defaults to 1080 pixels)

There are also 4 tables:
- top_right
- top_left
- bottom_right
- bottom_left
Each table is associated with one of your corners and can have following keys:
- radius (defaults to 10 pixels)
- dispatcher (string, defaults to "workspace"). This is the name of the dispatcher that you want to call
- arg (string, defaults to ""). These are arguments that specified dispatcher accepts

All values and tables are optional or have default values.
