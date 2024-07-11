use std::mem;

const MEM_SIZE: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy)]
struct Header {
    size: usize,
    is_free: bool,
    next: Option<usize>,
}

pub struct Memory {
    memory_space: Vec<u8>,
    program_break: usize,
    head: Option<usize>,
    tail: Option<usize>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { 
            memory_space: vec![0; MEM_SIZE],
            program_break: MEM_SIZE / 3,
            head: None,
            tail: None,
        }
    }

    pub fn sbrk(&mut self, increment_value: isize) -> Result<usize, &'static str> {
        let new_program_break = if increment_value >= 0 {
            self.program_break.checked_add(increment_value as usize)
        } else {
            self.program_break.checked_sub((-increment_value) as usize)
        };

        match new_program_break {
            Some(new_break) if new_break <= self.memory_space.len() => {
                self.program_break = new_break;
                Ok(self.program_break)
            }
            Some(_) => Err("Program Break has exceeded memory space!"),
            None => Err("Program Break arithmetic overflow!"),
        }
    }

    fn get_free_block(&self, size: usize) -> Option<usize> {
        let mut current = self.head;
        while let Some(offset) = current {
            let header = self.read_header(offset);
            if header.is_free && header.size >= size {
                return Some(offset);
            }
            current = header.next;
        }
        None
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if size == 0 {
            return None;
        }

        if let Some(block_offset) = self.get_free_block(size) {
            let mut header = self.read_header(block_offset);
            header.is_free = false;
            self.write_header(block_offset, header);
            return Some(block_offset + mem::size_of::<Header>());
        }

        let total_size = mem::size_of::<Header>() + size;
        match self.sbrk(total_size as isize) {
            Ok(new_break) => {
                let block_start = new_break - total_size;
                let header = Header {
                    size,
                    is_free: false,
                    next: None,
                };
                self.write_header(block_start, header);

                if self.head.is_none() {
                    self.head = Some(block_start);
                }
                if let Some(tail_offset) = self.tail {
                    let mut tail_header = self.read_header(tail_offset);
                    tail_header.next = Some(block_start);
                    self.write_header(tail_offset, tail_header);
                }
                self.tail = Some(block_start);

                Some(block_start + mem::size_of::<Header>())
            }
            Err(_) => None,
        }
    }

    fn read_header(&self, offset: usize) -> Header {
        let header_bytes = &self.memory_space[offset..offset + mem::size_of::<Header>()];
        unsafe { mem::transmute_copy(&header_bytes) }
    }

    fn write_header(&mut self, offset: usize, header: Header) {
        let header_bytes: [u8; mem::size_of::<Header>()] = unsafe { mem::transmute(header) };
        self.memory_space[offset..offset + mem::size_of::<Header>()].copy_from_slice(&header_bytes);
    }

    pub fn free(&mut self, block_offset: usize) {
        if block_offset == 0 {
            return;
        }

        let header_offset = block_offset - mem::size_of::<Header>();
        let mut header = self.read_header(header_offset);

        let program_break = self.program_break;
        if block_offset + header.size == program_break {
            // This block is at the end of the heap
            if self.head == Some(header_offset) && self.tail == Some(header_offset) {
                // This is the only block
                self.head = None;
                self.tail = None;
            } else {
                // Find the new tail
                let mut current = self.head;
                while let Some(curr_offset) = current {
                    let curr_header = self.read_header(curr_offset);
                    if curr_header.next == Some(header_offset) {
                        // Update the new tail
                        let mut new_tail = curr_header;
                        new_tail.next = None;
                        self.write_header(curr_offset, new_tail);
                        self.tail = Some(curr_offset);
                        break;
                    }
                    current = curr_header.next;
                }
            }

            // Shrink the heap
            let shrink_size = mem::size_of::<Header>() + header.size;
            if let Ok(_) = self.sbrk(-(shrink_size as isize)) {
                self.program_break -= shrink_size;
            }
        } else {
            // This block is not at the end, just mark it as free
            header.is_free = true;
            self.write_header(header_offset, header);
        }
    }

    pub fn calloc(&mut self, num: usize, nsize: usize) -> Option<usize> {
        // Check for zero size allocation
        if num == 0 || nsize == 0 {
            return None;
        }

        // Check for multiplication overflow
        let size = num.checked_mul(nsize)?;

        // Allocate memory
        let block_offset = self.allocate(size)?;

        // Zero out the allocated memory
        let start = block_offset;
        let end = start + size;
        self.memory_space[start..end].fill(0);

        Some(block_offset)
    }

    pub fn realloc(&mut self, block_offset: usize, new_size: usize) -> Option<usize> {
        // Handle null block or zero size
        if block_offset == 0 || new_size == 0 {
            return self.allocate(new_size);
        }

        let header_offset = block_offset - mem::size_of::<Header>();
        let header = self.read_header(header_offset);

        // If current block is large enough, just return it
        if header.size >= new_size {
            return Some(block_offset);
        }

        // Allocate new block
        let new_block_offset = self.allocate(new_size)?;

        // Copy data from old block to new block
        let copy_size = header.size.min(new_size);
        let src_start = block_offset;
        let src_end = src_start + copy_size;
        let dest_start = new_block_offset;
        self.memory_space.copy_within(src_start..src_end, dest_start);

        // Free old block
        self.free(block_offset);

        Some(new_block_offset)
    }
}