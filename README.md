This project is trying to get data from a Honeywell Bluetooth scanner in SPP mode (useing Bluetooth RFCOMM).

Up to now, the function **initiate_connection()** is working. The scanner has to be in discoverable mode.
The appplication is scanning for available Bluetooth devices, you can select the scanner from a list, and then connect to it.
The incoming data is displayed in the console.

However, if the scanner is initiating the connection, the application hangs, i.e. it stays in "Listening for incoming connections...". 
The scanner does connect to a Win10
computer, and in Wireshark I do see scanned barcode data coming in over the USB port, but the application stays in listinging mode.

On a Linux PC (RPi500 with 64bit OS), the application fails to compile with 38 errors like the following:

    error[E0308]: mismatched types
    --> /home/henka/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libbluetooth-0.1.0/src/rfcomm.rs:28:35
    |
    28 | ...T: u64 = request_code_read!('R', 210, std::mem::size_of::<c_int>());
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u64`, found `u32`
    |
    = note: this error originates in the macro `ioc` which comes from the expansion of the macro `request_code_read` (in Nightly builds, run with -Z macro-backtrace for more info)
    
    error[E0308]: mismatched types
    --> /home/henka/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libbluetooth-0.1.0/src/rfcomm.rs:29:35
    |
    29 | ...O: u64 = request_code_read!('R', 211, std::mem::size_of::<c_int>());
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u64`, found `u32`
    |
    = note: this error originates in the macro `ioc` which comes from the expansion of the macro `request_code_read` (in Nightly builds, run with -Z macro-backtrace for more info)
    
