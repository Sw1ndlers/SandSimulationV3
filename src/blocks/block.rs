use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

use crate::structs::{
    cell_size::{self, CellSize},
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

    fn append_to_mesh(&self, mesh_builder: &mut graphics::MeshBuilder) {
        self.get_pixel().append_to_mesh(mesh_builder);
    }

    fn apply_gravity(
        &mut self,
        ctx: &mut ggez::Context,
        blocks: &HashSet<GridPosition>,
        cell_size: CellSize,
    ) {
        let position = self.get_position();
        let next_position = position + GridPosition::new(0, 1, cell_size);

        let is_occupied = blocks.contains(&next_position);
        let is_offscreen = next_position.is_offscreen(ctx);

        if is_occupied || is_offscreen {
            return;
        }

        self.set_position(next_position);
    }
}
