== Premise ==
=============

Welcome to Keyboard Control Panel, or **KeyCoP**

This program turns your second keyboard into your personal physical control panel.
It intercepts all events from specified device(s) and lets me program it to whatever I might want.

Currently it allows me to switch between my virtual desktops.

Additionally it will run executables at '~/.config/keycop/bind/just/<keyname>'
provided that they're there and execute permissions are set. For instance, to
trigger system reboot upon pressing key [,] ("comma") make a symbolic link to
your reboot binary as such:

    ln -s $(which reboot) ~/.config/keycop/bind/just/comma

More generally you can put any executable there, and it will spawn asynchronously
and will share the stdin, stdout and stderr with the **keycop** process

== Building ==
==============

Requires x11-devel (Fedora) or equivalent XOrg headers.
Requires libxdo-devel (Fedora) or equivalend libxdo headers.

== Running ==
=============

To run this with full functionality you need to:

- add your user to "input" group (Fedora) or whichever group grants you read permissions on /dev/input/event* files
- have a second keyboard plugged in
- change the id string of keyboard to your keyboard
- run compiz window decorator with both "Expo" and "D-Bus" plugins enabled
- have virtual desktops configured in a grid (currently hardcoded 2 rows by 3 columns)
- run the program, and keep it running

== TO DO ==
===========

- make keyboard identifier configurable
- add more functionality!
- add GUI configurator