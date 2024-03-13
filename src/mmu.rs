
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

    
}