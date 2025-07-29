use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    {AttributeSet, KeyCode, KeyEvent},
};
use std::io;
use std::io::ErrorKind;
use std::os::fd::AsRawFd;

pub struct Clicker {
    device: VirtualDevice,
}

impl Clicker {
    pub fn new() -> io::Result<Clicker> {
        let mut keys = AttributeSet::<KeyCode>::new();
        keys.insert(KeyCode::KEY_LEFT);
        keys.insert(KeyCode::KEY_RIGHT);

        // TODO see what happen if uinput is not here
        #[allow(deprecated)]
        let device = match VirtualDeviceBuilder::new() {
            // TODO could be improved
            Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                eprintln!("Permission error on /dev/uinput.");
                eprintln!("Check the documentation for various workarounds.");
                return Err(e);
            }
            Err(e) => {
                eprintln!("Error: {e:?}");
                return Err(e);
            }

            Ok(d) => d
                .name("Joycon2click virtual keyboard")
                .with_keys(&keys)
                .unwrap()
                .build()
                .unwrap(),
        };
        Ok(Clicker { device })
    }

    fn press_key(&mut self, keycode: KeyCode) -> Result<(), std::io::Error> {
        self.device.emit(&[
            // down event
            KeyEvent::new(keycode, 1).into(),
            // up event
            KeyEvent::new(keycode, 0).into(),
        ])
    }

    pub fn press_left(&mut self) -> Result<(), std::io::Error> {
        self.press_key(KeyCode::KEY_LEFT)
    }

    pub fn press_right(&mut self) -> Result<(), std::io::Error> {
        self.press_key(KeyCode::KEY_RIGHT)
    }

    #[allow(dead_code)]
    pub fn get_device_fd(&mut self) -> i32 {
        self.device.as_raw_fd()
    }
}
