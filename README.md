# dot-desktop

Does one of two things:

1. If no argument is specified, it prints the names from all .desktop files known.
2. If an argument is specified, it prints the exec line of the first desktop entry whose name matched.

Configuration can be done with these environment variables:
- `DOTDESKTOP_APP`: Prefix for application instantiations, e.g. "swaymsg exec"
- `DOTDESKTOP_TERM`: Prefix for terminal instantiations, e.g. "alacritty -e"