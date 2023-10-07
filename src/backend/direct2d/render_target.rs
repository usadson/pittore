// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use dashmap::DashMap;
use windows::{
    core::ComInterface,
    Win32::Graphics::Direct2D::{
        Common::{
            D2D1_COLOR_F,
            D2D_POINT_2F,
            D2D_RECT_F,
            D2D_SIZE_U,
        },
        D2D1_ELLIPSE,
        ID2D1Bitmap,
        ID2D1BitmapBrush,
        ID2D1Brush,
        ID2D1HwndRenderTarget,
        ID2D1SolidColorBrush, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
    },
};

use crate::{
    PittoreBitmap,
    PittoreBitmapLoadError,
    PittoreColor,
    PittoreMaterial,
    PittoreRect,
    PittoreShape,
    PittoreRenderError,
    PittoreRenderPass,
    PittoreResizeError,
    RenderTarget,
};

use super::wic::WicFactory;

#[derive(Debug)]
pub(super) struct DirectRenderTarget {
    inner: Mutex<ID2D1HwndRenderTarget>,
    bitmaps: DashMap<PittoreBitmap, DirectBitmap>,
    bitmap_idx: AtomicU64,
    wic_factory: WicFactory,
}

impl DirectRenderTarget {
    pub fn new(inner: ID2D1HwndRenderTarget) -> Self {
        Self {
            inner: Mutex::new(inner),
            bitmaps: DashMap::new(),
            bitmap_idx: AtomicU64::new(0),
            wic_factory: WicFactory::new().unwrap(),
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
            bitmaps: &self.bitmaps,
        };

        f(&mut pass);

        if let Err(e) = unsafe { target.EndDraw(None, None) } {
            return Err(PittoreRenderError::Direct2DGenericError(e.into()));
        }

        Ok(())
    }

    fn load_bitmap_from_file(&self, file_path: &str) -> Result<PittoreBitmap, PittoreBitmapLoadError> {
        let target = self.inner.lock().unwrap();
        let id = self.bitmap_idx.fetch_add(1, Ordering::AcqRel);

        match self.wic_factory.load_bitmap_from_file(target.cast().unwrap(), file_path) {
            Ok(d2_bitmap) => {
                let bitmap = PittoreBitmap::new(0xD2D, id);

                let brush = unsafe {
                    target.CreateBitmapBrush(&d2_bitmap, None, None)
                };

                match brush {
                    Ok(brush) => {
                        self.bitmaps.insert(bitmap, DirectBitmap {
                            bitmap: d2_bitmap,
                            brush,
                        });

                        Ok(bitmap)
                    }

                    Err(..) => todo!(),
                }
            }

            Err(..) => todo!()
        }
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
    bitmaps: &'handle DashMap<PittoreBitmap, DirectBitmap>,
    solid_color_brush: ID2D1SolidColorBrush,
}

impl<'handle> PittoreRenderPass for DirectRenderPass<'handle> {
    fn clear(&mut self, color: PittoreColor) {
        unsafe {
            self.handle.Clear(Some(&color.into()));
        }
    }

    fn fill(&mut self, material: PittoreMaterial, shape: PittoreShape) {
        let brush: ID2D1Brush = match material {
            PittoreMaterial::Bitmap(bitmap) => {
                let Some(bitmap) = self.bitmaps.get(&bitmap) else {
                    log::error!("Invalid bitmap material passed: {bitmap:?}");
                    return;
                };

                if let PittoreShape::Rectangle(rect) = shape {
                    unsafe {
                        self.handle.DrawBitmap(
                            &bitmap.bitmap,
                            Some(&convert_rect(rect)),
                            1.0,
                            D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
                            None,
                        );
                    }
                    return;
                }

                bitmap.brush.cast::<ID2D1Brush>().unwrap()
            }
            PittoreMaterial::Color(color) => unsafe {
                self.solid_color_brush.SetColor(&color.into());
                self.solid_color_brush.cast::<ID2D1Brush>().unwrap()
            }
        };

        match shape {
            PittoreShape::Rectangle(rect) => unsafe {
                self.handle.FillRectangle(&convert_rect(rect), &brush)
            }
            PittoreShape::Ellipse { center, radius } => unsafe {
                self.handle.FillEllipse(
                    &D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: center.x,
                            y: center.y,
                        },
                        radiusX: radius.x,
                        radiusY: radius.y,
                    },
                    &brush,
                );
            }
        }
    }
}

#[derive(Debug)]
struct DirectBitmap {
    /// Allow this bitmap to be unused, since we only keep this around so the
    /// bitmap brush stays valid.
    #[allow(unused)]
    bitmap: ID2D1Bitmap,

    brush: ID2D1BitmapBrush,
}

fn convert_rect(value: PittoreRect) -> D2D_RECT_F {
    D2D_RECT_F {
        left: value.min_x(),
        top: value.min_y(),
        right: value.max_x(),
        bottom: value.max_y(),
    }
}
