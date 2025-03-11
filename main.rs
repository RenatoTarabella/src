mod vector3;
mod boundingbox;
mod baseray;
mod baseobject;
mod matrix;
mod bvhnode;
mod scenesettings;
mod baselight;
mod basecamera;
mod bucket;

use crossbeam::thread;
use std::sync::Arc;

use crate::bucket::BucketState;
use crate::bucket::{Bucket, Rect};
use iced::{
    Color,
    container, Background, Element, Length, 
    Application, Button, Column, Command, Container, 
    Image, Settings, Slider, Text, button, slider, 
    image::Handle
};

use rayon::prelude::*;

use crate::basecamera::BaseCamera;
use crate::baselight::AGColor;
use crate::baselight::LightType;
use crate::baselight::FalloffType;
use baseobject::BaseObject;
use image::{ImageBuffer, Rgba};
use vector3::Vector3;
use std::time::Instant;
use rand::random;
use crate::scenesettings::SceneSettings;
use crate::baselight::BaseLight;
use crate::baseray::BaseRay;

use rayon::prelude::*;
// use std::sync::Arc;

const SCREEN_WIDTH: u32 = 1080;
const SCREEN_HEIGHT: u32 = 1080;

fn main() -> iced::Result {
    RustRender::run(Settings::default())
}
struct RustRender {
    render_image: Handle,
    generate_button: button::State,
    slider: slider::State,
    slider_value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Generate,
    SliderChanged(f32),
}

struct CustomStyle;

impl container::StyleSheet for CustomStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(
                0.2, 0.2, 0.2, // Sostituisci con i valori RGB che preferisci
            ))),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

impl Application for RustRender {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (RustRender, Command<Self::Message>) {
        let render_image = setup_scene();
        (
            RustRender {
                render_image: Handle::from_pixels(SCREEN_WIDTH, SCREEN_HEIGHT, render_image),
                generate_button: button::State::new(),
                slider: slider::State::new(),
                slider_value: 50.0, // valore iniziale dello slider
            },
            Command::none(),
        )
    }    

    fn title(&self) -> String {
        String::from("AGRay 2024 - Rust version")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Generate => {
                let render_image = setup_scene();
                self.render_image = Handle::from_pixels(SCREEN_WIDTH, SCREEN_HEIGHT, render_image);
            },
            Message::SliderChanged(value) => {
                self.slider_value = value;
            }
        }
        Command::none()
    }
    

    fn view(&mut self) -> Element<'_, Self::Message> {
        // Definizione del bottone
        let button = Button::new(&mut self.generate_button, Text::new("Generate"))
            .on_press(Message::Generate);
    
        // Definizione dello slider
        let slider = Slider::new(
            &mut self.slider,
            0.0..=100.0,
            self.slider_value,
            Message::SliderChanged
        );
    
        // Creazione del contenuto della colonna
        let content = Column::new()
            .spacing(10)
            .push(Image::new(self.render_image.clone()).width(Length::Fill).height(Length::Fill))
            .push(button)
            .push(slider);
    
        // Container che ingloba il contenuto
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(CustomStyle) // Assicurati che CustomStyle sia definito correttamente da qualche parte
            .into()
    }        
}

fn setup_scene() -> Vec<u8> {
    let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
    let imgbuf = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    // -------------------------------------------------------------------------------------------------------------------------

    // Init Lights ----------------------------------------------------
    let luce1 = BaseLight::new(
        "Light.2_Spot".to_string(),
        Vector3::new(-259.95, 518.74, 310.19),
        Vector3::new(0.39509776, -0.7884324, -0.47145748),
        AGColor::new(0.424, 0.536, 0.851),
        1.254,
        LightType::Spot,
        FalloffType::None,
        28.8,
        0.0,
        0.0,
        (72.83, 72.83),
    );
    
    let luce2 = BaseLight::new(
        "Light.6".to_string(),
        Vector3::new(-355.24, -47.73, -221.27),
        Vector3::new(0.84334135, 0.11331124, 0.525296),
        AGColor::new(0.98, 0.96, 0.94),
        0.582,
        LightType::Spot,
        FalloffType::Linear,
        28.8,
        0.0,
        518.0,
        (40.068222, 38.55295),
    );
    
    let luce3 = BaseLight::new(
        "Light.4".to_string(),
        Vector3::new(248.92, 322.63, 227.43),
        Vector3::new(-0.53340256, -0.6913534, -0.48735234),
        AGColor::new(0.567, 0.797, 1.042),
        1.088,
        LightType::Area,
        FalloffType::None,
        0.0,
        0.0,
        0.0,
        (72.83, 72.83),
    );
    
    let luce4 = BaseLight::new(
        "Light.1".to_string(),
        Vector3::new(284.31, 297.96, -348.31),
        Vector3::new(-0.5271039, -0.55241066, 0.6457584),
        AGColor::new(0.98, 0.96, 0.94),
        0.567,
        LightType::Area,
        FalloffType::None,
        0.0,
        0.0,
        0.0,
        (72.83, 72.83),
    );
    
    let luce5 = BaseLight::new(
        "Light".to_string(),
        Vector3::new(195.36, -171.45, -294.36),
        Vector3::new(-0.4974868, 0.43659967, 0.74959165),
        AGColor::new(1.0, 0.336, 0.084),
        0.199,
        LightType::Area,
        FalloffType::None,
        0.0,
        0.0,
        0.0,
        (72.83, 72.83),
    );
    
    let luce6 = BaseLight::new(
        "Light.5".to_string(),
        Vector3::new(-320.85, 120.83, -300.78),
        Vector3::new(0.7034878, -0.26492888, 0.6594828),
        AGColor::new(0.98, 0.96, 0.94),
        0.12,
        LightType::Area,
        FalloffType::None,
        0.0,
        0.0,
        0.0,
        (72.83, 72.83),
    ); 
    // end of Init Lights ----------------------------------------------   

    // add lights to Vec. ----------------------------------------------
    let mut lights = Vec::new();
    lights.push(luce1);
    lights.push(luce2);
    lights.push(luce3);
    lights.push(luce4);
    lights.push(luce5);
    lights.push(luce6);    
    // ------------------------------------------------------------------

    // Init Camera -----------------------------------------------------
    let mut camera = BaseCamera::new(
        Vector3::new(20.0, 20.0, -100.),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),    
        26.0,
        (SCREEN_WIDTH as f32) / (SCREEN_HEIGHT as f32),
        0.1,
        1000.0,
    );

    let scene_settings = SceneSettings::new();

    // Generate Buckets ------------------------------------------------
    let rect = Rect {
        x: 0,
        y: 0,
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
    };
    let buckets = generate_buckets(SCREEN_WIDTH, SCREEN_HEIGHT, rect, scene_settings.bucket_count);

    println!("Iniziando il caricamento del file STL...");
    let start_load = Instant::now();
    let mut obj = BaseObject::load_stl("C:\\Users\\renat\\Desktop\\Behemot Rider A Alone.stl").unwrap();
    println!("Caricamento STL completato in {:?}", start_load.elapsed());

    println!("Iniziando la costruzione del BVH...");
    let start_bvh = Instant::now();
    obj.build_bvh();
    println!("Costruzione BVH completata in {:?}", start_bvh.elapsed());

    BaseCamera::center_object(&mut camera, scene_settings.dolly_in as f32, &mut obj);

    // Renderizza l'immagine    
    let camera = Arc::new(camera);
    let obj = Arc::new(obj);    
    let lights_vec: Vec<BaseLight> = lights; // Supponiamo che `lights` sia già un `Vec<BaseLight>`
    let lights_slice: Box<[BaseLight]> = lights_vec.into_boxed_slice(); // Converti il Vec in Boxed Slice
    let lights_arc: Arc<[BaseLight]> = Arc::from(lights_slice); // Converti il Boxed Slice in Arc
    
    let scene_settings = Arc::new(SceneSettings::new());
    let num_threads = 31; // Usa tutti i core disponibili
    let start_rendering = Instant::now();

    println!("Iniziando il rendering");
    // let image_data = render(camera, obj, lights_arc, scene_settings, num_threads);
    let image_data: Vec<u8> = render_stoacastic(camera, obj, lights_arc, scene_settings, num_threads);
    
    println!("Rendering completato in {:?}", start_rendering.elapsed());

    // Salva l'immagine
    image::save_buffer("output.png", &image_data, SCREEN_WIDTH, SCREEN_HEIGHT, image::ColorType::Rgba8)
    .expect("Impossibile salvare l'immagine");

    println!("Immagine salvata come 'output.png'");
    image_data  
}
 

const BUCKET_SIZE: u32 = 32; // Puoi regolare questa dimensione

#[macro_use]
extern crate lazy_static;


pub fn render(
    camera: Arc<BaseCamera>,
    obj: Arc<BaseObject>,
    lights: Arc<[BaseLight]>,
    scene_settings: Arc<SceneSettings>,
    num_threads: usize
) -> Vec<u8> {
    let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
    let grid_size = scene_settings.max_samples_aa;

    // Configura il pool di thread
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    // Calcola il numero di bucket
    let num_buckets_x = (width + BUCKET_SIZE - 1) / BUCKET_SIZE;
    let num_buckets_y = (height + BUCKET_SIZE - 1) / BUCKET_SIZE;
    
    // Crea una sequenza di bucket dal centro verso l'esterno
    let center_x = num_buckets_x / 2;
    let center_y = num_buckets_y / 2;
    let mut bucket_sequence: Vec<(i32, i32)> = Vec::new();
    let max_distance = max(center_x, center_y) as i32;
    
    for distance in 0..=max_distance {
        for dy in -distance..=distance {
            for dx in -distance..=distance {
                if dx.abs() == distance || dy.abs() == distance {
                    let bx = center_x as i32 + dx;
                    let by = center_y as i32 + dy;
                    if bx >= 0 && by >= 0 && bx < num_buckets_x as i32 && by < num_buckets_y as i32 {
                        bucket_sequence.push((bx, by));
                    }
                }
            }
        }
    }

    // Renderizza i bucket in parallelo
    let pixels: Vec<Rgba<u8>> = bucket_sequence
        .into_par_iter()
        .flat_map(|(bx, by)| {
            let bucket_x = bx as u32 * BUCKET_SIZE;
            let bucket_y = by as u32 * BUCKET_SIZE;
            let mut bucket_pixels = Vec::with_capacity((BUCKET_SIZE * BUCKET_SIZE) as usize);

            for y in bucket_y..min(bucket_y + BUCKET_SIZE, height) {
                for x in bucket_x..min(bucket_x + BUCKET_SIZE, width) {
                    let mut color = AGColor::new(0.0, 0.0, 0.0);
                    for sub_x in 0..grid_size {
                        for sub_y in 0..grid_size {
                            let u_offset = (sub_x as f32 + 0.5) / grid_size as f32;
                            let v_offset = (sub_y as f32 + 0.5) / grid_size as f32;
                            let u = (x as f32 + u_offset) / (width - 1) as f32;
                            let v = (y as f32 + v_offset) / (height - 1) as f32;
                            let ray = camera.get_ray(u, v);
                            color += trace_ray(&ray, &obj, &lights, &scene_settings, 0);
                        }
                    }
                    // Media dei colori dei sottopixel
                    color /= (grid_size * grid_size) as f32;
                    bucket_pixels.push(Rgba([
                        (color.r * 255.0) as u8,
                        (color.g * 255.0) as u8,
                        (color.b * 255.0) as u8,
                        255
                    ]));
                }
            }
            bucket_pixels
        })
        .collect();

    // Converti il vettore di pixel in un vettore di byte raw
    pixels.into_iter().flat_map(|p| p.0.to_vec()).collect()
}





fn render_old_new(
    camera: Arc<BaseCamera>,
    obj: Arc<BaseObject>,
    lights: Arc<[BaseLight]>,
    scene_settings: Arc<SceneSettings>,
    num_threads: usize
) -> Vec<u8> {
    let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
    let grid_size = scene_settings.max_samples_aa;
    let num_pixels = width * height;

    // Configura il pool di thread globale
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    // Renderizza i pixel in parallelo
    let pixels: Vec<Rgba<u8>> = (0..num_pixels)
        .into_par_iter()
        .map(|i| {
            let x = i % width;
            let y = i / width;
            let mut color = AGColor::new(0.0, 0.0, 0.0);

            for sub_x in 0..grid_size {
                for sub_y in 0..grid_size {
                    let u_offset = (sub_x as f32 + 0.5) / grid_size as f32;
                    let v_offset = (sub_y as f32 + 0.5) / grid_size as f32;
                    let u = (x as f32 + u_offset) / (width - 1) as f32;
                    let v = (y as f32 + v_offset) / (height - 1) as f32;
                    let ray = camera.get_ray(u, v);
                    color += trace_ray(&ray, &obj, &lights, &scene_settings, 0);
                }
            }

            // Media dei colori dei sottopixel
            color /= (grid_size * grid_size) as f32;
            Rgba([
                (color.b * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.r * 255.0) as u8,
                255
            ])
        })
        .collect();

    // Converti il vettore di pixel in un vettore di byte raw
    pixels.into_iter().flat_map(|p| p.0.to_vec()).collect()
}

fn render_old(
        camera: Arc<BaseCamera>,
        obj: Arc<BaseObject>,
        lights: Arc<[BaseLight]>,
        scene_settings: Arc<SceneSettings>,
        num_threads: usize
    ) -> Vec<u8> {
    let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
    let num_pixels = width * height;
    let grid_size = scene_settings.max_samples_aa;// scene_settings.grid_size;  // Assumi che questa impostazione esista e sia, per esempio, 2 per 2x2, 3 per 3x3, ecc.

    // Configura il numero di thread
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
        
    // Crea un vettore di pixel in parallelo
    // Invece di mappare direttamente i pixel, li inverto nel processo di conversione
    // Crea un vettore di pixel in parallelo
    let pixels: Vec<Rgba<u8>> = (0..num_pixels)
    .into_par_iter()
    .map(|i| {
        let x = i % width;
        let y = i / width;
        let mut color = AGColor::new(0.0, 0.0, 0.0);

        for sub_x in 0..grid_size {
            for sub_y in 0..grid_size {
                let u_offset = (sub_x as f32 + 0.5) / grid_size as f32;
                let v_offset = (sub_y as f32 + 0.5) / grid_size as f32;

                let u = (x as f32 + u_offset) / (width - 1) as f32;
                let v = (y as f32 + v_offset) / (height - 1) as f32;

                let ray = camera.get_ray(u, v);
                color += trace_ray(&ray, &obj, &lights, &scene_settings, 0);
            }
        }

        // Media dei colori dei sottopixel
        color /= (grid_size * grid_size) as f32;

        Rgba([
            (color.b * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.r * 255.0) as u8,
            255
        ])
    })
    .collect();
        // Converti il vettore di pixel in un vettore di byte raw
        pixels.into_iter().flat_map(|p| p.0.to_vec()).collect()
    }



fn render_stoacastic(
    camera: Arc<BaseCamera>,
    obj: Arc<BaseObject>,
    lights: Arc<[BaseLight]>,
    scene_settings: Arc<SceneSettings>,
    num_threads: usize
) -> Vec<u8> {
    let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
    let num_pixels = width * height;
    let samples_aa = scene_settings.max_samples_aa;

    // Configura il numero di thread
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    // Crea un vettore di pixel in parallelo
    let pixels: Vec<Rgba<u8>> = (0..num_pixels)
        .into_par_iter()
        .map(|i| {
            let x = i % width;
            let y = i / width;

            let mut color = AGColor::new(0.0, 0.0, 0.0);

            for _ in 0..samples_aa {
                let random_offset_x = rand::random::<f32>() / (width as f32);
                let random_offset_y = rand::random::<f32>() / (height as f32);
                let u = (x as f32 + random_offset_x) / (width - 1) as f32;
                let v = (y as f32 + random_offset_y) / (height - 1) as f32;

                let ray = camera.get_ray(u, v);
                color += trace_ray(&ray, &obj, &lights, &scene_settings, 0);
            }

            color /= samples_aa as f32; // Media dei colori ottenuti

            Rgba([
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8,
                255
            ])
        })
        .collect();

    // Converti il vettore di pixel in un vettore di byte raw
    pixels.into_iter().flat_map(|p| p.0.to_vec()).collect()
}

// Funzione per tracciare un raggio e calcolare il colore del pixel
fn trace_ray(
    ray: &BaseRay,
    obj: &BaseObject,
    lights: &[BaseLight],
    scene_settings: &SceneSettings,
    depth: u32,
) -> AGColor {
    if depth > 5 {  // Limite di profondità per evitare ricorsione infinita
        return AGColor::new(0.0, 0.0, 0.0);
    }
    if let Some((distance, triangle_index)) = obj.bvh_root.as_ref().and_then(|bvh_node| bvh_node.find_nearest_intersection(ray, obj)) {
        let hit_point = ray.origin + ray.direction * distance;
        let normal = obj.norm[triangle_index];
        let mut color = AGColor::new(0.0, 0.0, 0.0);
        let diffuse_color = AGColor::new(0.75, 0.75, 0.75);

        for light in lights {
            match light.light_type {
                LightType::Area => {
                    let (width, height) = light.area_size;
                    let samples = scene_settings.max_samples_light;
                    let mut area_color = AGColor::new(0.0, 0.0, 0.0);

                    for _ in 0..samples {
                        let random_x = random::<f32>() - 0.5;
                        let random_y = random::<f32>() - 0.5;
                        let sample_pos = light.position + 
                            light.direction.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize() * ((random_x * width) as f32) +
                            light.direction.cross(&Vector3::new(1.0, 0.0, 0.0)).normalize() * ((random_y * height) as f32);
                        
                        let light_dir = (sample_pos - hit_point).normalize();
                        let light_distance = (sample_pos - hit_point).length();

                        if scene_settings.shadows_enabled {
                            let shadow_ray = BaseRay::new(hit_point + normal * 0.001, light_dir);
                            if let Some((shadow_dist, _)) = obj.bvh_root.as_ref().and_then(|bvh_node| bvh_node.find_nearest_intersection(&shadow_ray, obj)) {
                                if shadow_dist < light_distance {
                                    continue;  // Punto in ombra
                                }
                            }
                        }

                        let diff = normal.dot(&light_dir).max(0.0) as f32;
                        area_color = add_colors(&area_color, &multiply_color_scalar(&multiply_colors(&diffuse_color, &light.color), diff * light.intensity));

                        let reflect_dir = reflect(-light_dir, normal);
                        let spec = ray.direction.dot(&reflect_dir).max(0.0).powf(32.0) as f32;
                        area_color = add_colors(&area_color, &multiply_color_scalar(&light.color, spec * light.intensity * 0.5));
                    }
                    color = add_colors(&color, &multiply_color_scalar(&area_color, 1.0 / samples as f32));
                },
                _ => {
                    let light_dir = (light.position - hit_point).normalize();
                    let light_distance = (light.position - hit_point).length();

                    if scene_settings.shadows_enabled {
                        let shadow_ray = BaseRay::new(hit_point + normal * 0.001, light_dir);
                        if let Some((shadow_dist, _)) = obj.bvh_root.as_ref().and_then(|bvh_node| bvh_node.find_nearest_intersection(&shadow_ray, obj)) {
                            if shadow_dist < light_distance {
                                continue;  // Punto in ombra
                            }
                        }
                    }

                    let diff = normal.dot(&light_dir).max(0.0) as f32;
                    color = add_colors(&color, &multiply_color_scalar(&multiply_colors(&diffuse_color, &light.color), diff * light.intensity));

                    let reflect_dir = reflect(-light_dir, normal);
                    let spec = ray.direction.dot(&reflect_dir).max(0.0).powf(32.0) as f32;
                    color = add_colors(&color, &multiply_color_scalar(&light.color, spec * light.intensity * 0.5));
                }
            }

            // Applica il falloff della luce
            match light.falloff {
                FalloffType::Linear => {
                    let light_distance = (light.position - hit_point).length();
                    let attenuation = 1.0 / light_distance as f32;
                    color = multiply_color_scalar(&color, attenuation);
                },
                FalloffType::Quadratic => {
                    let light_distance = (light.position - hit_point).length();
                    let attenuation = 1.0 / (light_distance * light_distance);
                    color = multiply_color_scalar(&color, attenuation as f32);
                },
                FalloffType::None => {}
            }
        }

        // Ambient Occlusion
        if scene_settings.ao_enabled {
            let ao = compute_ao(hit_point, normal, obj, scene_settings.max_samples_ao);
            color = multiply_color_scalar(&color, ao as f32);
        }

        // Applicazione dei moltiplicatori
        color = multiply_color_scalar(&color, 1.0 - (scene_settings.shadow_mult as f32 / 100.0));
        color = multiply_color_scalar(&color, 1.0 - (scene_settings.ao_mult as f32 / 100.0));
        color
    } else {
        AGColor::new(0.0, 0.0, 0.0)  // Nessuna intersezione, colore di sfondo
    }
}

// Funzioni helper per le operazioni sui colori
fn add_colors(c1: &AGColor, c2: &AGColor) -> AGColor {
    AGColor::new(c1.r + c2.r, c1.g + c2.g, c1.b + c2.b)
}

fn multiply_colors(c1: &AGColor, c2: &AGColor) -> AGColor {
    AGColor::new(c1.r * c2.r, c1.g * c2.g, c1.b * c2.b)
}

fn multiply_color_scalar(c: &AGColor, s: f32) -> AGColor {
    AGColor::new(c.r * s, c.g * s, c.b * s)
}

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - n * 2.0 * v.dot(&n)
}

fn compute_ao(point: Vector3, normal: Vector3, obj: &BaseObject, samples: u32) -> f32 {
    let mut occlusion = 0.0;
    for _ in 0..samples {
        let sample_vec = Vector3::random_in_hemisphere(&normal);
        let ray = BaseRay::new(point + normal * 0.001, sample_vec);
        if let Some((distance, _)) = obj.bvh_root.as_ref().and_then(|bvh_node| bvh_node.find_nearest_intersection(&ray, obj)) {
            if distance < 1.0 {
                occlusion += 1.0 - distance;
            }
        }
    }
    1.0 - (occlusion / samples as f32)
}

use std::cmp::{max, min};

pub fn generate_buckets(width: u32, height: u32, rect: Rect, num_buckets: u32) -> Vec<Bucket> {
    let mut buckets = Vec::new();
    let min_buckets = max(num_cpus::get() as u32 * 3, 16);
    let (num_buckets_x, num_buckets_y) = if rect.width != width {
        let num = min(num_buckets, (min_buckets as f32).sqrt() as u32);
        (num, num)
    } else {
        (num_buckets, num_buckets)
    };

    let mut bucket_width = rect.width / num_buckets_x;
    let mut bucket_height = rect.height / num_buckets_y;
    let center_x = rect.x + rect.width / 2;
    let center_y = rect.y + rect.height / 2;

    bucket_width = max(1, bucket_width);
    bucket_height = max(1, bucket_height);

    let num_buckets_x = rect.width / bucket_width;
    let num_buckets_y = rect.height / bucket_height;

    for y in 0..num_buckets_y {
        for x in 0..num_buckets_x {
            let bucket_x = rect.x + x * bucket_width;
            let bucket_y = rect.y + y * bucket_height;
            let current_bucket_width = if x == num_buckets_x - 1 {
                min(bucket_width, rect.x + rect.width - bucket_x)
            } else {
                bucket_width
            };
            let current_bucket_height = if y == num_buckets_y - 1 {
                min(bucket_height, rect.y + rect.height - bucket_y)
            } else {
                bucket_height
            };

            let curr_rect = Rect {
                x: bucket_x,
                y: bucket_y,
                width: current_bucket_width,
                height: current_bucket_height,
            };
            buckets.push(Bucket::new(
                curr_rect
            ));
        }
    }

    sort_buckets(&mut buckets, center_x, center_y, bucket_width, bucket_height, num_buckets);
    buckets
}

// Assumiamo che questa funzione sia implementata altrove
fn sort_buckets(buckets: &mut Vec<Bucket>, center_x: u32, center_y: u32, bucket_width: u32, bucket_height: u32, num_buckets: u32) {
    // Implementazione della funzione di ordinamento
}

