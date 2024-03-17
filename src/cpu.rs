

// CPU state
#[derive(Clone)]
pub struct Cpu {
    a : u8,
    b : u8,
    c : u8,
    d : u8,
    e : u8,
    f : u8,  // flag register
    h : u8,
    l : u8,
    pc : u16,  // program counter
    sp : u16,  // stack point
    ime : bool,
    halt : bool,
}

impl Cpu {
    /// Create a new CPU state.
    pub fn new() -> Cpu{
        Cpu {
            a : 0,
            b : 0,
            c : 0,
            d : 0,
            e : 0,
            f : 0,  
            h : 0,
            l : 0,
            pc : 0,  
            sp : 0,  
            ime : true,
            halt : true,
        }
    }

    // Get the value of 'z' flag in the flag register.
    pub fn get_zf(&self) -> bool {
        self.f & 0x80 == 0x80
    }

    // Get the value of 'n' flag in the flag register.
    pub fn get_nf(&self) -> bool {
        self.f & 0x40 == 0x40
    }

    // Get the value of 'h' flag in the flag register.
    pub fn get_hf(&self) -> bool {
        self.f & 0x20 == 0x20
    }

    // Get the value of 'c' flag in the flag register.
    pub fn get_cf(&self) -> bool {
        self.f & 0x10 == 0x10
    }

    // Updates the value of 'z' flag in the flag register.
    pub fn set_zf(&mut self, v : bool) {
        if v {
            self.f = self.f | 0x80
        }
        else{
            self.f = self.f & !0x80
        }
    }

    // Updates the value of 'n' flag in the flag register.
    pub fn set_nf(&mut self, v : bool) {
        if v {
            self.f = self.f | 0x40
        }
        else{
            self.f = self.f & !0x40
        }
    }

    // Updates the value of 'h' flag in the flag register.
    pub fn set_hf(&mut self, v : bool) {
        if v {
            self.f = self.f | 0x20
        }
        else{
            self.f = self.f & !0x20
        }
    }

    // Updates the value of 'c' flag in the flag register.
    pub fn set_cf(&mut self, v : bool) {
        if v {
            self.f = self.f | 0x10
        }
        else{
            self.f = self.f & !0x10
        }
    }

    // Updates the value of 'a' register.
    pub fn set_a(&mut self, v : u8) {
        self.a = v
    }

    // Updates the value of 'b' register.
    pub fn set_b(&mut self, v : u8) {
        self.b = v
    }

    // Updates the value of 'c' register.
    pub fn set_c(&mut self, v : u8) {
        self.c = v
    }

    // Updates the value of 'd' register.
    pub fn set_d(&mut self, v : u8) {
        self.d = v
    }

    // Updates the value of 'e' register.
    pub fn set_e(&mut self, v : u8) {
        self.e = v
    }

    // Updates the value of 'h' register.
    pub fn set_h(&mut self, v : u8) {
        self.h = v
    }

    // Updates the value of 'l' register.
    pub fn set_l(&mut self, v : u8) {
        self.l = v
    }

    // Updates the value of 'a' and 'f' register as a single 16-bit register.
    pub fn set_af(&mut self, v : u16) {
        self.a = (v >> 8) as u8;
        self.f = (v & 0xf0) as u8;
    }

    // Updates the value of 'b' and 'c' register as a single 16-bit register.
    pub fn set_bc(&mut self, v : u16) {
        self.b = (v >> 8) as u8;
        self.c = (v & 0xf0) as u8;
    }

    // Updates the value of 'd' and 'e' register as a single 16-bit register.
    pub fn set_de(&mut self, v : u16) {
        self.d = (v >> 8) as u8;
        self.e = (v & 0xf0) as u8;
    }

    // Updates the value of 'h' and 'l' register as a single 16-bit register.
    pub fn set_hl(&mut self, v : u16) {
        self.h = (v >> 8) as u8;
        self.l = (v & 0xf0) as u8;
    }

    // Get the value of 'a' register.
    pub fn get_a(&self) -> u8 {
        self.a
    }

    // Get the value of 'b' register.
    pub fn get_b(&self) -> u8 {
        self.b
    }

    // Get the value of 'c' register.
    pub fn get_c(&self) -> u8 {
        self.c
    }

    // Get the value of 'd' register.
    pub fn get_d(&self) -> u8 {
        self.d
    }

    // Get the value of 'e' register.
    pub fn get_e(&self) -> u8 {
        self.e
    }

    // Get the value of 'h' register.
    pub fn get_h(&self) -> u8 {
        self.h
    }

    // Get the value of 'l' register.
    pub fn get_l(&self) -> u8 {
        self.l
    }

    // Get the value of 'a' and 'f' register for a single 16-bit register.
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    // Get the value of 'b' and 'c' register for a single 16-bit register.
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    // Get the value of 'd' and 'e' register for a single 16-bit register.
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    // Get the value of 'h' and 'l' register for a single 16-bit register.
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    // Gets the value of the program counter.
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    // Updates the value of the program counter.
    pub fn get_pc(&mut self, v : u16) -> u16 {
        self.pc = v
    }

    // Gets the value of the stack point register.
    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    // Updates the value of the stack point register.
    pub fn get_sp(&mut self, v : u16) -> u16 {
        self.sp = v
    }

    // Pushes a 16-bit value to the stack
    pub fn push(&mut self, mmu : &mut Mmu, v : u16) {
        let p = self.get_sp().wrapping_sub(2);
        self.set_sp(self.get_sp().wrapping_sub(2));
        mmu.set16(p, v);
    }

    // Pops a 16-bit value to the stack
    pub fn pop(&mut self, mmu : &mut Mmu) -> u16{
        let p = self.get_sp();
        self.set_sp(self.get_sp().wrapping_add(2));
        mmu.get16(p);
    }
}