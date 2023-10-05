// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    PittoreColor,
    PittoreShape,
};

pub trait PittoreRenderPass {
    fn clear(&mut self, color: PittoreColor);

    fn fill(&mut self, color: PittoreColor, shape: PittoreShape);
}
