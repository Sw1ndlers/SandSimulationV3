#![allow(private_interfaces)]

use std::collections::{HashMap, HashSet};

use blocks::sand::Sand;
use blocks::stone::Stone;
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::glam::*;
use ggez::graphics::{
    self, Canvas, Color, DrawMode, DrawParam, Drawable, Mesh, PxScale, Rect, Text,
};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};

mod components {
    pub mod block_types;
    pub mod cell_size;
    pub mod directions;
    pub mod grid_position;
    pub mod pixel;
}
mod blocks {
    pub mod block;
    pub mod sand;
    pub mod stone;
}

use blocks::block::Block;

use components::block_types::BlockType;
use components::cell_size::CellSize;
use components::grid_position::GridPosition;

const CELL_SIZE: CellSize = CellSize {
    width: 8.0,
    height: 8.0,
};

const FPS: u32 = 60;

struct MainState {
    blocks: HashMap<GridPosition, Box<dyn Block>>,
    place_range: i32,
    cell_size: CellSize,
    grid_color: Color,
    mouse_down: bool,
    selected_block: BlockType,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let blocks: HashMap<GridPosition, Box<dyn Block>> = HashMap::new();
        let cell_size = CELL_SIZE;

        MainState {
            blocks,
            cell_size,
            grid_color: Color::BLACK,
            mouse_down: false,
            selected_block: BlockType::Sand,
            place_range: 5,
        }
    }

    fn clone_blocks(&self) -> HashMap<GridPosition, Box<dyn Block>> {
        let mut cloned_blocks: HashMap<GridPosition, Box<dyn Block>> = HashMap::new();

        for (_position, block) in self.blocks.iter() {
            cloned_blocks.insert(block.get_position(), block.box_clone());
        }

        cloned_blocks
    }

    fn update_positions(&mut self) {
        let mut new_blocks: HashMap<GridPosition, Box<dyn Block>> = HashMap::new();

        for (_position, block) in self.blocks.iter_mut() {
            let position = block.get_position();
            new_blocks.insert(position, block.box_clone());
        }

        self.blocks = new_blocks;
    }

    fn position_occupied(&self, position: GridPosition) -> bool {
        self.blocks.contains_key(&position)
    }

    fn insert_block(&mut self, block: Box<dyn Block>) {
        self.blocks.insert(block.get_position(), block);
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

    fn draw_fps(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let fps = ctx.time.fps().round();
        let bounds = Vec2::new(80.0, 20.0);
        let rect = Rect::new(0.0, 0.0, bounds.x, bounds.y);

        let mut fps_text = Text::new(format!("Fps: {}", fps));

        fps_text.set_bounds(bounds);
        fps_text.set_scale(PxScale::from(18.0));

        let fps_background =
            Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::BLACK).unwrap();

        fps_background.draw(canvas, DrawParam::default());
        fps_text.draw(canvas, DrawParam::default());
    }

    fn draw_pixels(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let mut mesh_builder = graphics::MeshBuilder::new();

        for (_pos, block) in self.blocks.iter_mut() {
            block.append_to_mesh(&mut mesh_builder);
        }

        let pixel_mesh_data = mesh_builder.build();
        let pixel_mesh = Mesh::from_data(ctx, pixel_mesh_data);

        pixel_mesh.draw(canvas, DrawParam::default());
    }

    fn spawn_block(&mut self, block_type: BlockType, grid_position: GridPosition) -> GameResult {
        if self.position_occupied(grid_position) {
            return Ok(());
        }

        let block: Box<dyn Block>;

        match block_type {
            BlockType::Sand => {
                block = Box::new(Sand::new(grid_position));
            }
            BlockType::Stone => {
                block = Box::new(Stone::new(grid_position));
            }
            BlockType::Eraser => {
                return Ok(());
            }
        }

        self.insert_block(block);

        Ok(())
    }

    /// Range is in number of cells
    fn generate_positions(
        &self,
        start_position: GridPosition,
        range: i32,
    ) -> HashSet<GridPosition> {
        let mut positions: HashSet<GridPosition> = HashSet::new();

        for x in -range..range {
            for y in -range..range {
                let position = start_position + GridPosition::new(x, y, self.cell_size);

                positions.insert(position);
            }
        }

        positions
    }

    /// Range is in number of cells
    fn _generate_random_positions(
        &self,
        start_position: GridPosition,
        range: i32,
    ) -> HashSet<GridPosition> {
        let mut positions: HashSet<GridPosition> = HashSet::new();
        let num_positions = range / 2;

        for _ in 0..num_positions {
            let x = rand::random::<i32>() % range;
            let y = rand::random::<i32>() % range;

            let position = start_position + GridPosition::new(x, y, self.cell_size);

            positions.insert(position);
        }

        positions
    }

    fn draw_spawnbox(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        position: GridPosition,
        radius: f32,
    ) {
        let vec2 = position.as_vec2();

        let width = self.cell_size.width * (radius * 2.0);
        let height = self.cell_size.height * (radius * 2.0);

        let rect = Rect::new(
            vec2.x - (width / 2.0),
            vec2.y - (height / 2.0),
            width,
            height,
        );

        let inner_rectangle = Mesh::new_rectangle(
            &ctx.gfx,
            DrawMode::fill(),
            rect,
            Color::from_rgba(255, 255, 255, 13),
        )
        .unwrap();

        let outline =
            Mesh::new_rectangle(&ctx.gfx, DrawMode::stroke(2.0), rect, Color::WHITE).unwrap();

        inner_rectangle.draw(canvas, DrawParam::default());
        outline.draw(canvas, DrawParam::default());
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(FPS) {
            let cloned_blocks = self.clone_blocks();

            // Unmutable updates
            for (_position, block) in self.blocks.iter_mut() {
                block.apply_gravity(ctx, &cloned_blocks, self.cell_size);
                block.apply_motion(ctx, &cloned_blocks, self.cell_size);
            }

            // Mutable updates
            for (_position, _block) in self.blocks.iter() {}

            let mouse_position = ctx.mouse.position();

            if self.mouse_down && self.selected_block != BlockType::Eraser {
                let grid_position = GridPosition::from_vec2(mouse_position, self.cell_size);
                let positions = self.generate_positions(grid_position, self.place_range);

                for position in positions {
                    self.spawn_block(self.selected_block.clone(), position)?;
                }
            }

            if self.mouse_down && self.selected_block == BlockType::Eraser {
                let grid_position = GridPosition::from_vec2(mouse_position, self.cell_size);
                let positions = self.generate_positions(grid_position, self.place_range);

                for position in positions {
                    self.blocks.remove(&position);
                }
            }

            self.update_positions();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        // Overlap checking ---------------------------------

        let mut position_count: HashMap<GridPosition, u64> = HashMap::new();

        for (_pos, block) in self.blocks.iter_mut() {
            let position = block.get_position();
            let count = position_count.get(&position).unwrap_or(&0) + 1;

            position_count.insert(position, count);
        }

        for (position, count) in position_count {
            if count > 1 {
                println!("Position: {:?} Count: {}", position, count);
            }
        }

        // End overlap checking -----------------------------

        self.draw_grid(ctx, &mut canvas);
        self.draw_fps(ctx, &mut canvas);
        self.draw_pixels(ctx, &mut canvas);

        self.draw_spawnbox(
            ctx,
            &mut canvas,
            GridPosition::from_vec2(ctx.mouse.position(), self.cell_size),
            self.place_range as f32,
        );

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Some(key_code) = input.keycode {
            match key_code {
                KeyCode::Key1 => self.selected_block = BlockType::Sand,
                KeyCode::Key2 => self.selected_block = BlockType::Stone,
                KeyCode::Key3 => self.selected_block = BlockType::Eraser,
                _ => self.selected_block = BlockType::Sand,
            }
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        self.mouse_down = true;

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        self.mouse_down = false;

        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        // println!("Mousewheel event, x: {x}, y: {y}");
        if y > 0.0 {
            self.place_range += 1;
        } else {
            self.place_range -= 1;
        }

        if self.place_range < 1 {
            self.place_range = 1;
        }

        Ok(())
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Sand Simulation v3", "")
        .window_setup(ggez::conf::WindowSetup::default().title("Sand Simulation v3"))
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
