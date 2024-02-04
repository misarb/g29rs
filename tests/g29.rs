// use std::{sync::{Arc, Mutex}, thread};
// use g29::{G29, G29Driver};

// trait DeviceInteraction {
//     fn new() -> Self;
//     fn start_pumping(&mut self);
//     fn stop_pumping(&mut self);
//     fn get_reading_thread(&self) -> Option<thread::JoinHandle<()>>;
// }

// // Implement the trait for the actual G29Driver
// impl DeviceInteraction for G29 {
//     fn new() -> Self {
//         Self {
//             g29: Arc::new(Mutex::new(G29Driver::new())),
//             reading_thread: None,
//         }
//     }

//     fn start_pumping(&mut self) {
//         let local_g29 = self.g29.clone();
//         self.reading_thread = Some(thread::spawn(move || {
//             local_g29.lock().unwrap().read_loop();
//         }));
//     }

//     fn stop_pumping(&mut self) {
//         if let Some(handle) = self.reading_thread.take() {
//             handle.join().unwrap();
//         } else {
//             println!("No Thread spawned");
//         }
//     }

//     fn get_reading_thread(&self) -> Option<thread::JoinHandle<()>> {
//         self.reading_thread.clone()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::thread::sleep;
//     use std::time::Duration;

//     // Implement a mock device for testing purposes
//     struct MockDevice;

//     impl DeviceInteraction for MockDevice {
//         fn new() -> Self {
//             MockDevice
//         }

//         fn start_pumping(&mut self) {
//             // Implement mock behavior for starting pumping
//         }

//         fn stop_pumping(&mut self) {
//             // Implement mock behavior for stopping pumping
//         }

//         fn get_reading_thread(&self) -> Option<thread::JoinHandle<()>> {
//             None
//         }
//     }

//     #[test]
//     fn test_g29_start_stop_pumping() {
//         // Use the mock device for testing
//         let mut g29 = MockDevice::new();

//         // Ensure that starting and stopping the pumping thread doesn't panic
//         g29.start_pumping();

//         // Sleep for a short duration to allow some pumping
//         sleep(Duration::from_secs(1));

//         g29.stop_pumping();
//         // Additional assertions can be added to check the state after stopping pumping
//     }
// }
