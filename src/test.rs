#[cfg(test)]
pub mod tests {
    use std::cmp::{max, min};
    use crate::core::{CPU, Op};

    #[test]
    pub fn chip8_add_with_register0_and_register1() {
        let mut cpu = CPU::new();


        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x14;
        cpu.run();

        assert_eq!(cpu.registers[0], 15);
    }

    #[test]
    pub fn chip8_add_with_register0_and_register4() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 2;
        cpu.registers[4] = 3;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x44;

        cpu.run();

        assert_eq!(cpu.registers[0], 5);
    }

    #[test]
    pub fn chip8_add_with_overflow_flag() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 120;
        cpu.registers[1] = 250;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x14;

        cpu.run();

        assert_eq!(cpu.registers[15], 1);
    }


    #[test]
    pub fn chip8_or() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 0b10;
        cpu.registers[1] = 0b01;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x11;

        cpu.run();

        assert_eq!(cpu.registers[0], 0b11);
    }

    #[test]
    pub fn chip8_and() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 0b1110;
        cpu.registers[1] = 0b0110;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x12;

        cpu.run();

        assert_eq!(cpu.registers[0], 0b0110)
    }

    #[test]
    pub fn chip8_sub() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 20;
        cpu.registers[1] = 10;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x15;

        cpu.run();

        assert_eq!(cpu.registers[0], 10);
    }

    #[test]
    pub fn chip8_sub_with_negative() {
        let mut cpu = CPU::new();

        let a = 10u8;
        let b = 15u8;

        cpu.registers[0] = 10u8;
        cpu.registers[1] = 15u8;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x15;

        cpu.run();

        assert!(cpu.registers[0] < cpu.registers[1]);
        assert_eq!(cpu.registers[0], max(a,b) - min(a,b));
        assert_eq!(cpu.registers[15], 1);
    }

    #[test]
    pub fn chip8_and_or() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 0b0110;
        cpu.registers[1] = 0b1010;

        cpu.registers[2] = 0b1101;
        cpu.registers[3] = 0b0011;


        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x11;

        cpu.memory[2] = 0x82;
        cpu.memory[3] = 0x31;

        cpu.memory[4] = 0x80;
        cpu.memory[5] = 0x22;

        cpu.run();

        assert_eq!(cpu.registers[0], 0b1110)
    }

    #[test]
    pub fn chip8_add_more_than_two_registers() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 10;
        cpu.registers[1] = 15;

        cpu.registers[2] = 23;
        cpu.registers[3] = 24;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x14;

        cpu.memory[2] = 0x82;
        cpu.memory[3] = 0x34;

        cpu.memory[4] = 0x80;
        cpu.memory[5] = 0x24;

        cpu.run();

        assert_eq!(cpu.registers[0], 72);
    }
}