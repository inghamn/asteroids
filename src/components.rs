use specs::prelude::*;
use sdl2::rect::Rect;

pub struct Position {
    pub   x: i32,
    pub   y: i32,
    pub   w: u32,
    pub   h: u32,
    pub dir: f64
}

pub struct Velocity {
    pub x: i8,
    pub y: i8
}

pub struct Sprite {
    pub frame: Rect
}

impl Component for Position { type Storage = VecStorage<Self>; }
impl Component for Velocity { type Storage = VecStorage<Self>; }
impl Component for Sprite   { type Storage = VecStorage<Self>; }
