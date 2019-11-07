#![allow(bad_style)]

mod types;

use std::io::prelude::*;
use std::mem::transmute;
use std::time::Duration;

use clap::{App, Arg, SubCommand};
use crc16::XMODEM;
use serialport::prelude::*;

use crate::types::InfoResponse;

fn main() {
    let app = App::new("GB Cart Commander")
        .version("0.1")
        .author("Paul Sajna <sajattack@gmail.com>")
        .about("Command-line tool for ATmega8515-based Gameboy Cartridge Reader/Writers")
        .arg(
            Arg::with_name("device")
                .help(
                    "Sets the device to use for the serial port connect\nEx: COM4 or /dev/ttyUSB0",
                )
                .required(true),
        )
        .arg(
            Arg::with_name("baudrate")
                .short("b")
                .long("baudrate")
                .help("Sets the baudrate of the serial port connection")
                .default_value("187500")
                .possible_values(&["125000", "187500", "375000"]),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("Prints info about the currently connected cartridge"),
        )
        .subcommand(
            SubCommand::with_name("erase")
                .arg(Arg::with_name("what")
                    .possible_values(&["flash", "ram"])
                ) 
            .about("Erase flash or ram on a flash cart")
        )
        .subcommand(
            SubCommand::with_name("write")
                .arg(Arg::with_name("what")
                    .possible_values(&["flash", "ram"])
                    .required(true)
                )
                .arg(Arg::with_name("file")
                     .required(true)
                )
            .about("Write flash or ram on a flash cart")
        )
        .subcommand(
            SubCommand::with_name("read")
                .arg(Arg::with_name("what")
                    .possible_values(&["flash", "ram"])
                    .required(true)
                )
                .arg(Arg::with_name("file")
                     .required(true)
                )
            .about("Read flash or ram on a cart")
        );
        let mut helptext_vec = vec![0u8; 705];
        app.write_help(&mut helptext_vec).unwrap();
        let helptext = String::from_utf8(helptext_vec).unwrap(); 
        let matches = app.get_matches();
        

    let mut settings: SerialPortSettings = Default::default();
    settings.baud_rate = u32::from_str_radix(matches.value_of("baudrate").unwrap(), 10).unwrap();
    settings.timeout = Duration::from_millis(500);
    let mut serialport =
        serialport::open_with_settings(matches.value_of("device").unwrap(), &settings).unwrap();

    match matches.subcommand() {
        ("info", Some(_sub)) => {
            let info_command = [0x55, 0x04, 0x01];
            let mut tx_buf = [0u8; 72];
            tx_buf[0..3].copy_from_slice(&info_command);
            let checksum: u16 = crc16::State::<XMODEM>::calculate(&tx_buf[0..70]);
            tx_buf[70..72].copy_from_slice(&checksum.to_be_bytes());
            serialport.write(&tx_buf).expect("failed to write bytes");
            let mut rx_buf = [0u8; 72];
            serialport.read(&mut rx_buf).expect("failed to read bytes");
            let info_rx = unsafe { transmute::<[u8; 72], InfoResponse>(rx_buf) };
            //TODO: check the response for success, handle timeout
            println!("{}", info_rx);
        },
       ("erase", Some(sub)) => {
            match sub.value_of("what") {
                Some("flash") => {
                    let er_flash_cmd = [0x55, 0x03, 0x00, 0x00, 0x00, 0x03];
                    let mut tx_buf = [0u8; 72];
                    tx_buf[0..6].copy_from_slice(&er_flash_cmd);
                    let checksum: u16 = crc16::State::<XMODEM>::calculate(&tx_buf[0..70]);
                    tx_buf[70..72].copy_from_slice(&checksum.to_be_bytes());
                    serialport.write(&tx_buf).expect("failed to write bytes");
                    let mut rx_buf = [0u8; 72];
                    serialport.read(&mut rx_buf).expect("failed to read bytes");
                    //TODO: check the response for success, handle timeout
                },
                Some("ram") => {
                    let er_ram_cmd = [0x55, 0x03, 0x01, 0x00, 0x00, 0x03];
                    let mut tx_buf = [0u8; 72];
                    tx_buf[0..6].copy_from_slice(&er_ram_cmd);
                    let checksum: u16 = crc16::State::<XMODEM>::calculate(&tx_buf[0..70]);
                    tx_buf[70..72].copy_from_slice(&checksum.to_be_bytes());
                    serialport.write(&tx_buf).expect("failed to write bytes");
                    let mut rx_buf = [0u8; 72];
                    serialport.read(&mut rx_buf).expect("failed to read bytes");
                    //TODO: check the response for success, handle timeout
                },
                Some(_) => println!("{}", sub.usage()),
                None => println!("{}", sub.usage())
            }
        },
        ("write", Some(sub)) => {
            match sub.value_of("what") {
                Some("flash") => (),
                Some("ram") => (),
                Some(_) => println!("{}", sub.usage()),
                None =>  println!("{}", sub.usage()),
            }
        },
        ("read", Some(sub)) => {
            match sub.value_of("what") {
                Some("flash") => (),
                Some("ram") => (),
                Some(_) => println!("{}", sub.usage()),
                None => println!("{}", sub.usage()),
            }
        },
        (_, Some(sub)) => println!("{}", sub.usage()),
        (_, None) => println!("{}", helptext),
    }
}
