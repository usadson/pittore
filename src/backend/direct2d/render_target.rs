// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::Mutex, ops::Deref};

use windows::Win32::Graphics::Direct2D::ID2D1HwndRenderTarget;

use crate::{RenderTarget, PittoreRenderPass, PittoreRenderError};

#[derive(Debug)]
pub(super) struct DirectRenderTarget {
    inner: Mutex<ID2D1HwndRenderTarget>,
}

impl DirectRenderTarget {
    pub fn new(inner: ID2D1HwndRenderTarget) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }
}

impl RenderTarget for DirectRenderTarget {
    fn begin_render_pass(
        &self,
        f: &mut dyn FnMut(&mut dyn PittoreRenderPass),
    ) -> Result<(), PittoreRenderError> {
        let Ok(target) = self.inner.try_lock() else {
            return Err(PittoreRenderError::RenderTargetAlreadyInUse);
        };

        unsafe {
            target.BeginDraw();
        }

        let mut pass = DirectRenderPass {
            handle: target.deref()
        };

        f(&mut pass);

        if let Err(e) = unsafe { target.EndDraw(None, None) } {
            return Err(PittoreRenderError::Direct2DGenericError(e.into()));
        }

        Ok(())
    }
}

struct DirectRenderPass<'handle> {
    handle: &'handle ID2D1HwndRenderTarget,
}

impl<'handle> PittoreRenderPass for DirectRenderPass<'handle> {
    fn clear(&mut self, color: crate::PittoreColor) {
        unsafe {
            self.handle.Clear(Some(&color.into()));
        }
    }
}
