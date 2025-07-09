use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    {AttributeSet, KeyCode, KeyEvent},
};
use std::io::ErrorKind;
use std::os::fd::AsRawFd;
use std::process::exit;

pub struct Clicker {
    device: VirtualDevice,
}

impl Clicker {
    pub fn new() -> Clicker {
        let mut keys = AttributeSet::<KeyCode>::new();
        keys.insert(KeyCode::KEY_LEFT);
        keys.insert(KeyCode::KEY_RIGHT);

        // TODO see what happen if uinput is not here
        #[allow(deprecated)]
        let device = match VirtualDeviceBuilder::new() {
            Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                println!("Permission error on /dev/uinput.");
                println!("Check the documentation for various workarounds.");
                exit(1);
            }
            Err(e) => {
                println!("Error: {e:?}");
                exit(1);
            }

            Ok(d) => d
                .name("Joycon2click virtual keyboard")
                .with_keys(&keys)
                .unwrap()
                .build()
                .unwrap(),
        };
        Clicker { device }
    }

    fn press_key(&mut self, keycode: KeyCode) {
        let down_event = *KeyEvent::new(keycode, 1);
        let up_event = *KeyEvent::new(keycode, 0);
        self.device.emit(&[down_event, up_event]).unwrap();
    }

    pub fn press_left(&mut self) {
        self.press_key(KeyCode::KEY_LEFT)
    }

    pub fn press_right(&mut self) {
        self.press_key(KeyCode::KEY_RIGHT)
    }

    #[allow(dead_code)]
    pub fn get_device_fd(&mut self) -> i32 {
        self.device.as_raw_fd()
    }
}

impl Default for Clicker {
    fn default() -> Self {
        Self::new()
    }
}
