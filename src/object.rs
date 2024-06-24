#[derive(Copy, Clone)]
enum ObjType {
    String,
}

trait ObjTrait {
    fn get_type(&self) -> ObjType;
}

pub struct Obj {
    obj_type: ObjType,
    next: Option<Box<dyn ObjTrait>>,
}

struct ObjString {
    obj: Obj,
    length: usize,
    chars: String,
}

impl ObjTrait for ObjString {
    fn get_type(&self) -> ObjType {
        self.obj.obj_type.clone()
    }
}
