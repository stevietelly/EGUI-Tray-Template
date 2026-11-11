pub mod menu;

use std::error::Error;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

pub struct Tray {
    // Keeping this field alive keeps the system tray icon alive.
    _icon: TrayIcon,
    quit_requested: Arc<AtomicBool>,
}

impl Tray {
    pub fn new(
        ctx: eframe::egui::Context,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let quit_requested = Arc::new(AtomicBool::new(false));

        let menu = menu::build_menu(
            ctx,
            quit_requested.clone(),
        )?;

        let icon = create_placeholder_icon()?;

        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("Featherweight")
            .with_menu(Box::new(menu))
            .with_icon(icon)
            .build()?;

        Ok(Self {
            _icon: tray_icon,
            quit_requested,
        })
    }

    pub fn is_quit_requested(&self) -> bool {
        self.quit_requested.load(Ordering::SeqCst)
    }
}

// Temporary icon until we connect the final Featherweight icon asset.
fn create_placeholder_icon() -> Result<Icon, tray_icon::BadIcon> {
    const SIZE: u32 = 32;

    let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];

    let navy = [13, 20, 38, 255];
    let white = [245, 247, 250, 255];
    let red = [235, 55, 65, 255];

    draw_disc(&mut pixels, 16.0, 16.0, 15.0, navy);

    // Feather shaft and barbs.
    draw_line(&mut pixels, (9.0, 24.0), (23.0, 8.0), 1.5, white);

    draw_line(&mut pixels, (12.0, 21.0), (8.0, 15.0), 1.3, white);
    draw_line(&mut pixels, (15.0, 18.0), (10.0, 12.0), 1.3, white);
    draw_line(&mut pixels, (18.0, 15.0), (14.0, 10.0), 1.3, white);

    // Recording indicator.
    draw_disc(&mut pixels, 23.0, 8.0, 3.0, red);

    Icon::from_rgba(pixels, SIZE, SIZE)
}

fn draw_line(
    pixels: &mut [u8],
    start: (f32, f32),
    end: (f32, f32),
    thickness: f32,
    color: [u8; 4],
) {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let steps = dx.abs().max(dy.abs()).ceil() as usize;

    for i in 0..=steps {
        let t = if steps == 0 {
            0.0
        } else {
            i as f32 / steps as f32
        };

        draw_disc(pixels, start.0 + dx * t, start.1 + dy * t, thickness, color);
    }
}

fn draw_disc(pixels: &mut [u8], cx: f32, cy: f32, radius: f32, color: [u8; 4]) {
    const SIZE: i32 = 32;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;

            if dx * dx + dy * dy <= radius * radius {
                let index = ((y * SIZE + x) * 4) as usize;
                pixels[index..index + 4].copy_from_slice(&color);
            }
        }
    }
}