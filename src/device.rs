use core::cell::{Ref, RefCell, RefMut};

use crate::mmu;

// The wrapper type for I/O handlers to register to MMU.
pub struct Device<T>(Rc<RefCell<T>>, bool);

impl<T> Device<T> {
    // Create a new device.
    pub fn new(inner : T) -> Self {
        Self::inner(inner, false)
    }
    // Create a new mediater device.
    pub fn nmediate(inner : T) -> Self {
        Self::inner(inner, true)
    }

    fn inner(inner : T, debug : bool) -> Self {
        Self(Rc::new(RefCell::new(inner)), debug)
    }

    // Immutably borrow the underlying I/O handler.
    pub fn borrow<'a>(&'a self) -> Ref<'a,T> {
        self.0.borrow()
    }

    // Mutablly borrow the underlying I/O handler.
    pub fn borrow<'a>(&'a self) -> RefMut<'a,T> {
        self.0.borrow_mut()
    }
}

impl<T :IoHandler> Device<T> {
    // Return the memory-mapped I/O handler of the device.
    pub fn handler(&self) -> IoMemHandler<T> {
        IoMemHandler(self.0.clone(), self.1)
    }
}

// The trait which allows to hook I/O access from the CPU.
pub trait IoHandler {
   // The function is called when the cpu attempts to read from the memory.
    fn on_read(&self, mmu : &Mmu, addr : u16) -> MemRead;
    // The function is called when the cpu attempts to write to the memory.
    fn on_write(&self, mmu : &Mmu, addr : u16, value : u8) -> MemWrite;
}

// The handler to intercept memory-mapped I/O.
pub struct IoMemHandler<T>(Rc<RefCell<T>,bool);

impl<T : IoHandler> MemHandler for IoMemHandler {
    fn on_read(&self, mmu : &Mmu, addr : u16) -> MemRead {
        // Don't hook if it's already hooked
        match self.0.try_borrow_mut() {
            Ok(mut inner) => inner.on_read(mmu, addr),
            Err(e) => {
                if self.1 {
                    // In mediator mode, allow to recurisive read.
                    MemRead:PassThrough
                }
                else{
                    // panic
                }
            }
        }
    }

    fn on_write(&self, mmu : &Mmu, addr : u16, value : u8) -> MemWrite {
        // Don't hook if it's already hooked
        match self.0.try_borrow_mut() {
            Ok(mut inner) => inner.on_write(mmu, addr, value),
            Err(e) => {
                if self.1 
                    // In mediator mode, allow to recurisive write.
                    MemWrite:PassThrough
                }
                else{
                    // panic
                }
            }
        }
    }
}