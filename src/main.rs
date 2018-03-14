extern crate clap;
extern crate libusb;

use clap::{App, Arg};

use std::u16;
use std::time;
use std::net;

const UDMX_SET_CHANNEL_RANGE: u8 = 0x0002;

fn main() {
    let matches = App::new("netdmx")
        .about("Network to DMX")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("UDP port to receive data on (can be IP:PORT)")
            .takes_value(true))
        .arg(Arg::with_name("vendor_id")
            .long("vid")
            .value_name("VID")
            .help("USB vendor ID (in hex)")
            .takes_value(true))
        .arg(Arg::with_name("product_id")
            .long("pid")
            .value_name("PID")
            .help("USB product ID (in hex)")
            .takes_value(true))
        .get_matches();

    let context = libusb::Context::new().unwrap();

    let vendor_id = u16::from_str_radix(matches.value_of("vendor_id").unwrap_or("16c0"), 16)
        .expect("Ill-formatted vendor ID");
    let product_id = u16::from_str_radix(matches.value_of("product_id").unwrap_or("05dc"), 16)
        .expect("Ill-formatted vendor ID");

    let mut device = context.open_device_with_vid_pid(vendor_id, product_id)
        .expect("Unable to open USB device");

    device.claim_interface(0)
        .expect("Failed to claim interface");

    let socket = net::UdpSocket::bind(matches.value_of("port").unwrap_or("127.0.0.1:34254"))
        .expect("Failed to open socket");

    let mut data = [0; 512];

    println!("Running");

    loop {
        socket.recv(&mut data)
            .expect("Failed to receive data");

        device.write_control(libusb::request_type(libusb::Direction::Out, libusb::RequestType::Vendor, libusb::Recipient::Device),
                             UDMX_SET_CHANNEL_RANGE,
                             data.len() as u16,
                             0,
                             &data,
                             time::Duration::from_millis(500))
            .expect("Failed to send data");
    }
}
