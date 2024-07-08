const MEM_SIZE: usize = 1024 * 1024;

pub struct Memory {
    pub memory_space: Vec<u8>,
    pub program_break: usize,
}

impl Memory {
    pub fn new() -> Self {
        Memory { 
            memory_space: vec![0; MEM_SIZE],
            program_break: MEM_SIZE / 3
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
}