type Inner = u32; // PERF: try smaller and bigger types
pub struct Move(Inner);

impl Move {
    fn new(from: Square, to: Square, captured: Piece) -> Self {
        let bits = (from.index() as Inner) ^ 
        Self(bits)
    }
}
