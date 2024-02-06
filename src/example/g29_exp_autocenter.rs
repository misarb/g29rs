/// This example illustrates the use case of this library
/// for seting the G29 controller to AutoCenter

use g29::interface::G29Interface;

fn main() {
    let mut control = G29Interface::new();
    // Set steering to autocenter
    control.set_autocenter(0.5, 0.05);
}
