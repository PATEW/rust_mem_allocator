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

    pub fn malloc(&mut self, size: usize) -> Option<*mut u8> {
            match self.memory.sbrk(size as isize) {
                Ok(new_break) => {

                    let block_start = new_break - size;
                    
                    unsafe {
                        Some(self.memory.memory_space.as_mut_ptr().add(block_start))
                    }
                },
                Err(_) => None,
            }
    }
}