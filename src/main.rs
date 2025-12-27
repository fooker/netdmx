use std::net;
use std::thread;
use std::time::Duration;

use clap::{Arg, builder::EnumValueParser, builder::PossibleValue, Command, crate_version, ValueEnum};
use triple_buffer::triple_buffer;

use controller::Controller;

mod controller;

#[derive(PartialEq, Clone, Debug)]
enum ControllerType {
    Anyma,
    EurolitePro,
}

impl ValueEnum for ControllerType {
    fn value_variants<'a>() -> &'a [Self] {
        return &[
            Self::Anyma,
            Self::EurolitePro,
        ];
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        return Some(match self {
            Self::Anyma => PossibleValue::new("anyma"),
            Self::EurolitePro => PossibleValue::new("eurolite"),
        });
    }
}


fn main() {
    let matches = Command::new("netdmx")
        .about("Network to DMX")
        .version(crate_version!())
        .arg(Arg::new("listen")
            .short('l')
            .long("listen")
            .value_name("HOST:PORT")
            .help("UDP host and port to receive data on")
            .default_value("127.0.0.1:34254"))
        .arg(Arg::new("type")
            .short('t')
            .long("type")
            .value_name("TYPE")
            .help("DMX Controller type")
            .required(true)
            .value_parser(EnumValueParser::<ControllerType>::new()))
        .get_matches();

    let socket = net::UdpSocket::bind(matches.get_one::<String>("listen").unwrap())
        .expect("Failed to open socket");

    let context = rusb::Context::new()
        .expect("Failed to init libusb context");

    let mut controller: Box<dyn Controller> = match matches.get_one::<ControllerType>("type").unwrap() {
        ControllerType::Anyma => Box::new(controller::AnymaController::new(&context)),
        ControllerType::EurolitePro => Box::new(controller::EuroliteProController::new(&context)),
    };

    let (mut w, mut r) = triple_buffer(&[0u8; 512]);

    thread::spawn(move || {
        loop {
            socket.recv(w.input_buffer_mut())
                .expect("Failed to receive data");

            w.publish();
        }
    });

    eprintln!("Ready");

    loop {
        {
            r.update();
            controller.send(r.read());
        }

        thread::sleep(Duration::from_millis(10));
    }
}
