use tiny_skia::{Color, Paint, Pixmap, Rect, Transform};

use crate::game::Point;

pub trait Renderer {
    fn draw_points<'a>(&mut self, points: impl Iterator<Item = &'a Point>, color: Color);

    fn render(&self, buffer: &mut [u32]);

    fn clear(&mut self);
}

pub struct SkiaRenderer {
    scale: f32,
    pixmap: Pixmap,
}

impl SkiaRenderer {
    pub fn new(width: u32, height: u32, scale: f32) -> Self {
        SkiaRenderer {
            scale,
            pixmap: Pixmap::new(width, height).unwrap(),
        }
    }
}

impl Renderer for SkiaRenderer {
    fn draw_points<'a>(&mut self, points: impl Iterator<Item = &'a Point>, color: Color) {
        let mut paint = Paint::default();
        paint.set_color(color);

        for p in points {
            self.pixmap.fill_rect(
                Rect::from_xywh(
                    p.x as f32 * self.scale,
                    p.y as f32 * self.scale,
                    self.scale,
                    self.scale,
                )
                .unwrap(),
                &paint,
                Transform::identity(),
                None,
            );
        }
    }

    fn render(&self, buffer: &mut [u32]) {
        if buffer.is_empty() {
            return;
        }

        // AABBGGRR -> 00RRGGBB
        // for (i, p) in  frame.pixels().iter().enumerate() {
        //     let p = p.get() & 0x00FFFFFF;

        //     let mut b = (p & 0x00FF0000) >> 16;
        //     b |= p & 0x0000FF00;
        //     b |= (p & 0xFF) << 16;

        //     buffer[i] = b;
        // }

        // RRGGBBAA -> 00RRGGBB
        for (i, p) in self.pixmap.data().chunks(4).enumerate() {
            let b: u32 = ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32);
            buffer[i] = b;
        }

        // println!(
        //     "[0]: 0x{:08X} -> 0x{:08X}",
        //     self.pixmap.pixels()[0].get(),
        //     buffer[0]
        // );
    }

    fn clear(&mut self) {
        self.pixmap.fill(Color::from_rgba8(233, 233, 233, 255));
    }
}
