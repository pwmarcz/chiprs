const MEMORY_SIZE: usize = 0x1000;
const V_SIZE: usize = 0x10;
const STACK_SIZE: usize = 0x10;

struct Memory {
    bytes: [u8; MEMORY_SIZE],
}

impl Memory {
    fn new() -> Memory {
        Memory { bytes: [0; MEMORY_SIZE] }
    }
}

struct Registers {
    v: [i8; V_SIZE],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
}

impl Registers {
    fn new() -> Registers {
        Registers {
            v: [0; V_SIZE],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; STACK_SIZE]
        }
    }

    fn dump(&self) {
        for i in 0..V_SIZE {
            print!("V{:X} ", i);
        }
        println!("   I    PC  SP");
        for i in 0..V_SIZE {
            print!("{:02X} ", self.v[i]);
        }
        println!("  {:02x}  {:04x}  {:02x}", self.i, self.pc, self.sp);
        println!();
        for i in 0..STACK_SIZE {
            print!("S[{:X}] ", i);
        }
        println!();
        for i in 0..V_SIZE {
            print!("{:04X} ", self.stack[i]);
        }
        println!();
    }
}

fn main() {
    let memory = Memory::new();
    let registers = Registers::new();
    registers.dump();
}
