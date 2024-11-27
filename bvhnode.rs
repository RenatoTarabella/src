use crate::boundingbox::Boundingbox;
use crate::baseobject::BaseObject;
use crate::baseray::BaseRay;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct BvhNode {
    pub bbox: Boundingbox,
    pub left: Option<Box<BvhNode>>,
    pub right: Option<Box<BvhNode>>,
    pub triangle_indices: Option<Box<[usize]>>,
}
impl BvhNode {
    const MAX_TRIANGLES_PER_LEAF: usize = 4;
    const MAX_DEPTH: usize = 40;

    pub fn build(base_object: &BaseObject) -> Self {
        let indices: Vec<usize> = (0..base_object.vadr.len()).collect();
        Self::build_recursive(base_object, &indices, 0)
    }

    fn build_recursive(base_object: &BaseObject, indices: &[usize], depth: usize) -> Self {
        if indices.len() <= Self::MAX_TRIANGLES_PER_LEAF || depth >= Self::MAX_DEPTH {
            let bbox = Self::get_enclosing_box(base_object, indices);
            return BvhNode {
                bbox,
                left: None,
                right: None,
                triangle_indices: Some(Vec::from(indices).into_boxed_slice()),
            };
        }

        let bbox = Self::get_enclosing_box(base_object, indices);
        let extent = bbox.max - bbox.min;
        
        // Determiniamo l'asse più lungo
        let axis = if extent.x > extent.y && extent.x > extent.z {
            0 // asse x
        } else if extent.y > extent.z {
            1 // asse y
        } else {
            2 // asse z
        };

        let split_point = (bbox.min[axis] + bbox.max[axis]) / 2.0;
        let (left_indices, right_indices) = Self::partition(base_object, indices, split_point, axis);

        let left = Box::new(Self::build_recursive(base_object, &left_indices, depth + 1));
        let right = Box::new(Self::build_recursive(base_object, &right_indices, depth + 1));

        BvhNode {
            bbox,
            left: Some(left),
            right: Some(right),
            triangle_indices: None,
        }
    }
    
    pub fn find_nearest_intersection<'a>(
        &'a self,
        ray: &BaseRay,
        base_object: &'a BaseObject,
    ) -> Option<(f32, usize)> {
        // Verifica se il raggio interseca la bounding box.
        let (t_min, t_max) = match ray.intersects(&self.bbox) {
            Some((t_min, t_max)) => (t_min, t_max),
            None => return None,
        };
    
        if let Some(indices) = &self.triangle_indices {
            // Processa i triangoli se siamo in un nodo foglia.
            return indices.iter()
                .filter_map(|&index| {
                    let triangle = &base_object.vadr[index];
                    ray.intersects_triangle(triangle, base_object).map(|distance| (distance, index))
                })
                .filter(|&(distance, _)| distance >= t_min as f32 && distance <= t_max as f32)
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        }
    
        // Determina quale nodo controllare per primo.
        let check_left_first = ray.origin.x < self.bbox.center.x;
        let (first_node, second_node) = if check_left_first { (&self.left, &self.right) } else { (&self.right, &self.left) };
    
        // Cerca l'intersezione nel primo nodo.
        let first_hit = first_node.as_ref().and_then(|node| node.find_nearest_intersection(ray, base_object));
    
        // Calcola l'intersezione nel secondo nodo solo se necessario.
        let second_hit = if let Some(first) = &first_hit {
            let first_point = ray.point_at_parameter(first.0);
            // Prosegui solo se il secondo nodo potrebbe contenere un punto più vicino.
            if first_point.x > second_node.as_ref()?.bbox.min.x {
                second_node.as_ref()?.find_nearest_intersection(ray, base_object)
            } else {
                None
            }
        } else {
            // Se non abbiamo trovato nessun hit nel primo nodo, controlliamo il secondo.
            second_node.as_ref()?.find_nearest_intersection(ray, base_object)
        };
    
        // Confronta i risultati ottenuti dai due nodi e restituisce il migliore.
        match (first_hit, second_hit) {
            (Some(first), Some(second)) => Some(if first.0 < second.0 { first } else { second }),
            (None, second) => second,
            (first, None) => first,
        }
    }
    

    fn partition(base_object: &BaseObject, indices: &[usize], split_point: f32, axis: usize) -> (Vec<usize>, Vec<usize>) {
        indices.iter()
            .partition(|&&i| base_object.tri_bbox[i].center[axis] < split_point)
    }

    fn get_enclosing_box(base_object: &BaseObject, indices: &[usize]) -> Boundingbox {
        indices.iter()
            .fold(Boundingbox::new_empty(), |mut bbox, &i| {
                bbox.expand(&base_object.tri_bbox[i]);
                bbox
            })
    }
}