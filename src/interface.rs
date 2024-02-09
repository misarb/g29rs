//!
//! Interfaces to interacting with the G29 controller to set force feedback , Autocenter ...
//!

use hidapi::{HidApi, HidDevice};
use std::{collections::HashMap, thread, time::Duration};

///! The `G29Interface` struct is the underlying driver for the G29 device, managing communication and state.
#[derive(Debug)]
pub struct G29Interface {
    device: HidDevice,
    cache: Vec<u8>,
    state: HashMap<&'static str, u8>,
}

impl G29Interface {
    /// Initializes a new G29 driver, opens the device, and sets initial state.
    pub fn new() -> Self {
        let api = HidApi::new().unwrap();
        let device = api.open(0x046d, 0xc24f).unwrap();
        let mut state = HashMap::new();
        state.insert("steering", 255);
        state.insert("throttle", 255);
        state.insert("clutch", 255);
        state.insert("brake", 255);
        Self {
            device,
            cache: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], //Vec::new(),
            state,
        }
    }

    // Write to the G29Interface Driver
    /// Resets the G29 device, including steering wheel calibration.
    /// calibration the steering wheel of the G2
    ///
    pub fn reset(&self) {
        self.device
            .write(&[0xf8, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00])
            .unwrap();
        self.device
            .write(&[0xf8, 0x09, 0x05, 0x01, 0x01, 0x00, 0x00])
            .unwrap();
        // wait for setting the calibration
        thread::sleep(Duration::from_secs(10));
    }
    /// Connects to the G29 device by pumping data and resetting.
    pub fn connect(&mut self) {
        self.pump(10);
        self.reset();
    }
    /// Sets constant force feedback on the G29 device.
    pub fn force_feedback_constant(&self, val: f32) {
        if val < 0.0 || val > 1.0 {
            panic!("Value must be in range of 0 to 1");
        }
        let val_scale = (val * 255.0).round() as u8;
        let msg = [0x14, 0x00, val_scale, 0x00, 0x00, 0x00, 0x00];
        self.device.write(&msg).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    /// Configures autocentering strength and rate.
    /// default value to be used strength = 0.5 and rate = 0.05                                                                                   
    pub fn set_autocenter(&self, strength: f32, rate: f32) {
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
        thread::sleep(Duration::from_secs(10));
    }

    /// Turns off force feedback on the G29 device.
    pub fn force_off(&self) {
        self.device
            .write(&[0xf3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            .unwrap();
    }

    /// Reads data from the G29 device buffer.
    pub fn pump(&mut self, timeout: i32) -> usize {
        let mut buf = [0u8; 16]; //16
        let data = self.device.read_timeout(&mut buf, timeout).unwrap();
        let byte_array = buf[..data].to_vec();

        if byte_array.len() >= 12 {
            self.update_state(&byte_array);
            self.cache = byte_array;
        }
        return data;
    }
    /// Continuously pumps data in a loop.
    pub fn read_loop(&mut self) {
        loop {
            self.pump(10);
        }
    }

    /// Updates the internal state based on the latest data.
    pub fn update_state(&mut self, byte_array: &Vec<u8>) {
        if self.cache.is_empty() {
            panic!("cache is Empty");
        }

        //Update state
        // steering
        if byte_array[4] != self.cache[4] || byte_array[5] != self.cache[5] {
            let steering_val = self.calculate_steering(&byte_array[5], &byte_array[4]);
            //println!("steering_scaled = {}", steering_val);
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

    /// Retrieves the current state of the G29 device.
    pub fn get_state(&self) -> &HashMap<&str, u8> {
        &self.state
    }
    /// Calculates the scaled steering value based on raw input.
    pub fn calculate_steering(&self, start: &u8, end: &u8) -> u8 {
        // start from 0 to 255
        // end from 0 to 255
        // scale between 0 -> 100
        let start_scale = (*start as f32 / 256.0) * (100.0 - (100.0 / 256.0));
        // scale between 0 -> 3
        let end_scale = (*end as f32 / 255.0) * (100.0 / 256.0);
        return (start_scale + end_scale).round() as u8;
    }

    /// Transform G29 Controller input to caral controlle
    /// throttle in range of 0 -> 1
    /// brake in range of  0 -> 1
    /// steer in range of -1 -> 1
    pub fn carla_vehicle_controle(&self) -> HashMap<String, f32> {
        let mut state_transform_carla = HashMap::new();
        let state = &self.state;

        let throttle_value = state.get("throttle").map_or(0.0, |&v| f32::from(v) / 255.0);
        let brake_value = state.get("brake").map_or(0.0, |&v| f32::from(v) / 255.0);
        let steering_value = state
            .get("steering")
            .map_or(0.0, |&v| self.normalize_steering_to_carla_steer(v));

        state_transform_carla.insert("throttle".to_string(), throttle_value);
        state_transform_carla.insert("brake".to_string(), brake_value);
        state_transform_carla.insert("steering".to_string(), steering_value);

        state_transform_carla
    }

    fn normalize_steering_to_carla_steer(&self, steering: u8) -> f32 {
        let normilize_steering = (steering as f32 / 127.0) - 1.0;
        normilize_steering
    }
}
