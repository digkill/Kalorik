use image::{ImageBuffer, Rgb, RgbImage};
use rusttype::{Font, Scale};

pub fn draw_weekly_calories_chart(
    data: &[(String, f32)],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let width: u32 = 800;
    let height: u32 = 400;
    let margin: u32 = 50;
    let spacing: u32 = 20;
    let bar_width: u32 = 40;

    let max_val = data.iter().map(|(_, v)| *v).fold(0.0f32, f32::max);

    let mut img: RgbImage = ImageBuffer::from_pixel(width, height, Rgb([255, 255, 255]));

    // Load embedded font
    let font_data = include_bytes!("../../assets/Roboto-Regular.ttf");
    let font = Font::try_from_vec(font_data.to_vec()).ok_or("Failed to load font")?;
    let scale = Scale::uniform(18.0);

    for (i, (label, value)) in data.iter().enumerate() {
        let i = i as u32; // Convert usize to u32
        let bar_height = ((value / max_val) * (height - 2 * margin) as f32) as u32;
        let x = margin + i * (bar_width + spacing);
        let y = height - bar_height - margin;

        // Draw rectangle
        for dx in 0..bar_width {
            for dy in 0..bar_height {
                img.put_pixel(x + dx, y + dy, Rgb([100, 149, 237]));
            }
        }

        // Draw label (day name)
        draw_text(&mut img, label, x, height - margin + 5, scale, &font, Rgb([0, 0, 0]));
    }

    // Draw title
    draw_text(&mut img, "📊 Weekly Calorie Intake", 200, 10, Scale::uniform(26.0), &font, Rgb([0, 0, 0]));

    img.save(output_path)?;
    Ok(())
}

fn draw_text(
    image: &mut RgbImage,
    text: &str,
    x: u32,
    y: u32,
    scale: Scale,
    font: &rusttype::Font,
    color: Rgb<u8>,
) {
    use rusttype::{point, PositionedGlyph};

    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<PositionedGlyph<'_>> = font
        .layout(text, scale, point(x as f32, y as f32 + v_metrics.ascent))
        .collect();

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|gx, gy, gv| {
                let px = gx + bb.min.x as u32;
                let py = gy + bb.min.y as u32;
                if px < image.width() && py < image.height() {
                    let alpha = (gv * 255.0) as u8;
                    let pixel = image.get_pixel_mut(px, py);
                    *pixel = Rgb([
                        (color[0] as u16 * alpha as u16 / 255) as u8,
                        (color[1] as u16 * alpha as u16 / 255) as u8,
                        (color[2] as u16 * alpha as u16 / 255) as u8,
                    ]);
                }
            });
        }
    }
}