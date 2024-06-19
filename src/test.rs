#[cfg(test)]
pub mod tests {
    use crate::core::{CPU, Op};

    #[test]
    pub fn chip8_add() {
        let mut cpu = CPU::new();


        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        let mut mem =  cpu.get_memory();
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x14;
        cpu.run();

        assert_eq!(cpu.registers[0], 15);
    }
}