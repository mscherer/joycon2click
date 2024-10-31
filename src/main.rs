// SPDX-License-Identifier: MIT

const VID_NINTENDO: u16 = 1406;
const PID_JOYCON_LEFT: u16 = 8198;
const PID_JOYCON_RIGHT: u16 = 8199;

use netlink_sys::{protocols::NETLINK_KOBJECT_UEVENT, Socket, SocketAddr};
use std::process;

use kobject_uevent::ActionType;
use kobject_uevent::UEvent;

use clap::Parser;
use evdev::KeyCode;
use nix::errno::Errno;
use nix::unistd::{setuid, User};
use std::process::exit;
use std::thread;
use std::time::Duration;
use uinput::event::keyboard::Key::{Left, Right};

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
    device: uinput::Device,
}

impl Clicker {
    fn new() -> Clicker {
        let ui = match uinput::default() {
            Err(uinput::Error::NotFound) => {
                println!("Module uinput is not loaded.");
                println!("Run 'modprobe uinput' as root to fix.");
                exit(1);
            }
            Err(uinput::Error::Nix(Errno::EACCES)) => {
                println!("Incorrect permissions on /dev/uinput.");
                println!("Please see documentation on how to fix this.");
                exit(1);
            }
            Err(e) => {
                println!("Unknown error:");
                println!("{e:?}");
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
            device: ui,
        }
    }

    fn press_key(&mut self, key: uinput::event::keyboard::Key) {
            self.device.click(&key).unwrap();
            self.device.synchronize().unwrap()
    }

    fn press_left(&mut self) {
        self.press_key(Left)
    }

    fn press_right(&mut self) {
        self.press_key(Right)
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
