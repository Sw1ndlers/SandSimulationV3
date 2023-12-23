#![allow(private_interfaces)]

use std::collections::{HashMap, HashSet};

use blocks::sand::Sand;
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::glam::*;
use ggez::graphics::{
    self, Canvas, Color, DrawParam, Drawable, Mesh, PxScale, Rect, Text,
};
use ggez::{Context, ContextBuilder, GameResult};

mod structs {
    pub mod cell_size;
    pub mod grid_position;
    pub mod pixel;
}
mod blocks {
    pub mod block;
    pub mod sand;
}

use blocks::block::Block;

use structs::cell_size::CellSize;
use structs::grid_position::GridPosition;

struct MainState {
    blocks: Vec<Box<dyn Block>>,
    occupied_positions: HashSet<GridPosition>,
    cell_size: CellSize,
    grid_color: Color,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let mut blocks: Vec<Box<dyn Block>> = Vec::new();
        let cell_size = CellSize::new(15.0, 15.0);


        let sand = Sand::new(GridPosition::new(1, 1, cell_size));
        blocks.push(Box::new(sand));


        let occupied_positions =
            blocks.iter().map(|block| block.get_position()).collect();

        MainState {
            blocks,
            cell_size,
            occupied_positions,
            grid_color: Color::BLACK,
        }
    }

    /// Should be called ONLY once every draw loop or when a block is added or removed
    fn update_occupied_positions(&mut self) {
        self.occupied_positions = self
            .blocks
            .iter()
            .map(|block| block.get_position())
            .collect()
    }

    fn position_occupied(&self, position: GridPosition) -> bool {
        self.occupied_positions.contains(&position)
    }

    fn insert_block(&mut self, block: Box<dyn Block>) {
        self.blocks.push(block);
    }

    fn draw_grid(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let window_size = ctx.gfx.size();
        let cell_size = self.cell_size;
        let grid_color = self.grid_color;

        let mut mesh_builder = graphics::MeshBuilder::new();

        let horizontal_lines = (window_size.1 / cell_size.height) as i32;
        let vertical_lines = (window_size.0 / cell_size.width) as i32;

        // creating lines from left to right
        for y in 0..horizontal_lines {
            let y_column = y as f32 * cell_size.width;

            let left_point = Vec2::new(0.0, y_column);
            let right_point = Vec2::new(window_size.0, y_column);

            mesh_builder
                .line(&[left_point, right_point], 1.0, grid_color)
                .unwrap();
        }

        // creating lines from top to bottom
        for x in 0..vertical_lines {
            let x_row = x as f32 * cell_size.height;

            let top_point = Vec2::new(x_row, 0.0);
            let bottom_point = Vec2::new(x_row, window_size.1);

            mesh_builder
                .line(&[top_point, bottom_point], 1.0, grid_color)
                .unwrap();
        }

        let mesh_data = mesh_builder.build();
        let mesh = Mesh::from_data(ctx, mesh_data);

        mesh.draw(canvas, DrawParam::default());
    }

    pub fn draw_fps(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let fps = ctx.time.fps().round();
        let bounds = Vec2::new(80.0, 20.0);
        let rect = Rect::new(0.0, 0.0, bounds.x, bounds.y);

        let mut fps_text = Text::new(format!("Fps: {}", fps));

        fps_text.set_bounds(bounds);
        fps_text.set_scale(PxScale::from(18.0));

        let fps_background = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color::BLACK,
        )
        .unwrap();

        fps_background.draw(canvas, DrawParam::default());
        fps_text.draw(canvas, DrawParam::default());
    }

    fn draw_pixels(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let mut mesh_builder = graphics::MeshBuilder::new();

        for block in self.blocks.iter_mut() {
            block.append_to_mesh(&mut mesh_builder);
        }

        let pixel_mesh_data = mesh_builder.build();
        let pixel_mesh = Mesh::from_data(ctx, pixel_mesh_data);

        pixel_mesh.draw(canvas, DrawParam::default());
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        // self.occupied_positions = self
        //     .blocks
        //     .iter()
        //     .map(|block| block.get_position())
        //     .collect();

        self.update_occupied_positions();

        for block in self.blocks.iter_mut() {
            block.apply_gravity(ctx, &self.occupied_positions, self.cell_size);
        }

        self.draw_grid(ctx, &mut canvas);
        self.draw_fps(ctx, &mut canvas);
        self.draw_pixels(ctx, &mut canvas);

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        let position = GridPosition::from_vec2((x, y), self.cell_size);

        if self.position_occupied(position) {
            return Ok(());
        }

        let sand = Sand::new(position);
        self.insert_block(Box::new(sand));

        self.update_occupied_positions();

        Ok(())
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Sand Simulation v3", "")
        .window_setup(
            ggez::conf::WindowSetup::default().title("Sand Simulation v3"),
        )
        .build()
        .expect("Could not create ggez context");

    let state = MainState::new(&mut ctx);

    ctx.gfx
        .set_mode(WindowMode {
            resizable: true,
            min_height: 280.0,
            min_width: 350.0,

            ..Default::default()
        })
        .unwrap();

    event::run(ctx, event_loop, state);
}
