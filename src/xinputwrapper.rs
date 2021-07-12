use std::{ops::Deref, process::Command, thread::panicking};

#[non_exhaustive]
enum XInputProp {
    DeviceEnabled,
    DeviceNode,
    DeviceProductID,
}

impl ToString for XInputProp {
    fn to_string(&self) -> String {
        match self {
            &Self::DeviceEnabled => "Device Enabled",
            &Self::DeviceNode => "Device Node",
            &Self::DeviceProductID => "Device Product ID"
        }
        .to_owned()
    }
}

impl XInputProp {
    fn is_present_in(self: Self, xinputline: &str) -> bool {
        xinputline.contains(self.to_string().as_str())
    }
}

pub struct BasicXInputDevice {
    device_name: String,
    enabled: bool,
    event_device_path: String,
}

impl BasicXInputDevice {
    fn new(device: &str) -> Self {
        get_device_props(device)
    }

    fn enable(self: &Self) -> Self {
        if Command::new("xinput").arg("--enable").arg(self.device_name.clone()).status().expect("xinput failed").success() {
            Self {
                device_name: self.device_name.clone(),
                enabled: true,
                event_device_path: self.event_device_path.clone()
            }
        } else {
            panic!("xinput failed");
        }
    }

    fn disable(self: &Self) -> Self {
        if Command::new("xinput").arg("--disable").arg(self.device_name.clone()).status().expect("xinput failed").success() {
            Self {
                device_name: self.device_name.clone(),
                enabled: false,
                event_device_path: self.event_device_path.clone()
            }
        } else {
            panic!("xinput failed");
        }
    }
}

pub fn get_device_props(device: &str) -> BasicXInputDevice{
    let output = Command::new("xinput")
        .arg("list-props")
        .arg(device)
        .output()
        .expect("unable to execute xinput");

    let output = String::from_utf8_lossy(output.stdout.as_ref());

    // We limit ourselves to only lines containing 'Device'
    let mut output = output.split("\n").filter(|line| line.contains("Device"));

    // First line contains the device name
    let device_name = output.next().expect("can't parse xinput output!");
    let device_name = {
        let mut device_name_iter = device_name.split("'");
        let _ = device_name_iter.next();
        device_name_iter
            .next()
            .expect("can't parse xinput device name")
    };

    // Rest of the output is device properties, one property per line
    let plucked = output.filter_map(|line| {
        if XInputProp::DeviceEnabled.is_present_in(line) {
            Some((
                XInputProp::DeviceEnabled,
                line.split(":").last().expect("expected value for Device Enabled"),
            ))
        } else if XInputProp::DeviceNode.is_present_in(line) {
            Some((
                XInputProp::DeviceNode,
                line.split(":").last().expect("expected value for Device Node"),
            ))
        } else if XInputProp::DeviceProductID.is_present_in(line) {
            Some((
                XInputProp::DeviceProductID,
                line.split(":").last().expect("expected value for Device Product ID"),
            ))
        } else {
            None
        }
    });

    let mut device_info = BasicXInputDevice {
        device_name: String::from(device_name),
        enabled: false,
        event_device_path: String::from(""), 
    };

    let _ : Vec<()> = plucked.map(|(prop,value)| {
        match prop {
            XInputProp::DeviceEnabled => device_info.enabled = value.contains('1'),
            XInputProp::DeviceNode => device_info.event_device_path = String::from(value.trim().trim_matches('"')),
            _ => ()
        };
    }).collect();

    device_info
}

#[test]
fn debug_device_info() {
    let device = get_device_props("USB HCT Keyboard");

    println!("Device enabled? {:?}", device.enabled);
    println!("Event device located at {:}", device.event_device_path);
}

#[test]
fn debug_device_by_id() {
    let device = get_device_props("18");

    println!("Device name is {:?}", device.device_name);
    println!("Device enabled? {:?}", device.enabled);
    println!("Event device located at {:}", device.event_device_path);
}

#[test]
fn test_toggle() {
    let device = get_device_props("USB HCT Keyboard");

    println!("Device enabled? {:?}", device.enabled);

    device.disable();
    let device = get_device_props("USB HCT Keyboard");

    println!("Device enabled? {:?}", device.enabled);
    
    device.enable();
    let device = get_device_props("USB HCT Keyboard");

    println!("Device enabled? {:?}", device.enabled);
}