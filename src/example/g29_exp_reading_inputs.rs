/// This example illustrates the use case of this library 
/// for reading G29 input, including steering and pedals.
use g29::controller::Controller;

fn main() {
    let mut controle = Controller::new();
    // set force feedback for G29 controller - make sure to set the Logitech to PS3 Mode
    controle.g29.lock().unwrap().force_feedback_constant(0.6);
    // Start the reading thread to continuously read input from the G29 device
    controle.start_pumping();
    //g29.g29.lock().unwrap().set_autocenter(0.5, 0.05);
    loop {
        println!("steering = {:?}", g29.g29.lock().unwrap().get_state());
    }
}
