// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::PittoreColor;

/// An opaque bitmap reference.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PittoreBitmap {
    namespace: u64,
    id: u64,
}

impl PittoreBitmap {
    pub(crate) const fn new(namespace: u64, id: u64) -> Self {
        Self {
            namespace,
            id,
        }
    }

    #[allow(unused)]
    pub(crate) const fn namespace(&self) -> u64 {
        self.namespace
    }

    #[allow(unused)]
    pub(crate) const fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PittoreMaterial {
    Bitmap(PittoreBitmap),
    Color(PittoreColor),
}

impl From<PittoreBitmap> for PittoreMaterial {
    fn from(value: PittoreBitmap) -> Self {
        Self::Bitmap(value)
    }
}

impl From<PittoreColor> for PittoreMaterial {
    fn from(value: PittoreColor) -> Self {
        Self::Color(value)
    }
}
