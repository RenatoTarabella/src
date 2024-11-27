use std::fs::File;
use std::io::BufReader;
// use std::collections::HashMap;
use crate::boundingbox::Boundingbox;
use crate::vector3::Vector3;
use crate::matrix::Matrix;
use crate::bvhnode::BvhNode;
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::Read;

#[derive(Debug)]
pub struct BaseObject {
    pub name: String,
    pub filename: String,

    pub bvh_root: Option<BvhNode>,

    // pub bvh_node: u64, // Riferimento opzionale a BVHNode
    pub material:  u64,      // Riferimento a BaseMaterial nella lista principale dei materiali

    // liste di dati relative ai vertici --------------------------------------
    pub padr: Vec<Vector3>,          // Lista dei vertici
    pub uvw: Option<Vec<Vector3>>,           // Lista delle coordinate uv
    pub phong_normal: Option<Vec<Vector3>>,  // Lista delle normali di Phong

    // liste di dati relative ai triangoli -------------------------------------
    pub vadr: Vec<Triangle>,         // Lista dei triangoli, che sono proprietà di BaseObject
    pub norm: Vec<Vector3>,          // Lista delle normali
    pub tri_bbox: Vec<Boundingbox>,  // Lista delle bounding box dei triangoli

    pub mg: Matrix,                  // Matrice di trasformazione
    pub boundingbox: Option<Boundingbox>,    
    // pub adiacent_dict: Option<HashMap<Vector3, Vec<Triangle>>>, // Dizionario degli adiacenti
}


impl BaseObject {
    pub fn new(name: String, filename: String) -> Self {
        BaseObject {
            name,
            filename,
            bvh_root: None,
            material: 0,
            padr: Vec::new(),
            uvw: None,
            phong_normal: None,
            vadr: Vec::new(),
            tri_bbox: Vec::new(),
            norm: Vec::new(),
            mg: Matrix::identity(),
            boundingbox: None,
        }
    }    

    pub fn read_vector3(reader: &mut BufReader<File>) -> Result<Vector3, std::io::Error> {
        let x = reader.read_f32::<LittleEndian>()? as f32;
        let z = reader.read_f32::<LittleEndian>()? as f32; // Originariamente y
        let y = reader.read_f32::<LittleEndian>()? as f32; // Originariamente z
        Ok(Vector3::new(x, y, z))
    }
    
    pub fn load_stl(filename: &str) -> Result<BaseObject, std::io::Error> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        
        let mut header = [0u8; 80];
        reader.read_exact(&mut header)?;
        
        let num_triangles = reader.read_u32::<LittleEndian>()?;
        
        let mut obj = BaseObject::new(String::from("STL Object"), filename.to_string());
    
        for _ in 0..num_triangles {
            let normal = Self::read_vector3(&mut reader)?;
            let v1 = Self::read_vector3(&mut reader)?;
            let v2 = Self::read_vector3(&mut reader)?;
            let v3 = Self::read_vector3(&mut reader)?;
            
            obj.norm.push(normal);
            obj.padr.push(v1);
            obj.padr.push(v2);
            obj.padr.push(v3);
            
    
            let bboxtri = Boundingbox::from_triangle(v1, v2, v3);
            obj.tri_bbox.push(bboxtri);
    
            obj.vadr.push(Triangle::new(
                obj.padr.len() - 3,
                obj.padr.len() - 2,
                obj.padr.len() - 1,
            ));
            
            // Skip attribute byte count
            reader.read_u16::<LittleEndian>()?;
        }
    
        obj.boundingbox = Some(Self::calculate_bounding_box(&obj.padr));
        Ok(obj)
    }
    

    pub fn build_bvh(&mut self) {
        self.bvh_root = Some(BvhNode::build(self));
    }

    pub fn snap_to_grid(vertex: Vector3, grid_size: f32) -> Vector3 {
        Vector3::new(
            (vertex.x / grid_size).round() * grid_size,
            (vertex.y / grid_size).round() * grid_size,
            (vertex.z / grid_size).round() * grid_size,
        )
    }

    fn calculate_bounding_box(vertices: &[Vector3]) -> Boundingbox {
        let mut min = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vector3::new(f32::MIN, f32::MIN, f32::MIN);
    
        for vertex in vertices {
            min.x = min.x.min(vertex.x);
            min.y = min.y.min(vertex.y);
            min.z = min.z.min(vertex.z);
    
            max.x = max.x.max(vertex.x);
            max.y = max.y.max(vertex.y);
            max.z = max.z.max(vertex.z);
        }
    
        let center = Vector3::new(
            (min.x + max.x) * 0.5,
            (min.y + max.y) * 0.5,
            (min.z + max.z) * 0.5,
        );
    
        Boundingbox {
            min,
            max,
            center,
        }
    }
}

#[derive(Debug)]
pub struct Triangle{
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Triangle {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Triangle { a, b, c }
    }
}

