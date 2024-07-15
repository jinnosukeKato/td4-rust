pub struct Register {
    pub a: u8,
    pub b: u8,
    pub carry: bool,
    pub pc: u8,
}

impl Register {
    pub fn new() -> Self {
        Register {
            a: 0,
            b: 0,
            carry: false,
            pc: 0,
        }
    }
}
