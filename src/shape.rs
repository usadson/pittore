// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

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
