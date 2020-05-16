mod machine;
mod instructions;
mod pprint;

use std::{thread, time};

fn main() {
    let filename = "Maze (alt) [David Winter, 199x].ch8";
    let program: Vec<u8> = std::fs::read(filename).unwrap();

    let mut m = machine::Machine::from_program(program);
    loop {
        m.tick();
        println!("{}", pprint::machine(&m));
        thread::sleep(time::Duration::from_millis(500));
    }
}
