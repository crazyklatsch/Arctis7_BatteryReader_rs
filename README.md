# Arctis Battery Reader
This is a battery reader project I started for my Steelseries Arctis7 Headset.
It polls the current battery percent of a connected Arctis Headset every couple of seconds and displays it as a trayicon.
CPU usage is next to none and it needs about 1.5MB of RAM.

Currently its only target OS is windows, but all dependencies are multiplatform, so probably it's not very hard to change the code for multiplatform (since I don't need it, I spent no time trying).

### Screenshots:
Headset not connected:

![headset_not_connected](https://github.com/crazyklatsch/Arctis7_BatteryReader_rs/assets/70517887/b77cba29-4d25-4492-9e2c-44be4aa3acb1)

Headset connected:

![headset_connected](https://github.com/crazyklatsch/Arctis7_BatteryReader_rs/assets/70517887/0aedb6ad-1b7e-4334-b11a-31faa6717689)

# Currently known devices

**Disclaimer:**
I cannot test the program for any headsets except for the Arctis 7 (2019). Any feedback on working/not working devices from the list is welcome.

- Arctis 7 (2019) <span style="color:green"> -> tested</span>
- Arctis 7 (2017) <span style="color:red">  -> not tested</span>
- Arctis Pro Wireless <span style="color:red">  -> not tested</span>
- Arctis Pro (2019) <span style="color:red">  -> not tested</span>
- Arctis Pro GameDac <span style="color:red">  -> not tested</span>
- Arctis 1 Wireless <span style="color:red">  -> not tested</span>
- Arctis 1 X <span style="color:red">  -> not tested</span>
- Arctis 7 X <span style="color:red">  -> not tested</span>
- Arctis 7 P <span style="color:red">  -> not tested</span>
- Arctis 9 <span style="color:red">  -> not tested</span>
- Arctis 7 Plus <span style="color:red">  -> not tested</span>
- Arctis 7 Plus Destiny <span style="color:red">  -> not tested</span>
- Arctis 7 P Plus <span style="color:red">  -> not tested</span>
- Arctis 7 X Plus <span style="color:red">  -> not tested</span>
- Arctis Nova 7 <span style="color:yellow">  -> apparently works when compiled from source</span>
- Arctis Nova 7 X <span style="color:red">  -> not tested</span>
- Arctis Nova 7 P <span style="color:red">  -> not tested</span>

# Build & Install manually (using Rust)
Clone the repository and 
```
cargo build -r
```
then copy the "\<repo-dir\>/target/release/arctis7-batteryreader-rs.exe" file and the "\<repo-dir\>/headset_battery_icons" folder to any desired destination and run the .exe

# Install (Windows)
Download and execute the "Arctis7_BatteryReader_Installer.msi" from the latest release.
It is a default install wizard that will copy the .exe and the icons to the selected program folder and add the program to autostart. The program can be removed using the regular Windows "Add or Remove programms" settings.
