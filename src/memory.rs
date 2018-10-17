use instr::Instr;

pub const MEMORY_SIZE: usize = 0x1000;
pub const FONT_SIZE: usize = 0x1000;

pub struct Memory {
    pub bytes: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { bytes: [0; MEMORY_SIZE] }
    }

    pub fn load_font(&mut self) {
        let font = parse_font(FONT_STR);
        self.bytes[0..MEMORY_SIZE].copy_from_slice(&font[..]);
    }

    pub fn load_program(&mut self, addr: usize, program: &[Instr]) {
        for (i, instr) in program.iter().enumerate() {
            let b: u16 = instr.to();
            self.bytes[addr + i * 2] = (b >> 8) as u8;
            self.bytes[addr + i * 2 + 1] = (b & 0xFF) as u8;
        }
    }

    pub fn u16_at(&self, addr: usize) -> u16 {
        return ((self.bytes[addr] as u16) << 8) |
                (self.bytes[addr + 1] as u16);
    }
}

fn parse_font(s: &str) -> [u8; FONT_SIZE] {
    let mut result = [0; FONT_SIZE];
    for (i, letter) in s.split(".").enumerate() {
        for (j, line) in letter.trim().lines().enumerate() {
            let mut a: u8 = 0;
            if line.get(0..1) == Some("#") { a |= 0x80; }
            if line.get(1..2) == Some("#") { a |= 0x40; }
            if line.get(2..3) == Some("#") { a |= 0x20; }
            if line.get(3..4) == Some("#") { a |= 0x10; }
            result[i*5 + j] = a;
        }
    }
    result
}

const FONT_STR: &str = r"
####
#  #
#  #
#  #
####
.
  #
 ##
  #
  #
 ###
.
####
   #
####
#
####
.
####
   #
####
   #
####
.
#  #
#  #
####
   #
   #
.
####
#
####
   #
####
.
####
#
####
#  #
####
.
####
   #
  #
 #
 #
.
####
#  #
####
#  #
####
.
####
#  #
####
   #
####
.
####
#  #
####
#  #
#  #
.
###
#  #
###
#  #
###
.
####
#
#
#
####
.
###
#  #
#  #
#  #
###
.
####
#
####
#
####
.
####
#
####
#
#
";
