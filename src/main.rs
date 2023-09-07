use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;

use serialport::SerialPort;

fn add_sum_check(input_array: [u8; 6]) -> [u8; 8] {
    // Calculate the sum of the first 6 elements.
    let sum: u16 = input_array.iter().map(|&x| x as u16).sum();

    // Append the sum check to the end of the array.
    let mut output_array = [0; 8];
    output_array[..6].copy_from_slice(&input_array);

    output_array[6] = (sum & 0xFF) as u8; // lower 8 bits
    output_array[7] = (sum >> 8) as u8; // upper 8 bits

    output_array
}

macro_rules! sel {
    (selector $adr:literal connect to $to:expr) => {
        &add_sum_check([0xCC, $adr, 0x44, $to, 0x00, 0xDD])
    };
}

pub fn flush_port(port: &mut dyn SerialPort) {
    loop {
        if read_byte(port).is_none() {
            return;
        }
    }
}

fn read_byte(port: &mut dyn SerialPort) -> Option<u8> {
    let mut b: [u8; 1] = [0];
    if port.read(&mut b).is_ok() {
        //println!("READ 1 BYTE: {:?}", format!("{:x}", b[0]));
        Some(b[0])
    } else {
        None
    }
}

fn read_selector_response(port: &mut dyn SerialPort) -> [u8; 8] {
    let mut resp: [u8; 8] = [0xCC, 0, 0, 0, 0, 0, 0, 0];
    for i in 1..resp.len() - 1 {
        resp[i] = read_byte(port).expect("couldn`t read byte");
        println!("Byte #{}: {:x}", i, resp[i]);
    }
    return resp;
}

fn wait_selector_response(port: &mut dyn SerialPort) -> [u8; 8] {
    let mut b: [u8; 1] = [0];
    loop {
        if read_byte(port).expect("waiting is over") == 0xCC {
            return read_selector_response(port);
        }
    }
}

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_millis(5000))
        .open()
        .expect("Failed to open port");
    flush_port(port.as_mut());
    let mut i = 2;
    loop {
        // let req = sel!(selector 0 connect to 3);
        let req2 = &add_sum_check([0xCC, 0x00, 0x22, 0x00, 0x00, 0xDD] as [u8; 6]);
        println!("REQ2 - {:?}", req2);
        port.write(req2).unwrap();
        sleep(Duration::from_millis(4000));

        let resp = wait_selector_response(port.as_mut());
        println!("RESP - {:?}", resp);

        // let req = sel!(selector 0 connect to {i+=1;i});
        // println!("REQ - {:?}", req);
        // port.write(req).unwrap();
        // sleep(Duration::from_millis(2000));

        // let resp = read_selector_response(port.as_mut());
        // println!("RESP - {:?}", resp);
    }
    // println!("DONE");
}
