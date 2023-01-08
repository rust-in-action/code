use core::panic;

struct CPU{
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize
}

impl CPU {

    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16;

            let opcode = op_byte1 << 8 | op_byte2;


            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            //let n = ((opcode & 0x000F) >> 4) as u8;
            let kk = (opcode & 0x00FF) as u8;
            let op_minor= (opcode & 0x000F) as u8;
            let addr = opcode & 0x0FFF;

            match opcode {
                0x000 => { return; },
                0x00E0 => { /* Clear Screen */},
                0x00EE => { self.ret(); },
                0x1000..=0x1FFF => { self.jmp(addr); },
                0x2000..=0x2FFF => { self.call(addr); },
                0x3000..=0x3FFF => { self.se(x, kk); },
                0x4000..=0x4FFF => { self.sne(x, kk); },
                0x5000..=0x5FFF => { self.se(x, y); },
                0x6000..=0x6FFF => { self.ld(x, kk); },
                0x7000..=0x7FFF => { self.add_xy(x, kk); },
                0x8000..=0x8FFF => {
                    match op_minor {
                        0 => { self.ld(x, self.registers[y as usize ])},
                        1 => { self.or_xy(x,y)},
                        2 => { self.and_xy(x, y) },
                        3 => { self.xor_xy(x, y) },
                        4 => { self.add_xy(x, y) },
                        5 => { self.sub_xy(x, y) },
                        6 => { self.lsb(x) },
                        7 => { self.sub_yx(x, y) },
                        8 => { self.msb(x) },
                        9 => { self.sne(x, y) },
                        _ => { todo!("opcode: {:04x}", opcode)},
                    }
                },
                _ => { todo!("opcode: {:04x}", opcode)},
            }
        }
    }

    fn ld (&mut self, vx: u8, kk: u8){
        self.registers[vx as usize] += kk;
    }

    fn se(&mut self, vx: u8, kk: u8){
        if vx == kk{
            self.position_in_memory += 2;
        }
    }

    fn sne(&mut self, vx: u8, kk: u8){
        if vx != kk{
            self.position_in_memory += 1;
        }
    }

    fn jmp(&mut self, addr: u16){
        self.position_in_memory = addr as usize;
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        if sp > stack.len() {
            panic!("Stack Overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self){
        if self.stack_pointer == 0 {
            panic!("Stack Underflow!")
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detect) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow_detect {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detect) = arg1.overflowing_sub(arg2);
        self.registers[x as usize] = val;

        if overflow_detect {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn sub_yx(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detect) = arg2.overflowing_sub(arg1);
        self.registers[x as usize] = val;

        if overflow_detect {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn lsb(&mut self, x: u8) {
        self.registers[0xF] = self.registers[x] & 1;
        self.registers[x] >>= 1;
    }

    fn msb(&mut self, x: u8) {
        self.registers[0x0f] = (self.registers[x] & 0b10000000) >> 7;
        self.registers[x] <<= 1;
    }

    fn and_xy(&mut self, x: u8, y: u8){
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ & y_;
    }

    fn or_xy(&mut self, x: u8, y: u8){
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ | y_;
    }

    fn xor_xy(&mut self, x: u8, y: u8){
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ ^ y_;
    }

}

fn main() {
    let mut cpu = CPU{
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;


    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
