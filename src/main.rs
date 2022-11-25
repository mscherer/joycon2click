// SPDX-License-Identifier: MIT

const VID_NINTENDO: u16 = 1406;
const PID_JOYCON_LEFT: u16 = 8198;
const PID_JOYCON_RIGHT: u16 = 8199;

use evdev::Key;
use nix::errno::Errno;
use std::process::exit;
use std::time::{Duration, Instant};
use uinput::event::keyboard::Key::{Left, Right};

fn get_joycon() -> Option<evdev::Device> {
    evdev::enumerate().map(|(_, d)| d).find(|d| {
        (d.input_id().product() == PID_JOYCON_RIGHT || d.input_id().product() == PID_JOYCON_LEFT)
            && d.input_id().vendor() == VID_NINTENDO
    })
}

struct Clicker {
    last_press: Instant,
    device: uinput::Device,
}

impl Clicker {
    fn new() -> Clicker {
        // TODO check error with
        //    Err` value: Nix(Sys(EACCES))  => perms are wrong
        let ui = match uinput::default() {
            Err(uinput::Error::NotFound) => {
                println!("module uinput is not loaded");
                exit(1);
            }
            Err(uinput::Error::Nix(nix::Error::Sys(Errno::EACCES))) => {
                println!("incorrect permissions on /dev/uinput");
                exit(1);
            }
            Err(a) => {
                println!("{:#?}", a);
                println!("unknown error");
                exit(1);
            }
            // TODO see if there is a less ugly code
            Ok(u) => u
                .name("joycon2click")
                .unwrap()
                .event(uinput::event::Keyboard::All)
                .unwrap()
                .create()
                .unwrap(),
        };

        Clicker {
            last_press: Instant::now(),
            device: ui,
        }
    }

    fn press_key(&mut self, key: uinput::event::keyboard::Key) {
        if self.last_press.elapsed() >= Duration::from_millis(1000) {
            self.device.click(&key).unwrap();
            self.last_press = Instant::now();
            self.device.synchronize().unwrap()
        }
    }

    fn press_left(&mut self) {
        self.press_key(Left)
    }

    fn press_right(&mut self) {
        self.press_key(Right)
    }
}

fn main() {
    let mut c = Clicker::new();

    match get_joycon() {
        None => println!("No joycon detected"),

        Some(mut j) => {
            println!("Device found: {:?}", j.name());
            loop {
                for ev in j.fetch_events().unwrap() {
                    match ev.kind() {
                        evdev::InputEventKind::Key(k) => {
                            println!("{:?}", k);
                            match k {
                                Key::BTN_DPAD_LEFT | Key::BTN_WEST => c.press_left(),
                                Key::BTN_TR
                                | Key::BTN_TR2
                                | Key::BTN_DPAD_RIGHT
                                | Key::BTN_EAST => {
                                    c.press_right();
                                }
                                _ => {
                                    println!("{:?}", k)
                                }
                            }
                        }
                        k => println!("{:?}", k),
                    }
                }
            }
        }
    }
}
