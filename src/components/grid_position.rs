use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Mul},
};

use euclid::num::Round;
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::glam::*;
use ggez::graphics::{self, Canvas, Color};
use ggez::{Context, ContextBuilder, GameResult};

use crate::blocks::block::Block;

use super::cell_size::CellSize;

// pub fn round_to_multiple(number: i32, cell_size: CellSize) -> i32 {
//     (number / cell_size).round() * cell_size
// }

#[derive(Debug, Clone, Copy)]
pub struct GridPosition {
    pub cell_size: CellSize,
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32, cell_size: CellSize) -> Self {
        let x = x * cell_size.width as i32;
        let y = y * cell_size.height as i32;

        Self { cell_size, x, y }
    }

    pub fn from_vec2<T>(point: T, cell_size: CellSize) -> Self
    where
        T: Into<Vec2>,
    {
        let point: Vec2 = point.into();

        let x = (point.x / cell_size.width as f32).round() as i32;
        let y = (point.y / cell_size.height as f32).round() as i32;

        Self::new(x, y, cell_size)
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn as_rect(&self) -> graphics::Rect {
        graphics::Rect::new(
            self.x as f32,
            self.y as f32,
            self.cell_size.width,
            self.cell_size.height,
        )
    }

    pub fn is_occupied(
        &self,
        blocks: &HashSet<GridPosition>,
    ) -> bool {
        blocks.contains(self)
    }

    pub fn is_offscreen(&self, ctx: &mut ggez::Context) -> bool {
        let window_size = ctx.gfx.drawable_size();

        let top = 0.0;
        let left = 0.0;
        let bottom = window_size.1 - self.cell_size.height;
        let right = window_size.0 - self.cell_size.width;

        let position = self.as_vec2();

        return position.x < left
            || position.x > right
            || position.y < top
            || position.y > bottom;
    }
}

impl PartialEq for GridPosition {
    fn eq(&self, other: &Self) -> bool {
        self.cell_size == other.cell_size
            && self.x == other.x
            && self.y == other.y
    }
}

impl AddAssign for GridPosition {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for GridPosition {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cell_size: self.cell_size,
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for GridPosition {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            cell_size: self.cell_size,
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Eq for GridPosition {}

impl Hash for GridPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cell_size.width.to_bits().hash(state);
        self.cell_size.height.to_bits().hash(state);
        self.x.hash(state);
        self.y.hash(state);
    }
}
