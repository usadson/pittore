// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This module contains the code for loading images using the Windows Imaging
//! Component.

use windows::{
    core::{
        HSTRING,
        PCWSTR,
    },
    Win32::{
        Foundation::GENERIC_READ,
        Graphics::{
            Imaging::{
                CLSID_WICImagingFactory,

                GUID_WICPixelFormat32bppPBGRA,

                IWICImagingFactory,

                WICDecodeMetadataCacheOnLoad,
                WICBitmapDitherTypeNone,
                WICBitmapPaletteTypeMedianCut,
            },
            Direct2D::{
                ID2D1RenderTarget,
                ID2D1Bitmap,
            },
        },
        System::Com::{
            CoCreateInstance,
            CLSCTX_INPROC_SERVER
        },
    },
};

#[derive(Debug)]
pub struct WicFactory {
    inner: IWICImagingFactory,
}

impl WicFactory {
    pub fn new() -> windows::core::Result<Self> {
        let inner = unsafe {
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)?
        };

        Ok(Self {
            inner,
        })
    }

    pub fn load_bitmap_from_file(
        &self,
        render_target: ID2D1RenderTarget,
        file_path: &str,
    ) -> windows::core::Result<ID2D1Bitmap> {
        unsafe {
            let file_path = HSTRING::from(file_path);
            let file_path = PCWSTR::from_raw(file_path.as_ptr());

            let decoder = self.inner.CreateDecoderFromFilename(
                file_path,
                None,
                GENERIC_READ,
                WICDecodeMetadataCacheOnLoad,
            )?;

            let source = decoder.GetFrame(0)?;

            let converter = self.inner.CreateFormatConverter()?;

            converter.Initialize(
                &source,
                &GUID_WICPixelFormat32bppPBGRA,
                WICBitmapDitherTypeNone,
                None,
                0.0,
                WICBitmapPaletteTypeMedianCut,
            )?;

            let bitmap = render_target.CreateBitmapFromWicBitmap(
                &converter,
                None,
            )?;

            Ok(bitmap)
        }
    }
}

unsafe impl Send for WicFactory {}
unsafe impl Sync for WicFactory {}
