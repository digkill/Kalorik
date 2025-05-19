use image::{ImageBuffer, Rgb, RgbImage};
use rusttype::{Font, Scale, point, PositionedGlyph};

pub fn draw_weekly_calories_chart(
    data: &[(String, f32)],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let width: u32 = 800;
    let height: u32 = 400;
    let margin: u32 = 50;
    let spacing: u32 = 20;
    let bar_width: u32 = 40;

    // Calculate the maximum value for scaling the bars
    let max_val = data
        .iter()
        .map(|(_, v)| *v)
        .fold(0.0f32, f32::max)
        .max(1.0); // Avoid division by zero

    let mut img: RgbImage = ImageBuffer::from_pixel(width, height, Rgb([255, 255, 255]));

    // Load embedded font with better error handling
    let font_data = include_bytes!("../../assets/DejaVuSans.ttf");
    let font = Font::try_from_vec(font_data.to_vec())
        .ok_or_else(|| {
            log::error!("Failed to load font from ../../assets/DejaVuSans.ttf");
            "Failed to load font"
        })?;
    let scale = Scale::uniform(18.0);

    // Draw Y-axis labels and grid lines
    let num_grid_lines = 5;
    for i in 0..=num_grid_lines {
        let y = margin + (i * (height - 2 * margin) / num_grid_lines);
        let calorie_value = max_val * (1.0 - i as f32 / num_grid_lines as f32);

        // Draw grid line
        for x in margin..width {
            img.put_pixel(x, y, Rgb([200, 200, 200]));
        }

        // Draw Y-axis label
        let label = format!("{:.0}", calorie_value);
        draw_text(&mut img, &label, 5, y - 10, scale, &font, Rgb([0, 0, 0]));
    }

    // Draw bars and X-axis labels
    for (i, (label, value)) in data.iter().enumerate() {
        let i = i as u32;
        let bar_height = ((value / max_val) * (height - 2 * margin) as f32) as u32;
        let x = margin + i * (bar_width + spacing);
        let y = height - bar_height - margin;

        // Draw bar
        for dx in 0..bar_width {
            for dy in 0..bar_height {
                img.put_pixel(x + dx, y + dy, Rgb([100, 149, 237]));
            }
        }

        // Draw label (day name) below the bar
        let label_x = x + (bar_width / 2) - (label.len() as u32 * 5); // Center the label
        let label_y = height - margin + 15; // Move label below the bar
        draw_text(&mut img, label, label_x, label_y, scale, &font, Rgb([0, 0, 0]));
    }

    // Draw title
    let title = "Weekly Calorie Intake";
    let title_scale = Scale::uniform(26.0);
    let title_x = (width / 2) - (title.len() as u32 * 6); // Center the title
    draw_text(&mut img, title, title_x, 20, title_scale, &font, Rgb([0, 0, 0]));

    // Save the image
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
    let v_metrics = font.v_metrics(scale);
    let offset = point(x as f32, y as f32 + v_metrics.ascent);

    // Layout the glyphs
    let glyphs: Vec<PositionedGlyph<'_>> = font.layout(text, scale, offset).collect();

    // Render each glyph
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|gx, gy, gv| {
                let px = gx as i32 + bb.min.x;
                let py = gy as i32 + bb.min.y;
                // Ensure the pixel is within bounds
                if px >= 0 && px < image.width() as i32 && py >= 0 && py < image.height() as i32 {
                    let px = px as u32;
                    let py = py as u32;
                    let alpha = (gv * 255.0) as u8;
                    let pixel = image.get_pixel_mut(px, py);
                    // Blend the color with the background (white)
                    let bg = pixel.0;
                    let r = (color[0] as f32 * gv + bg[0] as f32 * (1.0 - gv)) as u8;
                    let g = (color[1] as f32 * gv + bg[1] as f32 * (1.0 - gv)) as u8;
                    let b = (color[2] as f32 * gv + bg[2] as f32 * (1.0 - gv)) as u8;
                    *pixel = Rgb([r, g, b]);
                }
            });
        }
    }
}