// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::Mutex, ops::Deref};

use windows::Win32::Graphics::Direct2D::{
    Common::{
        D2D1_COLOR_F,
        D2D_SIZE_U,
        D2D_RECT_F,
    },
    ID2D1HwndRenderTarget,
    ID2D1SolidColorBrush,
};

use crate::{
    PittoreColor,
    PittoreRect,
    PittoreShape,
    PittoreRenderError,
    PittoreRenderPass,
    PittoreResizeError,
    RenderTarget,
};

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

        let solid_color_brush = unsafe {
            target.CreateSolidColorBrush(&D2D1_COLOR_F::default(), None)
        }.unwrap();

        unsafe {
            target.BeginDraw();
        }

        let mut pass = DirectRenderPass {
            handle: target.deref(),
            solid_color_brush,
        };

        f(&mut pass);

        if let Err(e) = unsafe { target.EndDraw(None, None) } {
            return Err(PittoreRenderError::Direct2DGenericError(e.into()));
        }

        Ok(())
    }

    fn resize(&self, width: u32, height: u32) -> Result<(), PittoreResizeError> {
        let size = D2D_SIZE_U { width, height };
        let handle = self.inner.lock().unwrap();

        let result = unsafe { handle.Resize(&size) };

        result.map_err(|e| PittoreResizeError::Direct2DGenericError(e.into()))
    }
}

struct DirectRenderPass<'handle> {
    handle: &'handle ID2D1HwndRenderTarget,
    solid_color_brush: ID2D1SolidColorBrush,
}

impl<'handle> PittoreRenderPass for DirectRenderPass<'handle> {
    fn clear(&mut self, color: PittoreColor) {
        unsafe {
            self.handle.Clear(Some(&color.into()));
        }
    }

    fn fill(&mut self, color: PittoreColor, shape: PittoreShape) {
        unsafe {
            self.solid_color_brush.SetColor(&color.into());
        }

        match shape {
            PittoreShape::Rectangle(rect) => unsafe {
                self.handle.FillRectangle(&convert_rect(rect), &self.solid_color_brush)
            }
        }
    }
}

fn convert_rect(value: PittoreRect) -> D2D_RECT_F {
    D2D_RECT_F {
        left: value.min_x(),
        top: value.min_y(),
        right: value.max_x(),
        bottom: value.max_y(),
    }
}
