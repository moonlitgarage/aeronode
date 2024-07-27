use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Altimeter {
    pub pressure: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Magnetometer {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Accelerometer {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gyroscope {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sensors {
    altimeter: Option<Altimeter>,
    magnetometer: Option<Magnetometer>,
    accelerometer: Option<Accelerometer>,
    gyroscope: Option<Gyroscope>,
}

impl Sensors {
    pub fn new(
        altimeter: Option<Altimeter>,
        magnetometer: Option<Magnetometer>,
        accelerometer: Option<Accelerometer>,
        gyroscope: Option<Gyroscope>,
    ) -> Self {
        Self { altimeter, magnetometer, accelerometer, gyroscope }
    }

    pub fn with_altimeter(self, altimeter: Altimeter) -> Self {
        Self { altimeter: Some(altimeter), ..self }
    }

    pub fn with_magnetometer(self, magnetometer: Magnetometer) -> Self {
        Self { magnetometer: Some(magnetometer), ..self }
    }

    pub fn with_accelerometer(self, accelerometer: Accelerometer) -> Self {
        Self { accelerometer: Some(accelerometer), ..self }
    }

    pub fn with_gyroscope(self, gyroscope: Gyroscope) -> Self {
        Self { gyroscope: Some(gyroscope), ..self }
    }

    pub fn altimeter(&self) -> Option<Altimeter> {
        self.altimeter
    }

    pub fn magnetometer(&self) -> Option<Magnetometer> {
        self.magnetometer.clone()
    }

    pub fn accelerometer(&self) -> Option<Accelerometer> {
        self.accelerometer.clone()
    }

    pub fn gyroscope(&self) -> Option<Gyroscope> {
        self.gyroscope.clone()
    }
}