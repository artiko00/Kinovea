//! Borrowed RGBA8 pixels for `Renderer::draw_image`: row-major, four bytes per pixel.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageRef<'a> {
    pub width: u32,
    pub height: u32,
    pub rgba: &'a [u8],
}
