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

mod config {

    ///
    /// The keyboard device name in question. It might be different, and most likely
    /// will be different on any system other than mine.
    ///
    pub const KEYBOARD_IDENTIFIER: &str = "USB HCT Keyboard";

    ///
    /// XOrg display identifier.
    /// :0 means first screen on local machine.
    ///
    pub const XORG_DISPLAY: &str = ":0";

    ///
    /// Configuration resides here, mostly bindings.
    /// CONFIG_DIR/bind/just/a will be executed on pressing A
    ///
    /// TODO::
    /// CONFIG_DIR/bind/alt/a will be executed on pressing ALT+A
    /// CONFIG_DIR/bind/ctrl/a will be executed on pressing CTRL+A
    /// CONFIG_DIR/bind/shift/a will be executed on pressing SHIFT+A
    /// pressing CTRL+ALT+A will call both ./alt/a and ./ctrl/a
    ///
    pub const CONFIG_DIR: &str = ".config/keycop";

    ///
    /// Print additional info?
    ///
    pub const DEBUG_INFO: bool = true;
}

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
            expo_goto_specific_by_key(key);
        }
        Key::KEY_UP | Key::KEY_DOWN | Key::KEY_LEFT | Key::KEY_RIGHT => {
            expo_move_relative(Direction::from(key));
        }

        // Limit keys to main block - "`1234[...]QWERTY[...]VBNM,./"
        Key(code) if code > Key::KEY_ESC.code() && code <= Key::KEY_SLASH.code() => {
            // TODO: precompute this and lookup
            let key_name = format!("{:?}", key);
            let key_name = key_name.strip_prefix("KEY_").unwrap();

            trigger_bind("just", key_name);
        }

        _ => {}
    };

    match do_exit {
        true => EventResult::Exit,
        false => EventResult::Continue,
    }
}

fn trigger_bind(key_set: &str, key_name: &str) -> () {
    let key_name = key_name.to_ascii_lowercase();
    println!("Triggering {:}/{:}", key_set, key_name);

    let mut path = dirs::home_dir().expect("failed to acquire home directory path");
    path.extend(vec![crate::config::CONFIG_DIR, "bind", &key_set, &key_name]);

    match std::process::Command::new(path).spawn() {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => {
            eprintln!("{:?}", e)
        }
    }
}

///
/// Switch to specific virtual desktop
/// Keys map according to the usual physical layout of
///  INS, DEL, HOME, END, PGUP, PGDN block in a 3x2 grid
///
fn expo_goto_specific_by_key(key: Key) -> () {
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

fn on_keyrelease(_key: Key) -> EventResult {
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
fn xdo_send_arrow_key(direction: Direction) {
    let xdo = libxdo::XDo::new(Some(crate::config::XORG_DISPLAY)).expect("xdo acquire failed");
    xdo.send_keysequence(direction.into(), 1000)
        .expect("unable to simulate key");
}

///
/// Switch desktop relative to current
///
fn expo_move_relative(direction: Direction) {
    xorg_functionality::trigger_expo();
    xdo_send_arrow_key(direction);
    xorg_functionality::trigger_expo();
}

#[test]
fn test_traverse_expo() {
    xorg_functionality::trigger_expo();
    let xdo = libxdo::XDo::new(Some(crate::config::XORG_DISPLAY)).expect("xdo acquire failed");
    xdo.send_keysequence("Right", 0)
        .expect("unable to press right key");
    xorg_functionality::trigger_expo();
}

#[test]
fn test_trigger_bind() {
    trigger_bind("just", "a");
}
