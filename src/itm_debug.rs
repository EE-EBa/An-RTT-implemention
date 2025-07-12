use cortex_m::peripheral;
use core::ptr;

pub fn itm_init(cp: &mut peripheral::Peripherals) {
    unsafe {
        // enable TPIU and ITM
        cp.DCB.demcr.modify(|r| r | (1 << 24));

        // prescaler
        let swo_freq = 2_000_000;
        cp.TPIU.acpr.write((16_000_000 / swo_freq) - 1);

        // SWO NRZ
        cp.TPIU.sppr.write(2);

        cp.TPIU.ffcr.modify(|r| r & !(1 << 1));

        // STM32 specific: enable tracing in the DBGMCU_CR register
        const DBGMCU_CR: *mut u32 = 0xe0042004 as *mut u32;
        let r = ptr::read_volatile(DBGMCU_CR);
        ptr::write_volatile(DBGMCU_CR, r | (1 << 5));

        // unlock the ITM
        cp.ITM.lar.write(0xC5ACCE55);

        cp.ITM.tcr.write(
            (0b000001 << 16) | // TraceBusID
            (1 << 3) | // enable SWO output
            (1 << 0), // enable the ITM
        );

        // enable stimulus port 0
        cp.ITM.ter[0].write(1);
    }
}
