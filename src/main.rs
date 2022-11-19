use evdev;
use uinput;

// Nintendo product ID
const VENDOR_ID: u16 = 1406;
const PRODUCT_ID: u16 = 8198;

fn get_joycon() -> Option<evdev::Device> {
    // TODO use https://github.com/emberian/evdev/blob/master/examples/_pick_device.rs
    for d in evdev::enumerate() {
        if d.input_id().product() == PRODUCT_ID && d.input_id().vendor() == VENDOR_ID {
            return Some(d);
        }
    }
    None
}

/*
fn make_vibrate(j: &mut evdev::Device) {
     for i in 0..5 {
        let on = i % 2 != 0;
        j.send_events(&[
            evdev::InputEvent::new(
                evdev::EventType::FORCEFEEDBACK,
                10,
                if on { i32::MAX } else { 0 },
            ),
        ])
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
     }
} */

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
