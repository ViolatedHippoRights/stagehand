use crate::loading::Ticket;

pub struct DrawBatch<T, C> {
    pub context: C,
    pub instructions: Vec<T>,
}

impl<T, C> DrawBatch<T, C> {
    pub fn new(context: C) -> Self {
        DrawBatch {
            context,
            instructions: Vec::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct DrawColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub enum DrawType {
    Texture,
    Text(String, DrawColor),
}

pub struct Draw {
    pub ticket: Ticket,
    pub draw_type: DrawType,
    pub data: DrawData,
}

pub struct DrawData {
    pub source: Option<DrawRect>,
    pub destination: Option<DrawDestination>,
    pub rotation: Option<DrawRotation>,
    pub flip: Option<DrawFlip>,
}

impl DrawData {
    pub fn draw_at(x: f32, y: f32) -> DrawData {
        DrawData {
            source: None,
            destination: Some(DrawDestination::Location { x, y }),
            rotation: None,
            flip: None,
        }
    }

    pub fn draw_centered_at(x: f32, y: f32) -> DrawData {
        DrawData::draw_rotated_at(x, y, 0.0, (0.5, 0.5))
    }

    pub fn draw_rotated_at(x: f32, y: f32, angle: f32, origin: (f32, f32)) -> DrawData {
        DrawData {
            source: None,
            destination: Some(DrawDestination::Location { x, y }),
            rotation: Some(DrawRotation { angle, origin }),
            flip: None,
        }
    }
}

pub struct DrawRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub enum DrawDestination {
    Location { x: f32, y: f32 },
    Rect(DrawRect),
}

pub struct DrawRotation {
    pub angle: f32,
    pub origin: (f32, f32),
}

pub struct DrawFlip {
    pub horizontal: bool,
    pub vertical: bool,
}
