use kobject_uevent::{ActionType, UEvent};
use netlink_sys::{Socket, SocketAddr, protocols::NETLINK_KOBJECT_UEVENT};
use std::process;

const VID_NINTENDO: u16 = 1406;
const PID_JOYCON_LEFT: u16 = 8198;
const PID_JOYCON_RIGHT: u16 = 8199;

//pub struct Joycon {
//    device: evdev::Device,
//}

pub fn get_joycon() -> Option<evdev::Device> {
    evdev::enumerate().map(|(_, d)| d).find(|d| {
        (d.input_id().product() == PID_JOYCON_RIGHT || d.input_id().product() == PID_JOYCON_LEFT)
            && d.input_id().vendor() == VID_NINTENDO
    })
}

pub fn wait_for_joycon() {
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
