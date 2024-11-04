// SPDX-License-Identifier: MIT

const VID_NINTENDO: u16 = 1406;
const PID_JOYCON_LEFT: u16 = 8198;
const PID_JOYCON_RIGHT: u16 = 8199;

use std::io::ErrorKind;
use std::process::exit;
use std::time::Duration;
use std::{process, thread};

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    {AttributeSet, KeyCode, KeyEvent},
};

use nix::unistd::{setuid, User};

use netlink_sys::{protocols::NETLINK_KOBJECT_UEVENT, Socket, SocketAddr};

use kobject_uevent::{ActionType, UEvent};

use clap::Parser;

// return 1 single joycon, since the code use `find`
// multiple joycon support is out of scope for now
fn get_joycon() -> Option<evdev::Device> {
    evdev::enumerate().map(|(_, d)| d).find(|d| {
        (d.input_id().product() == PID_JOYCON_RIGHT || d.input_id().product() == PID_JOYCON_LEFT)
            && d.input_id().vendor() == VID_NINTENDO
    })
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    user: Option<String>,

    #[arg(short, long)]
    debug: bool,
}

struct Clicker {
    device: VirtualDevice,
}

impl Clicker {
    fn new() -> Clicker {
        let mut keys = AttributeSet::<KeyCode>::new();
        keys.insert(KeyCode::BTN_DPAD_LEFT);
        keys.insert(KeyCode::BTN_DPAD_RIGHT);

        // TODO see what happen if uinput is not here
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

    fn press_left(&mut self) {
        self.press_key(KeyCode::BTN_DPAD_LEFT)
    }

    fn press_right(&mut self) {
        self.press_key(KeyCode::BTN_DPAD_RIGHT)
    }
}

fn wait_for_joycon() {
    let mut socket = Socket::new(NETLINK_KOBJECT_UEVENT).unwrap();
    let sa = SocketAddr::new(process::id(), 1);
    socket.bind(&sa).unwrap();

    loop {
        let (buf, _) = socket.recv_from_full().unwrap();
        //        let s = std::str::from_utf8(&buf).unwrap();
        let u = UEvent::from_netlink_packet(&buf).unwrap();
        if u.action == ActionType::Bind && u.subsystem == "hid" {
            match u.env.get("DRIVER") {
                Some(a) if a == "nintendo" => {
                    break;
                }
                Some(_) => {}
                None => {}
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let mut c = Clicker::new();

    if let Some(user) = cli.user {
        match User::from_name(&user) {
            Err(e) => {
                println!("Error: {e:?}");
                exit(1);
            }
            Ok(None) => {
                println!("User {user} do not exist, exiting");
                exit(1);
            }
            Ok(Some(u)) => {
                setuid(u.uid).expect("setuid");
                if cli.debug {
                    println!("Changed uid to {user}");
                }
            }
        }
    }

    loop {
        match get_joycon() {
            None => {
                if cli.debug {
                    println!("No joycon detected, entering loop");
                }
                wait_for_joycon();
                // time needed to make sure that the device appear in
                // /dev/input after wait_for_joycon return
                // (not sure why, but 2 sec is enough)
                thread::sleep(Duration::from_millis(2000));
            }

            Some(mut j) => {
                println!("Device found: {:?}", j.name());
                loop {
                    for ev in j.fetch_events().unwrap() {
                        match ev.destructure() {
                            evdev::EventSummary::Key(_, k, 1) => {
                                if cli.debug {
                                    println!("{k:?}");
                                }
                                match k {
                                    KeyCode::BTN_DPAD_LEFT | KeyCode::BTN_WEST => c.press_left(),
                                    KeyCode::BTN_TR
                                    | KeyCode::BTN_TR2
                                    | KeyCode::BTN_DPAD_RIGHT
                                    | KeyCode::BTN_EAST => {
                                        c.press_right();
                                    }
                                    _ => {
                                        println!("Key: {k:?}")
                                    }
                                }
                            }
                            _ => {
                                if cli.debug {
                                    println!("Event: {ev:?}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
