pub struct Memory {
    pub data: Vec<String>,
}

impl Memory {
    pub fn new(program: Vec<&str>) -> Self {
        Memory {
            data: program.iter().map(|s| s.to_string()).collect(),
        }
    }
    
    pub fn fetch(&self, addr: usize) -> Option<String> {
        self.data.get(addr).cloned()
    }
}