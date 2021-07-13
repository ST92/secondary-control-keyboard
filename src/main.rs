/*!
  Xorg input device grabbing utility

  Claims all input from an USB keyboard to use it's keys to perform utility functions
  for me, regardless of what's on my screen or what window has focus.

  made by Supersonic Tumbleweed to teach myself Rust while solving a real problem.
*/
mod inputs;

use evdev::Key;
use inputs::{disable_device, get_keyboard, watch_keys, EventResult};

fn main() {
    let mut keyboard = get_keyboard();
    disable_device(&keyboard);
    watch_keys(&mut keyboard, &on_keypress, &on_keyrelease);
}

fn on_keypress(key: Key) -> EventResult {
    let mut do_exit = false;

    /* Using the fact that the range covered is continuous, I preface multiple
      comparisons with a range check beforehand. The evdev scancodes are kind
      of... frantically laid out:
    
        KEY_HOME = 102,
        KEY_UP = 103,
        KEY_PAGEUP = 104,
        KEY_LEFT = 105,
        KEY_RIGHT = 106,
        KEY_END = 107,
        KEY_DOWN = 108,
        KEY_PAGEDOWN = 109,
        KEY_INSERT = 110,
        KEY_DELETE = 111,
    */
    match key {
        Key::KEY_ESC => do_exit = true, // exit program
        Key::HOME..Key::KEY_DELETE => match key {
        Key::KEY_INSERT
        | Key::KEY_HOME
        | Key::KEY_DELETE
        | Key::KEY_END
        | Key::KEY_PAGEUP
        | Key::KEY_PAGEDOWN => { /* Switch to virtual desktop */ },
        Key::KEY_UP 
        | Key::KEY_DOWN 
        | Key::KEY_LEFT 
        | Key::KEY_RIGHT => { /* Switch desktop relative to current */ },
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
