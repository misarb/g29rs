/// This example illustrates the use case of this library
/// for reading G29 input, without runing a separate thread and transform the
/// input to Carla controle Value
use g29::interface::G29Interface;

fn main() {
    let mut controle = G29Interface::new();
    loop {
        controle.pump(10);
        println!("Carla_controle = {:?}", controle.carla_vehicle_controle());
        println!("State_Controlle = {:?}", controle.get_state());
    }
}
