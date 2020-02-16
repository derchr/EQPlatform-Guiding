# EQPlatform-PulseGuiding
This is a private project, that I am currently working on. The project contains the firmware for an AVR microcontroller and an INDI driver. The firmware controls the homemade equatorial platform and the INDI driver is used to let the µC communicate with guiding tools such as Ekos (Kstars) or PHD2.
It should also work with various other homemade mounts (eg. barn door trackers...)
### Building
#### AVR Hex File
To build the microcontroller hex file, navigate to the directory containing the Makefile.
```
make hex
```
To program the microcontroller using USBasp, run:
```
make program
```
If you use another programmer, edit the Makefile or flash the hex file manually using avrdude.

Alternatively you can use a bootloader (for example [FastBoot from Peter Dannegger](http://pointless-circuits.com/fastboot-generator/)) instead of flashing the hex file directly to the microcontroller. This way flashing can be done using the serial port.
#### INDI driver
The INDI driver is fairly simple. Just grab the compiled binary file and put it in your /usr/bin folder, if you have indi already installed. But if you want to build the driver by yourself, just follow this instruction to set up the development environment:
[INDI manual](https://www.indilib.org/develop/developer-manual/163-setting-development-environment.html "Official development manual of INDI")

You need to add this projects source files to the CMake list, so that it can be compiled.
Be careful whenever you run a 'sudo make install'. I recommend to install libindi before executing the next step to avoid package manager issues. Otherwise you have to manually clean up your /usr/bin and /usr/lib directory from the indi binaries if you want to install libindi using the package manager at a later time.
Run
```
sudo make install
```
in {indi base dir}/build after you have set up your CMake Build files (see [INDI manual](https://www.indilib.org/develop/developer-manual/163-setting-development-environment.html "Official development manual of INDI")) This will compile all drivers and it puts the binary files into /usr/bin/.
### Using the INDI driver
To run the indiserver type
```
indiserver -v indi_eq_platform
```
Ekos or PHD2 should now be able to connect to the driver and send guiding instructions to the mount.
