use crate::mcu::*;
use crate::reg::*;



pub fn enable_gpio_clock(port: u32) {
    let rcc_ahb_enr_addr = (RCC_BASE + 0x4c) as *mut u32;
    match port {
        GPIOA_BASE => {
            /* Enable the GPIOA clock in RCC_AHB2ENR*/
            reg_set_bit(rcc_ahb_enr_addr, 0, true);
        }
        GPIOB_BASE => {
            reg_set_bit(rcc_ahb_enr_addr, 1, true);
        }
        _ => {
            todo!("Implement GPIOA_BASE and GPIOB_BASE port!")
        }
    }
}

pub fn set_gpio_mode_output(port: u32, pin: u32) {
    let gpio_mode_reg_addr: *mut u32 = (port + 0x00) as *mut u32;
    let bit_position: u32 = pin * 2;
    let mode_value: u32 = 0x1;

    reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_mode_input(port: u32, pin: u32) {
    let gpio_mode_reg_addr: *mut u32 = (port + 0x00) as *mut u32;
    let bit_position: u32 = pin * 2;
    let mode_value: u32 = 0x00;

    reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_type_push_pull(port: u32, pin: u32) {
    let gpio_op_type_reg_addr: *mut u32 = (port + 0x04) as *mut u32;
    let bit_position: u32 = pin;
    let mode_value: bool = false;

    reg_set_bit(gpio_op_type_reg_addr, bit_position, mode_value);
}

pub enum PinState {
    High,
    Low,
    Toggle,
}


pub fn get_gpio_pin_state(port: u32, pin: u32) -> bool {
    let gpio_odr_addr: *mut u32 = (port + 0x14) as *mut u32;
    reg_read_bit(gpio_odr_addr, pin)
}


pub fn set_gpio_pin_state(port: u32, pin: u32, state: PinState) {
    let gpio_bsrr_adrr: *mut u32 = (port + 0x18) as *mut u32;

    match state {
        PinState::Low => {
            reg_set_val(gpio_bsrr_adrr, 1 << (pin + 16));
        }
        PinState::High => {
            reg_set_val(gpio_bsrr_adrr, 1 << pin);
        }
        PinState::Toggle => {
            if get_gpio_pin_state(port, pin) {
                reg_set_val(gpio_bsrr_adrr, 1 << (pin + 16));
            } else {
                reg_set_val(gpio_bsrr_adrr, 1 << pin);
            }
        }
    }
}

