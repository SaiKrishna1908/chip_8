pub enum Op {
    ADD
}

pub struct CPU {
    current_operation: u16,
    pub registers: [u8; 2]
}

impl CPU {

    /*
        Initialize CPU
     */
    pub fn new() -> CPU {
        CPU {
            current_operation: 0,
            registers: [0, 2]
        }
    }

    fn read_op(&self) -> u16 {
        return self.current_operation;
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
        self.registers[x as usize] += self.registers[y as usize]
    }

    pub fn set_current_operation(&mut self ,op: Op) {
        match op {
            Op::ADD => self.current_operation = 0x8014,
            _ => todo!()
        }
    }
}