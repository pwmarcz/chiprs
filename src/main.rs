mod instr;
use instr::Instr;

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

    fn load_program(&mut self, program: &[Instr]) {
        for (i, instr) in program.iter().enumerate() {
            let b: u16 = instr.to();
            self.bytes[i * 2] = (b >> 8) as u8;
            self.bytes[i * 2 + 1] = (b & 0xFF) as u8;
        }
    }

    fn u16_at(&self, addr: u16) -> u16 {
        return (((self.bytes[addr as usize] as u16) << 8) |
                (self.bytes[(addr + 1) as usize] as u16));
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

    fn dump(&self, mem: &Memory) {
        for i in 0..V_SIZE {
            print!("V{:X} ", i);
        }
        println!("   I    PC  SP  instr");
        for i in 0..V_SIZE {
            print!("{:02X} ", self.v[i]);
        }
        print!("  {:02x}  {:04x}  {:02x}", self.i, self.pc, self.sp);
        if self.pc as usize <= MEMORY_SIZE - 2 {
            let b = mem.u16_at(self.pc);
            match Instr::from(b) {
                Some(instr) => print!("  {:?}", instr),
                None => print!("  ????"),
            }
        }
        println!();
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

    fn step(&mut self, mem: &mut Memory) -> Result<(), String> {
        // check PC

        let b = mem.u16_at(self.pc);
        match Instr::from(b) {
            Some(instr) => self.run_instr(mem, instr),
            None => Err(format!("could not parse {:04X} as instruction", b)),
        }
    }

    fn run_instr(&mut self, mem: &mut Memory, instr: Instr) -> Result<(), String> {
        self.skip();

        use Instr::*;
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
            LD_R_R(x, y) => {
                self.v[x as usize] = self.v[y as usize];
            }
            LD_R_B(x, yz) => {
                self.v[x as usize] = yz;
            }
            ADD(x, y) => {
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                let (result, bit) = vx.overflowing_add(vy);
                self.v[x as usize] = result;
                self.v[0xF] = bit as u8;
            }
            SUB(x, y) => {
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                let (result, bit) = vx.overflowing_sub(vy);
                self.v[x as usize] = result;
                self.v[0xF] = (!bit) as u8;
            }
            _ => return Err(format!("instruction not implemented: {:?}", instr)),
        }
        Ok(())
    }
}

fn main() {
    let mut mem = Memory::new();
    let mut reg = Registers::new();

    use Instr::*;
    // Calc Fibonacci
    mem.load_program(&[
        LD_R_B(0, 7),
        LD_R_B(1, 1),
        LD_R_B(2, 0),

        // loop:
        // finish if V0 == 0
        SNE(0, 0),
        JP(0xFFF),

        // V1, V2 = V1 + V2, V1
        LD_R_R(3, 1),
        ADD(1, 2),
        LD_R_R(2, 3),

        // decrement V0
        LD_R_B(3, 1),
        SUB(0, 3),

        // jump to loop
        JP(6),
    ]);

    while reg.pc != 0xFFF {
        reg.dump(&mem);
        reg.step(&mut mem).unwrap();
    }
}
