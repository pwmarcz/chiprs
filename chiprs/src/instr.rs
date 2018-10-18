type Reg = u8;
type Addr = u16;

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Instr {
    CLS,
    RET,
    SYS(Addr),

    JP(Addr),
    CALL(Addr),
    JP_V0(Addr),

    SE(Reg, u8),
    SE_R(Reg, Reg),
    SNE(Reg, u8),
    SNE_R(Reg, Reg),

    OR(Reg, Reg),
    AND(Reg, Reg),
    XOR(Reg, Reg),
    ADD(Reg, Reg),
    SUB(Reg, Reg),
    SUBN(Reg, Reg),

    SHR(Reg, Reg),
    SHL(Reg, Reg),

    LD_R_B(Reg, u8),
    LD_R_R(Reg, Reg),
    LD_I_A(Addr),
    LD_R_DT(Reg),
    LD_R_K(Reg),
    LD_DT_R(Reg),
    LD_ST_R(Reg),
    LD_F_R(Reg),
    LD_B_R(Reg),
    LD_II_R(Reg),
    LD_R_II(Reg),

    ADD_R_B(Reg, u8),
    ADD_I_R(Reg),

    RND(Reg, u8),
    DRW(Reg, Reg, u8),
    SKP(Reg),
    SKNP(Reg),
}

impl Instr {
    pub fn from(b: u16) -> Option<Instr> {
        use self::Instr::*;

        let a: u8 = ((b & 0xF000) >> 12) as u8;
        let x: u8 = ((b & 0x0F00) >> 8) as u8;
        let y: u8 = ((b & 0x00F0) >> 4) as u8;
        let z: u8 = (b & 0x000F) as u8;
        let yz: u8 = (b & 0x00FF) as u8;
        let xyz: u16 = b & 0x0FFF;

        match a {
            0x0 => match xyz {
                0x0E0 => Some(CLS),
                0x0EE => Some(RET),
                _ => Some(SYS(xyz)),
            },
            0x1 => Some(JP(xyz)),
            0x2 => Some(CALL(xyz)),
            0x3 => Some(SE(x, yz)),
            0x4 => Some(SNE(x, yz)),
            0x5 => match z {
                0 =>  Some(SE_R(x, y)),
                _ => None,
            },
            0x6 => Some(LD_R_B(x, yz)),
            0x7 => Some(ADD_R_B(x, yz)),
            0x8 => match z {
                0 => Some(LD_R_R(x, y)),
                1 => Some(OR(x, y)),
                2 => Some(AND(x, y)),
                3 => Some(XOR(x, y)),
                4 => Some(ADD(x, y)),
                5 => Some(SUB(x, y)),
                6 => Some(SHR(x, y)),
                7 => Some(SUBN(x, y)),
                0xE => Some(SHL(x, y)),
                _ => None,
            },
            0x9 => match z {
                0 => Some(SNE_R(x, y)),
                _ => None,
            },
            0xA => Some(LD_I_A(xyz)),
            0xB => Some(JP_V0(xyz)),
            0xC => Some(RND(x, yz)),
            0xD => Some(DRW(x, y, z)),
            0xE => match yz {
                0x9E => Some(SKP(x)),
                0xA1 => Some(SKNP(x)),
                _ => None,
            },
            0xF => match yz {
                0x07 => Some(LD_R_DT(x)),
                0x0A => Some(LD_R_K(x)),
                0x15 => Some(LD_DT_R(x)),
                0x18 => Some(LD_ST_R(x)),
                0x1E => Some(ADD_I_R(x)),
                0x29 => Some(LD_F_R(x)),
                0x33 => Some(LD_B_R(x)),
                0x55 => Some(LD_II_R(x)),
                0x65 => Some(LD_R_II(x)),
                _ => None,
            },
            _ => panic!("impossible")
        }
    }

    pub fn to(self) -> u16 {
        use self::Instr::*;

        fn _x(x: u8) -> u16 { (x as u16) << 8 }
        fn _xy(x: u8, y: u8) -> u16 { ((x as u16) << 8) | ((y as u16) << 4)}
        fn _yz(yz: u8) -> u16 { yz as u16 }
        fn _xyz(x: u8, y: u8, z: u8) -> u16 { ((x as u16) << 8) | ((y as u16) << 4) | (z as u16) }

        match self {
            CLS => 0x00E0,
            RET => 0x00EE,
            SYS(xyz) => 0x0000 | xyz,
            JP(xyz) => 0x1000 | xyz,
            CALL(xyz) => 0x2000 | xyz,
            JP_V0(xyz) => 0xB000 | xyz,
            SE(x, yz) => 0x3000 | _x(x) | _yz(yz),
            SNE(x, yz) => 0x4000 | _x(x) | _yz(yz),
            SE_R(x, y) => 0x5000 | _xy(x, y),
            SNE_R(x, y) => 0x9000 | _xy(x, y),

            OR(x, y) => 0x8001 | _xy(x, y),
            AND(x, y) => 0x8002 | _xy(x, y),
            XOR(x, y) => 0x8003 | _xy(x, y),
            ADD(x, y) => 0x8004 | _xy(x, y),
            SUB(x, y) => 0x8005 | _xy(x, y),
            SUBN(x, y) => 0x8007 | _xy(x, y),

            SHR(x, y) => 0x8006 | _xy(x, y),
            SHL(x, y) => 0x800E | _xy(x, y),

            LD_R_B(x, yz) => 0x6000 | _x(x) | _yz(yz),
            LD_R_R(x, y) => 0x8000 | _xy(x, y),
            LD_I_A(xyz) => 0xA000 | xyz,
            LD_R_DT(x) => 0xF007 | _x(x),
            LD_R_K(x) => 0xF00A | _x(x),
            LD_DT_R(x) => 0xF015 | _x(x),
            LD_ST_R(x) => 0xF018 | _x(x),
            LD_F_R(x) => 0xF029 | _x(x),
            LD_B_R(x) => 0xF033 | _x(x),
            LD_II_R(x) => 0xF055 | _x(x),
            LD_R_II(x) => 0xF065 | _x(x),

            ADD_R_B(x, yz) => 0x7000 | _x(x) | _yz(yz),
            ADD_I_R(x) => 0xF01E | _x(x),

            RND(x, yz) => 0xC000 | _x(x) | _yz(yz),
            DRW(x, y, z) => 0xD000 | _xyz(x, y, z),
            SKP(x) => 0xE09E | _x(x),
            SKNP(x) => 0xE0A1 | _x(x),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Instr::*;

    static LEGAL: &'static [(u16, Instr)] = &[
        (0x00E0, CLS),
        (0x00EE, RET),
        (0x0123, SYS(0x123)),
        (0x1123, JP(0x123)),
        (0x2123, CALL(0x123)),
        (0x3123, SE(0x1, 0x23)),
        (0x4123, SNE(0x1, 0x23)),
        (0x5120, SE_R(0x1, 0x2)),
        (0x6123, LD_R_B(0x1, 0x23)),
        (0x7123, ADD_R_B(0x1, 0x23)),
        (0x8120, LD_R_R(0x1, 0x2)),
        (0x8121, OR(0x1, 0x2)),
        (0x8122, AND(0x1, 0x2)),
        (0x8123, XOR(0x1, 0x2)),
        (0x8124, ADD(0x1, 0x2)),
        (0x8125, SUB(0x1, 0x2)),
        (0x8126, SHR(0x1, 0x2)),
        (0x8127, SUBN(0x1, 0x2)),
        (0x812E, SHL(0x1, 0x2)),
        (0x9120, SNE_R(0x1, 0x2)),
        (0xA123, LD_I_A(0x123)),
        (0xB123, JP_V0(0x123)),
        (0xC123, RND(0x1, 0x23)),
        (0xD123, DRW(0x1, 0x2, 0x3)),
        (0xE19E, SKP(0x1)),
        (0xE1A1, SKNP(0x1)),
        (0xF107, LD_R_DT(0x1)),
        (0xF10A, LD_R_K(0x1)),
        (0xF115, LD_DT_R(0x1)),
        (0xF118, LD_ST_R(0x1)),
        (0xF11E, ADD_I_R(0x1)),
        (0xF129, LD_F_R(0x1)),
        (0xF133, LD_B_R(0x1)),
        (0xF155, LD_II_R(0x1)),
        (0xF165, LD_R_II(0x1)),
    ];

    static ILLEGAL: &'static [u16] = &[
        0x5121,
        0x812A,
        0x9121,
        0xE100,
        0xF100,
    ];

    #[test]
    fn test_from() {
        for (b, i) in LEGAL.iter() {
            assert_eq!(Instr::from(*b), Some(*i), "{:04X} should parse to {:?}", *b, *i);
        }

        for b in ILLEGAL.iter() {
            assert_eq!(Instr::from(*b), None, "{:04X} should not parse", *b);
        }
    }

    #[test]
    fn test_to() {
        for (b, i) in LEGAL.iter() {
            assert_eq!(i.to(), *b, "{:?} should generate {:04X}", *i, *b);
        }
    }

    #[test]
    fn test_all() {
        for b in 0x0000..=0xFFFF {
            if let Some(i) = Instr::from(b) {
                assert_eq!(i.to(), b, "{:?} should generate back {:04X}", i, b)
            }
        }
    }
}
