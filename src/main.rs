mod xinputwrapper;

use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

use evdev_rs;

use regex;

const XINPUT_DEVICE_NAME: &str = "USB HCT Keyboard";

fn main() {
    let mut input_dev = {
        let input_dev_path = get_device_file_path().expect("can't figure out device path");
        let input_dev_file = File::open(input_dev_path).expect("can't open event device, try sudo");

        evdev_rs::Device::new_from_file(input_dev_file).expect("unable to open device")
    };

    input_dev
        .grab(evdev_rs::GrabMode::Grab)
        .expect("grabbing failed");
    println!("Grabbed!");

    

    sleep(Duration::from_secs(10));

    input_dev
        .grab(evdev_rs::GrabMode::Ungrab)
        .expect("grabbed but returning failed!");
    println!("Returned!");
}



fn get_device_file_path() -> Option<String> {
    use std::process::Command;

    let xinput = Command::new("xinput")
        .arg("--list")
        .output()
        .expect("xinput not found.");

    let regex = {
        let mut expression = String::from(XINPUT_DEVICE_NAME);
        expression.push_str("\\s+id=(\\d+)");

        regex::Regex::new(expression.as_str()).expect("regex compilation failed")
    };

    let xstdout = String::from_utf8(xinput.stdout).expect("output contains invalid seq");

    if let Some(res) = regex.captures(xstdout.as_str()) {
        let id_number_string = res.get(1).expect("no group in static regex?").as_str();
        Some("/dev/input/event".to_owned() + id_number_string)
    } else {
        None
    }
}

