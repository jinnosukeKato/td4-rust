extern crate num_derive;

use crate::cpu::td4::TD4;

mod cpu;

fn main() {
    let mut td4 = TD4::new("./resources/test.bin");
    for i in 0..100 {
        let data = td4.fetch();
        let (opcode, operand) = TD4::decode(data);
        td4.execute(opcode, operand);
        println!("{:<2} {:>4b}", i, td4.port.output);
    }
}
