# Arctis7 Battery Reader
This is a battery reader for the Steelseries Arctis7 Headset.
It polls the current battery percent of a connected Arctis7 Headset every couple of seconds and displays it as a trayicon.
CPU usage is next to none and it needs about 1.5MB of RAM.
Currently its only target OS is windows, but all dependencies are multiplatform, so it's likely not very hard to change the code for multiplatform (since I don't need it, I spent no time trying).

# Build & Install manually (using Rust)
Clone the repository and 
```
cargo build -r
```
then copy the "\<repo-dir\>/target/release/arctis7-batteryreader-rs.exe" file and the "\<repo-dir\>/headset_battery_icons" folder to any desired destination and run the .exe

# Install (Windows)
Download and execute the "Arctis7_BatteryReader_Installer.msi" from the latest release.
