use ansi_term::Style;
use super::machine::*;

/// ANSI style for dimmed text
fn dimmed() -> Style {
    Style::new().dimmed()
}

/// ANSI style for reverse video text
fn revvid() -> Style {
    Style::new().reverse()
}

/// Indent a string to a level.
fn indent(string: String, level: usize) -> String {
    let space = String::from(" ").repeat(level);
    space + &string
}

/// Indented header for printing the memory.
fn memheader(indent_level: usize) -> String {
    let header = String::from("Offset:  0011 2233 4455 6677 8899 aabb ccdd eeff\n");
    indent(dimmed().paint(header).to_string(), indent_level)
}

/// Pretty print a memory block.
///
/// This function prints a single memory block: sixteen bytes in a
/// row with address offset in the leftmost column.
pub fn memblock(
    memory: Memory,
    start: usize,
    end: usize,
    counter: usize,
    indent_level: usize,
) -> String {
    let mut rows = vec![];

    for (row_index, chunk) in memory[start..end].chunks(16).enumerate() {
        let offset = 16 * row_index + start;
        let mut row = vec![];

        for (pair_index, pair) in chunk.chunks(2).enumerate() {
            let mut fststr = format!("{:02x}", pair[0]);
            let mut sndstr = format!("{:02x}", pair[1]);

            if counter == offset + 2 * pair_index {
                fststr = revvid().paint(&fststr).to_string();
            }
            else if counter == offset + 2 * pair_index + 1 {
                sndstr = revvid().paint(&sndstr).to_string();
            }

            row.push(fststr + &sndstr);
        }

        let offset = dimmed().paint(format!("0x{:04x}:  ", offset)).to_string();
        let rowstring = offset + &row.join(" ");
        rows.push(indent(rowstring, indent_level));
    }

    rows.join("\n")
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

    fn round_to_next_row(pos: usize) -> usize {
        ((pos / 16) + 1) * 16
    };

    let start = MEMORY_PROGRAM_START;
    let end = match memory.iter().rposition(|&byte| byte != 0x00) {
        // Round the position up to the entire row (16 bytes)
        Some(pos) => round_to_next_row(pos),
        None => round_to_next_row(counter),
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

    dimmed().paint(header).to_string() +
        &indent(registers.join(" "), indent_level + 6) + "\n"
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
    for (index, byte) in stack.iter().enumerate() {
        let mut bytestr = format!("{:02x}", byte);
        if index == pointer {
            bytestr = revvid().paint(bytestr).to_string();
        }
        values.push(bytestr);
    }

    dimmed().paint(header).to_string() +
        &indent(values.join(" "), indent_level + 7) + "\n"
}

/// Pretty print the entire machine.
///
/// This function prints the state of the machine
pub fn machine(machine: Machine) -> String {
    let registers = regs(machine.reg_v, machine.reg_i, 4);
    let stack = stack(machine.stack, machine.sp, 4);
    let memory = memnontriv(machine.memory, machine.pc, 4);

    let bold = Style::new().bold();

    bold.paint("REGISTERS\n").to_string() + &registers + "\n" +
        &bold.paint("STACK\n").to_string() + &stack + "\n" +
        &bold.paint("MEMORY\n").to_string() + &memory
}
