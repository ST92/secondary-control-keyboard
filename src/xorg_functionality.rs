use std::{time::Duration, u64};

use dbus::blocking::Connection;

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
pub fn trigger_expo() {
    let conn = Connection::new_session()
        .expect("D-Bus connection failed");

    let proxy = conn.with_proxy(DBUS_COMPIZ_ROOT, DBUS_EXPO_KEY, Duration::from_secs(3));

    let result: Result<(), dbus::Error> = proxy.method_call(
        DBUS_COMPIZ_ROOT,
        "activate",
        ("root", xorg_root_id() as i32),
    );

    result.expect("call failed");
}

///
/// Retrieve root window identifier from X11 runtime
///
fn xorg_root_id() -> u64 {
    use x11::xlib::{XOpenDisplay, XRootWindow};

    let display = std::ffi::CString::new(crate::config::XORG_DISPLAY).expect("wrong conversion");

    unsafe {
        let xdisplay = XOpenDisplay(display.as_ptr());

        let root_id = XRootWindow(xdisplay, 0);

        root_id
    }
}

#[test]
fn test_expo_trigger() {
    println!("Triggering Expo");
    trigger_expo();
}