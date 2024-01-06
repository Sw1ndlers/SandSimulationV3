use crate::components::{
    cell_size::CellSize, directions::Direction, grid_position::GridPosition, pixel::Pixel,
};

use ggez::graphics;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub trait Block {
    fn new(position: GridPosition) -> Self
    where
        Self: Sized;
    fn get_pixel(&self) -> &Pixel;
    fn set_position(&mut self, position: GridPosition);
    fn get_position(&self) -> GridPosition;
    fn box_clone(&self) -> Box<dyn Block>;
    fn is_falling(&self) -> bool;

    fn apply_motion(
        &mut self,
        ctx: &mut ggez::Context,
        blocks: &HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    );

    fn apply_gravity(
        &mut self,
        ctx: &mut ggez::Context,
        blocks: &HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    );

    fn append_to_mesh(&self, mesh_builder: &mut graphics::MeshBuilder) {
        self.get_pixel().append_to_mesh(mesh_builder);
    }

    fn should_apply_gravity(
        &self,
        ctx: &mut ggez::Context,
        blocks: &HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    ) -> Option<GridPosition> {
        let position = self.get_position();
        let next_position = position + GridPosition::new(0, 1, cell_size);

        let is_occupied = blocks.contains_key(&next_position);
        let is_offscreen = next_position.is_offscreen(ctx);

        if is_occupied || is_offscreen {
            return None;
        }

        Some(next_position)
    }

    fn get_open_directions(
        &self,
        blocks: &HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    ) -> Vec<Direction> {
        let position = self.get_position();
        let mut open_directions = Vec::new();

        let bottom_left = position + Direction::BottomLeft.get_offset(cell_size);
        let bottom_right = position + Direction::BottomRight.get_offset(cell_size);
        let bottom = position + Direction::Bottom.get_offset(cell_size);

        if !blocks.contains_key(&bottom_left) {
            open_directions.push(Direction::BottomLeft);
        }

        if !blocks.contains_key(&bottom_right) {
            open_directions.push(Direction::BottomRight);
        }

        if !blocks.contains_key(&bottom) {
            open_directions.push(Direction::Bottom);
        }

        open_directions
    }

    fn get_surrounding_blocks<'a>(
        &self,
        blocks: &'a HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    ) -> HashMap<Direction, Option<&'a Box<dyn Block>>> {
        let position = self.get_position();
        let mut surrounding_blocks = HashMap::new();

        for direction in Direction::iter() {
            let offset = direction.get_offset(cell_size);
            let position = position + offset;

            let block = blocks.get(&position);

            surrounding_blocks.insert(direction, block);
        }

        surrounding_blocks
    }

    /// Returns blocks if they exist in the given directions
    fn get_blocks_from_directions<'a>(
        &self,
        blocks: &'a HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
        directions: Vec<Direction>,
    ) -> HashMap<Direction, &'a Box<dyn Block>> {
        let mut direction_blocks = HashMap::new();
        let position = self.get_position();

        for direction in directions {
            let offset = direction.get_offset(cell_size);
            let position = position + offset;

            let block = blocks.get(&position);

            match block {
                Some(block) => {
                    direction_blocks.insert(direction, block);
                }
                None => continue,
            }
        }

        direction_blocks
    }

    fn offset_bottom_left(&mut self, cell_size: CellSize) -> GridPosition {
        self.get_position() + Direction::BottomLeft.get_offset(cell_size)
    }

    fn offset_bottom_right(&mut self, cell_size: CellSize) -> GridPosition {
        self.get_position() + Direction::BottomRight.get_offset(cell_size)
    }
}
