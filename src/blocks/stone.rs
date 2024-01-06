use std::collections::HashMap;

use ggez::graphics::Color;

use super::block::Block;
use crate::components::{cell_size::CellSize, grid_position::GridPosition, pixel::Pixel};

#[derive(Debug, Clone)]
pub struct Stone {
    pixel: Pixel,
    falling: bool,
}

impl Block for Stone {
    fn new(position: GridPosition) -> Self {
        let color = Color::from_rgb(98, 95, 89);

        Self {
            pixel: Pixel::new(position, color),
            falling: false,
        }
    }

    fn box_clone(&self) -> Box<dyn Block> {
        Box::new(self.clone())
    }

    fn get_pixel(&self) -> &Pixel {
        &self.pixel
    }

    fn set_position(&mut self, position: GridPosition) {
        self.pixel.position = position;
    }

    fn get_position(&self) -> GridPosition {
        self.pixel.position
    }

    fn is_falling(&self) -> bool {
        self.falling
    }

    fn apply_gravity(
        &mut self,
        _ctx: &mut ggez::Context,
        _blocks: &HashMap<GridPosition, Box<dyn Block>>,
        _cell_size: CellSize,
    ) {
        return;
    }

    fn apply_motion(
        &mut self,
        _ctx: &mut ggez::Context,
        _blocks: &HashMap<GridPosition, Box<dyn Block>>,
        _cell_size: CellSize,
    ) {
        return;
    }
}
