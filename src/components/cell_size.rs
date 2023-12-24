#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CellSize {
    pub width: f32,
    pub height: f32,
}

impl CellSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
