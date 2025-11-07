use tiny_skia::Transform;

pub struct TextEngine {
    font_db: fontdb::Database,
    default_font: fontdb::ID,
}

impl TextEngine {
    pub fn new() -> Self {
        let mut font_db = fontdb::Database::new();
        font_db.load_system_fonts();

        let query = fontdb::Query {
            families: &[fontdb::Family::Monospace, fontdb::Family::SansSerif],
            weight: fontdb::Weight::NORMAL,
            ..fontdb::Query::default()
        };

        let face_id = font_db.query(&query).expect("query font failed");

        unsafe {
            font_db
                .make_shared_face_data(face_id)
                .expect("load face failed");
        }

        let face = font_db.face(face_id).expect("get font face failed");

        println!("loaded default font: {}, heith", face.post_script_name);

        TextEngine {
            font_db,
            default_font: face_id,
        }
    }

    pub fn draw_text(&self, s: &str, pixmap: &mut tiny_skia::Pixmap, paint: &tiny_skia::Paint) {
        let (src, index) = self.font_db.face_source(self.default_font).unwrap();

        let font_data = match &src {
            fontdb::Source::SharedFile(_, data) => data.clone(),
            _ => unreachable!("font file not shared"),
        };

        let face = rustybuzz::Face::from_slice(font_data.as_ref().as_ref(), index)
            .expect("parse font failed");

        let mut buffer = rustybuzz::UnicodeBuffer::new();

        buffer.push_str(s);

        let glyphs = rustybuzz::shape(&face, &[], buffer);

        let scale = 64.0 / face.height() as f32;

        let mut x_off = 0.0;

        for (info, pos) in glyphs
            .glyph_infos()
            .iter()
            .zip(glyphs.glyph_positions().iter())
        {
            let mut outline_builder = GlyphOutineBuilder::new(face.height() as f32);

            match face.outline_glyph(
                rustybuzz::ttf_parser::GlyphId(info.glyph_id as u16),
                &mut outline_builder,
            ) {
                Some(rect) => {
                    let path = outline_builder.build();
                    pixmap.fill_path(
                        &path,
                        paint,
                        tiny_skia::FillRule::EvenOdd,
                        Transform::from_translate(x_off + pos.x_offset as f32, 0.0)
                            .post_scale(scale, scale),
                        None,
                    );

                    x_off += pos.x_advance as f32;
                }
                None => {
                    println!("outline_glyph failed");
                }
            }
        }
    }

    pub fn get_glyph_outline(&self) {}
}

struct GlyphOutineBuilder {
    inner: tiny_skia::PathBuilder,
    height: f32,
}

impl GlyphOutineBuilder {
    fn new(height: f32) -> Self {
        GlyphOutineBuilder {
            inner: tiny_skia::PathBuilder::new(),
            height,
        }
    }

    fn my_y(&self, y: f32) -> f32 {
        self.height - y
    }

    fn build(self) -> tiny_skia::Path {
        self.inner.finish().unwrap()
    }
}

impl rustybuzz::ttf_parser::OutlineBuilder for GlyphOutineBuilder {
    fn close(&mut self) {
        self.inner.close()
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.inner
            .cubic_to(x1, self.my_y(y1), x2, self.my_y(y2), x, self.my_y(y))
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.inner.line_to(x, self.my_y(y))
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.inner.move_to(x, self.my_y(y))
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.inner.quad_to(x1, self.my_y(y1), x, self.my_y(y))
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use tiny_skia::{Color, Paint, Pixmap};

    use super::*;

    #[test]
    fn test_draw_text() {
        let mut pixmap = Pixmap::new(1024, 1024).unwrap();

        let mut paint = Paint::default();

        paint.set_color_rgba8(0, 0, 0, 255);

        pixmap.fill(Color::from_rgba8(255, 255, 255, 255));

        let text_engine = TextEngine::new();

        text_engine.draw_text("asdfgiuytrewqFASDF", &mut pixmap, &paint);

        let png = pixmap.encode_png().unwrap();

        let mut file = std::fs::File::create("image.png").unwrap();

        file.write_all(&png).unwrap();
    }
}
