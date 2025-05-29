use crate::cpu::CPU;
use crate::memory::Memory;

mod utils;
mod memory;
mod cpu;

fn main() {
    
    let program = vec![
        "001012",
        "002001",
        "999000",
    ];
    
    let memory = Memory::new(program);
    let mut cpu = CPU::new();
    cpu.run(&memory);
}
