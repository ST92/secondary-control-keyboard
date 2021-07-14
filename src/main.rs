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
        | Key::KEY_PAGEDOWN => {
            /* Switch to specific virtual desktop */
            xorg_functionality::trigger_expo();

            // My local geometry is 3 wide x 2 tall virtual desktops
            // this goes to top left one always
            expo_move_relative(Direction::Up);
            expo_move_relative(Direction::Left);
            expo_move_relative(Direction::Left);

            match key {
                Key::KEY_DELETE | Key::KEY_END | Key::KEY_PAGEDOWN => {
                    expo_move_relative(Direction::Down);
                }
                _ => (),
            }

            match key {
                Key::KEY_HOME | Key::KEY_END => expo_move_relative(Direction::Right),
                Key::KEY_PAGEUP | Key::KEY_PAGEDOWN => {
                    expo_move_relative(Direction::Right);
                    expo_move_relative(Direction::Right);
                }
                _ => (),
            }

            xorg_functionality::trigger_expo();
        }
        Key::KEY_UP | Key::KEY_DOWN | Key::KEY_LEFT | Key::KEY_RIGHT => {
            /* Switch desktop relative to current */
            xorg_functionality::trigger_expo();
            expo_move_relative(Direction::from(key));
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

enum Direction {
    Left,
    Right,
    Up,
    Down,
    Stay,
}

impl From<Key> for Direction {
    fn from(key: Key) -> Self {
        match key {
            Key::KEY_LEFT => Self::Left,
            Key::KEY_RIGHT => Self::Right,
            Key::KEY_UP => Self::Up,
            Key::KEY_DOWN => Self::Down,
            _ => Self::Stay,
        }
    }
}

impl Into<&str> for Direction {
    fn into(self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Up => "Up",
            Self::Down => "Down",
            _ => "",
        }
    }
}

// FIXME: maybe send multiple keys with one call?
fn expo_move_relative(direction: Direction) {
    let xdo = libxdo::XDo::new(Some(xorg_functionality::XORG_DISPLAY)).expect("xdo acquire failed");
    xdo.send_keysequence(direction.into(), 1000)
        .expect("unable to simulate key");
}

#[test]
fn test_traverse_expo() {
    xorg_functionality::trigger_expo();
    let xdo = libxdo::XDo::new(Some(xorg_functionality::XORG_DISPLAY)).expect("xdo acquire failed");
    xdo.send_keysequence("Right", 0)
        .expect("unable to press right key");
    xorg_functionality::trigger_expo();
}
