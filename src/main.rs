
struct CPU{
    registers:[u8;16], // general purpose regs 0 to F
    position_in_memory:usize, // program counter
    memory:[u8;0x1000], // 4096 bytes 
    stack:[u16;16], // after 16 nested func calls produce a stack overflow
    stack_pointer:usize, // usize for indexing in the stack
}

impl CPU{   
    fn read_curr_opcode(&self) -> u16 {
        let p = self.position_in_memory; 
        let hi= self.memory[p] as u16;
        let lo= self.memory[p+1] as u16;

        return (hi<<8)|lo; 
    }

    // main loop
    fn run(&mut self){
        loop{
            let op_code = self.read_curr_opcode();
            self.position_in_memory += 2; // move the program counter
            
            // decode
            let c = ((op_code & 0xF000) >> 12) as u8; // instruction family
            let x = ((op_code & 0x0F00) >> 8) as u8; // x register
            let y = ((op_code & 0x00F0) >> 4) as u8; // y register
            let d = (op_code & 0x000F) as u8; // operation
            let nnn = op_code&0x0FFF;


            // dispatch (control unit)
            match c{
                0x0 => {
                    if(self.exec_0(op_code)){
                        break;
                    }
                }
                0x2 => self.call(nnn),
                0x8 => self.exec_8xy(x,y,d),
                _ => todo!("opcode {:04x}", op_code),
            }
        }
    }

    // 0x0 family
    fn exec_0(&mut self, op_code:u16) -> bool{
        match op_code{
            0x0000 => true, // HALT
            0x00EE => { 
                self.ret(); // RET
                false
            }
            _ => todo!("opcode {:04x}", op_code),
        }

    }

    // 0x8 family
    fn exec_8xy(&mut self, x:u8, y:u8, d:u8){
        match d{
            0x0 => self.mov_xy(x, y),
            0x1 => self.or_xy(x, y),
            0x2 => self.and_xy(x, y),
            0x3 => self.xor_xy(x, y),
            0x4 => self.add_xy(x, y),
            0x5 => self.sub_xy(x, y),
            _   => todo!("8xy{:x}", d),
        }
    }

    // ALU operations
    fn mov_xy(&mut self, x:u8, y:u8){
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn or_xy(&mut self, x:u8, y:u8){
        self.registers[x as usize] |= self.registers[y as usize];
    }

    fn and_xy(&mut self, x:u8, y:u8){
        self.registers[x as usize] &= self.registers[y as usize];
    }

    fn xor_xy(&mut self, x:u8, y:u8){
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    fn add_xy(&mut self, x:u8, y:u8){
        let arg1= self.registers[x as usize];
        let arg2= self.registers[y as usize];

        let(value, carry) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = value;
        self.registers[0xF] = carry as u8;
    }

    fn sub_xy(&mut self, x:u8, y:u8){
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let(value, borrow) = arg1.overflowing_sub(arg2);
        self.registers[x as usize] = value;
        self.registers[0xF] = (!borrow) as u8;
    }

    // stack control
    fn call(&mut self, addr:u16){
        let sp=self.stack_pointer;
        let stack = &mut self.stack;
        if(sp > stack.len()){
            panic!("STACK OVERFLOLW!!!");
        }
        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer+=1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self){
        if(self.stack_pointer == 0){
            panic!("STACK UNDERFLOW!!!"); 
        }
        self.stack_pointer -=1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize; 
    }

    // opcode helper
    fn opcode_8xy(x:u8, y:u8, d:u8) -> u16 {
        return (0x8 << 12) | ((x as u16) << 8) | ((y as u16) << 4) | d as u16;
    }
}

fn main() {
    let mut cpu = CPU{
        registers:[0; 16],
        memory:[0; 4096],
        position_in_memory: 0,
        stack:[0; 16],
        stack_pointer: 0,
    };


    // test
    let x = 5; 
    let y= 10;
    cpu.registers[0] = x;
    cpu.registers[1] = y;

    let mem = &mut cpu.memory;

    // in 0x000
    let prog_main = [
        0x2100, // call subrutine at 0x100
        0x2100, 
        0x000, // halt
    ];

    // load main program in memory
    let mut pc=0x000;
    for op in prog_main{
        mem[pc]= (op>>8) as u8; // high byte
        mem[pc+1]=op as u8; // low byte
        pc +=2;
    }

    // subrutine loaded at 0x100
    let prog_func=[
        CPU::opcode_8xy(0, 1, 0x4), // ADD V0, V1
        CPU::opcode_8xy(0, 1, 0x5), // SUB V0, V1
        0x00EE,
    ];

    // load func in memory
    pc = 0x100;
    for op in prog_func{
        mem[pc]= (op>>8) as u8; // high byte
        mem[pc+1]=op as u8; // low byte
        pc +=2;
    }
   
    // START!!!!!
    cpu.run();
    
    assert_eq!(cpu.registers[0], 5);

    let mid = x+y;
    println!("{} + {} = {}", x,y,mid);
    println!("{} - {} = {}", mid, y, cpu.registers[0]);
}
