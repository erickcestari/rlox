enum OpCode {
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpConstant,
    OpReturn,
}

struct Chunk {
    count: i32,
    capacity: i32,
    code: Vec<u8>,
    lines: Vec<i32>,
}
