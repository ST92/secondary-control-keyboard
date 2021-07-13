#!/bin/bash
dbus-send --print-reply --type=method_call --dest=org.freedesktop.compiz \
 /org/freedesktop/compiz/expo/allscreens/expo_key \
 org.freedesktop.compiz.activate string:'root' \
 int32:`xwininfo -root | grep id: | awk '{ print $4 }'`