use std::thread::JoinHandle;
use std::time::Duration;
use std::{
    sync::{Arc, Mutex},
    thread,
    
};

// Define a mock G29Driver
#[derive(Debug)]
pub struct MockG29Driver;

impl MockG29Driver {
    pub fn new() -> Self {
        Self
    }

    // Simulate the read_loop method
    pub fn read_loop(&self) {
        // Simulate reading input from the G29 device
        thread::sleep(Duration::from_millis(100));
    }
}

// Create a mock implementation of G29 for testing
#[derive(Debug)]
pub struct MockG29 {
    pub g29: Arc<Mutex<MockG29Driver>>,
    reading_thread: Option<JoinHandle<()>>,
}

impl MockG29 {
    pub fn new() -> Self {
        Self {
            g29: Arc::new(Mutex::new(MockG29Driver::new())),
            reading_thread: None,
        }
    }

    pub fn start_pumping(&mut self) {
        let local_g29 = self.g29.clone();
        self.reading_thread = Some(thread::spawn(move || {
            local_g29.lock().unwrap().read_loop();
        }));
    }

    pub fn stop_pumping(&mut self) {
        if let Some(handle) = self.reading_thread.take() {
            handle.join().unwrap();
        } else {
            println!("No Thread spawned");
        }
    }
}

// Example test case
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_stop_pumping() {
        let mut g29 = MockG29::new();

        g29.start_pumping();
        // Check if the reading thread is running
        assert!(g29.reading_thread.is_some(), "Thread not started");

        // Add assertions or additional tests here

        g29.stop_pumping();

        // Check if the reading thread is stopped
        assert!(g29.reading_thread.is_none(), "Thread not stopped");
    }
}
