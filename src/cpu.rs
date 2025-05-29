use crate::memory::Memory;
use crate::utils::from_ternary;

pub struct CPU {
    pub r0: usize,
    pub pc: usize,
}

impl CPU {
    pub fn new() -> CPU {
        CPU { r0: 0, pc: 0 }
    }
    
    pub fn run(&mut self, memory: &Memory) {
        loop {
            let instr = match memory.fetch(self.pc) {
                Some(code) => code,
                None => break,
            };
            self.pc += 1;
            
            if instr.len() != 6 {
                println!("An incorrect command: {}", instr);
                continue;
            }
            
            let opcode = &instr[..3];
            let operand = &instr[3..];
            
            match opcode {
                "001" => { // LOAD
                    self.r0 = from_ternary(operand);
                }
                "002" => { // ADD
                    self.r0 += from_ternary(operand);
                }
                "999" => { // HALT
                    println!("[END] R0 = {}", self.r0);
                    break;
                }
                _ => println!("an unknown command: {}", instr),
            }
        }
    }
}