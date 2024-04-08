//! arctis7 battery reader (by crazyklatsch)
//! Reads the battery percent of a connected steelseries arctis7 headset and displays it as a tray-icon
//!

#![windows_subsystem = "windows"]

use std::{process::exit, time::Duration};

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, MenuItemBuilder}, TrayIcon, TrayIconBuilder
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use std::{convert::TryInto, path::Path};


use hidapi::HidApi;

const READ_INTERVAL_SEC: u64 = 2;
const READ_TIMEOUT_MS: i32 = 100;
const READ_BUFFER_SIZE: usize = 32;

const REPORT_ID: u8 = 0x06;
const ARCTIS7_VID: u16 = 0x1038;
const ARCTIS7_PID: u16 = 0x12ad;
const ARCTIS7_ADDRESS_BATTERY: u8 = 0x18; // -> 0-100 in %
const _ARCTIS7_ADDRESS_MUTE_STATUS: u8 = 0x30; // -> 0 not muted, 1 muted

const ARCTIS7_BATTERY_REQUEST: [u8; 32] = [
    REPORT_ID,
    ARCTIS7_ADDRESS_BATTERY,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

fn main() {
    let icons = load_icons();

    let event_loop = EventLoopBuilder::new().build().unwrap();
    let menu_channel = MenuEvent::receiver();

    let item_refresh = MenuItemBuilder::new().text("Refresh").enabled(true).build();
    let item_quit = MenuItemBuilder::new().text("Quit").enabled(true).build();
    let menu = Menu::new();
    let _ = menu.append(&item_refresh);
    let _ = menu.append(&item_quit);

    let tray_icon = TrayIconBuilder::new()
    .with_menu(Box::new(menu.clone()))
    .with_tooltip("Arctis7 battery reader")
    .with_icon(icons[0].clone())
    .with_title("x")
    .build()
    .unwrap();

    let _ = event_loop.run(move |event, event_loop| {
        event_loop.set_control_flow(ControlFlow::wait_duration(Duration::from_secs(READ_INTERVAL_SEC)));

        if let winit::event::Event::NewEvents(winit::event::StartCause::ResumeTimeReached { start, requested_resume }) = event {
            let battery_percent: usize = read_arctis7_battery().into();
            let _ = tray_icon.set_icon(Some(icons[battery_percent.clamp(0, 100)].clone()));
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id() == item_refresh.id() {
                let battery_percent: usize = read_arctis7_battery().into();
                let _ = tray_icon.set_icon(Some(icons[battery_percent.clamp(0, 100)].clone()));
            }
            else if event.id() == item_quit.id() {
                exit(0);
            }
        }
    });
}

/// Reads the current battery percent of the first responding hid device with arctis7 vid and pid.
/// If no responding arctis7 device is found, 0 is returned.
fn read_arctis7_battery() -> u8 {
    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.vendor_id() == ARCTIS7_VID && device.product_id() == ARCTIS7_PID {
                    if let Ok(dev) = device.open_device(&api) {
                        let _ = dev.write(&ARCTIS7_BATTERY_REQUEST);
                        let mut buffer: [u8; READ_BUFFER_SIZE] = [0; READ_BUFFER_SIZE];
                        if let Ok(numbytesread) = dev.read_timeout(&mut buffer, READ_TIMEOUT_MS) {
                            if numbytesread >= 3
                                && buffer[0] == REPORT_ID
                                && buffer[1] == ARCTIS7_ADDRESS_BATTERY
                            {
                                return buffer[2];
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    0
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn load_icons() -> [tray_icon::Icon; 100] {
    let mut icons: Vec<tray_icon::Icon> = Vec::new();
    icons.reserve_exact(100);

    for i in 0..100 {
        let location = format!(
            ".\\headset_battery_icons\\pngs\\{}.png",
            //env!("CARGO_MANIFEST_DIR"),
            i
        );
        let path = Path::new(location.as_str());
        icons.push(load_icon(path));
    }

    return icons.try_into().unwrap_or_else(|v: Vec<tray_icon::Icon>| {
        panic!("Expected a Vec of length {} but it was {}", 100, v.len())
    });
}

