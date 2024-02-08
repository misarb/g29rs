use g29::controller::Controller;

fn main() {
    let mut g29 = Controller::new();

    // set force feedback for G29 controller - make sure to set the Logitech to PS3 Mode
    //binding.force_feedback_constant(0.6);
    // Start the reading thread to continuously read input from the G29 device
    g29.start_pumping();
    //let binding = g29.g29.lock().unwrap();

    println!("thread spawned");
    //let state = binding.carla_vehicle_controle();
    //    g29.g29.lock().unwrap().set_autocenter(0.5, 0.05);
    loop {
        // println!("throttle = {:?}", state.get("throttle"));
        println!("steering = {:?}", g29.g29.lock().unwrap().get_state());

        //println!("steering = {:?}", binding.get_state());
    }
}
