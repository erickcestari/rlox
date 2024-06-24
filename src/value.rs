use crate::object::Obj;

enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    Obj(Box<Obj>),
}

struct ValueArray {
    capacity: usize,
    count: usize,
    values: *mut Value,
}
