// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Size2D;

pub type PittoreRect = euclid::default::Rect<f32>;
pub type PittorePoint = euclid::default::Point2D<f32>;

#[derive(Debug, Clone)]
pub enum PittoreShape {
    Ellipse {
        center: PittorePoint,
        radius: PittorePoint,
    },
    Rectangle(PittoreRect),
}

impl PittoreShape {
    pub fn size(&self) -> Size2D<f32> {
        match self {
            Self::Ellipse { radius, .. } => Size2D::new(radius.x, radius.y),
            Self::Rectangle(rect) => rect.size,
        }
    }
}
