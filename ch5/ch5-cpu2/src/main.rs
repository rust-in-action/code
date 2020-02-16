const HALT: u8 = 0x0;
const ARITHMETIC_AND_LOGIC: u8 = 0x8;
const ADD_XY: u8 = 0x4;

struct CPU {
    // current_operation: u16, // no longer needed
    registers: [u8; 16], // increase number of registers to 16 (0..F), as per CHIP-8
    position_in_memory: usize, // add a program counter.. using a usize to simplify array indexing
    memory: [u8; 4096], // give the system 4kb memory
}

impl CPU {
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;     // 
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16; //
            let raw_op = op_byte1 << 8 | op_byte2;

            self.position_in_memory += 2; 

            let op_major = ((raw_op & 0xF000) >> 12) as u8;
            let x  =       ((raw_op & 0x0F00) >>  8) as u8; 
            let y  =       ((raw_op & 0x00F0) >>  4) as u8;
            let op_minor =  (raw_op & 0x000F) as u8;
        
            match (op_major, op_minor) {
                (HALT, HALT) => { return; },
                (ARITHMETIC_AND_LOGIC, ADD_XY) => self.add_xy(x, y),
                _ => unimplemented!("opcode {:04x}", raw_op), // this macro is useful when you are debugging, as it will promptly indicate that you have made a typo
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;  // initialise a few more registers
    cpu.registers[3] = 10;  //

    cpu.memory[0] = 0x80; cpu.memory[1] = 0x14; // 0x8014 -> ADD register 1 to register 0
    cpu.memory[2] = 0x80; cpu.memory[3] = 0x24; // 0x8014 -> ADD register 2 to register 0
    cpu.memory[4] = 0x80; cpu.memory[5] = 0x34; // 0x8014 -> ADD register 3 to register 0

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
