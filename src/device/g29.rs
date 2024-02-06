//! ## Usage
//!
//! ```rust
//! use g29::{G29, G29Driver};
//!
//! fn main() {
//!     // Create a new G29 instance
//!     let mut g29 = G29::new();
//!     // set force feedback for G29 controller - make sure to set the Logitech to PS3 Mode
//!     g29.g29.lock().unwrap().force_feedback_constant(0.6);
//!     // Start the reading thread to continuously read input from the G29 device
//!     g29.start_pumping();
//!     loop {
//!           println!("steering = {:?}", g29.g29.lock().unwrap().get_state());
//!      }
//! }
//! ```
//! 
//! 
//
// 

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};
use crate::device::g29_driver::G29Driver;
// ## G29 Struct
/// The `G29` struct represents the G29 device and provides methods for controlling and interacting with it.

#[derive(Debug)]
pub struct G29 {
    pub g29: Arc<Mutex<G29Driver>>,
    reading_thread: Option<JoinHandle<()>>,
}

impl G29 {
    /// Creates a new G29 instance.
    pub fn new() -> Self {
        Self {
            g29: Arc::new(Mutex::new(G29Driver::new())),
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
}