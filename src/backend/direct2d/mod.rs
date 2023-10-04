// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod factory;
mod render_target;

use std::sync::Arc;
use windows::Win32::Graphics::Direct2D::Common::{D2D_COLOR_F, D2D1_COLOR_F};

use crate::{
    Backend,
    PittoreInstantiationError,
    PittoreWindowAttachmentError,
    PittoreRenderTarget, PittoreColor,
};

use self::{factory::DirectFactory, render_target::DirectRenderTarget};

#[derive(Debug)]
pub struct DirectBackend {
    factory: DirectFactory,
}

impl Backend for DirectBackend {
    fn attach_to_window(&self, window: &winit::window::Window) -> Result<PittoreRenderTarget, PittoreWindowAttachmentError> {
        let render_target = self.factory.create_render_target(window)?;
        let render_target = DirectRenderTarget::new(render_target);
        Ok(PittoreRenderTarget::new(render_target))
    }
}

pub(crate) fn create_backend() -> Result<Arc<dyn Backend>, PittoreInstantiationError> {
    let factory = match DirectFactory::new() {
        Ok(factory) => factory,
        Err(e) => {
            log::error!("Failed to create Direct2D Factory: {e:?}");
            return Err(PittoreInstantiationError::Direct2DFactoryCreationFailure(e.into()))
        }
    };

    Ok(Arc::new(DirectBackend {
        factory,
    }))
}

impl From<PittoreColor> for D2D_COLOR_F {
    fn from(value: PittoreColor) -> Self {
        D2D_COLOR_F {
            r: value.red() as _,
            g: value.green() as _,
            b: value.blue() as _,
            a: value.alpha() as _,
        }
    }
}

impl From<PittoreColor> for D2D1_COLOR_F {
    fn from(value: PittoreColor) -> Self {
        D2D1_COLOR_F {
            r: value.red() as _,
            g: value.green() as _,
            b: value.blue() as _,
            a: value.alpha() as _,
        }
    }
}
