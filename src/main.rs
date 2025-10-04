extern crate io_bluetooth;

use std::io;
use std::iter;
use io_bluetooth::bt::{self, BtAddr, BtListener, BtStream};


fn main() {
    // initiate_connection().unwrap();
    
    let addr = BtAddr([0,0,0,0,0,0]);
    // let addr = BtAddr([0xcc as u8, 0x2f as u8, 0x71 as u8, 0x60 as u8, 0x1f as u8, 0x0c as u8]);
    let listener = BtListener::bind(iter::once(&addr), bt::BtProtocol::RFCOMM);
    println!("listener: {:?}", listener);
    let local_addr = listener.as_ref().map(|l| l.local_addr()).unwrap().unwrap();
    println!("Local BT address: {:?}", local_addr);

    // Listening for incoming connections
    println!("Listening for incoming connections...");
    let ret = listener.unwrap().accept();
    println!("After accept: {:?}", ret);
    
    
}

fn initiate_connection() -> io::Result<()> {


    let devices = bt::discover_devices()?;
    println!("Devices:");
    for (idx, device) in devices.iter().enumerate() {
        println!("{}: {}", idx, *device);
    }

    if devices.len() == 0 {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No Bluetooth devices found.",
        ));
    }

    let device_idx = request_device_idx(devices.len())?;

    let socket = BtStream::connect(iter::once(&devices[device_idx]), bt::BtProtocol::RFCOMM)?;

    match socket.peer_addr() {
        Ok(name) => println!("Peername: {}.", name.to_string()),
        Err(err) => println!("An error occured while retrieving the peername: {:?}", err),
    }

    match socket.local_addr() {
        Ok(name) => println!("Socket name: {}", name.to_string()),
        Err(err) => println!("An error occured while retrieving the sockname: {:?}", err),
    }

    let mut buffer = vec![0; 1024];
    loop {
        match socket.recv(&mut buffer[..]) {
            Ok(len) => println!("Received {} bytes: {}", len, String::from_utf8_lossy(&buffer[..len])),
            Err(err) => return Err(err),
        }
    }    
}

fn request_device_idx(len: usize) -> io::Result<usize> {
    println!("Please specify the index of the Bluetooth device you want to connect to:");

    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        if let Ok(idx) = buffer.trim_end().parse::<usize>() {
            if idx < len {
                return Ok(idx);
            }
        }
        buffer.clear();
        println!("Invalid index. Please try again.");
    }
}
