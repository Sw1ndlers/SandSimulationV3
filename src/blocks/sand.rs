use ggez::graphics::Color;

use super::block::Block;
use crate::structs::{
    cell_size::{self, CellSize},
    grid_position::GridPosition,
    pixel::Pixel,
};

pub struct Sand {
    pixel: Pixel,
}

impl Block for Sand {
    fn new(position: GridPosition) -> Self {
        let color = Color::BLACK;

        Self {
            pixel: Pixel::new(position, color),
        }
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
}
