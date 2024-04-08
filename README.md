# Arctis7 Battery Reader
This is a battery reader for the Steelseries Arctis7 Headset.
It polls the current battery percent of a connected Arctis7 Headset every couple of seconds and displays it as a trayicon.
CPU usage is next to none and it needs about 1.5MB of RAM.

# Build (using Rust)
Clone the repository and 
```
cargo build -r
```
then copy the "\<repo-dir\>/target/release/arctis7-batteryreader-rs.exe" file and the "\<repo-dir\>/headset_battery_icons" folder to any desired destination and run the .exe
