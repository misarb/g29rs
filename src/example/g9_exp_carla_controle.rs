/// This example illustrates the use case of this library
/// for reading G29 input, without runing a separate thread and transform the
/// input to Carla controle Value
use g29::controller::Controller;

fn main() {
    let mut controle = Controller::new();
    loop {
        g29.g29.lock().unwrap().pump(10);
        println!(
            "Carla_controle = {:?}",
            g29.g29.lock().unwrap().carla_vehicle_controle()
        );
        println!(
            "State_Controlle = {:?}",
            g29.g29.lock().unwrap().get_state()
        );
    }
}
