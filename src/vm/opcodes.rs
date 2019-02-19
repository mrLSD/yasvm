/// Opcode Instructions data
/// contain Opcode and Opcode data if exist.
/// It's possible that Opcode not recognized
pub struct OpcodeInstruction<T> {
    /// Opcode Instruction code
    code: u8,
    /// Opcode instruction
    opcode: Option<Opcode>,
    /// Opcode Item Value
    item: Option<T>,
}

/// Instructions - opcodes instructions set
pub struct Instructions<T> {
    data: Vec<u8>,
    /// Opcode parsed instructions set
    instructions: Vec<OpcodeInstruction<T>>,
}

/// Basic Opcode Instructions implementation
impl<T> Instructions<T> {
    /// Init Opcodes instructions set
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            instructions: vec![],
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Opcode {
    /// 0s: Stop and Arithmetic Operations
    /// ----------
    /// Halts execution.
    STOP = 0x00,
    /// Addition operation.
    ADD = 0x01,
    /// Multiplication operation.
    MUL = 0x02,
}
