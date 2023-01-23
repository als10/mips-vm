use std::env;

pub mod vm;
pub mod instruction;
pub mod filereader;
pub mod inputreader;
pub mod bitparser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Filename not provided.");
    }

    let program = filereader::read_file(&args[1]);

    let mut vm = vm::VM::new(program);
    vm.run();
    vm.print_registers();
}
