type Inner = u8; // PERF: try smaller and bigger types

pub struct Dignity(Inner);

impl Dignity {
    pub const None: Self = Dignity(0);

    pub const fn index(&self) -> Inner {
        self.0
    }
}
