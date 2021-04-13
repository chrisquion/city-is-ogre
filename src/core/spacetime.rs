use serde::Deserialize;

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

pub struct Time(pub f32);
