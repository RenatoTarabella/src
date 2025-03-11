use crate::boundingbox::Boundingbox;
use crate::baseobject::BaseObject;
use crate::baseray::BaseRay;
use std::cmp::Ordering;
use rayon::prelude::*; // Aggiungi questa dipendenza per la parallelizzazione

#[derive(Debug)]
pub struct BvhNode {
    pub bbox: Boundingbox,
    pub left: Option<Box<BvhNode>>,
    pub right: Option<Box<BvhNode>>,
    pub triangle_indices: Option<Box<[usize]>>,
}

impl BvhNode {
    const MAX_TRIANGLES_PER_LEAF: usize = 4; // Aumentato per ridurre la profondità dell'albero
    const MAX_DEPTH: usize = 32;

    pub fn build(base_object: &BaseObject) -> Self {
        let indices: Vec<usize> = (0..base_object.vadr.len()).collect();
        // Preallochiamo i buffer per le partizioni
        let mut left_buffer = Vec::with_capacity(indices.len());
        let mut right_buffer = Vec::with_capacity(indices.len());
        Self::build_recursive(base_object, &indices, 0, &mut left_buffer, &mut right_buffer)
    }

    fn build_recursive(
        base_object: &BaseObject, 
        indices: &[usize], 
        depth: usize,
        left_buffer: &mut Vec<usize>,
        right_buffer: &mut Vec<usize>,
    ) -> Self {
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

        // Utilizziamo la mediana invece del punto medio per un partizionamento più bilanciato
        let split_point = Self::find_median(base_object, indices, axis);
        
        // Riutilizziamo i buffer per evitare allocazioni
        left_buffer.clear();
        right_buffer.clear();
        
        // Partizionamento manuale per evitare allocazioni
        for &idx in indices {
            if base_object.tri_bbox[idx].center[axis] < split_point {
                left_buffer.push(idx);
            } else {
                right_buffer.push(idx);
            }
        }
        
        // Gestiamo il caso in cui tutti i triangoli finiscono da una parte
        if left_buffer.is_empty() || right_buffer.is_empty() {
            let mid = indices.len() / 2;
            left_buffer.clear();
            right_buffer.clear();
            left_buffer.extend_from_slice(&indices[..mid]);
            right_buffer.extend_from_slice(&indices[mid..]);
        }
        
        // Creiamo nuovi buffer per la ricorsione
        let mut left_child_buffer = Vec::with_capacity(left_buffer.len());
        let mut right_child_buffer = Vec::with_capacity(right_buffer.len());
        
        // Costruiamo i nodi figli
        let left = Box::new(Self::build_recursive(
            base_object, 
            left_buffer, 
            depth + 1,
            &mut left_child_buffer,
            &mut right_child_buffer
        ));
        
        let mut left_child_buffer = Vec::with_capacity(right_buffer.len());
        let mut right_child_buffer = Vec::with_capacity(right_buffer.len());
        
        let right = Box::new(Self::build_recursive(
            base_object, 
            right_buffer, 
            depth + 1,
            &mut left_child_buffer,
            &mut right_child_buffer
        ));

        BvhNode {
            bbox,
            left: Some(left),
            right: Some(right),
            triangle_indices: None,
        }
    }
    
    // Trova la mediana dei centri delle bounding box lungo un asse
    fn find_median(base_object: &BaseObject, indices: &[usize], axis: usize) -> f32 {
        if indices.len() <= 1 {
            return base_object.tri_bbox[indices[0]].center[axis];
        }
        
        // Per modelli grandi, utilizziamo un campionamento
        if indices.len() > 100 {
            // Prendiamo solo 100 campioni per modelli grandi
            let mut samples = Vec::with_capacity(100);
            let step = indices.len() / 100;
            for i in (0..indices.len()).step_by(step.max(1)) {
                if samples.len() < 100 {
                    samples.push(base_object.tri_bbox[indices[i]].center[axis]);
                }
            }
            samples.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            return samples[samples.len() / 2];
        }
        
        // Per modelli piccoli, usiamo l'approccio originale
        let mut values: Vec<f32> = indices.iter()
            .map(|&i| base_object.tri_bbox[i].center[axis])
            .collect();
        
        let mid = values.len() / 2;
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        values[mid]
    }
    
    pub fn find_nearest_intersection<'a>(
        &'a self,
        ray: &BaseRay,
        base_object: &'a BaseObject,
    ) -> Option<(f32, usize)> {
        // Verifica se il raggio interseca la bounding box
        let bbox_hit = ray.intersects(&self.bbox);
        if bbox_hit.is_none() {
            return None;
        }
        let (t_min, t_max) = bbox_hit.unwrap();
    
        // Caso foglia: controllo diretto dei triangoli
        if let Some(indices) = &self.triangle_indices {
            // Ridotto ulteriormente l'epsilon e rimosso il filtro t_min per catturare più intersezioni
            return indices.iter()
                .filter_map(|&index| {
                    let triangle = &base_object.vadr[index];
                    ray.intersects_triangle(triangle, base_object).map(|distance| (distance, index))
                })
                .filter(|&(distance, _)| distance > 0.000001 && distance <= t_max as f32)
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        }
    
        // Determina l'ordine di traversata basato sulla direzione del raggio
        // Modifica: utilizziamo l'asse dominante per determinare l'ordine
        let axis = if self.bbox.max.x - self.bbox.min.x > self.bbox.max.y - self.bbox.min.y &&
                     self.bbox.max.x - self.bbox.min.x > self.bbox.max.z - self.bbox.min.z {
            0 // asse x
        } else if self.bbox.max.y - self.bbox.min.y > self.bbox.max.z - self.bbox.min.z {
            1 // asse y
        } else {
            2 // asse z
        };
        
        let dir_is_neg = [ray.direction.x < 0.0, ray.direction.y < 0.0, ray.direction.z < 0.0];
        let (first, second) = if dir_is_neg[axis] {
            (&self.right, &self.left)
        } else {
            (&self.left, &self.right)
        };
    
        // Traversata ottimizzata
        let first_hit = first.as_ref().and_then(|node| node.find_nearest_intersection(ray, base_object));
        
        // Verifica se è necessario controllare il secondo nodo
        if let Some((dist, _)) = first_hit {
            if dist < t_max as f32 {
                return first_hit;
            }
        }
        
        let second_hit = second.as_ref().and_then(|node| node.find_nearest_intersection(ray, base_object));
        
        // Restituisci l'intersezione più vicina, assicurandoci che sia valida
        match (first_hit, second_hit) {
            (Some(first), Some(second)) => {
                if first.0 <= 0.0 && second.0 <= 0.0 {
                    None
                } else if first.0 <= 0.0 {
                    Some(second)
                } else if second.0 <= 0.0 {
                    Some(first)
                } else {
                    Some(if first.0 < second.0 { first } else { second })
                }
            },
            (None, Some(second)) if second.0 > 0.0 => Some(second),
            (Some(first), None) if first.0 > 0.0 => Some(first),
            _ => None,
        }
    }

    // Metodo rimosso perché sostituito dall'implementazione diretta nel build_recursive
    // fn partition(base_object: &BaseObject, indices: &[usize], split_point: f32, axis: usize) -> (Vec<usize>, Vec<usize>) {
    //     ...
    // }

    fn get_enclosing_box(base_object: &BaseObject, indices: &[usize]) -> Boundingbox {
        if indices.is_empty() {
            return Boundingbox::new_empty();
        }
        
        // Inizializza con i valori del primo triangolo
        let first_bbox = &base_object.tri_bbox[indices[0]];
        let mut min_x = first_bbox.min.x;
        let mut min_y = first_bbox.min.y;
        let mut min_z = first_bbox.min.z;
        let mut max_x = first_bbox.max.x;
        let mut max_y = first_bbox.max.y;
        let mut max_z = first_bbox.max.z;
        
        // Aggiorna con i valori dei triangoli rimanenti
        for &i in &indices[1..] {
            let bbox = &base_object.tri_bbox[i];
            min_x = min_x.min(bbox.min.x);
            min_y = min_y.min(bbox.min.y);
            min_z = min_z.min(bbox.min.z);
            max_x = max_x.max(bbox.max.x);
            max_y = max_y.max(bbox.max.y);
            max_z = max_z.max(bbox.max.z);
        }
        
        // Crea la bounding box risultante
        let min = crate::vector3::Vector3::new(min_x, min_y, min_z);
        let max = crate::vector3::Vector3::new(max_x, max_y, max_z);
        let center = (min + max) * 0.5;
        
        Boundingbox { min, max, center }
    }
}