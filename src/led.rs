//! LED control module for STM32 microcontrollers.
//! 
//! This module provides functionality to control LEDs connected to GPIO pins.
//! The implementation follows the common-anode LED connection where:
//! - LOW output turns the LED ON
//! - HIGH output turns the LED OFF
//! 
//! # Features
//! - LED initialization with proper GPIO configuration
//! - Basic LED control: ON, OFF, and Toggle operations
//! - Support for any GPIO port and pin combination

use crate::gpio::*;

/// Initializes an LED connected to the given gpio port and pin.
///
/// This function performs the following steps:
/// 1. Enable gpio clock
/// 2. Set the out type as push pull
/// 3. Set the output type
/// 4. Set the output speed (optional)
///
/// # Parameters
/// - port : The gpio port address to which the LED is connected
/// - pin : The gpio pin number to which the LED is connected
///
/// # Warning
/// Ensure proper GPIO port and pin numbers are provided
///
/// # Note
/// The LED is configured in common-anode mode
///
/// # Example
/// ```rust
///  led_init(GPIOA_BASE,GPIO_PIN_2);
/// ```
pub fn led_init(port: u32, pin: u32) {
    //1. Enable gpio clock for the LED pin
    enable_gpio_clock(port);
    // 2. Set the GPIO mode to output for LED control
    set_gpio_mode_output(port, pin);
    // 3. Configure push-pull output type for reliable LED control
    set_gpio_type_push_pull(port, pin);
    // 4. Set the output speed (optional) - default speed is sufficient for LEDs
}

/// Turns ON the LED connected to the specified GPIO port and pin.
///
/// For common-anode LED configuration, this sets the pin to LOW state.
///
/// # Parameters
/// - port: GPIO port address where LED is connected
/// - pin: Pin number where LED is connected
pub fn led_on(port: u32, pin: u32) {
    // Set pin to LOW state to turn ON the LED (common-anode configuration)
    set_gpio_pin_state(port, pin, PinState::Low);
}

/// Turns OFF the LED connected to the specified GPIO port and pin.
///
/// For common-anode LED configuration, this sets the pin to HIGH state.
///
/// # Parameters
/// - port: GPIO port address where LED is connected
/// - pin: Pin number where LED is connected
pub fn led_off(port: u32, pin: u32) {
    // Set pin to HIGH state to turn OFF the LED (common-anode configuration)
    set_gpio_pin_state(port, pin, PinState::High);
}

/// Toggles the state of the LED connected to the specified GPIO port and pin.
///
/// This function switches the LED from ON to OFF or OFF to ON.
///
/// # Parameters
/// - port: GPIO port address where LED is connected
/// - pin: Pin number where LED is connected
pub fn led_toggle(port: u32, pin: u32) {
    // Toggle the LED state using the PinState::Toggle option
    set_gpio_pin_state(port, pin, PinState::Toggle);
}
