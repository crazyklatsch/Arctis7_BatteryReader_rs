extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("headset_battery_icons\\Arctis7_BatteryReader.ico");
    res.compile().unwrap();
  }
}