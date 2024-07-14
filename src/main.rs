//! arctis7 battery reader (by crazyklatsch)
//! Reads the battery percent of a connected steelseries arctis7 headset and displays it as a tray-icon
//!

#![windows_subsystem = "windows"]

use core::fmt;
use std::{process::exit, time::Duration, usize};

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItemBuilder},
    TrayIconBuilder,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use std::{convert::TryInto, path::Path};

use hidapi::HidApi;

mod known_devices;
use crate::known_devices::*;

const READ_INTERVAL_SEC: u64 = 2;
const READ_TIMEOUT_MS: i32 = 100;

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
        .with_tooltip("Steelseries battery reader")
        .with_icon(icons[0].clone())
        .with_title("x")
        .build()
        .unwrap();

    let _ = event_loop.run(move |event, event_loop| {
        event_loop.set_control_flow(ControlFlow::wait_duration(Duration::from_secs(
            READ_INTERVAL_SEC,
        )));

        let mut update_required = false;

        if let winit::event::Event::NewEvents(winit::event::StartCause::ResumeTimeReached {
            start,
            requested_resume,
        }) = event
        {
            update_required = true;
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id() == item_refresh.id() {
                update_required = true;
            } else if event.id() == item_quit.id() {
                exit(0);
            }
        }

        if update_required {
            let (battery_percent, name) = read_battery();
            let _ = tray_icon.set_icon(Some(
                icons[Into::<usize>::into(battery_percent).clamp(0, 100)].clone(),
            ));
            let _ = tray_icon.set_tooltip(Some(format!("{}", name)));
        }
    });
}

/// Reads the current battery percent of the first responding hid device with a valid vid and pid.
/// If no responding device is found, 0 is returned.
fn read_battery() -> (u8, &'static str) {
    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.vendor_id() == STEELSERIES_VID {
                    for known_device in ARCTIS_DEVICES {
                        if device.product_id() == known_device.pid.into()
                        {
                            if let Ok(dev) = device.open_device(&api) {
                                let _ = dev.write(&known_device.generate_request());
                                let mut buffer: [u8; REPORT_BUFFER_SIZE] = [0; REPORT_BUFFER_SIZE];
                                if let Ok(numbytesread) = dev.read_timeout(&mut buffer, READ_TIMEOUT_MS)
                                {
                                    if numbytesread >= known_device.idx_battery.into()
                                    {
                                        return (known_device.read_battery_percent(&buffer).clamp(0, 100), known_device.name)
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
                
            }
        }
        Err(e) => {
            return (0, "Error in HidApi")
        }
    }
    (0, "No headset found")
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

fn load_icons() -> [tray_icon::Icon; 101] {
    let mut icons: Vec<tray_icon::Icon> = Vec::new();
    icons.reserve_exact(101);

    for i in 0..101 {
        let location = format!(
            ".\\headset_battery_icons\\pngs\\{}.png",
            //env!("CARGO_MANIFEST_DIR"),
            i
        );
        let path = Path::new(location.as_str());
        icons.push(load_icon(path));
    }

    return icons.try_into().unwrap_or_else(|v: Vec<tray_icon::Icon>| {
        panic!("Expected a Vec of length {} but it was {}", 101, v.len())
    });
}
