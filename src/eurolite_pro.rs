use super::Controller;

use std::time;

use libusb;

pub struct EuroliteProController<'a> {
    device: libusb::DeviceHandle<'a>,
}

impl<'a> EuroliteProController<'a> {
    const VENDOR_ID: u16 = 0x04d8;
    const PRODUCT_ID: u16 = 0xfa63;

    const ENDPOINT: u8 = 0x02;

    const FRAME_START_OF_MESSAGE: u8 = 0x7e;
    const FRAME_END_OF_MESSAGE: u8 = 0x7e;
    const FRAME_DMX_LABEL: u8 = 6;

    pub fn new(context: &'a libusb::Context) -> Self {
        let mut device = context.open_device_with_vid_pid(Self::VENDOR_ID, Self::PRODUCT_ID)
            .expect("Unable to open USB device");

        // TODO: OLA searches for an interface with endpoint address 0x02 (ENDPOINT) here
        device.claim_interface(1)
            .expect("Failed to claim interface");

        return Self {
            device
        };
    }
}

impl<'a> Controller for EuroliteProController<'a> {
    fn send(&mut self, data: [u8; 512]) {
        let frame: [u8; 518] = [
            Self::FRAME_START_OF_MESSAGE,
            Self::FRAME_DMX_LABEL,
            (((512 + 1 as u16).to_be() >> 0) & 0xFF) as u8,
            (((512 + 1 as u16).to_be() >> 8) & 0xFF) as u8,
            0, // DMX start code
            data[0x000], data[0x001], data[0x002], data[0x003], data[0x004], data[0x005], data[0x006], data[0x007],
            data[0x008], data[0x009], data[0x00a], data[0x00b], data[0x00c], data[0x00d], data[0x00e], data[0x00f],
            data[0x010], data[0x011], data[0x012], data[0x013], data[0x014], data[0x015], data[0x016], data[0x017],
            data[0x018], data[0x019], data[0x01a], data[0x01b], data[0x01c], data[0x01d], data[0x01e], data[0x01f],
            data[0x020], data[0x021], data[0x022], data[0x023], data[0x024], data[0x025], data[0x026], data[0x027],
            data[0x028], data[0x029], data[0x02a], data[0x02b], data[0x02c], data[0x02d], data[0x02e], data[0x02f],
            data[0x030], data[0x031], data[0x032], data[0x033], data[0x034], data[0x035], data[0x036], data[0x037],
            data[0x038], data[0x039], data[0x03a], data[0x03b], data[0x03c], data[0x03d], data[0x03e], data[0x03f],
            data[0x040], data[0x041], data[0x042], data[0x043], data[0x044], data[0x045], data[0x046], data[0x047],
            data[0x048], data[0x049], data[0x04a], data[0x04b], data[0x04c], data[0x04d], data[0x04e], data[0x04f],
            data[0x050], data[0x051], data[0x052], data[0x053], data[0x054], data[0x055], data[0x056], data[0x057],
            data[0x058], data[0x059], data[0x05a], data[0x05b], data[0x05c], data[0x05d], data[0x05e], data[0x05f],
            data[0x060], data[0x061], data[0x062], data[0x063], data[0x064], data[0x065], data[0x066], data[0x067],
            data[0x068], data[0x069], data[0x06a], data[0x06b], data[0x06c], data[0x06d], data[0x06e], data[0x06f],
            data[0x070], data[0x071], data[0x072], data[0x073], data[0x074], data[0x075], data[0x076], data[0x077],
            data[0x078], data[0x079], data[0x07a], data[0x07b], data[0x07c], data[0x07d], data[0x07e], data[0x07f],
            data[0x080], data[0x081], data[0x082], data[0x083], data[0x084], data[0x085], data[0x086], data[0x087],
            data[0x088], data[0x089], data[0x08a], data[0x08b], data[0x08c], data[0x08d], data[0x08e], data[0x08f],
            data[0x090], data[0x091], data[0x092], data[0x093], data[0x094], data[0x095], data[0x096], data[0x097],
            data[0x098], data[0x099], data[0x09a], data[0x09b], data[0x09c], data[0x09d], data[0x09e], data[0x09f],
            data[0x0a0], data[0x0a1], data[0x0a2], data[0x0a3], data[0x0a4], data[0x0a5], data[0x0a6], data[0x0a7],
            data[0x0a8], data[0x0a9], data[0x0aa], data[0x0ab], data[0x0ac], data[0x0ad], data[0x0ae], data[0x0af],
            data[0x0b0], data[0x0b1], data[0x0b2], data[0x0b3], data[0x0b4], data[0x0b5], data[0x0b6], data[0x0b7],
            data[0x0b8], data[0x0b9], data[0x0ba], data[0x0bb], data[0x0bc], data[0x0bd], data[0x0be], data[0x0bf],
            data[0x0c0], data[0x0c1], data[0x0c2], data[0x0c3], data[0x0c4], data[0x0c5], data[0x0c6], data[0x0c7],
            data[0x0c8], data[0x0c9], data[0x0ca], data[0x0cb], data[0x0cc], data[0x0cd], data[0x0ce], data[0x0cf],
            data[0x0d0], data[0x0d1], data[0x0d2], data[0x0d3], data[0x0d4], data[0x0d5], data[0x0d6], data[0x0d7],
            data[0x0d8], data[0x0d9], data[0x0da], data[0x0db], data[0x0dc], data[0x0dd], data[0x0de], data[0x0df],
            data[0x0e0], data[0x0e1], data[0x0e2], data[0x0e3], data[0x0e4], data[0x0e5], data[0x0e6], data[0x0e7],
            data[0x0e8], data[0x0e9], data[0x0ea], data[0x0eb], data[0x0ec], data[0x0ed], data[0x0ee], data[0x0ef],
            data[0x0f0], data[0x0f1], data[0x0f2], data[0x0f3], data[0x0f4], data[0x0f5], data[0x0f6], data[0x0f7],
            data[0x0f8], data[0x0f9], data[0x0fa], data[0x0fb], data[0x0fc], data[0x0fd], data[0x0fe], data[0x0ff],
            data[0x100], data[0x101], data[0x102], data[0x103], data[0x104], data[0x105], data[0x106], data[0x107],
            data[0x108], data[0x109], data[0x10a], data[0x10b], data[0x10c], data[0x10d], data[0x10e], data[0x10f],
            data[0x110], data[0x111], data[0x112], data[0x113], data[0x114], data[0x115], data[0x116], data[0x117],
            data[0x118], data[0x119], data[0x11a], data[0x11b], data[0x11c], data[0x11d], data[0x11e], data[0x11f],
            data[0x120], data[0x121], data[0x122], data[0x123], data[0x124], data[0x125], data[0x126], data[0x127],
            data[0x128], data[0x129], data[0x12a], data[0x12b], data[0x12c], data[0x12d], data[0x12e], data[0x12f],
            data[0x130], data[0x131], data[0x132], data[0x133], data[0x134], data[0x135], data[0x136], data[0x137],
            data[0x138], data[0x139], data[0x13a], data[0x13b], data[0x13c], data[0x13d], data[0x13e], data[0x13f],
            data[0x140], data[0x141], data[0x142], data[0x143], data[0x144], data[0x145], data[0x146], data[0x147],
            data[0x148], data[0x149], data[0x14a], data[0x14b], data[0x14c], data[0x14d], data[0x14e], data[0x14f],
            data[0x150], data[0x151], data[0x152], data[0x153], data[0x154], data[0x155], data[0x156], data[0x157],
            data[0x158], data[0x159], data[0x15a], data[0x15b], data[0x15c], data[0x15d], data[0x15e], data[0x15f],
            data[0x160], data[0x161], data[0x162], data[0x163], data[0x164], data[0x165], data[0x166], data[0x167],
            data[0x168], data[0x169], data[0x16a], data[0x16b], data[0x16c], data[0x16d], data[0x16e], data[0x16f],
            data[0x170], data[0x171], data[0x172], data[0x173], data[0x174], data[0x175], data[0x176], data[0x177],
            data[0x178], data[0x179], data[0x17a], data[0x17b], data[0x17c], data[0x17d], data[0x17e], data[0x17f],
            data[0x180], data[0x181], data[0x182], data[0x183], data[0x184], data[0x185], data[0x186], data[0x187],
            data[0x188], data[0x189], data[0x18a], data[0x18b], data[0x18c], data[0x18d], data[0x18e], data[0x18f],
            data[0x190], data[0x191], data[0x192], data[0x193], data[0x194], data[0x195], data[0x196], data[0x197],
            data[0x198], data[0x199], data[0x19a], data[0x19b], data[0x19c], data[0x19d], data[0x19e], data[0x19f],
            data[0x1a0], data[0x1a1], data[0x1a2], data[0x1a3], data[0x1a4], data[0x1a5], data[0x1a6], data[0x1a7],
            data[0x1a8], data[0x1a9], data[0x1aa], data[0x1ab], data[0x1ac], data[0x1ad], data[0x1ae], data[0x1af],
            data[0x1b0], data[0x1b1], data[0x1b2], data[0x1b3], data[0x1b4], data[0x1b5], data[0x1b6], data[0x1b7],
            data[0x1b8], data[0x1b9], data[0x1ba], data[0x1bb], data[0x1bc], data[0x1bd], data[0x1be], data[0x1bf],
            data[0x1c0], data[0x1c1], data[0x1c2], data[0x1c3], data[0x1c4], data[0x1c5], data[0x1c6], data[0x1c7],
            data[0x1c8], data[0x1c9], data[0x1ca], data[0x1cb], data[0x1cc], data[0x1cd], data[0x1ce], data[0x1cf],
            data[0x1d0], data[0x1d1], data[0x1d2], data[0x1d3], data[0x1d4], data[0x1d5], data[0x1d6], data[0x1d7],
            data[0x1d8], data[0x1d9], data[0x1da], data[0x1db], data[0x1dc], data[0x1dd], data[0x1de], data[0x1df],
            data[0x1e0], data[0x1e1], data[0x1e2], data[0x1e3], data[0x1e4], data[0x1e5], data[0x1e6], data[0x1e7],
            data[0x1e8], data[0x1e9], data[0x1ea], data[0x1eb], data[0x1ec], data[0x1ed], data[0x1ee], data[0x1ef],
            data[0x1f0], data[0x1f1], data[0x1f2], data[0x1f3], data[0x1f4], data[0x1f5], data[0x1f6], data[0x1f7],
            data[0x1f8], data[0x1f9], data[0x1fa], data[0x1fb], data[0x1fc], data[0x1fd], data[0x1fe], data[0x1ff],
            Self::FRAME_END_OF_MESSAGE
        ];

        // TODO: Handle transfer count?
        self.device.write_bulk(Self::ENDPOINT, &frame, time::Duration::from_millis(500))
            .expect("Failed to send data");
    }
}
