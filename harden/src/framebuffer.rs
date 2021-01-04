//! This module contains generic data types surrounding framebuffers.

use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum PixelFormat {
    R5G6B5,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Format {
    pub height: u32,
    pub width: u32,
    pub stride: u32,
    pub pixel: PixelFormat,
}
        
