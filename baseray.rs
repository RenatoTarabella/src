use crate::vector3::Vector3;
use crate::boundingbox::Boundingbox;
use crate::baseobject::Triangle;
use crate::baseobject::BaseObject;

#[derive(Clone, Debug)]
pub struct BaseRay {
    pub origin: Vector3,
    pub direction: Vector3,
    pub inv_direction: Vector3,
    pub sign: [usize; 3],
}

impl BaseRay {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        let inv_direction = Vector3::new(
            1.0 / direction.x,
            1.0 / direction.y,
            1.0 / direction.z,
        );
        let sign = [
            (inv_direction.x < 0.0) as usize,
            (inv_direction.y < 0.0) as usize,
            (inv_direction.z < 0.0) as usize,
        ];
        BaseRay {
            origin,
            direction: direction.normalize(),
            inv_direction,
            sign,
        }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }


    pub fn intersects(&self, bbox: &Boundingbox) -> Option<(f32, f32)> {
        let inv_d = Vector3::new(1.0 / self.direction.x, 1.0 / self.direction.y, 1.0 / self.direction.z);
        let t0s = (bbox.min - self.origin) * inv_d;
        let t1s = (bbox.max - self.origin) * inv_d;
        
        let tmin = t0s.min(&t1s);
        let tmax = t0s.max(&t1s);
        
        let t_min = tmin.x.max(tmin.y).max(tmin.z);
        let t_max = tmax.x.min(tmax.y).min(tmax.z);
        
        if t_max >= t_min && t_max >= 0.0 {
            Some((t_min as f32, t_max as f32))
        } else {
            None
        }
    }

    pub fn intersects_triangle(&self, triangle: &Triangle, base_object: &BaseObject) -> Option<f32> {
        const EPSILON: f32 = 1e-8;

        let a = &base_object.padr[triangle.a];
        let b = &base_object.padr[triangle.b];
        let c = &base_object.padr[triangle.c];
    
        let edge1 = *b - *a;
        let edge2 = *c - *a;
        let h = self.direction.cross(&edge2);
        let a_dot = edge1.dot(&h);
    
        if a_dot > -EPSILON && a_dot < EPSILON {
            return None;  // Il raggio è parallelo al triangolo
        }
    
        let f = 1.0 / a_dot;
        let s = self.origin - *a;  // Questa è la sottrazione corretta tra due Vector3
        let u = f * s.dot(&h);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * self.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > EPSILON {
            Some(t)
        } else {
            None
        }
    }
}