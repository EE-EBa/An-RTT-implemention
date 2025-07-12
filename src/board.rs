use crate::mcu;
pub const BLUE_LED_PIN: u32 = mcu::GPIO_PIN_3;
pub const BLUE_LED_PORT: u32 = mcu::GPIOA_BASE;

pub const GREEN_LED_PIN: u32 = mcu::GPIO_PIN_2;
pub const GREEN_LED_PORT: u32 = mcu::GPIOA_BASE;

pub const RED_LED_PIN: u32 = mcu::GPIO_PIN_6;
pub const RED_LED_PORT: u32 = mcu::GPIOA_BASE;

pub const BUTTON_PORT: u32 = mcu::GPIOB_BASE;
pub const BUTTON_PIN: u32 = mcu::GPIO_PIN_14;
