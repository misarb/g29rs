//!
//! This module provides functionality to run a separate thread and
//! initiate reading from the G29 controller
//!
//!
use crate::interface::G29Interface;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

// ## Controller Struct
/// The `Controller` struct represents the G29 device and provides methods for controlling and interacting with it.
#[derive(Debug)]
pub struct Controller {
    pub g29: Arc<Mutex<G29Interface>>,
    reading_thread: Option<JoinHandle<()>>,
}

impl Controller {
    /// Creates a new G29 instance.
    pub fn new() -> Self {
        Self {
            g29: Arc::new(Mutex::new(G29Interface::new())),
            reading_thread: None,
        }
    }
    /// Starts a thread to continuously read input from the G29 device.
    pub fn start_pumping(&mut self) {
        let local_g29 = self.g29.clone();
        self.reading_thread = Some(thread::spawn(move || {
            local_g29.lock().unwrap().read_loop();
        }));
    }

    /// Stops the reading thread.
    pub fn stop_pumping(&mut self) {
        if let Some(handle) = self.reading_thread.take() {
            handle.join().unwrap();
        } else {
            println!("No Thread spawned");
        }
    }
    
    /// Transforms the G29 controller state of throttle, brake, and steering control to Carla
    /// controller input, where Throttle is in the range [0, 1], Brake is in the range [0, 1],
    /// and Steering is in the range [-1, 1].

    pub fn carla_vehicle_controle(&self) -> HashMap<&str, f32> {
        let lock = self.g29.lock().unwrap();
        let state = lock.get_state();
        let mut state_transform_carla = HashMap::new();
        if let (Some(throttle), Some(brake), Some(steering)) = (
            state.get("throttle"),
            state.get("brake"),
            state.get("steering"),
        ) {
            println!("Throttle: {}, Brake: {}", throttle, brake);
            state_transform_carla
                .insert("steering", self.normalize_steering_to_carla_steer(*steering));
            state_transform_carla.insert("throttle", f32::from(*throttle) / 255.0);
            state_transform_carla.insert("brake", f32::from(*brake) / 255.0);
        } else {
            println!("Error: no State from the G29 controlelr");
            state_transform_carla.insert("throttle", 0.0);
            state_transform_carla.insert("brake", 0.0);
        }
        state_transform_carla
    }

    fn normalize_steering_to_carla_steer(&self,steering: u8) -> f32 {
        let normilize_steering = (steering as f32 / 127.0) - 1.0;
        normilize_steering
    }
}
