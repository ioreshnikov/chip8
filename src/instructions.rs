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
    ADDIVx(usize),
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
            (0x0, 0x0, 0xE, 0x0) => Instruction::CLS,
            // 00EE - RET
            (0x0, 0x0, 0xE, 0xE) => Instruction::RET,
            // 0nnn - SYS addr
            (0x0,   _,   _,   _) => Instruction::SYS(nnn as usize),
            // 1nnn - JP addr
            (0x1,   _,   _,   _) => Instruction::JP(nnn as usize),
            // 2nnn - CALL addr
            (0x2,   _,   _,   _) => Instruction::CALL(nnn as usize),
            // 3xkk - SE Vx, byte
            (0x3,   _,   _,   _) => Instruction::SEVxByte(x, kk),
            // 4xkk - SNE Vx, byte
            (0x4,   _,   _,   _) => Instruction::SNEVxByte(x, kk),
            // 5xy0 - SE Vx, Vy
            (0x5,   _,   _, 0x0) => Instruction::SEVxVy(x, y),
            // 6xkk - LD Vx, byte
            (0x6,   _,   _,   _) => Instruction::LDVxByte(x, kk),
            // 7xkk - ADD Vx, byte
            (0x7,   _,   _,   _) => Instruction::ADDVxByte(x, kk),
            // 8xy0 - LD Vx, Vy
            (0x8,   _,   _, 0x0) => Instruction::LDVxVy(x, y),
            // 8xy1 - OR Vx, Vy
            (0x8,   _,   _, 0x1) => Instruction::ORVxVy(x, y),
            // 8xy2 - AND Vx, Vy
            (0x8,   _,   _, 0x2) => Instruction::ANDVxVy(x, y),
            // 8xy3 - XOR Vx, Vy
            (0x8,   _,   _, 0x3) => Instruction::XORVxVy(x, y),
            // 8xy4 - ADD Vx, Vy
            (0x8,   _,   _, 0x4) => Instruction::ADDVxVy(x, y),
            // 8xy5 - SUB Vx, Vy
            (0x8,   _,   _, 0x5) => Instruction::SUBVxVy(x, y),
            // 8xy6 - SHR Vx {, Vy}
            (0x8,   _,   _, 0x6) => Instruction::SHRVxVy(x, y),
            // 8xy7 - SUBN Vx, Vy
            (0x8,   _,   _, 0x7) => Instruction::SUBNVxVy(x, y),
            // 8xyE - SHL Vx {, Vy}
            (0x8,   _,   _, 0xE) => Instruction::SHLVxVy(x, y),
            // 9xy0 - SNE Vx, Vy
            (0x9,   _,   _, 0x9) => Instruction::SNEVxVy(x, y),
            // Annn - LD I, addr
            (0xA,   _,   _,   _) => Instruction::LDI(nnn),
            // Bnnn - JP V0, addr
            (0xB,   _,   _,   _) => Instruction::JPV0(nnn as usize),
            // Cxkk - RND Vx, byte
            (0xC,   _,   _,   _) => Instruction::RNDVxByte(x, kk),
            // Dxyn - DRW Vx, Vy, nibble
            (0xD,   _,   _,   _) => Instruction::DRWVxVyNibble(x, y, n),
            // Ex9E - SKP Vx
            (0xE,   _, 0x9, 0xE) => Instruction::SKPVx(x),
            // ExA1 - SKNP Vx
            (0xE,   _, 0xA, 0x1) => Instruction::SKNPVx(x),
            // Fx07 - LD Vx, DT
            (0xF,   _, 0x0, 0x7) => Instruction::LDVxDT(x),
            // Fx0A - LD Vx, K
            (0xF,   _, 0x0, 0xA) => Instruction::LDVxK(x),
            // Fx15 - LD DT, Vx
            (0xF,   _, 0x1, 0x5) => Instruction::LDDTVx(x),
            // Fx18 - LD ST, Vx
            (0xF,   _, 0x1, 0x8) => Instruction::LDSTVx(x),
            // Fx1E - ADD I, Vx
            (0xF,   _, 0x1, 0xE) => Instruction::ADDIVx(x),
            // Fx29 - LD F, Vx
            (0xF,   _, 0x2, 0x9) => Instruction::LDFVx(x),
            // Fx33 - LD B, Vx
            (0xF,   _, 0x3, 0x3) => Instruction::LDBVx(x),
            // Fx55 - LD [I], Vx
            (0xF,   _, 0x5, 0x5) => Instruction::LDIVx(x),
            // Fx65 - LD Vx, [I]
            (0xF,   _, 0x6, 0x5) => Instruction::LDVxI(x),
            _ => unimplemented!()
        }
    }
}
