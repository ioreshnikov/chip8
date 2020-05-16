pub const MEMORY_SIZE: usize = 0x1000;
pub const MEMORY_PROGRAM_START: usize = 0x0200;

pub type Memory = [u8; MEMORY_SIZE];
pub type VRegisters = [u8; 16];
pub type IRegister = u16;
pub type Stack = [u16; 16];

pub struct Machine {
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
}
