use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
