use std::net;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use clap::{Arg, builder::EnumValueParser, builder::PossibleValue, Command, crate_version, ValueEnum};

use controller::Controller;

use crate::buffer::Buffer;

mod controller;
mod buffer;

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

    let context = libusb::Context::new()
        .expect("Failed to init libusb context");

    let mut controller: Box<dyn Controller> = match matches.get_one::<ControllerType>("type").unwrap() {
        ControllerType::Anyma => Box::new(controller::AnymaController::new(&context)),
        ControllerType::EurolitePro => Box::new(controller::EuroliteProController::new(&context)),
    };

    let buffer: Arc<Buffer<[u8; 512]>> = Buffer::with_constructor(|| [0; 512]).into();
    let r = buffer.clone();
    let w = buffer.clone();

    thread::spawn(move || {
        loop {
            w.update(|data| {
                socket.recv(&mut *data)
                    .expect("Failed to receive data");
            });
        }
    });

    eprintln!("Ready");

    loop {
        {
            let data = r.read();
            controller.send(&*data);
        }

        thread::sleep(Duration::from_millis(10));
    }
}
