// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub type PittoreRect = euclid::default::Rect<f32>;

#[derive(Debug, Clone)]
pub enum PittoreShape {
    Rectangle(PittoreRect),
}
