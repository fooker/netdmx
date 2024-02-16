use std::net;
use std::thread;
use std::sync::mpsc;

use clap::{Command, Arg, ValueEnum, builder::PossibleValue, builder::EnumValueParser, crate_version};


mod anyma;
mod eurolite_pro;


trait Controller {
    fn send(&mut self, data: [u8; 512]);
}


#[derive(PartialEq, Clone, Debug)]
enum ControllerType {
    Anyma,
    EurolitePro
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
