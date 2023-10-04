// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    AsDebug,
    PittoreRenderTarget,
    PittoreWindowAttachmentError,
};

#[cfg(windows)]
pub(crate) mod direct2d;

pub trait Backend: AsDebug + Send + Sync {
    fn attach_to_window(&self, window: &winit::window::Window) -> Result<PittoreRenderTarget, PittoreWindowAttachmentError>;
}
