const MEMORY_SIZE: usize = 0x1000;
const MEMORY_PROGRAM_START: usize = 0x0200;

type Memory = [u8; MEMORY_SIZE];

/// Pretty print the memory content.
///
/// This function prints the content of the machine memory: sixteen
/// bytes in a row with address offset in the leftmost column.
fn memory_pprint(memory: Memory) -> String {
    let mut output = vec![];

    for (index, chunk) in memory.chunks(16).enumerate() {
        let mut row = vec![];
        for pair in chunk.chunks(2) {
            row.push(format!("{:02x}{:02x}", pair[0], pair[1]));
        }

        let offset = 16 * index;
        output.push(format!("0x{:04x}:  {}", offset, row.join(" ")));
    }

    output.join("\n")
}

struct Machine {
    memory: Memory,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            memory: [0; MEMORY_SIZE],
        }
    }

    fn from_program(program: Vec<u8>) -> Machine {
        let mut machine = Machine::new();
        machine.load_program(program);
        machine
    }

    fn load_program(&mut self, program: Vec<u8>) {
        for (offset, byte) in program.iter().enumerate() {
            self.memory[MEMORY_PROGRAM_START + offset] = *byte;
        }
    }

    fn pprint(&self) -> String {
        memory_pprint(self.memory)
    }
}

fn main() {
    let filename = "Maze (alt) [David Winter, 199x].ch8";
    let program: Vec<u8> = std::fs::read(filename).unwrap();

    let machine = Machine::from_program(program);
    println!("{}", machine.pprint());
}
