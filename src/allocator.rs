use crate::memory::Memory;

pub struct Allocator {
    memory: Memory,
}

impl Allocator {
    pub fn new() -> Self {
        Allocator { 
            memory: Memory::new()
        }
    }

    pub fn malloc(&mut self, size: usize) -> Option<usize> {
            self.memory.allocate(size)
    }
}