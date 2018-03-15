#[macro_use]
extern crate clap;

extern crate libusb;

use clap::{App, Arg};

use std::net;

mod anymau;
mod eurolite_pro;

trait Controller {
    fn send(&mut self, data: [u8; 512]);
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    enum ControllerType {
        Anymau,
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

    let context = libusb::Context::new()
        .expect("Failed to init libusb context");

    let mut controller: Box<Controller> = match value_t_or_exit!(matches, "type", ControllerType) {
        ControllerType::Anymau => Box::new(anymau::AnymauController::new(&context)),
        ControllerType::EurolitePro => Box::new(eurolite_pro::EuroliteProController::new(&context)),
    };

    let socket = net::UdpSocket::bind(matches.value_of("port").unwrap())
        .expect("Failed to open socket");

    let mut data = [0; 512];

    loop {
        socket.recv(&mut data)
            .expect("Failed to receive data");


        controller.send(data);
    }
}
