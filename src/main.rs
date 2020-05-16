const MEMORY_SIZE: usize = 0x1000;
const MEMORY_PROGRAM_START: usize = 0x0200;

type Memory = [u8; MEMORY_SIZE];
type VRegisters = [u8; 16];
type IRegister = u16;
type Stack = [u16; 16];

mod pprint {
    use crate::*;

    /// Indent a string to a level.
    fn indent(string: String, level: usize) -> String {
        let space = String::from(" ").repeat(level);
        space + &string
    }

    /// Indented header for printing the memory.
    fn memheader(indent_level: usize) -> String {
        indent(
            String::from("Offset:  0011 2233 4455 6677 8899 aabb ccdd eeff\n"),
            indent_level,
        )
    }

    /// Pretty print a memory block.
    ///
    /// This function prints a single memory block: sixteen bytes in a
    /// row with address offset in the leftmost column.
    pub fn memblock(memory: Memory, start: usize, end: usize, counter: usize, indent_level: usize) -> String {
        let mut rows = vec![];

        for (index, chunk) in memory[start..end].chunks(16).enumerate() {
            let offset = 16 * index + start;
            let mut row = vec![];

            for pair in chunk.chunks(2) {
                row.push(format!("{:02x}{:02x}", pair[0], pair[1]));
            }

            let rowstring = format!("0x{:04x}:  {}", offset, row.join(" "));
            rows.push(indent(rowstring, indent_level));
        }

        rows.join("\n")
    }

    /// Pretty print the entire memory content.
    ///
    /// This function prints the content of the machine
    /// memory. Formatting is done by `memblock`: sixteen bytes in a
    /// row with address offset in the leftmost column.
    pub fn memory(memory: Memory, counter: usize, indent_level: usize) -> String {
        let header = memheader(indent_level);
        let static_memory = memblock(memory, 0, MEMORY_PROGRAM_START, counter, indent_level);
        let program_memory = memblock(memory, MEMORY_PROGRAM_START, MEMORY_SIZE, counter, indent_level);

        header + "\n" + &static_memory + "\n" + &program_memory
    }

    /// Pretty print the nontrivial memory content.
    ///
    /// This function prints the nontrivial part of the machine
    /// memory. The first 0x200 bytes are skipped and then only a
    /// nonzero block is selected. Formatting is done by `memblock`:
    /// sixteen bytes in a row with address offset in the leftmost
    /// column.
    pub fn memnontriv(memory: Memory, counter: usize, indent_level: usize) -> String {
        let header = memheader(indent_level);

        let start = MEMORY_PROGRAM_START;
        let end = match memory.iter().rposition(|&byte| byte != 0x00) {
            // Round the position up to the entire row (16 bytes)
            Some(pos) => (((pos as f32) / 16.0).ceil() * 16.0) as usize,
            None => MEMORY_SIZE,
        };

        header + "\n" + &memblock(memory, start, end, counter, indent_level)
    }

    /// Pretty print the registers.
    ///
    /// This function prints the content of the machine registers.
    pub fn regs(reg_v: VRegisters, reg_i: IRegister, indent_level: usize) -> String {
        let header = indent(
            String::from("Name: 0 1 2 3 4 5 6 7 8 9 A B C D E F  I\n"),
            indent_level,
        );

        let mut registers = vec![];
        for reg in reg_v.iter() {
            registers.push(format!("{:x}", reg));
        }
        registers.push(format!("{:02x}", reg_i));

        header + &indent(registers.join(" "), indent_level + 6) + "\n"
    }

    /// Pretty print the stack.
    ///
    /// This function print the stack content.
    pub fn stack(stack: Stack, pointer: usize, indent_level: usize) -> String {
        let header = indent(
            String::from("Index:  0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F\n"),
            indent_level,
        );

        let mut values = vec![];
        for byte in stack.iter() {
            values.push(format!("{:02x}", byte));
        }

        header + &indent(values.join(" "), indent_level + 7) + "\n"
    }
}

enum Instruction {
    LDVxByte(usize, u8),
}

struct Machine {
    reg_v: VRegisters,
    reg_i: IRegister,

    sp: usize,
    stack: Stack,

    pc: usize,
    memory: Memory,
}

impl Machine {
    /// Construct an empty machine.
    fn new() -> Machine {
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
    fn from_program(program: Vec<u8>) -> Machine {
        let mut machine = Machine::new();
        machine.load_program(program);
        machine
    }

    /// Load a program in memory.
    fn load_program(&mut self, program: Vec<u8>) {
        for (offset, byte) in program.iter().enumerate() {
            self.memory[MEMORY_PROGRAM_START + offset] = *byte;
        }
    }

    /// Pretty print the current state of the machine.
    fn pprint(&self) -> String {
        let registers = pprint::regs(self.reg_v, self.reg_i, 4);
        let stack = pprint::stack(self.stack, self.sp, 4);
        let memory = pprint::memnontriv(self.memory, self.pc, 4);

        String::from("REGISTERS\n") + &registers + "\nSTACK\n" + &stack + "\nMEMORY\n" + &memory
    }
}

fn main() {
    let filename = "Maze (alt) [David Winter, 199x].ch8";
    let program: Vec<u8> = std::fs::read(filename).unwrap();

    let machine = Machine::from_program(program);
    println!("{}", machine.pprint());
}
