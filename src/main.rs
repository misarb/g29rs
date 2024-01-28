use core::panic;
use hidapi::{HidApi, HidDevice, HidResult};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
#[warn(dead_code)]
struct G29Driver {
    device: HidDevice,
    cache: Vec<u8>,
    state: HashMap<&'static str, u8>,
}

struct G29 {
    g29: Arc<Mutex<G29Driver>>,
    reading_thread: Option<JoinHandle<()>>,
}

impl G29 {
    fn new() -> Self {
        Self {
            g29: Arc::new(Mutex::new(G29Driver::new())),
            reading_thread: None,
        }
    }
    fn start_pumping(&mut self) {
        let local_g29 = self.g29.clone();
        self.reading_thread = Some(thread::spawn(move || {
            local_g29.lock().unwrap().read_loop();
        }));
    }

    fn stop_pumping(&mut self) {
        if let Some(handle) = self.reading_thread.take() {
            handle.join().unwrap();
        } else {
            println!("No Thread spawned");
        }
    }
}

impl G29Driver {
    fn new() -> Self {
        let api = HidApi::new().unwrap();
        let device = api.open(0x17ef, 0x608d).unwrap();
        let mut state = HashMap::new();
        state.insert("steeringithrottle", 255);
        state.insert("clutch", 255);
        state.insert("brake", 255);
        Self {
            device,
            cache: Vec::new(),
            state,
        }
    }

    // Write to the G29Driver Driver
    // calibration the steering wheel of the G2
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
        if (strength < 0.0) || (strength > 1.0) {
            panic!("Strength must be in range of 0.0 to 1.0");
        }
        if (rate < 0.0) || (rate > 1.0) {
            panic!("Rate must be in range of 0.0 to 1.0 ");
        }

        // autocenter Up
        let up_msg = [0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
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

    fn force_off(&self) {
        self.device
            .write(&[0xf3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            .unwrap();
    }

    // Read from the G29Driver Input
    fn pump(&mut self, timeout: i32) -> usize {
        let mut buf = [0u8; 16];
        let data = self.device.read_timeout(&mut buf, timeout).unwrap();
        let byte_array = buf[..data].to_vec();

        if byte_array.len() >= 12 {
            self.update_state(&byte_array);
            self.cache = byte_array;
        }
        return data;
    }

    //    fn start_pumping(&'static mut self) -> JoinHandle<()> {
    //        thread::spawn(move || {
    //            self.read_loop();
    //        })
    //
    //        //return spawn_thread_read;
    //    }

    fn read_loop(&mut self) {
        loop {
            self.pump(10);
        }
    }

    //    fn stop_reading(&'static mut self) {
    //        if let Some(thread) = self.start_pumping(10) {
    //            thread.join();
    //        } else {
    //            println!("thread   reading not spawned");
    //        }
    //    }

    fn update_state(&mut self, byte_array: &Vec<u8>) {
        if self.cache.is_empty() {
            panic!("cache is Empty");
        }

        //Update state
        // steering
        if byte_array[4] != self.cache[4] || byte_array[5] != self.cache[5] {
            let steering_val = self.calculate_steering(&byte_array[5], &byte_array[4]);
            self.state.insert("steering", steering_val);
        }
        //throttle
        if byte_array[6] != self.cache[6] {
            self.state.insert("throttle", byte_array[6]);
        }
        //brake
        if byte_array[7] != self.cache[7] {
            self.state.insert("brake", byte_array[7]);
        }

        //clutch
        if byte_array[8] != self.cache[8] {
            self.state.insert("clutch", byte_array[8]);
        }
    }

    // geter State
    fn get_state(&self) -> HashMap<&str, u8> {
        self.state.clone()
    }

    fn calculate_steering(&self, start: &u8, end: &u8) -> u8 {
        // start from 0 to 255
        // end from 0 to 255
        // scale between 0 -> 100
        let start_scale = (*start / 255) * (100 - (100 / 255));
        // scale between 0 -> 3
        let end_scale = (*end / 255) * (100 / 255);

        return (start_scale + end_scale) as u8;
    }
}

fn main() {
    let mut g29 = G29::new();
    g29.start_pumping();
    g29.g29.lock().unwrap().set_autocenter(0.5, 0.05);
    loop {
        println!("steering = {:?}", g29.g29.lock().unwrap().get_state());
    }
    //    let g29 = G29Driver::new();
    //    g29.set_autocenter(0.5, 0.05);
    //    g29.force_feedback_constant(0.5);
    //    //let thread = g29.start_pumping();
    //
    //    let state = g29.get_state();
    //    println!("{:?}", state);

    //thread.join();

    // println!("Device : {:?}", g29.device);
    //println!("Show all available device");
    //    match HidApi::new() {
    //        Ok(api) => {
    //            for device in api.device_list() {
    //                println!("{:04x} :: {:04x}", device.vendor_id(), device.product_id());
    //            }
    //        }
    //        Err(e) => {
    //            println!("Err {}", e);
    //        }
    //    }
    // println!("Hello, world!");
}
