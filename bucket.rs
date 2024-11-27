use std::vec::Vec;
use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BucketState {
    NotProcessed = 0,
    StartProcessing = 1,
    Visualized = 2,
    NotVisualized = 3,
    EndProcessing = 4,
    Done = 5,
}

pub struct Bucket {
    pub rect: Rect,
    pub state: BucketState,
    pub buffer: Option<Vec<Color>>,
}

impl Bucket {
    // Inizializza un nuovo Bucket con una dimensione specificata nel Rect.
    pub fn new(rect: Rect) -> Self {
        let width = rect.width as usize; // Converti da u32 a usize
        let height = rect.height as usize; // Converti da u32 a usize
        Bucket {
            rect,
            state: BucketState::NotProcessed, // Assumendo che esista un metodo di costruzione per BucketState
            buffer: Some(vec![Color::default(); width * height]), // Inizializza con colori predefiniti
        }
    }

    // Ottieni un colore da una specifica posizione nel buffer.
    pub fn get_color(&self, x: usize, y: usize) -> Option<&Color> {
        self.buffer.as_ref()?.get(y * self.rect.width as usize + x)
    }

    // Imposta un colore in una specifica posizione nel buffer.
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        if let Some(buf) = self.buffer.as_mut() {
            let index = y * self.rect.width as usize + x;
            if index < buf.len() {
                buf[index] = color;
            }
        }
    }

}

// Struct per rappresentare un rettangolo
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}