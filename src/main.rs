// SPDX-License-Identifier: MIT

use std::process::exit;
use std::thread;
use std::time::Duration;

use evdev::KeyCode;

#[cfg(feature = "sandbox")]
use nix::sys::prctl::set_no_new_privs;

use nix::unistd::getuid;

use clap::Parser;

mod clicker;

pub mod joycon;
pub mod user_parser;

#[cfg(feature = "seccomp")]
pub mod seccomp;

#[cfg(feature = "landlock")]
pub mod landlock;

#[cfg(all(
    feature = "sandbox",
    not(any(feature = "landlock", feature = "seccomp"))
))]
compile_error!("Please enable \"landlock\" or \"seccomp\" for sandbox.");

#[cfg(all(feature = "landlock", feature = "seccomp"))]
compile_error!(
    "Only one feature out of \"landlock\" or \"seccomp\" must be enabled for this crate."
);

// return 1 single joycon, since the code use `find`
// multiple joycon support is out of scope for now
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Switch to specified user after opening the device as root
    #[arg(short, long)]
    user: Option<user_parser::ParsedUser>,

    /// Enable extra debug output
    #[arg(short, long)]
    debug: bool,

    /// Wait to reconnect if the joycon disappeared
    #[arg(short, long)]
    background: bool,

    /// Disable sandbox
    #[cfg(feature = "sandbox")]
    #[arg(long)]
    disable_sandbox: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut c = match clicker::Clicker::new() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Cannot create Clicker");
            exit(1)
        }
    };

    #[cfg(feature = "sandbox")]
    if !cli.disable_sandbox {
        if cli.debug {
            println!("Enabling sandboxing ");
        }

        if let Err(e) = set_no_new_privs() {
            println!("Can't set no new privs: {e}");
            exit(1);
        }

        #[cfg(feature = "landlock")]
        let confiner = landlock::LandlockConfiner::new();

        #[cfg(feature = "seccomp")]
        let confiner = seccomp::SeccompConfiner::new(true, c.get_device_fd());

        if let Err(e) = confiner.confine() {
            println!("Can't confine: {e}");
            exit(1);
        }
    }

    if let Some(user) = cli.user {
        user.setuid().expect("setuid");

        if cli.debug {
            println!("Changed uid to {user}");
        }
    }

    // potential bug, not sure where it comes from
    if getuid().is_root() {
        println!("Running as root, which prevent getting keypress in some configuration");
    }

    'get_joycon: loop {
        match joycon::get_joycon() {
            None => {
                if cli.debug {
                    println!("No joycon detected, entering loop");
                }
                joycon::wait_for_joycon();
                // time needed to make sure that the device appear in
                // /dev/input after wait_for_joycon return
                // (not sure why, but 2 sec is enough)
                thread::sleep(Duration::from_millis(2000));
            }

            Some(mut j) => {
                println!("Device found: {:?}", j.name());

                'fetch_events: loop {
                    // this return a error if the device no longer exist
                    match j.fetch_events() {
                        Ok(evs) => {
                            for ev in evs {
                                match ev.destructure() {
                                    evdev::EventSummary::Key(_, k, 1) => {
                                        if cli.debug {
                                            println!("{k:?}");
                                        }
                                        match k {
                                            KeyCode::BTN_DPAD_LEFT | KeyCode::BTN_WEST => {
                                                if c.press_left().is_err() {
                                                    eprintln!("Cannot press left");
                                                }
                                            }
                                            KeyCode::BTN_TR
                                            | KeyCode::BTN_TL
                                            | KeyCode::BTN_TR2
                                            | KeyCode::BTN_TL2
                                            | KeyCode::BTN_DPAD_RIGHT
                                            | KeyCode::BTN_EAST => {
                                                if c.press_right().is_err() {
                                                    eprintln!("Cannot press right");
                                                }
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
                        // might one day be fixed if the error is correctly handled
                        // eg, if it is no longer: value: Os { code: 19, kind: Uncategorized, message: "No such device" }
                        Err(e) if e.raw_os_error() == Some(19) => {
                            if !cli.background {
                                println!("Joycon disconnected, shutting down");
                                break 'get_joycon;
                            } else {
                                println!("Joycon disconnected, waiting in background");
                                // break jump at the end of the loop
                                break 'fetch_events;
                            }
                        }
                        Err(e) => {
                            println!("Error with fetch_events: {e:?}");
                            exit(1);
                        }
                    }
                }
            }
        }
    }
}
