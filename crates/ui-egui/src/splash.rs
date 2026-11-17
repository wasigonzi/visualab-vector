//! Startup splash (VL Vector), the same Visualab Studios loading screen as VL Studios: a small
//! centred card with the brand artwork (the Visualab logo pulsing with an accent glow, morphing
//! colour blobs, a slow pan), a status line that crossfades through loading steps, and a
//! shimmering accent progress bar.

use egui::{Color32, Id, Pos2, Rect, TextureOptions, pos2, vec2};

const BG_PNG: &[u8] = include_bytes!("../../../assets/brand/visualab-splash-bg.png");
const LOGO_PNG: &[u8] = include_bytes!("../../../assets/brand/visualab-splash-logo.png");
const FADE_IN: f64 = 0.35;
const HOLD: f64 = 2.8;
const FADE: f64 = 0.4;
const ACCENT: Color32 = Color32::from_rgb(0x00, 0xD4, 0xFF);
const STEPS: [&str; 6] = [
    "Inicializando\u{2026}",
    "Cargando herramientas\u{2026}",
    "Leyendo trazados\u{2026}",
    "Cargando fuentes\u{2026}",
    "Preparando lienzo\u{2026}",
    "Listo",
];

/// A PNG decoded once per context into a mipmapped texture.
pub(crate) fn texture(ctx: &egui::Context, name: &'static str, png: &[u8]) -> egui::TextureHandle {
    let id = Id::unique(name);
    if let Some(t) = ctx.data(|d| d.get_temp::<egui::TextureHandle>(id)) {
        return t;
    }
    let img = image::load_from_memory_with_format(png, image::ImageFormat::Png)
        .ok()
        .map(|i| {
            let i = i.to_rgba8();
            egui::ColorImage::from_rgba_unmultiplied([i.width() as usize, i.height() as usize], i.as_raw())
        })
        .unwrap_or_else(|| egui::ColorImage::filled([1, 1], Color32::TRANSPARENT));
    let opts = TextureOptions { mipmap_mode: Some(egui::TextureFilter::Linear), ..TextureOptions::LINEAR };
    let t = ctx.load_texture(name, img, opts);
    ctx.data_mut(|d| d.insert_temp(id, t.clone()));
    t
}

/// Paint the splash over everything. Returns true while it covers the app.
pub fn paint(ctx: &egui::Context) -> bool {
    if std::env::var_os("VL_NO_SPLASH").is_some() {
        return false;
    }
    let start_id = Id::unique("vl-splash-start");
    let now = ctx.input(|i| i.time);
    let start = ctx.data(|d| d.get_temp::<f64>(start_id)).unwrap_or_else(|| {
        ctx.data_mut(|d| d.insert_temp(start_id, now));
        now
    });
    let t = now - start;
    if t >= HOLD + FADE {
        return false;
    }
    let alpha = if t < FADE_IN {
        (t / FADE_IN) as f32
    } else if t < HOLD {
        1.0
    } else {
        (1.0 - (t - HOLD) / FADE) as f32
    };
    let tf = t as f32;
    let a = |c: Color32| c.gamma_multiply(alpha);
    let screen = ctx.content_rect();
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Debug, Id::unique("vl-splash")));
    // The app behind stays hidden until the card fades out.
    let back = if t < HOLD { 1.0 } else { alpha };
    painter.rect_filled(screen, 0.0, Color32::from_rgb(0x0B, 0x0B, 0x0C).gamma_multiply(back));
    let w = (screen.width() - 40.0).min(600.0);
    // Window "rises" slightly while fading in.
    let rise = (1.0 - alpha.min(1.0)) * 12.0;
    let card = Rect::from_center_size(screen.center() + vec2(0.0, rise), vec2(w, w * 0.5 + 80.0));
    let r = egui::CornerRadius::same(16);
    painter.rect_filled(card.expand(16.0), 30.0, a(Color32::from_black_alpha(80)));
    painter.rect_filled(card, r, a(Color32::from_rgb(0x14, 0x14, 0x16)));
    let art = Rect::from_min_size(card.min, vec2(w, w * 0.5));
    let full = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    // Slow pan of the background art.
    let pan = (tf * 0.25).sin() * 0.02;
    let uv = Rect::from_min_max(pos2(0.02 + pan, 0.02), pos2(0.98 + pan, 0.98));
    painter.add(
        egui::epaint::RectShape::filled(art, egui::CornerRadius { nw: 16, ne: 16, sw: 0, se: 0 }, a(Color32::WHITE))
            .with_texture(texture(ctx, "vl-splash-bg", BG_PNG).id(), uv),
    );
    let clip = painter.with_clip_rect(art);
    // Morphing blobs (lava-lamp): soft layered shapes whose outline wobbles over time.
    let blobs: [(Color32, f32, f32, f32, f32); 5] = [
        (Color32::from_rgb(0x00, 0xD4, 0xFF), 0.30, 0.55, 0.30, 0.0),
        (Color32::from_rgb(0x1E, 0x3A, 0xFF), 0.62, 0.40, 0.34, 1.7),
        (Color32::from_rgb(0x7B, 0x3C, 0xFF), 0.82, 0.70, 0.28, 3.1),
        (Color32::from_rgb(0x00, 0x8C, 0xC8), 0.50, 0.85, 0.24, 4.4),
        (Color32::from_rgb(0x4A, 0x2A, 0xC8), 0.15, 0.20, 0.22, 5.6),
    ];
    for (bi, (col, bx, by, br, ph)) in blobs.iter().enumerate() {
        let sp = 0.18 + bi as f32 * 0.04;
        let c = pos2(
            art.left() + (bx + 0.12 * (tf * sp + ph).sin()) * art.width(),
            art.top() + (by + 0.16 * (tf * sp * 1.3 + ph * 0.7).cos()) * art.height(),
        );
        let base = br * art.height();
        for layer in 0..14 {
            let k = layer as f32 / 13.0;
            let rad = base * (1.25 - k * 0.85);
            let n = 40;
            let pts: Vec<Pos2> = (0..n)
                .map(|q| {
                    let ang = q as f32 / n as f32 * std::f32::consts::TAU;
                    let wob = 1.0 + 0.16 * (ang * 2.0 + tf * 0.9 + ph).sin() + 0.09 * (ang * 3.0 - tf * 1.3 + ph * 2.0).cos();
                    c + vec2(ang.cos(), ang.sin()) * rad * wob
                })
                .collect();
            clip.add(egui::Shape::convex_polygon(pts, a(col.gamma_multiply(0.012 + 0.008 * k)), egui::Stroke::NONE));
        }
    }
    // Darken a little so the logo reads.
    {
        let mut mesh = egui::Mesh::default();
        let dark = a(Color32::from_black_alpha(190));
        let clear = Color32::TRANSPARENT;
        let m = art.left() + art.width() * 0.75;
        mesh.colored_vertex(art.left_top(), dark);
        mesh.colored_vertex(pos2(m, art.top()), clear);
        mesh.colored_vertex(pos2(m, art.bottom()), clear);
        mesh.colored_vertex(art.left_bottom(), dark);
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(0, 2, 3);
        clip.add(mesh);
    }
    // Pulsing logo (full mark with wordmark) with a soft accent glow.
    let pulse = 0.5 + 0.5 * (tf * 2.4).sin();
    let tex = texture(ctx, "vl-splash-logo", LOGO_PNG);
    let [tw, th] = tex.size();
    let lw = art.width() * 0.46 * (1.0 + 0.02 * pulse);
    let lh = lw * th as f32 / tw.max(1) as f32;
    let lc: Pos2 = pos2(art.left() + art.width() * 0.29, art.center().y);
    for g in 1..=8 {
        let gr = lh * 0.6 + g as f32 * 7.0;
        clip.circle_filled(lc - vec2(lw * 0.3, 0.0), gr, a(ACCENT.gamma_multiply((0.02 + 0.02 * pulse) / g as f32 * 2.0)));
    }
    clip.image(tex.id(), Rect::from_center_size(lc, vec2(lw, lh)), full, a(Color32::WHITE));
    // Mask the clipped overlays back to the rounded top corners.
    let behind = Color32::from_rgb(0x07, 0x07, 0x08).gamma_multiply(back);
    for (corner, sx) in [(art.left_top(), 1.0f32), (art.right_top(), -1.0f32)] {
        let rad = 16.0;
        let c = corner + vec2(sx * rad, rad);
        for q in 0..8 {
            let a0 = q as f32 / 8.0 * std::f32::consts::FRAC_PI_2;
            let a1 = (q + 1) as f32 / 8.0 * std::f32::consts::FRAC_PI_2;
            let p0 = c + vec2(-sx * a0.cos(), -a0.sin()) * rad;
            let p1 = c + vec2(-sx * a1.cos(), -a1.sin()) * rad;
            painter.add(egui::Shape::convex_polygon(vec![corner, p0, p1], behind, egui::Stroke::NONE));
        }
    }
    painter.rect_stroke(card, r, egui::Stroke::new(1.0, a(Color32::from_rgb(0x2E, 0x2E, 0x33))), egui::StrokeKind::Inside);
    painter.line_segment([art.left_bottom(), art.right_bottom()], egui::Stroke::new(1.0, a(ACCENT.gamma_multiply(0.35))));
    // Status text crossfade.
    let pos = ((t / HOLD) * 5.0).clamp(0.0, 5.0);
    let step = pos.floor() as usize;
    let frac = (pos - pos.floor()) as f32;
    let x = card.left() + 22.0;
    let ty = art.bottom() + 28.0;
    let font = egui::FontId::proportional(13.0);
    let text_c = Color32::from_rgb(0xB8, 0xB8, 0xC0);
    let fade = ((frac - 0.8) / 0.2).clamp(0.0, 1.0);
    painter.text(pos2(x, ty - fade * 6.0), egui::Align2::LEFT_CENTER, STEPS[step], font.clone(), a(text_c.gamma_multiply(1.0 - fade)));
    if step < 5 && fade > 0.0 {
        painter.text(pos2(x, ty + (1.0 - fade) * 6.0), egui::Align2::LEFT_CENTER, STEPS[step + 1], font, a(text_c.gamma_multiply(fade)));
    }
    // The product name, bottom-right over the artwork.
    clip.text(
        art.right_bottom() + vec2(-14.0, -12.0),
        egui::Align2::RIGHT_BOTTOM,
        "VL Vector",
        egui::FontId::proportional(15.0),
        a(Color32::WHITE.gamma_multiply(0.92)),
    );
    // Progress bar with shimmer.
    let bar = Rect::from_min_size(pos2(x, art.bottom() + 52.0), vec2(w - 44.0, 3.0));
    painter.rect_filled(bar, 1.5, a(Color32::from_rgb(0x26, 0x26, 0x2A)));
    let p = 1.0 - (1.0 - (t / HOLD).min(1.0) as f32).powi(2);
    let fill = Rect::from_min_size(bar.min, vec2(bar.width() * p, 3.0));
    painter.rect_filled(fill, 1.5, a(ACCENT));
    let shx = fill.left() + ((tf * 0.9) % 1.0) * fill.width();
    let sh = painter.with_clip_rect(fill);
    for k in 0..10 {
        let o = 1.0 - (k as f32 / 10.0);
        sh.rect_filled(
            Rect::from_center_size(pos2(shx, bar.center().y), vec2(4.0 + k as f32 * 6.0, 3.0)),
            1.5,
            a(Color32::from_white_alpha((o * 22.0) as u8)),
        );
    }
    ctx.request_repaint();
    if t < HOLD {
        egui::Area::new(Id::unique("vl-splash-block")).order(egui::Order::Debug).fixed_pos(screen.min).show(ctx, |ui| {
            ui.allocate_rect(Rect::from_min_size(screen.min, screen.size()), egui::Sense::click_and_drag());
        });
    }
    t < HOLD
}

#[cfg(test)]
mod tests {
    #[test]
    fn brand_art_decodes() {
        let ctx = egui::Context::default();
        for (name, png) in [("t-bg", super::BG_PNG), ("t-logo", super::LOGO_PNG)] {
            let t = super::texture(&ctx, name, png);
            assert!(t.size()[0] > 1 && t.size()[1] > 1, "{name}");
        }
    }
}
