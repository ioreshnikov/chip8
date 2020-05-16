#[derive(Debug)]
pub enum Instruction {
    CLS,
    RET,
    SYS(usize),
    JP(usize),
    CALL(usize),
    SEVxByte(usize, u8),
    SNEVxByte(usize, u8),
    SEVxVy(usize, usize),
    LDVxByte(usize, u8),
    ADDVxByte(usize, u8),
    LDVxVy(usize, usize),
    ORVxVy(usize, usize),
    ANDVxVy(usize, usize),
    XORVxVy(usize, usize),
    ADDVxVy(usize, usize),
    SUBVxVy(usize, usize),
    SHRVxVy(usize, usize),
    SUBNVxVy(usize, usize),
    SHLVxVy(usize, usize),
    SNEVxVy(usize, usize),
    LDI(u16),
    JPV0(usize),
    RNDVxByte(usize, u8),
    DRWVxVyNibble(usize, usize, u8),
    SKPVx(usize),
    SKNPVx(usize),
    LDVxDT(usize),
    LDVxK(usize),
    LDDTVx(usize),
    LDSTVx(usize),
    ADDIVx(u8),
    LDFVx(usize),
    LDBVx(usize),
    LDIVx(usize),
    LDVxI(usize),
}

impl Instruction {
    pub fn read(left: u8, right: u8) -> Instruction {
        // Split into nibbles
        let a = (left & 0xF0) >> 4;
        let b = left & 0x0F;
        let c = (right & 0xF0) >> 4;
        let d = right & 0x0F;

        let x = b as usize;
        let y = c as usize;
        let n = d;
        let kk = right;
        let nnn = (b as u16) * 0x100 + (right as u16);

        println!("{:x} {:x} {:x} {:x}", a, b, c, d);
        match (a, b, c, d) {
            // 00E0 - CLS
            // 00EE - RET
            // 0nnn - SYS addr
            // 1nnn - JP addr
            (0x1, _, _, _) => Instruction::JP(nnn as usize),
            // 2nnn - CALL addr
            // 3xkk - SE Vx, byte
            (0x3, _, _, _) => Instruction::SEVxByte(x, kk),
            // 4xkk - SNE Vx, byte
            // 5xy0 - SE Vx, Vy
            // 6xkk - LD Vx, byte
            (0x6, _, _, _) => Instruction::LDVxByte(x, kk),
            // 7xkk - ADD Vx, byte
            (0x7, _, _, _) => Instruction::ADDVxByte(x, kk),
            // 8xy0 - LD Vx, Vy
            // 8xy1 - OR Vx, Vy
            // 8xy2 - AND Vx, Vy
            // 8xy3 - XOR Vx, Vy
            // 8xy4 - ADD Vx, Vy
            // 8xy5 - SUB Vx, Vy
            // 8xy6 - SHR Vx {, Vy}
            // 8xy7 - SUBN Vx, Vy
            // 8xyE - SHL Vx {, Vy}
            // 9xy0 - SNE Vx, Vy
            // Annn - LD I, addr
            (0xA, _, _, _) => Instruction::LDI(nnn),
            // Bnnn - JP V0, addr
            // Cxkk - RND Vx, byte
            (0xC, _, _, _) => Instruction::RNDVxByte(x, kk),
            // Dxyn - DRW Vx, Vy, nibble
            (0xD, _, _, _) => Instruction::DRWVxVyNibble(x, y, n),
            // Ex9E - SKP Vx
            // ExA1 - SKNP Vx
            // Fx07 - LD Vx, DT
            // Fx0A - LD Vx, K
            // Fx15 - LD DT, Vx
            // Fx18 - LD ST, Vx
            // Fx1E - ADD I, Vx
            // Fx29 - LD F, Vx
            // Fx33 - LD B, Vx
            // Fx55 - LD [I], Vx
            // Fx65 - LD Vx, [I]
            _ => unimplemented!()
        }
    }
}
