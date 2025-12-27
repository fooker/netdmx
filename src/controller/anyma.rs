use super::Controller;

use std::time;

use rusb;
use rusb::{Context, UsbContext};

pub struct AnymaController {
    device: rusb::DeviceHandle<Context>,
}

impl AnymaController {
    const VENDOR_ID: u16 = 0x16c0;
    const PRODUCT_ID: u16 = 0x05dc;

    const REQUEST_SET_CHANNEL_RANGE: u8 = 0x02;

    pub fn new(context: &rusb::Context) -> Self {
        let device = context.open_device_with_vid_pid(Self::VENDOR_ID, Self::PRODUCT_ID)
                                .expect("Unable to open USB device");

        device.claim_interface(0)
              .expect("Failed to claim interface");

        return Self {
            device
        };
    }
}

impl Controller for AnymaController {
    fn send(&mut self, data: &[u8; 512]) {
        self.device.write_control(rusb::request_type(rusb::Direction::Out, rusb::RequestType::Vendor, rusb::Recipient::Device),
                                  Self::REQUEST_SET_CHANNEL_RANGE,
                                  data.len() as u16,
                                  0,
                                  data,
                                  time::Duration::from_millis(500))
            .expect("Failed to send data");
    }
}
