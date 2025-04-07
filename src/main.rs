use std::process::Command;
use std::thread;
use std::{error::Error, time::Duration};

use rppal::gpio::{Gpio, Trigger}; // 0.15.0

pub const BTN1: u8 = 5;
pub const BTN2: u8 = 6;
pub const BTN3: u8 = 16;
pub const BTN4: u8 = 24;
pub const BACKLIGHT: u8 = 13;

fn shutdown() {
    let status = Command::new("/usr/sbin/halt")
        .status()
        .expect("Failed to execute halt");

    if status.success() {
        println!("Halting the system");
    } else {
        eprintln!("process finished with: {status}");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Turn off the backlight
    let mut backlight = Gpio::new()?.get(BACKLIGHT)?.into_output_low();
    backlight.set_reset_on_drop(false);

    let debounce = Some(Duration::from_millis(50));
    // let mut btns = [
    //     Gpio::new()?.get(BTN1)?.into_input_pullup(),
    //     Gpio::new()?.get(BTN2)?.into_input_pullup(),
    //     Gpio::new()?.get(BTN3)?.into_input_pullup(),
    //     Gpio::new()?.get(BTN4)?.into_input_pullup(),
    // ];
    // for (_i, btn) in btns.iter_mut().enumerate() {
    //     btn.set_reset_on_drop(false);
    //     btn.set_async_interrupt(Trigger::FallingEdge, debounce, |_| match shutdown() {
    //         Ok(_) => println!("Shutting down, bye!"),
    //         Err(error) => eprintln!("Failed to shut down: {}", error),
    //     })?;
    // }

    // println!("Press any button to shutdown");
    // let mut s = String::new();
    // io::stdin().read_line(&mut s)?;

    let mut pin_a = Gpio::new()?.get(BTN1)?.into_input_pullup();

    pin_a.set_reset_on_drop(false);
    pin_a.set_interrupt(Trigger::FallingEdge, debounce)?;

    println!("waiting for button a!");
    pin_a.poll_interrupt(true, None)?;
    println!("button a pressed!");

    // blink screen to indicate something we got the button press
    backlight.set_high();
    thread::sleep(Duration::from_millis(500));
    backlight.set_low();

    shutdown();

    Ok(())
}

// use std::error::Error;
// use std::thread;
// use std::time::Duration;

// use rppal::gpio::Gpio;
// use rppal::system::DeviceInfo;

// // Gpio uses BCM pin numbering. BCM GPIO 13 is the backlight
// const GPIO_LED: u8 = 13;

// fn main() -> Result<(), Box<dyn Error>> {
//     println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

//     let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

//     // Blink the LED by setting the pin's logic level high for 500 ms.
//     println!("Before");
//     pin.set_high();
//     thread::sleep(Duration::from_millis(1000));
//     pin.set_low();
//     thread::sleep(Duration::from_millis(1000));
//     pin.set_high();
//     println!("After");

//     Ok(())
// }
