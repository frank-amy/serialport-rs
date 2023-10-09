//! Duplex example
//!
//! This example tests the ability to clone a serial port. It works by creating
//! a new file descriptor, and therefore a new `SerialPort` object that's safe
//! to send to a new thread.
//!
//! This example selects the first port on the system, clones the port into a child
//! thread that writes data to the port every second. While this is running the parent
//! thread continually reads from the port.
//!
//! To test this, have a physical or virtual loopback device connected as the
//! only port in the system.

use serialport::{available_ports, SerialPortInfo, SerialPortType, UsbPortInfo};
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

fn main() {
    // Open the first serialport available.
    let valid_serial = available_ports().expect("No serial port");

    for serial in valid_serial {
        match serial.port_type {
            SerialPortType::UsbPort(info) => {
                if info.manufacturer.as_ref().map_or("", String::as_str) == "SEGGER" {
                    println!("Open: {}", serial.port_name);

                    let mut port = serialport::new(serial.port_name, 115200)
                        .open()
                        .expect("Failed to open serial port");

                    // // Clone the port
                    let mut clone = port.try_clone().expect("Failed to clone");

                    // read SN in each second
                    thread::spawn(move || loop {
                        clone
                            .write(String::from("read SN\r").as_bytes())
                            .expect("Failed to write to serial port");
                        thread::sleep(Duration::from_millis(1000));
                    });

                    // Read the four bytes back from the cloned port
                    let mut rece_buf: Vec<u8> = vec![0; 1000];
                    loop {
                        match port.read(rece_buf.as_mut_slice()) {
                            Ok(t) => {
                                let rece_str = String::from_utf8_lossy(&rece_buf[..t]);
                                if rece_str.ends_with("\n") {
                                    println!("{}", rece_str);
                                } else {
                                    print!("{}", rece_str);
                                }
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                            Err(e) => eprintln!("{:?}", e),
                        }
                        thread::sleep(Duration::from_millis(2));
                    }
                }
            }

            __ => {}
        }
    }
}
