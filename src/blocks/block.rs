use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

use crate::components::{
    cell_size::{self, CellSize},
    directions::Direction,
    grid_position::GridPosition,
    pixel::Pixel,
};
use ggez::graphics;

pub trait Block {
    fn new(position: GridPosition) -> Self
    where
        Self: Sized;
    fn get_pixel(&self) -> &Pixel;
    fn set_position(&mut self, position: GridPosition);
    fn get_position(&self) -> GridPosition;
    fn box_clone(&self) -> Box<dyn Block>;
    fn is_falling(&self) -> bool;

    fn append_to_mesh(&self, mesh_builder: &mut graphics::MeshBuilder) {
        self.get_pixel().append_to_mesh(mesh_builder);
    }

    fn apply_gravity(
        &mut self,
        ctx: &mut ggez::Context,
        blocks: &HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    );

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

        // self.set_position(next_position);
        Some(next_position)
    }

    fn apply_motion(
        &mut self,
        ctx: &mut ggez::Context,
        blocks: &HashMap<GridPosition, Box<dyn Block>>,
        occupied_positions: &HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    );

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
        blocks: &'a mut HashMap<GridPosition, Box<dyn Block>>,
        cell_size: CellSize,
    ) -> Vec<Option<&'a mut Box<dyn Block>>> {
        let position = self.get_position();
        let mut surrounding_blocks = Vec::new();

        let top_left = position + Direction::TopLeft.get_offset(cell_size);
        let top = position + Direction::Top.get_offset(cell_size);
        let top_right = position + Direction::TopRight.get_offset(cell_size);
        let left = position + Direction::Left.get_offset(cell_size);
        let right = position + Direction::Right.get_offset(cell_size);
        let bottom_left = position + Direction::BottomLeft.get_offset(cell_size);
        let bottom = position + Direction::Bottom.get_offset(cell_size);
        let bottom_right = position + Direction::BottomRight.get_offset(cell_size);

        surrounding_blocks.push(blocks.get_mut(&top_left));



        surrounding_blocks
    }

    fn offset_bottom_left(&mut self, cell_size: CellSize) -> GridPosition {
        self.get_position() + Direction::BottomLeft.get_offset(cell_size)
    }

    fn offset_bottom_right(&mut self, cell_size: CellSize) -> GridPosition {
        self.get_position() + Direction::BottomRight.get_offset(cell_size)
    }
}
