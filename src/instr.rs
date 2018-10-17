type Reg = u8;
type Addr = u16;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instr {
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

    ADD_I_R(Reg),

    RND(Reg, u8),
    DRW(Reg, Reg, u8),
    SKP(Reg),
    SKNP(Reg),
}

impl Instr {
    fn from(b: u16) -> Option<Instr> {
        use self::Instr::*;

        let a: u8 = ((b & 0xF000) >> 12) as u8;
        let x: u8 = ((b & 0x0F00) >> 8) as u8;
        let y: u8 = ((b & 0x00F0) >> 4) as u8;
        let z: u8 = (b & 0x000F) as u8;
        let yz: u8 = (b & 0x00FF) as u8;
        let xyz: u16 = b & 0x0FFF;

        match a {
            0x0 => match yz {
                0xE0 => Some(CLS),
                0xEE => Some(RET),
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
            0x7 => Some(ADD(x, yz)),
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
            _ => None,
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
        (0x7123, ADD(0x1, 0x23)),
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
            assert_eq!(Instr::from(*b), Some(*i), "{:04x} should parse", *b);
        }

        for b in ILLEGAL.iter() {
            assert_eq!(Instr::from(*b), None, "{:04x} should not parse", *b);
        }
    }
}
