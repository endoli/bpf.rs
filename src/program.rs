// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Instruction;

/// A block of code represented by eBPF bytecodes.
pub struct Program {
    /// The BPF bytecode for this program.
    pub instructions: Vec<Instruction>,
}

impl Program {
    /// Construct a new BPF program. It must be
    /// valid.
    pub fn new(instructions: &[u64]) -> Result<Self, ValidationError> {
        let p = Program {
            instructions: instructions.iter()
                .map(|&i| Instruction { instruction_data: i })
                .collect::<Vec<Instruction>>(),
        };

        p.validate()
    }

    /// Validate a BPF program.
    fn validate(self) -> Result<Self, ValidationError> {
        Ok(self)
    }
}

pub enum ValidationError {
}
