use glam::Vec3;  // Nota: usiamo Vec3 invece di DVec3
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg, Index, IndexMut};
use std::hash::{Hash, Hasher};
use rand::Rng;

// prova

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
} 

impl Vector3 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.to_vec3().dot(other.to_vec3())
    }

    #[inline(always)]
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        let result = self.to_vec3().cross(other.to_vec3());
        Vector3::from_vec3(result)
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.to_vec3().length()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vector3 {
        let result = self.to_vec3().normalize();
        Vector3::from_vec3(result)
    }

    #[inline(always)]
    pub fn length_squared(&self) -> f32 {
        self.to_vec3().length_squared()
    }

    // #[inline(always)]
    // pub fn reflect(&self, normal: &Vector3) -> Vector3 {
    //     let result = self.to_vec3().reflect(normal.to_vec3());
    //     Vector3::from_vec3(result)
    // }

    #[inline(always)]
    pub fn min(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    #[inline(always)]
    pub fn max(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    #[inline]
    pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let mut rng = rand::thread_rng();
        let random_vector = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0)
        ).normalize();
        
        if random_vector.dot(normal) > 0.0 {
            random_vector
        } else {
            -random_vector
        }
    }

    #[inline]
    pub fn random_in_unit_sphere() -> Vector3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vector3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0)
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline(always)]
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    #[inline(always)]
    fn from_vec3(v: Vec3) -> Self {
        Vector3 { x: v.x, y: v.y, z: v.z }
    }
}

// Le implementazioni per le operazioni rimangono simili, ma usiamo f32

impl Add for Vector3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, scalar: f32) -> Self {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, scalar: f32) -> Self {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// Implementazioni per operatori di assegnazione
impl AddAssign for Vector3 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vector3 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f32> for Vector3 {
    #[inline(always)]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign<f32> for Vector3 {
    #[inline(always)]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vector3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Hash for Vector3 {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x = (self.x * 1000.0) as i32;
        let y = (self.y * 1000.0) as i32;
        let z = (self.z * 1000.0) as i32;
        x.hash(state);
        y.hash(state);
        z.hash(state);
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}