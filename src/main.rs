#[macro_use]
extern crate clap;

extern crate libusb;


use std::net;
use std::thread;
use std::sync::mpsc;

use clap::{App, Arg};


mod anyma;
mod eurolite_pro;


trait Controller {
    fn send(&mut self, data: [u8; 512]);
}


arg_enum! {
    #[derive(PartialEq, Debug)]
    enum ControllerType {
        Anyma,
        EurolitePro
    }
}


fn main() {
    let matches = App::new("netdmx")
            .about("Network to DMX")
            .version(crate_version!())
            .arg(Arg::with_name("listen")
                    .short("l")
                    .long("listen")
                    .value_name("HOST:PORT")
                    .help("UDP host and port to receive data on")
                    .takes_value(true)
                    .default_value("127.0.0.1:34254"))
            .arg(Arg::with_name("type")
                    .long("type")
                    .value_name("TYPE")
                    .help("DMX Controller type")
                    .takes_value(true)
                    .required(true)
                    .possible_values(&ControllerType::variants()))
            .get_matches();

    let socket = net::UdpSocket::bind(matches.value_of("listen").unwrap())
            .expect("Failed to open socket");

    let context = libusb::Context::new()
            .expect("Failed to init libusb context");

    let mut controller: Box<Controller> = match value_t_or_exit!(matches, "type", ControllerType) {
        ControllerType::Anyma => Box::new(anyma::AnymaController::new(&context)),
        ControllerType::EurolitePro => Box::new(eurolite_pro::EuroliteProController::new(&context)),
    };

    let (output_pub, output_sub) = mpsc::sync_channel(0);
    thread::spawn(move || {
        let mut data = [0; 512];
        loop {
            socket.recv(&mut data)
                  .expect("Failed to receive data");

            if let Err(mpsc::TrySendError::Full(_)) = output_pub.try_send(data) {
                eprintln!("Output is to slow. Dropping frame.");
            }
        }
    });

    loop {
        let data = output_sub.recv().unwrap();
        controller.send(data);
    }
}
