# RFID Reader

This is a small Rust library to poll from a RFIDeas pcProx USB reader in SDK
mode (in contrast to their keyboard mode). It relies on
[HIDAPI](https://github.com/signal11/hidapi) to talk with the device.

We are thankful for the API description from https://github.com/micolous/pcprox.

For the below example, be sure that you have access to the right device files,
e.g. with [these udev
rules](https://github.com/micolous/pcprox/blob/master/udev/60-rfideas-permissions.rules).

Check out the `examples` folder for examples of a simple poll or a loop to wait
for the next tag.
