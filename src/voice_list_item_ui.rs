use sdl2::rect::Rect;
use sdl2::render::Texture;


pub struct VoiceListItemUI<'tex> {
    pub timestamp: u64,
    pub rect: Rect,
    pub texture: Texture<'tex>,
}
