// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod backend;
mod color;
mod debug;
mod error;
mod render_pass;
mod render_target;
mod shape;

use std::sync::Arc;

pub use self::{
    color::PittoreColor,
    error::{
        PittoreInstantiationError,
        PittoreRenderError,
        PittoreResizeError,
        PittoreWindowAttachmentError,
    },
    render_pass::PittoreRenderPass,
    render_target::PittoreRenderTarget,
    shape::{
        PittoreRect,
        PittorePoint,
        PittoreShape,
    },
};

pub(crate) use self::{
    backend::Backend,
    debug::AsDebug,
    render_target::RenderTarget,
};

#[derive(Clone)]
pub struct PittoreContext {
    backend: Arc<dyn Backend>,
}

impl PittoreContext {
    pub fn attach_to_window(
        &self,
        window: &winit::window::Window
    ) -> Result<PittoreRenderTarget, PittoreWindowAttachmentError> {
        self.backend.attach_to_window(window)
    }
}

pub struct PittoreContextBuilder {
    debug: bool,
}

impl Default for PittoreContextBuilder {
    fn default() -> Self {
        Self {
            #[cfg(debug_assertions)]
            debug: true,

            #[cfg(not(debug_assertions))]
            debug: false,

        }
    }
}

impl PittoreContextBuilder {
    pub fn new() -> Self { Self::default() }

    /// Enable or disable the debug flag. Initially set to `true` on debug
    /// builds, and `false` on release builds.
    pub fn with_debug(self, debug: bool) -> Self {
        Self {
            debug,
            ..self
        }
    }

    pub fn build(self) -> Result<PittoreContext, PittoreInstantiationError> {
        #[cfg(windows)]
        {
            let backend = backend::direct2d::create_backend(self.debug)?;
            Ok(PittoreContext {
                backend,
            })
        }

        #[cfg(not(windows))]
        {
            Err(PittoreInstantiationError::UnsupportedSystem)
        }
    }
}
