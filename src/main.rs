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

    /*
    fn step(&mut self, mem: &mut Memory) -> Error<(), String> {
        // check PC
        let b = ((mem.bytes[self.pc as usize] as u16) << 8) | (mem.bytes[(self.pc+1) as usize] as u16);

        self.step_instr(self, mem, instr);
    }
    */

    fn run_instr(&mut self, mem: &mut Memory, instr: instr::Instr) -> Result<(), String> {
        self.skip();

        use instr::Instr::*;
        match instr {
            // CLS
            RET => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            SYS(_xyz) => (),
            JP(xyz) => {
                self.pc = xyz;
            }
            CALL(xyz) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = xyz;
            }
            SE(x, yz) => {
                if self.v[x as usize] == yz {
                    self.skip();
                }
            }
            SNE(x, yz) => {
                if self.v[x as usize] != yz {
                    self.skip();
                }
            }
            SE_R(x, y) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.skip();
                }
            }
            SNE_R(x, y) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.skip();
                }
            }
            _ => return Err(format!("instruction not implemented: {:?}", instr)),
        }
        Ok(())
    }
}

fn main() {
    let mut mem = Memory::new();
    let mut reg = Registers::new();
    reg.dump();
    reg.run_instr(&mut mem, instr::Instr::SNE(0x0, 0x12)).unwrap();
    reg.dump();
}
