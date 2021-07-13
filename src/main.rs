/*!
  Xorg input device grabbing utility

  Claims all input from an USB keyboard to use it's keys to perform utility functions
  for me, regardless of what's on my screen or what window has focus.

  made by Supersonic Tumbleweed to teach myself Rust while solving a real problem.
*/
mod inputs;
mod xorg_functionality;


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
            xorg_functionality::trigger_expo();
            let xdo = libxdo::XDo::new(Some(":0")).expect("xdo acquire failed");
            xdo.send_keysequence("Right", 0).expect("unable to press right key");
            xorg_functionality::trigger_expo();
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


#[test]
fn test_traverse_expo() {
    xorg_functionality::trigger_expo();
    let xdo = libxdo::XDo::new(Some(":0")).expect("xdo acquire failed");
    xdo.send_keysequence("Right", 0).expect("unable to press right key");
    xorg_functionality::trigger_expo();
}