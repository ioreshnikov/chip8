use rand;
use super::instructions::*;

pub const MEMORY_SIZE: usize = 0x1000;
pub const MEMORY_PROGRAM_START: usize = 0x0200;

pub type Memory = [u8; MEMORY_SIZE];
pub type VRegisters = [u8; 16];
pub type IRegister = u16;
pub type Stack = [u16; 16];

/// The CHIP8 virtual machine.
pub struct Machine {
    /// 16 8-bit registers
    pub reg_v: VRegisters,
    pub reg_i: IRegister,

    pub sp: usize,
    pub stack: Stack,

    pub pc: usize,
    pub memory: Memory,
}

impl Machine {
    /// Construct an empty machine.
    pub fn new() -> Machine {
        Machine {
            reg_v: [0; 16],
            reg_i: 0,

            sp: 0,
            stack: [0; 16],

            pc: MEMORY_PROGRAM_START,
            memory: [0; MEMORY_SIZE],
        }
    }

    /// Construct a machine with a program loaded in memory.
    pub fn from_program(program: Vec<u8>) -> Machine {
        let mut machine = Machine::new();
        machine.load_program(program);
        machine
    }

    /// Load a program in memory.
    pub fn load_program(&mut self, program: Vec<u8>) {
        for (offset, byte) in program.iter().enumerate() {
            self.memory[MEMORY_PROGRAM_START + offset] = *byte;
        }
    }

    /// Execute the next program instruction.
    pub fn tick(&mut self) {
        let left = self.memory[self.pc];
        let right = self.memory[self.pc + 1];
        let instruction = Instruction::read(left, right);

        println!("{:?}", instruction);

        match instruction {
            // 00E0 - CLS
            Instruction::CLS => self._cls(),
            // 00EE - RET
            // 0nnn - SYS addr
            // 1nnn - JP addr
            Instruction::JP(nnn) => {self._jp(nnn); return},
            // 2nnn - CALL addr
            // 3xkk - SE Vx, byte
            Instruction::SEVxByte(x, kk) => self._sevxbyte(x, kk),
            // 4xkk - SNE Vx, byte
            // 5xy0 - SE Vx, Vy
            // 6xkk - LD Vx, byte
            Instruction::LDVxByte(x, kk) => self._ldvxbyte(x, kk),
            // 7xkk - ADD Vx, byte
            Instruction::ADDVxByte(x, kk) => self._addvxbyte(x, kk),
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
            Instruction::LDI(nnn) => self._ldi(nnn),
            // Bnnn - JP V0, addr
            // Cxkk - RND Vx, byte
            Instruction::RNDVxByte(x, kk) => self._rngvxbyte(x, kk),
            // Dxyn - DRW Vx, Vy, nibble
            Instruction::DRWVxVyNibble(x, y, n) => self._drwvxvynibble(x, y, n),
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
        self.pc += 2;
    }

    /// Clear the display.
    fn _cls(&mut self) {
    }

    /// Jump to location nnn.
    fn _jp(&mut self, nnn: usize) {
        self.pc = nnn;
    }

    fn _sevxbyte(&mut self, x: usize, kk: u8) {
        if self.reg_v[x] == kk {
            self.pc += 2;
        }
    }

    fn _ldvxbyte(&mut self, x: usize, kk: u8) {
        self.reg_v[x] = kk;
    }

    fn _addvxbyte(&mut self, x: usize, kk: u8) {
        self.reg_v[x] += kk;
    }

    fn _ldi(&mut self, nnn: u16) {
        self.reg_i = nnn;
    }

    fn _rngvxbyte(&mut self, x: usize, kk: u8) {
        self.reg_v[x] = rand::random::<u8>() & kk;
    }

    fn _drwvxvynibble(&mut self, x: usize, y: usize, n: u8) {
        // The interpreter reads n bytes from memory, starting at the
        // address stored in I. These bytes are then displayed as
        // sprites on screen at coordinates (Vx, Vy). Sprites are
        // XORed onto the existing screen. If this causes any pixels
        // to be erased, VF is set to 1, otherwise it is set to 0. If
        // the sprite is positioned so part of it is outside the
        // coordinates of the display, it wraps around to the opposite
        // side of the screen. See instruction 8xy3 for more
        // information on XOR, and section 2.4, Display, for more
        // information on the Chip-8 screen and sprites.
    }
}
