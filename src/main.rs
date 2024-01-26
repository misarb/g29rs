use core::panic;
use hidapi::{HidApi, HidDevice, HidResult};
use std::{thread, time::Duration};

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

    // Write to the G29 Driver
    // calibration the steering wheel of the G2
    #[warn(dead_code)]
    fn reset(&self) {
        self.device
            .write(&[0xf8, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00])
            .unwrap();
        self.device
            .write(&[0xf8, 0x09, 0x05, 0x01, 0x01, 0x00, 0x00])
            .unwrap();
        // wait for setting the calibration
        thread::sleep(Duration::from_secs(10));
    }
    #[warn(dead_code)]
    fn force_feedback_constant(&self, val: f32) {
        if val < 0.0 || val > 1.0 {
            panic!("Value must be in range of 0 to 1");
        }
        let val_scale = (val * 255.0).round() as u8;
        let msg = [0x11, 0x00, val_scale, 0x00, 0x00, 0x00, 0x00];
        self.device.write(&msg).unwrap();
    }

    /// default value to be used strength = 0.5 and rate = 0.05
    fn set_autocenter(&self, strength: f32, rate: f32) {
        if strength < 0.0 || strength > 1.0 {
            panic!("Strength must be in range of 0.0 to 1.0");
        }
        if rate < 0.0 || rate > 1.0 {
            panic!("Rate must be in range of 0.0 to 1.0 ");
        }

        // autocenter Up
        let up_msg = [0x14, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.device.write(&up_msg).unwrap();

        //scale Rate 0 -> 255 and Strength to 0 ->15
        let strength_scale = (strength * 15.0).round() as u8;
        let rate_scale = (rate * 255.0).round() as u8;

        self.device
            .write(&[
                0xfe,
                0x0d,
                strength_scale,
                strength_scale,
                rate_scale,
                0x00,
                0x00,
                0x00,
            ])
            .unwrap();
    }

    fn force_off(self) {
        self.device
            .write(&[0xf3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            .unwrap();
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
