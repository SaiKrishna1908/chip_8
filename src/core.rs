pub enum Op {
    ADD
}

pub struct CPU {
    pub memory: [u8; 0x1000],
    position_in_memory: usize,
    pub registers: [u8; 16]
}

impl CPU {

    /// Initialize a new CPU instance
    /// with default values
    pub fn new() -> CPU {
        CPU {
            memory: [0; 4096],
            registers: [0; 16],
            position_in_memory: 0
        }
    }

    /// Returns the OP-Code according to Chip-8 instruction set
    /// Each instruction has a size of u8
    /// Combine two u8 instructions to create the final OP-Code
    fn read_op(&mut self) -> u16 {
        let p = self.position_in_memory;

        // read instruction from memory
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p+1] as u16;

        // reset memory to 0
        self.memory[p] = 0;
        self.memory[p+1] = 0;

        // increment memory pointer
        self.position_in_memory += 2;

        op_byte1 << 8 | op_byte2
    }

    /// Simulates the cpu
    /// loop until all instructions are carried out
    /// Load the addition op code to current_operation
    /// u16 -> ____ ____ ____ ____
    ///
    /// An instruction in Chip-8 is of 16 bits
    /// which is divided into two bytes - High Byte (HB) and Low Byte (LB)
    /// Each Byte is further divided into two Nibbles - High Nibble (HN) and Low Nibble (LN)
    ///
    /// HBHN  HBLN  LBHN  LBLN
    /// ----  ----  ----  ----
    pub fn run(&mut self) {

        loop {
            let op = self.read_op();

            /// Extract HN of HB
            let c = ((op & 0xf000) >> 12) as u8;

            /// Extract LN of HB
            let x = ((op & 0x0f00) >> 8) as u8;

            /// Extract HN of LB
            let y = ((op & 0x00f0) >> 4) as u8;

            /// Extract LN of LB
            let d = ((op & 0x000f) >> 0) as u8;

            match (c,x,y,d) {
                (0, 0, 0, 0) => { break; },
                (0x8, a, b, 0x1) => {
                    self.or_xy(a,b)
                }
                (0x8, a, b, 0x2) => {
                    self.and_xy(a,b)
                },
                (0x8, a, b, 0x4) => {
                    self.add_xy(a, b)
                },
                (0x8, a, b, 0x5) => {
                    self.sub_xy(a, b)
                }
                _ => todo!()
            }
        }
    }

    /// Xor of two values x, y and store in x
    /// OP-CODE: 0X8xy2
    ///
    /// # Arguments
    /// * `x` - index of first register
    /// * `y` - index of second register
    fn or_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        self.registers[x as usize] = arg1 | arg2;
    }

    /// And of two values x, y and store in x
    /// OP-CODE: 0x8xy1
    ///
    /// # Arguments
    /// * `x` - index of first register
    /// * `y` - index of second register
    fn and_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        self.registers[x as usize] = arg1 & arg2;
    }


    /// Add values at register x and register y and stores in register x.
    /// In case of overflow register 15 is set to high
    /// OP-CODE: 0X8xy4
    ///
    /// # Arguments
    /// * `x` - index of first register
    /// * `y` - index of second register
    fn add_xy(&mut self, x: u8, y: u8)  {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }

    /// Subtract values at register x and register y and stores in register x
    /// In case of negative value register 15 is set to high
    /// OP-CODE: 0X8xy5
    ///
    /// # Arguments
    /// * `x` - index of first register
    /// * `y` - index of second register
    fn sub_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];


        let (value, overflow) = arg1.overflowing_sub(arg2);

        let max_u8_size = 255;

        if overflow {
            self.registers[0xf] = 1;
            self.registers[x as usize] = max_u8_size - value + 1;
        } else {
            self.registers[0xf] = 0;
            self.registers[x as usize] = value;
        }
    }
}