
// The variants to control memory read access from the CPU
pub enum MemRead{
    // Replaces the value passed from the memory to the cpu.
    Replace(u8),
    // Shows the actual value passed from the memory to the cpu.
    PassThrough,
}

// The variants to control memory read access from the CPU
pub enum MemWrite{
    // Replaces the value to be written by the cpu to the memory.
    Replace(u8),
    // Allows to write the original value from the cpu to the memory.
    PassThrough,
    // Discard the write access from the cpu.
    Block,
}

// The handler to intercept memory access from the cpu.
pub trait MemHandler{
    // The function is called when the cpu attempts to read from the memory.
    fn on_read(&self, mmu : &Mmu, addr : u16) -> MemRead;
    // The function is called when the cpu attempts to write to the memory.
    fn on_write(&self, mmu : &Mmu, addr : u16, value : u8) -> MemWrite;
}

pub struct Handle(u64);

// The memory management unit (MMU)
/// This unit holds a memory byte array which represents address space of the memory.
/// It provides the logic to intercept access from the CPU to the memory byte array,
/// and to modify the memory access behaviour.
pub struct Mmu{
    ram : Vec<u8>,
    handles : HashMap<Handle, (u16,u16)>,
    handlers : HashMap<u16, Vec<>>,
    hdnum : u64,
}

impl Mmu{
    // Create a new mmu instance.
    pub fn new() -> Mmu {

    }

    // Reads one byte from the given address in the memory.
    pub fn get8(&self, addr : u16) -> u8 {
        if let Some(handlers) = self.handlers.get(&addr){
            for (_, handler) in handlers {
                match handler.on_read(self, addr) {
                    MemRead::Replace(alt) => return alt,
                    MemRead::PassThrough => {}
                }
            }
        }

        if addr >= 0xe000 && addr < 0xfdff {
            // echo ram
            self.ram[addr as uszie - 0x2000]
        }
        else{
            self.ram[addr as usizs]
        }
    }

    // Writes one byte at the given address in the memory.
    pub fn set8(&self, addr : u16, v : u8){
        if let Some(handlers) = self.handlers.get(&addr){
            for (_, handler) in handlers {
                match handler.on_write(self, addr) {
                    MemWrite::Replace(alt) => {
                        self.ram[addr as usize] = alt;
                        return;
                    }
                    MemWrite::PassThrough => {}
                    MemWrite::Block => return,
                }
            }
        }

        if addr >= 0xe000 && addr < 0xfdff {
            // echo ram
            self.ram[addr as uszie - 0x2000] = v
        }
        else{
            self.ram[addr as usizs] = v
        }
    }

    // Reads two bytes.
    pub fn get16(&self, addr : u16) -> u16{
        let l = self.get8(addr);
        let h = self.get8(addr + 1);
        (h as u16) << 8 | l as u16
    }

    // Writes two bytes.
    pub fn set16(&mut self, addr : u16, v : u16) {
        self.set8(addr, v as u8);
        self.set8(addr + 1, (v >> 8) as u8);
    }
}