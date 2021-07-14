use evdev::{self, Device, InputEvent, InputEventKind, Key};

///
/// Pick out the secondary keyboard (as opposed to a main keyboard) from the
/// xinput device list.
///
pub(crate) fn get_keyboard() -> Device {
    

    let mut keyboard: Vec<Device> = evdev::enumerate()
        .filter(|dev| dev.name().unwrap_or_default().eq(crate::config::KEYBOARD_IDENTIFIER))
        .collect();

    let keyboard = keyboard.pop().expect("No matching keyboards!");

    keyboard
}

///
/// Uses `xinput` tool to disable Xorg from handling any events from specified
/// device. Here it's used to allow full control of how we react to presses
/// on the secondary keyboard.
///
pub(crate) fn disable_device(device: &Device) {
    if std::process::Command::new("xinput")
        .arg("--disable")
        .arg(device.name().unwrap_or_default())
        .status()
        .is_err()
    {
        println!("Unable to disable device.\nThe program will continue, but the events will reach the system too.")
    };
}

pub enum EventResult {
    Continue,
    Exit
}

///
/// Synchronously handles each keypress, delegating it to appropriate handlers.
/// 
pub(crate) fn watch_keys(keyboard: &mut Device, keypress_handler : &dyn Fn(Key) -> EventResult,  _keyrelease_handler : &dyn Fn(Key) -> EventResult) {
    'event_handling: loop {
        let fetch = keyboard.fetch_events();
        if let Ok(event_iter) = fetch {
            let keys: Vec<InputEventKind> = event_iter
                .filter(is_key_press)
                .map(|event| event.kind())
                .collect();

            for eventkind in keys {
                if let InputEventKind::Key(key) = eventkind {

                    // TODO: make it a compile-time check?
                    if crate::config::DEBUG_INFO {
                        println!("{:?}", key);
                    }
                    
                    if let EventResult::Exit = keypress_handler(key) {
                        break 'event_handling;
                    }
                }
            }
        }
    }
}

fn is_key_press(e: &InputEvent) -> bool {
    (match e.kind() {
        InputEventKind::Key(_) => true,
        _ => false,
    }) && (e.value() > 0)
}
