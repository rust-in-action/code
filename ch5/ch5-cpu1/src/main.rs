const ARITHMETIC_AND_LOGIC: u8 = 0x8;
const HALT: u8 = 0x0; // <1> 
const ADD_XY: u8 = 0x4;

struct CPU {
    // current_operation: u16, // <2>
    registers: [u8; 16], // <3>
    position_in_memory: usize, // <4>
    memory: [u8; 4096], // <5>
}

impl CPU {
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;     // <6>
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16; // <6>
            let raw_op = op_byte1 << 8 | op_byte2; // <7>

            let op_major = ((raw_op & 0xF000) >> 12) as u8;
            let x  =       ((raw_op & 0x0F00) >>  8) as u8; 
            let y  =       ((raw_op & 0x00F0) >>  4) as u8;
            let op_minor =  (raw_op & 0x000F) as u8;

            self.position_in_memory += 2; // <8>
        
            match (op_major, op_minor) {
                (HALT, HALT) => { return; },  // <9>
                (ARITHMETIC_AND_LOGIC, ADD_XY) => self.add_xy(x, y),
                _ => unimplemented!("opcode {:04x}", raw_op), // <10>
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
    cpu.registers[2] = 10;  // <11>
    cpu.registers[3] = 10;  // <11>

    cpu.memory[0] = 0x80; cpu.memory[1] = 0x14; // <12> 
    cpu.memory[2] = 0x80; cpu.memory[3] = 0x24; // <13>
    cpu.memory[4] = 0x80; cpu.memory[5] = 0x34; // <14>

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}