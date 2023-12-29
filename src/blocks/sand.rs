use std::collections::{HashMap, HashSet};

use ggez::graphics::Color;

use super::block::Block;
use crate::components::{
    cell_size::CellSize, directions::Direction, grid_position::GridPosition, pixel::Pixel,
};

#[derive(Debug, Clone)]
pub struct Sand {
    pixel: Pixel,
    falling: bool,
}

impl Block for Sand {
    fn new(position: GridPosition) -> Self {
        let color = Color::BLACK;

        Self {
            pixel: Pixel::new(position, color),
            falling: true,
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
        ctx: &mut ggez::Context,
        blocks: &HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    ) {
        let next_position = self.should_apply_gravity(ctx, blocks, cell_size);

        match next_position {
            Some(position) => {
                self.falling = true;
                self.set_position(position);
            }
            None => {
                self.falling = false;
            }
        }

        if self.falling {
            self.pixel.color = Color::RED;
        } else {
            self.pixel.color = Color::BLACK;
        }
    }

    fn apply_motion(
        &mut self,
        ctx: &mut ggez::Context,
        blocks: &HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    ) {
        if self.falling {
            return;
        }

        // let open_directions = self.get_open_directions(blocks, cell_size);
        let directions = self.get_blocks_from_directions(blocks, cell_size, vec![
            Direction::BottomLeft,
            Direction::BottomRight,
            Direction::Bottom,
        ]);

        let bottom_block = directions.get(&Direction::Bottom);

        if directions.is_empty() || bottom_block.is_none() {
            return;
        }
        if bottom_block.unwrap().is_falling() {
            return;
        }

        let left_empty = directions.contains_key(&Direction::BottomLeft) == false;
        let right_empty = directions.contains_key(&Direction::BottomRight) == false;

        let next_position: GridPosition;

        if left_empty && right_empty {
            let random = rand::random::<bool>();

            if random {
                next_position = self.offset_bottom_left(cell_size)
            } else {
                next_position = self.offset_bottom_right(cell_size)
            }
        } else if left_empty {
            next_position = self.offset_bottom_left(cell_size)
        } else if right_empty {
            next_position = self.offset_bottom_right(cell_size)
        } else {
            // No open directions
            return;
        }

        if next_position.is_occupied(blocks) || next_position.is_offscreen(ctx) {
            return;
        }

        self.set_position(next_position);
    }
}
