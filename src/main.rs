// SPDX-License-Identifier: MIT

const VID_NINTENDO: u16 = 1406;
const PID_JOYCON_LEFT: u16 = 8198;
const PID_JOYCON_RIGHT: u16 = 8199;

fn get_joycon() -> Option<evdev::Device> {
    evdev::enumerate().map(|(_, d)| d).find(|d| {
        (d.input_id().product() == PID_JOYCON_RIGHT || d.input_id().product() == PID_JOYCON_LEFT)
            && d.input_id().vendor() == VID_NINTENDO
    })
}

fn press_right(ui: &mut uinput::Device) {
    ui.click(&uinput::event::keyboard::Key::Right).unwrap();
    // TODO check
    // ui.synchronize();
    println!("Pressed right");
}

fn main() {
    // TODO see if there is a less ugly code
    let mut ui = uinput::default()
        .unwrap()
        .name("joycon2click")
        .unwrap()
        .event(uinput::event::Keyboard::All)
        .unwrap()
        .create()
        .unwrap();

    match get_joycon() {
        None => println!("No joycon detected"),

        Some(mut j) => {
            println!("Device found: {:?}", j.name());
            loop {
                for ev in j.fetch_events().unwrap() {
                    match ev.kind() {
                        evdev::InputEventKind::Key(_) => press_right(&mut ui),
                        k => println!("{:?}", k),
                    }
                }
            }
        }
    }
}
