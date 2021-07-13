/*!
  Xorg input device grabbing utility

  Claims all input from an USB keyboard to use it's keys to perform utility functions
  for me, regardless of what's on my screen or what window has focus.

  made by Supersonic Tumbleweed to teach myself Rust while solving a real problem.
*/
mod inputs;

use std::{time::Duration, u64};

use dbus::blocking::Connection;
use evdev::Key;
use inputs::{disable_device, get_keyboard, watch_keys, EventResult};

fn main() {
    let mut keyboard = get_keyboard();
    disable_device(&keyboard);
    watch_keys(&mut keyboard, &on_keypress, &on_keyrelease);
}

fn on_keypress(key: Key) -> EventResult {
    let mut do_exit = false;

    match key {
        Key::KEY_ESC => do_exit = true, // exit program
        Key::KEY_INSERT
        | Key::KEY_HOME
        | Key::KEY_DELETE
        | Key::KEY_END
        | Key::KEY_PAGEUP
        | Key::KEY_PAGEDOWN => { /* Switch to virtual desktop */ }
        Key::KEY_UP | Key::KEY_DOWN | Key::KEY_LEFT | Key::KEY_RIGHT => {
            /* Switch desktop relative to current */
            trigger_expo();
        }
        _ => {}
    };

    match do_exit {
        true => EventResult::Exit,
        false => EventResult::Continue,
    }
}

fn on_keyrelease(key: Key) -> EventResult {
    EventResult::Continue
}

const DBUS_COMPIZ_ROOT: &str = "org.freedesktop.compiz";
const DBUS_EXPO_KEY: &str = "/org/freedesktop/compiz/expo/allscreens/expo_key";

/// The command to do this from the command line is:
///
/// dbus-send --print-reply --type=method_call --dest=org.freedesktop.compiz
///  /org/freedesktop/compiz/expo/allscreens/expo_key
///  org.freedesktop.compiz.activate string:'root'
///  int32:`xwininfo -root | grep id: | awk '{ print $4 }'`
///
/// For sanity reasons, though, I'm using d-bus interface directly.
///
fn trigger_expo() {
    let conn = Connection::new_session().expect("D-Bus connection failed");
    let proxy = conn.with_proxy(DBUS_COMPIZ_ROOT, DBUS_EXPO_KEY, Duration::from_secs(3));

    let result: Result<(), dbus::Error> = proxy.method_call(
        "org.freedesktop.compiz",
        "activate",
        ("root", xorg_root_id() as i32),
    );

    println!("{:?}", result);

    result.expect("call failed");
}

use x11::xlib::{XOpenDisplay, XDefaultRootWindow};

const XORG_DISPLAY: &str = ":0";

fn xorg_root_id() -> u64 {
    let display = std::ffi::CString::new(XORG_DISPLAY).expect("wrong conversion");

    unsafe {
        let xdisplay = XOpenDisplay(display.as_ptr());

        let root_id = XDefaultRootWindow(xdisplay);

        root_id
    }
}

#[test]
fn test_expo_trigger() {
    println!("Triggering Expo");
    trigger_expo();
}
