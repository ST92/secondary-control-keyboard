use evdev::{self, Device, InputEvent, InputEventKind, Key};

const XINPUT_DEVICE_NAME: &str = "USB HCT Keyboard";

fn main() {
    let mut keyboard: Vec<Device> = evdev::enumerate()
        .filter(|dev| dev.name().unwrap_or_default().eq(XINPUT_DEVICE_NAME))
        .map(|dev| {
            println!("Device {:}:", dev.name().unwrap_or_default());
            println!("\t{:?}", dev.supported_keys());
            println!("\t{:?}", dev.supported_leds());
            dev
        })
        .collect();
    let keyboard = keyboard.first_mut().expect("No matching keyboard");

    'event_handling: loop {
        let fetch = keyboard.fetch_events();
        if let Ok(event_iter) = fetch {
            let keys: Vec<InputEventKind> = event_iter
                .filter(is_key_press)
                .map(|event| event.kind())
                .collect();

            for eventkind in keys {
                if let InputEventKind::Key(key) = eventkind {
                    println!("{:?}", key);
                    if key == Key::KEY_ESC {
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
