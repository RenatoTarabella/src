// Amtrix struct definition --------------------------------------------
// mod vector3;
use crate::vector3::Vector3;



#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    pub off: Vector3,  // Offset translation vector
    pub v1: Vector3,   // First column vector
    pub v2: Vector3,   // Second column vector
    pub v3: Vector3,   // Third column vector
} 

impl Matrix {
    pub fn new(off: Vector3, v1: Vector3, v2: Vector3, v3: Vector3) -> Self {
        Matrix { off, v1, v2, v3 }
    }

    pub fn identity() -> Self {
        Matrix {
            off: Vector3::zero(),
            v1: Vector3::new(1.0, 0.0, 0.0),
            v2: Vector3::new(0.0, 1.0, 0.0),
            v3: Vector3::new(0.0, 0.0, 1.0),
        }
    }

    // Moltiplicazione della matrice per un Vector3
    pub fn transform(&self, vec: Vector3) -> Vector3 {
        Vector3 {
            x: self.v1.x * vec.x + self.v2.x * vec.y + self.v3.x * vec.z + self.off.x,
            y: self.v1.y * vec.x + self.v2.y * vec.y + self.v3.y * vec.z + self.off.y,
            z: self.v1.z * vec.x + self.v2.z * vec.y + self.v3.z * vec.z + self.off.z,
        }
    }
}