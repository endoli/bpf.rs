// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// A virtual machine for executing [a BPF program].
///
/// [a BPF program]: struct.Program.html
pub struct VM {
}

impl VM {
    /// Register an external function that can be invoked from
    /// bytecode.
    pub fn register_external_function(&mut self) {
        unimplemented!()
    }

    /// Execute the bytecode that has been loaded into
    /// this VM.
    pub fn execute(&mut self) {
        unimplemented!()
    }
}
