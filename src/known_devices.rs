use std::ops::{Div, Sub};

pub const STEELSERIES_VID: u16 = 0x1038;

pub const REQUEST_BUFFER_SIZE: usize = 32;
pub const REPORT_BUFFER_SIZE: usize = 32;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum ArctisPID {
    Arctis7_2019,
    Arctis7_2017,
    ArctisPro_Wireless,
    ArctisPro_2019,
    ArctisPro_GameDac,
    Arctis1W,
    Arctis1X,
    Arctis7X,
    Arctis7P,
    Arctis9,
    Arctis7_Plus,
    Arctis7_Plus_Destiny,
    Arctis7P_Plus,
    Arctis7X_Plus,
    ArctisNova7,
    ArctisNova7X,
    ArctisNova7P,
}

impl Into<u16> for ArctisPID {
    fn into(self) -> u16 {
        match self {
            ArctisPID::Arctis7_2019 => 0x12ad,
            ArctisPID::Arctis7_2017 => 0x1260,
            ArctisPID::ArctisPro_Wireless => 0x1290,
            ArctisPID::ArctisPro_2019 => 0x1252,
            ArctisPID::ArctisPro_GameDac => 0x1280,
            ArctisPID::Arctis1W => 0x12b3,
            ArctisPID::Arctis1X => 0x12b6,
            ArctisPID::Arctis7X => 0x12d7,
            ArctisPID::Arctis7P => 0x12d5,
            ArctisPID::Arctis9 => 0x12c2,
            ArctisPID::Arctis7_Plus => 0x220e,
            ArctisPID::Arctis7_Plus_Destiny => 0x2236,
            ArctisPID::Arctis7P_Plus => 0x2212,
            ArctisPID::Arctis7X_Plus => 0x2216,
            ArctisPID::ArctisNova7 => 0x2202,
            ArctisPID::ArctisNova7X => 0x2206,
            ArctisPID::ArctisNova7P => 0x220a,
        }
    }
}

pub struct Device<'a> {
    pub name: &'a str,                   // Name of the device
    pub pid: ArctisPID,                  // Product ID of the device
    reportid: u8,                    // Report id for the HID write
    addr_battery: u8, // Address to query via HID write to get the current battery charge
    pub idx_battery: u8,  // Index of the battery charge in the returned report
    calculate_battery: fn(u8) -> u8, // calculates the battery percent for a given return value from the HID device
}

impl<'a> Device<'a> {
    pub fn generate_request(&self) -> [u8; REQUEST_BUFFER_SIZE] {
        [
            self.reportid,
            self.addr_battery,
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
        ]
    }

    pub fn read_battery_percent(&self, report: &[u8; REPORT_BUFFER_SIZE]) -> u8 {
        assert!(report.len() > self.idx_battery.into());
        (self.calculate_battery)(report[self.idx_battery as usize])
    }
}

pub const ARCTIS_DEVICES: [Device; 17] = [
    Device {
        name: "Arctis 7 (2019)",
        pid: ArctisPID::Arctis7_2019,
        reportid: 0x06,
        addr_battery: 0x18,
        idx_battery: 2, // -> 0-100 in %
        //addr_mute_status 0x30, // -> 0 not muted, 1 muted
        calculate_battery: |value| value,
    },
    Device {
        name: "Arctis 7 (2017)",
        pid: ArctisPID::Arctis7_2017,
        reportid: 0x06,
        addr_battery: 0x18,
        idx_battery: 2, // -> 0-100 in %
        calculate_battery: |value| value,
    },
    Device {
        name: "Arctis Pro Wireless",
        pid: ArctisPID::ArctisPro_Wireless,
        reportid: 0x40,
        addr_battery: 0xaa,
        idx_battery: 0, // -> 0-4
        calculate_battery: |value| scale(value.into(), 0_f32, 4_f32, 0_f32, 100_f32).round() as u8,
    },
    Device {
        name: "Arctis Pro 2019",
        pid: ArctisPID::ArctisPro_2019,
        reportid: 0x06,
        addr_battery: 0x18,
        idx_battery: 2, // -> unknown
        calculate_battery: |value| value,
    },
    Device {
        name: "Arctis Pro GameDac",
        pid: ArctisPID::ArctisPro_GameDac,
        reportid: 0x06,
        addr_battery: 0x18,
        idx_battery: 2, // -> unknown
        calculate_battery: |value| value,
    },
    Device {
        name: "Arctis 9",
        pid: ArctisPID::Arctis9,
        reportid: 0x00,
        addr_battery: 0x20,
        idx_battery: 4, // -> 0x64 - 0x9a
        calculate_battery: |value| scale(value.into(), 100_f32, 154_f32, 0_f32, 100_f32).round() as u8,
    },
    Device {
        name: "Arctis 1 Wireless",
        pid: ArctisPID::Arctis1W,
        reportid: 0x06,
        addr_battery: 0x12,
        idx_battery: 3, // -> 0-100 in %
        calculate_battery: |value| value,
    },
    Device {
        name: "Arctis 1 Xbox",
        pid: ArctisPID::Arctis1X,
        reportid: 0x06,
        addr_battery: 0x12,
        idx_battery: 3, // -> 0-100 in %
        calculate_battery: |value| value,
    },
    Device {
        name: "Arctis 7X",
        pid: ArctisPID::Arctis7X,
        reportid: 0x06,
        addr_battery: 0x12,
        idx_battery: 3, // -> 0-100 in %
        calculate_battery: |value| value,
    },
    Device {
        name: "Arctis 7P",
        pid: ArctisPID::Arctis7P,
        reportid: 0x06,
        addr_battery: 0x12,
        idx_battery: 3, // -> 0-100 in %
        calculate_battery: |value| value,
    },
    Device {
        name: "Arctis 7 Plus",
        pid: ArctisPID::Arctis7_Plus,
        reportid: 0x00,
        addr_battery: 0xb0,
        idx_battery: 2, // -> 0-4
        calculate_battery: |value| scale(value.into(), 0_f32, 4_f32, 0_f32, 100_f32).round() as u8,
    },
    Device {
        name: "Arctis 7 Destiny Plus",
        pid: ArctisPID::Arctis7_Plus_Destiny,
        reportid: 0x00,
        addr_battery: 0xb0,
        idx_battery: 3, // -> 0-4
        calculate_battery: |value| scale(value.into(), 0_f32, 4_f32, 0_f32, 100_f32).round() as u8,
    },
    Device {
        name: "Arctis 7P Plus",
        pid: ArctisPID::Arctis7P_Plus,
        reportid: 0x00,
        addr_battery: 0xb0,
        idx_battery: 2, // -> 0-4
        calculate_battery: |value| scale(value.into(), 0_f32, 4_f32, 0_f32, 100_f32).round() as u8,
    },
    Device {
        name: "Arctis 7X Plus",
        pid: ArctisPID::Arctis7X_Plus,
        reportid: 0x00,
        addr_battery: 0xb0,
        idx_battery: 2, // -> 0-4
        calculate_battery: |value| scale(value.into(), 0_f32, 4_f32, 0_f32, 100_f32).round() as u8,
    },
    Device {
        name: "Arctis Nova 7",
        pid: ArctisPID::ArctisNova7,
        reportid: 0x00,
        addr_battery: 0xb0,
        idx_battery: 2, // -> 0-4
        calculate_battery: |value| scale(value.into(), 0_f32, 4_f32, 0_f32, 100_f32).round() as u8,
    },
    Device {
        name: "Arctis Nova 7X",
        pid: ArctisPID::ArctisNova7X,
        reportid: 0x00,
        addr_battery: 0xb0,
        idx_battery: 2, // -> 0-4
        calculate_battery: |value| scale(value.into(), 0_f32, 4_f32, 0_f32, 100_f32).round() as u8,
    },
    Device {
        name: "Arctis Nova 7P",
        pid: ArctisPID::ArctisNova7P,
        reportid: 0x00,
        addr_battery: 0xb0,
        idx_battery: 2, // -> 0-4
        calculate_battery: |value| scale(value.into(), 0_f32, 4_f32, 0_f32, 100_f32).round() as u8,
    },
];

fn scale(value: f32, min_before: f32, max_before: f32, min_after: f32, max_after: f32) -> f32 {
    assert!(min_before <= max_before);
    assert!(min_after <= max_after);
    value
        .clamp(min_before, max_before)
        .sub(min_before)
        .div(max_before - min_before)
        .mul_add(max_after - min_after, min_after)
}
