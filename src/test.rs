#[cfg(test)]
pub mod tests {
    use std::cmp::{max, min};
    use crate::core::{CPU};
    use crate::utils::decompose;

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

    #[test]
    pub fn test_decompose_u16_to_u8() {

        let number = 0b1101_1001_0011_1110;

        let decomposed_codes = decompose(number);

        assert_eq!(decomposed_codes.0, 0b1101_1001);
        assert_eq!(decomposed_codes.1, 0b0011_1110);
    }

    #[test]
    pub fn test_function_call() {

        let func: [u16; 3] = [
            0x8014,
            0x8014,
            0x00EE
        ];

        let mut cpu = CPU::new();

        cpu.registers[0] = 5;
        cpu.registers[1] = 10;


        // Call function at addr 0x100 2 times
        (cpu.memory[0x000], cpu.memory[0x001]) = decompose(0x2100);
        (cpu.memory[0x002], cpu.memory[0x003]) = decompose(0x2100);

        // Halt program
        (cpu.memory[0x004], cpu.memory[0x005]) = decompose(0x0000);

        // Function definition, it adds value in register 1 to register 0 and stores in 0.
        (cpu.memory[0x100], cpu.memory[0x101]) = decompose(0x8014);
        // Function definition, it adds value in register 1 to register 0 and stores in 0.
        (cpu.memory[0x102], cpu.memory[0x103]) = decompose(0x8014);
        // Return
        (cpu.memory[0x104], cpu.memory[0x105]) = decompose(0x00EE);

        cpu.run();

        assert_eq!(cpu.registers[0], 45);
    }
}