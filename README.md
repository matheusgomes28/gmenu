# GMenu - My Own dmenu Implementation

GMenu was written in an afternoon to solve the very annoying problem of having
a very simple TUI menu that can be configured and integrated into shell scripting.

Context: I wanted to copy the `omarchy` setup with it's simplistic and cool menus
triggered by actions on the `waybar`. However, due to installing `hyprland` on
a system that already had an app launcher, I did not want to install `walker`.

If you just want a simple `demnu` that can either be configured through a TOML file,
or options passed in from stdin, then this is for you.

**Note**: The font and styling are NOT concerns of `gmenu` - you can use your terminal
combined with `hyprland` to change fonts, placing and sizing of the menu.

## dmenu Mode

```bash
echo "Option One\nOption Two\nOption Three" | gmenu --dmenu --title "Ayo"
```

The above will simply return the selection.

## TOML Mode

You can use a TOML file to configure the options and what command to run for each
option selection.

Example of `config.toml`

```toml
[title]
name = "Power Settings"
icon = "some-image-path.jpg"

[[items]]
name = "shut-down"
text = "󰐥 Shut Down"
command = "echo"
args = ["Shutting Down..."]

[[items]]
name = "restart"
text = "󰤁 Restart"
command = "echo"
args = ["Restarting..."]

[[items]]
name = "lock"
text = " Lock Screen"
command = "hyprlock"

[[items]]
name = "screen-saver"
text = "󰹑 Screen Saver"
command = "echo"
args = ["Screen Saver..."]
```

Then just pass the config through `--config-file`:

```bash
gmenu --config-file config.toml
```
