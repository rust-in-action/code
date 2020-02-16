struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16], 
    stack_pointer: usize,
}

impl CPU {
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
            let opcode = op_byte1 << 8 | op_byte2;

            let x  =       ((opcode & 0x0F00) >>  8) as u8; 
            let y  =       ((opcode & 0x00F0) >>  4) as u8;
            let op_minor =  (opcode & 0x000F) as u8;
            let addr =       opcode & 0x0FFF;

            self.position_in_memory += 2;

            match opcode {
                0x0000 => { return; },
                0x00EE => { self.ret(); },
                0x2000...0x2FFF => { self.call(addr); },
                0x8000...0x8FFF => {
                    match op_minor {
                        4 => { self.add_xy(x, y); }
                        _ => { unimplemented!("opcode: {:04x}", opcode); },
                    }
                },
                _ => unimplemented!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
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
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.memory[0x000] = 0x21; cpu.memory[0x001] = 0x00; 
    cpu.memory[0x002] = 0x21; cpu.memory[0x003] = 0x00;

    cpu.memory[0x100] = 0x80; cpu.memory[0x101] = 0x14; 
    cpu.memory[0x102] = 0x80; cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00; cpu.memory[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}