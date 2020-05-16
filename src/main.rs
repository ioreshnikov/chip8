mod pprint;
mod machine;

fn main() {
    let filename = "Maze (alt) [David Winter, 199x].ch8";
    let program: Vec<u8> = std::fs::read(filename).unwrap();

    let mut m = machine::Machine::from_program(program);
    println!("{}", pprint::machine(m));
}
