// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::PittoreColor;

#[derive(Copy, Clone, Debug)]
pub enum PittoreMaterial {
    Color(PittoreColor),
}

impl From<PittoreColor> for PittoreMaterial {
    fn from(value: PittoreColor) -> Self {
        Self::Color(value)
    }
}
