pub struct SceneSettings {
    pub quality_preset: String,
    pub aa_threshold: f32,
    pub max_samples_aa: u32,
    pub ao_enabled: bool,
    pub max_samples_ao: u32,
    pub ao_mult: i32,
    pub shadows_enabled: bool,
    pub max_samples_light: u32,
    pub shadow_mult: i32,
    pub dolly_in: f32,
    pub field_of_view: f32,
    pub bucket_order: String,
    pub bucket_count: u32,
    pub rot_hor_camera: f32,
    pub rot_vert_camera: f32,
}

impl SceneSettings { 
    pub fn new() -> Self {
        SceneSettings {
            quality_preset: "AG Low Quality".to_string(),
            ao_enabled: true,
            shadows_enabled: true,
            aa_threshold: 0.1,
            max_samples_aa: 3,
            max_samples_ao: 16,
            max_samples_light: 32,
            ao_mult: 2,
            shadow_mult: 1,
            dolly_in: 440.0,
            field_of_view: 10.0,
            bucket_order: "CENTRAL".to_string(),
            bucket_count: 75,
            rot_hor_camera: 0.0,
            rot_vert_camera: 0.0,
        }
    }
}
