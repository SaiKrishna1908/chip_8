pub enum Op {
    ADD
}

pub struct CPU {
    pub memory: [u8; 0x1000],
    position_in_memory: usize,
    pub registers: [u8; 16]
}

impl CPU {

    pub fn get_memory(&mut self) -> [u8; 0x1000] {
        self.memory
    }

    /*
        Initialize CPU
     */
    pub fn new() -> CPU {
        CPU {
            memory: [0; 4096],
            registers: [0; 16],
            position_in_memory: 0
        }
    }

    fn read_op(&mut self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p+1] as u16;
        self.position_in_memory += 2;
        op_byte1 << 8 | op_byte2
    }

    /*
        Load the addition op code to current_operation
        u16 -> ____ ____ ____ ____
     */
    pub fn run(&mut self) {
        let op = self.read_op();

        let c = ((op & 0xf000) >> 12) as u8;
        let x = ((op & 0x0f00) >> 8) as u8;
        let y = ((op & 0x00f0) >> 4) as u8;
        let d = ((op & 0x000f) >> 0) as u8;

        match (c,x,y,d) {
            (0, 0, 0, 0) => { return; },
            (0x8, _, _, 0x4) => {
                self.add_xy(x, y)
            },
            _ => todo!()
        }
    }

    /*
        Sets value in register x to value in register x + register y
     */
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
}