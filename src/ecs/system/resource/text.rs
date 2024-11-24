use ab_glyph::{Font, FontVec, Glyph, OutlinedGlyph, Point, PxScale, Rect, ScaleFont};
use image::{DynamicImage, ImageBuffer, Rgba};

pub fn to_image(text: &str, font: &FontVec, font_size: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let scale = PxScale::from(font_size);
    let scaled_font = font.as_scaled(scale);

    let glyphs = gather_glyphs(scaled_font, text);

    let (outlined, px_bounds) = get_glyph_outlines(glyphs, font);

    create_image_from_gylphs(outlined, px_bounds)
}

fn gather_glyphs<F, SF>(font: SF, text: &str) -> Vec<Glyph>
where
    F: Font,
    SF: ScaleFont<F>,
{
    // https://github.com/alexheretic/ab-glyph/blob/main/dev/src/layout.rs#L7

    let mut glyphs: Vec<Glyph> = Vec::with_capacity(text.len());
    let mut last_glyph: Option<Glyph> = None;
    let mut caret = Point {
        x: 0.0,
        y: font.ascent(),
    };

    for char in text.chars() {
        let mut glyph = font.scaled_glyph(char);

        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        // line breaks?

        glyphs.push(glyph);
    }

    glyphs
}

fn get_glyph_outlines(glyphs: Vec<Glyph>, font: &FontVec) -> (Vec<OutlinedGlyph>, Rect) {
    let outlined: Vec<OutlinedGlyph> = glyphs
        .into_iter()
        .filter_map(|glyph| font.outline_glyph(glyph))
        .collect();
    let Some(px_bounds) = outlined
        .iter()
        .map(OutlinedGlyph::px_bounds)
        .reduce(|b, next| Rect {
            min: Point {
                x: b.min.x.min(next.min.x),
                y: b.min.y.min(next.min.y),
            },
            max: Point {
                x: b.max.x.max(next.max.x),
                y: b.max.y.max(next.max.y),
            },
        })
    else {
        panic!("no outlines?")
    };

    (outlined, px_bounds)
}

fn create_image_from_gylphs(
    glyphs: Vec<OutlinedGlyph>,
    boundary: Rect,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image =
        DynamicImage::new_rgba8(boundary.width() as u32, boundary.height() as u32).to_rgba8();

    for glyph in glyphs {
        let bounds = glyph.px_bounds();

        let img_left = bounds.min.x as u32 - boundary.min.x as u32;
        let img_top = bounds.min.y as u32 - boundary.min.y as u32;

        glyph.draw(|x, y, v| {
            let px = image.get_pixel_mut(img_left + x, img_top + y);

            *px = Rgba([255, 255, 255, px.0[3].saturating_add((v * 255.0) as u8)]);
        });
    }

    image
}
