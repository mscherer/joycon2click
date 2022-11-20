// Nintendo product ID
const VENDOR_ID: u16 = 1406;
const PRODUCT_ID_LEFT: u16 = 8198;
const PRODUCT_ID_RIGHT: u16 = 8199;

fn get_joycon() -> Option<evdev::Device> {
    evdev::enumerate().map(|(_, d)| d).find(|d| {
        (d.input_id().product() == PRODUCT_ID_RIGHT || d.input_id().product() == PRODUCT_ID_LEFT)
            && d.input_id().vendor() == VENDOR_ID
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
