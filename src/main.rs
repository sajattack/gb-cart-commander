#![allow(bad_style)]

mod types;

use std::io::prelude::*;
use std::mem::transmute;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;

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
                .takes_value(true)
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
                .arg(Arg::with_name("size")
                    .short("s")
                    .long("size")
                    .takes_value(true)
                    .possible_values(
                        &["2K", "8K", "32K", "64K", "128K", "256K", "512K", "1M", "2M",
                        "4M", "8M"]))
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
                .arg(Arg::with_name("size")
                    .short("s")
                    .long("size")
                    .takes_value(true)
                    .possible_values(
                        &["2K", "8K", "32K", "64K", "128K", "256K", "512K", "1M", "2M",
                        "4M", "8M"]))

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
                .arg(Arg::with_name("size")
                    .short("s")
                    .long("size")
                    .takes_value(true)
                    .possible_values(
                        &["2K", "8K", "32K", "64K", "128K", "256K", "512K", "1M", "2M",
                        "4M", "8M"]))
            .about("Read flash or ram on a cart")
        );
        let mut helptext_vec = vec![0u8; 800];
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
                    // this can take a while, increase the timeout to 1 minute
                    serialport.set_timeout(Duration::from_secs(60));
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
            let mut file = File::open(sub.value_of("file").unwrap()).unwrap();
            match sub.value_of("what") {
                Some("flash") => {
                    let wr_flash_cmd = [
                        0x55, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 00, 05
                    ];
                    let mut tx_buf = [0u8; 72];
                    tx_buf[0..9].copy_from_slice(&wr_flash_cmd);
                    let checksum = crc16::State::<XMODEM>::calculate(&tx_buf[0..70]);
                    tx_buf[70..72].copy_from_slice(&checksum.to_be_bytes());
                    serialport.write(&tx_buf).expect("failed to write bytes");
                    let mut rx_buf = [0u8; 72];
                    serialport.read(&mut rx_buf).expect("failed to read bytes");
                    // TODO do something if the file size and flash size don't match
                    let n_packets = file.metadata().unwrap().len() / 64; 
                    let mut page_index: u16 = 0;
                    for i in 0..n_packets { 
                        page_index = i as u16 / 256;
                        //println!("{}", page_index);
                        tx_buf[0..3].copy_from_slice(&[0x55, 0x01, 00]);
                        tx_buf[3] = i as u8;
                        tx_buf[4..6].copy_from_slice(&page_index.to_be_bytes());
                        file.read(&mut tx_buf[6..70]).unwrap();
                        let checksum = crc16::State::<XMODEM>::calculate(&tx_buf[0..70]);
                        tx_buf[70..72].copy_from_slice(&checksum.to_be_bytes());
                        serialport.write(&tx_buf).expect("failed to write bytes");
                        serialport.read(&mut rx_buf).expect("failed to read bytes");
                        //TODO check the reponse
                    }
                },
                Some("ram") => unimplemented!(),
                Some(_) => println!("{}", sub.usage()),
                None =>  println!("{}", sub.usage()),
            }
        },
        ("read", Some(sub)) => {
            match sub.value_of("what") {
                Some("flash") => unimplemented!(),
                Some("ram") => unimplemented!(),
                Some(_) => println!("{}", sub.usage()),
                None => println!("{}", sub.usage()),
            }
        },
        (_, Some(sub)) => println!("{}", sub.usage()),
        (_, None) => println!("{}", helptext),
    }
}
