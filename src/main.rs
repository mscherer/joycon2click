// Nintendo product ID
const VENDOR_ID: u16 = 1406;
const PRODUCT_ID: u16 = 8198;

fn get_joycon() -> evdev::Device {
    // vendor 1406
    // productid 8198
    // TODO use https://github.com/emberian/evdev/blob/master/examples/_pick_device.rs
    let mut c = 0;
    let devices = evdev::enumerate().collect::<Vec<_>>();
    // readdir returns them in reverse order from their eventN names for some reason
    for (i, d) in devices.iter().enumerate() {
        if d.input_id().product() == PRODUCT_ID && d.input_id().vendor() == VENDOR_ID {
            c = i;
        }
    }
    devices.into_iter().nth(c).unwrap()
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
    println!("Pressed right");
}

fn main() {
    let mut j = get_joycon();
    let mut ui = uinput::default()
        .unwrap()
        .name("test")
        .unwrap()
        .event(uinput::event::Keyboard::All)
        .unwrap()
        .create()
        .unwrap();

    println!("Device: {:?}", j.name());
    // make_vibrate(&mut j);
    loop {
        for ev in j.fetch_events().unwrap() {
            let k = ev.kind();
            match k {
                evdev::InputEventKind::Key(_) => press_right(&mut ui),
                //_ => (),
                _ => println!("{:?}", ev.kind()),
            }
        }
    }
}
