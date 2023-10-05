// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// An error that occurred whilst creating the backend.
#[derive(Debug, thiserror::Error)]
pub enum PittoreInstantiationError {
    #[error("No backend supports the current environment or operating system.")]
    UnsupportedSystem,

    #[cfg(windows)]
    #[error("Direct2D backend failed to create a factory")]
    Direct2DFactoryCreationFailure(WindowsError),
}

/// An error that occurred whilst rendering, or beginning a render pass.
#[derive(Debug, thiserror::Error)]
pub enum PittoreRenderError {
    #[error("A render pass has already started on this render pass")]
    RenderTargetAlreadyInUse,

    #[cfg(windows)]
    #[error("Direct2D failed to render")]
    Direct2DGenericError(WindowsError),
}

/// An error that occurred whilst resizing a render target.
#[derive(Debug, thiserror::Error)]
pub enum PittoreResizeError {
    #[error("The new dimensions for resizing the render target are invalid.")]
    InvalidResizeDimensions,

    #[cfg(windows)]
    #[error("Direct2D failed to resize the render target")]
    Direct2DGenericError(WindowsError),
}

/// An error that occurred whilst attaching to a window.
#[derive(Debug, thiserror::Error)]
pub enum PittoreWindowAttachmentError {
    #[error("The backend doesn't support attaching to this window handle type")]
    UnsupportedWindowHandle,

    #[cfg(windows)]
    #[error("Direct2D backend failed to create an HWND render target")]
    Direct2DHwndRenderTargetCreationFailure(WindowsError),
}

#[derive(Debug)]
#[cfg(windows)]
pub struct WindowsError {
    #[allow(unused)]
    error: windows::core::Error,
}

#[cfg(windows)]
impl From<windows::core::Error> for WindowsError {
    fn from(error: windows::core::Error) -> Self {
        Self { error }
    }
}
