#G29rs 
Rust driver for logitech G29 
#Getting Started

#Prerequisites
 - Rust
 - Hidapi : "2.5.0"





#How to use 

```rust 
     let  g29 = G29::new();
     // write centering the wheel 
     g29.set_autocenter(0.5, 0.05);

     // setting Force feedbacki
     g29.force_feedback_constant(0.5);   
     
     // Reading from the G29
     // steel under test
```


#TODO

- [ ] Thread for reading data from G29
- [ ] Writing Test
- [ ] Make it as Lib after testing the full code 
- [ ] Reading reverse mode from button in the G29 controller 
- [ ] Make methode to transform the State for Carla user "throttle  [0 -> 1] brake [0 -> 1] brake[0->1]"

#Contributing
Contributions are welcome! If you have improvements, bug fixes, or new features to propose, please submit a pull request.


#support

only G29 logitech controller Driving wheel and pedals is support for ps3 mode

support Force feedback
