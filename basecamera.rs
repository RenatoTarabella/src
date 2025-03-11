use crate::vector3::Vector3;
use crate::baseray::BaseRay;
use crate::baseobject::BaseObject;
use rand::Rng;
// test 
pub struct BaseCamera {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub _aperture: f32,
    pub focus_dist: f32,
   
    // Campi calcolati
    direction: Vector3,
    right: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    lens_radius: f32,
}

impl BaseCamera {
    pub fn new(
        position: Vector3,
        target: Vector3,
        up: Vector3,
        fov: f32,
        aspect_ratio: f32,
        _aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let direction = (target - position).normalize();
        let right = direction.cross(&up).normalize();
        let up = right.cross(&direction);  // Ricalcola up per assicurare l'ortogonalità

        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let horizontal = right * (focus_dist * viewport_width);
        let vertical = up * (focus_dist * viewport_height);
        let lower_left_corner = position + direction * focus_dist - horizontal/2.0 - vertical/2.0;

        let lens_radius = _aperture / 2.0;

        BaseCamera {
            position,
            target,
            up,
            fov,
            aspect_ratio,
            _aperture,
            focus_dist,
            direction,
            right,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
        }
    }


    pub fn center_object(&mut self, dolly_in: f32, obj: &BaseObject) {
        if let Some(bbox) = &obj.boundingbox {
            let bbox_center = bbox.center;
    
            self.target = bbox_center;
    
            let bbox_size = bbox.max - bbox.min;
            let max_dimension = bbox_size.x.max(bbox_size.y).max(bbox_size.z);
    
            // Calcola l'angolo di visione verticale
            let vertical_fov = self.fov.to_radians();
            let distance = (max_dimension / 2.0) / (vertical_fov / 2.0).tan();
    
            let scale_factor = 0.8; // Scala regolabile per assicurare la completa visibilità dell'oggetto
    
            // Inverti la direzione per guardare verso l'oggetto
            self.direction = (self.position - bbox_center).normalize(); // Inverti qui la direzione
    
            // Calcola la nuova posizione considerando la direzione invertita
            self.position += self.direction * (distance * scale_factor ); // - dolly_in
            self.position.y += 50.0;
            // self.position.z = -self.position.z; // Inverti la posizione lungo l'asse z
            // self.direction = -self.direction; // Inverti la direzione
    
            // Aggiornamento della distanza di messa a fuoco
            self.focus_dist = distance * scale_factor - dolly_in;
    
            // Ricalcola i vettori della camera in base alla nuova direzione
            self.update_camera_vectors();
        }
    }
 
    fn update_camera_vectors(&mut self) {
        self.direction = (self.target - self.position).normalize(); // Assicura che la direzione sia aggiornata
        self.right = self.direction.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize();
        self.up = self.right.cross(&self.direction).normalize();
    
        // Calcolo dei vettori per definire il frustum della camera
        let half_height = (self.fov.to_radians() / 2.0).tan();
        let half_width = half_height * self.aspect_ratio;
        self.horizontal = self.right * (2.0 * half_width * self.focus_dist);
        self.vertical = self.up * (2.0 * half_height * self.focus_dist);
        self.lower_left_corner = self.position - self.horizontal / 2.0 - self.vertical / 2.0 - self.direction * self.focus_dist;
    }


    pub fn random_in_unit_disk() -> Vector3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vector3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> BaseRay {
        let rd = BaseCamera::random_in_unit_disk() * self.lens_radius;
        let offset = self.right * rd.x + self.up * rd.y;
    
        let direction = self.lower_left_corner + self.horizontal * s + self.vertical * t - self.position - offset;
        BaseRay::new(
            self.position + offset,
            direction.normalize()  // Normalizza la direzione del raggio
        )
    }
    
    // pub fn get_ray(&self, s: f32, t: f32) -> BaseRay {
    //     let rd = BaseCamera::random_in_unit_disk() * self.lens_radius ;
    //     let offset = self.right * rd.x + self.up * rd.y;

    //     BaseRay::new(
    //         self.position + offset,
    //         self.lower_left_corner + self.horizontal * s + self.vertical * t - self.position - offset
    //     )
    // }
}