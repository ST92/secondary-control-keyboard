/*!
  Xorg input device grabbing utility

  Claims all input from an USB keyboard to use it's keys to perform utility functions
  for me, regardless of what's on my screen or what window has focus.

  made by Supersonic Tumbleweed to teach myself Rust while solving a real problem.
*/
mod inputs;

use evdev::Key;
use inputs::{get_keyboard, disable_device, watch_keys};

fn main() {
    let mut keyboard = get_keyboard();
    disable_device(&keyboard);
    watch_keys(&mut keyboard, &on_keypress, &on_keyrelease);
}

fn on_keypress(key: Key) {

}

fn on_keyrelease(key: Key) {

}

