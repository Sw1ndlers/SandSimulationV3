use crate::structs::{
    cell_size::CellSize, grid_position::GridPosition, pixel::Pixel,
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
}
