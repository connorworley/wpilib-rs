# wpilib-rs
wpilib-rs is a Rust port of [WPILib](https://github.com/wpilibsuite/allwpilib). The project binds the C library for the RoboRIO HAL, with WPILib C++ classes ported to pure Rust.

## Usage
Using wpilib-rs is easy. For example, periodically reading the voltage on an analog input pin can be accomplished like so:
```rust
extern crate wpilib;

use std::{time,thread};

fn main() {
    let status = unsafe { wpilib::hal::HAL_Initialize(0) };
    assert!(status == 0, "HAL failed to initialize!");
    let input = wpilib::AnalogInput::fromChannel(1);
    loop {
        println!("Value: {}", input.get_value());
        println!("Voltage: {}", input.get_voltage());
        let wait_time = time::Duration::from_millis(500);
        thread::sleep(wait_time);
    }
}
```

## Roadmap
So far, only a stripped-down version of the `AnalogInput` class has been ported as a proof of concept. As more contributors come onboard, we will have to create a plan for porting the remaining classes.
