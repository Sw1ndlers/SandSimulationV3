use super::{cell_size::CellSize, grid_position::GridPosition};
use strum_macros::EnumIter; 

/// [TopLeft]    | [Top]    | [TopRight]
///
/// [Left]       | [Middle] | [Right]
///
/// [BottomLeft] | [Bottom] | [BottomRight]
#[derive(Eq, Hash, PartialEq, EnumIter)]
pub enum Direction {
    TopLeft,
    Top,
    TopRight,

    Left,
    Middle,
    Right,

    BottomLeft,
    Bottom,
    BottomRight,
}

impl Direction {
    pub fn get_offset(&self, cell_size: CellSize) -> GridPosition {
        match self {
            Direction::TopLeft => GridPosition::new(-1, -1, cell_size),
            Direction::Top => GridPosition::new(0, -1, cell_size),
            Direction::TopRight => GridPosition::new(1, -1, cell_size),

            Direction::Left => GridPosition::new(-1, 0, cell_size),
            Direction::Middle => panic!("Offset within itself"), // Should never happen
            Direction::Right => GridPosition::new(1, 0, cell_size),

            Direction::BottomLeft => GridPosition::new(-1, 1, cell_size),
            Direction::Bottom => GridPosition::new(0, 1, cell_size),
            Direction::BottomRight => GridPosition::new(1, 1, cell_size),
        }
    }
}
