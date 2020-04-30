use serde::Serialize;

use crate::colors;

pub struct Rect<T> {
    pub x1: T,
    pub y1: T,
    pub x2: T,
    pub y2: T,
}

#[derive(Debug, Serialize, Copy, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Serialize)]
pub struct Bar {
    orientation: Orientation,
    pub origin: Position,
    pub attempted_origin: Position,
}

#[derive(Debug, Serialize)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Bar {
    pub fn new(x: u32, y: u32) -> Self {
        let origin = Position { x, y };

        Bar {
            orientation: Orientation::Vertical,
            origin,
            attempted_origin: origin,
        }
    }

    pub fn bounding_box(&self) -> Rect<u32> {
        match self.orientation {
            Orientation::Horizontal => Rect {
                x1: self.origin.x - 1,
                y1: self.origin.y,
                x2: self.origin.x + 3,
                y2: self.origin.y,
            },
            _ => Rect {
                x1: self.origin.x,
                y1: self.origin.y - 1,
                x2: self.origin.x,
                y2: self.origin.y + 3,
            },
        }
    }

    pub fn rotate_left(&mut self) {
        self.rotate_right();
    }

    pub fn rotate_right(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Horizontal => Orientation::Vertical,
            _ => Orientation::Horizontal,
        }
    }

    pub fn set_origin(&mut self, x: u32, y: u32) {
        self.origin.x = x;
        self.origin.y = y;
    }

    pub fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        origin_x: f64,
        origin_y: f64,
        pixels_per_cell: f64,
    ) {
        context.begin_path();

        let bb: Rect<f64> = match self.orientation {
            Orientation::Horizontal => Rect {
                x1: origin_x - pixels_per_cell,
                y1: origin_y,
                x2: origin_x + 3.0 * pixels_per_cell,
                y2: origin_y + pixels_per_cell,
            },
            _ => Rect {
                x1: origin_x,
                y1: origin_y - pixels_per_cell,
                x2: origin_x + pixels_per_cell,
                y2: origin_y + 3.0 * pixels_per_cell,
            },
        };

        context.set_stroke_style(&colors::BAR_STROKE.into());
        context.set_fill_style(&colors::BAR_FILL.into());

        context.move_to(bb.x1, bb.y1);
        context.line_to(bb.x1, bb.y2);
        context.line_to(bb.x2, bb.y2);
        context.line_to(bb.x2, bb.y1);
        context.line_to(bb.x1, bb.y1);

        context.stroke();
        context.fill();
    }
}