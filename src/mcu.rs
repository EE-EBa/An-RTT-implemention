pub const GPIOA_BASE: u32 = 0x4800_0000;
pub const GPIOB_BASE: u32 = GPIOA_BASE + 0x400;
pub const GPIOC_BASE: u32 = GPIOA_BASE + 0x800;

pub const SYSCFG_BASE: u32 = 0x4001_0000;

pub const EXTI_BASE: u32 = 0x4001_0400;

pub const RCC_BASE: u32 = 0x4002_1000;
pub const GPIO_PIN_0: u32 = 0;
pub const GPIO_PIN_1: u32 = 1;
pub const GPIO_PIN_2: u32 = 2;
pub const GPIO_PIN_3: u32 = 3;
pub const GPIO_PIN_4: u32 = 4;
pub const GPIO_PIN_5: u32 = 5;
pub const GPIO_PIN_6: u32 = 6;
pub const GPIO_PIN_7: u32 = 7;
pub const GPIO_PIN_8: u32 = 8;
pub const GPIO_PIN_9: u32 = 9;
pub const GPIO_PIN_10: u32 = 10;
pub const GPIO_PIN_11: u32 = 11;
pub const GPIO_PIN_12: u32 = 12;
pub const GPIO_PIN_13: u32 = 13;
pub const GPIO_PIN_14: u32 = 14;
pub const GPIO_PIN_15: u32 = 15;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub enum IRQn {
    WWDG = 0,                // Window Watchdog interrupt
    PVD_PVM = 1,             // PVD through EXTI line 16 interrupt
    RTC_TAMP_CSS_LSE = 2,    // RTC/TAMP/CSS on LSE through EXTI line 19 interrupt
    RTC_WKUP = 3,            // RTC wake-up timer through EXTI line 20 interrupt
    FLASH = 4,               // Flash global interrupt
    RCC = 5,                 // RCC global interrupt
    EXTI0 = 6,               // EXTI Line0 interrupt
    EXTI1 = 7,               // EXTI Line1 interrupt
    EXTI2 = 8,               // EXTI Line2 interrupt
    EXTI3 = 9,               // EXTI Line3 interrupt
    EXTI4 = 10,              // EXTI Line4 interrupt
    DMA1_CH1 = 11,           // DMA1 channel 1 interrupt
    DMA1_CH2 = 12,           // DMA1 channel 2 interrupt
    DMA1_CH3 = 13,           // DMA1 channel 3 interrupt
    DMA1_CH4 = 14,           // DMA1 channel 4 interrupt
    DMA1_CH5 = 15,           // DMA1 channel 5 interrupt
    DMA1_CH6 = 16,           // DMA1 channel 6 interrupt
    DMA1_CH7 = 17,           // DMA1 channel 7 interrupt
    ADC1_2 = 18,             // ADC1 and ADC2 global interrupt
    USB_HP = 19,             // USB High priority interrupts
    USB_LP = 20,             // USB Low priority interrupts
    FDCAN1_IT0 = 21,         // FDCAN1 interrupt 0
    FDCAN1_IT1 = 22,         // FDCAN1 interrupt1
    EXTI9_5 = 23,            // EXTI Line[9:5] interrupts
    TIM1_BRK_TIM15 = 24,     // TIM1 Break/TIM15 global interrupts
    TIM1_UP_TIM16 = 25,      // TIM1 Update/TIM16 global interrupts
    TIM1_TRG_COM_TIM17 = 26, // TIM1 trigger and commutation/TIM17 interrupts
    TIM1_CC = 27,            // TIM1 capture compare interrupt
    TIM2 = 28,               // TIM2 global interrupt
    TIM3 = 29,               // TIM3 global interrupt
    TIM4 = 30,               // TIM4 global interrupt
    I2C1_EV = 31,            // I2C1 event interrupt
    I2C1_ER = 32,            // I2C1 error interrupt
    I2C2_EV = 33,            // I2C2 event interrupt
    I2C2_ER = 34,            // I2C2 error interrupt
    SPI1 = 35,               // SPI1 global interrupt
    SPI2 = 36,               // SPI2 global interrupt
    USART1 = 37,             // USART1 global interrupt
    USART2 = 38,             // USART2 global interrupt
    USART3 = 39,             // USART3 global interrupt
    EXTI15_10 = 40,          // EXTI line[15:10] interrupts
    RTC_ALARM = 41,          // RTC alarms interrupts
    USB_WAKEUP = 42,         // USB wake-up from suspend
    TIM8_BRK = 43,           // TIM8 Break interrupt
    TIM8_UP = 44,            // TIM8 Update interrupt
    TIM8_TRG_COM = 45,       // TIM8 trigger and commutation interrupt
    TIM8_CC = 46,            // TIM8 capture compare interrupt
    ADC3 = 47,               // ADC3 global interrupt
    FSMC = 48,               // FSMC global interrupt
    LPTIM1 = 49,             // LPTIM1 global interrupt
    TIM5 = 50,               // TIM5 global interrupt
    SPI3 = 51,               // SPI3 global interrupt
    UART4 = 52,              // UART4 global interrupt
    UART5 = 53,              // UART5 global interrupt
    TIM6_DACUNDER = 54,      // TIM6 and DAC1/3 underrun global interrupts
    TIM7_DACUNDER = 55,      // TIM7 and DAC2/4 underrun global interrupts
    DMA2_CH1 = 56,           // DMA2 channel 1 interrupt
    DMA2_CH2 = 57,           // DMA2 channel 2 interrupt
    DMA2_CH3 = 58,           // DMA2 channel 3 interrupt
    DMA2_CH4 = 59,           // DMA2 channel 4 interrupt
    DMA2_CH5 = 60,           // DMA2 channel 5 interrupt
    ADC4 = 61,               // ADC4 global interrupt
    ADC5 = 62,               // ADC5 global interrupt
    UCPD1 = 63,              // UCPD1 global interrupt
    COMP1_2_3 = 64,          // COMP1/COMP2/COMP3 interrupts
    COMP4_5_6 = 65,          // COMP4/COMP5/COMP6 interrupts
    COMP7 = 66,              // COMP7 interrupt
    HRTIM_MASTER = 67,       // HRTIM master timer interrupt
    HRTIM_TIMA = 68,         // HRTIM timer A interrupt
    HRTIM_TIMB = 69,         // HRTIM timer B interrupt
    HRTIM_TIMC = 70,         // HRTIM timer C interrupt
    HRTIM_TIMD = 71,         // HRTIM timer D interrupt
    HRTIM_TIME = 72,         // HRTIM timer E interrupt
    HRTIM_FLT = 73,          // HRTIM fault interrupt
    HRTIM_TIMF = 74,         // HRTIM timer F interrupt
    CRS = 75,                // CRS interrupt
    SAI = 76,                // SAI interrupt
    TIM20_BRK = 77,          // TIM20 Break interrupt
    TIM20_UP = 78,           // TIM20 Update interrupt
    TIM20_TRG_COM = 79,      // TIM20 Trigger and commutation interrupt
    TIM20_CC = 80,           // TIM20 capture compare interrupt
    FPU = 81,                // Floating point interrupt
    I2C4_EV = 82,            // I2C4 event interrupt
    I2C4_ER = 83,            // I2C4 error interrupt
    SPI4 = 84,               // SPI4 global interrupt
    AES = 85,                // AES global interrupt
    FDCAN2_IT0 = 86,         // FDCAN2 Interrupt 0
    FDCAN2_IT1 = 87,         // FDCAN2 Interrupt 1
    FDCAN3_IT0 = 88,         // FDCAN3 Interrupt 0
    FDCAN3_IT1 = 89,         // FDCAN3 Interrupt 1
    RNG = 90,                // RNG global interrupt
    LPUART = 91,             // LPUART global interrupt
    I2C3_EV = 92,            // I2C3 event interrupt
    I2C3_ER = 93,            // I2C3 error interrupt
    DMAMUX_OVR = 94,         // DMAMUX overrun interrupt
    QUADSPI = 95,            // QUADSPI global interrupt
    DMA1_CH8 = 96,           // DMA1 channel 8 interrupt
    DMA2_CH6 = 97,           // DMA2 channel 6 interrupt
    DMA2_CH7 = 98,           // DMA2 channel 7 interrupt
    DMA2_CH8 = 99,           // DMA2 channel 8 interrupt
    CORDIC = 100,            // CORDIC interrupt
    FMAC = 101,              // FMAC interrupt
}

impl IRQn {
    pub fn from_pin(pin: u32) -> Option<u32> {
        match pin {
            0 => Some(IRQn::EXTI0 as u32),
            1 => Some(IRQn::EXTI1 as u32),
            2 => Some(IRQn::EXTI2 as u32),
            3 => Some(IRQn::EXTI3 as u32),
            4 => Some(IRQn::EXTI4 as u32),
            5..=9 => Some(IRQn::EXTI9_5 as u32),
            10..=15 => Some(IRQn::EXTI15_10 as u32),
            _ => None,
        }
    }
}
