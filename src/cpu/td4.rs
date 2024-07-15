use crate::cpu::memory::*;
use crate::cpu::opcode::*;
use crate::cpu::port::*;
use crate::cpu::register::*;

pub struct TD4 {
    pub memory: Memory,
    pub port: Port,
    pub register: Register,
}

impl TD4 {
    pub fn new(path: &str) -> Self {
        TD4 {
            memory: Memory::load_new(path),
            port: Port::new(),
            register: Register::new(),
        }
    }

    pub fn fetch(&self) -> u8 {
        let pc = self.register.pc;
        self.memory.memory[pc as usize]
    }

    pub fn decode(data: u8) -> (OpCode, u8) {
        let op_code: OpCode = num_traits::FromPrimitive::from_u8(data >> 4).unwrap();
        let operand = data & 0x0f;
        (op_code, operand)
    }

    pub fn execute(&mut self, op_code: OpCode, operand: u8) {
        match op_code {
            OpCode::MovA => {
                self.register.a = operand;
                self.register.carry = false;
            }
            OpCode::MovB => {
                self.register.b = operand;
                self.register.carry = false;
            }
            OpCode::MovAB => {
                self.register.a = self.register.b;
                self.register.carry = false;
            }
            OpCode::MovBA => {
                self.register.b = self.register.a;
                self.register.carry = false;
            }
            OpCode::AddA => {
                let result = self.register.a + operand;
                self.register.carry = (0b1_0000 & result) != 0;
                self.register.a = 0xF & result;
            }
            OpCode::AddB => {
                let result = self.register.b + operand;
                self.register.carry = (0b1_0000 & result) != 0;
                self.register.b = 0xF & result;
            }
            OpCode::InA => {
                self.register.a = self.port.input;
                self.register.carry = false;
            }
            OpCode::InB => {
                self.register.b = self.port.input;
                self.register.carry = false;
            }
            OpCode::OutIm => {
                self.port.output = operand;
                self.register.carry = false;
            }
            OpCode::OutB => {
                self.port.output = self.register.b;
                self.register.carry = false;
            }
            OpCode::Jmp => {
                self.register.pc = operand;
                self.register.carry = false;
                return;
            }
            OpCode::Jnc => {
                if !self.register.carry {
                    self.register.pc = operand;
                    self.register.carry = false;
                    return;
                }
            }
        }

        self.register.pc = (self.register.pc + 1) & 0xF;
    }
}
