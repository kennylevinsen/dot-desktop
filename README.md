# dot-desktop

Desktop file reader. Does one of two things:

1. If no argument is specified, it prints the names from all .desktop files known.
2. If an argument is specified, it prints the exec line of the first desktop entry whose name matched.

Configuration can be done with these environment variables:
- `DOTDESKTOP_APP`: Prefix for application instantiations, e.g. "swaymsg exec"
- `DOTDESKTOP_TERM`: Prefix for terminal instantiations, e.g. "alacritty -e"

## Examples

One-liner (list, select, find match, run):
```
eval "$(dot-desktop "$(dot-desktop | bemenu -i -p 'exec:')")"
```

Example wrapper that handles arbitrary scripts:
```
#!/bin/bash

selection="$(dot-desktop | bemenu -i -p 'exec:')"
if [ "$selection" != "" ]
then
    command_str="$(DOT_DESKTOP_APP="swaymsg exec" dot-desktop "${selection}")"
    if [ "$command_str" == "" ]
    then
        # No desktop file matched, raw command line
        command_str="${selection}"
    fi

    $command_str
    exit 0
fi

exit 1
```

## Which folders are read

- /usr/share/applications
- /usr/local/share/applications
- /var/lib/flatpak/exports/share/applications
- ${HOME}/.local/share/applications
- ${HOME}/.local/share/flatpak/exports/share/applications

Might be updated to handle some XDG stuff later.

## How to build

1. Clone the repo
2. `cargo build --release`
3. ???
4. Profit.
