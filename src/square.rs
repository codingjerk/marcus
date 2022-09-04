type Inner = u8; // PERF: try smaller and bigger types

pub struct Square(Inner);

impl Square {
    pub const fn index(&self) -> Inner {
        self.0
    }
}
