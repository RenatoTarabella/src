use crate::vector3::Vector3;

#[derive(Clone, Debug)]
pub struct Boundingbox {
    pub min: Vector3,
    pub max: Vector3,
    pub center: Vector3,
}

impl Boundingbox {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        let center = Boundingbox::calculate_center(&min, &max);
        Boundingbox { min, max, center }
    }

    pub fn diagonal(&self) -> f32 {
        (self.max - self.min).length()
    }

    pub fn new_empty() -> Self {
        Boundingbox {
            min: Vector3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vector3::new(f32::MIN, f32::MIN, f32::MIN),
            center: Vector3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn get(&self, index: usize) -> &Vector3 {
        match index {
            0 => &self.min,
            1 => &self.max,
            _ => panic!("Index out of bounds for Boundingbox"),
        }
    }

    pub fn expand(&mut self, other: &Boundingbox) {
        self.min.x = self.min.x.min(other.min.x);
        self.min.y = self.min.y.min(other.min.y);
        self.min.z = self.min.z.min(other.min.z);

        self.max.x = self.max.x.max(other.max.x);
        self.max.y = self.max.y.max(other.max.y);
        self.max.z = self.max.z.max(other.max.z);

        self.update_center();
    }

    fn update_center(&mut self) {
        self.center = Vector3::new(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
            (self.min.z + self.max.z) * 0.5,
        );
    }

    pub fn from_triangle(a: Vector3, b: Vector3, c: Vector3) -> Boundingbox {
        let min = Vector3::new(
            a.x.min(b.x.min(c.x)),
            a.y.min(b.y.min(c.y)),
            a.z.min(b.z.min(c.z)),
        );
        let max = Vector3::new(
            a.x.max(b.x.max(c.x)),
            a.y.max(b.y.max(c.y)),
            a.z.max(b.z.max(c.z)),
        );
        let center = Boundingbox::calculate_center(&min, &max);

        Boundingbox { min, max, center }
    }

    pub fn union(&self, other: &Boundingbox) -> Boundingbox {
        let min = Vector3::new(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );
        let max = Vector3::new(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z),
        );
        let center = Boundingbox::calculate_center(&min, &max);

        Boundingbox { min, max, center }
    }

    fn calculate_center(min: &Vector3, max: &Vector3) -> Vector3 {
        Vector3::new(
            (min.x + max.x) / 2.0,
            (min.y + max.y) / 2.0,
            (min.z + max.z) / 2.0,
        )
    }
}

// Remove the manual implementation of Clone for Boundingbox