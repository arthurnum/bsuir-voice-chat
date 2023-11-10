use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::surface::Surface;


pub struct TextRender<'ttf_module, 'rwops> {
    pub font: Font<'ttf_module, 'rwops>
}

impl<'ttf_module, 'rwops> TextRender<'ttf_module, 'rwops> {
    pub fn surface_from_timestamp(&self, timestamp: u64) -> Surface<'_> {
        self.font
            .render(format!("{timestamp:}").as_str())
            .solid(Color::RGB(12, 12, 12))
            .unwrap()
    }
}
