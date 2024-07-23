use serde::{Deserialize, Serialize};
use crate::data::commons::Vec3d;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sensors {
    altimeter: Option<f32>,
    magnetometer: Option<Vec3d>,
    accelerometer: Option<Vec3d>,
    gyroscope: Option<Vec3d>,
}

impl Sensors {
    pub fn new(
        altimeter: Option<f32>,
        magnetometer: Option<Vec3d>,
        accelerometer: Option<Vec3d>,
        gyroscope: Option<Vec3d>,
    ) -> Self {
        Self { altimeter, magnetometer, accelerometer, gyroscope }
    }

    pub fn with_altimeter(self, altimeter: f32) -> Self {
        Self { altimeter: Some(altimeter), ..self }
    }

    pub fn with_magnetometer(self, magnetometer: Vec3d) -> Self {
        Self { magnetometer: Some(magnetometer), ..self }
    }

    pub fn with_accelerometer(self, accelerometer: Vec3d) -> Self {
        Self { accelerometer: Some(accelerometer), ..self }
    }

    pub fn with_gyroscope(self, gyroscope: Vec3d) -> Self {
        Self { gyroscope: Some(gyroscope), ..self }
    }

    pub fn altimeter(&self) -> Option<f32> {
        self.altimeter
    }

    pub fn magnetometer(&self) -> Option<Vec3d> {
        self.magnetometer.clone()
    }

    pub fn accelerometer(&self) -> Option<Vec3d> {
        self.accelerometer.clone()
    }

    pub fn gyroscope(&self) -> Option<Vec3d> {
        self.gyroscope.clone()
    }
}