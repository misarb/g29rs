use g29::controller::Controller;

fn main() {
    let g29 = Controller::new();

    // set the steering to Auto centering
    g29.g29.lock().unwrap().set_autocenter(0.5, 0.05);

    // start puming values from the Logitech G29
    loop {
        // reading value from the G29 every 10ms
        g29.g29.lock().unwrap().pump(10);
        // get the values state in carla values
        println!(
            "steering = {:?}",
            g29.g29.lock().unwrap().carla_vehicle_controle()
        );
    }
}
