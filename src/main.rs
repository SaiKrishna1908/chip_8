use crate::core::{CPU, Op};

mod core;

fn main() {
    let mut cpu = CPU::new();

    cpu.set_current_operation(Op::ADD);
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);
}
