use rand;

use super::instructions::*;
use super::pprint;

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

        println!("{}", pprint::instruction(self.pc, &instruction));

        match instruction {
            // Instruction::CLS => "CLS".to_string(),
            // Instruction::RET => "RET".to_string(),
            // Instruction::SYS(nnn) => format!("SYS 0x{:04x}", nnn),
            Instruction::JP(nnn) => { self._exec_jp(nnn); return },
            // Instruction::CALL(nnn) => format!("CALL 0x{:04x}", nnn),
            Instruction::SEVxByte(x, kk) => self._exec_se_vx_byte(x, kk),
            // Instruction::SNEVxByte(x, kk) => format!("SNE V{:x} 0x{:02x}", x, kk),
            // Instruction::SEVxVy(x, y) => format!("SE V{:x} V{:x}", x, y),
            Instruction::LDVxByte(x, kk) => self._exec_ld_vx_byte(x, kk),
            Instruction::ADDVxByte(x, kk) => self._exec_add_vx_byte(x, kk),
            // Instruction::LDVxVy(x, y) => format!("LD V{:x} V{:x}", x, y),
            // Instruction::ORVxVy(x, y) => format!("OR V{:x} V{:x}", x, y),
            // Instruction::ANDVxVy(x, y) => format!("AND V{:x} V{:x}", x, y),
            // Instruction::XORVxVy(x, y) => format!("XOR V{:x} V{:x}", x, y),
            // Instruction::ADDVxVy(x, y) => format!("ADD V{:x} V{:x}", x, y),
            // Instruction::SUBVxVy(x, y) => format!("SUB V{:x} V{:x}", x, y),
            // Instruction::SHRVxVy(x, y) => format!("SHR V{:x} V{:x}", x, y),
            // Instruction::SUBNVxVy(x, y) => format!("SUB V{:x} V{:x}", x, y),
            // Instruction::SHLVxVy(x, y) => format!("SHL V{:x} V{:x}", x, y),
            // Instruction::SNEVxVy(x, y) => format!("SNE V{:x} V{:x}", x, y),
            Instruction::LDI(nnn) => self._exec_ldi(nnn),
            // Instruction::JPV0(nnn) => format!("JP VO, 0x{:04x}", nnn),
            Instruction::RNDVxByte(x, kk) => self._exec_rnd_vx_byte(x, kk),
            Instruction::DRWVxVyNibble(x, y, n) => self._exec_drw_vx_vy_nibble(x, y, n),
            // Instruction::SKPVx(x) => format!("SKP V{:x}", x),
            // Instruction::SKNPVx(x) => format!("SKNP V{:x}", x),
            // Instruction::LDVxDT(x) => format!("LD V{:x}, DT", x),
            // Instruction::LDVxK(x) => format!("LD V{:x}, K", x),
            // Instruction::LDDTVx(x) => format!("LD DT, V{:x}", x),
            // Instruction::LDSTVx(x) => format!("LD ST, V{:x}", x),
            // Instruction::ADDIVx(x) => format!("ADD I, V{:x}", x),
            // Instruction::LDFVx(x) => format!("LD F, V{:x}", x),
            // Instruction::LDBVx(x) => format!("LD B, V{:x}", x),
            // Instruction::LDIVx(x) => format!("LD [I], V{:x}", x),
            // Instruction::LDVxI(x) => format!("LD V{:x}, [I]", x),
            _ => unimplemented!()
        }

        self.pc += 2;
    }

    /// The interpreter sets the program counter to nnn.
    fn _exec_jp(&mut self, nnn: usize) { self.pc = nnn }

    /// The interpreter compares register Vx to kk, and if they are
    /// equal, increments the program counter by 2.
    fn _exec_se_vx_byte(&mut self, x: usize, kk: u8) {
        if self.reg_v[x] == kk {
            self.pc += 2
        }
    }

    /// The interpreter puts the value kk into register Vx.
    fn _exec_ld_vx_byte(&mut self, x: usize, kk: u8) { self.reg_v[x] = kk }

    /// Adds the value kk to the value of register Vx, then stores the
    /// result in Vx.
    fn _exec_add_vx_byte(&mut self, x: usize, kk: u8) { self.reg_v[x] += kk }

    /// The value of register I is set to nnn.
    fn _exec_ldi(&mut self, nnn: u16) { self.reg_i = nnn }

    /// The interpreter generates a random number from 0 to 255, which
    /// is then ANDed with the value kk.
    fn _exec_rnd_vx_byte(&mut self, x: usize, kk: u8) {
        self.reg_v[x] = rand::random::<u8>() & kk
    }

    /// The interpreter reads n bytes from memory, starting at the
    /// address stored in I. These bytes are then displayed as sprites
    /// on screen at coordinates (Vx, Vy). Sprites are XORed onto the
    /// existing screen. If this causes any pixels to be erased, VF is
    /// set to 1, otherwise it is set to 0. If the sprite is
    /// positioned so part of it is outside the coordinates of the
    /// display, it wraps around to the opposite side of the
    /// screen.
    fn _exec_drw_vx_vy_nibble(&mut self, x: usize, y: usize, n: u8) {
        let start = self.reg_i as usize;
        let end = start + n as usize;
        let sprite = self.memory[start .. end].to_vec();

        println!("{:?}", sprite);
    }
}
