// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem::transmute;

/// A single instruction from BPF bytecode.
pub struct Instruction {
    #[allow(missing_docs)]
    pub instruction_data: u64,
}

#[allow(missing_docs)]
impl Instruction {
    pub fn opcode(&self) {
        unimplemented!()
    }

    pub fn src(&self) {
        unimplemented!()
    }

    pub fn dst(&self) {
        unimplemented!()
    }

    pub fn immediate_value(&self) -> u32 {
        (self.instruction_data >> 32) as u32
    }

    #[allow(unsafe_code)]
    pub fn instruction_class(&self) -> InstructionClass {
        unsafe { transmute((self.instruction_data & 0x07) as u8) }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum InstructionClass {
    LD = 0x00,
    LDX = 0x01,
    ST = 0x02,
    STX = 0x03,
    ALU = 0x04,
    JMP = 0x05,
    ALU64 = 0x07,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_class() {
        let inst = Instruction { instruction_data: 0xffffffff00000104 };
        assert_eq!(inst.immediate_value(), 0xffffffff);
        assert_eq!(inst.instruction_class(), InstructionClass::ALU);

        let inst = Instruction { instruction_data: 0x0000000300000107 };
        assert_eq!(inst.immediate_value(), 0x03);
        assert_eq!(inst.instruction_class(), InstructionClass::ALU64);
    }
}
