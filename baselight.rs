use crate::vector3::Vector3;
use std::ops::AddAssign;
use std::ops::DivAssign;

pub struct AGColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl AGColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        AGColor { r, g, b }
    }
}

impl DivAssign<f32> for AGColor {
    fn div_assign(&mut self, scalar: f32) {
        self.r /= scalar;
        self.g /= scalar;
        self.b /= scalar;
    }
}

impl AddAssign for AGColor {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

pub enum LightType {
    Spot,
    Point,
    Directional,
    Area,
}

pub enum FalloffType {
    None,
    Linear,
    Quadratic,
}

pub struct BaseLight {
    pub name: String,
    pub position: Vector3,
    pub direction: Vector3,
    pub color: AGColor,
    pub intensity: f32,
    pub light_type: LightType,
    pub falloff: FalloffType,
    pub spot_angle: f32,
    pub inner_radius: f32,
    pub radius_decay: f32,
    pub area_size: (f32, f32),
}

impl BaseLight {
    pub fn new(name: String, position: Vector3, direction: Vector3, color: AGColor, intensity: f32, light_type: LightType, falloff: FalloffType, spot_angle: f32, inner_radius: f32, radius_decay: f32, area_size: (f32, f32)) -> Self {
        BaseLight {
            name,
            position,
            direction,
            color,
            intensity,
            light_type,
            falloff,
            spot_angle,
            inner_radius,
            radius_decay,
            area_size,
        }
    }
}
