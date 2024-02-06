//! 
//! This module provides functionality to run a separate thread and
//! initiate reading from the G29 controller
//!
//! 
use crate::interface::G29Interface;
use std::{
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
}