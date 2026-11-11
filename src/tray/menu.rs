use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use eframe::egui;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};

pub fn build_menu(
    ctx: eframe::egui::Context,
    quit_requested: Arc<AtomicBool>,
) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
    let open = MenuItem::with_id("open", "Open Featherweight", true, None);
    let hide = MenuItem::with_id("hide", "Hide Featherweight", true, None);
    let quit = MenuItem::with_id("quit", "Quit", true, None);

    let menu = Menu::new();

    menu.append_items(&[&open, &hide, &quit])?;

    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        match event.id.0.as_str() {
            "open" => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                ctx.request_repaint();
            }

            "hide" => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            }

            "quit" => {
                quit_requested.store(true, Ordering::SeqCst);

                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

            _ => {}
        }
    }));

    Ok(menu)
}