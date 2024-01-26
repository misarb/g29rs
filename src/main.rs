use hidapi::{HidApi, HidDevice, HidResult};

#[derive(Debug)]
struct G29 {
    device: HidDevice,
}

impl G29 {
    fn new() -> Self {
        let api = HidApi::new().unwrap();
        let device = api.open(0x17ef, 0x608d).unwrap();

        Self { device }
    }
}

fn main() {
    let gdriver = G29::new();
    println!("Device : {:?}", gdriver.device);
    println!("Show all available device");
    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                println!("{:04x} :: {:04x}", device.vendor_id(), device.product_id());
            }
        }
        Err(e) => {
            println!("Err {}", e);
        }
    }
    // println!("Hello, world!");
}
