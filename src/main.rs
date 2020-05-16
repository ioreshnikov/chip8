mod machine;
mod instructions;
mod pprint;

fn main() {
    let filename = "Clock Program [Bill Fisher, 1981].ch8";
    let program: Vec<u8> = std::fs::read(filename).unwrap();

    let mut m = machine::Machine::from_program(program);
    loop {
        println!("{}", pprint::machine(&m));
        m.tick();
    }
}
