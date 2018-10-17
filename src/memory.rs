use instr::Instr;

pub const MEMORY_SIZE: usize = 0x1000;

pub struct Memory {
    pub bytes: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { bytes: [0; MEMORY_SIZE] }
    }

    pub fn load_program(&mut self, program: &[Instr]) {
        for (i, instr) in program.iter().enumerate() {
            let b: u16 = instr.to();
            self.bytes[i * 2] = (b >> 8) as u8;
            self.bytes[i * 2 + 1] = (b & 0xFF) as u8;
        }
    }

    pub fn u16_at(&self, addr: u16) -> u16 {
        return ((self.bytes[addr as usize] as u16) << 8) |
                (self.bytes[(addr + 1) as usize] as u16);
    }
}
