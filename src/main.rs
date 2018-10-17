mod instr;

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
    v: [u8; V_SIZE],
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
        println!();
    }

    fn skip(&mut self) {
        self.pc += 2;
    }

    fn step(&mut self, mem: &mut Memory) {
        let instr = ((mem.bytes[self.pc as usize] as u16) << 8) | (mem.bytes[(self.pc+1) as usize] as u16);
        let a: u8 = ((instr & 0xF000) >> 12) as u8;
        let x: u8 = ((instr & 0x0F00) >> 8) as u8;
        let y: u8 = ((instr & 0x00F0) >> 4) as u8;
        let z: u8 = (instr & 0x000F) as u8;
        let yz: u8 = (instr & 0x00FF) as u8;
        let xyz: u16 = (instr & 0x00FF) as u16;
        let vx = self.v[x as usize];
        let vy = self.v[y as usize];

        self.skip();

        match (a, y, z) {
            // CLS
            (0, 0xE, 0) => (),
            // RET
            (0, 0xE, 0xE) => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }
            // SYS xyz
            (0, _, _) => (),
            // JMP xyz
            (1, _, _) => {
                self.pc = xyz;
            },
            // CALL xyz
            (2, _, _) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = xyz;
                self.sp += 1;
            },

            // skip
            (3, _, _) => { if vx == yz { self.skip() } },
            (4, _, _) => { if vx != yz { self.skip() } },
            (5, _, _) => { if vx != vy { self.skip() } },

            // MOV Vx, yz
            (6, _, _) => { self.v[x as usize] = yz; },
            // ADD Vx, yz
            (7, _, _) => {
                let (res, bit) = vx.overflowing_add(yz);
                self.v[x as usize] = res;
                self.v[0xF] = bit as u8;
            },
            // MOV Vx, Vy
            (8, _, 0) => { self.v[x as usize] = vy; },
            // OR Vx, Vy
            (8, _, 1) => { self.v[x as usize] = vx | vy; },
            // AND Vx, Vy
            (8, _, 2) => { self.v[x as usize] = vx & vy; },
            // XOR Vx, Vy
            (8, _, 3) => { self.v[x as usize] = vx ^ vy; },
            // ADD Vx, Vy
            (8, _, 4) => {
                let (res, bit) = vx.overflowing_add(vy);
                self.v[x as usize] = res;
                self.v[0xF] = bit as u8;
            }
            // SUB Vx, Vy
            (8, _, 5) => {
                let (res, bit) = vx.overflowing_sub(vy);
                self.v[x as usize] = res;
                self.v[0xF] = (!bit) as u8;
            }
            // SHR Vx, 1
            (8, 0, 6) => {
                self.v[0xF] = vx & 1;
                self.v[x as usize] = vx >> 1;
            },
            // SUBN Vx, Vy
            (8, _, 7) => {
                let (res, bit) = vy.overflowing_sub(vx);
                self.v[x as usize] = res;
                self.v[0xF] = (!bit) as u8;
            },
            (8, 0, 0xE) => {
                let (res, bit) = vx.overflowing_shl(1);
                self.v[x as usize] = res;
                self.v[0xF] = (bit) as u8;
            },
            (9, _, 0) => {
                if vx != vy {
                    self.skip();
                }
            }

    /*
        (8, _, 7) => {
            *vx = *vy - *vx;
            // last bit
        },
        (8, _, 0xE) => {
            *vx <<= 1;
            //
        },
        (9, _, 0) => {
            if *vx != *vy {
                self.pc += 2;
            }
        },
        (0xA, _, _) => {
            self.i = xyz;
        }
*/
        _ => (),
    }
    }
}

fn main() {
    let mut mem = Memory::new();
    let mut reg = Registers::new();
    reg.dump();
    reg.step(&mut mem);
    reg.dump();
}
