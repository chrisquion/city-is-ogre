use serde::{Deserialize, Serialize};

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

pub struct Time(pub f32);

impl Time {
    pub fn let_there_be_time() -> Time {
        Time(0.0)
    }
}