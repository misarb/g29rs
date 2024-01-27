use core::panic;
use hidapi::{HidApi, HidDevice, HidResult};
use std::{collections::HashMap, thread, time::Duration};

#[derive(Debug)]
struct G29<'a> {
    device: HidDevice,
    cashe: usize,
    state: HashMap<&'a str, u8>,
}

impl G29<'_> {
    fn new() -> Self {
        let api = HidApi::new().unwrap();
        let device = api.open(0x17ef, 0x608d).unwrap();
        let mut state = HashMap::new();
        state.insert("steering", 50);
        state.insert("throttle", 255);
        state.insert("clutch", 255);
        state.insert("brake", 255);
        Self {
            device,
            cashe: 0,
            state,
        }
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
    #[warn(unused_variables)]
    // Read from the G29 Input
    fn pump(&mut self) -> usize {
        let mut buf = [0u8; 16];
        let data = self.device.read(&mut buf).unwrap();
        if data >= 23 {
            //self.update_state();
            self.cashe = data;
        }
        return data;
    }
    fn update_state(&self, byte_array: Vec<f32>) {
        if self.cashe == 0 {
            panic!("cashe is Empty");
        }

        //Update state
        // steering
        if byte_array[4] != self.cashe[4] or byte_array != self.cashe{
            let steering_val = self.calc_steering(&mut byte_array[5],&mut byte_array[4]);
            self.state.insert("steering", steering_val);
        }
        //throttle 
        if byte_array[6] != self.cashe[6]{
            self.state.insert("throttle", byte_array[6]);
        }
        //brake
        if byte_array[7] != self.cashe[7]{
            self.state.insert("brake", byte_array[7]);
        }

        //clutch
        if byte_array[8] != self.cashe[8] {
            self.state.insert("clutch", byte_array[8]);
        }
    }
    // geter State
    fn get_state(&self) -> HashMap<&str,u8>{
        self.state
    }

    fn calc_steering(&self,start:&mut f32 , end:&mut f32) -> u8{

        // start from 0 to 255
        // end from 0 to 255
        // scale between 0 -> 100
        *start = (*start/256.0) * (100.0-(100.0/256.0));
        // scale between 0 -> 3
        *end = (*end/256.0) * (100.0/256.0);

        return (*start + *end).round() as u8;

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
