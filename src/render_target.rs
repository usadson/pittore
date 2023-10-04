// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;

use crate::{AsDebug, PittoreRenderPass, PittoreRenderError};

pub struct PittoreRenderTarget {
    target: Arc<dyn RenderTarget>,
}

impl PittoreRenderTarget {
    pub(crate) fn new<T>(target: T) -> Self
            where T: RenderTarget + 'static {
        Self {
            target: Arc::new(target),
        }
    }
}

impl std::ops::Deref for PittoreRenderTarget {
    type Target = dyn RenderTarget;

    fn deref(&self) -> &Self::Target {
        self.target.deref()
    }
}

pub trait RenderTarget: AsDebug + Send + Sync {
    fn begin_render_pass(&self, f: &mut dyn FnMut(&mut dyn PittoreRenderPass)) -> Result<(), PittoreRenderError>;
}
