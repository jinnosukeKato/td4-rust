use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: vec![0; 16],
        }
    }

    pub fn load_new(path: &str) -> Self {
        let mut mem = Memory::new();
        mem.load(path);
        mem
    }

    pub fn load(&mut self, path: &str) {
        let bin_file = File::open(path).unwrap();
        let lines = io::BufReader::new(bin_file).lines();
        for (addr, line) in lines.enumerate() {
            if let Ok(bin_line) = line {
                self.memory[addr] = u8::from_str_radix(&bin_line, 2).unwrap()
            }
        }
    }
}
