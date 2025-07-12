use crate::mcu::*;
use crate::reg::*;

pub mod gpio {
    use super::*;

    pub enum EdgeTrigger {
        Rising,
        Falling,
    }

    pub fn set_edge(pin: u32, edge: EdgeTrigger) {
        let exti_rtsr1_addr: *mut u32 = (EXTI_BASE + 0x08) as *mut u32;
        let exti_ftsr1_addr: *mut u32 = (EXTI_BASE + 0x0c) as *mut u32;

        match edge {
            EdgeTrigger::Rising => {
                reg_set_bit(exti_rtsr1_addr, pin, true);
            }
            EdgeTrigger::Falling => {
                reg_set_bit(exti_ftsr1_addr, pin, true);
            }
        }
    }

    pub fn configure_syscfg(port: u32, pin: u32) {
        let rcc_apb2enr_addr: *mut u32 = (RCC_BASE + 0x60) as *mut u32;
        reg_set_bit(rcc_apb2enr_addr, 0, true);

        let reg_offset: u32 = (pin / 4) * 4;
        let bit_position = (pin % 4) * 4;
        let syscfg_exicr_addr: *mut u32 = (SYSCFG_BASE + 0x08 + reg_offset) as *mut u32;

        match port {
            GPIOA_BASE => {
                reg_set_bits(syscfg_exicr_addr, 0, bit_position, 4);
            }
            GPIOB_BASE => {
                reg_set_bits(syscfg_exicr_addr, 1_u32, bit_position, 4);
            }
            _ => {}
        }
    }
}

pub enum ExtiLine {
    Line0 = 0,
    Line1,
    Line2,
    Line3,
    Line4,
    Line5,
    Line6,
    Line7,
    Line8,
    Line9,
    Line10,
    Line11,
    Line12,
    Line13,
    Line14,
    Line15,
    Line16,
    Line17,
    Line18,
    Line19,
    Line20,
    Line21,
    Line22,
    Line23,
    Line24,
    Line25,
    Line26,
    Line27,
    Line28,
    Line29,
    Line30,
    Line31,
    Line32,
    Line33,
    Line34,
    Line35,
    Line36,
    Line37,
    Line38,
    Line39,
    Line40,
    Line41,
    Line42,
    Line43,
}
impl ExtiLine {
    pub fn from_pin(pin: u32) -> Option<ExtiLine> {
        match pin {
            0 => Some(ExtiLine::Line0),
            1 => Some(ExtiLine::Line1),
            2 => Some(ExtiLine::Line2),
            3 => Some(ExtiLine::Line3),
            4 => Some(ExtiLine::Line4),
            5 => Some(ExtiLine::Line5),
            6 => Some(ExtiLine::Line6),
            7 => Some(ExtiLine::Line7),
            8 => Some(ExtiLine::Line8),
            9 => Some(ExtiLine::Line9),
            10 => Some(ExtiLine::Line10),
            11 => Some(ExtiLine::Line11),
            12 => Some(ExtiLine::Line12),
            13 => Some(ExtiLine::Line13),
            14 => Some(ExtiLine::Line14),
            15 => Some(ExtiLine::Line15),
            _ => None,
        }
    }
}

fn configure_interrupt(exti_line: ExtiLine, is_enable: bool) {
    let exti_imr1_addr: *mut u32 = (EXTI_BASE) as *mut u32;
    let exti_imr2_addr: *mut u32 = (EXTI_BASE + 0x20) as *mut u32;
    let line: u32 = exti_line as u32;
    match line {
        0..=31 => {
            reg_set_bit(exti_imr1_addr, line, is_enable);
        }
        32..=42 => {
            reg_set_bit(exti_imr2_addr, line, is_enable);
        }
        _ => {}
    }
}

pub fn enable_interrupt(exti_line: ExtiLine) {
    configure_interrupt(exti_line, true);
}

pub fn disable_interrupt(exti_line: ExtiLine) {
    configure_interrupt(exti_line, false);
}

pub fn clear_pending_interrupt(exti_line: ExtiLine) {
    let exti_pr1_addr: *mut u32 = (EXTI_BASE + 0x14) as *mut u32;
    let line: u32 = exti_line as u32;
    reg_set_bit(exti_pr1_addr, line, true);
}
