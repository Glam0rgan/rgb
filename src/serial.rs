
pub struct Serial {
    hw: HardwareHandle,
    irq: Irq,
    data: u8,
    recv: u8,
    ctrl: u8,
    clock: usize,
}

impl Serial {
    pub fn new(hw: HardwareHandle, irq: Irq) -> Self {
        Self {
            hw,
            irq,
            data: 0,
            recv: 0,
            ctrl: 0,
            clock: 0,
        }
    }    
}