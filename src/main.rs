mod pprint;
mod machine;

fn main() {
    let filename = "Maze (alt) [David Winter, 199x].ch8";
    let program: Vec<u8> = std::fs::read(filename).unwrap();

    // let mut machine = Machine::from_program(program);
    let mut m = machine::Machine::new();
    println!("{}", pprint::machine(m));
}
