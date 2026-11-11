use eframe::egui_wgpu::{WgpuSetup, WgpuSetupCreateNew};
use eframe::{Renderer, UserEvent, WgpuConfiguration};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::log;

pub mod window;

const APP_NAME: &str = "Featherweight";

pub fn run() -> eframe::Result {
    log::log_init();
    log::log("Initializing Featherweight");

    // Renderers to try, in order. Glow goes first by default: it is the most
    // compatible backend on Windows, where wgpu's Vulkan path can crash with
    // an access violation on older Intel iGPU drivers. The wgpu fallback
    // forces the DX12 backend for the same reason.
    //
    // Set FTHWT_RENDERER=wgpu to reverse the order.
    let try_wgpu_first = std::env::var("FTHWT_RENDERER").as_deref() == Ok("wgpu");
    let attempts: [(&str, bool); 2] = if try_wgpu_first {
        [("wgpu", true), ("glow", false)]
    } else {
        [("glow", false), ("wgpu", true)]
    };

    let mut last_err: Option<eframe::Error> = None;
    for (name, use_wgpu) in attempts {
        log::log(&format!("Trying renderer '{name}'"));
        match try_run(use_wgpu) {
            Ok(()) => return Ok(()),
            Err(err) => {
                log::log(&format!("Renderer '{name}' failed: {err}"));
                last_err = Some(err);
            }
        }
    }

    Err(last_err.unwrap_or(eframe::Error::AppCreation(
        "No renderer could start".into(),
    )))
}

/// Build a fresh event loop + app and run it once with the requested renderer.
///
/// A fresh event loop per attempt is required: an event loop cannot safely be
/// reused after `run_app` has returned.
fn try_run(use_wgpu: bool) -> eframe::Result {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    log::log("Winit event loop created");

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title(APP_NAME)
            .with_inner_size([1000.0, 700.0])
            .with_visible(false),
        renderer: if use_wgpu {
            Renderer::Wgpu
        } else {
            Renderer::Glow
        },
        wgpu_options: wgpu_options(),
        ..Default::default()
    };

    let mut app = eframe::create_native(
        APP_NAME,
        native_options,
        Box::new(move |cc| {
            // The tray requires an egui context, which we get from the CreationContext.
            let tray = crate::tray::Tray::new(cc.egui_ctx.clone())?;

            Ok(Box::new(
                window::FeatherweightApp::new(cc, tray)
            ))
        }),
        &event_loop,
    );

    log::log("Starting Featherweight event loop");

    event_loop
        .run_app(&mut app)
        .map_err(eframe::Error::WinitEventLoop)
}

/// wgpu options that pin the DX12 backend on Windows.
///
/// wgpu otherwise prefers Vulkan, which some Intel iGPU drivers mishandle
/// (process dies with STATUS_ACCESS_VIOLATION during adapter init).
/// On other platforms the default configuration is used unchanged.
#[cfg(windows)]
fn wgpu_options() -> WgpuConfiguration {
    WgpuConfiguration {
        wgpu_setup: WgpuSetup::CreateNew(WgpuSetupCreateNew {
            instance_descriptor: eframe::wgpu::InstanceDescriptor {
                backends: eframe::wgpu::Backends::DX12,
                flags: eframe::wgpu::InstanceFlags::from_build_config().with_env(),
                backend_options: eframe::wgpu::BackendOptions::from_env_or_default(),
                memory_budget_thresholds: eframe::wgpu::MemoryBudgetThresholds::default(),
                display: None,
            },
            ..WgpuSetupCreateNew::without_display_handle()
        }),
        ..Default::default()
    }
}

#[cfg(not(windows))]
fn wgpu_options() -> WgpuConfiguration {
    WgpuConfiguration::default()
}
